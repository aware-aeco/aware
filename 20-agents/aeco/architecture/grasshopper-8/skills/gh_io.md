---
name: grasshopper-gh_io
description: This skill encodes the grasshopper 8.0 surface of the GH_IO namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: GH_ISerializable, VersionNumber.Branch, VersionNumber.
---

# GH_IO

Auto-generated from vendor docs for grasshopper 8.0. 3 types in this namespace.

## GH_ISerializable (interface)

Every object which needs to (de)serialize itself using GH_IO methodology must implement this interface.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_GH_IO_GH_ISerializable.htm)

### Methods
#### `bool Read(GH_IReader reader)`

This method is called whenever the instance is required to deserialize itself.

**Parameters:**
- `reader` (GH_IO.Serialization.GH_IReader) — Reader object to deserialize from.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_GH_ISerializable_Read.htm)

#### `bool Write(GH_IWriter writer)`

This method is called whenever the instance is required to serialize itself.

**Parameters:**
- `writer` (GH_IO.Serialization.GH_IWriter) — Writer object to serialize with.

**Returns:** `Boolean` — True on success, false on failure.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_GH_ISerializable_Write.htm)

## VersionNumber (struct)

Represents a product version number that encodes major, minor, time and build branch information. as major.minor.yyddd.hhmmb where: yy = year of build - 2000 ddd = year day of build (1-366) hh = hour of build mm = minute of build b = branch of build

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_GH_IO_VersionNumber.htm)

### Constructors
- `public VersionNumber(Version version)` — Construct VersionNumber based on existing System.Version class.
- `public VersionNumber(int major, int minor, DateTime time, VersionNumber.Branch buildBranch)` — Initializes a new instance of the VersionNumber structure to the version specified by the parameters.
- `public VersionNumber(int majorVersionNumber, int minorVersionNumber, int versionQuartetYyddd, int versionQuartetHhmmb)` — Initializes a new instance of the VersionNumber structure to the version specified by the version quartet values.

### Methods
#### `public int CompareTo(Object value)`

Compares the value of this instance to a specified VersionNumber value and returns an integer that indicates whether this instance is earlier than, the same as, or later than the specified VersionNumber value.

**Parameters:**
- `value` (System.Object) — The object to compare to the current instance.

**Returns:** `Int32` — A signed number indicating the relative values of this instance and the value parameter. Less than zero indicates this instance is earier than value. Zero indicates this instance is the same as value. Greater than zero indicates this instance is later than value.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_VersionNumber_CompareTo_1.htm)

#### `public int CompareTo(VersionNumber value)`

Compares the value of this instance to a specified VersionNumber value and returns an integer that indicates whether this instance is earlier than, the same as, or later than the specified VersionNumber value.

**Parameters:**
- `value` (GH_IO.VersionNumber) — The object to compare to the current instance.

**Returns:** `Int32` — A signed number indicating the relative values of this instance and the value parameter. Less than zero indicates this instance is earier than value. Zero indicates this instance is the same as value. Greater than zero indicates this instance is later than value.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_VersionNumber_CompareTo.htm)

#### `public override string ToString()`

Converts the value of the current VersionNumber object to its equivalent string representation major.minor.yyddd.hhmmb. (Overrides ValueType.ToString().)

**Returns:** `String` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_VersionNumber_ToString.htm)

#### `public Version ToVersion()`

Convert this VersionNumber class to System.Version()

**Returns:** `Version` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_VersionNumber_ToVersion.htm)

#### `public static bool TryParse(string s, out VersionNumber result)`

Converts the specified string representation of a version number to its VersionNumber equivalent and returns a value that indicates whether the conversion succeeded.

**Parameters:**
- `s` (System.String) — A string containing four integers separated by full stops in the format major.minor.yyddd.hhmmb, where yy is year-2000, ddd is the day of the year, hh is the hour, mm is the minute and b identifies the build branch.
- `result` (GH_IO.VersionNumber) — When this method returns, result contains the VersionNumber value equivalent to the version number contained in s, if the conversion succeeded, or VersionNumber.Unset if the conversion failed. The conversion fails if the s parameter is null, is an empty string (""), or does not contain a valid string representation of version number. This parameter is passed uninitialized.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_VersionNumber_TryParse.htm)

#### `public static bool TryParse(Version v, out VersionNumber result)`

Attempt so convert the System.Version representation of a version number to its VersionNumber equivalent and returns a value that indicates whether the conversion succeeded.

**Parameters:**
- `v` (System.Version) — A version number with quartet values (v.Major, v.Minor, v.Build, v.Revision) that are valid ((0 to 63),(0 to 127), (1 to 99365), (0 to 23593)).
- `result` (GH_IO.VersionNumber) — When this method returns, result contains the VersionNumber value equivalent to the version number contained in v, if the conversion succeeded, or VersionNumber.Unset if the conversion failed. The conversion fails if the v parameter is null, is an empty string (""), or does not contain a valid string representation of version number. This parameter is passed uninitialized.

**Returns:** `Boolean` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_GH_IO_VersionNumber_TryParse_1.htm)

### Properties
- `BuildBranch` (VersionNumber.Branch, get) — Gets the build branch component of this instance.
- `IsValid` (Boolean, get) — True when all information in the VersionNumber has valid values.
- `Major` (Int32, get) — Gets the major version number component of this instance.
- `MaxMajorVersionNumber` (Int32, get) — The largest possible valid value of VersionNumber.MajorVersionNumber.
- `MaxMinorVersionNumber` (Int32, get) — The largest possible valid value of VersionNumber.MinorVersionNumber.
- `MaxValid` (VersionNumber, get) — The largest possible valid VersionNumber.
- `MaxValidBuildBranch` (VersionNumber.Branch, get) — Represents the largest possible valid value of VersionNumber.BuildBranch. This field is read-only.
- `MaxValidTime` (DateTime, get) — The largest possible valid value of VersionNumber.Time. This field is read-only.
- `MinMajorVersionNumber` (Int32, get) — The smallest possible valid value of VersionNumber.MajorVersionNumber.
- `MinMinorVersionNumber` (Int32, get) — The smallest possible valid value of VersionNumber.MinorVersionNumber.
- `Minor` (Int32, get) — Gets the minor version number component of this instance.
- `MinValid` (VersionNumber, get) — The smallest possible valid VersionNumber.
- `MinValidBuildBranch` (VersionNumber.Branch, get) — Represents the smallest possible valid value of VersionNumber.BuildBranch. This field is read-only.
- `MinValidTime` (DateTime, get) — The smallest possible valid value of VersionNumber.Time. This field is read-only.
- `Time` (DateTime, get) — Gets the version time component of this instance.
- `Unset` (VersionNumber, get) — The Unset VersionNumber. VersionNumber.Unset.IsValid is false.
- `UnsetBuildBranch` (VersionNumber.Branch, get) — The value of an Unset VersionNumber.BuildBranch.
- `UnsetTime` (DateTime, get) — The value of an Unset VersionNumber.Time.

## VersionNumber.Branch (enum)

Build branch, set by the build process.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_GH_IO_VersionNumber_Branch.htm)

### Values
- `Unset` = `0` — Uninitialized value.
- `Developer` = `1` — Private developer build; should never be released to the public.
- `Trunk` = `2` — Mainline trunk build. May be released to the public.
- `ReleaseCandidate` = `3` — Release candidate build. Will be released to the public.
- `Release` = `4` — Final Release build. Will be released to the public.

