---
name: rhino-rhino-fileio
description: This skill encodes the rhino 7.0 surface of the Rhino.FileIO namespace — 65 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BinaryArchiveFile, BinaryArchiveReader, CommonComponentTable<T>, BinaryArchiveWriter, BinaryArchiveException, ContentHash, DracoCompressionOptions, DracoCompression, and 57 more types.
---

# Rhino.FileIO

Auto-generated from vendor docs for rhino 7.0. 65 types in this namespace.

## BinaryArchiveException (class)

Thrown by BinaryArchiveReader and BinaryArchiveWriter classes when an IO error has occurred.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_BinaryArchiveException.htm)

### Constructors
- `public BinaryArchiveException(string message)` — Initializes a new instance of the BinaryArchiveException class.

## BinaryArchiveFile (class)

[Missing <summary> documentation for "T:Rhino.FileIO.BinaryArchiveFile"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_BinaryArchiveFile.htm)

### Constructors
- `public BinaryArchiveFile(string filename, BinaryArchiveMode mode)` — Initializes a new instance of the BinaryArchiveFile class

### Methods
#### `public void Close()`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveFile_Close.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveFile_Dispose.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveFile_Finalize.htm)

#### `public bool Open()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.FileIO.BinaryArchiveFile.Open"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveFile_Open.htm)

### Properties
- `Reader` (BinaryArchiveReader, get) — 
- `Writer` (BinaryArchiveWriter, get) — 

## BinaryArchiveMode (enum)

[Missing <summary> documentation for "T:Rhino.FileIO.BinaryArchiveMode"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_BinaryArchiveMode.htm)

### Values
- `Unknown` = `0`
- `Read` = `1`
- `Write` = `2`
- `ReadWrite` = `3`
- `Read3dm` = `5`
- `Write3dm` = `6`

## BinaryArchiveReader (class)

Represents an entity that is capable of reading a binary archive and instantiating strongly-typed objects.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_BinaryArchiveReader.htm)

### Methods
#### `public bool AtEnd()`

true if at end of a file

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.FileIO.BinaryArchiveReader.AtEnd"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_AtEnd.htm)

#### `public bool BeginRead3dmChunk(uint expectedTypeCode, out int majorVersion, out int minorVersion)`

Begins reading a chunk that must be in the archive at this location.

**Parameters:**
- `expectedTypeCode` (System.UInt32) — [Missing <param name="expectedTypeCode"/> documentation for "M:Rhino.FileIO.BinaryArchiveReader.BeginRead3dmChunk(System.UInt32,System.Int32@,System.Int32@)"]
- `majorVersion` (System.Int32) — [Missing <param name="majorVersion"/> documentation for "M:Rhino.FileIO.BinaryArchiveReader.BeginRead3dmChunk(System.UInt32,System.Int32@,System.Int32@)"]
- `minorVersion` (System.Int32) — [Missing <param name="minorVersion"/> documentation for "M:Rhino.FileIO.BinaryArchiveReader.BeginRead3dmChunk(System.UInt32,System.Int32@,System.Int32@)"]

**Returns:** `Boolean` — True if beginning of the chunk was read. In this case you must call EndRead3dmChunk(), even if something goes wrong while you attempt to read the interior of the chunk. False if the chunk did not exist at the current location in the file.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_BeginRead3dmChunk.htm)

#### `public bool BeginRead3dmChunk(out uint typeCode, out long value)`

Begins reading a chunk that must be in the archive at this location.

**Parameters:**
- `typeCode` (System.UInt32) — [Missing <param name="typeCode"/> documentation for "M:Rhino.FileIO.BinaryArchiveReader.BeginRead3dmChunk(System.UInt32@,System.Int64@)"]
- `value` (System.Int64) — [Missing <param name="value"/> documentation for "M:Rhino.FileIO.BinaryArchiveReader.BeginRead3dmChunk(System.UInt32@,System.Int64@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.FileIO.BinaryArchiveReader.BeginRead3dmChunk(System.UInt32@,System.Int64@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_BeginRead3dmChunk_1.htm)

#### `public uint Dump3dmChunk(TextLog log)`

Function for studying contents of a file. The primary use is as an aid to help dig through files that have been damaged (bad disks, transmission errors, etc.) If an error is found, a line that begins with the word "ERROR" is printed.

**Parameters:**
- `log` (Rhino.FileIO.TextLog) — log where information is printed to

**Returns:** `UInt32` — 0 if something went wrong, otherwise the typecode of the chunk that was just studied.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_Dump3dmChunk.htm)

#### `public bool EnableCRCCalculation(bool enable)`

Expert user function to control CRC calculation while reading and writing. Typically this is used when seeking around and reading/writing information in non-serial order.

**Parameters:**
- `enable` (System.Boolean) — [Missing <param name="enable"/> documentation for "M:Rhino.FileIO.BinaryArchiveReader.EnableCRCCalculation(System.Boolean)"]

**Returns:** `Boolean` — Current state of CRC calculation. Use the returned value to restore the CRC calculation setting after you are finished doing your fancy pants expert IO.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_EnableCRCCalculation.htm)

#### `public bool EndRead3dmChunk(bool suppressPartiallyReadChunkWarning)`

Calling this will skip rest of stuff in chunk if it was only partially read.

**Parameters:**
- `suppressPartiallyReadChunkWarning` (System.Boolean) — Generally, a call to ON_WARNING is made when a chunk is partially read. If suppressPartiallyReadChunkWarning is true, then no warning is issued for partially read chunks.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.FileIO.BinaryArchiveReader.EndRead3dmChunk(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_EndRead3dmChunk.htm)

#### `public void Read3dmChunkVersion(out int major, out int minor)`

A chunk version is a single byte that encodes a major.minor version number. Useful when creating I/O code for 3dm chunks that may change in the future. Increment the minor version number if new information is added to the end of the chunk. Increment the major version if the format of the chunk changes in some other way.

**Parameters:**
- `major` (System.Int32) — 0 to 15.
- `minor` (System.Int32) — 0 to 16.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_Read3dmChunkVersion.htm)

#### `public bool Read3dmStartSection(out int version, out string comment)`



**Parameters:**
- `version` (System.Int32) — .3dm file version (2, 3, 4, 5 or 50)
- `comment` (System.String) — String with application name, et cetera. This information is primarily used when debugging files that contain problems. McNeel and Associates stores application name, application version, compile date, and the OS in use when file was written.

**Returns:** `Boolean` — true on success

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_Read3dmStartSection.htm)

#### `public bool ReadBool()`

Reads a Boolean from the archive.

**Returns:** `Boolean` — The value that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadBool.htm)

#### `public bool[] ReadBoolArray()`

Reads an array of Boolean from the archive. An array is returned even if the input was another enumerable type.

**Returns:** `Boolean[]` — The array that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadBoolArray.htm)

#### `public BoundingBox ReadBoundingBox()`

Reads a BoundingBox from the archive.

**Returns:** `BoundingBox` — The element that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadBoundingBox.htm)

#### `public byte ReadByte()`

Reads a Byte from the archive.

**Returns:** `Byte` — The value that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadByte.htm)

#### `public byte[] ReadByteArray()`

Reads an array of Byte from the archive. An array is returned even if the input was another enumerable type.

**Returns:** `Byte[]` — The array that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadByteArray.htm)

#### `public Color ReadColor()`

Reads a Color from the archive.

**Returns:** `Color` — The element that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadColor.htm)

#### `public byte[] ReadCompressedBuffer()`

Reads an array of compressed Byte information from the archive and uncompresses it. An array is returned even if the input was another enumerable type.

**Returns:** `Byte[]` — The array that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadCompressedBuffer.htm)

#### `public ArchivableDictionary ReadDictionary()`

Reads a complete ArchivableDictionary from the archive.

**Returns:** `ArchivableDictionary` — The newly instantiated object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadDictionary.htm)

#### `public double ReadDouble()`

Reads a Double from the archive.

**Returns:** `Double` — The value that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadDouble.htm)

#### `public double[] ReadDoubleArray()`

Reads an array of Double from the archive. An array is returned even if the input was another enumerable type.

**Returns:** `Double[]` — The array that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadDoubleArray.htm)

#### `public Font ReadFont()`

Reads a Font from the archive.

**Returns:** `Font` — The element that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadFont.htm)

#### `public GeometryBase ReadGeometry()`

Reads a GeometryBase-derived object from the archive. The GeometryBase class is abstract.

**Returns:** `GeometryBase` — The element that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadGeometry.htm)

#### `public GeometryBase[] ReadGeometryArray()`



**Returns:** `GeometryBase[]` — [Missing <returns> documentation for "M:Rhino.FileIO.BinaryArchiveReader.ReadGeometryArray"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadGeometryArray.htm)

#### `public Guid ReadGuid()`

Reads a Guid from the archive.

**Returns:** `Guid` — The value that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadGuid.htm)

#### `public Guid[] ReadGuidArray()`

Reads an array of Guid from the archive. An array is returned even if the input was another enumerable type.

**Returns:** `Guid[]` — The array that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadGuidArray.htm)

#### `public int ReadInt()`

Reads a Int32 from the archive.

**Returns:** `Int32` — The value that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadInt.htm)

#### `public long ReadInt64()`

Reads a Int64 from the archive.

**Returns:** `Int64` — The value that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadInt64.htm)

#### `public int[] ReadIntArray()`

Reads an array of Int32 from the archive. An array is returned even if the input was another enumerable type.

**Returns:** `Int32[]` — The array that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadIntArray.htm)

#### `public Interval ReadInterval()`

Reads a Interval from the archive.

**Returns:** `Interval` — The element that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadInterval.htm)

#### `public Line ReadLine()`

Reads a Line from the archive.

**Returns:** `Line` — The element that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadLine.htm)

#### `public MeshingParameters ReadMeshingParameters()`

Reads a MeshingParameters from the archive.

**Returns:** `MeshingParameters` — The element that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadMeshingParameters.htm)

#### `public ObjRef ReadObjRef()`

Reads a ObjRef from the archive

**Returns:** `ObjRef` — the element that was read

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadObjRef.htm)

#### `public ObjRef[] ReadObjRefArray()`

Reads an array of Double from the archive. An array is returned even if the input was another enumerable type.

**Returns:** `ObjRef[]` — The array that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadObjRefArray.htm)

#### `public Plane ReadPlane()`

Reads a Plane from the archive.

**Returns:** `Plane` — The element that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadPlane.htm)

#### `public Point ReadPoint()`

Reads a Point from the archive.

**Returns:** `Point` — The element that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadPoint.htm)

#### `public Point2d ReadPoint2d()`

Reads a Point2d from the archive.

**Returns:** `Point2d` — The element that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadPoint2d.htm)

#### `public Point3d ReadPoint3d()`

Reads a Point3d from the archive.

**Returns:** `Point3d` — The element that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadPoint3d.htm)

#### `public Point3f ReadPoint3f()`

Reads a Point3f from the archive.

**Returns:** `Point3f` — The element that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadPoint3f.htm)

#### `public Point4d ReadPoint4d()`

Reads a Point4d from the archive.

**Returns:** `Point4d` — The element that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadPoint4d.htm)

#### `public PointF ReadPointF()`

Reads a PointF from the archive.

**Returns:** `PointF` — The element that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadPointF.htm)

#### `public Ray3d ReadRay3d()`

Reads a Ray3d from the archive.

**Returns:** `Ray3d` — The element that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadRay3d.htm)

#### `public Rectangle ReadRectangle()`

Reads a Rectangle from the archive.

**Returns:** `Rectangle` — The element that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadRectangle.htm)

#### `public RectangleF ReadRectangleF()`

Reads a RectangleF from the archive.

**Returns:** `RectangleF` — The element that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadRectangleF.htm)

#### `public RenderSettings ReadRenderSettings()`

Reads a RenderSettings-derived object from the archive.

**Returns:** `RenderSettings` — The element that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadRenderSettings.htm)

#### `public sbyte ReadSByte()`

Reads a SByte from the archive.

**Returns:** `SByte` — The value that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadSByte.htm)

#### `public sbyte[] ReadSByteArray()`

Reads an array of SByte from the archive. An array is returned even if the input was another enumerable type.

**Returns:** `SByte[]` — The array that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadSByteArray.htm)

#### `public short ReadShort()`

Reads a Int16 from the archive.

**Returns:** `Int16` — The value that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadShort.htm)

#### `public short[] ReadShortArray()`

Reads an array of Int16 from the archive. An array is returned even if the input was another enumerable type.

**Returns:** `Int16[]` — The array that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadShortArray.htm)

#### `public float ReadSingle()`

Reads a Single from the archive.

**Returns:** `Single` — The value that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadSingle.htm)

#### `public float[] ReadSingleArray()`

Reads an array of Single from the archive. An array is returned even if the input was another enumerable type.

**Returns:** `Single[]` — The array that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadSingleArray.htm)

#### `public Size ReadSize()`

Reads a Size from the archive.

**Returns:** `Size` — The element that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadSize.htm)

#### `public SizeF ReadSizeF()`

Reads a SizeF from the archive.

**Returns:** `SizeF` — The element that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadSizeF.htm)

#### `public string ReadString()`

Reads a String from the archive.

**Returns:** `String` — The value that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadString.htm)

#### `public string[] ReadStringArray()`

Reads an array of String from the archive. An array is returned even if the input was another enumerable type.

**Returns:** `String[]` — The array that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadStringArray.htm)

#### `public Transform ReadTransform()`

Reads a Transform from the archive.

**Returns:** `Transform` — The element that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadTransform.htm)

#### `public uint ReadUInt()`

Reads a UInt32 from the archive.

**Returns:** `UInt32` — The value that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadUInt.htm)

#### `public ushort ReadUShort()`

Reads a UInt16 from the archive.

**Returns:** `UInt16` — The value that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadUShort.htm)

#### `public string ReadUtf8String()`

Reads a String from the archive.

**Returns:** `String` — The value that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadUtf8String.htm)

#### `public Vector2d ReadVector2d()`

Reads a Vector2d from the archive.

**Returns:** `Vector2d` — The element that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadVector2d.htm)

#### `public Vector3d ReadVector3d()`

Reads a Vector3d from the archive.

**Returns:** `Vector3d` — The element that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadVector3d.htm)

#### `public Vector3f ReadVector3f()`

Reads a Vector3f from the archive.

**Returns:** `Vector3f` — The element that was read.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_ReadVector3f.htm)

#### `public bool SeekFromCurrentPosition(long byteOffset)`

seek from current position ( like fseek( ,SEEK_CUR) )

**Parameters:**
- `byteOffset` (System.Int64) — [Missing <param name="byteOffset"/> documentation for "M:Rhino.FileIO.BinaryArchiveReader.SeekFromCurrentPosition(System.Int64)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.FileIO.BinaryArchiveReader.SeekFromCurrentPosition(System.Int64)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_SeekFromCurrentPosition.htm)

#### `public bool SeekFromCurrentPosition(ulong byteOffset, bool forward)`

seek from current position ( like fseek( ,SEEK_CUR) )

**Parameters:**
- `byteOffset` (System.UInt64) — [Missing <param name="byteOffset"/> documentation for "M:Rhino.FileIO.BinaryArchiveReader.SeekFromCurrentPosition(System.UInt64,System.Boolean)"]
- `forward` (System.Boolean) — seek forward of backward in the archive

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.FileIO.BinaryArchiveReader.SeekFromCurrentPosition(System.UInt64,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_SeekFromCurrentPosition_1.htm)

#### `public bool SeekFromStart(ulong byteOffset)`

seek from start position ( like fseek( ,SEEK_SET) )

**Parameters:**
- `byteOffset` (System.UInt64) — [Missing <param name="byteOffset"/> documentation for "M:Rhino.FileIO.BinaryArchiveReader.SeekFromStart(System.UInt64)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.FileIO.BinaryArchiveReader.SeekFromStart(System.UInt64)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveReader_SeekFromStart.htm)

### Properties
- `Archive3dmVersion` (Int32, get) — If a 3dm archive is being read or written, then this is the version of the 3dm archive format (1, 2, 3, 4 or 5). 0 a 3dm archive is not being read/written 1 a version 1 3dm archive is being read/written 2 a version 2 3dm archive is being read/written 3 a version 3 3dm archive is being read/written 4 a version 4 3dm archive is being read/written 5 an old version 5 3dm archive is being read 50 a version 5 3dm archive is being read/written.
- `CurrentPosition` (UInt64, get) — current offset (in bytes) into archive ( like ftell() )
- `ReadErrorOccured` (Boolean, get/set) — Gets or sets whether en error occurred during reading.

## BinaryArchiveWriter (class)

Represents an entity that is able to write data to an archive.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_BinaryArchiveWriter.htm)

### Methods
#### `public bool BeginWrite3dmChunk(uint typecode, int majorVersion, int minorVersion)`

Begins writing a chunk

**Parameters:**
- `typecode` (System.UInt32) — chunk's typecode
- `majorVersion` (System.Int32) — [Missing <param name="majorVersion"/> documentation for "M:Rhino.FileIO.BinaryArchiveWriter.BeginWrite3dmChunk(System.UInt32,System.Int32,System.Int32)"]
- `minorVersion` (System.Int32) — [Missing <param name="minorVersion"/> documentation for "M:Rhino.FileIO.BinaryArchiveWriter.BeginWrite3dmChunk(System.UInt32,System.Int32,System.Int32)"]

**Returns:** `Boolean` — True if input was valid and chunk was started. In this case you must call EndWrite3dmChunk(), even if something goes wrong while you attempt to write the contents of the chunk. False if input was not valid or the write failed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_BeginWrite3dmChunk.htm)

#### `public bool BeginWrite3dmChunk(uint typecode, long value)`

Begins writing a chunk

**Parameters:**
- `typecode` (System.UInt32) — [Missing <param name="typecode"/> documentation for "M:Rhino.FileIO.BinaryArchiveWriter.BeginWrite3dmChunk(System.UInt32,System.Int64)"]
- `value` (System.Int64) — [Missing <param name="value"/> documentation for "M:Rhino.FileIO.BinaryArchiveWriter.BeginWrite3dmChunk(System.UInt32,System.Int64)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.FileIO.BinaryArchiveWriter.BeginWrite3dmChunk(System.UInt32,System.Int64)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_BeginWrite3dmChunk_1.htm)

#### `public bool EnableCRCCalculation(bool enable)`

Expert user function to control CRC calculation while reading and writing. Typically this is used when seeking around and reading/writing information in non-serial order.

**Parameters:**
- `enable` (System.Boolean) — [Missing <param name="enable"/> documentation for "M:Rhino.FileIO.BinaryArchiveWriter.EnableCRCCalculation(System.Boolean)"]

**Returns:** `Boolean` — Current state of CRC calculation. Use the returned value to restore the CRC calculation setting after you are finished doing your fancy pants expert IO.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_EnableCRCCalculation.htm)

#### `public bool EndWrite3dmChunk()`

updates length in chunk header

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.FileIO.BinaryArchiveWriter.EndWrite3dmChunk"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_EndWrite3dmChunk.htm)

#### `public void Write3dmChunkVersion(int major, int minor)`

A chunk version is a single byte that encodes a major.minor version number. Useful when creating I/O code for 3dm chunks that may change in the future. Increment the minor version number if new information is added to the end of the chunk. Increment the major version if the format of the chunk changes in some other way.

**Parameters:**
- `major` (System.Int32) — 0 to 15.
- `minor` (System.Int32) — 0 to 16.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_Write3dmChunkVersion.htm)

#### `public void WriteBool(bool value)`

Writes a Boolean value to the archive.

**Parameters:**
- `value` (System.Boolean) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteBool.htm)

#### `public void WriteBoolArray(IEnumerable<bool> value)`

Writes a list, an array, or any enumerable of Boolean to the archive. The return will always be an array.

**Parameters:**
- `value` (System.Collections.Generic.IEnumerable<Boolean>) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteBoolArray.htm)

#### `public void WriteBoundingBox(BoundingBox value)`

Writes a BoundingBox value to the archive.

**Parameters:**
- `value` (Rhino.Geometry.BoundingBox) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteBoundingBox.htm)

#### `public void WriteByte(byte value)`

Writes a Byte value to the archive.

**Parameters:**
- `value` (System.Byte) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteByte.htm)

#### `public void WriteByteArray(IEnumerable<byte> value)`

Writes a list, an array, or any enumerable of Byte to the archive. The return will always be an array.

**Parameters:**
- `value` (System.Collections.Generic.IEnumerable<Byte>) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteByteArray.htm)

#### `public void WriteColor(Color value)`

Writes a Color value to the archive.

**Parameters:**
- `value` (System.Drawing.Color) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteColor.htm)

#### `public void WriteCompressedBuffer(IEnumerable<byte> value)`

Writes a list, an array, or any enumerable of Byte to the archive as a compressed buffer. The return will always be an array.

**Parameters:**
- `value` (System.Collections.Generic.IEnumerable<Byte>) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteCompressedBuffer.htm)

#### `public void WriteDictionary(ArchivableDictionary dictionary)`

Delivers the complete content of a dictionary to the archive.

**Parameters:**
- `dictionary` (Rhino.Collections.ArchivableDictionary) — A dictionary to archive.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteDictionary.htm)

#### `public void WriteDouble(double value)`

Writes a Double value to the archive.

**Parameters:**
- `value` (System.Double) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteDouble.htm)

#### `public void WriteDoubleArray(IEnumerable<double> value)`

Writes a list, an array, or any enumerable of Double to the archive. The return will always be an array.

**Parameters:**
- `value` (System.Collections.Generic.IEnumerable<Double>) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteDoubleArray.htm)

#### `public void WriteFont(Font value)`

Writes a Font value to the archive.

**Parameters:**
- `value` (System.Drawing.Font) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteFont.htm)

#### `public void WriteGeometry(GeometryBase value)`

Writes a GeometryBase value to the archive.

**Parameters:**
- `value` (Rhino.Geometry.GeometryBase) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteGeometry.htm)

#### `public void WriteGeometryArray(IEnumerable<GeometryBase> geometry)`



**Parameters:**
- `geometry` (System.Collections.Generic.IEnumerable<GeometryBase>) — [Missing <param name="geometry"/> documentation for "M:Rhino.FileIO.BinaryArchiveWriter.WriteGeometryArray(System.Collections.Generic.IEnumerable{Rhino.Geometry.GeometryBase})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteGeometryArray.htm)

#### `public void WriteGuid(Guid value)`

Writes a Guid value to the archive.

**Parameters:**
- `value` (System.Guid) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteGuid.htm)

#### `public void WriteGuidArray(IEnumerable<Guid> value)`

Writes a list, an array, or any enumerable of Guid to the archive. The return will always be an array.

**Parameters:**
- `value` (System.Collections.Generic.IEnumerable<Guid>) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteGuidArray.htm)

#### `public void WriteInt(int value)`

Writes a Int32 value to the archive.

**Parameters:**
- `value` (System.Int32) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteInt.htm)

#### `public void WriteInt64(long value)`

Writes a Int64 value to the archive.

**Parameters:**
- `value` (System.Int64) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteInt64.htm)

#### `public void WriteIntArray(IEnumerable<int> value)`

Writes a list, an array, or any enumerable of Int32 to the archive. The return will always be an array.

**Parameters:**
- `value` (System.Collections.Generic.IEnumerable<Int32>) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteIntArray.htm)

#### `public void WriteInterval(Interval value)`

Writes a Interval value to the archive.

**Parameters:**
- `value` (Rhino.Geometry.Interval) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteInterval.htm)

#### `public void WriteLine(Line value)`

Writes a Line value to the archive.

**Parameters:**
- `value` (Rhino.Geometry.Line) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteLine.htm)

#### `public void WriteMeshingParameters(MeshingParameters value)`

Writes a MeshingParameters value to the archive.

**Parameters:**
- `value` (Rhino.Geometry.MeshingParameters) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteMeshingParameters.htm)

#### `public void WriteObjRef(ObjRef objref)`

Writes a ObjRef to the archive

**Parameters:**
- `objref` (Rhino.DocObjects.ObjRef) — [Missing <param name="objref"/> documentation for "M:Rhino.FileIO.BinaryArchiveWriter.WriteObjRef(Rhino.DocObjects.ObjRef)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteObjRef.htm)

#### `public void WriteObjRefArray(IEnumerable<ObjRef> objrefs)`

Writes a list, an array, or any enumerable of ObjRef to the archive. The return will always be an array.

**Parameters:**
- `objrefs` (System.Collections.Generic.IEnumerable<ObjRef>) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteObjRefArray.htm)

#### `public void WritePlane(Plane value)`

Writes a Plane value to the archive.

**Parameters:**
- `value` (Rhino.Geometry.Plane) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WritePlane.htm)

#### `public void WritePoint(Point value)`

Writes a Point value to the archive.

**Parameters:**
- `value` (System.Drawing.Point) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WritePoint.htm)

#### `public void WritePoint2d(Point2d value)`

Writes a Point2d value to the archive.

**Parameters:**
- `value` (Rhino.Geometry.Point2d) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WritePoint2d.htm)

#### `public void WritePoint3d(Point3d value)`

Writes a Point3d value to the archive.

**Parameters:**
- `value` (Rhino.Geometry.Point3d) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WritePoint3d.htm)

#### `public void WritePoint3f(Point3f value)`

Writes a Point3f value to the archive.

**Parameters:**
- `value` (Rhino.Geometry.Point3f) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WritePoint3f.htm)

#### `public void WritePoint4d(Point4d value)`

Writes a Point4d value to the archive.

**Parameters:**
- `value` (Rhino.Geometry.Point4d) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WritePoint4d.htm)

#### `public void WritePointF(PointF value)`

Writes a PointF value to the archive.

**Parameters:**
- `value` (System.Drawing.PointF) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WritePointF.htm)

#### `public void WriteRay3d(Ray3d value)`

Writes a Ray3d value to the archive.

**Parameters:**
- `value` (Rhino.Geometry.Ray3d) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteRay3d.htm)

#### `public void WriteRectangle(Rectangle value)`

Writes a Rectangle value to the archive.

**Parameters:**
- `value` (System.Drawing.Rectangle) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteRectangle.htm)

#### `public void WriteRectangleF(RectangleF value)`

Writes a RectangleF value to the archive.

**Parameters:**
- `value` (System.Drawing.RectangleF) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteRectangleF.htm)

#### `public void WriteRenderSettings(RenderSettings value)`

Writes a RenderSettings value to the archive.

**Parameters:**
- `value` (Rhino.Render.RenderSettings) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteRenderSettings.htm)

#### `public void WriteSByte(sbyte value)`

Writes a SByte value to the archive.

**Parameters:**
- `value` (System.SByte) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteSByte.htm)

#### `public void WriteSByteArray(IEnumerable<sbyte> value)`

Writes a list, an array, or any enumerable of SByte to the archive. The return will always be an array.

**Parameters:**
- `value` (System.Collections.Generic.IEnumerable<SByte>) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteSByteArray.htm)

#### `public void WriteShort(short value)`

Writes a Int16 value to the archive.

**Parameters:**
- `value` (System.Int16) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteShort.htm)

#### `public void WriteShortArray(IEnumerable<short> value)`

Writes a list, an array, or any enumerable of Int16 to the archive. The return will always be an array.

**Parameters:**
- `value` (System.Collections.Generic.IEnumerable<Int16>) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteShortArray.htm)

#### `public void WriteSingle(float value)`

Writes a Single value to the archive.

**Parameters:**
- `value` (System.Single) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteSingle.htm)

#### `public void WriteSingleArray(IEnumerable<float> value)`

Writes a list, an array, or any enumerable of Single to the archive. The return will always be an array.

**Parameters:**
- `value` (System.Collections.Generic.IEnumerable<Single>) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteSingleArray.htm)

#### `public void WriteSize(Size value)`

Writes a Size value to the archive.

**Parameters:**
- `value` (System.Drawing.Size) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteSize.htm)

#### `public void WriteSizeF(SizeF value)`

Writes a SizeF value to the archive.

**Parameters:**
- `value` (System.Drawing.SizeF) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteSizeF.htm)

#### `public void WriteString(string value)`

Writes a String value to the archive.

**Parameters:**
- `value` (System.String) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteString.htm)

#### `public void WriteStringArray(IEnumerable<string> value)`

Writes a list, an array, or any enumerable of String to the archive. The return will always be an array.

**Parameters:**
- `value` (System.Collections.Generic.IEnumerable<String>) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteStringArray.htm)

#### `public void WriteTransform(Transform value)`

Writes a Transform value to the archive.

**Parameters:**
- `value` (Rhino.Geometry.Transform) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteTransform.htm)

#### `public void WriteUInt(uint value)`

Writes a UInt32 value to the archive.

**Parameters:**
- `value` (System.UInt32) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteUInt.htm)

#### `public void WriteUShort(ushort value)`

Writes a UInt16 value to the archive.

**Parameters:**
- `value` (System.UInt16) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteUShort.htm)

#### `public void WriteUtf8String(string value)`

Writes a String value to the archive.

**Parameters:**
- `value` (System.String) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteUtf8String.htm)

#### `public void WriteVector2d(Vector2d value)`

Writes a Vector2d value to the archive.

**Parameters:**
- `value` (Rhino.Geometry.Vector2d) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteVector2d.htm)

#### `public void WriteVector3d(Vector3d value)`

Writes a Vector3d value to the archive.

**Parameters:**
- `value` (Rhino.Geometry.Vector3d) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteVector3d.htm)

#### `public void WriteVector3f(Vector3f value)`

Writes a Vector3f value to the archive.

**Parameters:**
- `value` (Rhino.Geometry.Vector3f) — A value to write.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_BinaryArchiveWriter_WriteVector3f.htm)

### Properties
- `Archive3dmVersion` (Int32, get) — If a 3dm archive is being read or written, then this is the version of the 3dm archive format (1, 2, 3, 4 or 5). 0 a 3dm archive is not being read/written 1 a version 1 3dm archive is being read/written 2 a version 2 3dm archive is being read/written 3 a version 3 3dm archive is being read/written 4 a version 4 3dm archive is being read/written 5 an old version 5 3dm archive is being read 50 a version 5 3dm archive is being read/written.
- `WriteErrorOccured` (Boolean, get/set) — Gets or sets whether an error occurred.

## CommonComponentTable<T> (class)

Provides a base table type that encompasses all document tables, both in RhinoDoc and File3dm.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_CommonComponentTable_1.htm)

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

## ContentHash (class)

Contains information that is useful to uniquely identify an object. RemarksThis object is immutable.

**Remarks:** This object is immutable.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_ContentHash.htm)

### Constructors
- `protected ContentHash(ContentHash other)` — Constructs a copy of a content hash.

### Methods
#### `public ContentHash Clone()`

Creates a copy of this content hash. Because content hash is immutable, this can be used as a deep copy.

**Returns:** `ContentHash` — A different instance of the same content hash.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_ContentHash_Clone.htm)

#### `public static ContentHash CreateFromFile(string path)`

Creates a new ContentHash, representing the content of a file.

**Parameters:**
- `path` (System.String) — A path. This can be null and can refer to a non-existing path.

**Returns:** `ContentHash` — [Missing <returns> documentation for "M:Rhino.FileIO.ContentHash.CreateFromFile(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_ContentHash_CreateFromFile.htm)

#### `public static bool operator ==(ContentHash left, ContentHash right)`

Determines if two ContentHash instances are equal by value.

**Parameters:**
- `left` (Rhino.FileIO.ContentHash) — The first hash.
- `right` (Rhino.FileIO.ContentHash) — The second hash.

**Returns:** `Boolean` — True if they are equal by value, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_ContentHash_op_Equality.htm)

#### `public bool Equals(ContentHash other)`

Determines if another content hash has the same value.

**Parameters:**
- `other` (Rhino.FileIO.ContentHash) — The other content hash to compare.

**Returns:** `Boolean` — True if the two hashes are equal.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_ContentHash_Equals.htm)

#### `public override bool Equals(Object obj)`

Determines if another object is a content hash with same value.

**Parameters:**
- `obj` (System.Object) — The other content hash to compare.

**Returns:** `Boolean` — True if the two hashes are equal.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_ContentHash_Equals_1.htm)

#### `public override int GetHashCode()`

Gets an hash code for this content hash. Two equal content hashes have equal hash code. The other way around might not be true.

**Returns:** `Int32` — An hash code value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_ContentHash_GetHashCode.htm)

#### `public static bool operator !=(ContentHash left, ContentHash right)`

Determines if two ContentHash instances are different by value.

**Parameters:**
- `left` (Rhino.FileIO.ContentHash) — The first hash.
- `right` (Rhino.FileIO.ContentHash) — The second hash.

**Returns:** `Boolean` — True if they are different by value, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_ContentHash_op_Inequality.htm)

### Properties
- `ByteCount` (UInt64, get) — Gets the length of the content, in bytes.
- `HashTime` (DateTime, get) — Gets the hash time, rounded to seconds.
- `Sha1ContentHash` (Byte[], get) — Gets the 20-bytes long SHA1 hash of the content.
- `Sha1NameHash` (Byte[], get) — Gets the 20-bytes long SHA1 hash of the name.

## DracoColorFormat (enum)

Color format for vertex colors

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_DracoColorFormat.htm)

### Values
- `ARGB` = `0` — Alpha, Red, Green, Blue order
- `RGBA` = `1` — Red, Green, Blue, Alpha order

## DracoCompression (class)

Google Draco compression for mesh and point cloud data

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_DracoCompression.htm)

### Methods
#### `public static DracoCompression Compress(Mesh mesh)`

Compress a mesh using default compression options.

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — mesh to compress

**Returns:** `DracoCompression` — instance of class representing the compressed data

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_DracoCompression_Compress.htm)

#### `public static DracoCompression Compress(Mesh mesh, DracoCompressionOptions options)`

Compress a mesh

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — mesh to compress
- `options` (Rhino.FileIO.DracoCompressionOptions) — options used to determine how the compression will occur

**Returns:** `DracoCompression` — instance of class representing the compressed data

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_DracoCompression_Compress_1.htm)

#### `public static GeometryBase DecompressBase64String(string encoded)`

Decompress base64 encoded version of Draco data into either a mesh or point cloud

**Parameters:**
- `encoded` (System.String) — compressed Draco data

**Returns:** `GeometryBase` — Mesh or point cloud on success. null on failure

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_DracoCompression_DecompressBase64String.htm)

#### `public static GeometryBase DecompressByteArray(byte[] bytes)`

Decompress data into either a mesh or point cloud.

**Parameters:**
- `bytes` (System.Byte[]) — compressed Draco data

**Returns:** `GeometryBase` — Mesh or point cloud on success. null on failure

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_DracoCompression_DecompressByteArray.htm)

#### `public static GeometryBase DecompressFile(string path)`

Read compressed data from disk and decompress to RhinoCommon geometry

**Parameters:**
- `path` (System.String) — path to read from

**Returns:** `GeometryBase` — Mesh or point cloud on success. null on failure

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_DracoCompression_DecompressFile.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_DracoCompression_Dispose.htm)

#### `protected override void Finalize()`

Finalizer for DracoCompression

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_DracoCompression_Finalize.htm)

#### `public string ToBase64String()`

Convert byte array of Draco compressed data into a base64 encoded string

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.FileIO.DracoCompression.ToBase64String"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_DracoCompression_ToBase64String.htm)

#### `public bool Write(string path)`

Write the compressed data to disk

**Parameters:**
- `path` (System.String) — path to write to

**Returns:** `Boolean` — true on success

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_DracoCompression_Write.htm)

### Properties
- `NormalAttributePosition` (Int32, get) — The attribute position for normals when compressed
- `TextureCoordinatesAttributePosition` (Int32, get) — The attribute position for texture coordinates when compressed
- `VertexAttributePosition` (Int32, get) — The attribute position for vertices when compressed
- `VertexColorAttributePosition` (Int32, get) — The attribute position for vertex colors when compressed

## DracoCompressionOptions (class)

Options for applying Draco compression

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_DracoCompressionOptions.htm)

### Constructors
- `public DracoCompressionOptions()` — Initializes to default options

### Properties
- `CompressionLevel` (Int32, get/set) — Compression level. 0 if fastest, but least compression. 10 is slowest, but best compression
- `NormalQuantizationBits` (Int32, get/set) — Sets the quantization compression options for normal values. The values will be quantized in a box defined by the maximum extent of the values. I.e., the actual precision of this option depends on the scale of the attribute values.
- `PositionQuantizationBits` (Int32, get/set) — Sets the quantization compression options for position values. The values will be quantized in a box defined by the maximum extent of the values. I.e., the actual precision of this option depends on the scale of the attribute values.
- `TextureCoordintateQuantizationBits` (Int32, get/set) — Sets the quantization compression options for texture coordinate values. The values will be quantized in a box defined by the maximum extent of the values. I.e., the actual precision of this option depends on the scale of the attribute values.
- `IncludeNormals` (Boolean, get) — Include vertex normals in the compressed data.
- `IncludeTextureCoordinates` (Boolean, get) — Include texture coordinates in the compressed data.
- `IncludeVertexColors` (Boolean, get) — Include vertex colors in the compressed data.
- `VertexColorFormat` (DracoColorFormat, get) — Color format of vertex colors in compressed data

## File3dm (class)

Represents a 3dm file, which is stored using the OpenNURBS file standard. The 3dm format is the main Rhinoceros storage format.Visit http://www.opennurbs.com/ for more details.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_File3dm.htm)

### Constructors
- `public File3dm()` — Initializes a new instance of a 3dm file.

### Methods
#### `[ObsoleteAttribute("Polish and Audit functionality no longer exist.")] public int Audit(bool attemptRepair, out int repairCount, out string errors, out int[] warnings)`

This function is only kept for forward assembly compatibility.

**Parameters:**
- `attemptRepair` (System.Boolean) — Ignored.
- `repairCount` (System.Int32) — Is set to 0.
- `errors` (System.String) — Contains no meaningful error.
- `warnings` (System.Int32[]) — Is set to null.

**Returns:** `Int32` — Returns 0.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dm_Audit.htm)

#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dm_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dm_Dispose_1.htm)

#### `public string Dump()`

Prepares a text dump of the entire model.

**Returns:** `String` — The text dump.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dm_Dump.htm)

#### `public string DumpSummary()`

Prepares a text dump of model properties and settings.

**Returns:** `String` — The text dump.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dm_DumpSummary.htm)

#### `public void DumpToTextLog(TextLog log)`

Prepares a text dump of the entire model.

**Parameters:**
- `log` (Rhino.FileIO.TextLog) — [Missing <param name="log"/> documentation for "M:Rhino.FileIO.File3dm.DumpToTextLog(Rhino.FileIO.TextLog)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dm_DumpToTextLog.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dm_Finalize.htm)

#### `public static File3dm FromByteArray(byte[] bytes)`

Read a 3dm file from a byte array

**Parameters:**
- `bytes` (System.Byte[]) — [Missing <param name="bytes"/> documentation for "M:Rhino.FileIO.File3dm.FromByteArray(System.Byte[])"]

**Returns:** `File3dm` — New File3dm on success, null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dm_FromByteArray.htm)

#### `public Bitmap GetPreviewImage()`

Preview image used for file explorer

**Returns:** `Bitmap` — [Missing <returns> documentation for "M:Rhino.FileIO.File3dm.GetPreviewImage"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dm_GetPreviewImage.htm)

#### `[ObsoleteAttribute("IsValid now returns always true.")] public bool IsValid(out string errors)`

The File3dm object is kept consistent during its creation. Therefore, this function now returns only true.

**Parameters:**
- `errors` (System.String) — No errors are found.

**Returns:** `Boolean` — true in any case.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dm_IsValid_1.htm)

#### `[ObsoleteAttribute("IsValid now returns always true.")] public bool IsValid(TextLog errors)`

The File3dm object is kept consistent during its creation. Therefore, this function now returns only true.

**Parameters:**
- `errors` (Rhino.FileIO.TextLog) — No errors are found.

**Returns:** `Boolean` — >true in any case.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dm_IsValid.htm)

#### `[ObsoleteAttribute("Polish and Audit functionality no longer exist.")] public void Polish()`

This function is only kept for forward assembly compatibility.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dm_Polish.htm)

#### `public static File3dm Read(string path)`

Reads a 3dm file from a specified location.

**Parameters:**
- `path` (System.String) — The file to read.

**Returns:** `File3dm` — new File3dm on success, null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dm_Read.htm)

#### `public static File3dm Read(string path, File3dm.TableTypeFilter tableTypeFilterFilter, File3dm.ObjectTypeFilter objectTypeFilter)`

Reads a 3dm file from a specified location.

**Parameters:**
- `path` (System.String) — The file to read.
- `tableTypeFilterFilter` (Rhino.FileIO.File3dm.TableTypeFilter) — If tableTypeFilterFilter is None, then everything in the archive is read. Otherwise tableTypeFilterFilter identifies what tables should be read.
- `objectTypeFilter` (Rhino.FileIO.File3dm.ObjectTypeFilter) — If objectTypeFilter is not None, then is a filter made by bitwise or-ing values to select which types of objects will be read from the model object table.

**Returns:** `File3dm` — new File3dm on success, null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dm_Read_1.htm)

#### `public static void ReadApplicationData(string path, out string applicationName, out string applicationUrl, out string applicationDetails)`

Reads only the application information from an existing 3dm file.

**Parameters:**
- `path` (System.String) — A location on disk or network.
- `applicationName` (System.String) — The application name. This out parameter is assigned during this call.
- `applicationUrl` (System.String) — The application URL. This out parameter is assigned during this call.
- `applicationDetails` (System.String) — The application details. This out parameter is assigned during this call.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dm_ReadApplicationData.htm)

#### `public static int ReadArchiveVersion(string path)`

Reads only the archive 3dm version from an existing 3dm file.

**Parameters:**
- `path` (System.String) — The file from which to read the archive version.

**Returns:** `Int32` — The 3dm file archive version.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dm_ReadArchiveVersion.htm)

#### `public static DimensionStyle[] ReadDimensionStyles(string path)`

Read the dimension styles table out of a 3dm file.

**Parameters:**
- `path` (System.String) — The location of the file.

**Returns:** `DimensionStyle[]` — Array of dimension styles on success (empty array if file does not contain dimension styles) null on error

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dm_ReadDimensionStyles.htm)

#### `public static string ReadNotes(string path)`

Reads only the notes from an existing 3dm file.

**Parameters:**
- `path` (System.String) — The file from which to read the notes.

**Returns:** `String` — The 3dm file notes.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dm_ReadNotes.htm)

#### `public static Bitmap ReadPreviewImage(string path)`

Attempts to read the preview image out of a 3dm file.

**Parameters:**
- `path` (System.String) — The location of the file.

**Returns:** `Bitmap` — A bitmap, or null on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dm_ReadPreviewImage.htm)

#### `public static bool ReadRevisionHistory(string path, out string createdBy, out string lastEditedBy, out int revision, out DateTime createdOn, out DateTime lastEditedOn)`

Quickly check a file for it's revision information. This function does not read the entire file, just what it needs to get revision information out

**Parameters:**
- `path` (System.String) — path to the 3dm file
- `createdBy` (System.String) — original author of the file
- `lastEditedBy` (System.String) — last person to edit the file
- `revision` (System.Int32) — which revision this file is at
- `createdOn` (System.DateTime) — date file was created (DateTime.MinValue if not set in file)
- `lastEditedOn` (System.DateTime) — date file was last edited (DateTime.MinValue if not set in file)

**Returns:** `Boolean` — true on success

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dm_ReadRevisionHistory.htm)

#### `public static File3dm ReadWithLog(string path, File3dm.TableTypeFilter tableTypeFilterFilter, File3dm.ObjectTypeFilter objectTypeFilter, out string errorLog)`

Reads a 3dm file from a specified location.

**Parameters:**
- `path` (System.String) — The file to read.
- `tableTypeFilterFilter` (Rhino.FileIO.File3dm.TableTypeFilter) — If tableTypeFilterFilter is None, then everything in the archive is read. Otherwise tableTypeFilterFilter identifies what tables should be read.
- `objectTypeFilter` (Rhino.FileIO.File3dm.ObjectTypeFilter) — If objectTypeFilter is not None, then is a filter made by bitwise or-ing values to select which types of objects will be read from the model object table.
- `errorLog` (System.String) — Any archive reading errors are logged here.

**Returns:** `File3dm` — new File3dm on success, null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dm_ReadWithLog.htm)

#### `public static File3dm ReadWithLog(string path, out string errorLog)`

Read a 3dm file from a specified location and log any archive reading errors.

**Parameters:**
- `path` (System.String) — The file to read.
- `errorLog` (System.String) — Any archive reading errors are logged here.

**Returns:** `File3dm` — New File3dm on success, null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dm_ReadWithLog_1.htm)

#### `public void SetPreviewImage(Bitmap image)`

Preview image used for file explorer

**Parameters:**
- `image` (System.Drawing.Bitmap) — [Missing <param name="image"/> documentation for "M:Rhino.FileIO.File3dm.SetPreviewImage(System.Drawing.Bitmap)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dm_SetPreviewImage.htm)

#### `public byte[] ToByteArray()`

Write to an in-memory byte[]

**Returns:** `Byte[]` — [Missing <returns> documentation for "M:Rhino.FileIO.File3dm.ToByteArray"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dm_ToByteArray.htm)

#### `public byte[] ToByteArray(File3dmWriteOptions options)`

Write to an in-memory byte[]

**Parameters:**
- `options` (Rhino.FileIO.File3dmWriteOptions) — [Missing <param name="options"/> documentation for "M:Rhino.FileIO.File3dm.ToByteArray(Rhino.FileIO.File3dmWriteOptions)"]

**Returns:** `Byte[]` — [Missing <returns> documentation for "M:Rhino.FileIO.File3dm.ToByteArray(Rhino.FileIO.File3dmWriteOptions)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dm_ToByteArray_1.htm)

#### `public bool Write(string path, File3dmWriteOptions options)`

Writes contents of this model to an openNURBS archive. If the model is not valid, then Write will refuse to write it.

**Parameters:**
- `path` (System.String) — The file name to use for writing.
- `options` (Rhino.FileIO.File3dmWriteOptions) — An options instance, or null for default.

**Returns:** `Boolean` — true if archive is written with no error. false if errors occur.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dm_Write.htm)

#### `public bool Write(string path, int version)`

Writes contents of this model to an openNURBS archive. If the model is not valid, then Write will refuse to write it.

**Parameters:**
- `path` (System.String) — The file name to use for writing.
- `version` (System.Int32) — Version of the openNURBS archive to write. Must be [2; current version]. Rhino can read its current version, plus earlier file versions except 1. Use latest version when possible. Alternatively, 0 is a placeholder for the last valid version.

**Returns:** `Boolean` — true if archive is written with no error. false if errors occur.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dm_Write_1.htm)

#### `public static bool WriteMultipleObjects(string path, IEnumerable<GeometryBase> geometry)`

Creates a simple 3dm file that contains a multiple geometric objects.

**Parameters:**
- `path` (System.String) — Path to the 3dm file to create.
- `geometry` (System.Collections.Generic.IEnumerable<GeometryBase>) — The geometry to be saved in the archive's object table. This is typically some Curves, Surfaces, Breps, Meshs, or SubDs.

**Returns:** `Boolean` — True if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dm_WriteMultipleObjects.htm)

#### `public static bool WriteOneObject(string path, GeometryBase geometry)`

Creates a simple 3dm file that contains a single geometric object.

**Parameters:**
- `path` (System.String) — Path to the 3dm file to create.
- `geometry` (Rhino.Geometry.GeometryBase) — The geometry to be saved in the archive's object table. This is typically a Curve, Surface, Brep, Mesh, or SubD.

**Returns:** `Boolean` — True if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dm_WriteOneObject.htm)

#### `public bool WriteWithLog(string path, File3dmWriteOptions options, out string errorLog)`

Writes contents of this model to an openNURBS archive. If the model is not valid, then Write will refuse to write it.

**Parameters:**
- `path` (System.String) — The file name to use for writing.
- `options` (Rhino.FileIO.File3dmWriteOptions) — An options instance, or null for default.
- `errorLog` (System.String) — This argument will be filled by out reference.

**Returns:** `Boolean` — true if archive is written with no error. false if errors occur.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dm_WriteWithLog.htm)

#### `public bool WriteWithLog(string path, int version, out string errorLog)`

Writes contents of this model to an openNURBS archive. If the model is not valid, then Write will refuse to write it.

**Parameters:**
- `path` (System.String) — The file name to use for writing.
- `version` (System.Int32) — Version of the openNURBS archive to write. Must be [2; current version]. Rhino can read its current version, plus earlier file versions except 1. Use latest version when possible. Alternatively, 0 is a placeholder for the last valid version.
- `errorLog` (System.String) — This argument will be filled by out reference.

**Returns:** `Boolean` — true if archive is written with no error. false if errors occur.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dm_WriteWithLog_1.htm)

### Properties
- `AllDimStyles` (File3dmDimStyleTable, get) — Dimension Styles in this file.
- `AllGroups` (File3dmGroupTable, get) — Groups in this file.
- `AllHatchPatterns` (File3dmHatchPatternTable, get) — Hatch patterns in this file.
- `AllInstanceDefinitions` (File3dmInstanceDefinitionTable, get) — Instance definitions in this file
- `AllLayers` (File3dmLayerTable, get) — Layers in this file.
- `AllLinetypes` (File3dmLinetypeTable, get) — Linetypes in this file.
- `AllMaterials` (File3dmMaterialTable, get) — Materials in this file.
- `AllNamedConstructionPlanes` (File3dmNamedConstructionPlanes, get) — Named construction planes in this file.
- `AllNamedViews` (File3dmViewTable, get) — Named views in this file.
- `AllViews` (File3dmViewTable, get) — Views that represent the RhinoViews which are displayed when Rhino loads this file.
- `ApplicationDetails` (String, get/set) — Gets or sets details for the application that wrote this file.
- `ApplicationName` (String, get/set) — Gets or sets the name of the application that wrote this file.
- `ApplicationUrl` (String, get/set) — Gets or sets a URL for the application that wrote this file.
- `ArchiveVersion` (Int32, get) — Gets the 3dm file archive version.
- `Created` (DateTime, get) — Get the DateTime that this file was originally created. If the value is not set in the 3dm file, then DateTime.MinValue is returned
- `CreatedBy` (String, get) — Gets a string that names the user who created the file.
- `DimStyles` (IList<DimensionStyle>, get) — Dimension Styles in this file.
- `HatchPatterns` (IList<HatchPattern>, get) — Hatch patterns in this file
- `InstanceDefinitions` (IList<InstanceDefinitionGeometry>, get) — Instance definitions in this file.
- `LastEdited` (DateTime, get) — Get the DateTime that this file was last edited. If the value is not set in the 3dm file, then DateTime.MinValue is returned
- `LastEditedBy` (String, get) — Gets a string that names the user who last edited the file.
- `Layers` (IList<Layer>, get) — Layers in this file.
- `Linetypes` (IList<Linetype>, get) — Linetypes in this file.
- `Manifest` (ManifestTable, get) — Retrieves the manifest with all object descriptions in this file.
- `Materials` (IList<Material>, get) — Materials in this file.
- `NamedConstructionPlanes` (IList<ConstructionPlane>, get) — Named construction planes in this file.
- `NamedViews` (IList<ViewInfo>, get) — Named views in this file.
- `Notes` (File3dmNotes, get/set) — Gets or sets the model notes.
- `Objects` (File3dmObjectTable, get) — Gets access to the File3dmObjectTable class associated with this file, which contains all objects.
- `PlugInData` (File3dmPlugInDataTable, get) — Custom plug-in data in this file. This data is not attached to any geometry or attributes
- `Revision` (Int32, get/set) — Gets or sets the revision number.
- `Settings` (File3dmSettings, get) — Settings include tolerance, and unit system, and defaults used for creating views and objects.
- `StartSectionComments` (String, get/set) — Gets or sets the start section comments, which are the comments with which the 3dm file begins.
- `Strings` (File3dmStringTable, get) — Document user strings in this file
- `Views` (IList<ViewInfo>, get) — Views that represent the RhinoViews which are displayed when Rhino loads this file.

## File3dm.ObjectTypeFilter (enum)

[Missing <summary> documentation for "T:Rhino.FileIO.File3dm.ObjectTypeFilter"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_File3dm_ObjectTypeFilter.htm)

### Values
- `None` = `0`
- `Point` = `1` — some type of Point
- `Pointset` = `2` — some type of PointCloud, PointGrid, ...
- `Curve` = `4` — some type of Curve like LineCurve, NurbsCurve, etc.
- `Surface` = `8` — some type of Surface like PlaneSurface, NurbsSurface, etc.
- `Brep` = `16` — some type of Brep
- `Mesh` = `32` — some type of Mesh
- `Annotation` = `512` — some type of Annotation
- `InstanceDefinition` = `2048` — some type of InstanceDefinition
- `InstanceReference` = `4096` — some type of InstanceReference
- `TextDot` = `8192` — some type of TextDot
- `DetailView` = `32768` — some type of DetailView
- `Hatch` = `65536` — some type of Hatch
- `Extrusion` = `1073741824` — some type of Extrusion
- `Any` = `4294967295`

## File3dm.TableTypeFilter (enum)

[Missing <summary> documentation for "T:Rhino.FileIO.File3dm.TableTypeFilter"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_File3dm_TableTypeFilter.htm)

### Values
- `None` = `0`
- `Properties` = `1`
- `Settings` = `2`
- `Bitmap` = `4`
- `TextureMapping` = `8`
- `Material` = `16`
- `Linetype` = `32`
- `Layer` = `64`
- `Group` = `128`
- `Font` = `256`
- `FutureFont` = `512`
- `Dimstyle` = `1024`
- `Light` = `2048`
- `Hatchpattern` = `4096`
- `InstanceDefinition` = `8192`
- `ObjectTable` = `16384`
- `Historyrecord` = `32768`
- `UserTable` = `65536`

## File3dmCommonComponentTable<T> (class)

Provides a base table type that is shared among all File3dm tables.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_File3dmCommonComponentTable_1.htm)

### Methods
#### `public virtual void Add(T item)`

Adds an item.

**Parameters:**
- `item` (T) — The item to add.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Add.htm)

#### `public void Clear()`

Marks all items as deleted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_Clear.htm)

#### `public virtual void Delete(int index)`

Flags a component as deleted.

**Parameters:**
- `index` (System.Int32) — The index of the item to flag.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Delete.htm)

#### `public override bool Delete(T item)`

Flags a component as deleted.

**Parameters:**
- `item` (T) — The item to flag.

**Returns:** `Boolean` — True on success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Delete_1.htm)

#### `public string Dump()`

Prepares a text dump of object table.

**Returns:** `String` — A string containing the dump.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Dump.htm)

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

## File3dmDimStyleTable (class)

Provides access to annotation styles in the 3dm file.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_File3dmDimStyleTable.htm)

### Methods
#### `public virtual void Add(T item)`

Adds an item.

**Parameters:**
- `item` (T) — The item to add.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Add.htm)

#### `public void Clear()`

Marks all items as deleted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_Clear.htm)

#### `public virtual void Delete(int index)`

Flags a component as deleted.

**Parameters:**
- `index` (System.Int32) — The index of the item to flag.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Delete.htm)

#### `public override bool Delete(T item)`

Flags a component as deleted.

**Parameters:**
- `item` (T) — The item to flag.

**Returns:** `Boolean` — True on success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Delete_1.htm)

#### `public string Dump()`

Prepares a text dump of object table.

**Returns:** `String` — A string containing the dump.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Dump.htm)

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

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmDimStyleTable_FindIndex.htm)

#### `public DimensionStyle FindName(string name)`

Finds a DimensionStyle given its name.

**Parameters:**
- `name` (System.String) — The name of the DimensionStyle to be searched.

**Returns:** `DimensionStyle` — An DimensionStyle, or null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmDimStyleTable_FindName.htm)

#### `public DimensionStyle FindNameHash(NameHash nameHash)`

Finds a DimensionStyle given its name hash.

**Parameters:**
- `nameHash` (Rhino.FileIO.NameHash) — The name hash of the DimensionStyle to be searched.

**Returns:** `DimensionStyle` — An DimensionStyle, or null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmDimStyleTable_FindNameHash.htm)

#### `public virtual IEnumerator<T> GetEnumerator()`

Returns the enumerator that yields all items.

**Returns:** `IEnumerator<T>` — [Missing <returns> documentation for "M:Rhino.FileIO.CommonComponentTable`1.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_GetEnumerator.htm)

### Properties
- `ComponentType` (ModelComponentType, get) — Returns DimStyle.
- `Count` (Int32, get) — Returns the count of all items, including deleted ones.

## File3dmGroupTable (class)

Provides access to groups in the 3dm file.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_File3dmGroupTable.htm)

### Methods
#### `public virtual void Add(T item)`

Adds an item.

**Parameters:**
- `item` (T) — The item to add.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Add.htm)

#### `public void Clear()`

Marks all items as deleted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_Clear.htm)

#### `public virtual void Delete(int index)`

Flags a component as deleted.

**Parameters:**
- `index` (System.Int32) — The index of the item to flag.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Delete.htm)

#### `public override bool Delete(T item)`

Flags a component as deleted.

**Parameters:**
- `item` (T) — The item to flag.

**Returns:** `Boolean` — True on success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Delete_1.htm)

#### `public string Dump()`

Prepares a text dump of object table.

**Returns:** `String` — A string containing the dump.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Dump.htm)

#### `public virtual T FindId(Guid id)`

Uses the guid to find a model component. Deleted objects cannot be found by id. The guid is the value that is stored in the .Id property. In a single document, no two active objects have the same guid. If an object is replaced with a new object, then the guid persists. For example, if the _Move command moves an object, then the moved object inherits its guid from the starting object. If the Copy command copies an object, then the copy gets a new guid. This guid persists through file saving/opening operations. This function will not find grip objects.

**Parameters:**
- `id` (System.Guid) — ID of model component to search for.

**Returns:** `T` — Reference to the rhino object with the objectId or null if no such object could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_FindId.htm)

#### `public Group FindIndex(int groupIndex)`

Retrieves a Group object based on an index. This search type of search is discouraged. We are moving towards using only IDs for all tables.

**Parameters:**
- `groupIndex` (System.Int32) — The index to search for.

**Returns:** `Group` — A Group object, or null if none was found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmGroupTable_FindIndex.htm)

#### `public Group FindName(string name)`

Finds a Group given its name.

**Parameters:**
- `name` (System.String) — The name of the Group to be searched.

**Returns:** `Group` — A Group, or null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmGroupTable_FindName.htm)

#### `public Group FindNameHash(NameHash nameHash)`

Finds a Group given its name hash.

**Parameters:**
- `nameHash` (Rhino.FileIO.NameHash) — The name hash of the Group to be searched.

**Returns:** `Group` — A Group, or null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmGroupTable_FindNameHash.htm)

#### `public virtual IEnumerator<T> GetEnumerator()`

Returns the enumerator that yields all items.

**Returns:** `IEnumerator<T>` — [Missing <returns> documentation for "M:Rhino.FileIO.CommonComponentTable`1.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_GetEnumerator.htm)

#### `public File3dmObject[] GroupMembers(int groupIndex)`

Gets an array of all of the objects in a group.

**Parameters:**
- `groupIndex` (System.Int32) — The index of the group in this table.

**Returns:** `File3dmObject[]` — Array of objects that belong to the specified group or empty array if no objects could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmGroupTable_GroupMembers.htm)

### Properties
- `ComponentType` (ModelComponentType, get) — Returns Group.
- `Count` (Int32, get) — Returns the count of all items, including deleted ones.

## File3dmHatchPatternTable (class)

Provides access to hatch pattern definitions in the 3dm file.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_File3dmHatchPatternTable.htm)

### Methods
#### `public virtual void Add(T item)`

Adds an item.

**Parameters:**
- `item` (T) — The item to add.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Add.htm)

#### `public void Clear()`

Marks all items as deleted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_Clear.htm)

#### `public virtual void Delete(int index)`

Flags a component as deleted.

**Parameters:**
- `index` (System.Int32) — The index of the item to flag.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Delete.htm)

#### `public override bool Delete(T item)`

Flags a component as deleted.

**Parameters:**
- `item` (T) — The item to flag.

**Returns:** `Boolean` — True on success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Delete_1.htm)

#### `public string Dump()`

Prepares a text dump of object table.

**Returns:** `String` — A string containing the dump.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Dump.htm)

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

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmHatchPatternTable_FindIndex.htm)

#### `public HatchPattern FindName(string name)`

Finds a HatchPattern given its name.

**Parameters:**
- `name` (System.String) — The name of the HatchPattern to be searched.

**Returns:** `HatchPattern` — An HatchPattern, or null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmHatchPatternTable_FindName.htm)

#### `public HatchPattern FindNameHash(NameHash nameHash)`

Finds a HatchPattern given its name hash.

**Parameters:**
- `nameHash` (Rhino.FileIO.NameHash) — The name hash of the HatchPattern to be searched.

**Returns:** `HatchPattern` — An HatchPattern, or null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmHatchPatternTable_FindNameHash.htm)

#### `public virtual IEnumerator<T> GetEnumerator()`

Returns the enumerator that yields all items.

**Returns:** `IEnumerator<T>` — [Missing <returns> documentation for "M:Rhino.FileIO.CommonComponentTable`1.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_GetEnumerator.htm)

### Properties
- `ComponentType` (ModelComponentType, get) — Returns HatchPattern.
- `Count` (Int32, get) — Returns the count of all items, including deleted ones.

## File3dmInstanceDefinitionTable (class)

Provides access to instance (block) definitions in the 3dm file.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_File3dmInstanceDefinitionTable.htm)

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

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmInstanceDefinitionTable_Add.htm)

#### `public int Add(string name, string description, Point3d basePoint, IEnumerable<GeometryBase> geometry)`

Adds an instance definition to the instance definition table.

**Parameters:**
- `name` (System.String) — The definition name.
- `description` (System.String) — The definition description.
- `basePoint` (Rhino.Geometry.Point3d) — A base point.
- `geometry` (System.Collections.Generic.IEnumerable<GeometryBase>) — An array, a list or any enumerable set of geometry.

**Returns:** `Int32` — >=0 index of instance definition in the instance definition table. -1 on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmInstanceDefinitionTable_Add_1.htm)

#### `public int Add(string name, string description, Point3d basePoint, IEnumerable<GeometryBase> geometry, IEnumerable<ObjectAttributes> attributes)`

Adds an instance definition to the instance definition table.

**Parameters:**
- `name` (System.String) — The definition name.
- `description` (System.String) — The definition description.
- `basePoint` (Rhino.Geometry.Point3d) — A base point.
- `geometry` (System.Collections.Generic.IEnumerable<GeometryBase>) — An array, a list or any enumerable set of geometry.
- `attributes` (System.Collections.Generic.IEnumerable<ObjectAttributes>) — An array, a list or any enumerable set of attributes.

**Returns:** `Int32` — >=0 index of instance definition in the instance definition table. -1 on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmInstanceDefinitionTable_Add_2.htm)

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

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmInstanceDefinitionTable_Add_3.htm)

#### `public virtual void Add(T item)`

Adds an item.

**Parameters:**
- `item` (T) — The item to add.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Add.htm)

#### `public int AddLinked(string filename, string name, string description)`

Adds a linked instance definition to the instance definition table.

**Parameters:**
- `filename` (System.String) — Full path of the file to link.
- `name` (System.String) — The definition name.
- `description` (System.String) — The definition description.

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.FileIO.File3dmInstanceDefinitionTable.AddLinked(System.String,System.String,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmInstanceDefinitionTable_AddLinked.htm)

#### `public void Clear()`

Marks all items as deleted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_Clear.htm)

#### `public virtual void Delete(int index)`

Flags a component as deleted.

**Parameters:**
- `index` (System.Int32) — The index of the item to flag.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Delete.htm)

#### `public override bool Delete(T item)`

Flags a component as deleted.

**Parameters:**
- `item` (T) — The item to flag.

**Returns:** `Boolean` — True on success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Delete_1.htm)

#### `public string Dump()`

Prepares a text dump of object table.

**Returns:** `String` — A string containing the dump.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Dump.htm)

#### `public virtual T FindId(Guid id)`

Uses the guid to find a model component. Deleted objects cannot be found by id. The guid is the value that is stored in the .Id property. In a single document, no two active objects have the same guid. If an object is replaced with a new object, then the guid persists. For example, if the _Move command moves an object, then the moved object inherits its guid from the starting object. If the Copy command copies an object, then the copy gets a new guid. This guid persists through file saving/opening operations. This function will not find grip objects.

**Parameters:**
- `id` (System.Guid) — ID of model component to search for.

**Returns:** `T` — Reference to the rhino object with the objectId or null if no such object could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_FindId.htm)

#### `public InstanceDefinitionGeometry FindName(string name)`

Finds an InstanceDefinitionGeometry given its name.

**Parameters:**
- `name` (System.String) — The name of the InstanceDefinitionGeometry to be searched.

**Returns:** `InstanceDefinitionGeometry` — An InstanceDefinitionGeometry, or null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmInstanceDefinitionTable_FindName.htm)

#### `public InstanceDefinitionGeometry FindNameHash(NameHash nameHash)`

Finds a InstanceDefinitionGeometry given its name hash.

**Parameters:**
- `nameHash` (Rhino.FileIO.NameHash) — The name hash of the InstanceDefinitionGeometry to be searched.

**Returns:** `InstanceDefinitionGeometry` — An InstanceDefinitionGeometry, or null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmInstanceDefinitionTable_FindNameHash.htm)

#### `public virtual IEnumerator<T> GetEnumerator()`

Returns the enumerator that yields all items.

**Returns:** `IEnumerator<T>` — [Missing <returns> documentation for "M:Rhino.FileIO.CommonComponentTable`1.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_GetEnumerator.htm)

### Properties
- `ComponentType` (ModelComponentType, get) — Returns InstanceDefinition.
- `Count` (Int32, get) — Returns the count of all items, including deleted ones.

## File3dmLayerTable (class)

Provides access to layers in the 3dm file.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_File3dmLayerTable.htm)

### Methods
#### `public virtual void Add(T item)`

Adds an item.

**Parameters:**
- `item` (T) — The item to add.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Add.htm)

#### `public int AddLayer(string name, Color color)`

Easy way to add a layer to the model

**Parameters:**
- `name` (System.String) — new layer name
- `color` (System.Drawing.Color) — new layer color

**Returns:** `Int32` — If layer_name is valid, the layer's index (>=0) is returned. Otherwise, RhinoMath.UnsetIntIndex is returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmLayerTable_AddLayer.htm)

#### `public void Clear()`

Marks all items as deleted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_Clear.htm)

#### `public virtual void Delete(int index)`

Flags a component as deleted.

**Parameters:**
- `index` (System.Int32) — The index of the item to flag.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Delete.htm)

#### `public override bool Delete(T item)`

Flags a component as deleted.

**Parameters:**
- `item` (T) — The item to flag.

**Returns:** `Boolean` — True on success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Delete_1.htm)

#### `public string Dump()`

Prepares a text dump of object table.

**Returns:** `String` — A string containing the dump.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Dump.htm)

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

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmLayerTable_FindIndex.htm)

#### `public Layer FindName(string name, Guid parentId)`

Finds a Layer given its name.

**Parameters:**
- `name` (System.String) — The name of the Layer to be searched.
- `parentId` (System.Guid) — The id of the parent Layer to be searched.

**Returns:** `Layer` — A Layer, or null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmLayerTable_FindName.htm)

#### `public Layer FindNameHash(NameHash nameHash)`

Finds a Layer given its name hash.

**Parameters:**
- `nameHash` (Rhino.FileIO.NameHash) — The name hash of the Layer to be searched.

**Returns:** `Layer` — An Layer, or null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmLayerTable_FindNameHash.htm)

#### `public virtual IEnumerator<T> GetEnumerator()`

Returns the enumerator that yields all items.

**Returns:** `IEnumerator<T>` — [Missing <returns> documentation for "M:Rhino.FileIO.CommonComponentTable`1.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_GetEnumerator.htm)

### Properties
- `ComponentType` (ModelComponentType, get) — Returns Layer.
- `Count` (Int32, get) — Returns the count of all items, including deleted ones.

## File3dmLinetypeTable (class)

Provides access to Linetypes in the 3dm file.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_File3dmLinetypeTable.htm)

### Methods
#### `public virtual void Add(T item)`

Adds an item.

**Parameters:**
- `item` (T) — The item to add.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Add.htm)

#### `public void Clear()`

Marks all items as deleted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_Clear.htm)

#### `public virtual void Delete(int index)`

Flags a component as deleted.

**Parameters:**
- `index` (System.Int32) — The index of the item to flag.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Delete.htm)

#### `public override bool Delete(T item)`

Flags a component as deleted.

**Parameters:**
- `item` (T) — The item to flag.

**Returns:** `Boolean` — True on success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Delete_1.htm)

#### `public string Dump()`

Prepares a text dump of object table.

**Returns:** `String` — A string containing the dump.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Dump.htm)

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

**Returns:** `Linetype` — A Linetype, or null if none was found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmLinetypeTable_FindIndex.htm)

#### `public Linetype FindName(string name)`

Finds a Linetype given its name.

**Parameters:**
- `name` (System.String) — The name of the Linetype to be searched.

**Returns:** `Linetype` — A Linetype, or null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmLinetypeTable_FindName.htm)

#### `public Linetype FindNameHash(NameHash nameHash)`

Finds a Linetype given its name hash.

**Parameters:**
- `nameHash` (Rhino.FileIO.NameHash) — The name hash of the Linetype to be searched.

**Returns:** `Linetype` — An Linetype, or null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmLinetypeTable_FindNameHash.htm)

#### `public virtual IEnumerator<T> GetEnumerator()`

Returns the enumerator that yields all items.

**Returns:** `IEnumerator<T>` — [Missing <returns> documentation for "M:Rhino.FileIO.CommonComponentTable`1.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_GetEnumerator.htm)

### Properties
- `ComponentType` (ModelComponentType, get) — Returns LinePattern.
- `Count` (Int32, get) — Returns the count of all items, including deleted ones.

## File3dmMaterialTable (class)

Provides access to materials in the 3dm file.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_File3dmMaterialTable.htm)

### Methods
#### `public virtual void Add(T item)`

Adds an item.

**Parameters:**
- `item` (T) — The item to add.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Add.htm)

#### `public void Clear()`

Marks all items as deleted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_Clear.htm)

#### `public virtual void Delete(int index)`

Flags a component as deleted.

**Parameters:**
- `index` (System.Int32) — The index of the item to flag.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Delete.htm)

#### `public override bool Delete(T item)`

Flags a component as deleted.

**Parameters:**
- `item` (T) — The item to flag.

**Returns:** `Boolean` — True on success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Delete_1.htm)

#### `public string Dump()`

Prepares a text dump of object table.

**Returns:** `String` — A string containing the dump.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Dump.htm)

#### `public virtual T FindId(Guid id)`

Uses the guid to find a model component. Deleted objects cannot be found by id. The guid is the value that is stored in the .Id property. In a single document, no two active objects have the same guid. If an object is replaced with a new object, then the guid persists. For example, if the _Move command moves an object, then the moved object inherits its guid from the starting object. If the Copy command copies an object, then the copy gets a new guid. This guid persists through file saving/opening operations. This function will not find grip objects.

**Parameters:**
- `id` (System.Guid) — ID of model component to search for.

**Returns:** `T` — Reference to the rhino object with the objectId or null if no such object could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_FindId.htm)

#### `public Material FindIndex(int index)`

Retrieves a material based on Index. This search type of search is discouraged. We are moving towards using only IDs for all tables.

**Parameters:**
- `index` (System.Int32) — The index to search for.

**Returns:** `Material` — A material, or null if none was found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmMaterialTable_FindIndex.htm)

#### `public virtual IEnumerator<T> GetEnumerator()`

Returns the enumerator that yields all items.

**Returns:** `IEnumerator<T>` — [Missing <returns> documentation for "M:Rhino.FileIO.CommonComponentTable`1.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_GetEnumerator.htm)

### Properties
- `ComponentType` (ModelComponentType, get) — Returns RenderMaterial.
- `Count` (Int32, get) — Returns the count of all items, including deleted ones.

## File3dmNamedConstructionPlanes (class)

Provides access to named construction planes in the 3dm file.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_File3dmNamedConstructionPlanes.htm)

### Methods
#### `public void Add(ConstructionPlane cplane)`

Adds a named construction plane to the table.

**Parameters:**
- `cplane` (Rhino.DocObjects.ConstructionPlane) — The construction plane to add.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmNamedConstructionPlanes_Add.htm)

#### `public int Add(string name, Plane plane)`

Adds a named construction plane to the table.

**Parameters:**
- `name` (System.String) — The name of the named construction plane.
- `plane` (Rhino.Geometry.Plane) — The plane value.

**Returns:** `Int32` — 0 based index of the named construction plane. -1 on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmNamedConstructionPlanes_Add_1.htm)

#### `public void Clear()`

Removes all named construction planes from the table.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmNamedConstructionPlanes_Clear.htm)

#### `public bool Contains(ConstructionPlane cplane)`

Returns an indication of the presence of a named construction plane in the table.

**Parameters:**
- `cplane` (Rhino.DocObjects.ConstructionPlane) — The construction plane to check.

**Returns:** `Boolean` — true if the named construction plane is in the table; false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmNamedConstructionPlanes_Contains.htm)

#### `public void CopyTo(ConstructionPlane[] array, int arrayIndex)`

Copies the content of the table to an array.

**Parameters:**
- `array` (Rhino.DocObjects.ConstructionPlane[]) — [Missing <param name="array"/> documentation for "M:Rhino.FileIO.File3dmNamedConstructionPlanes.CopyTo(Rhino.DocObjects.ConstructionPlane[],System.Int32)"]
- `arrayIndex` (System.Int32) — [Missing <param name="arrayIndex"/> documentation for "M:Rhino.FileIO.File3dmNamedConstructionPlanes.CopyTo(Rhino.DocObjects.ConstructionPlane[],System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmNamedConstructionPlanes_CopyTo.htm)

#### `public bool Delete(ConstructionPlane cplane)`

Deletes a named construction plane from the table.

**Parameters:**
- `cplane` (Rhino.DocObjects.ConstructionPlane) — The construction plane to delete.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.FileIO.File3dmNamedConstructionPlanes.Delete(Rhino.DocObjects.ConstructionPlane)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmNamedConstructionPlanes_Delete.htm)

#### `public bool Delete(int index)`

Remove a named construction plane from the table.

**Parameters:**
- `index` (System.Int32) — Zero based array index.

**Returns:** `Boolean` — true if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmNamedConstructionPlanes_Delete_1.htm)

#### `public ConstructionPlane FindName(string name)`

Finds a named construction plane given its name.

**Parameters:**
- `name` (System.String) — The name of the construction plane to be searched.

**Returns:** `ConstructionPlane` — A ConstructionPlane, or null if not found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmNamedConstructionPlanes_FindName.htm)

#### `public IEnumerator<ConstructionPlane> GetEnumerator()`

Gets an enumerator that yields all construction planes in this collection.

**Returns:** `IEnumerator<ConstructionPlane>` — The enumerator.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmNamedConstructionPlanes_GetEnumerator.htm)

#### `public int IndexOf(ConstructionPlane cplane)`

Returns the index of a named construction plane.

**Parameters:**
- `cplane` (Rhino.DocObjects.ConstructionPlane) — The construction plane to be searched.

**Returns:** `Int32` — The index of the named construction plane, -1 if not found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmNamedConstructionPlanes_IndexOf.htm)

### Properties
- `Count` (Int32, get) — Number of named construction planes in the table.
- `Item` (ConstructionPlane, get) — Gets the named construction plane at an index.

## File3dmNotes (class)

Represents the notes information stored in a 3dm file.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_File3dmNotes.htm)

### Constructors
- `public File3dmNotes()` — Creates empty default notes

### Properties
- `IsHtml` (Boolean, get/set) — Gets or sets the text format. If the format is HTML, true; false otherwise.
- `IsVisible` (Boolean, get/set) — Gets or sets the notes visibility. If the notes are visible, true; false otherwise.
- `Notes` (String, get/set) — Gets or sets the text content of the notes.
- `WindowRectangle` (Rectangle, get/set) — Gets or sets the position of the Notes when they were saved.

## File3dmObject (class)

Used to store geometry table object definition and attributes in a File3dm.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_File3dmObject.htm)

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

#### `public bool Equals(File3dmObject other)`

Verified that two File3dmObject items refer to the same object in a document.

**Parameters:**
- `other` (Rhino.FileIO.File3dmObject) — The other item to test.

**Returns:** `Boolean` — true is the two objects coincide.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObject_Equals.htm)

#### `public override bool Equals(Object obj)`

Verified that two objects refer to the same object in a document.

**Parameters:**
- `obj` (System.Object) — The other item to test.

**Returns:** `Boolean` — true is the two objects coincide.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObject_Equals_1.htm)

#### `public override int GetHashCode()`

Provides an hash code for this item.

**Returns:** `Int32` — The hash code.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObject_GetHashCode.htm)

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

#### `public bool TryReadUserData(Guid userDataId, bool readFromAttributes, Func<File3dm, BinaryArchiveReader, bool> dataReader)`

Attempts to read a Rhino plug-in's custom userdata from the File3dmObject object.

**Parameters:**
- `userDataId` (System.Guid) — The id of the custom userdata object whose data you want to try to read
- `readFromAttributes` (System.Boolean) — Set true to attempt to read custom userdata object from the object's Attributes. Set false to attempt to read custom userdata object from the object's Geometry.
- `dataReader` (System.Func<File3dm,BinaryArchiveReader,Boolean>) — The function that will read the data. This function must be implemented identical to the the originating UserData-inherited class's Read method.

**Returns:** `Boolean` — The value returned by the data reading function if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObject_TryReadUserData.htm)

### Properties
- `Attributes` (ObjectAttributes, get) — Gets the attributes that are linked with this document object.
- `ComponentStatus` (ComponentStatus, get/set) — Gets or sets the component status of the model component.
- `ComponentType` (ModelComponentType, get) — Returns ModelGeometry.
- `DeletedName` (String, get) — Gets the name of a component that is deleted.
- `Disposed` (Boolean, get) — Indicates if this object has been disposed or the document it originally belonged to has been disposed.
- `Geometry` (GeometryBase, get) — Gets the geometry that is linked with this document object.
- `HasId` (Boolean, get) — Returns a value indicating whether the component has an ID.
- `HasIndex` (Boolean, get) — Returns a value indicating whether the component has an Index.
- `HasName` (Boolean, get) — Returns a value indicating whether the component has a Name.
- `HasUserData` (Boolean, get) — Gets true if this class has any custom information attached to it through UserData.
- `Id` (Guid, get/set) — Gets or sets the ID of the current instance.
- `IdIsLocked` (Boolean, get) — Returns a value indicating whether the component ID is already locked.
- `Index` (Int32, get/set) — Gets or sets the model component index attribute.
- `IndexIsLocked` (Boolean, get) — Returns a value indicating whether the component Index is already locked.
- `InstanceDefinitionModelSerialNumber` (UInt32, get) — When a component is in a model as part of the information required for a linked instance definition, this value identifies the linked instance definition reference model.
- `IsComponentStatusLocked` (Boolean, get) — The component status itself can be locked. This returns an indication.
- `IsDocumentControlled` (Boolean, get) — If true this object may not be modified. Any properties or functions that attempt to modify this object when it is set to "IsReadOnly" will throw a NotSupportedException.
- `IsSystemComponent` (Boolean, get) — True if this model component is a system constant. An incomplete list of system constant model components is below:ON_ModelComponent::Unset ON_InstanceDefinition::Empty ON_Linetype::UnsetON_Linetype::ContinuousON_Linetype::ByLayerON_Linetype::ByParent ON_Layer::UnsetON_Layer::Default ON_TextStyle::UnsetON_TextStyle::DefaultON_TextStyle::ByLayerON_TextStyle::ByParent ON_DimStyle::UnsetON_DimStyle::DefaultON_DimStyle::DefaultInchDecimalON_DimStyle::DefaultInchFractionalON_DimStyle::DefaultFootInchArchitectureON_DimStyle::DefaultMillimeterSmallON_DimStyle::DefaultMillimeterLargeON_DimStyle::DefaultMillimeterArchitecture
- `IsValid` (Boolean, get) — Tests an object to see if it is valid.
- `ModelSerialNumber` (UInt32, get) — A value identifying the model that manages this component.
- `Name` (String, get/set) — Gets or sets the Name of the object. Equivalent to this.Attributes.Name.
- `NameIsLocked` (Boolean, get) — Returns a value indicating whether the component Name is already locked.
- `ReferenceModelSerialNumber` (UInt32, get) — When a component is in a model for reference, this value identifies the reference model.
- `UserData` (UserDataList, get) — List of custom information that is attached to this class.
- `UserDictionary` (ArchivableDictionary, get) — Dictionary of custom information attached to this class. The dictionary is actually user data provided as an easy to use sharable set of information.

## File3dmObjectTable (class)

Represents a simple object table for a file that is open externally. This class mimics Rhino.DocObjects.Tables.ObjectTable while providing external access to the file.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_File3dmObjectTable.htm)

### Methods
#### `public override void Add(File3dmObject item)`

Duplicates the object, then adds a copy of the object to the document.

**Parameters:**
- `item` (Rhino.FileIO.File3dmObject) — The item to duplicate and add.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_Add.htm)

#### `public Guid AddAngularDimension(AngularDimension dimension)`

Adds a angular dimension object to the 3dm file object table.

**Parameters:**
- `dimension` (Rhino.Geometry.AngularDimension) — Dimension object to add.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddAngularDimension.htm)

#### `public Guid AddAngularDimension(AngularDimension dimension, ObjectAttributes attributes)`

Adds a angular dimension object to the 3dm file object table.

**Parameters:**
- `dimension` (Rhino.Geometry.AngularDimension) — Dimension object to add.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to dimension.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddAngularDimension_1.htm)

#### `public Guid AddArc(Arc arc)`

Adds a curve object to the document representing an arc.

**Parameters:**
- `arc` (Rhino.Geometry.Arc) — An arc.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddArc.htm)

#### `public Guid AddArc(Arc arc, ObjectAttributes attributes)`

Adds a curve object to the document representing an arc.

**Parameters:**
- `arc` (Rhino.Geometry.Arc) — An arc to add.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — attributes to apply to arc.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddArc_1.htm)

#### `public Guid AddBrep(Brep brep)`

Adds a brep object to Rhino.

**Parameters:**
- `brep` (Rhino.Geometry.Brep) — A duplicate of this brep is added to Rhino.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddBrep.htm)

#### `public Guid AddBrep(Brep brep, ObjectAttributes attributes)`

Adds a brep object to Rhino.

**Parameters:**
- `brep` (Rhino.Geometry.Brep) — A duplicate of this brep is added to Rhino.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to brep.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddBrep_1.htm)

#### `public Guid AddCircle(Circle circle)`

Adds a curve object to the document representing a circle.

**Parameters:**
- `circle` (Rhino.Geometry.Circle) — A circle to add.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddCircle.htm)

#### `public Guid AddCircle(Circle circle, ObjectAttributes attributes)`

Adds a curve object to the document representing a circle.

**Parameters:**
- `circle` (Rhino.Geometry.Circle) — A circle to add.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — attributes to apply to circle.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddCircle_1.htm)

#### `public Guid AddClippingPlane(Plane plane, double uMagnitude, double vMagnitude, Guid clippedViewportId)`

Adds a clipping plane object to Rhino.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — A plane.
- `uMagnitude` (System.Double) — The size in U direction.
- `vMagnitude` (System.Double) — The size in V direction.
- `clippedViewportId` (System.Guid) — The viewport id that the new clipping plane will clip.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddClippingPlane_2.htm)

#### `public Guid AddClippingPlane(Plane plane, double uMagnitude, double vMagnitude, IEnumerable<Guid> clippedViewportIds)`

Adds a clipping plane object to Rhino.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — A plane.
- `uMagnitude` (System.Double) — The size in U direction.
- `vMagnitude` (System.Double) — The size in V direction.
- `clippedViewportIds` (System.Collections.Generic.IEnumerable<Guid>) — A list, an array or any enumerable of viewport ids that the new clipping plane will clip.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddClippingPlane.htm)

#### `public Guid AddClippingPlane(Plane plane, double uMagnitude, double vMagnitude, IEnumerable<Guid> clippedViewportIds, ObjectAttributes attributes)`

Adds a clipping plane object to Rhino.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — A plane.
- `uMagnitude` (System.Double) — The size in U direction.
- `vMagnitude` (System.Double) — The size in V direction.
- `clippedViewportIds` (System.Collections.Generic.IEnumerable<Guid>) — list of viewport ids that the new clipping plane will clip.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to point cloud.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddClippingPlane_1.htm)

#### `public Guid AddCurve(Curve curve)`

Adds a curve object to the table.

**Parameters:**
- `curve` (Rhino.Geometry.Curve) — A curve to add.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddCurve.htm)

#### `public Guid AddCurve(Curve curve, ObjectAttributes attributes)`

Adds a curve object to the table.

**Parameters:**
- `curve` (Rhino.Geometry.Curve) — A duplicate of this curve is added to Rhino.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to curve.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddCurve_1.htm)

#### `public Guid AddEllipse(Ellipse ellipse)`

Adds a curve object to the document representing an ellipse.

**Parameters:**
- `ellipse` (Rhino.Geometry.Ellipse) — An ellipse to add.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddEllipse.htm)

#### `public Guid AddEllipse(Ellipse ellipse, ObjectAttributes attributes)`

Adds a curve object to the document representing an ellipse.

**Parameters:**
- `ellipse` (Rhino.Geometry.Ellipse) — An ellipse to add.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — attributes to apply to ellipse.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddEllipse_1.htm)

#### `public Guid AddExtrusion(Extrusion extrusion)`

Adds an extrusion object to Rhino.

**Parameters:**
- `extrusion` (Rhino.Geometry.Extrusion) — A duplicate of this extrusion is added to Rhino.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddExtrusion.htm)

#### `public Guid AddExtrusion(Extrusion extrusion, ObjectAttributes attributes)`

Adds an extrusion object to Rhino.

**Parameters:**
- `extrusion` (Rhino.Geometry.Extrusion) — A duplicate of this extrusion is added to Rhino.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to link to the object.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddExtrusion_1.htm)

#### `public Guid AddHatch(Hatch hatch)`

Adds a hatch to the document.

**Parameters:**
- `hatch` (Rhino.Geometry.Hatch) — A hatch.

**Returns:** `Guid` — A unique identifier for the hatch, or Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddHatch.htm)

#### `public Guid AddHatch(Hatch hatch, ObjectAttributes attributes)`

Adds a hatch to the document.

**Parameters:**
- `hatch` (Rhino.Geometry.Hatch) — A hatch.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply

**Returns:** `Guid` — A unique identifier for the hatch, or Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddHatch_1.htm)

#### `public Guid AddInstanceObject(InstanceReferenceGeometry instanceReference)`

Adds an instance reference geometry object to the table.

**Parameters:**
- `instanceReference` (Rhino.Geometry.InstanceReferenceGeometry) — The instance reference geometry object.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddInstanceObject.htm)

#### `public Guid AddInstanceObject(InstanceReferenceGeometry instanceReference, ObjectAttributes attributes)`

Adds an instance reference geometry object to the table.

**Parameters:**
- `instanceReference` (Rhino.Geometry.InstanceReferenceGeometry) — The instance reference geometry object.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — The attributes to link with the object.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddInstanceObject_1.htm)

#### `public Guid AddInstanceObject(int instanceDefinitionIndex, Transform instanceXform)`

Adds an instance reference geometry object to the table.

**Parameters:**
- `instanceDefinitionIndex` (System.Int32) — The index of the instance definition geometry object.
- `instanceXform` (Rhino.Geometry.Transform) — The transformation.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddInstanceObject_2.htm)

#### `public Guid AddInstanceObject(int instanceDefinitionIndex, Transform instanceXform, ObjectAttributes attributes)`

Adds an instance reference geometry object to the table.

**Parameters:**
- `instanceDefinitionIndex` (System.Int32) — The index of the instance definition geometry object.
- `instanceXform` (Rhino.Geometry.Transform) — The transformation.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — The object attributes.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddInstanceObject_3.htm)

#### `public Guid AddLeader(IEnumerable<Point3d> points)`

Adds an annotation leader to the document. This overload is only provided in the Rhino SDK.

**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — A list, an array or any enumerable set of 2d points.

**Returns:** `Guid` — A unique identifier for the object; or Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddLeader_2.htm)

#### `public Guid AddLeader(Plane plane, IEnumerable<Point2d> points)`

Adds an annotation leader to the document.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — A plane.
- `points` (System.Collections.Generic.IEnumerable<Point2d>) — A list, an array or any enumerable set of 2d points.

**Returns:** `Guid` — A unique identifier for the object; or Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddLeader.htm)

#### `public Guid AddLeader(Plane plane, IEnumerable<Point2d> points, ObjectAttributes attributes)`

Adds an annotation leader to the document.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — A plane.
- `points` (System.Collections.Generic.IEnumerable<Point2d>) — A list, an array or any enumerable set of 2d points.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to brep.

**Returns:** `Guid` — A unique identifier for the object; or Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddLeader_1.htm)

#### `public Guid AddLeader(string text, IEnumerable<Point3d> points)`

Adds an annotation leader to the document. This overload is only provided in the Rhino SDK.

**Parameters:**
- `text` (System.String) — The text.
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — A list, an array or any enumerable set of 2d points.

**Returns:** `Guid` — A unique identifier for the object; or Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddLeader_5.htm)

#### `public Guid AddLeader(string text, Plane plane, IEnumerable<Point2d> points)`

Adds an annotation leader to the document.

**Parameters:**
- `text` (System.String) — The text.
- `plane` (Rhino.Geometry.Plane) — A plane.
- `points` (System.Collections.Generic.IEnumerable<Point2d>) — A list, an array or any enumerable set of 2d points.

**Returns:** `Guid` — A unique identifier for the object; or Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddLeader_3.htm)

#### `public Guid AddLeader(string text, Plane plane, IEnumerable<Point2d> points, ObjectAttributes attributes)`

Adds an annotation leader to the document.

**Parameters:**
- `text` (System.String) — The text.
- `plane` (Rhino.Geometry.Plane) — A plane.
- `points` (System.Collections.Generic.IEnumerable<Point2d>) — A list, an array or any enumerable set of 2d points.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to brep.

**Returns:** `Guid` — A unique identifier for the object; or Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddLeader_4.htm)

#### `public Guid AddLine(Line line)`

Adds a line object to Rhino.

**Parameters:**
- `line` (Rhino.Geometry.Line) — [Missing <param name="line"/> documentation for "M:Rhino.FileIO.File3dmObjectTable.AddLine(Rhino.Geometry.Line)"]

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddLine.htm)

#### `public Guid AddLine(Line line, ObjectAttributes attributes)`

Adds a line object to Rhino.

**Parameters:**
- `line` (Rhino.Geometry.Line) — A line.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to line.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddLine_1.htm)

#### `public Guid AddLine(Point3d from, Point3d to)`

Adds a line object to Rhino.

**Parameters:**
- `from` (Rhino.Geometry.Point3d) — A line start point.
- `to` (Rhino.Geometry.Point3d) — A line end point.

**Returns:** `Guid` — A unique identifier of new rhino object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddLine_2.htm)

#### `public Guid AddLine(Point3d from, Point3d to, ObjectAttributes attributes)`

Adds a line object to Rhino.

**Parameters:**
- `from` (Rhino.Geometry.Point3d) — The start point of the line.
- `to` (Rhino.Geometry.Point3d) — The end point of the line.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to line.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddLine_3.htm)

#### `public Guid AddLinearDimension(LinearDimension dimension)`

Adds a linear dimension to the 3dm file object table.

**Parameters:**
- `dimension` (Rhino.Geometry.LinearDimension) — A dimension.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddLinearDimension.htm)

#### `public Guid AddLinearDimension(LinearDimension dimension, ObjectAttributes attributes)`

Adds a linear dimension to the 3dm file object table.

**Parameters:**
- `dimension` (Rhino.Geometry.LinearDimension) — A dimension.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to dimension.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddLinearDimension_1.htm)

#### `public Guid AddMesh(Mesh mesh)`

Adds a mesh object to Rhino.

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — A duplicate of this mesh is added to Rhino.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddMesh.htm)

#### `public Guid AddMesh(Mesh mesh, ObjectAttributes attributes)`

Adds a mesh object to Rhino.

**Parameters:**
- `mesh` (Rhino.Geometry.Mesh) — A duplicate of this mesh is added to Rhino.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to link to the object.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddMesh_1.htm)

#### `public Guid AddPoint(double x, double y, double z)`

Adds a point object to the table.

**Parameters:**
- `x` (System.Double) — X component of point coordinate.
- `y` (System.Double) — Y component of point coordinate.
- `z` (System.Double) — Z component of point coordinate.

**Returns:** `Guid` — id of new object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddPoint_4.htm)

#### `public Guid AddPoint(Point3d point)`

Adds a point object to the table.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — A location for point.

**Returns:** `Guid` — Id of new object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddPoint.htm)

#### `public Guid AddPoint(Point3d point, ObjectAttributes attributes)`

Adds a point object to the document.

**Parameters:**
- `point` (Rhino.Geometry.Point3d) — A location for point.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — attributes to apply to point.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddPoint_1.htm)

#### `public Guid AddPoint(Point3f point)`

Adds a point object to the document.

**Parameters:**
- `point` (Rhino.Geometry.Point3f) — location of point.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddPoint_2.htm)

#### `public Guid AddPoint(Point3f point, ObjectAttributes attributes)`

Adds a point object to the document.

**Parameters:**
- `point` (Rhino.Geometry.Point3f) — location of point.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — attributes to apply to point.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddPoint_3.htm)

#### `public Guid AddPointCloud(IEnumerable<Point3d> points)`

Adds a point cloud object to the document.

**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — A list, an array or any enumerable set of Point3d.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddPointCloud_2.htm)

#### `public Guid AddPointCloud(IEnumerable<Point3d> points, ObjectAttributes attributes)`

Adds a point cloud object to the document.

**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — A list, an array or any enumerable set of Point3d.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to point cloud.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddPointCloud_3.htm)

#### `public Guid AddPointCloud(PointCloud cloud)`

Adds a point cloud object to the document.

**Parameters:**
- `cloud` (Rhino.Geometry.PointCloud) — PointCloud to add.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddPointCloud.htm)

#### `public Guid AddPointCloud(PointCloud cloud, ObjectAttributes attributes)`

Adds a point cloud object to the document.

**Parameters:**
- `cloud` (Rhino.Geometry.PointCloud) — PointCloud to add.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — attributes to apply to point cloud.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddPointCloud_1.htm)

#### `public Guid[] AddPoints(IEnumerable<Point3d> points)`

Adds multiple points to the document.

**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — Points to add.

**Returns:** `Guid[]` — List of object ids.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddPoints.htm)

#### `public Guid[] AddPoints(IEnumerable<Point3d> points, ObjectAttributes attributes)`

Adds multiple points to the document.

**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — Points to add.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to point objects.

**Returns:** `Guid[]` — An array of object unique identifiers.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddPoints_1.htm)

#### `public Guid[] AddPoints(IEnumerable<Point3f> points)`

Adds multiple points to the document.

**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3f>) — Points to add.

**Returns:** `Guid[]` — An array of object unique identifiers.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddPoints_2.htm)

#### `public Guid[] AddPoints(IEnumerable<Point3f> points, ObjectAttributes attributes)`

Adds multiple points to the document.

**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3f>) — Points to add.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to point objects.

**Returns:** `Guid[]` — An array of object unique identifiers.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddPoints_3.htm)

#### `public Guid AddPolyline(IEnumerable<Point3d> points)`

Adds a polyline object to Rhino.

**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — A list, an array or any enumerable set of Point3d.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddPolyline.htm)

#### `public Guid AddPolyline(IEnumerable<Point3d> points, ObjectAttributes attributes)`

Adds a polyline object to Rhino.

**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Point3d>) — A list, an array or any enumerable set of Point3d.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply to line.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddPolyline_1.htm)

#### `public Guid AddSphere(Sphere sphere)`

Adds a surface object to the document representing a sphere.

**Parameters:**
- `sphere` (Rhino.Geometry.Sphere) — A sphere to add.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddSphere.htm)

#### `public Guid AddSphere(Sphere sphere, ObjectAttributes attributes)`

Adds a surface object to the document representing a sphere.

**Parameters:**
- `sphere` (Rhino.Geometry.Sphere) — A sphere to add.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to link with the sphere.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddSphere_1.htm)

#### `public Guid AddSubD(SubD subd)`

Adds a SubD to the document

**Parameters:**
- `subd` (Rhino.Geometry.SubD) — the Subd to add

**Returns:** `Guid` — A unique identifier for the SubD, or Empty on failure

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddSubD.htm)

#### `public Guid AddSubD(SubD subd, ObjectAttributes attributes)`

Adds a SubD to the document

**Parameters:**
- `subd` (Rhino.Geometry.SubD) — the Subd to add
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to apply

**Returns:** `Guid` — A unique identifier for the SubD, or Empty on failure

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddSubD_1.htm)

#### `public Guid AddSurface(Surface surface)`

Adds a surface object to Rhino.

**Parameters:**
- `surface` (Rhino.Geometry.Surface) — A duplicate of this surface is added to Rhino.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddSurface.htm)

#### `public Guid AddSurface(Surface surface, ObjectAttributes attributes)`

Adds a surface object to Rhino.

**Parameters:**
- `surface` (Rhino.Geometry.Surface) — A duplicate of this surface is added to Rhino.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to link to the object.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddSurface_1.htm)

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

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddText_2.htm)

#### `public Guid AddText(string text, Plane plane, double height, string fontName, bool bold, bool italic, ObjectAttributes attributes)`

Adds an annotation text object to the document.

**Parameters:**
- `text` (System.String) — Text string.
- `plane` (Rhino.Geometry.Plane) — Plane of text.
- `height` (System.Double) — Height of text.
- `fontName` (System.String) — Name of FontFace.
- `bold` (System.Boolean) — Bold flag.
- `italic` (System.Boolean) — Italic flag.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Object Attributes.

**Returns:** `Guid` — The Guid of the newly added object or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddText_3.htm)

#### `public Guid AddText(string text, Plane plane, double height, string fontName, bool bold, bool italic, TextJustification justification)`

Adds an annotation text object to the document.

**Parameters:**
- `text` (System.String) — Text string.
- `plane` (Rhino.Geometry.Plane) — Plane of text.
- `height` (System.Double) — Height of text.
- `fontName` (System.String) — Name of FontFace.
- `bold` (System.Boolean) — Bold flag.
- `italic` (System.Boolean) — Italic flag.
- `justification` (Rhino.Geometry.TextJustification) — The justification of the text.

**Returns:** `Guid` — The Guid of the newly added object or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddText_4.htm)

#### `public Guid AddText(string text, Plane plane, double height, string fontName, bool bold, bool italic, TextJustification justification, ObjectAttributes attributes)`

Adds an annotation text object to the document.

**Parameters:**
- `text` (System.String) — Text string.
- `plane` (Rhino.Geometry.Plane) — Plane of text.
- `height` (System.Double) — Height of text.
- `fontName` (System.String) — Name of FontFace.
- `bold` (System.Boolean) — Bold flag.
- `italic` (System.Boolean) — Italic flag.
- `justification` (Rhino.Geometry.TextJustification) — The justification of the text.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to link to the object.

**Returns:** `Guid` — The Guid of the newly added object or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddText_5.htm)

#### `public Guid AddText(Text3d text3d)`

Adds an annotation text object to the document.

**Parameters:**
- `text3d` (Rhino.Display.Text3d) — The text object to add.

**Returns:** `Guid` — The Guid of the newly added object or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddText.htm)

#### `public Guid AddText(Text3d text3d, ObjectAttributes attributes)`

Adds an annotation text object to the document.

**Parameters:**
- `text3d` (Rhino.Display.Text3d) — The text object to add.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to link to the object.

**Returns:** `Guid` — The Guid of the newly added object or Guid.Empty on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddText_1.htm)

#### `public Guid AddTextDot(string text, Point3d location)`

Adds a text dot object to the table.

**Parameters:**
- `text` (System.String) — The text.
- `location` (Rhino.Geometry.Point3d) — The location.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddTextDot_2.htm)

#### `public Guid AddTextDot(string text, Point3d location, ObjectAttributes attributes)`

Adds a text dot object to the table.

**Parameters:**
- `text` (System.String) — The text.
- `location` (Rhino.Geometry.Point3d) — The location.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to link with curve.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddTextDot_3.htm)

#### `public Guid AddTextDot(TextDot dot)`

Adds a text dot object to the table.

**Parameters:**
- `dot` (Rhino.Geometry.TextDot) — The text dot.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddTextDot.htm)

#### `public Guid AddTextDot(TextDot dot, ObjectAttributes attributes)`

Adds a text dot object to the table.

**Parameters:**
- `dot` (Rhino.Geometry.TextDot) — The text dot.
- `attributes` (Rhino.DocObjects.ObjectAttributes) — Attributes to link with text dot.

**Returns:** `Guid` — A unique identifier for the object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_AddTextDot_1.htm)

#### `public void Clear()`

Marks all items as deleted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_Clear.htm)

#### `public bool Delete(Guid objectId)`

Deletes object from document.

**Parameters:**
- `objectId` (System.Guid) — Id of the object to delete.

**Returns:** `Boolean` — true on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_Delete_1.htm)

#### `public int Delete(IEnumerable<Guid> objectIds)`

Deletes a collection of objects from the document.

**Parameters:**
- `objectIds` (System.Collections.Generic.IEnumerable<Guid>) — Ids of all objects to delete.

**Returns:** `Int32` — The number of successfully deleted objects.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_Delete.htm)

#### `public virtual void Delete(int index)`

Flags a component as deleted.

**Parameters:**
- `index` (System.Int32) — The index of the item to flag.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Delete.htm)

#### `public override bool Delete(T item)`

Flags a component as deleted.

**Parameters:**
- `item` (T) — The item to flag.

**Returns:** `Boolean` — True on success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Delete_1.htm)

#### `public string Dump()`

Prepares a text dump of object table.

**Returns:** `String` — A string containing the dump.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmCommonComponentTable_1_Dump.htm)

#### `public File3dmObject[] FindByGroup(Group group)`

Finds all File3dmObject that are in a given group.

**Parameters:**
- `group` (Rhino.DocObjects.Group) — A group instance.

**Returns:** `File3dmObject[]` — Array of objects that belong to the specified group or empty array if no objects could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_FindByGroup.htm)

#### `public File3dmObject[] FindByLayer(Layer layer)`

Finds all File3dmObject that are in a given layer.

**Parameters:**
- `layer` (Rhino.DocObjects.Layer) — A layer instance.

**Returns:** `File3dmObject[]` — Array of objects that belong to the specified layer or empty array if no objects could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_FindByLayer.htm)

#### `public File3dmObject[] FindByLayer(string layer)`

Finds all File3dmObject that are in a given layer.

**Parameters:**
- `layer` (System.String) — Layer to search.

**Returns:** `File3dmObject[]` — Array of objects that belong to the specified layer or empty array if no objects could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_FindByLayer_1.htm)

#### `public virtual T FindId(Guid id)`

Uses the guid to find a model component. Deleted objects cannot be found by id. The guid is the value that is stored in the .Id property. In a single document, no two active objects have the same guid. If an object is replaced with a new object, then the guid persists. For example, if the _Move command moves an object, then the moved object inherits its guid from the starting object. If the Copy command copies an object, then the copy gets a new guid. This guid persists through file saving/opening operations. This function will not find grip objects.

**Parameters:**
- `id` (System.Guid) — ID of model component to search for.

**Returns:** `T` — Reference to the rhino object with the objectId or null if no such object could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_CommonComponentTable_1_FindId.htm)

#### `public BoundingBox GetBoundingBox()`

Gets the bounding box containing every object in this table.

**Returns:** `BoundingBox` — The computed bounding box.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_GetBoundingBox.htm)

#### `public override IEnumerator<File3dmObject> GetEnumerator()`

Returns an enumerator that yields all objects in this document. Like in Rhino, this includes lights. Unlike in Rhino, however, all lights are returned in the end of the list.

**Returns:** `IEnumerator<File3dmObject>` — An enumerator that yields all objects in a document.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmObjectTable_GetEnumerator.htm)

### Properties
- `ComponentType` (ModelComponentType, get) — Returns ModelGeometry.
- `Count` (Int32, get) — Returns the total amount of items in the object table, including lights.

## File3dmPlugInData (class)

Represents custom plug-in data, in the 3dm file, written by a plug-in.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_File3dmPlugInData.htm)

### Properties
- `PlugInId` (Guid, get) — Gets the id of the plug-in that is associated with this custom data.

## File3dmPlugInDataTable (class)

Table of custom data provided by plug-ins

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_File3dmPlugInDataTable.htm)

### Methods
#### `public void Clear()`

Remove all entries from this table.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmPlugInDataTable_Clear.htm)

#### `public string Dump()`

Prepares a text dump of table.

**Returns:** `String` — A string containing the dump.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmPlugInDataTable_Dump.htm)

#### `public IEnumerator<File3dmPlugInData> GetEnumerator()`

Gets the enumerator that visits any File3dmPlugInData in this table.

**Returns:** `IEnumerator<File3dmPlugInData>` — The enumerator.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmPlugInDataTable_GetEnumerator.htm)

#### `public bool TryRead(File3dmPlugInData pluginData, Func<File3dm, BinaryArchiveReader, bool> dataReader)`

Attempts to read a Rhino plug-in's custom data from the File3dm file.

**Parameters:**
- `pluginData` (Rhino.FileIO.File3dmPlugInData) — The plug-in whose data you want to try to read.
- `dataReader` (System.Func<File3dm,BinaryArchiveReader,Boolean>) — The function that will read the data. This function must be implemented identical to the the originating plug-in's ReadDocument(RhinoDoc, BinaryArchiveReader, FileReadOptions) method.

**Returns:** `Boolean` — The value returned by the data reading function if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmPlugInDataTable_TryRead.htm)

### Properties
- `Count` (Int32, get) — Gets the number of File3dmPlugInData objects in this table.
- `Item` (File3dmPlugInData, get) — Gets the File3dmPlugInData object at the given index.

## File3dmSettings (class)

General settings in a 3dm file.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_File3dmSettings.htm)

### Properties
- `ModelAbsoluteTolerance` (Double, get/set) — Gets or sets the model space absolute tolerance.
- `ModelAngleToleranceDegrees` (Double, get/set) — Gets or sets the model space angle tolerance.
- `ModelAngleToleranceRadians` (Double, get/set) — Gets or sets the model space angle tolerance.
- `ModelBasepoint` (Point3d, get/set) — Gets or sets the model base point that is used when the file is read as an instance definition. This point is mapped to the origin in the instance definition.
- `ModelRelativeTolerance` (Double, get/set) — Gets or sets the model space relative tolerance.
- `ModelUnitSystem` (UnitSystem, get/set) — Gets or sets the model unit system, using UnitSystem enumeration.
- `ModelUrl` (String, get/set) — Gets or sets a Uniform Resource Locator (URL) direction for the model.
- `PageAbsoluteTolerance` (Double, get/set) — Gets or sets the page space absolute tolerance.
- `PageAngleToleranceDegrees` (Double, get/set) — Gets or sets the page space angle tolerance.
- `PageAngleToleranceRadians` (Double, get/set) — Gets or sets the page space angle tolerance.
- `PageRelativeTolerance` (Double, get/set) — Gets or sets the page space relative tolerance.
- `PageUnitSystem` (UnitSystem, get/set) — Gets or sets the page unit system, using UnitSystem enumeration.

## File3dmStringTable (class)

Provides access to document strings in the 3dm file.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_File3dmStringTable.htm)

### Methods
#### `public void Delete(string key)`

Removes a document string from the 3dm file.

**Parameters:**
- `key` (System.String) — The key to remove.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmStringTable_Delete.htm)

#### `public void Delete(string section, string entry)`

Removes document strings from the 3dm file.

**Parameters:**
- `section` (System.String) — name of section to delete. If null, all sections will be deleted.
- `entry` (System.String) — name of entry to delete. If null, all entries will be deleted for a given section.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmStringTable_Delete_1.htm)

#### `public string[] GetEntryNames(string section)`

Return list of all entry names for a given section of document strings in the 3dm file.

**Parameters:**
- `section` (System.String) — The section from which to retrieve section names.

**Returns:** `String[]` — An array of section names. This can be empty, but not null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmStringTable_GetEntryNames.htm)

#### `public string GetKey(int i)`

Returns a key value at a given index.

**Parameters:**
- `i` (System.Int32) — The index.

**Returns:** `String` — The key if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmStringTable_GetKey.htm)

#### `public string[] GetSectionNames()`

Returns a list of all the section names for document strings in the 3dm file. By default a section name is a key that is prefixed with a string separated by a backslash.

**Returns:** `String[]` — An array of section names. This can be empty, but not null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmStringTable_GetSectionNames.htm)

#### `public string GetValue(int i)`

Returns a string value at a given index.

**Parameters:**
- `i` (System.Int32) — The index at which to get the value.

**Returns:** `String` — The string value if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmStringTable_GetValue.htm)

#### `public string GetValue(string key)`

Returns a string value at a key.

**Parameters:**
- `key` (System.String) — The key at which to get the value.

**Returns:** `String` — The string value if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmStringTable_GetValue_1.htm)

#### `public string GetValue(string section, string entry)`

Returns a string value given a section and entry.

**Parameters:**
- `section` (System.String) — The section at which to get the value.
- `entry` (System.String) — The entry to search for.

**Returns:** `String` — The string value if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmStringTable_GetValue_2.htm)

#### `public string SetString(string key, string value)`

Adds or sets a a document string in the 3dm file.

**Parameters:**
- `key` (System.String) — The key.
- `value` (System.String) — The entry value.

**Returns:** `String` — The previous value if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmStringTable_SetString.htm)

#### `public string SetString(string section, string entry, string value)`

Adds or sets a document string in the 3dm file.

**Parameters:**
- `section` (System.String) — The section.
- `entry` (System.String) — The entry name.
- `value` (System.String) — The entry value.

**Returns:** `String` — The previous value if successful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmStringTable_SetString_1.htm)

### Properties
- `Count` (Int32, get) — Returns the number of document strings in the 3dm file.
- `DocumentUserTextCount` (Int32, get) — Returns the number of Section/Entry-style key values.

## File3dmTypeCodes (class)

Typecode format 4 bytes long Copyx xxxxxxxxxxxxxxx,x xxx xxxx xxxx x x xx | | | | | | | | | | | | | | | +--- "stuff" bit | | | | | | | +-- specific codes | | | | | +-- RESERVED - DO NOT USE (should be 0) (will be used to control CRC on/off) | | | +-- category:_000 0000 0000 0001 Legacy geometry TCODE_LEGACY_GEOMETRY | _000 0000 0000 0010 openNURBS object TCODE_OPENNURBS_OBJECT | _000 0000 0000 0100 -- RESERVED - DO NOT USE (should be 0 in any typecode) -- | _000 0000 0000 1000 -- RESERVED - DO NOT USE (should be 0 in any typecode) -- | _000 0000 0001 0000 Geometry TCODE_GEOMETRY | _000 0000 0010 0000 Annotation | _000 0000 0100 0000 Display Attributes TCODE_DISPLAY | _000 0000 1000 0000 Rendering TCODE_RENDER | _000 0001 0000 0000 | _000 0010 0000 0000 Interface TCODE_INTERFACE | _000 0100 0000 0000 -- RESERVED - DO NOT USE (should be 0 in any typecode) -- | _000 1000 0000 0000 Tolerances TCODE_TOLERANCE | _001 0000 0000 0000 Tables TCODE_TABLE | _010 0000 0000 0000 Table record TCODE_TABLEREC | _100 0000 0000 0000 User information TCODE_USER | +-- format: 0 - data size in header - data block follows TCODE_SHORT 1 - data in header - no data block follows

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_File3dmTypeCodes.htm)

### Properties
- `TCODE_ANALYSIS_MESH` (UInt32, get) — 
- `TCODE_ANGULAR_DIMENSION` (UInt32, get) — 
- `TCODE_ANNOTATION` (UInt32, get) — 0x0020000.
- `TCODE_ANNOTATION_LEADER` (UInt32, get) — 
- `TCODE_ANNOTATION_SETTINGS` (UInt32, get) — 
- `TCODE_ANONYMOUS_CHUNK` (UInt32, get) — (TCODE_USER | TCODE_CRC | 0x0000)
- `TCODE_BITMAP_RECORD` (UInt32, get) — 
- `TCODE_BITMAP_TABLE` (UInt32, get) — 
- `TCODE_BITMAPPREVIEW` (UInt32, get) — 
- `TCODE_BUMPMAP` (UInt32, get) — 
- `TCODE_COMMENTBLOCK` (UInt32, get) — (0x00000001) The TCODE_COMMENTBLOCK is the first chunk in the file, starts 32 bytes into the file, and contains text information terminated with a ^m_z. This ^m_z and contents of this chunk were expanded in February 2000. Files written with code released earlier than this will not have the ^m_z.
- `TCODE_COMPRESSED_MESH_GEOMETRY` (UInt32, get) — 
- `TCODE_CPLANE` (UInt32, get) — 
- `TCODE_CRC` (UInt32, get) — 0x8000.
- `TCODE_CURRENTLAYER` (UInt32, get) — 
- `TCODE_DICTIONARY` (UInt32, get) — 
- `TCODE_DICTIONARY_END` (UInt32, get) — 
- `TCODE_DICTIONARY_ENTRY` (UInt32, get) — 
- `TCODE_DICTIONARY_ID` (UInt32, get) — 
- `TCODE_DIMSTYLE_RECORD` (UInt32, get) — 
- `TCODE_DIMSTYLE_TABLE` (UInt32, get) — 
- `TCODE_DISP_AM_RESOLUTION` (UInt32, get) — 
- `TCODE_DISP_CPLINES` (UInt32, get) — 
- `TCODE_DISP_MAXLENGTH` (UInt32, get) — 
- `TCODE_DISPLAY` (UInt32, get) — 0x0040000.
- `TCODE_ENDOFFILE` (UInt32, get) — (0x00007FFF) The TCODE_ENDOFFILE is the last chunk in the file and the first 4 bytes of information in this chunk is an integer that contains the file length. This chunk was added in February 2000 and files written with code released earlier than this will not have this termination block.
- `TCODE_ENDOFFILE_GOO` (UInt32, get) — (0x00007FFE) this typecode is returned when a rogue eof marker is found Some v1 3dm file writers put these markers in a "goo". Simply skip these chunks and continue.
- `TCODE_ENDOFTABLE` (UInt32, get) — 
- `TCODE_FONT_RECORD` (UInt32, get) — 
- `TCODE_FONT_TABLE` (UInt32, get) — 
- `TCODE_GEOMETRY` (UInt32, get) — 0x0010000.
- `TCODE_GROUP_RECORD` (UInt32, get) — 
- `TCODE_GROUP_TABLE` (UInt32, get) — 
- `TCODE_HATCHPATTERN_RECORD` (UInt32, get) — 
- `TCODE_HATCHPATTERN_TABLE` (UInt32, get) — 
- `TCODE_HIDE_TRACE` (UInt32, get) — 
- `TCODE_HISTORYRECORD_RECORD` (UInt32, get) — 
- `TCODE_HISTORYRECORD_TABLE` (UInt32, get) — 
- `TCODE_INSTANCE_DEFINITION_RECORD` (UInt32, get) — 
- `TCODE_INSTANCE_DEFINITION_TABLE` (UInt32, get) — 
- `TCODE_INTERFACE` (UInt32, get) — 0x02000000.
- `TCODE_LAYER` (UInt32, get) — 
- `TCODE_LAYER_OBSELETE_1` (UInt32, get) — 
- `TCODE_LAYER_OBSELETE_2` (UInt32, get) — 
- `TCODE_LAYER_OBSELETE_3` (UInt32, get) — 
- `TCODE_LAYER_RECORD` (UInt32, get) — 
- `TCODE_LAYER_TABLE` (UInt32, get) — layers.
- `TCODE_LAYERINDEX` (UInt32, get) — 
- `TCODE_LAYERLOCKED` (UInt32, get) — 
- `TCODE_LAYERMATERIALINDEX` (UInt32, get) — 
- `TCODE_LAYERNAME` (UInt32, get) — 
- `TCODE_LAYERON` (UInt32, get) — 
- `TCODE_LAYERPICKABLE` (UInt32, get) — 
- `TCODE_LAYERREF` (UInt32, get) — 
- `TCODE_LAYERRENDERABLE` (UInt32, get) — 
- `TCODE_LAYERSNAPABLE` (UInt32, get) — 
- `TCODE_LAYERSTATE` (UInt32, get) — 
- `TCODE_LAYERTABLE` (UInt32, get) — 
- `TCODE_LAYERTHAWED` (UInt32, get) — 
- `TCODE_LAYERVISIBLE` (UInt32, get) — 
- `TCODE_LEGACY_ASM` (UInt32, get) — 
- `TCODE_LEGACY_ASMSTUFF` (UInt32, get) — 
- `TCODE_LEGACY_BND` (UInt32, get) — 
- `TCODE_LEGACY_BNDSTUFF` (UInt32, get) — 
- `TCODE_LEGACY_CRV` (UInt32, get) — 
- `TCODE_LEGACY_CRVSTUFF` (UInt32, get) — 
- `TCODE_LEGACY_FAC` (UInt32, get) — 
- `TCODE_LEGACY_FACSTUFF` (UInt32, get) — 
- `TCODE_LEGACY_GEOMETRY` (UInt32, get) — 
- `TCODE_LEGACY_PNT` (UInt32, get) — 
- `TCODE_LEGACY_PNTSTUFF` (UInt32, get) — 
- `TCODE_LEGACY_PRT` (UInt32, get) — 
- `TCODE_LEGACY_PRTSTUFF` (UInt32, get) — 
- `TCODE_LEGACY_SHL` (UInt32, get) — 
- `TCODE_LEGACY_SHLSTUFF` (UInt32, get) — 
- `TCODE_LEGACY_SPL` (UInt32, get) — 
- `TCODE_LEGACY_SPLSTUFF` (UInt32, get) — 
- `TCODE_LEGACY_SRF` (UInt32, get) — 
- `TCODE_LEGACY_SRFSTUFF` (UInt32, get) — 
- `TCODE_LEGACY_TOL_ANGLE` (UInt32, get) — 
- `TCODE_LEGACY_TOL_FIT` (UInt32, get) — 
- `TCODE_LEGACY_TRM` (UInt32, get) — 
- `TCODE_LEGACY_TRMSTUFF` (UInt32, get) — 
- `TCODE_LIGHT_RECORD` (UInt32, get) — 
- `TCODE_LIGHT_RECORD_ATTRIBUTES` (UInt32, get) — 
- `TCODE_LIGHT_RECORD_ATTRIBUTES_USERDATA` (UInt32, get) — 
- `TCODE_LIGHT_RECORD_END` (UInt32, get) — 
- `TCODE_LIGHT_TABLE` (UInt32, get) — rendering lights.
- `TCODE_LINEAR_DIMENSION` (UInt32, get) — 
- `TCODE_LINETYPE_RECORD` (UInt32, get) — 
- `TCODE_LINETYPE_TABLE` (UInt32, get) — 
- `TCODE_MATERIAL_RECORD` (UInt32, get) — 
- `TCODE_MATERIAL_TABLE` (UInt32, get) — rendering materials.
- `TCODE_MAXIMIZED_VIEWPORT` (UInt32, get) — 
- `TCODE_MESH_OBJECT` (UInt32, get) — 
- `TCODE_NAME` (UInt32, get) — 
- `TCODE_NAMED_CPLANE` (UInt32, get) — 
- `TCODE_NAMED_VIEW` (UInt32, get) — 
- `TCODE_NEAR_CLIP_PLANE` (UInt32, get) — 
- `TCODE_NOTES` (UInt32, get) — 
- `TCODE_OBJECT_RECORD` (UInt32, get) — 
- `TCODE_OBJECT_RECORD_ATTRIBUTES` (UInt32, get) — 
- `TCODE_OBJECT_RECORD_ATTRIBUTES_USERDATA` (UInt32, get) — 
- `TCODE_OBJECT_RECORD_END` (UInt32, get) — 
- `TCODE_OBJECT_RECORD_HISTORY` (UInt32, get) — 
- `TCODE_OBJECT_RECORD_HISTORY_DATA` (UInt32, get) — 
- `TCODE_OBJECT_RECORD_HISTORY_HEADER` (UInt32, get) — 
- `TCODE_OBJECT_RECORD_TYPE` (UInt32, get) — 
- `TCODE_OBJECT_TABLE` (UInt32, get) — geometry and annotation.
- `TCODE_OBSOLETE_LAYERSET_RECORD` (UInt32, get) — 
- `TCODE_OBSOLETE_LAYERSET_TABLE` (UInt32, get) — 
- `TCODE_OLD_FULLMESH` (UInt32, get) — 
- `TCODE_OLD_MESH_UV` (UInt32, get) — 
- `TCODE_OLD_MESH_VERTEX_NORMALS` (UInt32, get) — 
- `TCODE_OLD_RH_TRIMESH` (UInt32, get) — 
- `TCODE_OPENNURBS_CLASS` (UInt32, get) — 
- `TCODE_OPENNURBS_CLASS_DATA` (UInt32, get) — 
- `TCODE_OPENNURBS_CLASS_END` (UInt32, get) — 
- `TCODE_OPENNURBS_CLASS_USERDATA` (UInt32, get) — 
- `TCODE_OPENNURBS_CLASS_USERDATA_HEADER` (UInt32, get) — 
- `TCODE_OPENNURBS_CLASS_UUID` (UInt32, get) — 
- `TCODE_OPENNURBS_OBJECT` (UInt32, get) — 0x00020000.
- `TCODE_PROPERTIES_APPLICATION` (UInt32, get) — 
- `TCODE_PROPERTIES_COMPRESSED_PREVIEWIMAGE` (UInt32, get) — 
- `TCODE_PROPERTIES_NOTES` (UInt32, get) — 
- `TCODE_PROPERTIES_OPENNURBS_VERSION` (UInt32, get) — 
- `TCODE_PROPERTIES_PREVIEWIMAGE` (UInt32, get) — 
- `TCODE_PROPERTIES_REVISIONHISTORY` (UInt32, get) — 
- `TCODE_PROPERTIES_TABLE` (UInt32, get) — Model Properties: revision history, notes, preview image.
- `TCODE_RADIAL_DIMENSION` (UInt32, get) — 
- `TCODE_RENDER` (UInt32, get) — 0x0080000.
- `TCODE_RENDER_MATERIAL_ID` (UInt32, get) — 
- `TCODE_RENDERMESHPARAMS` (UInt32, get) — 
- `TCODE_RGB` (UInt32, get) — 
- `TCODE_RGBDISPLAY` (UInt32, get) — 
- `TCODE_RH_POINT` (UInt32, get) — 
- `TCODE_RH_SPOTLIGHT` (UInt32, get) — 
- `TCODE_RHINOIO_OBJECT_BREP` (UInt32, get) — 
- `TCODE_RHINOIO_OBJECT_DATA` (UInt32, get) — 
- `TCODE_RHINOIO_OBJECT_END` (UInt32, get) — 
- `TCODE_RHINOIO_OBJECT_NURBS_CURVE` (UInt32, get) — 
- `TCODE_RHINOIO_OBJECT_NURBS_SURFACE` (UInt32, get) — 
- `TCODE_SETTINGS__NEVER__USE__THIS` (UInt32, get) — 
- `TCODE_SETTINGS_ANALYSISMESH` (UInt32, get) — 
- `TCODE_SETTINGS_ANNOTATION` (UInt32, get) — 
- `TCODE_SETTINGS_ATTRIBUTES` (UInt32, get) — 
- `TCODE_SETTINGS_CURRENT_COLOR` (UInt32, get) — 
- `TCODE_SETTINGS_CURRENT_DIMSTYLE_INDEX` (UInt32, get) — 
- `TCODE_SETTINGS_CURRENT_FONT_INDEX` (UInt32, get) — 
- `TCODE_SETTINGS_CURRENT_LAYER_INDEX` (UInt32, get) — 
- `TCODE_SETTINGS_CURRENT_MATERIAL_INDEX` (UInt32, get) — 
- `TCODE_SETTINGS_CURRENT_WIRE_DENSITY` (UInt32, get) — 
- `TCODE_SETTINGS_GRID_DEFAULTS` (UInt32, get) — 
- `TCODE_SETTINGS_MODEL_URL` (UInt32, get) — 
- `TCODE_SETTINGS_NAMED_CPLANE_LIST` (UInt32, get) — 
- `TCODE_SETTINGS_NAMED_VIEW_LIST` (UInt32, get) — 
- `TCODE_SETTINGS_PLUGINLIST` (UInt32, get) — 
- `TCODE_SETTINGS_RENDER` (UInt32, get) — 
- `TCODE_SETTINGS_RENDERMESH` (UInt32, get) — 
- `TCODE_SETTINGS_TABLE` (UInt32, get) — 
- `TCODE_SETTINGS_UNITSANDTOLS` (UInt32, get) — 
- `TCODE_SETTINGS_VIEW_LIST` (UInt32, get) — 
- `TCODE_SHORT` (UInt32, get) — 0x80000000.
- `TCODE_SHOWGRID` (UInt32, get) — 
- `TCODE_SHOWGRIDAXES` (UInt32, get) — 
- `TCODE_SHOWWORLDAXES` (UInt32, get) — 
- `TCODE_SNAPSIZE` (UInt32, get) — 
- `TCODE_STUFF` (UInt32, get) — 
- `TCODE_SUMMARY` (UInt32, get) — 
- `TCODE_TABLE` (UInt32, get) — 0x10000000.
- `TCODE_TABLEREC` (UInt32, get) — 0x20000000.
- `TCODE_TEXT_BLOCK` (UInt32, get) — 
- `TCODE_TEXTURE_MAPPING_RECORD` (UInt32, get) — 
- `TCODE_TEXTURE_MAPPING_TABLE` (UInt32, get) — 
- `TCODE_TEXTUREMAP` (UInt32, get) — 
- `TCODE_TOLERANCE` (UInt32, get) — 0x08000000.
- `TCODE_TRANSPARENCY` (UInt32, get) — 
- `TCODE_UNIT_AND_TOLERANCES` (UInt32, get) — 
- `TCODE_USER` (UInt32, get) — 0x40000000.
- `TCODE_USER_RECORD` (UInt32, get) — 
- `TCODE_USER_TABLE` (UInt32, get) — 
- `TCODE_USER_TABLE_UUID` (UInt32, get) — 
- `TCODE_VIEW` (UInt32, get) — 
- `TCODE_VIEW_ATTRIBUTES` (UInt32, get) — 
- `TCODE_VIEW_CPLANE` (UInt32, get) — 
- `TCODE_VIEW_DISPLAYMODE` (UInt32, get) — 
- `TCODE_VIEW_NAME` (UInt32, get) — 
- `TCODE_VIEW_POSITION` (UInt32, get) — 
- `TCODE_VIEW_RECORD` (UInt32, get) — 
- `TCODE_VIEW_SHOWCONAXES` (UInt32, get) — 
- `TCODE_VIEW_SHOWCONGRID` (UInt32, get) — 
- `TCODE_VIEW_SHOWWORLDAXES` (UInt32, get) — 
- `TCODE_VIEW_TARGET` (UInt32, get) — 
- `TCODE_VIEW_TRACEIMAGE` (UInt32, get) — 
- `TCODE_VIEW_VIEWPORT` (UInt32, get) — 
- `TCODE_VIEW_VIEWPORT_USERDATA` (UInt32, get) — 
- `TCODE_VIEW_WALLPAPER` (UInt32, get) — 
- `TCODE_VIEW_WALLPAPER_V3` (UInt32, get) — 
- `TCODE_VIEWPORT` (UInt32, get) — 
- `TCODE_VIEWPORT_DISPLAY_MODE` (UInt32, get) — 
- `TCODE_VIEWPORT_POSITION` (UInt32, get) — 
- `TCODE_VIEWPORT_TRACEINFO` (UInt32, get) — 
- `TCODE_VIEWPORT_WALLPAPER` (UInt32, get) — 
- `TCODE_XDATA` (UInt32, get) — 

## File3dmViewTable (class)

Provides access to views in the 3dm file.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_File3dmViewTable.htm)

### Methods
#### `public void Add(ViewInfo item)`

Adds a

**Parameters:**
- `item` (Rhino.DocObjects.ViewInfo) — [Missing <param name="item"/> documentation for "M:Rhino.FileIO.File3dmViewTable.Add(Rhino.DocObjects.ViewInfo)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmViewTable_Add.htm)

#### `public void Clear()`

Removes all items from the table.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmViewTable_Clear.htm)

#### `public bool Contains(ViewInfo item)`

Returns an indication of the presence of a view in the table.

**Parameters:**
- `item` (Rhino.DocObjects.ViewInfo) — The view to check.

**Returns:** `Boolean` — true if the item is in the table; false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmViewTable_Contains.htm)

#### `public void CopyTo(ViewInfo[] array, int arrayIndex)`

Copies the content of the table to an array.

**Parameters:**
- `array` (Rhino.DocObjects.ViewInfo[]) — [Missing <param name="array"/> documentation for "M:Rhino.FileIO.File3dmViewTable.CopyTo(Rhino.DocObjects.ViewInfo[],System.Int32)"]
- `arrayIndex` (System.Int32) — [Missing <param name="arrayIndex"/> documentation for "M:Rhino.FileIO.File3dmViewTable.CopyTo(Rhino.DocObjects.ViewInfo[],System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmViewTable_CopyTo.htm)

#### `public bool Delete(int index)`

Removes an item.

**Parameters:**
- `index` (System.Int32) — The index of the item to remove.

**Returns:** `Boolean` — True if the item was removed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmViewTable_Delete_1.htm)

#### `public bool Delete(ViewInfo item)`

Deletes an item.

**Parameters:**
- `item` (Rhino.DocObjects.ViewInfo) — [Missing <param name="item"/> documentation for "M:Rhino.FileIO.File3dmViewTable.Delete(Rhino.DocObjects.ViewInfo)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.FileIO.File3dmViewTable.Delete(Rhino.DocObjects.ViewInfo)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmViewTable_Delete.htm)

#### `public ViewInfo FindName(string name)`

Finds a ViewInfo given its name.

**Parameters:**
- `name` (System.String) — The name of the ViewInfo to be searched.

**Returns:** `ViewInfo` — An ViewInfo, or null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmViewTable_FindName.htm)

#### `public IEnumerator<ViewInfo> GetEnumerator()`

Returns an enumerator that yields all views in the table.

**Returns:** `IEnumerator<ViewInfo>` — An enumerator.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmViewTable_GetEnumerator.htm)

#### `public int IndexOf(ViewInfo item)`

Returns the index of the current ViewInfo.

**Parameters:**
- `item` (Rhino.DocObjects.ViewInfo) — The item to be searched.

**Returns:** `Int32` — The index of the ViewInfo.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmViewTable_IndexOf.htm)

### Properties
- `Count` (Int32, get) — Gets the amount of items in the table.
- `Item` (ViewInfo, get) — Gets the view info at an index. The set method always throws NotSupportedException.

## File3dmWriteOptions (class)

Options used by File3dm.Write

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_File3dmWriteOptions.htm)

### Constructors
- `public File3dmWriteOptions()` — Initializes properties to defaults.

### Methods
#### `public void EnableAnalysisMeshes(ObjectType objectType, bool enable)`

Activates saving of analysis meshes for specific types of objects. If you do not specify the state for an object type, its default is used. Currently SubD mesh saving is disabled by default, while Brep and Extrusion is enabled.

**Parameters:**
- `objectType` (Rhino.DocObjects.ObjectType) — The object type. Mostly mesh, brep, extrusion and SubD (or their flag combinations) make sense here. DO NOT specify a 'filter' or sub-object type.
- `enable` (System.Boolean) — If false, disables saving for this object type.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmWriteOptions_EnableAnalysisMeshes.htm)

#### `public void EnableRenderMeshes(ObjectType objectType, bool enable)`

Activates saving of render meshes for specific types of objects. If you do not specify the state for an object type, its default is used. Specifically, currently SubD mesh saving is disabled by default, while Brep and Extrusion is on.

**Parameters:**
- `objectType` (Rhino.DocObjects.ObjectType) — The object type. Mostly brep, extrusion and SubD (or their flag combinations) make sense here. DO NOT specify a 'filter' or sub-object type.
- `enable` (System.Boolean) — If false, disables saving for this object type.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_File3dmWriteOptions_EnableRenderMeshes.htm)

### Properties
- `SaveAnalysisMeshes` (Boolean, get/set) — Include analysis meshes in the file. Default is true
- `SaveRenderMeshes` (Boolean, get/set) — Include Render meshes in the file. Default is true
- `SaveUserData` (Boolean, get/set) — Include custom user data in the file. Default is true
- `Version` (Int32, get/set) — File version. Default is major version number of this assembly version.Must be in range [2; current version].Alternatively, 0 is a placeholder for the last valid version.Rhino can read its current version, plus earlier file versions except 1.Use latest version when possible.

## FileFindPreference (enum)

Defines options for file search.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_FileFindPreference.htm)

### Values
- `None` = `0` — The choice is not defined.
- `FullPath` = `1` — File name exists in FullPath().
- `RelativePath` = `2` — File name exists in base path + RelativePath().
- `BasePath` = `3` — File name exists in base path directory.
- `ContentMatch` = `4` — File with mathing content exists.
- `MostRecent` = `5` — Most recently modifed file.

## FileObj (class)

Support for obj file format

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_FileObj.htm)

### Methods
#### `public static bool Read(string filename, RhinoDoc doc, FileObjReadOptions options)`



**Parameters:**
- `filename` (System.String) — [Missing <param name="filename"/> documentation for "M:Rhino.FileIO.FileObj.Read(System.String,Rhino.RhinoDoc,Rhino.FileIO.FileObjReadOptions)"]
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.FileIO.FileObj.Read(System.String,Rhino.RhinoDoc,Rhino.FileIO.FileObjReadOptions)"]
- `options` (Rhino.FileIO.FileObjReadOptions) — [Missing <param name="options"/> documentation for "M:Rhino.FileIO.FileObj.Read(System.String,Rhino.RhinoDoc,Rhino.FileIO.FileObjReadOptions)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.FileIO.FileObj.Read(System.String,Rhino.RhinoDoc,Rhino.FileIO.FileObjReadOptions)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_FileObj_Read.htm)

#### `public static WriteFileResult Write(StreamWriter stream, RhinoDoc doc, FileObjWriteOptions options)`

Write an obj stream based on the contents of a RhinoDoc

**Remarks:** No .mtl file is written, hence no materials, when the WriteFile that takes a StreamWriter is called directly. If you need materials call the WriteFile that takes a string for the obj file path.

**Parameters:**
- `stream` (System.IO.StreamWriter) — [Missing <param name="stream"/> documentation for "M:Rhino.FileIO.FileObj.Write(System.IO.StreamWriter,Rhino.RhinoDoc,Rhino.FileIO.FileObjWriteOptions)"]
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.FileIO.FileObj.Write(System.IO.StreamWriter,Rhino.RhinoDoc,Rhino.FileIO.FileObjWriteOptions)"]
- `options` (Rhino.FileIO.FileObjWriteOptions) — [Missing <param name="options"/> documentation for "M:Rhino.FileIO.FileObj.Write(System.IO.StreamWriter,Rhino.RhinoDoc,Rhino.FileIO.FileObjWriteOptions)"]

**Returns:** `WriteFileResult` — [Missing <returns> documentation for "M:Rhino.FileIO.FileObj.Write(System.IO.StreamWriter,Rhino.RhinoDoc,Rhino.FileIO.FileObjWriteOptions)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_FileObj_Write.htm)

#### `public static WriteFileResult Write(string filename, Mesh[] meshes, FileObjWriteOptions options)`

Write an obj file with an array of meshes

**Parameters:**
- `filename` (System.String) — path to write to
- `meshes` (Rhino.Geometry.Mesh[]) — meshes to write as obj format
- `options` (Rhino.FileIO.FileObjWriteOptions) — [Missing <param name="options"/> documentation for "M:Rhino.FileIO.FileObj.Write(System.String,Rhino.Geometry.Mesh[],Rhino.FileIO.FileObjWriteOptions)"]

**Returns:** `WriteFileResult` — [Missing <returns> documentation for "M:Rhino.FileIO.FileObj.Write(System.String,Rhino.Geometry.Mesh[],Rhino.FileIO.FileObjWriteOptions)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_FileObj_Write_1.htm)

#### `public static WriteFileResult Write(string filename, RhinoDoc doc, FileObjWriteOptions options)`

Write an obj file based on the contents of a RhinoDoc

**Parameters:**
- `filename` (System.String) — [Missing <param name="filename"/> documentation for "M:Rhino.FileIO.FileObj.Write(System.String,Rhino.RhinoDoc,Rhino.FileIO.FileObjWriteOptions)"]
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.FileIO.FileObj.Write(System.String,Rhino.RhinoDoc,Rhino.FileIO.FileObjWriteOptions)"]
- `options` (Rhino.FileIO.FileObjWriteOptions) — [Missing <param name="options"/> documentation for "M:Rhino.FileIO.FileObj.Write(System.String,Rhino.RhinoDoc,Rhino.FileIO.FileObjWriteOptions)"]

**Returns:** `WriteFileResult` — [Missing <returns> documentation for "M:Rhino.FileIO.FileObj.Write(System.String,Rhino.RhinoDoc,Rhino.FileIO.FileObjWriteOptions)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_FileObj_Write_2.htm)

## FileObjReadOptions (class)

Options used when reading an obj file.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_FileObjReadOptions.htm)

### Constructors
- `public FileObjReadOptions(FileReadOptions readOptions)` — Initializes a new instance of the FileObjReadOptions class

### Methods
#### `public Transform GetTransform()`

Calculates the YToZ transform.

**Returns:** `Transform` — [Missing <returns> documentation for "M:Rhino.FileIO.FileObjReadOptions.GetTransform"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_FileObjReadOptions_GetTransform.htm)

### Properties
- `DisplayColorFromObjMaterial` (Boolean, get/set) — Determines whether textures are read from the .mtl file, if it exists.
- `IgnoreTextures` (Boolean, get/set) — Determines whether textures are read from the .mtl file, if it exists.
- `MapYtoZ` (Boolean, get/set) — Setting to transform OBJ's Y axis to Rhino's Z axis
- `MorphTargetOnly` (Boolean, get/set) — TODO
- `ReverseGroupOrder` (Boolean, get/set) — Determines how groups/layers are nested when reading an obj file. Left to Right (default = false) or Right to Left (true)
- `Split32BitTextures` (Boolean, get/set) — 
- `UseObjGroupsAs` (FileObjReadOptions.UseObjGsAs, get/set) — 
- `UseObjObjects` (Boolean, get/set) — Determines whether or not "o"s in the obj file will be interpreted as objects in the Rhino model
- `ReadOptions` (FileReadOptions, get) — Rhino's FileReadOptions passed into ReadFile

## FileObjReadOptions.UseObjGsAs (enum)

Determines how "g"s in the obj file will be interpreted on import

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_FileObjReadOptions_UseObjGsAs.htm)

### Values
- `IgnoreObjGroups` = `0` — OBJ "g"s in the file are ignored
- `ObjGroupsAsLayers` = `1` — OBJ "g"s in the file will become Rhino layers
- `ObjGroupsAsGroups` = `2` — OBJ "g"s in the file will become Rhino groups
- `ObjGroupsAsObjects` = `3` — OBJ "g"s in the file will become Rhino objects

## FileObjWriteOptions (class)

[Missing <summary> documentation for "T:Rhino.FileIO.FileObjWriteOptions"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_FileObjWriteOptions.htm)

### Constructors
- `public FileObjWriteOptions(FileWriteOptions writeOptions)` — Initializes a new instance of the FileObjWriteOptions class

### Methods
#### `public Transform GetTransform()`

Calculates the transform combination of ZToY and any the translation that might occur in a SavewithOrigin.

**Returns:** `Transform` — [Missing <returns> documentation for "M:Rhino.FileIO.FileObjWriteOptions.GetTransform"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_FileObjWriteOptions_GetTransform.htm)

### Properties
- `ActualFilePathOnMac` (String, get/set) — 
- `CreateNgons` (Boolean, get/set) — Setting to enable/disable the creation of ngons for the output
- `CullUnnecessaryVertexesInNgons` (Boolean, get/set) — Setting to determine whether interior collinear vertexes are part of the ngon.
- `EolType` (FileObjWriteOptions.AsciiEol, get/set) — 
- `ExportAsTriangles` (Boolean, get/set) — Enable/disable export of faces as triangles.
- `ExportGroupNameLayerNames` (FileObjWriteOptions.ObjGroupNames, get/set) — Setting to determine whether object, group or layer names will become "g"s in the OBJ output file
- `ExportMaterialDefinitions` (Boolean, get/set) — Setting to write an .mtl file and "usemtl"s in the obj file
- `ExportNormals` (Boolean, get/set) — Enable/disable export of vertex normals, if they exist.
- `ExportObjectNames` (FileObjWriteOptions.ObjObjectNames, get/set) — Setting to determine what object names in Rhino will become in the OBJ output file
- `ExportOpenMeshes` (Boolean, get/set) — Enable/Disable bailing when an open mesh is encountered.
- `ExportTcs` (Boolean, get/set) — Enable/disable export of texture coordinates, if they exist.
- `ExportVcs` (Boolean, get/set) — Enable/disable export of vertex colors, if they exist.
- `IncludeUnweldedEdgesInNgons` (Boolean, get/set) — Setting to determine whether unwelded edges are ignored in the creation of an ngon.
- `MapZtoY` (Boolean, get/set) — Setting to transform Rhino's Z axis to OBJ's Y axis
- `MergeNestedGroupingNames` (Boolean, get/set) — Setting to merge nested layer or group names into a single OBJ group name
- `MeshParameters` (MeshingParameters, get/set) — Mesh parameters to use when meshing geometry that is not already a mesh.
- `MeshType` (FileObjWriteOptions.VertexWelding, get/set) — 
- `MinNgonFaceCount` (Int32, get/set) — Minimum number of faces to consider creation of ngon
- `NgonMode` (FileObjWriteOptions.NGons, get/set) — 
- `ObjectType` (FileObjWriteOptions.GeometryType, get/set) — 
- `SignificantDigits` (Int32, get/set) — Number of significant digits to write out for floating point numbers
- `SortObjGroups` (Boolean, get/set) — Setting to enable/disable sorting of OBJ groups
- `SubDMeshType` (FileObjWriteOptions.SubDMeshing, get/set) — 
- `SubDSurfaceMeshingDensity` (Int32, get/set) — Determines how coarse the mesh output will be when surface meshing subd objects See comments for ON_SubDDisplayParameters in opennurbs_mesh.h for details regarding numbers used.
- `TrimCurveType` (FileObjWriteOptions.CurveType, get/set) — trimming curve option
- `UnderbarMaterialNames` (Boolean, get/set) — Enable/disable replacing white space with under-bars in material names.
- `UseDisplayColorForMaterial` (Boolean, get/set) — Setting to display color as material when material index for object is -1.
- `UseRelativeIndexing` (Boolean, get/set) — Determines whether to use relative indexing. TRUE = use relative (negative) indexing FALSE = use absolute (positive) indexing
- `UseRenderMeshes` (Boolean, get/set) — Use existing or generate render meshes instead of calling RhinoObject.MeshObjects.
- `UseSimpleDialog` (Boolean, get/set) — Determines whether to use the simple or detailed meshing dialog.
- `WrapLongLines` (Boolean, get/set) — Setting to enable/disable line wrapping with "\"s
- `AngleTolRadians` (Double, get) — If trim is TL_OBJ_FILE_TRIM_POLYLINE, angle_tol (in radians) controls the level of linearization on parameter space trimming curves.
- `WriteOptions` (FileWriteOptions, get) — 

## FileObjWriteOptions.AsciiEol (enum)

End of Line

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_FileObjWriteOptions_AsciiEol.htm)

### Values
- `Crlf` = `0` — MicroSoft
- `Lf` = `1` — UNIX
- `Cr` = `2` — Apple

## FileObjWriteOptions.CurveType (enum)

Type of curve used for trimmed surfaces

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_FileObjWriteOptions_CurveType.htm)

### Values
- `Polyline` = `0` — Polyline approximation, see comments for AngleTolRadians
- `Nurbs` = `1`

## FileObjWriteOptions.GeometryType (enum)

(trimmed) NURBS surfaces may be exported as either NURBS or meshes

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_FileObjWriteOptions_GeometryType.htm)

### Values
- `Nurbs` = `0`
- `Mesh` = `1`

## FileObjWriteOptions.NGons (enum)

[Missing <summary> documentation for "T:Rhino.FileIO.FileObjWriteOptions.NGons"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_FileObjWriteOptions_NGons.htm)

### Values
- `None` = `0` — Ngons will not be created, nor will they be exported if they already exist
- `Preserve` = `1` — If ngons already exist, they will be used as is, no ngons will be created
- `Create` = `2` — Ngons will be created, if possible, by calling Mesh.Ngons.AddPlanarNgons

## FileObjWriteOptions.ObjGroupNames (enum)

[Missing <summary> documentation for "T:Rhino.FileIO.FileObjWriteOptions.ObjGroupNames"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_FileObjWriteOptions_ObjGroupNames.htm)

### Values
- `NoGroups` = `0` — Neither layer or group names are exported as OBJ groups
- `LayerAsGroup` = `1` — Rhino layer names are exported as OBJ groups
- `GroupAsGroup` = `2` — Rhino group names are exported as OBJ groups

## FileObjWriteOptions.ObjObjectNames (enum)

[Missing <summary> documentation for "T:Rhino.FileIO.FileObjWriteOptions.ObjObjectNames"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_FileObjWriteOptions_ObjObjectNames.htm)

### Values
- `NoObjects` = `0` — Object names are not exported
- `ObjectAsGroup` = `1` — Rhino Object names are exported as OBJ groups
- `ObjectAsObject` = `2` — Rhino Object names are exported as OBJ objects

## FileObjWriteOptions.SubDMeshing (enum)

Determines whether the surface or control net of a SubD object is used to generate a mesh

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_FileObjWriteOptions_SubDMeshing.htm)

### Values
- `Surface` = `0` — The SubD surface is used
- `ControlNet` = `1` — The SubD control net is used

## FileObjWriteOptions.VertexWelding (enum)

Determines how/if vertexes of the mesh in Rhino will be modified in the output

**Remarks:** The actual values of any vertex, normal or texture coordinate are not modified, this setting determines whether they are duplicated or merged.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_FileObjWriteOptions_VertexWelding.htm)

### Values
- `Normal` = `0` — Mesh is exported in existing state
- `Welded` = `1` — Mesh topology vertex indexing is used for the v in the OBJ output file normals and texture coordinates, if they exist, come from the mesh
- `Unwelded` = `2` — Each face gets it's own vertex, and normal and texture coordinates if they exist, in the output

## FilePdf (class)

Support for PDF file format

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_FilePdf.htm)

### Constructors
- `protected FilePdf()` — Initializes a new instance of the FilePdf class

### Methods
#### `public abstract int AddPage(int widthInDots, int heightInDots, int dotsPerInch)`

Add a blank page to this document

**Parameters:**
- `widthInDots` (System.Int32) — [Missing <param name="widthInDots"/> documentation for "M:Rhino.FileIO.FilePdf.AddPage(System.Int32,System.Int32,System.Int32)"]
- `heightInDots` (System.Int32) — [Missing <param name="heightInDots"/> documentation for "M:Rhino.FileIO.FilePdf.AddPage(System.Int32,System.Int32,System.Int32)"]
- `dotsPerInch` (System.Int32) — [Missing <param name="dotsPerInch"/> documentation for "M:Rhino.FileIO.FilePdf.AddPage(System.Int32,System.Int32,System.Int32)"]

**Returns:** `Int32` — page number on success

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_FilePdf_AddPage_1.htm)

#### `public abstract int AddPage(ViewCaptureSettings settings)`

Add a new page to this document and draw a viewport into it based on provided ViewCaptureSettings

**Parameters:**
- `settings` (Rhino.Display.ViewCaptureSettings) — [Missing <param name="settings"/> documentation for "M:Rhino.FileIO.FilePdf.AddPage(Rhino.Display.ViewCaptureSettings)"]

**Returns:** `Int32` — page number on success

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_FilePdf_AddPage.htm)

#### `public static FilePdf Create()`

Create a new instance of a FilePdf class

**Returns:** `FilePdf` — [Missing <returns> documentation for "M:Rhino.FileIO.FilePdf.Create"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_FilePdf_Create.htm)

#### `public abstract void DrawBitmap(int pageNumber, Bitmap bitmap, float left, float top, float width, float height, float rotationInDegrees)`

Draw a bitmap

**Parameters:**
- `pageNumber` (System.Int32) — [Missing <param name="pageNumber"/> documentation for "M:Rhino.FileIO.FilePdf.DrawBitmap(System.Int32,System.Drawing.Bitmap,System.Single,System.Single,System.Single,System.Single,System.Single)"]
- `bitmap` (System.Drawing.Bitmap) — [Missing <param name="bitmap"/> documentation for "M:Rhino.FileIO.FilePdf.DrawBitmap(System.Int32,System.Drawing.Bitmap,System.Single,System.Single,System.Single,System.Single,System.Single)"]
- `left` (System.Single) — [Missing <param name="left"/> documentation for "M:Rhino.FileIO.FilePdf.DrawBitmap(System.Int32,System.Drawing.Bitmap,System.Single,System.Single,System.Single,System.Single,System.Single)"]
- `top` (System.Single) — [Missing <param name="top"/> documentation for "M:Rhino.FileIO.FilePdf.DrawBitmap(System.Int32,System.Drawing.Bitmap,System.Single,System.Single,System.Single,System.Single,System.Single)"]
- `width` (System.Single) — [Missing <param name="width"/> documentation for "M:Rhino.FileIO.FilePdf.DrawBitmap(System.Int32,System.Drawing.Bitmap,System.Single,System.Single,System.Single,System.Single,System.Single)"]
- `height` (System.Single) — [Missing <param name="height"/> documentation for "M:Rhino.FileIO.FilePdf.DrawBitmap(System.Int32,System.Drawing.Bitmap,System.Single,System.Single,System.Single,System.Single,System.Single)"]
- `rotationInDegrees` (System.Single) — [Missing <param name="rotationInDegrees"/> documentation for "M:Rhino.FileIO.FilePdf.DrawBitmap(System.Int32,System.Drawing.Bitmap,System.Single,System.Single,System.Single,System.Single,System.Single)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_FilePdf_DrawBitmap.htm)

#### `public void DrawLine(int pageNumber, PointF from, PointF to, Color strokeColor, float strokeWidth)`

Draw a line

**Parameters:**
- `pageNumber` (System.Int32) — [Missing <param name="pageNumber"/> documentation for "M:Rhino.FileIO.FilePdf.DrawLine(System.Int32,System.Drawing.PointF,System.Drawing.PointF,System.Drawing.Color,System.Single)"]
- `from` (System.Drawing.PointF) — [Missing <param name="from"/> documentation for "M:Rhino.FileIO.FilePdf.DrawLine(System.Int32,System.Drawing.PointF,System.Drawing.PointF,System.Drawing.Color,System.Single)"]
- `to` (System.Drawing.PointF) — [Missing <param name="to"/> documentation for "M:Rhino.FileIO.FilePdf.DrawLine(System.Int32,System.Drawing.PointF,System.Drawing.PointF,System.Drawing.Color,System.Single)"]
- `strokeColor` (System.Drawing.Color) — [Missing <param name="strokeColor"/> documentation for "M:Rhino.FileIO.FilePdf.DrawLine(System.Int32,System.Drawing.PointF,System.Drawing.PointF,System.Drawing.Color,System.Single)"]
- `strokeWidth` (System.Single) — [Missing <param name="strokeWidth"/> documentation for "M:Rhino.FileIO.FilePdf.DrawLine(System.Int32,System.Drawing.PointF,System.Drawing.PointF,System.Drawing.Color,System.Single)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_FilePdf_DrawLine.htm)

#### `public abstract void DrawPolyline(int pageNumber, PointF[] polyline, Color fillColor, Color strokeColor, float strokeWidth)`

Draw a polyline path

**Parameters:**
- `pageNumber` (System.Int32) — [Missing <param name="pageNumber"/> documentation for "M:Rhino.FileIO.FilePdf.DrawPolyline(System.Int32,System.Drawing.PointF[],System.Drawing.Color,System.Drawing.Color,System.Single)"]
- `polyline` (System.Drawing.PointF[]) — [Missing <param name="polyline"/> documentation for "M:Rhino.FileIO.FilePdf.DrawPolyline(System.Int32,System.Drawing.PointF[],System.Drawing.Color,System.Drawing.Color,System.Single)"]
- `fillColor` (System.Drawing.Color) — [Missing <param name="fillColor"/> documentation for "M:Rhino.FileIO.FilePdf.DrawPolyline(System.Int32,System.Drawing.PointF[],System.Drawing.Color,System.Drawing.Color,System.Single)"]
- `strokeColor` (System.Drawing.Color) — [Missing <param name="strokeColor"/> documentation for "M:Rhino.FileIO.FilePdf.DrawPolyline(System.Int32,System.Drawing.PointF[],System.Drawing.Color,System.Drawing.Color,System.Single)"]
- `strokeWidth` (System.Single) — [Missing <param name="strokeWidth"/> documentation for "M:Rhino.FileIO.FilePdf.DrawPolyline(System.Int32,System.Drawing.PointF[],System.Drawing.Color,System.Drawing.Color,System.Single)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_FilePdf_DrawPolyline.htm)

#### `public abstract void DrawText(int pageNumber, string text, double x, double y, float heightPoints, Font onfont, Color fillColor, Color strokeColor, float strokeWidth, float angleDegrees, TextHorizontalAlignment horizontalAlignment, TextVerticalAlignment verticalAlignment)`

Draw text on a page

**Parameters:**
- `pageNumber` (System.Int32) — [Missing <param name="pageNumber"/> documentation for "M:Rhino.FileIO.FilePdf.DrawText(System.Int32,System.String,System.Double,System.Double,System.Single,Rhino.DocObjects.Font,System.Drawing.Color,System.Drawing.Color,System.Single,System.Single,Rhino.DocObjects.TextHorizontalAlignment,Rhino.DocObjects.TextVerticalAlignment)"]
- `text` (System.String) — [Missing <param name="text"/> documentation for "M:Rhino.FileIO.FilePdf.DrawText(System.Int32,System.String,System.Double,System.Double,System.Single,Rhino.DocObjects.Font,System.Drawing.Color,System.Drawing.Color,System.Single,System.Single,Rhino.DocObjects.TextHorizontalAlignment,Rhino.DocObjects.TextVerticalAlignment)"]
- `x` (System.Double) — [Missing <param name="x"/> documentation for "M:Rhino.FileIO.FilePdf.DrawText(System.Int32,System.String,System.Double,System.Double,System.Single,Rhino.DocObjects.Font,System.Drawing.Color,System.Drawing.Color,System.Single,System.Single,Rhino.DocObjects.TextHorizontalAlignment,Rhino.DocObjects.TextVerticalAlignment)"]
- `y` (System.Double) — [Missing <param name="y"/> documentation for "M:Rhino.FileIO.FilePdf.DrawText(System.Int32,System.String,System.Double,System.Double,System.Single,Rhino.DocObjects.Font,System.Drawing.Color,System.Drawing.Color,System.Single,System.Single,Rhino.DocObjects.TextHorizontalAlignment,Rhino.DocObjects.TextVerticalAlignment)"]
- `heightPoints` (System.Single) — [Missing <param name="heightPoints"/> documentation for "M:Rhino.FileIO.FilePdf.DrawText(System.Int32,System.String,System.Double,System.Double,System.Single,Rhino.DocObjects.Font,System.Drawing.Color,System.Drawing.Color,System.Single,System.Single,Rhino.DocObjects.TextHorizontalAlignment,Rhino.DocObjects.TextVerticalAlignment)"]
- `onfont` (Rhino.DocObjects.Font) — [Missing <param name="onfont"/> documentation for "M:Rhino.FileIO.FilePdf.DrawText(System.Int32,System.String,System.Double,System.Double,System.Single,Rhino.DocObjects.Font,System.Drawing.Color,System.Drawing.Color,System.Single,System.Single,Rhino.DocObjects.TextHorizontalAlignment,Rhino.DocObjects.TextVerticalAlignment)"]
- `fillColor` (System.Drawing.Color) — [Missing <param name="fillColor"/> documentation for "M:Rhino.FileIO.FilePdf.DrawText(System.Int32,System.String,System.Double,System.Double,System.Single,Rhino.DocObjects.Font,System.Drawing.Color,System.Drawing.Color,System.Single,System.Single,Rhino.DocObjects.TextHorizontalAlignment,Rhino.DocObjects.TextVerticalAlignment)"]
- `strokeColor` (System.Drawing.Color) — [Missing <param name="strokeColor"/> documentation for "M:Rhino.FileIO.FilePdf.DrawText(System.Int32,System.String,System.Double,System.Double,System.Single,Rhino.DocObjects.Font,System.Drawing.Color,System.Drawing.Color,System.Single,System.Single,Rhino.DocObjects.TextHorizontalAlignment,Rhino.DocObjects.TextVerticalAlignment)"]
- `strokeWidth` (System.Single) — [Missing <param name="strokeWidth"/> documentation for "M:Rhino.FileIO.FilePdf.DrawText(System.Int32,System.String,System.Double,System.Double,System.Single,Rhino.DocObjects.Font,System.Drawing.Color,System.Drawing.Color,System.Single,System.Single,Rhino.DocObjects.TextHorizontalAlignment,Rhino.DocObjects.TextVerticalAlignment)"]
- `angleDegrees` (System.Single) — [Missing <param name="angleDegrees"/> documentation for "M:Rhino.FileIO.FilePdf.DrawText(System.Int32,System.String,System.Double,System.Double,System.Single,Rhino.DocObjects.Font,System.Drawing.Color,System.Drawing.Color,System.Single,System.Single,Rhino.DocObjects.TextHorizontalAlignment,Rhino.DocObjects.TextVerticalAlignment)"]
- `horizontalAlignment` (Rhino.DocObjects.TextHorizontalAlignment) — [Missing <param name="horizontalAlignment"/> documentation for "M:Rhino.FileIO.FilePdf.DrawText(System.Int32,System.String,System.Double,System.Double,System.Single,Rhino.DocObjects.Font,System.Drawing.Color,System.Drawing.Color,System.Single,System.Single,Rhino.DocObjects.TextHorizontalAlignment,Rhino.DocObjects.TextVerticalAlignment)"]
- `verticalAlignment` (Rhino.DocObjects.TextVerticalAlignment) — [Missing <param name="verticalAlignment"/> documentation for "M:Rhino.FileIO.FilePdf.DrawText(System.Int32,System.String,System.Double,System.Double,System.Single,Rhino.DocObjects.Font,System.Drawing.Color,System.Drawing.Color,System.Single,System.Single,Rhino.DocObjects.TextHorizontalAlignment,Rhino.DocObjects.TextVerticalAlignment)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_FilePdf_DrawText.htm)

#### `protected void FirePreWriteEvent()`

Called by the framework to fire a PreWrite event

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_FilePdf_FirePreWriteEvent.htm)

#### `public abstract Object PdfDocumentImplementation()`

Get actual implementation of PdfDocument class

**Returns:** `Object` — [Missing <returns> documentation for "M:Rhino.FileIO.FilePdf.PdfDocumentImplementation"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_FilePdf_PdfDocumentImplementation.htm)

#### `public abstract void Write(Stream stream)`

Write PDF to a stream

**Parameters:**
- `stream` (System.IO.Stream) — [Missing <param name="stream"/> documentation for "M:Rhino.FileIO.FilePdf.Write(System.IO.Stream)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_FilePdf_Write.htm)

#### `public abstract void Write(string filename)`

Write PDF to a file

**Parameters:**
- `filename` (System.String) — [Missing <param name="filename"/> documentation for "M:Rhino.FileIO.FilePdf.Write(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_FilePdf_Write_1.htm)

### Properties
- `LayersAsOptionalContentGroups` (Boolean, get/set) — Add layers as "optional content groups" to the PDF. This is the visible layer tree available in PDF viewers

### Events
#### `PreWrite` (`System.EventHandler<FilePdfEventArgs>`)

**Signature:** `public static event EventHandler<FilePdfEventArgs> PreWrite`

Fired immediately before a PDF is written

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/E_Rhino_FileIO_FilePdf_PreWrite.htm)

## FilePdfEventArgs (class)

Used for events that are fired while constructing/saving a PDF

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_FilePdfEventArgs.htm)

### Properties
- `Pdf` (FilePdf, get) — The document that is about to be written

## FilePly (class)

Support for ply file format

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_FilePly.htm)

### Methods
#### `public static WriteFileResult Write(string filename, RhinoDoc doc, FilePlyWriteOptions options)`

Write a ply file based on the contents of a RhinoDoc

**Parameters:**
- `filename` (System.String) — [Missing <param name="filename"/> documentation for "M:Rhino.FileIO.FilePly.Write(System.String,Rhino.RhinoDoc,Rhino.FileIO.FilePlyWriteOptions)"]
- `doc` (Rhino.RhinoDoc) — [Missing <param name="doc"/> documentation for "M:Rhino.FileIO.FilePly.Write(System.String,Rhino.RhinoDoc,Rhino.FileIO.FilePlyWriteOptions)"]
- `options` (Rhino.FileIO.FilePlyWriteOptions) — [Missing <param name="options"/> documentation for "M:Rhino.FileIO.FilePly.Write(System.String,Rhino.RhinoDoc,Rhino.FileIO.FilePlyWriteOptions)"]

**Returns:** `WriteFileResult` — [Missing <returns> documentation for "M:Rhino.FileIO.FilePly.Write(System.String,Rhino.RhinoDoc,Rhino.FileIO.FilePlyWriteOptions)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_FilePly_Write.htm)

## FilePlyWriteOptions (class)

[Missing <summary> documentation for "T:Rhino.FileIO.FilePlyWriteOptions"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_FilePlyWriteOptions.htm)

### Constructors
- `public FilePlyWriteOptions(FileWriteOptions writeOptions)` — Initializes a new instance of the FilePlyWriteOptions class

### Properties
- `ExportASCII` (Boolean, get/set) — Determines whether to export as Ascii.
- `ExportColors` (Boolean, get/set) — Determines whether to export vertex colors.
- `ExportDoubles` (Boolean, get/set) — Determines whether vertexes are exported as doubles or floats.
- `ExportMaterial` (Boolean, get/set) — Determines whether to export a material.
- `ExportNormals` (Boolean, get/set) — Determines whether to export vertex normals.
- `MeshingParameters` (MeshingParameters, get/set) — 
- `UseSimpleDialog` (Boolean, get/set) — Determines whether to use the simple or detailed meshing dialog.
- `WriteOptions` (FileWriteOptions, get) — 

## FileReadOptions (class)

[Missing <summary> documentation for "T:Rhino.FileIO.FileReadOptions"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_FileReadOptions.htm)

### Constructors
- `public FileReadOptions()` — Initializes a new instance of the FileReadOptions class

### Methods
#### `public void Dispose()`

Releases all resources used by the FileReadOptions

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_FileReadOptions_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

Releases the unmanaged resources used by the FileReadOptions and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_FileReadOptions_Dispose_1.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_FileReadOptions_Finalize.htm)

### Properties
- `BatchMode` (Boolean, get/set) — true means you cannot ask questions during reading. (no dialogs, no "getters", etc.)
- `ImportMode` (Boolean, get/set) — true means we are merging whatever is being read into an existing document. This means you need to consider things like: If the information being read is in a different unit system, it should be scaled if UseScaleGeometry is true. There can be existing layers, fonts, materials, dimension styles, hatch patterns, and so on with the same name as items being read from the file.
- `ImportReferenceMode` (Boolean, get/set) — true means we are reading information for a work session reference model or a linked instance definition.
- `InsertMode` (Boolean, get/set) — true means we are reading information that will be used to create an instance definition or some other type of "inserting" that is supported by Rhino's "Insert" command.
- `NewMode` (Boolean, get/set) — true means we are reading template information in something like a OnFileNew event.
- `OpenMode` (Boolean, get/set) — true means we are reading the information into an empty document. This means you need to consider things like: Setting the unit system (if the file has a unit system)Creating a default layer if one is not there.Setting up appropriate views when you're finished reading.
- `ScaleGeometry` (Boolean, get/set) — true: If ImportMode is true and the geometry in the file being read has a unit system different from the model's unit system, then apply the unit conversion scale to the file's geometry before adding it to the model. false: Do not scale. Once case where this happens is when an instance definition is read from a file and the model space instance references have been scaled. In case the instance definition geometry cannot be scaled or the net result is that the size of the instance reference object is scaled by the square of the scale factor.
- `UseScaleGeometry` (Boolean, get/set) — If this parameter is true, then no questions are asked when unit conversion scaling is optional and the setting specified by ScaleGeometry is used.

## FileReference (class)

Manages a reference to an existing or non-existing file, using either or both absolute or relative paths. Once constructed, this class is immutable.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_FileReference.htm)

### Constructors
- `public FileReference(string fullPath, string relativePath, ContentHash hash, FileReferenceStatus status)` — Constructs a new instance of the FileReference class, given a fullPath, a relativePath a content hash and a status value.

### Methods
#### `public static FileReference CreateFromFullAndRelativePaths(string fullPath, string relativePath)`

Returns a new file reference. This returns a new instance even if the path does not exist.

**Parameters:**
- `fullPath` (System.String) — A full path. This parameter cannot be null.
- `relativePath` (System.String) — A relative path. This parameter can be null.

**Returns:** `FileReference` — A file reference to the specified paths.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_FileReference_CreateFromFullAndRelativePaths.htm)

#### `public static FileReference CreateFromFullPath(string fullPath)`

Returns a new file reference. This returns a new instance even if the path does not exist.

**Parameters:**
- `fullPath` (System.String) — A full path.

**Returns:** `FileReference` — A file reference to the specified path.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_FileReference_CreateFromFullPath.htm)

#### `public void Dispose()`

Reclaims unmanaged resources used by this object.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_FileReference_Dispose.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_FileReference_Finalize.htm)

### Properties
- `ContentHash` (ContentHash, get) — Gets the content hash.
- `FullPath` (String, get) — Gets the absolute path of this file reference.
- `FullPathStatus` (FileReferenceStatus, get) — Gets the file reference status.
- `IsSet` (Boolean, get) — Returns an indication of the fact that the reference is actually set to a non-null value.
- `RelativePath` (String, get) — Gets the relative path of this file reference.

## FileReferenceStatus (enum)

Enumerates a list of file statuses.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_FileReferenceStatus.htm)

### Values
- `Unknown` = `0` — Status of a the full path is not known.
- `FullPathValid` = `1` — Full path is valid.
- `FileNotFound` = `2` — Unable to locate file.

## FileSlc (class)

Support for writing slice (SLC) file format

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_FileSlc.htm)

### Methods
#### `public static bool Write(string filename, RhinoDoc doc, FileSlcWriteOptions options)`

Write a SLC file based on the contents of a RhinoDoc

**Parameters:**
- `filename` (System.String) — path to write a file to
- `doc` (Rhino.RhinoDoc) — document to get geometry from
- `options` (Rhino.FileIO.FileSlcWriteOptions) — options used for generating the SLC file

**Returns:** `Boolean` — true on success

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_FileSlc_Write.htm)

## FileSlcWriteOptions (class)

Options used when writing a SLC file

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_FileSlcWriteOptions.htm)

### Constructors
- `public FileSlcWriteOptions()` — Initializes a new instance of the FileSlcWriteOptions class

### Properties
- `AngleBetweenSegmentsDegrees` (Double, get/set) — The angle that determines how smooth the polylines of the slice curves will be. When the number is small you will get a smooth final output but it will take more time to export and a larger file.
- `EndPoint` (Point3d, get/set) — End of the slicing normal
- `SliceDistance` (Double, get/set) — The distance between the slices or layers of curves that your final output will contain. The distance should be based on the slice thickness of your final output device
- `StartPoint` (Point3d, get/set) — Start of the slicing normal
- `UseMeshes` (Boolean, get/set) — Use meshes to generate slices. The curves for each slice are generated by intersecting the object mesh with a plane. the Angle between polyline segments data is not used when this value is true

## FileStp (class)

Support for writing STEP (STP) file format

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_FileStp.htm)

### Methods
#### `public static bool Write(string filename, RhinoDoc doc, FileStpWriteOptions options)`

Write a STP file based on the contents of a RhinoDoc

**Parameters:**
- `filename` (System.String) — path to write a file to
- `doc` (Rhino.RhinoDoc) — document to get geometry from
- `options` (Rhino.FileIO.FileStpWriteOptions) — options used for generating the STP file

**Returns:** `Boolean` — true on success

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_FileStp_Write.htm)

## FileStpWriteOptions (class)

Options used when writing a STP file

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_FileStpWriteOptions.htm)

### Constructors
- `public FileStpWriteOptions()` — Initializes a new instance of the FileStpWriteOptions class

### Properties
- `Export2dCurves` (Boolean, get/set) — Some (not most, and not Rhino) importing applications can make use of the 2-D trimming curves to get a more accurate and faster import. The size of the step file will be larger
- `ExportBlack` (Boolean, get/set) — Let importing application set color for black objects. If a Rhino object has color black, no color is assigned to the object in the step file.This will cause the importing application to give the object its default color.This is desirable because black objects look like ink blots in some applications.This option is grayed out if the schema option is AP203ControConfigDesign since that schema does not include color entities.
- `SplitClosedSurfaces` (Boolean, get/set) — Splits closed surfaces, for example, the interior surfaces of drilled holes.

## FileType (class)

[Missing <summary> documentation for "T:Rhino.FileIO.FileType"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_FileType.htm)

### Constructors
- `public FileType(string extension, string description)` — Initializes a new instance of the FileType class

### Properties
- `Description` (String, get) — 
- `Extension` (String, get) — 

## FileWriteOptions (class)

[Missing <summary> documentation for "T:Rhino.FileIO.FileWriteOptions"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_FileWriteOptions.htm)

### Constructors
- `public FileWriteOptions()` — Initializes a new instance of the FileWriteOptions class

### Methods
#### `public void Dispose()`

Releases all resources used by the FileWriteOptions

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_FileWriteOptions_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

Releases the unmanaged resources used by the FileWriteOptions and optionally releases the managed resources

**Parameters:**
- `disposing` (System.Boolean) — True to release both managed and unmanaged resources; false to release only unmanaged resources

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_FileWriteOptions_Dispose_1.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_FileWriteOptions_Finalize.htm)

### Properties
- `DestinationFileName` (String, get) — For use on Apple frameworks only. Retrns the final destination file name.
- `FileVersion` (Int32, get/set) — 
- `IncludeBitmapTable` (Boolean, get/set) — The file written should include the bitmap table if your File Writing Plug-in supports it.
- `IncludeHistory` (Boolean, get/set) — The file written should include history information if your File Writing Plug-In supports it.
- `IncludePreviewImage` (Boolean, get/set) — The file written should include a preview image if your File Writing Plug-in supports it.
- `IncludeRenderMeshes` (Boolean, get/set) — The file written should include the render meshes if your File Writing Plug-in supports it.
- `RhinoDoc` (RhinoDoc, get) — Source RhinoDoc that is being written
- `SuppressAllInput` (Boolean, get/set) — 
- `SuppressDialogBoxes` (Boolean, get/set) — If true, it means the command has been run with a '-', meaning you should not ask questions during writing. (no dialogs, no "getters", etc.)
- `UpdateDocumentPath` (Boolean, get/set) — If a complete, current version, 3dm file is successfully saved, then the name of the file will be used to update the document's default file path and title and document will be marked as not modified.
- `WriteAsTemplate` (Boolean, get) — Write as template
- `WriteGeometryOnly` (Boolean, get/set) — If true, the file written should include only geometry File Writing Plug-in supports it.
- `WriteSelectedObjectsOnly` (Boolean, get/set) — If true, this command should export only the objects currently selected in the Rhino model.
- `WriteUserData` (Boolean, get/set) — If true, the file written should include User Data if your File Writing Plug-in supports it.
- `Xform` (Transform, get/set) — 

## ICommonComponentTable<T> (interface)

Provides methods to use all File3dm and RhinoDoc tables under the same contract. Do not derive from this interface. This is to ensure all tables can be used with the same method list.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_ICommonComponentTable_1.htm)

### Methods
#### `T FindId(Guid id)`

Retrieves an object based on ID. You should prefer ID search over Index search.

**Parameters:**
- `id` (System.Guid) — The id to search for.

**Returns:** `T` — A model component, or null if none was found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_ICommonComponentTable_1_FindId.htm)

#### `T FindNameHash(NameHash nameHash)`

Retrieves an object based on Name.

**Parameters:**
- `nameHash` (Rhino.FileIO.NameHash) — The name hash for which to search.

**Returns:** `T` — [Missing <returns> documentation for "M:Rhino.FileIO.ICommonComponentTable`1.FindNameHash(Rhino.FileIO.NameHash)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_ICommonComponentTable_1_FindNameHash.htm)

### Properties
- `ComponentType` (ModelComponentType, get) — Returns the model component type the table handles.

## ImageFile (class)

Support functions for image files

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_ImageFile.htm)

### Methods
#### `public static bool SupportsAlphaChannel(string filename)`

Returns true if file at given path is an image file and that file format supports an alpha channel

**Parameters:**
- `filename` (System.String) — [Missing <param name="filename"/> documentation for "M:Rhino.FileIO.ImageFile.SupportsAlphaChannel(System.String)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.FileIO.ImageFile.SupportsAlphaChannel(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_ImageFile_SupportsAlphaChannel.htm)

## ManifestTable (class)

Maintains an index to every model component that is in the 3dm file. This is the "more comprehensive" table that contains all objects in all other tables.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_ManifestTable.htm)

### Methods
#### `public int ActiveObjectCount(ModelComponentType type)`

Total number of items in the manifest, including deleted items.

**Parameters:**
- `type` (Rhino.DocObjects.ModelComponentType) — [Missing <param name="type"/> documentation for "M:Rhino.FileIO.ManifestTable.ActiveObjectCount(Rhino.DocObjects.ModelComponentType)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.FileIO.ManifestTable.ActiveObjectCount(Rhino.DocObjects.ModelComponentType)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_ManifestTable_ActiveObjectCount.htm)

#### `public virtual void Clear()`

Marks all items as deleted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_ManifestTable_Clear.htm)

#### `public bool Contains(ModelComponent item)`

Determines if an items is contained in this table.

**Parameters:**
- `item` (Rhino.DocObjects.ModelComponent) — An item, or null. Null is never contained.

**Returns:** `Boolean` — True if the item is contained; otherwise, false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_ManifestTable_Contains.htm)

#### `public void CopyTo(ModelComponent[] array, int arrayIndex)`

Copies the content of this table to an array.

**Parameters:**
- `array` (Rhino.DocObjects.ModelComponent[]) — The array to copy to.
- `arrayIndex` (System.Int32) — The position in the array from which to start copying.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_ManifestTable_CopyTo.htm)

#### `public abstract ModelComponent FindId(Guid id)`

Uses the guid to find a model component. Deleted objects cannot be found by id. The guid is the value that is stored in the .Id property. In a single document, no two active objects have the same guid. If an object is replaced with a new object, then the guid persists. For example, if the _Move command moves an object, then the moved object inherits its guid from the starting object. If the Copy command copies an object, then the copy gets a new guid. This guid persists through file saving/opening operations. This function will not find grip objects.

**Parameters:**
- `id` (System.Guid) — ID of model component to search for.

**Returns:** `ModelComponent` — Reference to the rhino object with the objectId or null if no such object could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_ManifestTable_FindId.htm)

#### `public abstract ModelComponent FindId(Guid id, ModelComponentType type)`

Uses the guid to find a model component. Deleted objects cannot be found by id. The guid is the value that is stored in the .Id property. In a single document, no two active objects have the same guid. If an object is replaced with a new object, then the guid persists. For example, if the _Move command moves an object, then the moved object inherits its guid from the starting object. If the Copy command copies an object, then the copy gets a new guid. This guid persists through file saving/opening operations. This function will not find grip objects.

**Parameters:**
- `id` (System.Guid) — ID of model component to search for.
- `type` (Rhino.DocObjects.ModelComponentType) — The type to be searched. If this is Unset then all types are searched.

**Returns:** `ModelComponent` — Reference to the rhino object with the objectId or null if no such object could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_ManifestTable_FindId_1.htm)

#### `public T FindId<T>(Guid id) where T : ModelComponent`

Uses the guid to find a model component. Deleted objects cannot be found by id. The guid is the value that is stored in the .Id property. In a single document, no two active objects have the same guid. If an object is replaced with a new object, then the guid persists. For example, if the _Move command moves an object, then the moved object inherits its guid from the starting object. If the Copy command copies an object, then the copy gets a new guid. This guid persists through file saving/opening operations. This function will not find grip objects.

**Parameters:**
- `id` (System.Guid) — Index of model component to search for.

**Returns:** `T` — Reference to the rhino object or null if no such object could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_ManifestTable_FindId__1.htm)

#### `public abstract ModelComponent FindIndex(int index, ModelComponentType type)`

Uses the index to find a model component. The index is the value that is stored in the .Index property.

**Parameters:**
- `index` (System.Int32) — Index of model component to search for.
- `type` (Rhino.DocObjects.ModelComponentType) — The type to be searched. Cannot be Unset.

**Returns:** `ModelComponent` — Reference to the rhino object or null if no such object could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_ManifestTable_FindIndex.htm)

#### `public T FindIndex<T>(int index) where T : ModelComponent`

Uses the index to find a model component. The index is the value that is stored in the .Index property.

**Parameters:**
- `index` (System.Int32) — Index of model component to search for.

**Returns:** `T` — Reference to the rhino object or null if no such object could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_ManifestTable_FindIndex__1.htm)

#### `public abstract ModelComponent FindName(string name, ModelComponentType type, Guid parent)`

Uses the name to find a model component. The name is the value that is stored in the .Name property. Deleted objects have no name.

**Parameters:**
- `name` (System.String) — Name of model component to search for.
- `type` (Rhino.DocObjects.ModelComponentType) — The type to be searched. Cannot be Unset.
- `parent` (System.Guid) — Parent object id. This is only required for layers.

**Returns:** `ModelComponent` — Reference to the rhino object or null if no such object could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_ManifestTable_FindName.htm)

#### `public T FindName<T>(string name, Guid parent) where T : ModelComponent`

Uses the name to find a model component. The name is the value that is stored in the .Name property. Deleted objects have no name.

**Parameters:**
- `name` (System.String) — Name of model component to search for.
- `parent` (System.Guid) — Parent object id. This is only required for layers.

**Returns:** `T` — Reference to the rhino object or null if no such object could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_ManifestTable_FindName__1.htm)

#### `public abstract ModelComponent FindNameHash(NameHash nameHash, ModelComponentType type)`

Uses the hash of the name to find a model component. Deleted objects have no name.

**Parameters:**
- `nameHash` (Rhino.FileIO.NameHash) — NameHash of model component to search for.
- `type` (Rhino.DocObjects.ModelComponentType) — The type to be searched. Cannot be Unset.

**Returns:** `ModelComponent` — Reference to the rhino object or null if no such object could be found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_ManifestTable_FindNameHash.htm)

#### `public T FindNameHash<T>(NameHash nameHash) where T : ModelComponent`

Uses the hash of the name to find a model component. Deleted objects have no name.

**Parameters:**
- `nameHash` (Rhino.FileIO.NameHash) — Name hash of model component to search for.

**Returns:** `T` — [Missing <returns> documentation for "M:Rhino.FileIO.ManifestTable.FindNameHash``1(Rhino.FileIO.NameHash)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_ManifestTable_FindNameHash__1.htm)

#### `public virtual IEnumerator<ModelComponent> GetEnumerator()`

Visits all model components in the document, including default ones.

**Returns:** `IEnumerator<ModelComponent>` — An enumerator.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_ManifestTable_GetEnumerator.htm)

#### `public abstract IEnumerator<ModelComponent> GetEnumerator(ModelComponentType type)`

Returns an enumerators that yields all model components, including default ones, relating to a particular type.

**Parameters:**
- `type` (Rhino.DocObjects.ModelComponentType) — The model component type.

**Returns:** `IEnumerator<ModelComponent>` — An enumerator.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_ManifestTable_GetEnumerator_1.htm)

#### `public virtual IEnumerator<T> GetEnumerator<T>() where T : ModelComponent`

Visits all model components in the document, including default ones.

**Returns:** `IEnumerator<T>` — An enumerator.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_ManifestTable_GetEnumerator__1.htm)

#### `public static ModelComponentType GetModelComponentTypeFromGenericType<T>() where T : ModelComponent`

Returns the result of the ComponentType property of a ModelComponent.

**Returns:** `ModelComponentType` — A ModelComponentType.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_ManifestTable_GetModelComponentTypeFromGenericType__1.htm)

### Properties
- `ComponentType` (ModelComponentType, get) — Returns Mixed.
- `Count` (Int32, get) — Total number of items in the manifest, including deleted items.
- `LongCount` (Int64, get) — Total number of items in the manifest, including deleted items.
- `Parent` (Object, get) — Returns the parent object. This is the RhinoDoc, or the File3md file.

## NameHash (class)

Contains information that is useful to uniquely identify an object name. RemarksThis object is immutable.

**Remarks:** This object is immutable.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_NameHash.htm)

### Constructors
- `public NameHash(string name)` — Creates a new NameHash, representing a piece of text.
- `protected NameHash(NameHash other)` — Constructs a copy of a content hash.
- `public NameHash(string name, Guid parentId)` — Creates a new NameHash, representing a piece of text.
- `public NameHash(string name, Guid parentId, ModelComponentType type)` — Creates a new NameHash, representing a piece of text.
- `public NameHash(string name, Guid parentId, bool ignoreCase)` — Creates a new NameHash, representing a piece of text.

### Methods
#### `public NameHash Clone()`

Creates a copy of this name hash. Because content hash is immutable, this can be used as a deep copy.

**Returns:** `NameHash` — A different instance of the same name hash.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_NameHash_Clone.htm)

#### `public static NameHash CreateFilePathHash(string path)`

Creates a new NameHash, representing the name of a file.

**Parameters:**
- `path` (System.String) — A path. This can be null and can refer to a non-existing path.

**Returns:** `NameHash` — [Missing <returns> documentation for "M:Rhino.FileIO.NameHash.CreateFilePathHash(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_NameHash_CreateFilePathHash.htm)

#### `public static bool operator ==(NameHash left, NameHash right)`

Determines if two NameHash instances are equal by value.

**Parameters:**
- `left` (Rhino.FileIO.NameHash) — The first hash.
- `right` (Rhino.FileIO.NameHash) — The second hash.

**Returns:** `Boolean` — True if they are equal by value, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_NameHash_op_Equality.htm)

#### `public bool Equals(NameHash other)`

Determines if another name hash has the same value.

**Parameters:**
- `other` (Rhino.FileIO.NameHash) — The other name hash to compare.

**Returns:** `Boolean` — True if the two hashes are equal.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_NameHash_Equals.htm)

#### `public override bool Equals(Object obj)`

Determines if another object is a name hash with same value.

**Parameters:**
- `obj` (System.Object) — The other content hash to compare.

**Returns:** `Boolean` — True if the two hashes are equal.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_NameHash_Equals_1.htm)

#### `public override int GetHashCode()`

Gets an hash code for this name hash. Two equal content hashes have equal hash code. The other way around might not be true.

**Returns:** `Int32` — An hash code value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_NameHash_GetHashCode.htm)

#### `public static bool operator !=(NameHash left, NameHash right)`

Determines if two NameHash instances are different by value.

**Parameters:**
- `left` (Rhino.FileIO.NameHash) — The first hash.
- `right` (Rhino.FileIO.NameHash) — The second hash.

**Returns:** `Boolean` — True if they are different by value, otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_NameHash_op_Inequality.htm)

### Properties
- `MappedCodePoints` (UInt32, get) — Gets the NameHash flags. In some cases = number of mapped code points.
- `ParentId` (Guid, get) — Only useful if this participates in a tree structure, as with layers.
- `Sha1Hash` (Byte[], get) — Gets the 20-bytes long SHA-1 hash of ordinal minimum mapped Unicode (UTF-32) code points.

## SHA1OpenNURBS (class)

Provides the OpenNURBS implementation of SHA1. This class is provided only with the purpose of hashing. It is not meant to be used for any cryptographic purpose.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_SHA1OpenNURBS.htm)

### Constructors
- `public SHA1OpenNURBS()` — Constructs a new instance of the SHA1 algorithm.

### Methods
#### `public static byte[] FileSystemPathHash(string path, Nullable<bool> ignoreCase = null)`

Computes the SHA1 hash of a file system path, converted to UTF8. These file system paths have identical values of FileSystemPathHash():/x/y/z/name.ext\x\y\z\name.ext/x//y//z/name.ext/x/y/a/b/c/../../../z/name.ext/X/Y/Z/NAME.EXT (When ignoreCase is true)

**Parameters:**
- `path` (System.String) — A non-null path string.
- `ignoreCase` (System.Nullable<Boolean>) — If case should be ignored. If this is null or unspecified, the operating system default is used.

**Returns:** `Byte[]` — A 20-byte long SHA1 hash.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_SHA1OpenNURBS_FileSystemPathHash.htm)

#### `protected override void Finalize()`

Instructs the runtime to reclaim unmanaged resources if the developer forgot to call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_SHA1OpenNURBS_Finalize.htm)

#### `public override void Initialize()`

Resets this instance of the algorithm, so that it can be used again. It is not required to call this method after creation.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_SHA1OpenNURBS_Initialize.htm)

#### `public static byte[] StringHash(string input)`

Computes the SHA1 hash of a string, converted to UTF8.

**Parameters:**
- `input` (System.String) — [Missing <param name="input"/> documentation for "M:Rhino.FileIO.SHA1OpenNURBS.StringHash(System.String)"]

**Returns:** `Byte[]` — A 20-byte long SHA1 hash.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_SHA1OpenNURBS_StringHash.htm)

## SerializationOptions (class)

Contains options for serializing -or storing- data, such as Rhino version and user data.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_SerializationOptions.htm)

### Constructors
- `public SerializationOptions()` — Initializes a new instance of the SerializationOptions class.

### Properties
- `RhinoVersion` (Int32, get/set) — Gets or sets a value indicating the Rhino version.
- `WriteUserData` (Boolean, get/set) — Gets or sets a value indicating whether to write user data.

## TextLog (class)

Used for collecting text data

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/T_Rhino_FileIO_TextLog.htm)

### Constructors
- `public TextLog()` — Creates a text log that stores all text in memory. Use ToString on this version of the TextLog to get the text that we written
- `public TextLog(IntPtr ptr)` — Initializes a new instance of the TextLog class
- `public TextLog(string filename)` — Creates a text log that writes all text to a file. If no filename is provided, then text is written to StdOut

### Methods
#### `public void Dispose()`

Actively reclaims unmanaged resources that this instance uses.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_TextLog_Dispose.htm)

#### `protected virtual void Dispose(bool disposing)`

For derived class implementers. This method is called with argument true when class user calls Dispose(), while with argument false when the Garbage Collector invokes the finalizer, or Finalize() method.You must reclaim all used unmanaged resources in both cases, and can use this chance to call Dispose on disposable fields if the argument is true.Also, you must call the base virtual method within your overriding method.

**Parameters:**
- `disposing` (System.Boolean) — true if the call comes from the Dispose() method; false if it comes from the Garbage Collector finalizer.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_TextLog_Dispose_1.htm)

#### `protected override void Finalize()`

Passively reclaims unmanaged resources when the class user did not explicitly call Dispose().

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_TextLog_Finalize.htm)

#### `public static TextLog NewCommandLine()`

Returns a reference to a TextLog that prints to the Rhino command line. Each new command line reference holds its own indents.

**Remarks:** All Print methods of this instance are guaranteed to be thread safe. Other methods are not guaranteed to be thread safe.

**Returns:** `TextLog` — [Missing <returns> documentation for "M:Rhino.FileIO.TextLog.NewCommandLine"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_TextLog_NewCommandLine.htm)

#### `public void PopIndent()`

Decrease the indentation level

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_TextLog_PopIndent.htm)

#### `public void Print(string text)`

Send text to the text log

**Parameters:**
- `text` (System.String) — [Missing <param name="text"/> documentation for "M:Rhino.FileIO.TextLog.Print(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_TextLog_Print.htm)

#### `public void Print(string format, Object arg0)`

Send formatted text to the text log

**Parameters:**
- `format` (System.String) — [Missing <param name="format"/> documentation for "M:Rhino.FileIO.TextLog.Print(System.String,System.Object)"]
- `arg0` (System.Object) — [Missing <param name="arg0"/> documentation for "M:Rhino.FileIO.TextLog.Print(System.String,System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_TextLog_Print_1.htm)

#### `public void Print(string format, Object arg0, Object arg1)`

Send formatted text to the text log

**Parameters:**
- `format` (System.String) — [Missing <param name="format"/> documentation for "M:Rhino.FileIO.TextLog.Print(System.String,System.Object,System.Object)"]
- `arg0` (System.Object) — [Missing <param name="arg0"/> documentation for "M:Rhino.FileIO.TextLog.Print(System.String,System.Object,System.Object)"]
- `arg1` (System.Object) — [Missing <param name="arg1"/> documentation for "M:Rhino.FileIO.TextLog.Print(System.String,System.Object,System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_TextLog_Print_2.htm)

#### `public void PrintWrappedText(string text, int lineLength)`

Send text wrapped at a set line length

**Parameters:**
- `text` (System.String) — [Missing <param name="text"/> documentation for "M:Rhino.FileIO.TextLog.PrintWrappedText(System.String,System.Int32)"]
- `lineLength` (System.Int32) — [Missing <param name="lineLength"/> documentation for "M:Rhino.FileIO.TextLog.PrintWrappedText(System.String,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_TextLog_PrintWrappedText.htm)

#### `public void PushIndent()`

Increase the indentation level

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_TextLog_PushIndent.htm)

#### `public override string ToString()`

If the TextLog was constructed using the empty constructor, then the text information is stored in a runtime string. The contents of this string is retrieved using ToString for this case

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.FileIO.TextLog.ToString"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/M_Rhino_FileIO_TextLog_ToString.htm)

### Properties
- `IndentSize` (Int32, get/set) — 0: one tab per indent. >0: number of spaces per indent

