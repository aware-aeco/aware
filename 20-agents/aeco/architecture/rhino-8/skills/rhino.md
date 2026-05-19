---
name: rhino-rhino
description: This skill encodes the rhino 8.0 surface of the Rhino namespace — 41 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BitmapExtensions, DisplayDeviceInfo, DocumentEventArgs, DocumentOpenEventArgs, DocumentSaveEventArgs, GpuDeviceInfo, PersistentSettings, LengthValue, and 33 more types.
---

# Rhino

Auto-generated from vendor docs for rhino 8.0. 41 types in this namespace.

## AngleUnitSystem (enum)

ON::AngleUnitSystem identifies an angle unit system

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_AngleUnitSystem.htm)

### Values
- `None` = `0` — ON::AngleUnitSystem::None indicates no angle unit system is specified and model angle unit system should be used.
- `Turns` = `1` — 1 turn = 2pi radians.
- `Radians` = `2` — 1 turn = 2pi radians.
- `Degrees` = `3` — 360 arc degrees = 1 turn = 2pi radians
- `Minutes` = `4` — 60 arc minutes = 1 arc degree
- `Seconds` = `5` — 60 arc seconds = 1 arc minute
- `Gradians` = `6` — 400 gradians = 2pi radians.
- `Unset` = `255` — The ON::AngleUnitSystem::Unset is used to indicates no angle unit system has been specified in user interface code.

## AntialiasLevel (enum)

Provides the anti-alias levels used for render quality

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_AntialiasLevel.htm)

### Values
- `None` = `0` — Low quality
- `Draft` = `1` — Draft quality
- `Good` = `2` — Good quality
- `High` = `3` — High quality

## BitmapExtensions (class)

Rhino specific extension methods for System.Drawing.Bitmap

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_BitmapExtensions.htm)

### Methods
#### `public static FileReference BitmapAsTextureFileReference(this Bitmap bitmap, uint crc)`

Inserts bitmap into Rhino's texture manager and returns a FileReference.

**Parameters:**
- `bitmap` (System.Drawing.Bitmap) — The bitmap which will be referenced by the FileReference.
- `crc` (System.UInt32) — The crc of the bitmap. This should be a unique number which changes if the contents of the bitmap changes. NOTE: if a different bitmap is provided using the same crc as a previous bitmap, then the previous bitmap will be overwritten in the texture manager and both previously returned FileReferences will reference the newly provided bitmap.

**Returns:** `FileReference` — [Missing <returns> documentation for "M:Rhino.BitmapExtensions.BitmapAsTextureFileReference(System.Drawing.Bitmap,System.UInt32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_BitmapExtensions_BitmapAsTextureFileReference.htm)

#### `public static Bitmap ConvertToNormalMap(this Bitmap bitmap, bool bLossyCompressionSource, out bool bPositiveZComponent)`

Use this function to convert a System.Drawing.Bitmap from a bump to a normal texture

**Parameters:**
- `bitmap` (System.Drawing.Bitmap) — [Missing <param name="bitmap"/> documentation for "M:Rhino.BitmapExtensions.ConvertToNormalMap(System.Drawing.Bitmap,System.Boolean,System.Boolean@)"]
- `bLossyCompressionSource` (System.Boolean) — True if the source of the bitmap is an image with lossy compression (e.g. jpg). False otherwise. The check will be less strict if the image can contain errors due to lossy compression.
- `bPositiveZComponent` (System.Boolean) — True if the image is a normal map with the z-component mapped to the range 0 .. +1. False if the image is a normal map with the z-component mapped to the range -1 .. +1.

**Returns:** `Bitmap` — [Missing <returns> documentation for "M:Rhino.BitmapExtensions.ConvertToNormalMap(System.Drawing.Bitmap,System.Boolean,System.Boolean@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_BitmapExtensions_ConvertToNormalMap.htm)

#### `public static bool IsNormalMap(this Bitmap bitmap, bool bLossyCompressionSource, out bool bPositiveZComponent)`

Call this method to see if the DIB appears to be a normal map.

**Parameters:**
- `bitmap` (System.Drawing.Bitmap) — [Missing <param name="bitmap"/> documentation for "M:Rhino.BitmapExtensions.IsNormalMap(System.Drawing.Bitmap,System.Boolean,System.Boolean@)"]
- `bLossyCompressionSource` (System.Boolean) — True if the source of the bitmap is an image with lossy compression (e.g. jpg). False otherwise. The check will be less strict if the image can contain errors due to lossy compression.
- `bPositiveZComponent` (System.Boolean) — True if the image is a normal map with the z-component mapped to the range 0 .. +1. False if the image is a normal map with the z-component mapped to the range -1 .. +1.

**Returns:** `Boolean` — Returns true if the bitmap appears to be a normal map. False otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_BitmapExtensions_IsNormalMap.htm)

## DisplayDeviceInfo (class)

Get information about display devices found on this machine (GPUs).

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_DisplayDeviceInfo.htm)

### Constructors
- `public DisplayDeviceInfo()` — Initializes a new instance of the DisplayDeviceInfo class

### Methods
#### `public static List<GpuDeviceInfo> GpuDeviceInfos()`

Get a list of GpuDeviceInfo for GPUs found on this machine.

**Returns:** `List<GpuDeviceInfo>` — List of GpuDeviceInfo

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DisplayDeviceInfo_GpuDeviceInfos.htm)

#### `public static List<string> GpuNames()`

Get a list with the names of all GPUs on this machine.

**Returns:** `List<String>` — List of strings

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_DisplayDeviceInfo_GpuNames.htm)

## DocumentEventArgs (class)

Provides document information for RhinoDoc events.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_DocumentEventArgs.htm)

### Properties
- `Document` (RhinoDoc, get) — Gets the document for this event. This field might be null.
- `DocumentId` (Int32, get) — Gets the document Id of the document for this event.
- `DocumentSerialNumber` (UInt32, get) — Gets the uniques document serial number for this event

## DocumentOpenEventArgs (class)

Provides document information for RhinoDoc events.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_DocumentOpenEventArgs.htm)

### Properties
- `Document` (RhinoDoc, get) — Gets the document for this event. This field might be null.
- `DocumentId` (Int32, get) — Gets the document Id of the document for this event.
- `DocumentSerialNumber` (UInt32, get) — Gets the uniques document serial number for this event
- `FileName` (String, get) — Name of file being opened.
- `Merge` (Boolean, get) — true if file is being merged into the current document. This occurs during the "Import" command.
- `Reference` (Boolean, get) — true if file is opened as a reference file.

## DocumentSaveEventArgs (class)

Provides document information for RhinoDoc events.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_DocumentSaveEventArgs.htm)

### Properties
- `Document` (RhinoDoc, get) — Gets the document for this event. This field might be null.
- `DocumentId` (Int32, get) — Gets the document Id of the document for this event.
- `DocumentSerialNumber` (UInt32, get) — Gets the uniques document serial number for this event
- `ExportSelected` (Boolean, get) — true if only selected objects are being written to a file.
- `FileName` (String, get) — Name of file being written.

## GpuDeviceInfo (class)

Represents a GPU device providing name, vendor and memory all as strings. Currently fully implemented only on Windows.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_GpuDeviceInfo.htm)

### Properties
- `DriverDateAsString` (String, get) — The driver date as string formatted year-month-day
- `Memory` (UInt64, get) — Memory of the device in bytes
- `MemoryAsString` (String, get) — Memory of the device as a human-readable string
- `Name` (String, get) — Name of the device
- `Vendor` (String, get) — Vendor of the device

## IEpsilonComparable<T> (interface)

[Missing <summary> documentation for "T:Rhino.IEpsilonComparable`1"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_IEpsilonComparable_1.htm)

### Methods
#### `bool EpsilonEquals(T other, double epsilon)`



**Parameters:**
- `other` (T) — [Missing <param name="other"/> documentation for "M:Rhino.IEpsilonComparable`1.EpsilonEquals(`0,System.Double)"]
- `epsilon` (System.Double) — [Missing <param name="epsilon"/> documentation for "M:Rhino.IEpsilonComparable`1.EpsilonEquals(`0,System.Double)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.IEpsilonComparable`1.EpsilonEquals(`0,System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_IEpsilonComparable_1_EpsilonEquals.htm)

## IEpsilonFComparable<T> (interface)

[Missing <summary> documentation for "T:Rhino.IEpsilonFComparable`1"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_IEpsilonFComparable_1.htm)

### Methods
#### `bool EpsilonEquals(T other, float epsilon)`



**Parameters:**
- `other` (T) — [Missing <param name="other"/> documentation for "M:Rhino.IEpsilonFComparable`1.EpsilonEquals(`0,System.Single)"]
- `epsilon` (System.Single) — [Missing <param name="epsilon"/> documentation for "M:Rhino.IEpsilonFComparable`1.EpsilonEquals(`0,System.Single)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.IEpsilonFComparable`1.EpsilonEquals(`0,System.Single)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_IEpsilonFComparable_1_EpsilonEquals.htm)

## IRhinoDocObserver (interface)

Implement this interface if you are a modeless interface to aid in handling multiple document implementations

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_IRhinoDocObserver.htm)

### Methods
#### `void ActiveRhinoDocChanged(RhinoDocObserverArgs e)`

In Windows Rhino this will mean a new document has been created or opened. In Mac Rhino this can mean the same thing as well it can indicate switching from one active open document to another.

**Parameters:**
- `e` (Rhino.RhinoDocObserverArgs) — [Missing <param name="e"/> documentation for "M:Rhino.IRhinoDocObserver.ActiveRhinoDocChanged(Rhino.RhinoDocObserverArgs)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_IRhinoDocObserver_ActiveRhinoDocChanged.htm)

#### `void RhinoDocClosed(RhinoDocObserverArgs e)`

When a document is closed

**Parameters:**
- `e` (Rhino.RhinoDocObserverArgs) — [Missing <param name="e"/> documentation for "M:Rhino.IRhinoDocObserver.RhinoDocClosed(Rhino.RhinoDocObserverArgs)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_IRhinoDocObserver_RhinoDocClosed.htm)

## IndexPair (struct)

Represents two indices: I and J.

**Remarks:** Only 0 and 1 are valid indices.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_IndexPair.htm)

### Constructors
- `public IndexPair(int i, int j)` — Initializes a new instance of IndexPair with two indices.

### Methods
#### `public bool Contains(int item)`

Determines whether the IndexPair contains a specific value.

**Parameters:**
- `item` (System.Int32) — The number to locate in the IndexPair.

**Returns:** `Boolean` — true if item is found in the IndexPair; otherwise, false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_IndexPair_Contains.htm)

#### `public void CopyTo(int[] array, int arrayIndex)`

Copies the elements of the IndexPair to an Array, starting at a particular Array index.

**Parameters:**
- `array` (System.Int32[]) — The one-dimensional Array that is the destination of the elements copied from IndexPair. The Array must have zero-based indexing.
- `arrayIndex` (System.Int32) — The zero-based index in array at which copying begins.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_IndexPair_CopyTo.htm)

#### `public IEnumerator<int> GetEnumerator()`

Gets an enumerator that goes over I and J, in this order.

**Returns:** `IEnumerator<Int32>` — The needed enumerator.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_IndexPair_GetEnumerator.htm)

#### `public int IndexOf(int item)`

Determines the index of a specific item in IndexPair.

**Parameters:**
- `item` (System.Int32) — The object to locate in the IList<T>.

**Returns:** `Int32` — The index, 0 for I or 1 for J of item if found in the list; otherwise, -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_IndexPair_IndexOf.htm)

### Properties
- `Count` (Int32, get) — Returns the amount of elements in this pair of indices, which is always 2.
- `I` (Int32, get/set) — Gets or sets the first, I index.
- `Item` (Int32, get/set) — Gets or sets a particular index of this structure. RemarksOnly 0 and 1 are valid indices.
- `J` (Int32, get/set) — Gets or sets the second, J index.

## LengthValue (class)

Represents a length with an associated unit system and a string representation of that length. This allows for going back and forth from numerical representation of a length and a string representation without "guessing" at the initial string

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_LengthValue.htm)

### Methods
#### `public LengthValue ChangeLength(double newLength)`

Changes the numeric value in a LengthValue and leaves all of the other info unchanged UnitSystem, ParseSettings and StringFormat stay as they were

**Parameters:**
- `newLength` (System.Double) — [Missing <param name="newLength"/> documentation for "M:Rhino.LengthValue.ChangeLength(System.Double)"]

**Returns:** `LengthValue` — A new LengthValue

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_LengthValue_ChangeLength.htm)

#### `public LengthValue ChangeUnitSystem(UnitSystem newUnits)`

Change the UnitSystem of a LengthValue The numeric value of Length is scaled by new_us / current unit system so that the absolute length stays the same

**Parameters:**
- `newUnits` (Rhino.UnitSystem) — [Missing <param name="newUnits"/> documentation for "M:Rhino.LengthValue.ChangeUnitSystem(Rhino.UnitSystem)"]

**Returns:** `LengthValue` — [Missing <returns> documentation for "M:Rhino.LengthValue.ChangeUnitSystem(Rhino.UnitSystem)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_LengthValue_ChangeUnitSystem.htm)

#### `public static LengthValue Create(double length, UnitSystem us, LengthValue.StringFormat format)`

Create from Length and UnitSystem

**Parameters:**
- `length` (System.Double) — Numeric length value
- `us` (Rhino.UnitSystem) — Unit system
- `format` (Rhino.LengthValue.StringFormat) — [Missing <param name="format"/> documentation for "M:Rhino.LengthValue.Create(System.Double,Rhino.UnitSystem,Rhino.LengthValue.StringFormat)"]

**Returns:** `LengthValue` — [Missing <returns> documentation for "M:Rhino.LengthValue.Create(System.Double,Rhino.UnitSystem,Rhino.LengthValue.StringFormat)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_LengthValue_Create.htm)

#### `public static LengthValue Create(double length, UnitSystem us, LengthValue.StringFormat format, uint localeId)`

Create from Length and UnitSystem

**Parameters:**
- `length` (System.Double) — Numeric length value
- `us` (Rhino.UnitSystem) — Unit system
- `format` (Rhino.LengthValue.StringFormat) — [Missing <param name="format"/> documentation for "M:Rhino.LengthValue.Create(System.Double,Rhino.UnitSystem,Rhino.LengthValue.StringFormat,System.UInt32)"]
- `localeId` (System.UInt32) — [Missing <param name="localeId"/> documentation for "M:Rhino.LengthValue.Create(System.Double,Rhino.UnitSystem,Rhino.LengthValue.StringFormat,System.UInt32)"]

**Returns:** `LengthValue` — [Missing <returns> documentation for "M:Rhino.LengthValue.Create(System.Double,Rhino.UnitSystem,Rhino.LengthValue.StringFormat,System.UInt32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_LengthValue_Create_1.htm)

#### `public static LengthValue Create(string s, StringParserSettings ps, out bool parsedAll)`

Create from string

**Parameters:**
- `s` (System.String) — string to parse
- `ps` (Rhino.Input.StringParserSettings) — [Missing <param name="ps"/> documentation for "M:Rhino.LengthValue.Create(System.String,Rhino.Input.StringParserSettings,System.Boolean@)"]
- `parsedAll` (System.Boolean) — true if the whole string was parsed

**Returns:** `LengthValue` — [Missing <returns> documentation for "M:Rhino.LengthValue.Create(System.String,Rhino.Input.StringParserSettings,System.Boolean@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_LengthValue_Create_2.htm)

#### `public void Dispose()`

actively reclaim native allocated ON_LenghtValue*

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_LengthValue_Dispose.htm)

#### `protected override void Finalize()`

passively reclaim native allocated ON_LenghtValue*

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_LengthValue_Finalize.htm)

#### `public bool IsUnset()`

Test IsUnset

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.LengthValue.IsUnset"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_LengthValue_IsUnset.htm)

#### `public double Length()`

Length value in this instance's current unit system

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.LengthValue.Length"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_LengthValue_Length.htm)

#### `public double Length(UnitSystem units)`

Length value in a given unit system

**Parameters:**
- `units` (Rhino.UnitSystem) — [Missing <param name="units"/> documentation for "M:Rhino.LengthValue.Length(Rhino.UnitSystem)"]

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.LengthValue.Length(Rhino.UnitSystem)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_LengthValue_Length_1.htm)

### Properties
- `ContextAngleUnitSystem` (AngleUnitSystem, get) — Returns the context AngleUnitSystem from this LengthValue's ParseSettings
- `ContextLocaleId` (UInt32, get) — returns the context LocaleId from this LengthValue
- `LengthString` (String, get) — Return length as a string
- `LengthStringFormat` (LengthValue.StringFormat, get) — Returns the StringFormat from this LengthValue
- `ParseSettings` (StringParserSettings, get) — Parse settings
- `UnitSystem` (UnitSystem, get) — UnitSystem used by this LengthValue

## LengthValue.StringFormat (enum)

Formatting to apply when creating a length value from a double.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_LengthValue_StringFormat.htm)

### Values
- `ExactDecimal` = `0` — Use exact decimal string.
- `ExactProperFraction` = `1` — If possible, use exact integer-fraction format (1.125 becomes 1-1/8).
- `ExactImproperFraction` = `2` — If possible, use exact fraction format (1.125 becomes 9/8).
- `CleanDecimal` = `3` — The value may be adjusted slightly to improve clarity (1.124999... becomes 1.125).
- `CleanProperFraction` = `4` — The value may be adjusted slightly to improve clarity (1.124999... becomes 1-1/8).
- `CleanImproperFraction` = `5` — The value may be adjusted slightly to improve clarity (1.124999... becomes 9/8).

## LinkedInstanceDefinitionUpdateStyle (enum)

Linked and linked and embedded instance definition update style.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_LinkedInstanceDefinitionUpdateStyle.htm)

### Values
- `Prompt` = `1` — Ask the user if the linked and linked and embedded instance definitions should be updated.
- `AlwaysUpdate` = `2` — Always update linked and linked and embedded instance definitions.
- `NeverUpdate` = `3` — Never update linked and linked and embedded instance definitions.

## PersistentSettings (class)

A dictionary of SettingValue items.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PersistentSettings.htm)

### Constructors
- `protected PersistentSettings(SerializationInfo info, StreamingContext context)` — Initializes a new instance of the PersistentSettings class

### Methods
#### `public PersistentSettings AddChild(string key)`

Call this method to add a new child key, if the key is exists then the existing key is returned otherwise a new empty PersistentSettings child key is added and the new settings are returned.

**Parameters:**
- `key` (System.String) — Key to add to the child dictionary.

**Returns:** `PersistentSettings` — If the key is exists then the existing key is returned otherwise a new empty PersistentSettings child key is added and the new settings are returned.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_AddChild.htm)

#### `public void ClearChangedFlag()`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_ClearChangedFlag.htm)

#### `public bool ContainsChangedValues()`



**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.ContainsChangedValues"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_ContainsChangedValues.htm)

#### `public bool ContainsModifiedValues(PersistentSettings allUserSettings)`



**Parameters:**
- `allUserSettings` (Rhino.PersistentSettings) — [Missing <param name="allUserSettings"/> documentation for "M:Rhino.PersistentSettings.ContainsModifiedValues(Rhino.PersistentSettings)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.ContainsModifiedValues(Rhino.PersistentSettings)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_ContainsModifiedValues.htm)

#### `public void DeleteChild(string key)`

Call this method to delete a child settings key.

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.DeleteChild(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_DeleteChild.htm)

#### `public void DeleteItem(string key)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.DeleteItem(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_DeleteItem.htm)

#### `public static PersistentSettings FromPlugInId(Guid pluginId)`



**Parameters:**
- `pluginId` (System.Guid) — [Missing <param name="pluginId"/> documentation for "M:Rhino.PersistentSettings.FromPlugInId(System.Guid)"]

**Returns:** `PersistentSettings` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.FromPlugInId(System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_FromPlugInId.htm)

#### `public bool GetBool(string key)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetBool(System.String)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetBool(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetBool.htm)

#### `public bool GetBool(string key, bool defaultValue)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetBool(System.String,System.Boolean)"]
- `defaultValue` (System.Boolean) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetBool(System.String,System.Boolean)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetBool(System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetBool_1.htm)

#### `public bool GetBool(string key, bool defaultValue, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetBool(System.String,System.Boolean,System.Collections.Generic.IEnumerable{System.String})"]
- `defaultValue` (System.Boolean) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetBool(System.String,System.Boolean,System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.GetBool(System.String,System.Boolean,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetBool(System.String,System.Boolean,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetBool_2.htm)

#### `public byte GetByte(string key)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetByte(System.String)"]

**Returns:** `Byte` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetByte(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetByte.htm)

#### `public byte GetByte(string key, byte defaultValue)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetByte(System.String,System.Byte)"]
- `defaultValue` (System.Byte) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetByte(System.String,System.Byte)"]

**Returns:** `Byte` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetByte(System.String,System.Byte)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetByte_1.htm)

#### `public byte GetByte(string key, byte defaultValue, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetByte(System.String,System.Byte,System.Collections.Generic.IEnumerable{System.String})"]
- `defaultValue` (System.Byte) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetByte(System.String,System.Byte,System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.GetByte(System.String,System.Byte,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `Byte` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetByte(System.String,System.Byte,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetByte_2.htm)

#### `public char GetChar(string key)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetChar(System.String)"]

**Returns:** `Char` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetChar(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetChar.htm)

#### `public char GetChar(string key, char defaultValue)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetChar(System.String,System.Char)"]
- `defaultValue` (System.Char) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetChar(System.String,System.Char)"]

**Returns:** `Char` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetChar(System.String,System.Char)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetChar_1.htm)

#### `public char GetChar(string key, char defaultValue, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetChar(System.String,System.Char,System.Collections.Generic.IEnumerable{System.String})"]
- `defaultValue` (System.Char) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetChar(System.String,System.Char,System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.GetChar(System.String,System.Char,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `Char` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetChar(System.String,System.Char,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetChar_2.htm)

#### `public PersistentSettings GetChild(string key)`

Call this method to get a nested settings PersistentSettings instance, will throw a KeyNotFoundException exception if the key does not exist.

**Parameters:**
- `key` (System.String) — Key name

**Returns:** `PersistentSettings` — Returns persistent settings for the specified key or throws an exception if the key is invalid.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetChild.htm)

#### `public Color GetColor(string key)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetColor(System.String)"]

**Returns:** `Color` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetColor(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetColor.htm)

#### `public Color GetColor(string key, Color defaultValue)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetColor(System.String,System.Drawing.Color)"]
- `defaultValue` (System.Drawing.Color) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetColor(System.String,System.Drawing.Color)"]

**Returns:** `Color` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetColor(System.String,System.Drawing.Color)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetColor_1.htm)

#### `public Color GetColor(string key, Color defaultValue, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetColor(System.String,System.Drawing.Color,System.Collections.Generic.IEnumerable{System.String})"]
- `defaultValue` (System.Drawing.Color) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetColor(System.String,System.Drawing.Color,System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.GetColor(System.String,System.Drawing.Color,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `Color` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetColor(System.String,System.Drawing.Color,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetColor_2.htm)

#### `public Color? GetColor(string key, Color? defaultValue)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetColor(System.String,System.Nullable{System.Drawing.Color})"]
- `defaultValue` (System.Nullable<Color>) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetColor(System.String,System.Nullable{System.Drawing.Color})"]

**Returns:** `Nullable<Color>` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetColor(System.String,System.Nullable{System.Drawing.Color})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetColor_3.htm)

#### `public Color? GetColor(string key, Color? defaultValue, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetColor(System.String,System.Nullable{System.Drawing.Color},System.Collections.Generic.IEnumerable{System.String})"]
- `defaultValue` (System.Nullable<Color>) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetColor(System.String,System.Nullable{System.Drawing.Color},System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.GetColor(System.String,System.Nullable{System.Drawing.Color},System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `Nullable<Color>` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetColor(System.String,System.Nullable{System.Drawing.Color},System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetColor_4.htm)

#### `public DateTime GetDate(string key)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetDate(System.String)"]

**Returns:** `DateTime` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetDate(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetDate.htm)

#### `public DateTime GetDate(string key, DateTime defaultValue)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetDate(System.String,System.DateTime)"]
- `defaultValue` (System.DateTime) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetDate(System.String,System.DateTime)"]

**Returns:** `DateTime` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetDate(System.String,System.DateTime)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetDate_1.htm)

#### `public DateTime GetDate(string key, DateTime defaultValue, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetDate(System.String,System.DateTime,System.Collections.Generic.IEnumerable{System.String})"]
- `defaultValue` (System.DateTime) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetDate(System.String,System.DateTime,System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.GetDate(System.String,System.DateTime,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `DateTime` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetDate(System.String,System.DateTime,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetDate_2.htm)

#### `public double GetDouble(string key)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetDouble(System.String)"]

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetDouble(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetDouble.htm)

#### `public double GetDouble(string key, double defaultValue)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetDouble(System.String,System.Double)"]
- `defaultValue` (System.Double) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetDouble(System.String,System.Double)"]

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetDouble(System.String,System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetDouble_1.htm)

#### `public double GetDouble(string key, double defaultValue, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetDouble(System.String,System.Double,System.Collections.Generic.IEnumerable{System.String})"]
- `defaultValue` (System.Double) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetDouble(System.String,System.Double,System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.GetDouble(System.String,System.Double,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetDouble(System.String,System.Double,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetDouble_2.htm)

#### `public T GetEnumValue<T>(string key) where T : struct, new(), IConvertible`

Get a stored enumerated value using a custom key.

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetEnumValue``1(System.String)"]

**Returns:** `T` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetEnumValue``1(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetEnumValue__1.htm)

#### `public T GetEnumValue<T>(string key, T defaultValue) where T : struct, new(), IConvertible`

Gets a stored enumerated value using a custom key, or return default value if not found.

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetEnumValue``1(System.String,``0)"]
- `defaultValue` (T) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetEnumValue``1(System.String,``0)"]

**Returns:** `T` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetEnumValue``1(System.String,``0)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetEnumValue__1_1.htm)

#### `public T GetEnumValue<T>(T defaultValue) where T : struct, new(), IConvertible`

Get a stored enumerated value, or return default value if not found

**Parameters:**
- `defaultValue` (T) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetEnumValue``1(``0)"]

**Returns:** `T` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetEnumValue``1(``0)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetEnumValue__1_2.htm)

#### `public Guid GetGuid(string key)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetGuid(System.String)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetGuid(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetGuid.htm)

#### `public Guid GetGuid(string key, Guid defaultValue)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetGuid(System.String,System.Guid)"]
- `defaultValue` (System.Guid) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetGuid(System.String,System.Guid)"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetGuid(System.String,System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetGuid_1.htm)

#### `public Guid GetGuid(string key, Guid defaultValue, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetGuid(System.String,System.Guid,System.Collections.Generic.IEnumerable{System.String})"]
- `defaultValue` (System.Guid) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetGuid(System.String,System.Guid,System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.GetGuid(System.String,System.Guid,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `Guid` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetGuid(System.String,System.Guid,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetGuid_2.htm)

#### `public int GetInteger(string key)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetInteger(System.String)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetInteger(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetInteger.htm)

#### `public int GetInteger(string key, int defaultValue)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetInteger(System.String,System.Int32)"]
- `defaultValue` (System.Int32) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetInteger(System.String,System.Int32)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetInteger(System.String,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetInteger_1.htm)

#### `public int GetInteger(string key, int defaultValue, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetInteger(System.String,System.Int32,System.Collections.Generic.IEnumerable{System.String})"]
- `defaultValue` (System.Int32) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetInteger(System.String,System.Int32,System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.GetInteger(System.String,System.Int32,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetInteger(System.String,System.Int32,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetInteger_2.htm)

#### `public int GetInteger(string key, int defaultValue, int bound, bool boundIsLower)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetInteger(System.String,System.Int32,System.Int32,System.Boolean)"]
- `defaultValue` (System.Int32) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetInteger(System.String,System.Int32,System.Int32,System.Boolean)"]
- `bound` (System.Int32) — [Missing <param name="bound"/> documentation for "M:Rhino.PersistentSettings.GetInteger(System.String,System.Int32,System.Int32,System.Boolean)"]
- `boundIsLower` (System.Boolean) — [Missing <param name="boundIsLower"/> documentation for "M:Rhino.PersistentSettings.GetInteger(System.String,System.Int32,System.Int32,System.Boolean)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetInteger(System.String,System.Int32,System.Int32,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetInteger_3.htm)

#### `public int GetInteger(string key, int defaultValue, int lowerBound, int upperBound)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetInteger(System.String,System.Int32,System.Int32,System.Int32)"]
- `defaultValue` (System.Int32) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetInteger(System.String,System.Int32,System.Int32,System.Int32)"]
- `lowerBound` (System.Int32) — [Missing <param name="lowerBound"/> documentation for "M:Rhino.PersistentSettings.GetInteger(System.String,System.Int32,System.Int32,System.Int32)"]
- `upperBound` (System.Int32) — [Missing <param name="upperBound"/> documentation for "M:Rhino.PersistentSettings.GetInteger(System.String,System.Int32,System.Int32,System.Int32)"]

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetInteger(System.String,System.Int32,System.Int32,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetInteger_4.htm)

#### `public Point GetPoint(string key)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetPoint(System.String)"]

**Returns:** `Point` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetPoint(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetPoint.htm)

#### `public Point GetPoint(string key, Point defaultValue)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetPoint(System.String,System.Drawing.Point)"]
- `defaultValue` (System.Drawing.Point) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetPoint(System.String,System.Drawing.Point)"]

**Returns:** `Point` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetPoint(System.String,System.Drawing.Point)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetPoint_1.htm)

#### `public Point GetPoint(string key, Point defaultValue, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetPoint(System.String,System.Drawing.Point,System.Collections.Generic.IEnumerable{System.String})"]
- `defaultValue` (System.Drawing.Point) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetPoint(System.String,System.Drawing.Point,System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.GetPoint(System.String,System.Drawing.Point,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `Point` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetPoint(System.String,System.Drawing.Point,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetPoint_2.htm)

#### `public Point3d GetPoint3d(string key)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetPoint3d(System.String)"]

**Returns:** `Point3d` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetPoint3d(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetPoint3d.htm)

#### `public Point3d GetPoint3d(string key, Point3d defaultValue)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetPoint3d(System.String,Rhino.Geometry.Point3d)"]
- `defaultValue` (Rhino.Geometry.Point3d) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetPoint3d(System.String,Rhino.Geometry.Point3d)"]

**Returns:** `Point3d` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetPoint3d(System.String,Rhino.Geometry.Point3d)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetPoint3d_1.htm)

#### `public Point3d GetPoint3d(string key, Point3d defaultValue, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetPoint3d(System.String,Rhino.Geometry.Point3d,System.Collections.Generic.IEnumerable{System.String})"]
- `defaultValue` (Rhino.Geometry.Point3d) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetPoint3d(System.String,Rhino.Geometry.Point3d,System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.GetPoint3d(System.String,Rhino.Geometry.Point3d,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `Point3d` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetPoint3d(System.String,Rhino.Geometry.Point3d,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetPoint3d_2.htm)

#### `public Rectangle GetRectangle(string key)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetRectangle(System.String)"]

**Returns:** `Rectangle` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetRectangle(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetRectangle.htm)

#### `public Rectangle GetRectangle(string key, Rectangle defaultValue)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetRectangle(System.String,System.Drawing.Rectangle)"]
- `defaultValue` (System.Drawing.Rectangle) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetRectangle(System.String,System.Drawing.Rectangle)"]

**Returns:** `Rectangle` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetRectangle(System.String,System.Drawing.Rectangle)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetRectangle_1.htm)

#### `public Rectangle GetRectangle(string key, Rectangle defaultValue, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetRectangle(System.String,System.Drawing.Rectangle,System.Collections.Generic.IEnumerable{System.String})"]
- `defaultValue` (System.Drawing.Rectangle) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetRectangle(System.String,System.Drawing.Rectangle,System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.GetRectangle(System.String,System.Drawing.Rectangle,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `Rectangle` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetRectangle(System.String,System.Drawing.Rectangle,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetRectangle_2.htm)

#### `public bool GetSettingIsHiddenFromUserInterface(string key)`

Values read from all users settings files will be marked as read-only which will cause any future calls to Set... to fail.

**Parameters:**
- `key` (System.String) — Key name for which to search.

**Returns:** `Boolean` — Returns true if the setting is read-only otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetSettingIsHiddenFromUserInterface.htm)

#### `public bool GetSettingIsHiddenFromUserInterface(string key, IEnumerable<string> legacyKeyList)`

Values read from all users settings files will be marked as read-only which will cause any future calls to Set... to fail.

**Parameters:**
- `key` (System.String) — Key name for which to search.
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.GetSettingIsHiddenFromUserInterface(System.String,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `Boolean` — Returns true if the setting is read-only otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetSettingIsHiddenFromUserInterface_1.htm)

#### `public bool GetSettingIsReadOnly(string key)`

Values read from all users settings files will be marked as read-only which will cause any future calls to Set... to fail.

**Parameters:**
- `key` (System.String) — Key name for which to search.

**Returns:** `Boolean` — Returns true if the setting is read-only otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetSettingIsReadOnly.htm)

#### `public Type GetSettingType(string key)`

Gets the type of the last value passed to Set... or Get... for the specified setting.

**Parameters:**
- `key` (System.String) — Key name for which to search.

**Returns:** `Type` — Type of the last value passed to Set... or Get... for the specified setting.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetSettingType.htm)

#### `public Size GetSize(string key)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetSize(System.String)"]

**Returns:** `Size` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetSize(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetSize.htm)

#### `public Size GetSize(string key, Size defaultValue)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetSize(System.String,System.Drawing.Size)"]
- `defaultValue` (System.Drawing.Size) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetSize(System.String,System.Drawing.Size)"]

**Returns:** `Size` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetSize(System.String,System.Drawing.Size)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetSize_1.htm)

#### `public Size GetSize(string key, Size defaultValue, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetSize(System.String,System.Drawing.Size,System.Collections.Generic.IEnumerable{System.String})"]
- `defaultValue` (System.Drawing.Size) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetSize(System.String,System.Drawing.Size,System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.GetSize(System.String,System.Drawing.Size,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `Size` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetSize(System.String,System.Drawing.Size,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetSize_2.htm)

#### `public string GetString(string key)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetString(System.String)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetString(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetString.htm)

#### `public string GetString(string key, string defaultValue)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetString(System.String,System.String)"]
- `defaultValue` (System.String) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetString(System.String,System.String)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetString(System.String,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetString_1.htm)

#### `public string GetString(string key, string defaultValue, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetString(System.String,System.String,System.Collections.Generic.IEnumerable{System.String})"]
- `defaultValue` (System.String) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetString(System.String,System.String,System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.GetString(System.String,System.String,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetString(System.String,System.String,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetString_2.htm)

#### `public KeyValuePair<string, string>[] GetStringDictionary(string key)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetStringDictionary(System.String)"]

**Returns:** `KeyValuePair<String,String>[]` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetStringDictionary(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetStringDictionary.htm)

#### `public KeyValuePair<string, string>[] GetStringDictionary(string key, KeyValuePair<string, string>[] defaultValue)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetStringDictionary(System.String,System.Collections.Generic.KeyValuePair{System.String,System.String}[])"]
- `defaultValue` (System.Collections.Generic.KeyValuePair<String,String>[]) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetStringDictionary(System.String,System.Collections.Generic.KeyValuePair{System.String,System.String}[])"]

**Returns:** `KeyValuePair<String,String>[]` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetStringDictionary(System.String,System.Collections.Generic.KeyValuePair{System.String,System.String}[])"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetStringDictionary_1.htm)

#### `public KeyValuePair<string, string>[] GetStringDictionary(string key, KeyValuePair<string, string>[] defaultValue, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetStringDictionary(System.String,System.Collections.Generic.KeyValuePair{System.String,System.String}[],System.Collections.Generic.IEnumerable{System.String})"]
- `defaultValue` (System.Collections.Generic.KeyValuePair<String,String>[]) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetStringDictionary(System.String,System.Collections.Generic.KeyValuePair{System.String,System.String}[],System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.GetStringDictionary(System.String,System.Collections.Generic.KeyValuePair{System.String,System.String}[],System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `KeyValuePair<String,String>[]` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetStringDictionary(System.String,System.Collections.Generic.KeyValuePair{System.String,System.String}[],System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetStringDictionary_2.htm)

#### `public string[] GetStringList(string key)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetStringList(System.String)"]

**Returns:** `String[]` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetStringList(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetStringList.htm)

#### `public string[] GetStringList(string key, string[] defaultValue)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetStringList(System.String,System.String[])"]
- `defaultValue` (System.String[]) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetStringList(System.String,System.String[])"]

**Returns:** `String[]` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetStringList(System.String,System.String[])"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetStringList_1.htm)

#### `public string[] GetStringList(string key, string[] defaultValue, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetStringList(System.String,System.String[],System.Collections.Generic.IEnumerable{System.String})"]
- `defaultValue` (System.String[]) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetStringList(System.String,System.String[],System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.GetStringList(System.String,System.String[],System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `String[]` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetStringList(System.String,System.String[],System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetStringList_2.htm)

#### `public uint GetUnsignedInteger(string key)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetUnsignedInteger(System.String)"]

**Returns:** `UInt32` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetUnsignedInteger(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetUnsignedInteger.htm)

#### `public uint GetUnsignedInteger(string key, uint defaultValue)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetUnsignedInteger(System.String,System.UInt32)"]
- `defaultValue` (System.UInt32) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetUnsignedInteger(System.String,System.UInt32)"]

**Returns:** `UInt32` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetUnsignedInteger(System.String,System.UInt32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetUnsignedInteger_1.htm)

#### `public uint GetUnsignedInteger(string key, uint defaultValue, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.GetUnsignedInteger(System.String,System.UInt32,System.Collections.Generic.IEnumerable{System.String})"]
- `defaultValue` (System.UInt32) — [Missing <param name="defaultValue"/> documentation for "M:Rhino.PersistentSettings.GetUnsignedInteger(System.String,System.UInt32,System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.GetUnsignedInteger(System.String,System.UInt32,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `UInt32` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.GetUnsignedInteger(System.String,System.UInt32,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetUnsignedInteger_2.htm)

#### `public EventHandler<PersistentSettingsEventArgs<T>> GetValidator<T>(string key)`

Provides a way to find a ready-to-use validator for the PersistentSetting class for the given the key, or obtaining null.

**Parameters:**
- `key` (System.String) — The name of the setting key.

**Returns:** `EventHandler<PersistentSettingsEventArgs<T>>` — A valid validator, or null if no validator was found.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_GetValidator__1.htm)

#### `public void HideSettingFromUserInterface(string key)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.HideSettingFromUserInterface(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_HideSettingFromUserInterface.htm)

#### `public void RegisterSettingsValidator<T>(string key, EventHandler<PersistentSettingsEventArgs<T>> validator)`

Sets a validator for a given key. Note to implementers: T should be one of the supported types for the PersistentSettings class and should match the type associated with the key.This method allows to use anonymous methods and lambda expressions.

**Parameters:**
- `key` (System.String) — The key to which to bind the validator.
- `validator` (System.EventHandler<PersistentSettingsEventArgs<T>>) — A validator instance of your own class.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_RegisterSettingsValidator__1.htm)

#### `public void SetBool(string key, bool value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetBool(System.String,System.Boolean)"]
- `value` (System.Boolean) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetBool(System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetBool.htm)

#### `public void SetByte(string key, byte value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetByte(System.String,System.Byte)"]
- `value` (System.Byte) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetByte(System.String,System.Byte)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetByte.htm)

#### `public void SetChar(string key, char value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetChar(System.String,System.Char)"]
- `value` (System.Char) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetChar(System.String,System.Char)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetChar.htm)

#### `public void SetColor(string key, Color value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetColor(System.String,System.Drawing.Color)"]
- `value` (System.Drawing.Color) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetColor(System.String,System.Drawing.Color)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetColor.htm)

#### `public void SetColor(string key, Color? value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetColor(System.String,System.Nullable{System.Drawing.Color})"]
- `value` (System.Nullable<Color>) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetColor(System.String,System.Nullable{System.Drawing.Color})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetColor_1.htm)

#### `public void SetDate(string key, DateTime value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetDate(System.String,System.DateTime)"]
- `value` (System.DateTime) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetDate(System.String,System.DateTime)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetDate.htm)

#### `public void SetDefault(string key, bool value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetDefault(System.String,System.Boolean)"]
- `value` (System.Boolean) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetDefault(System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetDefault_1.htm)

#### `public void SetDefault(string key, byte value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetDefault(System.String,System.Byte)"]
- `value` (System.Byte) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetDefault(System.String,System.Byte)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetDefault_2.htm)

#### `public void SetDefault(string key, char value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetDefault(System.String,System.Char)"]
- `value` (System.Char) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetDefault(System.String,System.Char)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetDefault_3.htm)

#### `public void SetDefault(string key, Color value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetDefault(System.String,System.Drawing.Color)"]
- `value` (System.Drawing.Color) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetDefault(System.String,System.Drawing.Color)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetDefault_7.htm)

#### `public void SetDefault(string key, DateTime value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetDefault(System.String,System.DateTime)"]
- `value` (System.DateTime) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetDefault(System.String,System.DateTime)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetDefault_5.htm)

#### `public void SetDefault(string key, double value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetDefault(System.String,System.Double)"]
- `value` (System.Double) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetDefault(System.String,System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetDefault_6.htm)

#### `public void SetDefault(string key, Guid value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetDefault(System.String,System.Guid)"]
- `value` (System.Guid) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetDefault(System.String,System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetDefault_11.htm)

#### `public void SetDefault(string key, int value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetDefault(System.String,System.Int32)"]
- `value` (System.Int32) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetDefault(System.String,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetDefault_12.htm)

#### `public void SetDefault(string key, KeyValuePair<string, string>[] value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetDefault(System.String,System.Collections.Generic.KeyValuePair{System.String,System.String}[])"]
- `value` (System.Collections.Generic.KeyValuePair<String,String>[]) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetDefault(System.String,System.Collections.Generic.KeyValuePair{System.String,System.String}[])"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetDefault_4.htm)

#### `public void SetDefault(string key, Color? value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetDefault(System.String,System.Nullable{System.Drawing.Color})"]
- `value` (System.Nullable<Color>) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetDefault(System.String,System.Nullable{System.Drawing.Color})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetDefault_13.htm)

#### `public void SetDefault(string key, Point value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetDefault(System.String,System.Drawing.Point)"]
- `value` (System.Drawing.Point) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetDefault(System.String,System.Drawing.Point)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetDefault_8.htm)

#### `public void SetDefault(string key, Point3d value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetDefault(System.String,Rhino.Geometry.Point3d)"]
- `value` (Rhino.Geometry.Point3d) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetDefault(System.String,Rhino.Geometry.Point3d)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetDefault.htm)

#### `public void SetDefault(string key, Rectangle value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetDefault(System.String,System.Drawing.Rectangle)"]
- `value` (System.Drawing.Rectangle) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetDefault(System.String,System.Drawing.Rectangle)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetDefault_9.htm)

#### `public void SetDefault(string key, Size value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetDefault(System.String,System.Drawing.Size)"]
- `value` (System.Drawing.Size) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetDefault(System.String,System.Drawing.Size)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetDefault_10.htm)

#### `public void SetDefault(string key, string value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetDefault(System.String,System.String)"]
- `value` (System.String) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetDefault(System.String,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetDefault_14.htm)

#### `public void SetDefault(string key, string[] value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetDefault(System.String,System.String[])"]
- `value` (System.String[]) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetDefault(System.String,System.String[])"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetDefault_15.htm)

#### `public void SetDouble(string key, double value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetDouble(System.String,System.Double)"]
- `value` (System.Double) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetDouble(System.String,System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetDouble.htm)

#### `public void SetEnumValue<T>(string key, T value) where T : struct, new(), IConvertible`

Set an enumerated value in the settings using a custom key

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetEnumValue``1(System.String,``0)"]
- `value` (T) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetEnumValue``1(System.String,``0)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetEnumValue__1.htm)

#### `public void SetEnumValue<T>(T enumValue) where T : struct, new(), IConvertible`

Set an enumerated value in the settings.

**Parameters:**
- `enumValue` (T) — [Missing <param name="enumValue"/> documentation for "M:Rhino.PersistentSettings.SetEnumValue``1(``0)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetEnumValue__1_1.htm)

#### `public void SetGuid(string key, Guid value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetGuid(System.String,System.Guid)"]
- `value` (System.Guid) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetGuid(System.String,System.Guid)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetGuid.htm)

#### `public void SetInteger(string key, int value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetInteger(System.String,System.Int32)"]
- `value` (System.Int32) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetInteger(System.String,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetInteger.htm)

#### `public void SetPoint(string key, Point value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetPoint(System.String,System.Drawing.Point)"]
- `value` (System.Drawing.Point) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetPoint(System.String,System.Drawing.Point)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetPoint.htm)

#### `public void SetPoint3d(string key, Point3d value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetPoint3d(System.String,Rhino.Geometry.Point3d)"]
- `value` (Rhino.Geometry.Point3d) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetPoint3d(System.String,Rhino.Geometry.Point3d)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetPoint3d.htm)

#### `public void SetRectangle(string key, Rectangle value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetRectangle(System.String,System.Drawing.Rectangle)"]
- `value` (System.Drawing.Rectangle) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetRectangle(System.String,System.Drawing.Rectangle)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetRectangle.htm)

#### `public void SetSize(string key, Size value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetSize(System.String,System.Drawing.Size)"]
- `value` (System.Drawing.Size) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetSize(System.String,System.Drawing.Size)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetSize.htm)

#### `public void SetString(string key, string value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetString(System.String,System.String)"]
- `value` (System.String) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetString(System.String,System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetString.htm)

#### `public void SetStringDictionary(string key, KeyValuePair<string, string>[] value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetStringDictionary(System.String,System.Collections.Generic.KeyValuePair{System.String,System.String}[])"]
- `value` (System.Collections.Generic.KeyValuePair<String,String>[]) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetStringDictionary(System.String,System.Collections.Generic.KeyValuePair{System.String,System.String}[])"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetStringDictionary.htm)

#### `public void SetStringList(string key, string[] value)`

Including a item with the value of StringListRootKey will cause the ProgramData value to get inserted at that location in the list when calling GetStringList.

**Parameters:**
- `key` (System.String) — The string key.
- `value` (System.String[]) — An array of values to set.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetStringList.htm)

#### `public void SetUnsignedInteger(string key, uint value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.SetUnsignedInteger(System.String,System.UInt32)"]
- `value` (System.UInt32) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.SetUnsignedInteger(System.String,System.UInt32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_SetUnsignedInteger.htm)

#### `public bool TryGetBool(string key, out bool value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetBool(System.String,System.Boolean@)"]
- `value` (System.Boolean) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetBool(System.String,System.Boolean@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetBool(System.String,System.Boolean@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetBool.htm)

#### `public bool TryGetBool(string key, out bool value, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetBool(System.String,System.Boolean@,System.Collections.Generic.IEnumerable{System.String})"]
- `value` (System.Boolean) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetBool(System.String,System.Boolean@,System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.TryGetBool(System.String,System.Boolean@,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetBool(System.String,System.Boolean@,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetBool_1.htm)

#### `public bool TryGetByte(string key, out byte value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetByte(System.String,System.Byte@)"]
- `value` (System.Byte) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetByte(System.String,System.Byte@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetByte(System.String,System.Byte@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetByte.htm)

#### `public bool TryGetByte(string key, out byte value, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetByte(System.String,System.Byte@,System.Collections.Generic.IEnumerable{System.String})"]
- `value` (System.Byte) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetByte(System.String,System.Byte@,System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.TryGetByte(System.String,System.Byte@,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetByte(System.String,System.Byte@,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetByte_1.htm)

#### `public bool TryGetChar(string key, out char value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetChar(System.String,System.Char@)"]
- `value` (System.Char) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetChar(System.String,System.Char@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetChar(System.String,System.Char@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetChar.htm)

#### `public bool TryGetChar(string key, out char value, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetChar(System.String,System.Char@,System.Collections.Generic.IEnumerable{System.String})"]
- `value` (System.Char) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetChar(System.String,System.Char@,System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.TryGetChar(System.String,System.Char@,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetChar(System.String,System.Char@,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetChar_1.htm)

#### `public bool TryGetChild(string key, out PersistentSettings value)`

Call this method to get a nested settings PersistentSettings instance, will return true if the key exists and value was set otherwise; will return false and value will be set to null.

**Parameters:**
- `key` (System.String) — [in] Key name
- `value` (Rhino.PersistentSettings) — [out] Will be set the child settings if the key is valid otherwise it will be null.

**Returns:** `Boolean` — Returns true if the key exists and value was set otherwise; returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetChild.htm)

#### `public bool TryGetColor(string key, out Color value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetColor(System.String,System.Drawing.Color@)"]
- `value` (System.Drawing.Color) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetColor(System.String,System.Drawing.Color@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetColor(System.String,System.Drawing.Color@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetColor.htm)

#### `public bool TryGetColor(string key, out Color value, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetColor(System.String,System.Drawing.Color@,System.Collections.Generic.IEnumerable{System.String})"]
- `value` (System.Drawing.Color) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetColor(System.String,System.Drawing.Color@,System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.TryGetColor(System.String,System.Drawing.Color@,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetColor(System.String,System.Drawing.Color@,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetColor_1.htm)

#### `public bool TryGetColor(string key, out Color? value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetColor(System.String,System.Nullable{System.Drawing.Color}@)"]
- `value` (System.Nullable<Color>) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetColor(System.String,System.Nullable{System.Drawing.Color}@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetColor(System.String,System.Nullable{System.Drawing.Color}@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetColor_2.htm)

#### `public bool TryGetColor(string key, out Color? value, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetColor(System.String,System.Nullable{System.Drawing.Color}@,System.Collections.Generic.IEnumerable{System.String})"]
- `value` (System.Nullable<Color>) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetColor(System.String,System.Nullable{System.Drawing.Color}@,System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.TryGetColor(System.String,System.Nullable{System.Drawing.Color}@,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetColor(System.String,System.Nullable{System.Drawing.Color}@,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetColor_3.htm)

#### `public bool TryGetDate(string key, out DateTime value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetDate(System.String,System.DateTime@)"]
- `value` (System.DateTime) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetDate(System.String,System.DateTime@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetDate(System.String,System.DateTime@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetDate.htm)

#### `public bool TryGetDate(string key, out DateTime value, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetDate(System.String,System.DateTime@,System.Collections.Generic.IEnumerable{System.String})"]
- `value` (System.DateTime) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetDate(System.String,System.DateTime@,System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.TryGetDate(System.String,System.DateTime@,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetDate(System.String,System.DateTime@,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetDate_1.htm)

#### `public bool TryGetDefault(string key, out bool value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.Boolean@)"]
- `value` (System.Boolean) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.Boolean@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.Boolean@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetDefault_1.htm)

#### `public bool TryGetDefault(string key, out byte value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.Byte@)"]
- `value` (System.Byte) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.Byte@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.Byte@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetDefault_2.htm)

#### `public bool TryGetDefault(string key, out char value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.Char@)"]
- `value` (System.Char) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.Char@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.Char@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetDefault_3.htm)

#### `public bool TryGetDefault(string key, out Color value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.Drawing.Color@)"]
- `value` (System.Drawing.Color) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.Drawing.Color@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.Drawing.Color@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetDefault_6.htm)

#### `public bool TryGetDefault(string key, out DateTime value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.DateTime@)"]
- `value` (System.DateTime) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.DateTime@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.DateTime@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetDefault_4.htm)

#### `public bool TryGetDefault(string key, out double value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.Double@)"]
- `value` (System.Double) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.Double@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.Double@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetDefault_5.htm)

#### `public bool TryGetDefault(string key, out int value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.Int32@)"]
- `value` (System.Int32) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.Int32@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.Int32@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetDefault_9.htm)

#### `public bool TryGetDefault(string key, out Point3d value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,Rhino.Geometry.Point3d@)"]
- `value` (Rhino.Geometry.Point3d) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,Rhino.Geometry.Point3d@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,Rhino.Geometry.Point3d@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetDefault.htm)

#### `public bool TryGetDefault(string key, out Rectangle value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.Drawing.Rectangle@)"]
- `value` (System.Drawing.Rectangle) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.Drawing.Rectangle@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.Drawing.Rectangle@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetDefault_7.htm)

#### `public bool TryGetDefault(string key, out Size value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.Drawing.Size@)"]
- `value` (System.Drawing.Size) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.Drawing.Size@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.Drawing.Size@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetDefault_8.htm)

#### `public bool TryGetDefault(string key, out string value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.String@)"]
- `value` (System.String) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.String@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.String@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetDefault_10.htm)

#### `public bool TryGetDefault(string key, out string[] value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.String[]@)"]
- `value` (System.String[]) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.String[]@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetDefault(System.String,System.String[]@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetDefault_11.htm)

#### `public bool TryGetDouble(string key, out double value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetDouble(System.String,System.Double@)"]
- `value` (System.Double) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetDouble(System.String,System.Double@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetDouble(System.String,System.Double@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetDouble.htm)

#### `public bool TryGetDouble(string key, out double value, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetDouble(System.String,System.Double@,System.Collections.Generic.IEnumerable{System.String})"]
- `value` (System.Double) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetDouble(System.String,System.Double@,System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.TryGetDouble(System.String,System.Double@,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetDouble(System.String,System.Double@,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetDouble_1.htm)

#### `public bool TryGetEnumValue<T>(string key, out T enumValue) where T : struct, new(), IConvertible`

Attempt to get the stored value for an enumerated setting using a custom key. Note: the enumerated value ALWAYS gets assigned! Be sure to check for success of this method to prevent erroneous use of the value.

**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetEnumValue``1(System.String,``0@)"]
- `enumValue` (T) — [Missing <param name="enumValue"/> documentation for "M:Rhino.PersistentSettings.TryGetEnumValue``1(System.String,``0@)"]

**Returns:** `Boolean` — true if successful

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetEnumValue__1.htm)

#### `public bool TryGetGuid(string key, out Guid value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetGuid(System.String,System.Guid@)"]
- `value` (System.Guid) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetGuid(System.String,System.Guid@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetGuid(System.String,System.Guid@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetGuid.htm)

#### `public bool TryGetGuid(string key, out Guid value, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetGuid(System.String,System.Guid@,System.Collections.Generic.IEnumerable{System.String})"]
- `value` (System.Guid) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetGuid(System.String,System.Guid@,System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.TryGetGuid(System.String,System.Guid@,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetGuid(System.String,System.Guid@,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetGuid_1.htm)

#### `public bool TryGetInteger(string key, out int value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetInteger(System.String,System.Int32@)"]
- `value` (System.Int32) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetInteger(System.String,System.Int32@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetInteger(System.String,System.Int32@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetInteger.htm)

#### `public bool TryGetInteger(string key, out int value, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetInteger(System.String,System.Int32@,System.Collections.Generic.IEnumerable{System.String})"]
- `value` (System.Int32) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetInteger(System.String,System.Int32@,System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.TryGetInteger(System.String,System.Int32@,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetInteger(System.String,System.Int32@,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetInteger_1.htm)

#### `public bool TryGetPoint(string key, out Point value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetPoint(System.String,System.Drawing.Point@)"]
- `value` (System.Drawing.Point) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetPoint(System.String,System.Drawing.Point@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetPoint(System.String,System.Drawing.Point@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetPoint.htm)

#### `public bool TryGetPoint(string key, out Point value, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetPoint(System.String,System.Drawing.Point@,System.Collections.Generic.IEnumerable{System.String})"]
- `value` (System.Drawing.Point) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetPoint(System.String,System.Drawing.Point@,System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.TryGetPoint(System.String,System.Drawing.Point@,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetPoint(System.String,System.Drawing.Point@,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetPoint_1.htm)

#### `public bool TryGetPoint3d(string key, out Point3d value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetPoint3d(System.String,Rhino.Geometry.Point3d@)"]
- `value` (Rhino.Geometry.Point3d) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetPoint3d(System.String,Rhino.Geometry.Point3d@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetPoint3d(System.String,Rhino.Geometry.Point3d@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetPoint3d.htm)

#### `public bool TryGetPoint3d(string key, out Point3d value, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetPoint3d(System.String,Rhino.Geometry.Point3d@,System.Collections.Generic.IEnumerable{System.String})"]
- `value` (Rhino.Geometry.Point3d) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetPoint3d(System.String,Rhino.Geometry.Point3d@,System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.TryGetPoint3d(System.String,Rhino.Geometry.Point3d@,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetPoint3d(System.String,Rhino.Geometry.Point3d@,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetPoint3d_1.htm)

#### `public bool TryGetRectangle(string key, out Rectangle value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetRectangle(System.String,System.Drawing.Rectangle@)"]
- `value` (System.Drawing.Rectangle) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetRectangle(System.String,System.Drawing.Rectangle@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetRectangle(System.String,System.Drawing.Rectangle@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetRectangle.htm)

#### `public bool TryGetRectangle(string key, out Rectangle value, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetRectangle(System.String,System.Drawing.Rectangle@,System.Collections.Generic.IEnumerable{System.String})"]
- `value` (System.Drawing.Rectangle) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetRectangle(System.String,System.Drawing.Rectangle@,System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.TryGetRectangle(System.String,System.Drawing.Rectangle@,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetRectangle(System.String,System.Drawing.Rectangle@,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetRectangle_1.htm)

#### `public bool TryGetSettingIsHiddenFromUserInterface(string key, out bool value)`

Values read from all users settings files will be marked as read-only which will cause any future calls to Set... to fail.

**Parameters:**
- `key` (System.String) — Key name for which to search.
- `value` (System.Boolean) — Value will be true if the setting is read-only otherwise false. setting.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetSettingIsHiddenFromUserInterface(System.String,System.Boolean@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetSettingIsHiddenFromUserInterface.htm)

#### `public bool TryGetSettingIsHiddenFromUserInterface(string key, out bool value, IEnumerable<string> legacyKeyList)`

Values read from all users settings files will be marked as read-only which will cause any future calls to Set... to fail.

**Parameters:**
- `key` (System.String) — Key name for which to search.
- `value` (System.Boolean) — Value will be true if the setting is read-only otherwise false. setting.
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.TryGetSettingIsHiddenFromUserInterface(System.String,System.Boolean@,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetSettingIsHiddenFromUserInterface(System.String,System.Boolean@,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetSettingIsHiddenFromUserInterface_1.htm)

#### `public bool TryGetSettingIsReadOnly(string key, out bool value)`

Values read from all users settings files will be marked as read-only which will cause any future calls to Set... to fail.

**Parameters:**
- `key` (System.String) — Key name for which to search.
- `value` (System.Boolean) — Value will be true if the setting is read-only otherwise false. setting.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetSettingIsReadOnly(System.String,System.Boolean@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetSettingIsReadOnly.htm)

#### `public bool TryGetSettingType(string key, out Type type)`

Get the type of the last value passed to Set... or Get... for the specified setting.

**Parameters:**
- `key` (System.String) — Key name for which to search.
- `type` (System.Type) — Type of the last value passed to Set... or Get... for the specified setting.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetSettingType(System.String,System.Type@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetSettingType.htm)

#### `public bool TryGetSize(string key, out Size value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetSize(System.String,System.Drawing.Size@)"]
- `value` (System.Drawing.Size) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetSize(System.String,System.Drawing.Size@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetSize(System.String,System.Drawing.Size@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetSize.htm)

#### `public bool TryGetSize(string key, out Size value, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetSize(System.String,System.Drawing.Size@,System.Collections.Generic.IEnumerable{System.String})"]
- `value` (System.Drawing.Size) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetSize(System.String,System.Drawing.Size@,System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.TryGetSize(System.String,System.Drawing.Size@,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetSize(System.String,System.Drawing.Size@,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetSize_1.htm)

#### `public bool TryGetString(string key, out string value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetString(System.String,System.String@)"]
- `value` (System.String) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetString(System.String,System.String@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetString(System.String,System.String@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetString.htm)

#### `public bool TryGetString(string key, out string value, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetString(System.String,System.String@,System.Collections.Generic.IEnumerable{System.String})"]
- `value` (System.String) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetString(System.String,System.String@,System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.TryGetString(System.String,System.String@,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetString(System.String,System.String@,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetString_1.htm)

#### `public bool TryGetStringDictionary(string key, out KeyValuePair<string, string>[] value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetStringDictionary(System.String,System.Collections.Generic.KeyValuePair{System.String,System.String}[]@)"]
- `value` (System.Collections.Generic.KeyValuePair<String,String>[]) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetStringDictionary(System.String,System.Collections.Generic.KeyValuePair{System.String,System.String}[]@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetStringDictionary(System.String,System.Collections.Generic.KeyValuePair{System.String,System.String}[]@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetStringDictionary.htm)

#### `public bool TryGetStringDictionary(string key, out KeyValuePair<string, string>[] value, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetStringDictionary(System.String,System.Collections.Generic.KeyValuePair{System.String,System.String}[]@,System.Collections.Generic.IEnumerable{System.String})"]
- `value` (System.Collections.Generic.KeyValuePair<String,String>[]) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetStringDictionary(System.String,System.Collections.Generic.KeyValuePair{System.String,System.String}[]@,System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.TryGetStringDictionary(System.String,System.Collections.Generic.KeyValuePair{System.String,System.String}[]@,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetStringDictionary(System.String,System.Collections.Generic.KeyValuePair{System.String,System.String}[]@,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetStringDictionary_1.htm)

#### `public bool TryGetStringList(string key, out string[] value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetStringList(System.String,System.String[]@)"]
- `value` (System.String[]) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetStringList(System.String,System.String[]@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetStringList(System.String,System.String[]@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetStringList.htm)

#### `public bool TryGetStringList(string key, out string[] value, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetStringList(System.String,System.String[]@,System.Collections.Generic.IEnumerable{System.String})"]
- `value` (System.String[]) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetStringList(System.String,System.String[]@,System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.TryGetStringList(System.String,System.String[]@,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetStringList(System.String,System.String[]@,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetStringList_1.htm)

#### `public bool TryGetUnsignedInteger(string key, out uint value)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetUnsignedInteger(System.String,System.UInt32@)"]
- `value` (System.UInt32) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetUnsignedInteger(System.String,System.UInt32@)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetUnsignedInteger(System.String,System.UInt32@)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetUnsignedInteger.htm)

#### `public bool TryGetUnsignedInteger(string key, out uint value, IEnumerable<string> legacyKeyList)`



**Parameters:**
- `key` (System.String) — [Missing <param name="key"/> documentation for "M:Rhino.PersistentSettings.TryGetUnsignedInteger(System.String,System.UInt32@,System.Collections.Generic.IEnumerable{System.String})"]
- `value` (System.UInt32) — [Missing <param name="value"/> documentation for "M:Rhino.PersistentSettings.TryGetUnsignedInteger(System.String,System.UInt32@,System.Collections.Generic.IEnumerable{System.String})"]
- `legacyKeyList` (System.Collections.Generic.IEnumerable<String>) — [Missing <param name="legacyKeyList"/> documentation for "M:Rhino.PersistentSettings.TryGetUnsignedInteger(System.String,System.UInt32@,System.Collections.Generic.IEnumerable{System.String})"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.PersistentSettings.TryGetUnsignedInteger(System.String,System.UInt32@,System.Collections.Generic.IEnumerable{System.String})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettings_TryGetUnsignedInteger_1.htm)

### Properties
- `ChildKeys` (ICollection<String>, get) — Gets a collection containing the keys in the settings dictionary.
- `HiddenFromUserInterface` (Boolean, get/set) — If false then values will appear in the EditOptions window
- `Keys` (ICollection<String>, get) — Gets a collection containing the keys in the settings dictionary.
- `RhinoAppSettings` (PersistentSettings, get) — 
- `StringListRootKey` (String, get) — Adding this string to a string list when calling SetStringList will cause the ProgramData setting to get inserted at that location in the list.

## PersistentSettingsConverter (class)

Used to convert string to string lists and string dictionaries and back to strings again.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PersistentSettingsConverter.htm)

### Methods
#### `public static bool IsStringDictionary(string s)`

Determines if the string value is formatted as a key value pair string list.

**Parameters:**
- `s` (System.String) — String to check

**Returns:** `Boolean` — Returns true if it is a XML key value pair list otherwise return false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettingsConverter_IsStringDictionary.htm)

#### `public static bool IsStringList(string s)`

Determines if the string value is formatted as a string list.

**Parameters:**
- `s` (System.String) — String to check

**Returns:** `Boolean` — Returns true if it is a XML string list otherwise return false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettingsConverter_IsStringList.htm)

#### `public static string ToString(double value)`

Converts a double value to a string.

**Parameters:**
- `value` (System.Double) — double value

**Returns:** `String` — Returns the double value as a settings file formatted string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettingsConverter_ToString_1.htm)

#### `public static string ToString(KeyValuePair<string, string>[] value)`

Converts a key value string pair array to a properly formatted string dictionary XML string.

**Parameters:**
- `value` (System.Collections.Generic.KeyValuePair<String,String>[]) — List of string pairs to turn into a dictionary XML string.

**Returns:** `String` — Returns a properly formatted XML string that represents the string dictionary.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettingsConverter_ToString.htm)

#### `public static string ToString(string[] values)`

Converts a string array to a properly formatted string list XML string.

**Parameters:**
- `values` (System.String[]) — List of strings to turn into a string list XML string.

**Returns:** `String` — Returns a properly formatted XML string that represents the list of strings.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettingsConverter_ToString_2.htm)

#### `public static bool TryParseDouble(string s, out double value)`

Converts the string representation of a number to its double-precision floating-point number equivalent. A return value indicates whether the conversion succeeded or failed. system culture.

**Parameters:**
- `s` (System.String) — A string containing a number to convert.
- `value` (System.Double) — When this method returns, contains the double-precision floating-point number equivalent of the s parameter, if the conversion succeeded, or zero if the conversion failed. The conversion fails if the s parameter is null or Empty, is not a number in a valid format, or represents a number less than MinValue or greater than MaxValue. This parameter is passed uninitialized; any value originally supplied in result will be overwritten.

**Returns:** `Boolean` — Returns true if s was converted successfully; otherwise, false..

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettingsConverter_TryParseDouble.htm)

#### `public static bool TryParseEnum(Type type, string enumValueName, out int value)`

Converts an enumerated value name to its integer equivalent.

**Parameters:**
- `type` (System.Type) — The enumerated type
- `enumValueName` (System.String) — Enumerated value name as string
- `value` (System.Int32) — Output value, will get set to -1 on error

**Returns:** `Boolean` — Returns true if the successfully converted or false if not.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettingsConverter_TryParseEnum.htm)

#### `public static bool TryParseEnum(Type type, string intValueAsString, out string value)`

Converts an enumerated value string (integer as string) to a enumerated value name.

**Parameters:**
- `type` (System.Type) — The enumerated type
- `intValueAsString` (System.String) — enumerated integer value as string
- `value` (System.String) — Output value, will be null on error

**Returns:** `Boolean` — Returns true if the successfully converted or false if not.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettingsConverter_TryParseEnum_1.htm)

#### `public static bool TryParseStringDictionary(string s, out KeyValuePair<string, string>[] value)`

Attempts to convert a string to a key value string pair array.

**Parameters:**
- `s` (System.String) — String to parse
- `value` (System.Collections.Generic.KeyValuePair<String,String>[]) — Result will get copied here, if the string is null or empty then this will be an empty array, if there was an error parsing then this will be null otherwise it will be the string parsed as a key value string pair array.

**Returns:** `Boolean` — Returns true if the string is not empty and properly formatted as a key value string pair list otherwise returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettingsConverter_TryParseStringDictionary.htm)

#### `public static bool TryParseStringList(string s, out string[] value)`

Attempts to convert a string to a string value list.

**Parameters:**
- `s` (System.String) — String to parse
- `value` (System.String[]) — Result will get copied here, if the string is null or empty then this will be an empty list, if there was an error parsing then this will be null otherwise it will be the string parsed as a list.

**Returns:** `Boolean` — Returns true if the string is not empty and properly formatted as a string list otherwise returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettingsConverter_TryParseStringList.htm)

## PersistentSettingsEventArgs (class)

Represents event data that is passed as state in persistent settings events.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PersistentSettingsEventArgs.htm)

### Constructors
- `protected PersistentSettingsEventArgs()` — Initializes a new instance of the PersistentSettingsEventArgs class

### Properties
- `Cancel` (Boolean, get/set) — 

## PersistentSettingsEventArgs<T> (class)

Represents the persistent settings modification event arguments.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PersistentSettingsEventArgs_1.htm)

### Constructors
- `public PersistentSettingsEventArgs(T currentValue, T newValue)` — Initializes a new instance of the PersistentSettingsEventArgs<T> class

### Properties
- `Cancel` (Boolean, get/set) — (Inherited from PersistentSettingsEventArgs.)
- `CurrentValue` (T, get/set) — 
- `NewValue` (T, get) — 

## PersistentSettingsSavedEventArgs (class)

Event argument passed to the SettingsSaved event.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_PersistentSettingsSavedEventArgs.htm)

### Methods
#### `public PersistentSettings CommandSettings(string englishCommandName)`

The new command settings

**Parameters:**
- `englishCommandName` (System.String) — English command to find settings for

**Returns:** `PersistentSettings` — [Missing <returns> documentation for "M:Rhino.PersistentSettingsSavedEventArgs.CommandSettings(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_PersistentSettingsSavedEventArgs_CommandSettings.htm)

### Properties
- `PlugInSettings` (PersistentSettings, get) — The old PlugIn settings
- `SavedByThisRhino` (Boolean, get) — Will be true if this instance of Rhino is writing the settings file or false if a different instance of Rhino has modified the settings file.

## ReadFileResult (enum)

ON::ReadFileResult reports what happened when a file read was attempted.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_ReadFileResult.htm)

### Values
- `Unset` = `0` — No result is available.
- `Completed` = `1` — Read completed with no errors.
- `CompletedWithErrors` = `2` — Read completed with non fatal errors.
- `Failed` = `3` — Read failed.

## RhinoApp (class)

.NET RhinoApp is parallel to C++ CRhinoApp.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_RhinoApp.htm)

### Methods
#### `public static bool AskUserForRhinoLicense(bool standAlone, Object parentWindow)`

Display UI asking the user to enter a license for Rhino or use one from the Zoo.

**Parameters:**
- `standAlone` (System.Boolean) — True to ask for a stand-alone license, false to ask the user for a license from the Zoo
- `parentWindow` (System.Object) — Parent window for the user interface dialog.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.RhinoApp.AskUserForRhinoLicense(System.Boolean,System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_AskUserForRhinoLicense.htm)

#### `public static string[] CapturedCommandWindowStrings(bool clearBuffer)`

Get list of strings sent to the command window through calls to Write or WriteLine when capturing has been enabled

**Parameters:**
- `clearBuffer` (System.Boolean) — Clear the captured buffer after this call

**Returns:** `String[]` — array of captured strings

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_CapturedCommandWindowStrings.htm)

#### `public static bool ChangeLicenseKey(Guid pluginId)`

Display UI asking the user to enter a license for the product specified by licenseId.

**Parameters:**
- `pluginId` (System.Guid) — Guid identifying the plug-in that is requesting a change of license key

**Returns:** `Boolean` — true on success, false otherwise

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_ChangeLicenseKey.htm)

#### `public static void ClearCommandHistoryWindow()`

Clear the text in Rhino's command history window.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_ClearCommandHistoryWindow.htm)

#### `public static Result ExecuteCommand(RhinoDoc document, string commandName)`

Execute a Rhino command.

**Parameters:**
- `document` (Rhino.RhinoDoc) — Document to execute the command for
- `commandName` (System.String) — Name of command to run. Use command's localized name or preface with an underscore.

**Returns:** `Result` — Returns the result of the command.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_ExecuteCommand.htm)

#### `public static void Exit()`

Exits, or closes, Rhino.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_Exit.htm)

#### `public static void Exit(bool allowCancel)`

Exits, or forcefully closes Rhino. A prompt to allow saving will appear if necessary when forcefully exiting Works on Windows and MacOS true to allow the user to cancel exiting false to force exit

**Parameters:**
- `allowCancel` (System.Boolean) — [Missing <param name="allowCancel"/> documentation for "M:Rhino.RhinoApp.Exit(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_Exit_1.htm)

#### `public static string GetDataDirectory(bool localUser, bool forceDirectoryCreation)`

Gets the data directory.

**Parameters:**
- `localUser` (System.Boolean) — If set to true local user.
- `forceDirectoryCreation` (System.Boolean) — If set to true force directory creation.

**Returns:** `String` — The data directory.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_GetDataDirectory.htm)

#### `public static string GetDataDirectory(bool localUser, bool forceDirectoryCreation, string subDirectory)`

Gets the data directory.

**Parameters:**
- `localUser` (System.Boolean) — If set to true local user.
- `forceDirectoryCreation` (System.Boolean) — If set to true force directory creation.
- `subDirectory` (System.String) — Sub directory, will get appended to the end of the data directory. if forceDirectoryCreation is true then this directory will get created and writable.

**Returns:** `String` — The data directory.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_GetDataDirectory_1.htm)

#### `public static DirectoryInfo GetExecutableDirectory()`

directory

**Returns:** `DirectoryInfo` — [Missing <returns> documentation for "M:Rhino.RhinoApp.GetExecutableDirectory"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_GetExecutableDirectory.htm)

#### `public static Object GetPlugInObject(Guid pluginId)`

Gets the object that is returned by PlugIn.GetPlugInObject for a given plug-in. This function attempts to find and load a plug-in with a given Id. When a plug-in is found, it's GetPlugInObject function is called and the result is returned here. Note the plug-in must have already been installed in Rhino or the plug-in manager will not know where to look for a plug-in with a matching id.

**Parameters:**
- `pluginId` (System.Guid) — Guid for a given plug-in.

**Returns:** `Object` — Result of PlugIn.GetPlugInObject for a given plug-in on success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_GetPlugInObject.htm)

#### `public static Object GetPlugInObject(string plugin)`

Gets the object that is returned by PlugIn.GetPlugInObject for a given plug-in. This function attempts to find and load a plug-in with a given name. When a plug-in is found, it's GetPlugInObject function is called and the result is returned here. Note the plug-in must have already been installed in Rhino or the plug-in manager will not know where to look for a plug-in with a matching name.

**Parameters:**
- `plugin` (System.String) — Name of a plug-in.

**Returns:** `Object` — Result of PlugIn.GetPlugInObject for a given plug-in on success.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_GetPlugInObject_1.htm)

#### `public static bool InFullScreen()`

Verifies that Rhino is running in full screen mode.

**Returns:** `Boolean` — true if Rhino is running full screen, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_InFullScreen.htm)

#### `public static void InvokeAndWait(Action action)`

Work-In-Progress. Testing this with our unit test framework

**Parameters:**
- `action` (System.Action) — [Missing <param name="action"/> documentation for "M:Rhino.RhinoApp.InvokeAndWait(System.Action)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_InvokeAndWait.htm)

#### `public static void InvokeOnUiThread(Delegate method, params Object[] args)`

Execute a function on the main UI thread.

**Parameters:**
- `method` (System.Delegate) — function to execute
- `args` (System.Object[]) — parameters to pass to the function

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_InvokeOnUiThread.htm)

#### `public static bool IsInstallationBeta(Installation licenseType)`

If licenseType is a beta license, returns true. A beta license grants full use of the product during the pre-release development period.

**Parameters:**
- `licenseType` (Rhino.ApplicationSettings.Installation) — [Missing <param name="licenseType"/> documentation for "M:Rhino.RhinoApp.IsInstallationBeta(Rhino.ApplicationSettings.Installation)"]

**Returns:** `Boolean` — true if licenseType is a beta license. false otherwise

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_IsInstallationBeta.htm)

#### `public static bool IsInstallationCommercial(Installation licenseType)`

If licenseType is a commercial license, returns true. A commercial license grants full use of the product.

**Parameters:**
- `licenseType` (Rhino.ApplicationSettings.Installation) — [Missing <param name="licenseType"/> documentation for "M:Rhino.RhinoApp.IsInstallationCommercial(Rhino.ApplicationSettings.Installation)"]

**Returns:** `Boolean` — true if licenseType is a commercial license. false otherwise

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_IsInstallationCommercial.htm)

#### `public static bool IsInstallationEvaluation(Installation licenseType)`

If licenseType is an evaluation license, returns true. An evaluation license limits the ability of Rhino to save based on either the number of saves or a fixed period of time.

**Parameters:**
- `licenseType` (Rhino.ApplicationSettings.Installation) — [Missing <param name="licenseType"/> documentation for "M:Rhino.RhinoApp.IsInstallationEvaluation(Rhino.ApplicationSettings.Installation)"]

**Returns:** `Boolean` — true if licenseType is an evaluation license. false otherwise

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_IsInstallationEvaluation.htm)

#### `public static bool LoginToCloudZoo()`

Logs in to the cloud zoo.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.RhinoApp.LoginToCloudZoo"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_LoginToCloudZoo.htm)

#### `public static IntPtr MainWindowHandle()`

Gets the HWND of the Rhino main window.

**Returns:** `IntPtr` — [Missing <returns> documentation for "M:Rhino.RhinoApp.MainWindowHandle"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_MainWindowHandle.htm)

#### `public static void OutputDebugString(string str)`

Print a string to the Visual Studio Output window, if a debugger is attached. Note that the developer needs to add a newline manually if the next output should come on a separate line.

**Parameters:**
- `str` (System.String) — The string to print to the Output window.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_OutputDebugString.htm)

#### `public static string ParseTextField(string formula, RhinoObject obj, RhinoObject topParentObject)`

Parses a text field string.

**Parameters:**
- `formula` (System.String) — The text formula.
- `obj` (Rhino.DocObjects.RhinoObject) — The Rhino object. Value can be IntPtr.Zero.
- `topParentObject` (Rhino.DocObjects.RhinoObject) — The parent Rhino object. Value can be IntPtr.Zero.

**Returns:** `String` — The parsed text field if sucessful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_ParseTextField.htm)

#### `public static string ParseTextField(string formula, RhinoObject obj, RhinoObject topParentObject, InstanceObject immediateParent)`

Parses a text field string.

**Parameters:**
- `formula` (System.String) — The text formula.
- `obj` (Rhino.DocObjects.RhinoObject) — The Rhino object. Value can be IntPtr.Zero.
- `topParentObject` (Rhino.DocObjects.RhinoObject) — The parent Rhino object. Value can be IntPtr.Zero.
- `immediateParent` (Rhino.DocObjects.InstanceObject) — The immediate parent instance object. Value can be IntPtr.Zero.

**Returns:** `String` — The parsed text field if sucessful.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_ParseTextField_1.htm)

#### `public static void PostCancelEvent(uint runtimeDocSerialNumber)`

Post a cancel event to the command line

**Parameters:**
- `runtimeDocSerialNumber` (System.UInt32) — Unique serialNumber for the document to post the event to.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_PostCancelEvent.htm)

#### `public static void PostEnterEvent(uint runtimeDocSerialNumber, bool bRepeatedEnter)`

Post an enter event to the command line

**Parameters:**
- `runtimeDocSerialNumber` (System.UInt32) — Unique serialNumber for the document to post the event to.
- `bRepeatedEnter` (System.Boolean) — if true, allow multiple enter events to be posted simultaneouslyt.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_PostEnterEvent.htm)

#### `public static bool RefreshRhinoLicense()`

Refresh the license used by Rhino. This allows any part of Rhino to ensure that the most current version of the license file on disk is in use.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.RhinoApp.RefreshRhinoLicense"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_RefreshRhinoLicense.htm)

#### `public static bool ReleaseMouseCapture()`

Releases the mouse capture.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.RhinoApp.ReleaseMouseCapture"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_ReleaseMouseCapture.htm)

#### `public static bool RunMenuScript(string script)`

Run a Rhino menu item script. Will add the selected menu string to the MRU command menu.

**Remarks:** Rhino acts as if each character in the script string had been typed in the command prompt. When RunScript is called from a "script runner" command, it completely runs the script before returning. When RunScript is called outside of a command, it returns and the script is run. This way menus and buttons can use RunScript to execute complicated functions.

**Parameters:**
- `script` (System.String) — [in] script to run.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.RhinoApp.RunMenuScript(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_RunMenuScript.htm)

#### `public static bool RunScript(string script, bool echo)`

Runs a Rhino command script.

**Remarks:** Rhino acts as if each character in the script string had been typed in the command prompt. When RunScript is called from a "script runner" command, it completely runs the script before returning. When RunScript is called outside of a command, it returns and the script is run. This way menus and buttons can use RunScript to execute complicated functions.

**Parameters:**
- `script` (System.String) — [in] script to run.
- `echo` (System.Boolean) — Controls how the script is echoed in the command output window. false = silent - nothing is echoed. true = verbatim - the script is echoed literally.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.RhinoApp.RunScript(System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_RunScript.htm)

#### `public static bool RunScript(string script, string mruDisplayString, bool echo)`

Runs a Rhino command script.

**Remarks:** Rhino acts as if each character in the script string had been typed in the command prompt. When RunScript is called from a "script runner" command, it completely runs the script before returning. When RunScript is called outside of a command, it returns and the script is run. This way menus and buttons can use RunScript to execute complicated functions.

**Parameters:**
- `script` (System.String) — [in] script to run.
- `mruDisplayString` (System.String) — [in] String to display in the most recent command list.
- `echo` (System.Boolean) — Controls how the script is echoed in the command output window. false = silent - nothing is echoed. true = verbatim - the script is echoed literally.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.RhinoApp.RunScript(System.String,System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_RunScript_1.htm)

#### `public static bool RunScript(uint documentSerialNumber, string script, bool echo)`

Runs a Rhino command script.

**Remarks:** Rhino acts as if each character in the script string had been typed in the command prompt. When RunScript is called from a "script runner" command, it completely runs the script before returning. When RunScript is called outside of a command, it returns and the script is run. This way menus and buttons can use RunScript to execute complicated functions.

**Parameters:**
- `documentSerialNumber` (System.UInt32) — [in] Document serial number for the document to run the script for.
- `script` (System.String) — [in] script to run.
- `echo` (System.Boolean) — [in] Controls how the script is echoed in the command output window. false = silent - nothing is echoed. true = verbatim - the script is echoed literally.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.RhinoApp.RunScript(System.UInt32,System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_RunScript_2.htm)

#### `public static bool RunScript(uint documentSerialNumber, string script, string mruDisplayString, bool echo)`

Runs a Rhino command script.

**Remarks:** Rhino acts as if each character in the script string had been typed in the command prompt. When RunScript is called from a "script runner" command, it completely runs the script before returning. When RunScript is called outside of a command, it returns and the script is run. This way menus and buttons can use RunScript to execute complicated functions.

**Parameters:**
- `documentSerialNumber` (System.UInt32) — [in] Document serial number for the document to run the script for.
- `script` (System.String) — [in] script to run.
- `mruDisplayString` (System.String) — [in] String to display in the most recent command list.
- `echo` (System.Boolean) — [in] Controls how the script is echoed in the command output window. false = silent - nothing is echoed. true = verbatim - the script is echoed literally.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.RhinoApp.RunScript(System.UInt32,System.String,System.String,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_RunScript_3.htm)

#### `public static bool RunningInRdp()`

Find out if Rhino is running in a remote session

**Returns:** `Boolean` — true if Rhino is running in a RDP session, false otherwise

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_RunningInRdp.htm)

#### `public static bool RunningOnVMWare()`

Verifies that Rhino is running on VMWare

**Returns:** `Boolean` — true if Rhino is running in Windows on VMWare, false otherwise

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_RunningOnVMWare.htm)

#### `public static void SendKeystrokes(string characters, bool appendReturn)`

Sends a string of printable characters, including spaces, to Rhino's command line.

**Parameters:**
- `characters` (System.String) — [in] A string to characters to send to the command line. This can be null.
- `appendReturn` (System.Boolean) — [in] Append a return character to the end of the string.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_SendKeystrokes.htm)

#### `public static void SetCommandPrompt(string prompt)`

Set Rhino command prompt.

**Parameters:**
- `prompt` (System.String) — The new prompt text.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_SetCommandPrompt.htm)

#### `public static void SetCommandPrompt(string prompt, string promptDefault)`

Sets the command prompt in Rhino.

**Parameters:**
- `prompt` (System.String) — The new prompt text.
- `promptDefault` (System.String) — Text that appears in angle brackets and indicates what will happen if the user pressed ENTER.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_SetCommandPrompt_1.htm)

#### `public static void SetCommandPromptMessage(string prompt)`

Set the text that appears in the Rhino command prompt. In general, you should use the SetCommandPrompt functions. In rare cases, like worker thread messages, the message that appears in the prompt has non-standard formatting. In these rare cases, SetCommandPromptMessage can be used to literally specify the text that appears in the command prompt window.

**Parameters:**
- `prompt` (System.String) — A literal text for the command prompt window.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_SetCommandPromptMessage.htm)

#### `public static void SetFocusToMainWindow()`

Sets the focus to the main window. This function attempts to use the ActiveDoc on Mac to figure out which window to set focus to.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_SetFocusToMainWindow.htm)

#### `public static void SetFocusToMainWindow(RhinoDoc doc)`

Sets the focus to the main windows for a given document

**Parameters:**
- `doc` (Rhino.RhinoDoc) — the document to use for determining a "main window"

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_SetFocusToMainWindow_1.htm)

#### `public static void Wait()`

Pauses to keep Windows message pump alive so views will update and windows will repaint.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_Wait.htm)

#### `public static void Write(string message)`

Print formatted text in the command window.

**Parameters:**
- `message` (System.String) — [Missing <param name="message"/> documentation for "M:Rhino.RhinoApp.Write(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_Write.htm)

#### `public static void Write(string format, Object arg0)`

Print formatted text in the command window.

**Parameters:**
- `format` (System.String) — [Missing <param name="format"/> documentation for "M:Rhino.RhinoApp.Write(System.String,System.Object)"]
- `arg0` (System.Object) — [Missing <param name="arg0"/> documentation for "M:Rhino.RhinoApp.Write(System.String,System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_Write_1.htm)

#### `public static void Write(string format, Object arg0, Object arg1)`

Print formatted text in the command window.

**Parameters:**
- `format` (System.String) — [Missing <param name="format"/> documentation for "M:Rhino.RhinoApp.Write(System.String,System.Object,System.Object)"]
- `arg0` (System.Object) — [Missing <param name="arg0"/> documentation for "M:Rhino.RhinoApp.Write(System.String,System.Object,System.Object)"]
- `arg1` (System.Object) — [Missing <param name="arg1"/> documentation for "M:Rhino.RhinoApp.Write(System.String,System.Object,System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_Write_2.htm)

#### `public static void Write(string format, Object arg0, Object arg1, Object arg2)`

Print formatted text in the command window.

**Parameters:**
- `format` (System.String) — [Missing <param name="format"/> documentation for "M:Rhino.RhinoApp.Write(System.String,System.Object,System.Object,System.Object)"]
- `arg0` (System.Object) — [Missing <param name="arg0"/> documentation for "M:Rhino.RhinoApp.Write(System.String,System.Object,System.Object,System.Object)"]
- `arg1` (System.Object) — [Missing <param name="arg1"/> documentation for "M:Rhino.RhinoApp.Write(System.String,System.Object,System.Object,System.Object)"]
- `arg2` (System.Object) — [Missing <param name="arg2"/> documentation for "M:Rhino.RhinoApp.Write(System.String,System.Object,System.Object,System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_Write_3.htm)

#### `public static void WriteLine()`

Print a newline in the command window.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_WriteLine.htm)

#### `public static void WriteLine(string message)`

Print text in the command window.

**Parameters:**
- `message` (System.String) — [Missing <param name="message"/> documentation for "M:Rhino.RhinoApp.WriteLine(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_WriteLine_1.htm)

#### `public static void WriteLine(string format, Object arg0)`

Print formatted text with a newline in the command window.

**Parameters:**
- `format` (System.String) — [Missing <param name="format"/> documentation for "M:Rhino.RhinoApp.WriteLine(System.String,System.Object)"]
- `arg0` (System.Object) — [Missing <param name="arg0"/> documentation for "M:Rhino.RhinoApp.WriteLine(System.String,System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_WriteLine_2.htm)

#### `public static void WriteLine(string format, Object arg0, Object arg1)`

Print formatted text with a newline in the command window.

**Parameters:**
- `format` (System.String) — [Missing <param name="format"/> documentation for "M:Rhino.RhinoApp.WriteLine(System.String,System.Object,System.Object)"]
- `arg0` (System.Object) — [Missing <param name="arg0"/> documentation for "M:Rhino.RhinoApp.WriteLine(System.String,System.Object,System.Object)"]
- `arg1` (System.Object) — [Missing <param name="arg1"/> documentation for "M:Rhino.RhinoApp.WriteLine(System.String,System.Object,System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_WriteLine_3.htm)

#### `public static void WriteLine(string format, Object arg0, Object arg1, Object arg2)`

Print formatted text with a newline in the command window.

**Parameters:**
- `format` (System.String) — [Missing <param name="format"/> documentation for "M:Rhino.RhinoApp.WriteLine(System.String,System.Object,System.Object,System.Object)"]
- `arg0` (System.Object) — [Missing <param name="arg0"/> documentation for "M:Rhino.RhinoApp.WriteLine(System.String,System.Object,System.Object,System.Object)"]
- `arg1` (System.Object) — [Missing <param name="arg1"/> documentation for "M:Rhino.RhinoApp.WriteLine(System.String,System.Object,System.Object,System.Object)"]
- `arg2` (System.Object) — [Missing <param name="arg2"/> documentation for "M:Rhino.RhinoApp.WriteLine(System.String,System.Object,System.Object,System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_WriteLine_4.htm)

### Properties
- `BuildDate` (DateTime, get) — Gets the build date.
- `CanSave` (Boolean, get) — Returns true when Rhino is allowed to save, false otherwise Conditions where Rhino is not allowed to save are: when evaluation licenses are expired; when a Cloud Zoo lease is expired; when a license is shared by a single user on multiple computers, and the current computer is not active.
- `CheckNewObject` (Boolean, get/set) — Returns true if Rhino will validate each object added to the document. This can be time consuming but is valuable for debugging.
- `CommandHistoryWindowText` (String, get) — Text in Rhino's command history window.
- `CommandLineOut` (RhinoApp.CommandLineTextWriter, get) — Provides a TextWriter that can write to the command line.
- `CommandPrompt` (String, get/set) — Rhino command prompt.
- `CommandWindowCaptureEnabled` (Boolean, get/set) — Enable or disable capturing of the strings sent to the CommandWindow through Write and WriteLine calls
- `CurrentRhinoId` (Guid, get) — Gets the current ID of Rhino.
- `DaysUntilExpiration` (Int32, get) — Returns number of days until license expires. Zero when license is expired. Raises InvalidLicenseTypeException if LicenseExpires would return false.
- `DefaultUiFont` (Font, get) — Default font used to render user interface
- `ExeServiceRelease` (Int32, get) — Service release version of Rhino executable (0, 1, 2, ...) The integer is the service release number of Rhino. For example, this function returns "0" if Rhino V4SR0 is running and returns "1" if Rhino V4SR1 is running.
- `ExeVersion` (Int32, get) — Major version of Rhino executable 4, 5, ...
- `InCommand` (Int32, get) — Gets the nested command count.
- `InstallationType` (Installation, get) — Gets the product installation type, as seen in Rhino's ABOUT dialog box.
- `InstallationTypeString` (String, get) — Gets the type of installation (product edition) of the license or lease.
- `InvokeRequired` (Boolean, get) — Returns true if we are currently not running on the main user interface thread
- `IsClosing` (Boolean, get) — Returns true if Rhino is in the process of closing, false otherwise. This can be true even before the Closing event fires, such as when RhinoDoc.CloseDocument event is called.
- `IsCloudZooNode` (Boolean, get) — Returns true if rhino is currently using the Cloud Zoo false otherwise
- `IsEvaluation` (Boolean, get) — Returns true if Rhino is licensed as an Evaluation product false otherwise
- `IsExiting` (Boolean, get) — Returns true if Rhino is in the process of exiting, false otherwise. This can be true even before the Closing event fires, such as when RhinoDoc.CloseDocument event is called.
- `IsInternetAccessAllowed` (Boolean, get) — Returns true when Rhino is allowed to access the Internet, false otherwise. Note, this does not test if Internet access is available.
- `IsLicenseValidated` (Boolean, get) — Returns true if the license is validated false otherwise
- `IsOnMainThread` (Boolean, get) — Is the current thread the main thread
- `IsParentDesktop` (Boolean, get) — Returns true if Rhino's parent window is the desktop, false otherwise.
- `IsPreRelease` (Boolean, get) — Returns true if Rhino is compiled as pre-release build (Beta, WIP) false otherwise
- `IsRunningAutomated` (Boolean, get) — Is Rhino currently being executed through automation
- `IsRunningHeadless` (Boolean, get) — Is Rhino currently being executed in headless mode
- `IsSafeModeEnabled` (Boolean, get) — Is Rhino being executed in safe mode
- `IsSkinned` (Boolean, get) — Is Rhino currently using custom, user-interface Skin.
- `LicenseExpires` (Boolean, get) — Returns true if the license will expire false otherwise
- `LicenseUserName` (String, get) — Gets the name of the user that owns the license or lease.
- `LicenseUserOrganization` (String, get) — Gets the name of the organization of the user that owns the license or lease.
- `LoggedInUserAvatar` (Image, get) — Returns the logged in user's avatar picture. Returns a default avatar if the user does not have an avatar or if the avatar could not be fetched.
- `LoggedInUserName` (String, get) — Returns the name of the logged in user, or null if the user is not logged in.
- `Name` (String, get) — Gets the application name.
- `NodeType` (LicenseNode, get) — Gets license the node type.
- `Rhino2Id` (Guid, get) — Gets the ID of Rhino 2.
- `Rhino3Id` (Guid, get) — Gets the ID of Rhino 3.
- `Rhino4Id` (Guid, get) — Gets the ID of Rhino 4.
- `Rhino5Id` (Guid, get) — Gets the ID of Rhino 5
- `Rhino6Id` (Guid, get) — Gets the ID of Rhino 6
- `Rhino7Id` (Guid, get) — Gets the ID of Rhino 7
- `SchemeName` (String, get) — Gets the current Registry scheme name.
- `SdkServiceRelease` (Int32, get) — Rhino SDK 9 digit SDK service release number in the form YYYYMMDDn Service release of the Rhino SDK supported by this executable. Rhino will only load plug-ins that require a service release of <= this release number. For example, SR1 will load all plug-ins made with any SDK released up through and including the SR1 SDK. But, SR1 will not load a plug-in built using the SR2 SDK. If an "old" Rhino tries to load a "new" plug-in, the user is told that they have to get a free Rhino.exe update in order for the plug-in to load. Rhino.exe updates are available from http://www.rhino3d.com.
- `SdkVersion` (Int32, get) — Rhino SDK 9 digit SDK version number in the form YYYYMMDDn Rhino will only load plug-ins that were build with exactly the same version of the SDK.
- `SendWriteToConsole` (Boolean, get/set) — Enable or disable sending command window strings to the console RhinoApp.Write(...) calls would be sent to the console when this is enabled
- `SerialNumber` (String, get) — Gets the product serial number, as seen in Rhino's ABOUT dialog box.
- `ToolbarFiles` (ToolbarFileCollection, get) — Collection of currently open toolbar files in the application
- `UpdatesAndStatisticsStatus` (Int32, get) — Returns true when Rhino is allowed to access the Internet, false otherwise. Note, this does not test if Internet access is available.
- `UserIsLoggedIn` (Boolean, get) — Returns true if the user is logged in; else returns false. A logged in user does not guarantee that the auth tokens managed by the CloudZooManager instance are valid.
- `ValidationGracePeriodDaysLeft` (Int32, get) — Returns number of days within which validation must occur. Zero when validation grace period has expired. Raises InvalidLicenseTypeException if LicenseType is one of: EvaluationSaveLimited EvaluationTimeLimited Viewer Unknown
- `Version` (Version, get) — File version of the main Rhino process
- `VersionControlRevision` (String, get) — McNeel version control revision identifier at the time this version of Rhino was built.

### Events
#### `AppSettingsChanged` (`System.EventHandler`)

**Signature:** `public static event EventHandler AppSettingsChanged`

Is raised when settings are changed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoApp_AppSettingsChanged.htm)

#### `Closing` (`System.EventHandler`)

**Signature:** `public static event EventHandler Closing`

Is raised when the application is about to close.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoApp_Closing.htm)

#### `EscapeKeyPressed` (`System.EventHandler`)

**Signature:** `public static event EventHandler EscapeKeyPressed`

Can add or removed delegates that are raised when the escape key is clicked.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoApp_EscapeKeyPressed.htm)

#### `Idle` (`System.EventHandler`)

**Signature:** `public static event EventHandler Idle`

Occurs when the application finishes processing and is about to enter the idle state

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoApp_Idle.htm)

#### `Initialized` (`System.EventHandler`)

**Signature:** `public static event EventHandler Initialized`

Is raised when the application is fully initialized.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoApp_Initialized.htm)

#### `KeyboardEvent` (`Rhino.RhinoApp.KeyboardHookEvent`)

**Signature:** `public static event RhinoApp.KeyboardHookEvent KeyboardEvent`

Can add or removed delegates that are raised by a keyboard event.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoApp_KeyboardEvent.htm)

#### `LicenseStateChanged` (`System.EventHandler<LicenseStateChangedEventArgs>`)

**Signature:** `public static event EventHandler<LicenseStateChangedEventArgs> LicenseStateChanged`

Fires when the license state has changed

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoApp_LicenseStateChanged.htm)

#### `MainLoop` (`System.EventHandler`)

**Signature:** `public static event EventHandler MainLoop`

Gets called every loop iteration inside Rhino's main message loop.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoApp_MainLoop.htm)

#### `RdkCacheImageChanged` (`System.EventHandler`)

**Signature:** `public static event EventHandler RdkCacheImageChanged`

Monitors when the RDK thumbnail cache images are changed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoApp_RdkCacheImageChanged.htm)

#### `RdkGlobalSettingsChanged` (`System.EventHandler`)

**Signature:** `public static event EventHandler RdkGlobalSettingsChanged`

Monitors when RDK global settings are modified.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoApp_RdkGlobalSettingsChanged.htm)

#### `RdkNewDocument` (`System.EventHandler`)

**Signature:** `public static event EventHandler RdkNewDocument`

Monitors when RDK document information is rebuilt.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoApp_RdkNewDocument.htm)

#### `RdkPlugInUnloading` (`System.EventHandler`)

**Signature:** `public static event EventHandler RdkPlugInUnloading`

Monitors when RDK client plug-ins are unloaded.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoApp_RdkPlugInUnloading.htm)

#### `RdkUpdateAllPreviews` (`System.EventHandler`)

**Signature:** `public static event EventHandler RdkUpdateAllPreviews`

Monitors when RDK thumbnails are updated.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoApp_RdkUpdateAllPreviews.htm)

#### `RendererChanged` (`System.EventHandler`)

**Signature:** `public static event EventHandler RendererChanged`

Monitors when Rhino's current renderer changes.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoApp_RendererChanged.htm)

## RhinoApp.CommandLineTextWriter (class)

Provides a text writer that writes to the command line.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_RhinoApp_CommandLineTextWriter.htm)

### Constructors
- `public CommandLineTextWriter()` — Initializes a new instance of the RhinoApp.CommandLineTextWriter class

### Methods
#### `public override void Write(char value)`

Writes a char to the command line.

**Parameters:**
- `value` (System.Char) — [Missing <param name="value"/> documentation for "M:Rhino.RhinoApp.CommandLineTextWriter.Write(System.Char)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_CommandLineTextWriter_Write.htm)

#### `public override void Write(char[] buffer, int index, int count)`

Writes a char buffer to the command line.

**Parameters:**
- `buffer` (System.Char[]) — [Missing <param name="buffer"/> documentation for "M:Rhino.RhinoApp.CommandLineTextWriter.Write(System.Char[],System.Int32,System.Int32)"]
- `index` (System.Int32) — [Missing <param name="index"/> documentation for "M:Rhino.RhinoApp.CommandLineTextWriter.Write(System.Char[],System.Int32,System.Int32)"]
- `count` (System.Int32) — [Missing <param name="count"/> documentation for "M:Rhino.RhinoApp.CommandLineTextWriter.Write(System.Char[],System.Int32,System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_CommandLineTextWriter_Write_1.htm)

#### `public override void Write(string value)`

Writes a string to the command line.

**Parameters:**
- `value` (System.String) — [Missing <param name="value"/> documentation for "M:Rhino.RhinoApp.CommandLineTextWriter.Write(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoApp_CommandLineTextWriter_Write_2.htm)

### Properties
- `Encoding` (Encoding, get) — Returns Encoding Unicode.

## RhinoApp.KeyboardHookEvent (delegate)

KeyboardEvent delegate

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_RhinoApp_KeyboardHookEvent.htm)

## RhinoDoc (class)

Represents an active model.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_RhinoDoc.htm)

### Methods
#### `public bool AddCustomUndoEvent(string description, EventHandler<CustomUndoEventArgs> handler)`



**Parameters:**
- `description` (System.String) — [Missing <param name="description"/> documentation for "M:Rhino.RhinoDoc.AddCustomUndoEvent(System.String,System.EventHandler{Rhino.Commands.CustomUndoEventArgs})"]
- `handler` (System.EventHandler<CustomUndoEventArgs>) — [Missing <param name="handler"/> documentation for "M:Rhino.RhinoDoc.AddCustomUndoEvent(System.String,System.EventHandler{Rhino.Commands.CustomUndoEventArgs})"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.RhinoDoc.AddCustomUndoEvent(System.String,System.EventHandler{Rhino.Commands.CustomUndoEventArgs})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_AddCustomUndoEvent.htm)

#### `public bool AddCustomUndoEvent(string description, EventHandler<CustomUndoEventArgs> handler, Object tag)`

Add a custom undo event so you can undo private plug-in data when the user performs an undo or redo

**Parameters:**
- `description` (System.String) — [Missing <param name="description"/> documentation for "M:Rhino.RhinoDoc.AddCustomUndoEvent(System.String,System.EventHandler{Rhino.Commands.CustomUndoEventArgs},System.Object)"]
- `handler` (System.EventHandler<CustomUndoEventArgs>) — [Missing <param name="handler"/> documentation for "M:Rhino.RhinoDoc.AddCustomUndoEvent(System.String,System.EventHandler{Rhino.Commands.CustomUndoEventArgs},System.Object)"]
- `tag` (System.Object) — [Missing <param name="tag"/> documentation for "M:Rhino.RhinoDoc.AddCustomUndoEvent(System.String,System.EventHandler{Rhino.Commands.CustomUndoEventArgs},System.Object)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.RhinoDoc.AddCustomUndoEvent(System.String,System.EventHandler{Rhino.Commands.CustomUndoEventArgs},System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_AddCustomUndoEvent_1.htm)

#### `public void AdjustModelUnitSystem(UnitSystem newUnitSystem, bool scale)`



**Parameters:**
- `newUnitSystem` (Rhino.UnitSystem) — [Missing <param name="newUnitSystem"/> documentation for "M:Rhino.RhinoDoc.AdjustModelUnitSystem(Rhino.UnitSystem,System.Boolean)"]
- `scale` (System.Boolean) — [Missing <param name="scale"/> documentation for "M:Rhino.RhinoDoc.AdjustModelUnitSystem(Rhino.UnitSystem,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_AdjustModelUnitSystem.htm)

#### `public void AdjustPageUnitSystem(UnitSystem newUnitSystem, bool scale)`



**Parameters:**
- `newUnitSystem` (Rhino.UnitSystem) — [Missing <param name="newUnitSystem"/> documentation for "M:Rhino.RhinoDoc.AdjustPageUnitSystem(Rhino.UnitSystem,System.Boolean)"]
- `scale` (System.Boolean) — [Missing <param name="scale"/> documentation for "M:Rhino.RhinoDoc.AdjustPageUnitSystem(Rhino.UnitSystem,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_AdjustPageUnitSystem.htm)

#### `public bool Audit(TextLog textLog, bool attemptRepair)`

Audits the contents of the document.

**Parameters:**
- `textLog` (Rhino.FileIO.TextLog) — If an error is detected, then a description of the error is logged here.
- `attemptRepair` (System.Boolean) — If true, then the method attempts to repair any detected errors.

**Returns:** `Boolean` — True if document is valid.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_Audit.htm)

#### `public uint BeginUndoRecord(string description)`

Instructs Rhino to begin recording undo information when the document is changed outside of a command. We use this, e.g., to save changes caused by the modeless layer or object properties dialogs when commands are not running.

**Parameters:**
- `description` (System.String) — A text describing the record.

**Returns:** `UInt32` — Serial number of record. Returns 0 if record is not started because undo information is already being recorded or undo is disabled.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_BeginUndoRecord.htm)

#### `public void ClearRedoRecords()`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_ClearRedoRecords.htm)

#### `public void ClearUndoRecords(bool purgeDeletedObjects)`



**Parameters:**
- `purgeDeletedObjects` (System.Boolean) — [Missing <param name="purgeDeletedObjects"/> documentation for "M:Rhino.RhinoDoc.ClearUndoRecords(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_ClearUndoRecords.htm)

#### `public void ClearUndoRecords(uint undoSerialNumber, bool purgeDeletedObjects)`



**Parameters:**
- `undoSerialNumber` (System.UInt32) — [Missing <param name="undoSerialNumber"/> documentation for "M:Rhino.RhinoDoc.ClearUndoRecords(System.UInt32,System.Boolean)"]
- `purgeDeletedObjects` (System.Boolean) — [Missing <param name="purgeDeletedObjects"/> documentation for "M:Rhino.RhinoDoc.ClearUndoRecords(System.UInt32,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_ClearUndoRecords_1.htm)

#### `public static RhinoDoc Create(string modelTemplateFileName)`

Creates a new RhinoDoc

**Parameters:**
- `modelTemplateFileName` (System.String) — Name of a Rhino model to use as a template to initialize the document. If the template contains views, those views are created. If null, an empty document with no views is created

**Returns:** `RhinoDoc` — [Missing <returns> documentation for "M:Rhino.RhinoDoc.Create(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_Create.htm)

#### `public ObjectAttributes CreateDefaultAttributes()`

Gets the default object attributes for this document. The attributes will be linked to the currently active layer and they will inherit the Document WireDensity setting.

**Returns:** `ObjectAttributes` — [Missing <returns> documentation for "M:Rhino.RhinoDoc.CreateDefaultAttributes"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_CreateDefaultAttributes.htm)

#### `public static RhinoDoc CreateHeadless(string file3dmTemplatePath)`

Create a new headless RhinoDoc from a template file

**Parameters:**
- `file3dmTemplatePath` (System.String) — Name of a Rhino model to use as a template to initialize the document. If null, an empty document is created

**Returns:** `RhinoDoc` — New RhinoDoc on success. Note that this is a "headless" RhinoDoc and it's lifetime is under your control.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_CreateHeadless.htm)

#### `public bool CustomRenderMeshesBoundingBox(MeshType mt, ViewportInfo vp, ref RenderMeshProvider.Flags flags, PlugIn plugin, DisplayPipelineAttributes attrs, out BoundingBox boundingBox)`

Returns the bounding box of custom render primitives for this object .

**Parameters:**
- `mt` (Rhino.Geometry.MeshType) — The mesh type requested (render or analysis).
- `vp` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered
- `flags` (Rhino.Render.CustomRenderMeshes.RenderMeshProvider.Flags) — See MeshProvider.Flags
- `plugin` (Rhino.PlugIns.PlugIn) — The requesting plug-in (typically the calling plugin)
- `attrs` (Rhino.Display.DisplayPipelineAttributes) — Display attributes for the caller - null if this is a full rendering.
- `boundingBox` (Rhino.Geometry.BoundingBox) — The requested bounding box

**Returns:** `Boolean` — True if the process was a success

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_CustomRenderMeshesBoundingBox.htm)

#### `public void Dispose()`

Releases all resources used by the RhinoDoc

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_Dispose.htm)

#### `public bool EndUndoRecord(uint undoRecordSerialNumber)`

Ends the undo record.

**Parameters:**
- `undoRecordSerialNumber` (System.UInt32) — The serial number of the undo record.

**Returns:** `Boolean` — True if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_EndUndoRecord.htm)

#### `public override bool Equals(Object obj)`

(Overrides Object.Equals(Object).)

**Parameters:**
- `obj` (System.Object) — [Missing <param name="obj"/> documentation for "M:Rhino.RhinoDoc.Equals(System.Object)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.RhinoDoc.Equals(System.Object)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_Equals.htm)

#### `public bool Export(string filePath)`

Export the entire document to a file. All file formats that Rhino can export to are supported by this function.

**Parameters:**
- `filePath` (System.String) — [Missing <param name="filePath"/> documentation for "M:Rhino.RhinoDoc.Export(System.String)"]

**Returns:** `Boolean` — true on success

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_Export.htm)

#### `public bool Export(string filePath, ArchivableDictionary options)`

Export the entire document to a file. All file formats that Rhino can export to are supported by this function.

**Parameters:**
- `filePath` (System.String) — [Missing <param name="filePath"/> documentation for "M:Rhino.RhinoDoc.Export(System.String,Rhino.Collections.ArchivableDictionary)"]
- `options` (Rhino.Collections.ArchivableDictionary) — Options to help define how data should be exported.

**Returns:** `Boolean` — true on success

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_Export_1.htm)

#### `public bool ExportSelected(string filePath)`

Export selected geometry to a file. All file formats that Rhino can export to are supported by this function.

**Parameters:**
- `filePath` (System.String) — [Missing <param name="filePath"/> documentation for "M:Rhino.RhinoDoc.ExportSelected(System.String)"]

**Returns:** `Boolean` — true on success

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_ExportSelected.htm)

#### `public bool ExportSelected(string filePath, ArchivableDictionary options)`

Export selected geometry to a file. All file formats that Rhino can export to are supported by this function.

**Parameters:**
- `filePath` (System.String) — [Missing <param name="filePath"/> documentation for "M:Rhino.RhinoDoc.ExportSelected(System.String,Rhino.Collections.ArchivableDictionary)"]
- `options` (Rhino.Collections.ArchivableDictionary) — Options to help define how data should be exported.

**Returns:** `Boolean` — true on success

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_ExportSelected_1.htm)

#### `public static Bitmap ExtractPreviewImage(string path)`

Extracts the bitmap preview image from the specified .3dm file.

**Parameters:**
- `path` (System.String) — The .3dm file from which to extract the preview image. If null, the path to the active document is used.

**Returns:** `Bitmap` — The preview bitmap if successful, null otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_ExtractPreviewImage.htm)

#### `protected override void Finalize()`

(Overrides Object.Finalize().)

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_Finalize.htm)

#### `public string FindFile(string filename)`

Search for a file using Rhino's search path. Rhino will look in the following places: 1. Current model folder 2. Path specified in options dialog/File tab 3. Rhino system folders 4. Rhino executable folder

**Parameters:**
- `filename` (System.String) — [Missing <param name="filename"/> documentation for "M:Rhino.RhinoDoc.FindFile(System.String)"]

**Returns:** `String` — Path to existing file if found, an empty string if no file was found

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_FindFile.htm)

#### `public string FormatNumber(double value)`

Call this method to get string representing the specified value using the documents display coordinate system and display precision.

**Parameters:**
- `value` (System.Double) — [Missing <param name="value"/> documentation for "M:Rhino.RhinoDoc.FormatNumber(System.Double)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.RhinoDoc.FormatNumber(System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_FormatNumber.htm)

#### `public string FormatNumber(double value, bool appendUnitSystemName, bool abbreviate)`

Call this method to get string representing the specified value using the documents display coordinate system and display precision.

**Parameters:**
- `value` (System.Double) — [Missing <param name="value"/> documentation for "M:Rhino.RhinoDoc.FormatNumber(System.Double,System.Boolean,System.Boolean)"]
- `appendUnitSystemName` (System.Boolean) — [Missing <param name="appendUnitSystemName"/> documentation for "M:Rhino.RhinoDoc.FormatNumber(System.Double,System.Boolean,System.Boolean)"]
- `abbreviate` (System.Boolean) — [Missing <param name="abbreviate"/> documentation for "M:Rhino.RhinoDoc.FormatNumber(System.Double,System.Boolean,System.Boolean)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.RhinoDoc.FormatNumber(System.Double,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_FormatNumber_1.htm)

#### `public static RhinoDoc FromFilePath(string filePath)`

Search the open document list for a document with a Path equal to the specified file path.

**Parameters:**
- `filePath` (System.String) — The full path to the file to search for.

**Returns:** `RhinoDoc` — The file name to search for

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_FromFilePath.htm)

#### `[ObsoleteAttribute("Use FromRuntimeSerialNumber")] public static RhinoDoc FromId(int docId)`

Obsolete.

**Parameters:**
- `docId` (System.Int32) — [Missing <param name="docId"/> documentation for "M:Rhino.RhinoDoc.FromId(System.Int32)"]

**Returns:** `RhinoDoc` — [Missing <returns> documentation for "M:Rhino.RhinoDoc.FromId(System.Int32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_FromId.htm)

#### `public static RhinoDoc FromRuntimeSerialNumber(uint serialNumber)`



**Parameters:**
- `serialNumber` (System.UInt32) — [Missing <param name="serialNumber"/> documentation for "M:Rhino.RhinoDoc.FromRuntimeSerialNumber(System.UInt32)"]

**Returns:** `RhinoDoc` — [Missing <returns> documentation for "M:Rhino.RhinoDoc.FromRuntimeSerialNumber(System.UInt32)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_FromRuntimeSerialNumber.htm)

#### `public MeshingParameters GetAnalysisMeshingParameters()`

Get analysis meshing parameters currently used by the document

**Returns:** `MeshingParameters` — [Missing <returns> documentation for "M:Rhino.RhinoDoc.GetAnalysisMeshingParameters"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_GetAnalysisMeshingParameters.htm)

#### `public MeshingParameters GetCurrentMeshingParameters()`

Get the custom meshing parameters that this document will use.

**Returns:** `MeshingParameters` — [Missing <returns> documentation for "M:Rhino.RhinoDoc.GetCurrentMeshingParameters"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_GetCurrentMeshingParameters.htm)

#### `public bool GetCustomUnitSystem(bool modelUnits, out string customUnitName, out double metersPerCustomUnit)`

Get the custom unit system name and custom unit scale.

**Parameters:**
- `modelUnits` (System.Boolean) — Set true to get values from the document's model unit system. Set false to get values from the document's page unit system.
- `customUnitName` (System.String) — The custom unit system name.
- `metersPerCustomUnit` (System.Double) — The meters per custom unit scale.

**Returns:** `Boolean` — true if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_GetCustomUnitSystem.htm)

#### `public string[] GetEmbeddedFilesList(bool missingOnly)`



**Parameters:**
- `missingOnly` (System.Boolean) — [Missing <param name="missingOnly"/> documentation for "M:Rhino.RhinoDoc.GetEmbeddedFilesList(System.Boolean)"]

**Returns:** `String[]` — [Missing <returns> documentation for "M:Rhino.RhinoDoc.GetEmbeddedFilesList(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_GetEmbeddedFilesList.htm)

#### `public ConstructionPlaneGridDefaults GetGridDefaults()`



**Returns:** `ConstructionPlaneGridDefaults` — [Missing <returns> documentation for "M:Rhino.RhinoDoc.GetGridDefaults"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_GetGridDefaults.htm)

#### `public bool GetGumballPlane(out Plane plane)`

Returns the active plane of Rhino's auto-gumball widget. Note, when calling from a Rhino command, make sure the command class has the Rhino.Commands.Style.Transparent command style attribute.

**Parameters:**
- `plane` (Rhino.Geometry.Plane) — The active plane.

**Returns:** `Boolean` — true if the auto-gumball widget is enabled and visible. False otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_GetGumballPlane.htm)

#### `public override int GetHashCode()`

(Overrides Object.GetHashCode().)

**Returns:** `Int32` — [Missing <returns> documentation for "M:Rhino.RhinoDoc.GetHashCode"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_GetHashCode.htm)

#### `public MeshingParameters GetMeshingParameters(MeshingParameterStyle style)`

Get MeshingParameters currently used by the document

**Parameters:**
- `style` (Rhino.Geometry.MeshingParameterStyle) — [Missing <param name="style"/> documentation for "M:Rhino.RhinoDoc.GetMeshingParameters(Rhino.Geometry.MeshingParameterStyle)"]

**Returns:** `MeshingParameters` — [Missing <returns> documentation for "M:Rhino.RhinoDoc.GetMeshingParameters(Rhino.Geometry.MeshingParameterStyle)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_GetMeshingParameters.htm)

#### `[ObsoleteAttribute] public RenderPrimitiveList GetRenderPrimitiveList(ViewportInfo viewport, DisplayPipelineAttributes attrs)`

Build custom render mesh(es) for this document (i.e. - GH meshes).

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `attrs` (Rhino.Display.DisplayPipelineAttributes) — Attributes for the view mode you are supplying meshes for. Will be null if this is a modal rendering.

**Returns:** `RenderPrimitiveList` — Returns a RenderPrimitiveList if successful otherwise returns null.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_GetRenderPrimitiveList.htm)

#### `[ObsoleteAttribute("This version is obsolete because - uses the old school custom render meshes. Prefer CustomRenderMeshes")] public IEnumerable<RenderPrimitive> GetRenderPrimitives(bool forceTriangleMeshes, bool quietly)`

Get a enumerable list of custom mesh primitives

**Parameters:**
- `forceTriangleMeshes` (System.Boolean) — If true all mesh faces will be triangulated
- `quietly` (System.Boolean) — Iterate quietly, if true then no user interface will be displayed

**Returns:** `IEnumerable<RenderPrimitive>` — [Missing <returns> documentation for "M:Rhino.RhinoDoc.GetRenderPrimitives(System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_GetRenderPrimitives_1.htm)

#### `[ObsoleteAttribute("This version is obsolete because - uses the old school custom render meshes. Prefer RenderPrimitives")] public IEnumerable<RenderPrimitive> GetRenderPrimitives(Guid plugInId, ViewportInfo viewport, bool forceTriangleMeshes, bool quietly)`

Get a enumerable list of custom mesh primitives

**Parameters:**
- `plugInId` (System.Guid) — The Id of the plug-in creating the iterator.
- `viewport` (Rhino.DocObjects.ViewportInfo) — The rendering view camera.
- `forceTriangleMeshes` (System.Boolean) — If true all mesh faces will be triangulated
- `quietly` (System.Boolean) — Iterate quietly, if true then no user interface will be displayed

**Returns:** `IEnumerable<RenderPrimitive>` — [Missing <returns> documentation for "M:Rhino.RhinoDoc.GetRenderPrimitives(System.Guid,Rhino.DocObjects.ViewportInfo,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_GetRenderPrimitives_2.htm)

#### `[ObsoleteAttribute("This version is obsolete because - uses the old school custom render meshes. Prefer RenderMeshes")] public IEnumerable<RenderPrimitive> GetRenderPrimitives(ViewportInfo viewport, bool forceTriangleMeshes, bool quietly)`

Get a enumerable list of custom mesh primitives

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The rendering view camera.
- `forceTriangleMeshes` (System.Boolean) — If true all mesh faces will be triangulated
- `quietly` (System.Boolean) — Iterate quietly, if true then no user interface will be displayed

**Returns:** `IEnumerable<RenderPrimitive>` — [Missing <returns> documentation for "M:Rhino.RhinoDoc.GetRenderPrimitives(Rhino.DocObjects.ViewportInfo,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_GetRenderPrimitives.htm)

#### `public string GetUnitSystemName(bool modelUnits, bool capitalize, bool singular, bool abbreviate)`



**Parameters:**
- `modelUnits` (System.Boolean) — [Missing <param name="modelUnits"/> documentation for "M:Rhino.RhinoDoc.GetUnitSystemName(System.Boolean,System.Boolean,System.Boolean,System.Boolean)"]
- `capitalize` (System.Boolean) — [Missing <param name="capitalize"/> documentation for "M:Rhino.RhinoDoc.GetUnitSystemName(System.Boolean,System.Boolean,System.Boolean,System.Boolean)"]
- `singular` (System.Boolean) — [Missing <param name="singular"/> documentation for "M:Rhino.RhinoDoc.GetUnitSystemName(System.Boolean,System.Boolean,System.Boolean,System.Boolean)"]
- `abbreviate` (System.Boolean) — [Missing <param name="abbreviate"/> documentation for "M:Rhino.RhinoDoc.GetUnitSystemName(System.Boolean,System.Boolean,System.Boolean,System.Boolean)"]

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.RhinoDoc.GetUnitSystemName(System.Boolean,System.Boolean,System.Boolean,System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_GetUnitSystemName.htm)

#### `public bool HasCustomRenderMeshes(MeshType mt, ViewportInfo vp, ref RenderMeshProvider.Flags flags, PlugIn plugin, DisplayPipelineAttributes attrs)`

Returns true if the document has a set of custom render primitives - ie, CustomRenderMeshes will return non-null.

**Parameters:**
- `mt` (Rhino.Geometry.MeshType) — The mesh type requested (render or analysis).
- `vp` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `flags` (Rhino.Render.CustomRenderMeshes.RenderMeshProvider.Flags) — See MeshProvider.Flags
- `plugin` (Rhino.PlugIns.PlugIn) — The requesting plug-in (typically the calling plugin)
- `attrs` (Rhino.Display.DisplayPipelineAttributes) — Display attributes for the caller - null if this is a full rendering.

**Returns:** `Boolean` — Returns true if the object will has a set of custom render primitives

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_HasCustomRenderMeshes.htm)

#### `public bool Import(string filePath)`

Import geometry into a RhinoDoc from a file. This can be any file format that Rhino can import

**Parameters:**
- `filePath` (System.String) — [Missing <param name="filePath"/> documentation for "M:Rhino.RhinoDoc.Import(System.String)"]

**Returns:** `Boolean` — true on success

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_Import.htm)

#### `public bool Import(string filePath, ArchivableDictionary options)`

Import geometry into a RhinoDoc from a file. This can be any file format that Rhino can import

**Parameters:**
- `filePath` (System.String) — [Missing <param name="filePath"/> documentation for "M:Rhino.RhinoDoc.Import(System.String,Rhino.Collections.ArchivableDictionary)"]
- `options` (Rhino.Collections.ArchivableDictionary) — [Missing <param name="options"/> documentation for "M:Rhino.RhinoDoc.Import(System.String,Rhino.Collections.ArchivableDictionary)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.RhinoDoc.Import(System.String,Rhino.Collections.ArchivableDictionary)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_Import_1.htm)

#### `public int InCommand(bool bIgnoreScriptRunnerCommands)`

This is a low level tool to determine if Rhino is currently running a command.

**Parameters:**
- `bIgnoreScriptRunnerCommands` (System.Boolean) — If true, script running commands, like "ReadCommandFile" and the RhinoScript plug-ins "RunScript" command, are not counted.

**Returns:** `Int32` — Number of active commands.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_InCommand.htm)

#### `public bool IsMetricUnitSystem(bool modelUnits)`

Determines if a document unit system is a metric unit system.

**Parameters:**
- `modelUnits` (System.Boolean) — True to query model units, false to query page units.

**Returns:** `Boolean` — Return true if the length unit is a metric unit system.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_IsMetricUnitSystem.htm)

#### `public static RhinoDoc Open(string filePath, out bool wasAlreadyOpen)`

Opens a 3dm file and makes it the active document. If called on windows the active document will be saved and closed and the new document will be opened and become the active document. If called on the Mac the file will be opened in a new document window.

**Parameters:**
- `filePath` (System.String) — Full path to the 3dm file to open
- `wasAlreadyOpen` (System.Boolean) — Will get set to true if there is a currently open document with the specified path; otherwise it will get set to false.

**Returns:** `RhinoDoc` — Returns the newly opened document on success or null on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_Open.htm)

#### `public static RhinoDoc[] OpenDocuments()`

Returns a list of currently open Rhino documents

**Returns:** `RhinoDoc[]` — [Missing <returns> documentation for "M:Rhino.RhinoDoc.OpenDocuments"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_OpenDocuments.htm)

#### `public static RhinoDoc[] OpenDocuments(bool includeHeadless)`

Returns a list of currently open Rhino documents

**Parameters:**
- `includeHeadless` (System.Boolean) — pass true to include headless docs in the list

**Returns:** `RhinoDoc[]` — [Missing <returns> documentation for "M:Rhino.RhinoDoc.OpenDocuments(System.Boolean)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_OpenDocuments_1.htm)

#### `[ObsoleteAttribute("OpenFile is obsolete, use Open instead")] public static bool OpenFile(string path)`

Obsolete.

**Parameters:**
- `path` (System.String) — [Missing <param name="path"/> documentation for "M:Rhino.RhinoDoc.OpenFile(System.String)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.RhinoDoc.OpenFile(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_OpenFile.htm)

#### `public static RhinoDoc OpenHeadless(string file3dmPath)`

Opens a 3DM file into a new headless RhinoDoc.

**Parameters:**
- `file3dmPath` (System.String) — Path of a Rhino model to load.

**Returns:** `RhinoDoc` — [Missing <returns> documentation for "M:Rhino.RhinoDoc.OpenHeadless(System.String)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_OpenHeadless.htm)

#### `public static RhinoDoc OpenHeadless(string filePath, ArchivableDictionary options)`

Opens a file into a new headless RhinoDoc.

**Parameters:**
- `filePath` (System.String) — Path of a Rhino model to load.
- `options` (Rhino.Collections.ArchivableDictionary) — [Missing <param name="options"/> documentation for "M:Rhino.RhinoDoc.OpenHeadless(System.String,Rhino.Collections.ArchivableDictionary)"]

**Returns:** `RhinoDoc` — [Missing <returns> documentation for "M:Rhino.RhinoDoc.OpenHeadless(System.String,Rhino.Collections.ArchivableDictionary)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_OpenHeadless_1.htm)

#### `public static bool ReadFile(string path, FileReadOptions options)`



**Parameters:**
- `path` (System.String) — [Missing <param name="path"/> documentation for "M:Rhino.RhinoDoc.ReadFile(System.String,Rhino.FileIO.FileReadOptions)"]
- `options` (Rhino.FileIO.FileReadOptions) — [Missing <param name="options"/> documentation for "M:Rhino.RhinoDoc.ReadFile(System.String,Rhino.FileIO.FileReadOptions)"]

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.RhinoDoc.ReadFile(System.String,Rhino.FileIO.FileReadOptions)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_ReadFile.htm)

#### `public int ReadFileVersion()`

Returns the file version of the current document. Use this function to determine which version of Rhino last saved the document.

**Returns:** `Int32` — The file version (e.g. 1, 2, 3, 4, etc.) or -1 if the document has not been read from disk.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_ReadFileVersion.htm)

#### `public bool Redo()`

Redo the last action that was "undone"

**Returns:** `Boolean` — true on success

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_Redo.htm)

#### `public RenderMeshes[] RenderMeshes(MeshType mt, ViewportInfo vp, ref RenderMeshProvider.Flags flags, PlugIn plugin, DisplayPipelineAttributes attrs)`

Returns a set of non-object custom render primitives for this document.

**Parameters:**
- `mt` (Rhino.Geometry.MeshType) — The mesh type requested (render or analysis).
- `vp` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered
- `flags` (Rhino.Render.CustomRenderMeshes.RenderMeshProvider.Flags) — See MeshProvider.Flags
- `plugin` (Rhino.PlugIns.PlugIn) — The requesting plug-in (typically the calling plugin)
- `attrs` (Rhino.Display.DisplayPipelineAttributes) — Display attributes for the caller - null if this is a full rendering.

**Returns:** `RenderMeshes[]` — Returns a set of custom render primitives for this object

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_RenderMeshes.htm)

#### `public bool Save()`

Save doc to disk using the document's Path

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.RhinoDoc.Save"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_Save.htm)

#### `public bool SaveAs(string file3dmPath)`

Save doc as a 3dm to a specified path using the current Rhino file version

**Parameters:**
- `file3dmPath` (System.String) — [Missing <param name="file3dmPath"/> documentation for "M:Rhino.RhinoDoc.SaveAs(System.String)"]

**Returns:** `Boolean` — true on success

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_SaveAs.htm)

#### `public bool SaveAs(string file3dmPath, int version)`

Save doc as a 3dm to a specified path

**Parameters:**
- `file3dmPath` (System.String) — [Missing <param name="file3dmPath"/> documentation for "M:Rhino.RhinoDoc.SaveAs(System.String,System.Int32)"]
- `version` (System.Int32) — Rhino file version

**Returns:** `Boolean` — true on success

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_SaveAs_1.htm)

#### `public bool SaveAs(string file3dmPath, int version, bool saveSmall, bool saveTextures, bool saveGeometryOnly, bool savePluginData)`

Save doc as a 3dm to a specified path

**Parameters:**
- `file3dmPath` (System.String) — [Missing <param name="file3dmPath"/> documentation for "M:Rhino.RhinoDoc.SaveAs(System.String,System.Int32,System.Boolean,System.Boolean,System.Boolean,System.Boolean)"]
- `version` (System.Int32) — Rhino file version
- `saveSmall` (System.Boolean) — whether to inlcude render meshes and preview image
- `saveTextures` (System.Boolean) — whether to include the bitmap table
- `saveGeometryOnly` (System.Boolean) — whether to write enything besides geometry
- `savePluginData` (System.Boolean) — whether to write plugin user data

**Returns:** `Boolean` — true on success

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_SaveAs_2.htm)

#### `public bool SaveAsTemplate(string file3dmTemplatePath)`

Save this document as a template

**Parameters:**
- `file3dmTemplatePath` (System.String) — [Missing <param name="file3dmTemplatePath"/> documentation for "M:Rhino.RhinoDoc.SaveAsTemplate(System.String)"]

**Returns:** `Boolean` — true on success

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_SaveAsTemplate.htm)

#### `public bool SaveAsTemplate(string file3dmTemplatePath, int version)`

Save this document as a template to a specific Rhino file version

**Parameters:**
- `file3dmTemplatePath` (System.String) — [Missing <param name="file3dmTemplatePath"/> documentation for "M:Rhino.RhinoDoc.SaveAsTemplate(System.String,System.Int32)"]
- `version` (System.Int32) — [Missing <param name="version"/> documentation for "M:Rhino.RhinoDoc.SaveAsTemplate(System.String,System.Int32)"]

**Returns:** `Boolean` — true on success

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_SaveAsTemplate_1.htm)

#### `public void SelectRenderContentInEditor(RenderContentCollection collection, bool append)`

Selects a collection of contents in any editors they appear in.

**Parameters:**
- `collection` (Rhino.Render.RenderContentCollection) — A collection of RenderContents to select
- `append` (System.Boolean) — Append to current selection

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_SelectRenderContentInEditor.htm)

#### `public void SetCustomMeshingParameters(MeshingParameters mp)`

Set the custom meshing parameters that this document will use. You must also modify the MeshingParameterStyle property on the document to Custom if you want these meshing parameters to be used

**Parameters:**
- `mp` (Rhino.Geometry.MeshingParameters) — [Missing <param name="mp"/> documentation for "M:Rhino.RhinoDoc.SetCustomMeshingParameters(Rhino.Geometry.MeshingParameters)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_SetCustomMeshingParameters.htm)

#### `public bool SetCustomUnitSystem(bool modelUnits, string customUnitName, double metersPerCustomUnit, bool scale)`

Changes the unit system to custom units and sets the custom unit scale.

**Parameters:**
- `modelUnits` (System.Boolean) — Set true to set values from the document's model unit system. Set false to set values from the document's page unit system.
- `customUnitName` (System.String) — The custom unit system name.
- `metersPerCustomUnit` (System.Double) — The meters per custom unit scale.
- `scale` (System.Boolean) — Set true to scale existing objects.

**Returns:** `Boolean` — true if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_SetCustomUnitSystem.htm)

#### `public void SetGridDefaults(ConstructionPlaneGridDefaults defaults)`



**Parameters:**
- `defaults` (Rhino.DocObjects.ConstructionPlaneGridDefaults) — [Missing <param name="defaults"/> documentation for "M:Rhino.RhinoDoc.SetGridDefaults(Rhino.DocObjects.ConstructionPlaneGridDefaults)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_SetGridDefaults.htm)

#### `[ObsoleteAttribute] public bool SupportsRenderPrimitiveList(ViewportInfo viewport, DisplayPipelineAttributes attrs)`

Determines if custom render meshes will be built for this document (i.e. - GH meshes).

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `attrs` (Rhino.Display.DisplayPipelineAttributes) — Type of mesh to build. If attrs is non-null then a smaller mesh may be generated in less time, false is meant when actually rendering.

**Returns:** `Boolean` — Returns true if custom render mesh(es) will get built for this document.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_SupportsRenderPrimitiveList.htm)

#### `[ObsoleteAttribute] public bool TryGetRenderPrimitiveBoundingBox(ViewportInfo viewport, DisplayPipelineAttributes attrs, out BoundingBox boundingBox)`

Get the bounding box for the custom render meshes associated with this document (i.e. - GH meshes).

**Parameters:**
- `viewport` (Rhino.DocObjects.ViewportInfo) — The viewport being rendered.
- `attrs` (Rhino.Display.DisplayPipelineAttributes) — Attributes for the view mode you are supplying meshes for. Will be null if this is a modal rendering.
- `boundingBox` (Rhino.Geometry.BoundingBox) — This will be set to BoundingBox.Unset on failure otherwise it will be the bounding box for the custom render meshes associated with this object.

**Returns:** `Boolean` — Returns true if the bounding box was successfully calculated otherwise returns false on error.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_TryGetRenderPrimitiveBoundingBox.htm)

#### `public bool Undo()`

Undo the last action

**Returns:** `Boolean` — true on success

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_Undo.htm)

#### `public bool Write3dmFile(string path, FileWriteOptions options)`

Write information in this document to a .3dm file. Note, the active document's name will not be changed.

**Parameters:**
- `path` (System.String) — The name of the .3dm file to write.
- `options` (Rhino.FileIO.FileWriteOptions) — The file writing options.

**Returns:** `Boolean` — true if successful, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_Write3dmFile.htm)

#### `public bool WriteFile(string path, FileWriteOptions options)`

Write information in this document to a file. Note, the active document's name will be changed to that of the path provided.

**Remarks:** This is the best choice for general file writing. It handles making backups using temporary files, locking and unlocking, loading file writing plug-ins, and many other details.

**Parameters:**
- `path` (System.String) — The name of the file to write.
- `options` (Rhino.FileIO.FileWriteOptions) — The file writing options.

**Returns:** `Boolean` — true if successful, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDoc_WriteFile.htm)

### Properties
- `ActiveCommandId` (Guid, get) — Get the ID of the active command.
- `ActiveDoc` (RhinoDoc, get/set) — WARNING!! Do not use the ActiveDoc if you don't have to. Under Mac Rhino the ActiveDoc can change while a command is running. Use the doc that is passed to you in your RunCommand function or continue to use the same doc after the first call to ActiveDoc.
- `ActiveSpace` (ActiveSpace, get) — Get the space associated with the active viewport for this document
- `AnimationProperties` (AnimationProperties, get/set) — 
- `Bitmaps` (BitmapTable, get) — bitmaps used in textures, backgrounds, wallpapers, ...
- `CurrentEnvironment` (ICurrentEnvironment, get) — Access to the current environment for various uses
- `CurrentUndoRecordSerialNumber` (UInt32, get) — >0: undo recording is active and being saved on the undo record with the specified serial number. 0: undo recording is not active. (Disabled or nothing is being recorded.)
- `CustomRenderSizes` (List<Size>, get) — 
- `DateCreated` (DateTime, get) — 
- `DateLastEdited` (DateTime, get) — Returns the date the document was created in Coordinated Universal Time (UTC). Use DateTime.ToLocalTime to convert to local time.
- `DimStyles` (DimStyleTable, get) — 
- `DistanceDisplayPrecision` (Int32, get) — 
- `EarthAnchorPoint` (EarthAnchorPoint, get/set) — 
- `Fonts` (FontTable, get) — Obsolete.
- `GroundPlane` (GroundPlane, get) — Gets the ground plane of this document.
- `Groups` (GroupTable, get) — 
- `HatchPatterns` (HatchPatternTable, get) — 
- `InGetPoint` (Boolean, get) — Returns true if currently in a GetPoint.Get().
- `InstanceDefinitions` (InstanceDefinitionTable, get) — 
- `IsAvailable` (Boolean, get) — 
- `IsClosing` (Boolean, get) — 
- `IsCommandRunning` (Boolean, get) — Returns true if Rhino is currently running a command.
- `IsCreating` (Boolean, get) — 
- `IsHeadless` (Boolean, get) — 
- `IsInitializing` (Boolean, get) — 
- `IsLocked` (Boolean, get) — Check to see if the file associated with this document is locked. If it is locked then this is the only document that will be able to write the file. Other instances of Rhino will fail to write this document.
- `IsOpening` (Boolean, get) — 
- `IsReadOnly` (Boolean, get) — Current read-only mode for this document. true if the document can be viewed but NOT saved. false if document can be viewed and saved.
- `IsSendingMail` (Boolean, get) — true if Rhino is in the process of sending this document as an email attachment.
- `Layers` (LayerTable, get) — Layers in the document.
- `LayoutSpaceAnnotationScalingEnabled` (Boolean, get/set) — If LayoutSpaceAnnotationScaling is on, sizes in dimstyles are multiplied by dimscale when the annotation is displayed in a detail viewport not in a detail
- `Lights` (LightTable, get) — 
- `Linetypes` (LinetypeTable, get) — Linetypes in the document.
- `LinkedInstanceDefinitionUpdate` (LinkedInstanceDefinitionUpdateStyle, get/set) — Controls if, when, and how linked and linked and embedded instance definitions are updated when the source archive that was used to create the instance definition has changed.
- `Manifest` (ManifestTable, get) — 
- `Materials` (MaterialTable, get) — Materials in the document.
- `MeshingParameterStyle` (MeshingParameterStyle, get/set) — Type of MeshingParameters currently used by the document to mesh objects
- `ModelAbsoluteTolerance` (Double, get/set) — Model space absolute tolerance.
- `ModelAngleToleranceDegrees` (Double, get/set) — Model space angle tolerance.
- `ModelAngleToleranceRadians` (Double, get/set) — Model space angle tolerance.
- `ModelBasepoint` (Point3d, get/set) — The base point in the model that is used when inserting the model into another as a block definition. By default the base point in any model is 0,0,0.
- `ModelDistanceDisplayPrecision` (Int32, get/set) — 
- `ModelRelativeTolerance` (Double, get/set) — Model space relative tolerance.
- `ModelSpaceAnnotationScalingEnabled` (Boolean, get/set) — If ModelSpaceAnnotationScaling is on, sizes in dimstyles are multiplied by dimscale when the annotation is displayed in a model space viewport not in a detail
- `ModelSpaceHatchScale` (Double, get/set) — The scale factor for hatches in model space when Hatch Scaling is enabled
- `ModelSpaceHatchScalingEnabled` (Boolean, get/set) — True if hatch scaling is enabled, false if not.
- `ModelSpaceTextScale` (Double, get/set) — The scale factor for text in model space when Annotation Scaling is enabled
- `ModelUnitSystem` (UnitSystem, get/set) — 
- `Modified` (Boolean, get/set) — Returns or sets the document's modified flag.
- `Name` (String, get) — Returns the name of the currently loaded Rhino document (3DM file).
- `NamedConstructionPlanes` (NamedConstructionPlaneTable, get) — Collection of named construction planes.
- `NamedLayerStates` (NamedLayerStateTable, get) — Collection of named layer states.
- `NamedPositions` (NamedPositionTable, get) — Collection of named positions.
- `NamedViews` (NamedViewTable, get) — Collection of named views.
- `NextUndoRecordSerialNumber` (UInt32, get) — The serial number that will be assigned to the next undo record that is constructed.
- `Notes` (String, get/set) — Returns or sets the document's notes.
- `NotesLocked` (Boolean, get/set) — 
- `Objects` (ObjectTable, get) — 
- `PageAbsoluteTolerance` (Double, get/set) — Page space absolute tolerance.
- `PageAngleToleranceDegrees` (Double, get/set) — Page space angle tolerance.
- `PageAngleToleranceRadians` (Double, get/set) — Page space angle tolerance.
- `PageDistanceDisplayPrecision` (Int32, get/set) — 
- `PageRelativeTolerance` (Double, get/set) — Page space relative tolerance.
- `PageUnitSystem` (UnitSystem, get/set) — 
- `Path` (String, get) — Returns the path of the currently loaded Rhino document (3DM file).
- `PostEffects` (IPostEffects, get) — Access to the post effects
- `RedoActive` (Boolean, get) — Returns true if Redo is currently active.
- `RenderEnvironments` (RenderEnvironmentTable, get) — 
- `RenderMaterials` (RenderMaterialTable, get) — 
- `RenderSettings` (RenderSettings, get/set) — 
- `RenderTextures` (RenderTextureTable, get) — 
- `RuntimeData` (RuntimeDocumentDataTable, get) — Collection of document runtime data. This is a good place to put non-serializable, per document data, such as panel view models. Note well: This data will be dispose with the document and does not get serialized.
- `RuntimeSerialNumber` (UInt32, get) — Unique serialNumber for the document while the application is running. This is not a persistent value.
- `Snapshots` (SnapshotTable, get) — Collection of snapshots.
- `Strings` (StringTable, get) — Collection of document user data strings in this document
- `SubDAppearance` (SubDComponentLocation, get/set) — Returns or sets the appearance of all SubD objects in the document.
- `TemplateFileUsed` (String, get) — name of the template file used to create this document. This is a runtime value only present if the document was newly created.
- `UndoActive` (Boolean, get) — Returns true if Undo is currently active.
- `UndoRecordingEnabled` (Boolean, get/set) — 
- `UndoRecordingIsActive` (Boolean, get) — true if undo recording is actually happening now.
- `Views` (ViewTable, get) — 
- `Worksession` (Worksession, get) — Provides access to the document's worksession.

### Events
#### `ActiveDocumentChanged` (`System.EventHandler<DocumentEventArgs>`)

**Signature:** `public static event EventHandler<DocumentEventArgs> ActiveDocumentChanged`

This event is raised when the active document used by modeless user interface changes. On Mac Rhino this will get raised before the NewDocument, BeginOpenDocument and EndOpenDocument events. Mac Rhino will also raise this event with 0 for the document Id and a null document pointer when the last document is closed. Windows Rhino will raise this event after the NewDocument, BeginOpenDocument and EndOpenDocument events when a new or existing model is opened.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_ActiveDocumentChanged.htm)

#### `AddRhinoObject` (`System.EventHandler<RhinoObjectEventArgs>`)

**Signature:** `public static event EventHandler<RhinoObjectEventArgs> AddRhinoObject`

Called if a new object is added to the document.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_AddRhinoObject.htm)

#### `AfterTransformObjects` (`System.EventHandler<RhinoAfterTransformObjectsEventArgs>`)

**Signature:** `public static event EventHandler<RhinoAfterTransformObjectsEventArgs> AfterTransformObjects`

Called after objects are being transformed

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_AfterTransformObjects.htm)

#### `BeforeTransformObjects` (`System.EventHandler<RhinoTransformObjectsEventArgs>`)

**Signature:** `public static event EventHandler<RhinoTransformObjectsEventArgs> BeforeTransformObjects`

Called before objects are being transformed

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_BeforeTransformObjects.htm)

#### `BeginOpenDocument` (`System.EventHandler<DocumentOpenEventArgs>`)

**Signature:** `public static event EventHandler<DocumentOpenEventArgs> BeginOpenDocument`

This event is raised when the document open operation begins. NOTE: On Windows, this event will be fired when a clipboard paste operation occurs, as Rhino opens a .tmp file in the User's Local folder with the contents of the pasted document.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_BeginOpenDocument.htm)

#### `BeginSaveDocument` (`System.EventHandler<DocumentSaveEventArgs>`)

**Signature:** `public static event EventHandler<DocumentSaveEventArgs> BeginSaveDocument`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_BeginSaveDocument.htm)

#### `CloseDocument` (`System.EventHandler<DocumentEventArgs>`)

**Signature:** `public static event EventHandler<DocumentEventArgs> CloseDocument`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_CloseDocument.htm)

#### `DeleteRhinoObject` (`System.EventHandler<RhinoObjectEventArgs>`)

**Signature:** `public static event EventHandler<RhinoObjectEventArgs> DeleteRhinoObject`

Called if an object is deleted. At some later point the object can be un-deleted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_DeleteRhinoObject.htm)

#### `DeselectAllObjects` (`System.EventHandler<RhinoDeselectAllObjectsEventArgs>`)

**Signature:** `public static event EventHandler<RhinoDeselectAllObjectsEventArgs> DeselectAllObjects`

Called when all objects are deselected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_DeselectAllObjects.htm)

#### `DeselectObjects` (`System.EventHandler<RhinoObjectSelectionEventArgs>`)

**Signature:** `public static event EventHandler<RhinoObjectSelectionEventArgs> DeselectObjects`

Called when object(s) are deselected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_DeselectObjects.htm)

#### `DimensionStyleTableEvent` (`System.EventHandler<DimStyleTableEventArgs>`)

**Signature:** `public static event EventHandler<DimStyleTableEventArgs> DimensionStyleTableEvent`

Called when any modification happens to a document's dimension style table.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_DimensionStyleTableEvent.htm)

#### `DocumentPropertiesChanged` (`System.EventHandler<DocumentEventArgs>`)

**Signature:** `public static event EventHandler<DocumentEventArgs> DocumentPropertiesChanged`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_DocumentPropertiesChanged.htm)

#### `EndOpenDocument` (`System.EventHandler<DocumentOpenEventArgs>`)

**Signature:** `public static event EventHandler<DocumentOpenEventArgs> EndOpenDocument`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_EndOpenDocument.htm)

#### `EndOpenDocumentInitialViewUpdate` (`System.EventHandler<DocumentOpenEventArgs>`)

**Signature:** `public static event EventHandler<DocumentOpenEventArgs> EndOpenDocumentInitialViewUpdate`

This event is raised after EndOpenDocument when the documents initial views have been created and initialized.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_EndOpenDocumentInitialViewUpdate.htm)

#### `EndSaveDocument` (`System.EventHandler<DocumentSaveEventArgs>`)

**Signature:** `public static event EventHandler<DocumentSaveEventArgs> EndSaveDocument`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_EndSaveDocument.htm)

#### `GroupTableEvent` (`System.EventHandler<GroupTableEventArgs>`)

**Signature:** `public static event EventHandler<GroupTableEventArgs> GroupTableEvent`

Called when any modification happens to a document's group table.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_GroupTableEvent.htm)

#### `HatchPatternTableEvent` (`System.EventHandler<HatchPatternTableEventArgs>`)

**Signature:** `public static event EventHandler<HatchPatternTableEventArgs> HatchPatternTableEvent`

Called when any modification happens to a document's hatch pattern table.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_HatchPatternTableEvent.htm)

#### `InstanceDefinitionTableEvent` (`System.EventHandler<InstanceDefinitionTableEventArgs>`)

**Signature:** `public static event EventHandler<InstanceDefinitionTableEventArgs> InstanceDefinitionTableEvent`

Called when any modification happens to a document's instance definition table.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_InstanceDefinitionTableEvent.htm)

#### `LayerTableEvent` (`System.EventHandler<LayerTableEventArgs>`)

**Signature:** `public static event EventHandler<LayerTableEventArgs> LayerTableEvent`

Called when any modification happens to a document's layer table.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_LayerTableEvent.htm)

#### `LightTableEvent` (`System.EventHandler<LightTableEventArgs>`)

**Signature:** `public static event EventHandler<LightTableEventArgs> LightTableEvent`

Called when any modification happens to a document's light table.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_LightTableEvent.htm)

#### `LinetypeTableEvent` (`System.EventHandler<LinetypeTableEventArgs>`)

**Signature:** `public static event EventHandler<LinetypeTableEventArgs> LinetypeTableEvent`

Called when any modification happens to a document's linetype table.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_LinetypeTableEvent.htm)

#### `MaterialTableEvent` (`System.EventHandler<MaterialTableEventArgs>`)

**Signature:** `public static event EventHandler<MaterialTableEventArgs> MaterialTableEvent`

Called when any modification happens to a document's material table.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_MaterialTableEvent.htm)

#### `ModifyObjectAttributes` (`System.EventHandler<RhinoModifyObjectAttributesEventArgs>`)

**Signature:** `public static event EventHandler<RhinoModifyObjectAttributesEventArgs> ModifyObjectAttributes`

Called when all object attributes are changed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_ModifyObjectAttributes.htm)

#### `NewDocument` (`System.EventHandler<DocumentEventArgs>`)

**Signature:** `public static event EventHandler<DocumentEventArgs> NewDocument`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_NewDocument.htm)

#### `PurgeRhinoObject` (`System.EventHandler<RhinoObjectEventArgs>`)

**Signature:** `public static event EventHandler<RhinoObjectEventArgs> PurgeRhinoObject`

Called if an object is being purged from a document. The object will cease to exist forever.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_PurgeRhinoObject.htm)

#### `RenderEnvironmentTableEvent` (`System.EventHandler<RhinoDoc.RenderContentTableEventArgs>`)

**Signature:** `public static event EventHandler<RhinoDoc.RenderContentTableEventArgs> RenderEnvironmentTableEvent`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_RenderEnvironmentTableEvent.htm)

#### `RenderMaterialsTableEvent` (`System.EventHandler<RhinoDoc.RenderContentTableEventArgs>`)

**Signature:** `public static event EventHandler<RhinoDoc.RenderContentTableEventArgs> RenderMaterialsTableEvent`



[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_RenderMaterialsTableEvent.htm)

#### `RenderTextureTableEvent` (`System.EventHandler<RhinoDoc.RenderContentTableEventArgs>`)

**Signature:** `public static event EventHandler<RhinoDoc.RenderContentTableEventArgs> RenderTextureTableEvent`

Called when the RenderTextureTable has been loaded, is about to be cleared or has been cleared. See RhinoDoc.RenderContentTableEventType for more information.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_RenderTextureTableEvent.htm)

#### `ReplaceRhinoObject` (`System.EventHandler<RhinoReplaceObjectEventArgs>`)

**Signature:** `public static event EventHandler<RhinoReplaceObjectEventArgs> ReplaceRhinoObject`

Called if an object is about to be replaced. If both RhinoDoc.UndoActive() and RhinoDoc.RedoActive() return false, then immediately after the ReplaceObject event, there will be a DeleteObject event followed by an AddObject event. If either RhinoDoc.UndoActive() or RhinoDoc::RedoActive() return true, then immediately after the ReplaceObject event, there will be a DeleteObject event followed by an UndeleteObject event.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_ReplaceRhinoObject.htm)

#### `SelectObjects` (`System.EventHandler<RhinoObjectSelectionEventArgs>`)

**Signature:** `public static event EventHandler<RhinoObjectSelectionEventArgs> SelectObjects`

Called when object(s) are selected.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_SelectObjects.htm)

#### `TextureMappingEvent` (`System.EventHandler<RhinoDoc.TextureMappingEventArgs>`)

**Signature:** `public static event EventHandler<RhinoDoc.TextureMappingEventArgs> TextureMappingEvent`

Called when any modification happens to a document objects texture mapping.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_TextureMappingEvent.htm)

#### `UndeleteRhinoObject` (`System.EventHandler<RhinoObjectEventArgs>`)

**Signature:** `public static event EventHandler<RhinoObjectEventArgs> UndeleteRhinoObject`

Called if an object is un-deleted.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_UndeleteRhinoObject.htm)

#### `UnitsChangedWithScaling` (`System.EventHandler<UnitsChangedWithScalingEventArgs>`)

**Signature:** `public static event EventHandler<UnitsChangedWithScalingEventArgs> UnitsChangedWithScaling`

Called when a change in the model units results in a scaling operation on all of the objects in the document. This call is made before any of the objects are scaled. A call to RhinoDoc.DocumentPropertiesChanged follows.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_UnitsChangedWithScaling.htm)

#### `UserStringChanged` (`System.EventHandler<RhinoDoc.UserStringChangedArgs>`)

**Signature:** `public static event EventHandler<RhinoDoc.UserStringChangedArgs> UserStringChanged`

This event is raised when document user text strings are changed

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_RhinoDoc_UserStringChanged.htm)

## RhinoDoc.RenderContentTableEventArgs (class)

Passed to the RenderMaterialsTableEvent, RenderEnvironmentTableEvent and the RenderTextureTableEvent events.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_RhinoDoc_RenderContentTableEventArgs.htm)

### Properties
- `Document` (RhinoDoc, get) — Document the table belongs to
- `EventType` (RhinoDoc.RenderContentTableEventType, get) — Event type

## RhinoDoc.RenderContentTableEventType (enum)

Type of content table event

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_RhinoDoc_RenderContentTableEventType.htm)

### Values
- `Loaded` = `0` — The document has been read and the table has been loaded
- `Clearing` = `1` — The table is about to be cleared
- `Cleared` = `2` — The table has been cleared
- `MaterialAssignmentChanged` = `3` — Object or layer material assignment changed

## RhinoDoc.RenderMaterialAssignmentChangedEventArgs (class)

[Missing <summary> documentation for "T:Rhino.RhinoDoc.RenderMaterialAssignmentChangedEventArgs"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_RhinoDoc_RenderMaterialAssignmentChangedEventArgs.htm)

### Properties
- `Document` (RhinoDoc, get) — Document the table belongs to
- `EventType` (RhinoDoc.RenderContentTableEventType, get) — Event type
- `IsLayer` (Boolean, get) — 
- `IsObject` (Boolean, get) — 
- `LayerId` (Guid, get) — 
- `NewRenderMaterial` (Guid, get) — 
- `ObjectId` (Guid, get) — 
- `OldRenderMaterial` (Guid, get) — 

## RhinoDoc.TextureMappingEventArgs (class)

Event arguments passed to the RhinoDoc.TextureMappingEvent.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_RhinoDoc_TextureMappingEventArgs.htm)

### Properties
- `Document` (RhinoDoc, get) — 
- `EventType` (RhinoDoc.TextureMappingEventType, get) — 
- `NewMapping` (TextureMapping, get) — 
- `OldMapping` (TextureMapping, get) — 

## RhinoDoc.TextureMappingEventType (enum)

[Missing <summary> documentation for "T:Rhino.RhinoDoc.TextureMappingEventType"]

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_RhinoDoc_TextureMappingEventType.htm)

### Values
- `Added` = `0` — Adding texture mapping to a document
- `Deleted` = `1` — A texture mapping was deleted from a document
- `Undeleted` = `2` — A texture mapping was undeleted in a document
- `Modified` = `3` — A texture mapping was modified in a document

## RhinoDoc.UserStringChangedArgs (class)

This event is raised when document user text strings are changed

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_RhinoDoc_UserStringChangedArgs.htm)

### Properties
- `Document` (RhinoDoc, get) — Document containing the user string
- `Key` (String, get) — Key for the string being changed

## RhinoDocObserverArgs (class)

Arguments passed to IRhinoDocObserver methods.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_RhinoDocObserverArgs.htm)

### Constructors
- `public RhinoDocObserverArgs(RhinoDoc doc)` — Initializes a new instance of the RhinoDocObserverArgs class

### Properties
- `Doc` (RhinoDoc, get) — Document
- `RuntimeSerialNumber` (UInt32, get) — Document runtime serial number, will be different across Rhino sessions.

## RhinoDocUndoRecord (class)

This helper class manages calls to RhinoDoc.BeginUndoRecord and RhinoDoc.UndoRecord. Use this if you are modifying a document from some modeless user interface. Do not use this if you are just writing Rhino commands. ALWAYS call Dispose on this class. It is best if the class is placed in a using block so Dispose will be called for you.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_RhinoDocUndoRecord.htm)

### Constructors
- `public RhinoDocUndoRecord(RhinoDoc doc, string description)` — Begin a RhinoDoc undo record.
- `public RhinoDocUndoRecord(uint docRuntimeSerialNumber, string description)` — Begin a RhinoDoc undo record.

### Methods
#### `public void Dispose()`

IDisposable implementaton

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoDocUndoRecord_Dispose.htm)

## RhinoFileWatcherChangeReason (enum)

Passed to IRhinoFileEventWatcher::Changed when a change event is raised.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_RhinoFileWatcherChangeReason.htm)

### Values
- `Created` = `1` — The creation of a file or folder.
- `Deleted` = `2` — The deletion of a file or folder.
- `Changed` = `4` — The change of a file or folder. The types of changes include: changes to size, attributes, security settings, last write, and last access time.
- `Renamed` = `5` — The renaming of a file or folder.

## RhinoMath (class)

Provides constants and static methods that are additional to Math.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_RhinoMath.htm)

### Methods
#### `public static uint CRC32(uint currentRemainder, byte[] buffer)`

Advances the cyclic redundancy check value remainder given a byte array. http://en.wikipedia.org/wiki/Cyclic_redundancy_check.

**Parameters:**
- `currentRemainder` (System.UInt32) — The remainder from which to start.
- `buffer` (System.Byte[]) — The value to add to the current remainder.

**Returns:** `UInt32` — The new current remainder.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoMath_CRC32.htm)

#### `public static uint CRC32(uint currentRemainder, double value)`

Advances the cyclic redundancy check value remainder given a Double. http://en.wikipedia.org/wiki/Cyclic_redundancy_check.

**Parameters:**
- `currentRemainder` (System.UInt32) — The remainder from which to start.
- `value` (System.Double) — The value to add to the current remainder.

**Returns:** `UInt32` — The new current remainder.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoMath_CRC32_1.htm)

#### `public static uint CRC32(uint currentRemainder, int value)`

Advances the cyclic redundancy check value remainder given a Int32. http://en.wikipedia.org/wiki/Cyclic_redundancy_check.

**Parameters:**
- `currentRemainder` (System.UInt32) — The remainder from which to start.
- `value` (System.Int32) — The value to add to the current remainder.

**Returns:** `UInt32` — The new current remainder.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoMath_CRC32_2.htm)

#### `public static double Clamp(double value, double bound1, double bound2)`

Limits a Double to be specified within an interval of two numbers, by specifying a fixed minimum and maximum.

**Parameters:**
- `value` (System.Double) — A number.
- `bound1` (System.Double) — A first bound.
- `bound2` (System.Double) — A second bound. This does not necessarily need to be larger or smaller than bound1.

**Returns:** `Double` — The clamped value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoMath_Clamp.htm)

#### `public static int Clamp(int value, int bound1, int bound2)`

Restricts a Int32 to be specified within an interval of two integers.

**Parameters:**
- `value` (System.Int32) — An integer.
- `bound1` (System.Int32) — A first bound.
- `bound2` (System.Int32) — A second bound. This does not necessarily need to be larger or smaller than bound1.

**Returns:** `Int32` — The clamped value.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoMath_Clamp_1.htm)

#### `public static bool EpsilonEquals(double x, double y, double epsilon)`

Compare two doubles for equality within some "epsilon" range

**Parameters:**
- `x` (System.Double)
- `y` (System.Double)
- `epsilon` (System.Double)

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.RhinoMath.EpsilonEquals(System.Double,System.Double,System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoMath_EpsilonEquals.htm)

#### `public static bool EpsilonEquals(float x, float y, float epsilon)`

Compare to floats for equality within some "epsilon" range

**Parameters:**
- `x` (System.Single)
- `y` (System.Single)
- `epsilon` (System.Single)

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.RhinoMath.EpsilonEquals(System.Single,System.Single,System.Single)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoMath_EpsilonEquals_1.htm)

#### `public static bool EvaluateNormal(int limitDirection, Vector3d ds, Vector3d dt, Vector3d dss, Vector3d dst, Vector3d dtt, out Vector3d n)`

Expert tool to evaluate surface unit normal.

**Parameters:**
- `limitDirection` (System.Int32) — Determines which direction is used to compute the limit, where: 0 = default, 1 = from quadrant I, 2 = from quadrant II, etc.
- `ds` (Rhino.Geometry.Vector3d) — First partial derivative.
- `dt` (Rhino.Geometry.Vector3d) — First partial derivative.
- `dss` (Rhino.Geometry.Vector3d) — Second partial derivative.
- `dst` (Rhino.Geometry.Vector3d) — Second partial derivative.
- `dtt` (Rhino.Geometry.Vector3d) — Second partial derivative.
- `n` (Rhino.Geometry.Vector3d) — Unit normal.

**Returns:** `Boolean` — True if successful, false otherwise.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoMath_EvaluateNormal.htm)

#### `public static bool EvaluateNormalPartials(Vector3d ds, Vector3d dt, Vector3d dss, Vector3d dst, Vector3d dtt, out Vector3d ns, out Vector3d nt)`

Expert tool to evaluate partial derivatives of surface unit normal.

**Parameters:**
- `ds` (Rhino.Geometry.Vector3d) — First partial derivative.
- `dt` (Rhino.Geometry.Vector3d) — First partial derivative.
- `dss` (Rhino.Geometry.Vector3d) — Second partial derivative.
- `dst` (Rhino.Geometry.Vector3d) — Second partial derivative.
- `dtt` (Rhino.Geometry.Vector3d) — Second partial derivative.
- `ns` (Rhino.Geometry.Vector3d) — First partial derivative of unit normal. If the Jacobian is degenerate, ns is set to zero.
- `nt` (Rhino.Geometry.Vector3d) — First partial derivative of unit normal. If the Jacobian is degenerate, nt is set to zero.

**Returns:** `Boolean` — true if Jacobian is non-degenerate, false if Jacobian is degenerate.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoMath_EvaluateNormalPartials.htm)

#### `public static bool EvaluateSectionalCurvature(Vector3d ds, Vector3d dt, Vector3d dss, Vector3d dst, Vector3d dtt, Vector3d planeNormal, out Vector3d k)`

Expert tool to evaluate sectional curvature from surface derivatives and section plane normal.

**Parameters:**
- `ds` (Rhino.Geometry.Vector3d) — First partial derivative.
- `dt` (Rhino.Geometry.Vector3d) — First partial derivative.
- `dss` (Rhino.Geometry.Vector3d) — Second partial derivative.
- `dst` (Rhino.Geometry.Vector3d) — Second partial derivative.
- `dtt` (Rhino.Geometry.Vector3d) — Second partial derivative.
- `planeNormal` (Rhino.Geometry.Vector3d) — Unit normal to section plane.
- `k` (Rhino.Geometry.Vector3d) — Sectional curvature. Curvature of the intersection curve of the surface and plane through the surface point where the partial derivatives were evaluated.

**Returns:** `Boolean` — True if successful. False if first partials are not linearly independent, in which case the k is set to zero.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoMath_EvaluateSectionalCurvature.htm)

#### `public static string IntIndexToString(int index)`

Portrays an Int32 index in text.

**Parameters:**
- `index` (System.Int32) — Int32 number express as string.

**Returns:** `String` — The text representation of the int index.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoMath_IntIndexToString.htm)

#### `public static bool IsValidDouble(double x)`

Determines whether a Double value is valid within the RhinoCommon context. Rhino does not use Double.NaN by convention, so this test evaluates to true if:x is not equal to RhinoMath.UnsetValueSystem.Double.IsNaN(x) evaluates to falseSystem.Double.IsInfinity(x) evaluates to false

**Parameters:**
- `x` (System.Double) — Double number to test for validity.

**Returns:** `Boolean` — true if the number if valid, false if the number is NaN, Infinity or Unset.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoMath_IsValidDouble.htm)

#### `public static bool IsValidSingle(float x)`

Determines whether a Single value is valid within the RhinoCommon context. Rhino does not use Single.NaN by convention, so this test evaluates to true if:x is not equal to RhinoMath.UnsetValue,System.Single.IsNaN(x) evaluates to falseSystem.Single.IsInfinity(x) evaluates to false

**Parameters:**
- `x` (System.Single) — Single number to test for validity.

**Returns:** `Boolean` — true if the number if valid, false if the number is NaN, Infinity or Unset.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoMath_IsValidSingle.htm)

#### `public static double MetersPerUnit(UnitSystem units)`

Return number of meters per one unit of a given unit system

**Parameters:**
- `units` (Rhino.UnitSystem)

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.RhinoMath.MetersPerUnit(Rhino.UnitSystem)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoMath_MetersPerUnit.htm)

#### `public static double ParseNumber(string expression)`

Evaluates command line math expression.

**Parameters:**
- `expression` (System.String)

**Returns:** `Double` — result

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoMath_ParseNumber.htm)

#### `public static double ToDegrees(double radians)`

Convert an angle from radians to degrees.

**Parameters:**
- `radians` (System.Double) — Radians to convert (180 degrees equals pi radians).

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.RhinoMath.ToDegrees(System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoMath_ToDegrees.htm)

#### `public static double ToRadians(double degrees)`

Convert an angle from degrees to radians.

**Parameters:**
- `degrees` (System.Double) — Degrees to convert (180 degrees equals pi radians).

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.RhinoMath.ToRadians(System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoMath_ToRadians.htm)

#### `public static bool TryParseNumber(string expression, out double result)`

Evaluates command line math expression.

**Parameters:**
- `expression` (System.String)
- `result` (System.Double)

**Returns:** `Boolean` — true if successful otherwise false

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoMath_TryParseNumber.htm)

#### `public static double UnitScale(UnitSystem from, double fromMetersPerUnit, UnitSystem to, double toMetersPerUnit)`

Computes the scale factor for changing the measurements unit systems.

**Parameters:**
- `from` (Rhino.UnitSystem) — The system to convert from.
- `fromMetersPerUnit` (System.Double) — For custom units, specify the meters per unit.
- `to` (Rhino.UnitSystem) — The system to convert measurements into.
- `toMetersPerUnit` (System.Double) — For custom units, specify the meters per unit.

**Returns:** `Double` — A scale multiplier.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoMath_UnitScale_1.htm)

#### `public static double UnitScale(UnitSystem from, UnitSystem to)`

Computes the scale factor for changing the measurements unit systems.

**Parameters:**
- `from` (Rhino.UnitSystem) — The system to convert from.
- `to` (Rhino.UnitSystem) — The system to convert measurements into.

**Returns:** `Double` — A scale multiplier.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoMath_UnitScale.htm)

#### `public static double Wrap(double value, double bound1, double bound2)`

Limits a Double to be specified within an interval of two numbers by repeating the available interval cyclically.

**Parameters:**
- `value` (System.Double) — A number.
- `bound1` (System.Double) — A first bound.
- `bound2` (System.Double) — A second bound. This does not necessarily need to be larger or smaller than bound1.

**Returns:** `Double` — [Missing <returns> documentation for "M:Rhino.RhinoMath.Wrap(System.Double,System.Double,System.Double)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_RhinoMath_Wrap.htm)

### Properties
- `DefaultAngleTolerance` (Double, get) — Represents the default angle tolerance, used when no other values are provided. This is one degree, expressed in radians.
- `DefaultDistanceToleranceMillimeters` (Double, get) — Get Rhino's default distance tolerance in millimeters.
- `Epsilon` (Double, get) — Gets the value of DBL_EPSILON, which is the smallest positive floating point number x such that 1 + x != 1. This is different than Double.Epsilon which is the smallest positive Double value that is greater than zero.
- `HalfPI` (Double, get) — Quarter of a rotation. 90 degrees. 1.57...
- `QuarterPI` (Double, get) — One eigth of a rotation. 45 degrees. 0.78...
- `SqrtEpsilon` (Double, get) — Represents a default value that is used when comparing square roots. This value is several orders of magnitude larger than ZeroTolerance.
- `Tau` (Double, get) — Ratio of circumference divided by radius. Full rotation. 360 degrees. 6.28...
- `TwoPI` (Double, get) — Full rotation. 360 degrees. 6.28...
- `UnsetIntIndex` (Int32, get) — When signed int values are used in a context where 0 and small negative values are valid indices and there needs to be a value that indicates the index is not set.
- `UnsetSingle` (Single, get) — Gets the single precision floating point number that is considered 'unset' in Rhino.
- `UnsetValue` (Double, get) — Gets the Rhino standard Unset value. Use this value rather than Double.NaN when a bogus floating point value is required.
- `ZeroTolerance` (Double, get) — Gets Rhino's Zero Tolerance constant, which is 2^-32. In cases when an absolute "zero" tolerance is required to compare model space coordinates, ZeroTolerance is used. The value of ZeroTolerance should be no smaller than Epsilon and should be several orders of magnitude smaller than SqrtEpsilon.

## RuntimeEnvironment (enum)

ON::RuntimeEnvironment identifies a runtime environment (operating system). This value is saved in binary archives so appropriate adjustments to resources provided by runtime environments, like fonts, can be made when an archive created in one runtime environment is used in another.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_RuntimeEnvironment.htm)

### Values
- `Unset` = `0` — ON::RuntimeEnvironment::Unset indicates no runtime is set.
- `None` = `1` — ON::RuntimeEnvironment::None indicates no runtime. This is a different condition from ON::Runtime::Unset.
- `Windows` = `2` — ON::RuntimeEnvironment::Windows indicates some version of Microsoft Windows.
- `Apple` = `3` — ON::RuntimeEnvironment::Apple indicates some version of Apple OS X or iOS.
- `Android` = `4` — ON::RuntimeEnvironment::Android indicates some version of Google Android.
- `Linux` = `5` — ON::RuntimeEnvironment::Linux indicates some version of Linux.
- `WebAssembly` = `6` — ON::RuntimeEnvironment::WebAssembly indicates some version of WASM / WebAssembly.

## ScaleValue (class)

Represents a scale with associated LengthValues and string representations of the scale. This allows for going back and forth from numerical representations of a scale and a string representation without "guessing" at the initial scale.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_ScaleValue.htm)

### Constructors
- `public ScaleValue()` — Default constructor

### Methods
#### `public static ScaleValue Create(LengthValue left, LengthValue right, ScaleValue.ScaleStringFormat format)`

Create from 2 length values

**Parameters:**
- `left` (Rhino.LengthValue) — [Missing <param name="left"/> documentation for "M:Rhino.ScaleValue.Create(Rhino.LengthValue,Rhino.LengthValue,Rhino.ScaleValue.ScaleStringFormat)"]
- `right` (Rhino.LengthValue) — [Missing <param name="right"/> documentation for "M:Rhino.ScaleValue.Create(Rhino.LengthValue,Rhino.LengthValue,Rhino.ScaleValue.ScaleStringFormat)"]
- `format` (Rhino.ScaleValue.ScaleStringFormat) — [Missing <param name="format"/> documentation for "M:Rhino.ScaleValue.Create(Rhino.LengthValue,Rhino.LengthValue,Rhino.ScaleValue.ScaleStringFormat)"]

**Returns:** `ScaleValue` — [Missing <returns> documentation for "M:Rhino.ScaleValue.Create(Rhino.LengthValue,Rhino.LengthValue,Rhino.ScaleValue.ScaleStringFormat)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_ScaleValue_Create.htm)

#### `public static ScaleValue Create(string s, StringParserSettings ps)`

Create from string

**Parameters:**
- `s` (System.String) — [Missing <param name="s"/> documentation for "M:Rhino.ScaleValue.Create(System.String,Rhino.Input.StringParserSettings)"]
- `ps` (Rhino.Input.StringParserSettings) — [Missing <param name="ps"/> documentation for "M:Rhino.ScaleValue.Create(System.String,Rhino.Input.StringParserSettings)"]

**Returns:** `ScaleValue` — [Missing <returns> documentation for "M:Rhino.ScaleValue.Create(System.String,Rhino.Input.StringParserSettings)"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_ScaleValue_Create_1.htm)

#### `public void Dispose()`

actively reclaim native allocated ON_SacleValue*

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_ScaleValue_Dispose.htm)

#### `protected override void Finalize()`

passively reclaim native allocated ON_ScaleValue*

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_ScaleValue_Finalize.htm)

#### `public bool IsUnset()`

Test IsUnset

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.ScaleValue.IsUnset"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_ScaleValue_IsUnset.htm)

#### `public LengthValue LeftLengthValue()`

Get the Left LengthValue from Scale

**Returns:** `LengthValue` — [Missing <returns> documentation for "M:Rhino.ScaleValue.LeftLengthValue"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_ScaleValue_LeftLengthValue.htm)

#### `public static ScaleValue OneToOne()`

Make a new ScaleValue set to OneToOne

**Returns:** `ScaleValue` — [Missing <returns> documentation for "M:Rhino.ScaleValue.OneToOne"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_ScaleValue_OneToOne.htm)

#### `public LengthValue RightLengthValue()`

Get the Right LengthValue from Scale

**Returns:** `LengthValue` — [Missing <returns> documentation for "M:Rhino.ScaleValue.RightLengthValue"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_ScaleValue_RightLengthValue.htm)

### Properties
- `LeftToRightScale` (Double, get) — LeftLengthValue / RightLengthValue
- `RightToLeftScale` (Double, get) — RightLengthValue / LeftLengthValue

## ScaleValue.ScaleStringFormat (enum)

Specifies preferred formats for automatically created string descriptions of a scale value.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_ScaleValue_ScaleStringFormat.htm)

### Values
- `None` = `0` — No preference for automatically created string descriptions of a scale value.
- `RatioFormat` = `1` — Prefer the ratio format using a colon, like "1:4" or "4:1".
- `EquationFormat` = `2` — Prefer the equation format using an equal sign, like "1 = 4" or "4 = 1".
- `FractionFormat` = `3` — Prefer the fraction format using a slash, like "1/4" or "4/1".
- `Unset` = `255` — ON_ScaleValue::ScaleStringFormat::Unset is used to indicate no preference is set. This condition is different from ON_ScaleValue::ScaleStringFormat::None.

## Symbols (class)

Characters used for different 'drafting style' symbols

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Symbols.htm)

### Properties
- `DegreeSymbol` (Char, get) — Degree symbol used for angles
- `DiameterSymbol` (Char, get) — Diameter symbol
- `PlusMinusSymbol` (Char, get) — Plus-Minus tolerance symbol
- `RadiusSymbol` (Char, get) — Radius symbol

## UnitSystem (enum)

ON::LengthUnitSystem identifies a length unit system United States customary length units references: http://www.nist.gov/pml/wmd/metric/upload/frn-59-5442-1959.pdf http://en.wikipedia.org/wiki/United_States_customary_units http://en.wikipedia.org/wiki/International_yard_and_pound

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UnitSystem.htm)

### Values
- `None` = `0` — ON::LengthUnitSystem::None indicates no length unit system. The scale factor when converting between a specified unit system and None is always 1.0. ON::LengthUnitSystem::None is used as a unit system for models and instance definitions that should be imported or referenced with no scaling applied.
- `Angstroms` = `12` — 1 angstroms = 1.0e-10 meters
- `Nanometers` = `13` — 1 nanometer = 1.0e-9 meters
- `Microns` = `1` — 1 micron = 1.0e-6 meters
- `Millimeters` = `2` — 1 millimeter = 1.0e-3 meters
- `Centimeters` = `3` — 1 centimeter = 1.0e-2 meters
- `Decimeters` = `14` — 1 decimeter = 1.0e-1 meters
- `Meters` = `4` — SI meter length unit
- `Dekameters` = `15` — 1 dekameter = 1.0e+1 meters
- `Hectometers` = `16` — 1 hectometer = 1.0e+2 meters
- `Kilometers` = `5` — 1 kilometer = 1.0e+3 meters
- `Megameters` = `17` — 1 megameter = 1.0e+6 meters
- `Gigameters` = `18` — 1 gigameter = 1.0e+9 meters
- `Microinches` = `6` — 1 microinches = 2.54e-8 meters = 1.0e-6 inches
- `Mils` = `7` — 1 mil = 2.54e-5 meters = 0.001 inches
- `Inches` = `8` — 1 inch = 0.0254 meters = 1/12 foot
- `Feet` = `9` — 1 foot = 0.3048 meters (12 inches)
- `Yards` = `19` — 1 foot = 0.3048 meters = 12 inches
- `Miles` = `10` — 1 US statute mile = 1609.344 meters = 5280 feet
- `PrinterPoints` = `20` — 1 printer point = 1/72 inch
- `PrinterPicas` = `21` — 1 printer pica = 1/6 inch
- `NauticalMiles` = `22` — 1 nautical mile = 1852 meters Approximately 1 minute of arc on a terrestrial great circle. Reference: http://en.wikipedia.org/wiki/Nautical_mile
- `AstronomicalUnits` = `23` — 1 astronomical unit = 1.4959787e+11 meters An astronomical unit (au) is the mean distance from the center of the earth to the center of the sun. References: http://en.wikipedia.org/wiki/Astronomical_unit (1.4959787e+11 meters) http://units.nist.gov/Pubs/SP811/appenB9.htm (1.495979e+11 meters)
- `LightYears` = `24` — 1 light year = 9.4607304725808e+15 meters A light year is the distance light travels in one Julian year. The speed of light is exactly 299792458 meters/second. A Julian year is exactly 365.25 * 86400 seconds and is approximately the time it takes for one earth orbit. References: http://en.wikipedia.org/wiki/Light_year (9.4607304725808e+15 meters) http://units.nist.gov/Pubs/SP811/appenB9.htm (9.46073e+15 meters)
- `Parsecs` = `25` — 1 parsec = 3.08567758e+16 meters References: http://en.wikipedia.org/wiki/Parsec (3.08567758e+16 meters) http://units.nist.gov/Pubs/SP811/appenB9.htm (3.085678e+16)
- `CustomUnits` = `11` — The name of a custom unit and the conversion to meters are saved in the ON_UnitSystem class.
- `Unset` = `255` — The ON::LengthUnitSystem::Unset is used to indicate no unit system is set. This is a different condition from ON::LengthUnitSystem::None.

## UnitsChangedWithScalingEventArgs (class)

Provides information about UnitsChangedWithScaling events.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_UnitsChangedWithScalingEventArgs.htm)

### Properties
- `Document` (RhinoDoc, get) — Gets the document for this event. This field might be null.
- `DocumentSerialNumber` (UInt32, get) — Gets the uniques document serial number for this event.
- `Scale` (Double, get) — The scale factor.

