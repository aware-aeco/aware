---
name: tekla-tekla-structures-datatype
description: This skill encodes the tekla 2026.0 surface of the Tekla.Structures.Datatype namespace — 25 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AngleList, Angle, BooleanConverter, Constants, DistanceConverter, DistanceListConverter, DoubleConverter, EnumConverter, and 17 more types.
---

# Tekla.Structures.Datatype

Auto-generated from vendor docs for tekla 2026.0. 25 types in this namespace.

## Angle (class)

Provides support for angles using either the current Tekla Structures units, or user defined units

[Vendor docs](https://developer.tekla.com/topic/en/18/47/08e0a838-72ea-816e-f76e-3344b1d23e2f)

### Methods
#### `public int CompareTo(Angle other)`

Compares this angle with another angle

**Parameters:**
- `other` (Tekla.Structures.Datatype.Angle) — Angle to compare to

**Returns:** `Int32` — Greater than zero if this angle is greater than other, otherwise less than zero if this angle is less than other, otherwise 0 if they are equal

[Docs](https://developer.tekla.com/topic/en/18/47/9b50fc0f-955f-1707-d103-d0a2f68d2fca)

#### `public bool Equals(Angle other)`

Checks for approximate angle equality

**Parameters:**
- `other` (Tekla.Structures.Datatype.Angle) — Angle to compare to

**Returns:** `Boolean` — True if the angles are the same, false otherwise

[Docs](https://developer.tekla.com/topic/en/18/47/6eb7ef8e-82ab-34b5-8cab-b271146fce8d)

#### `public static Angle FromCurrentUnit(double angle)`

Creates an angle given in the current unit type

**Parameters:**
- `angle` (System.Double) — Angle given in the current unit type

**Returns:** `Angle` — An Angle object representing the given angle

[Docs](https://developer.tekla.com/topic/en/18/47/8eea6759-0d84-ac54-4123-08f75daafd8e)

#### `public static Angle FromDegrees(double degrees)`

Creates an angle from a value in degrees

**Parameters:**
- `degrees` (System.Double) — The angle in degrees

**Returns:** `Angle` — An Angle object representing the given angle

[Docs](https://developer.tekla.com/topic/en/18/47/f3b45685-c2c6-c229-bd8d-a60a3993b690)

#### `public static Angle FromRadians(double radians)`

Creates an angle from a value in radians

**Parameters:**
- `radians` (System.Double) — The angle in radians

**Returns:** `Angle` — An Angle object representing the given angle

[Docs](https://developer.tekla.com/topic/en/18/47/ce4f5faa-0af0-5d4a-3054-91f419644f30)

#### `public static Angle Parse(string text)`

Parses the given text as an angle in the current unit type

**Parameters:**
- `text` (System.String) — Text to parse

**Returns:** `Angle` — Resulting angle from the text

[Docs](https://developer.tekla.com/topic/en/18/47/6392e038-4934-caf1-743b-cb2f4a110187)

#### `public static Angle Parse(string text, IFormatProvider formatProvider)`

Parses the given text as an angle in the current unit type

**Parameters:**
- `text` (System.String) — Text to parse
- `formatProvider` (System.IFormatProvider) — Format provider

**Returns:** `Angle` — Resulting angle from the text

[Docs](https://developer.tekla.com/topic/en/18/47/fed77922-44d4-c8ce-763b-f4f3ccc7e793)

#### `public static Angle Parse(string text, IFormatProvider formatProvider, Angle.UnitType unitType)`

Parses the given text as an angle in the current unit type

**Parameters:**
- `text` (System.String) — Text to parse
- `formatProvider` (System.IFormatProvider) — Format provider
- `unitType` (Tekla.Structures.Datatype.Angle.UnitType) — Input unit type

**Returns:** `Angle` — Resulting angle from the text

[Docs](https://developer.tekla.com/topic/en/18/47/e50fed20-8e48-aa8a-defe-5dd610183f40)

#### `public string ToString(Angle.UnitType unitType)`

Converts the represented angle to a string in the given units

**Parameters:**
- `unitType` (Tekla.Structures.Datatype.Angle.UnitType) — Input unit type

**Returns:** `String` — This angle as a string

[Docs](https://developer.tekla.com/topic/en/18/47/03bfa56e-97b9-9274-eb38-aa22528502dd)

#### `public string ToString(string format, IFormatProvider formatProvider)`

Converts the represented angle to a string with the given parameters

**Parameters:**
- `format` (System.String) — The format string. Can be null.
- `formatProvider` (System.IFormatProvider) — The number format provider.

**Returns:** `String` — This angle as a string

[Docs](https://developer.tekla.com/topic/en/18/47/6a45a7f7-9fca-bd37-df54-e6a26e58f575)

#### `public string ToString(string format, IFormatProvider formatProvider, Angle.UnitType angleUnit)`

Converts the represented angle to a string with the given parameters

**Parameters:**
- `format` (System.String) — The format string. Can be null.
- `formatProvider` (System.IFormatProvider) — The number format provider.
- `angleUnit` (Tekla.Structures.Datatype.Angle.UnitType) — The unit type.

**Returns:** `String` — This angle as a string

[Docs](https://developer.tekla.com/topic/en/18/47/79ca5535-ce7d-7f31-3ef0-d0d87bbb19ae)

#### `public override string ToString()`

Converts the represented angle to a string in the current units

**Returns:** `String` — This angle as a string

[Docs](https://developer.tekla.com/topic/en/18/47/4aa323d3-8ea8-8f87-565a-304f1ac29617)

#### `public static bool TryParse(string text, out Angle result)`

Parses the given text as an angle in the current unit type

**Parameters:**
- `text` (System.String) — Text to parse
- `result` (Tekla.Structures.Datatype.Angle.) — Resulting angle if success

**Returns:** `Boolean` — True if the conversion succeeds, false otherwise

[Docs](https://developer.tekla.com/topic/en/18/47/c93cffab-483e-40cb-fe9d-a557d004038f)

#### `public static bool TryParse(string text, IFormatProvider formatProvider, out Angle result)`

Parses the given text as an angle in the current unit type

**Parameters:**
- `text` (System.String) — Text to parse
- `formatProvider` (System.IFormatProvider) — Format provider
- `result` (Tekla.Structures.Datatype.Angle.) — Resulting angle if success

**Returns:** `Boolean` — True if the conversion succeeds, false otherwise

[Docs](https://developer.tekla.com/topic/en/18/47/30a22c28-8ae0-dc9d-00dd-f42c6079d87c)

#### `public static bool TryParse(string text, IFormatProvider formatProvider, Angle.UnitType unitType, out Angle result)`

Parses the given text as an angle in the current unit type

**Parameters:**
- `text` (System.String) — Text to parse
- `formatProvider` (System.IFormatProvider) — Format provider
- `unitType` (Tekla.Structures.Datatype.Angle.UnitType) — Input unit type
- `result` (Tekla.Structures.Datatype.Angle.) — Resulting angle if success

**Returns:** `Boolean` — True if the conversion succeeds, false otherwise

[Docs](https://developer.tekla.com/topic/en/18/47/892a0ae8-d10f-36f9-a991-1aa9ccfef95b)

### Properties
- `CurrentUnitType` (Angle.UnitType, get/set) — Gets or sets the Tekla Structures current angle type
- `CurrentUnitValue` (Double, get) — Gets the value of the angle in the current unit
- `DecimalPlaces` (Int32, get/set) — Gets the number of decimal places used for angles.
- `Degrees` (Double, get) — Gets the angle value in degrees
- `Radians` (Double, get) — Gets the angle value in radians

## Angle.UnitType (enum)

The angle units.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/4f573ed1-28d7-08c2-766b-035f700015a2)

### Values
- `Degrees` = `0` — Represents an angle in degrees, with 0 degrees representing the empty angle and 360 the full angle
- `Radians` = `1` — Represents an angle in radians, with 0 radians representing the empty angle and 2*PI the full angle

## AngleList (class)

This class provides methods to handle lists of angles

[Vendor docs](https://developer.tekla.com/topic/en/18/47/c1bc620d-d79c-42a2-f0ac-9009f6a87509)

### Methods
#### `public static List<Angle> Parse(string text)`

Parses the given text as an angle list in the current unit type

**Parameters:**
- `text` (System.String) — Text to parse

**Returns:** `List<Angle>` — Resulting angle list from the text

[Docs](https://developer.tekla.com/topic/en/18/47/5a037ea2-783e-73d7-f7a1-34662923a055)

#### `public static List<Angle> Parse(string text, IFormatProvider formatProvider)`

Parses the given text as an angle list in the current unit type

**Parameters:**
- `text` (System.String) — Text to parse
- `formatProvider` (System.IFormatProvider) — Format provider

**Returns:** `List<Angle>` — Resulting angle list from the text

[Docs](https://developer.tekla.com/topic/en/18/47/bf366b35-4717-fdcd-f174-23607938c085)

#### `public static List<Angle> Parse(string text, IFormatProvider formatProvider, Angle.UnitType unitType)`

Parses the given text as an angle list in the current unit type

**Parameters:**
- `text` (System.String) — Text to parse
- `formatProvider` (System.IFormatProvider) — Format provider
- `unitType` (Tekla.Structures.Datatype.Angle.UnitType) — Input unit type

**Returns:** `List<Angle>` — Resulting angle list from the text

[Docs](https://developer.tekla.com/topic/en/18/47/4113d54a-0ab4-8b2e-7062-a3771be61c87)

#### `public static string ToString(IEnumerable<Angle> angleList)`

Converts the given angles to a string in the current units

**Parameters:**
- `angleList` (System.Collections.Generic.IEnumerable<Angle>) — List of angles to convert

**Returns:** `String` — The angle list as a string

[Docs](https://developer.tekla.com/topic/en/18/47/8d7e5255-799a-1f85-a13f-08dd45382341)

#### `public static string ToString(IEnumerable<Angle> angleList, Angle.UnitType unitType)`

Converts the given angles to a string in the given units

**Parameters:**
- `angleList` (System.Collections.Generic.IEnumerable<Angle>) — List of angles to convert
- `unitType` (Tekla.Structures.Datatype.Angle.UnitType) — Input unit type

**Returns:** `String` — The angle list as a string

[Docs](https://developer.tekla.com/topic/en/18/47/9b32abf0-6afc-3f7b-dab7-dd9098e60177)

#### `public static string ToString(IEnumerable<Angle> angleList, string format, IFormatProvider formatProvider)`

Converts the given angles to a string with the given parameters in the current unit

**Parameters:**
- `angleList` (System.Collections.Generic.IEnumerable<Angle>) — List of angles to convert
- `format` (System.String) — The format string. Can be null.
- `formatProvider` (System.IFormatProvider) — The number format provider.

**Returns:** `String` — The angle as a string

[Docs](https://developer.tekla.com/topic/en/18/47/15ac3c4f-5d3f-0463-54ba-088f3f089f97)

#### `public static string ToString(IEnumerable<Angle> angleList, string format, IFormatProvider formatProvider, Angle.UnitType angleUnit)`

Converts the given angles to a string with the given parameters in the current unit

**Parameters:**
- `angleList` (System.Collections.Generic.IEnumerable<Angle>) — List of angles to convert
- `format` (System.String) — The format string. Can be null.
- `formatProvider` (System.IFormatProvider) — The number format provider.
- `angleUnit` (Tekla.Structures.Datatype.Angle.UnitType) — The unit type.

**Returns:** `String` — The angle as a string

[Docs](https://developer.tekla.com/topic/en/18/47/d2f3fab0-5ef0-1c85-20a3-053346a181d2)

## Boolean (struct)

The Boolean datatype.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/0dbf9276-f795-5425-8573-6ff1d402fa38)

### Constructors
- `public Boolean(bool boolValue)` — Initializes a new instance of the structure.

### Properties
- `Value` (Boolean, get/set) — Gets or sets the boolean value.

## BooleanConverter (class)

The BooleanConverter class converts types to and from the boolean type.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/e88cfafe-df6e-34c6-94a2-66ac5f5c45c8)

### Constructors
- `public BooleanConverter()` — Initializes a new instance of the BooleanConverter class

### Methods
#### `public virtual bool CanConvertFrom(ITypeDescriptorContext context, Type sourceType)`

Checks whether the conversion can be made from the source type to the boolean type.

**Parameters:**
- `context` (System.ComponentModel.ITypeDescriptorContext) — The context.
- `sourceType` (System.Type) — The type to convert from.

**Returns:** `Boolean` — True if possible, false if not possible.

[Docs](https://developer.tekla.com/topic/en/18/47/9ed92bf9-a0b1-1e74-3a6c-1d0282840a95)

#### `public virtual bool CanConvertTo(ITypeDescriptorContext context, Type destinationType)`

Checks whether the conversion can be made from the boolean type to the destination type.

**Parameters:**
- `context` (System.ComponentModel.ITypeDescriptorContext) — The context.
- `destinationType` (System.Type) — The type to convert to.

**Returns:** `Boolean` — True if possible, false if not possible.

[Docs](https://developer.tekla.com/topic/en/18/47/34334764-e5fb-2756-3815-49476e791327)

#### `public virtual Object ConvertFrom(ITypeDescriptorContext context, CultureInfo culture, Object value)`

Converts an object from the given culture to the boolean type.

**Parameters:**
- `context` (System.ComponentModel.ITypeDescriptorContext) — The context.
- `culture` (System.Globalization.CultureInfo) — The culture.
- `value` (System.Object) — The object to be converted.

**Returns:** `Object` — The new boolean object.

[Docs](https://developer.tekla.com/topic/en/18/47/ebeec01c-5dbd-8080-3bfb-c917accf0edb)

#### `public virtual Object ConvertTo(ITypeDescriptorContext context, CultureInfo culture, Object value, Type destinationType)`

Converts an object from the boolean type to the given destination type.

**Parameters:**
- `context` (System.ComponentModel.ITypeDescriptorContext) — The context.
- `culture` (System.Globalization.CultureInfo) — The target culture.
- `value` (System.Object) — The boolean object to be converted.
- `destinationType` (System.Type) — The destination type.

**Returns:** `Object` — The given boolean object converted to the new type.

[Docs](https://developer.tekla.com/topic/en/18/47/14446341-9404-e5cc-cf09-25f1930a9386)

## Constants (class)

The Constants class is a repository for constants used by the Tekla.Structures.Datatype.* types.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/a9c87d6b-3c67-250d-c2d6-addcbb36aa96)

## Distance (struct)

The Distance datatype.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/db01624c-588b-13b0-0d37-cae81b0408e4)

### Constructors
- `public Distance(double millimeters)` — Initializes a new instance of the structure.
- `public Distance(double distance, Distance.UnitType unitType)` — Initializes a new instance of the structure.

### Methods
#### `public int CompareTo(Distance other)`

Compares the current object with another object of the same type.

**Parameters:**
- `other` (Tekla.Structures.Datatype.Distance) — The object to be compared with the current object.

**Returns:** `Int32` — A 32-bit signed integer that indicates the relative order of the objects being compared.The return value has the following meanings:ValueMeaningLess than zeroThe current object is less than the other object.ZeroThe current object is equal to the other object.Greater than zeroThe current object is greater than the other object.

[Docs](https://developer.tekla.com/topic/en/18/47/0696222a-5d13-2d03-130b-b84be5abb7bd)

#### `public double ConvertTo(Distance.UnitType unitType)`

Converts the distance to the specified units.

**Parameters:**
- `unitType` (Tekla.Structures.Datatype.Distance.UnitType) — The distance unit type.

**Returns:** `Double` — The distance in the specified units.

[Docs](https://developer.tekla.com/topic/en/18/47/8ae399a4-8b28-afd9-57ca-3f5e9d9a30d9)

#### `public bool Equals(Distance other)`

Indicates whether the current object is equal to another object of the same type.

**Parameters:**
- `other` (Tekla.Structures.Datatype.Distance) — The object to be compared with the current object.

**Returns:** `Boolean` — True if the current object is equal to the other object; otherwise, false.

[Docs](https://developer.tekla.com/topic/en/18/47/e692e8e8-f12f-0b5c-315d-d2c9ae41baca)

#### `public override bool Equals(Object obj)`

Indicates whether the current instance and the specified object are equal.

**Parameters:**
- `obj` (System.Object) — The object to be compared with the current object.

**Returns:** `Boolean` — True if the specified object and the current instance are of the same type and represent the same value; otherwise, false.

[Docs](https://developer.tekla.com/topic/en/18/47/33943554-de05-52a1-9af6-71ea101f7ba4)

#### `public static Distance FromDecimalString(string text)`

Creates a distance from a decimal string representation.

**Parameters:**
- `text` (System.String) — The distance string.

**Returns:** `Distance` — The representation of the specified distance.

[Docs](https://developer.tekla.com/topic/en/18/47/6211cf0a-42b1-8e32-d5e4-70d96368b1b4)

#### `public static Distance FromDecimalString(string text, IFormatProvider formatProvider)`

Creates a distance from a decimal string representation.

**Parameters:**
- `text` (System.String) — The distance string.
- `formatProvider` (System.IFormatProvider) — The number format provider.

**Returns:** `Distance` — The representation of the specified distance.

[Docs](https://developer.tekla.com/topic/en/18/47/5a0332b9-9aff-7068-3336-fb43cd5dc640)

#### `public static Distance FromDecimalString(string text, IFormatProvider formatProvider, Distance.UnitType unitType)`

Creates a distance from a decimal string representation.

**Parameters:**
- `text` (System.String) — The distance string.
- `formatProvider` (System.IFormatProvider) — The number format provider.
- `unitType` (Tekla.Structures.Datatype.Distance.UnitType) — The unit type of the distance to parse.

**Returns:** `Distance` — The representation of the specified distance.

[Docs](https://developer.tekla.com/topic/en/18/47/ef97a044-fb84-c24e-9925-2169502e7e19)

#### `public static Distance FromFractionalFeetAndInchesString(string text)`

Creates a distance from a string representation of fractional feet and inches.

**Parameters:**
- `text` (System.String) — The distance string.

**Returns:** `Distance` — The representation of the specified distance.

[Docs](https://developer.tekla.com/topic/en/18/47/1529c894-c88a-6e32-4bf8-04b60df86127)

#### `public static Distance FromFractionalFeetAndInchesString(string text, IFormatProvider formatProvider, Distance.UnitType unitType)`

Creates a distance from a string representation of fractional feet and inches.

**Parameters:**
- `text` (System.String) — The distance string.
- `formatProvider` (System.IFormatProvider) — The number format provider.
- `unitType` (Tekla.Structures.Datatype.Distance.UnitType) — The unit type of the distance to parse.

**Returns:** `Distance` — The representation of the specified distance.

[Docs](https://developer.tekla.com/topic/en/18/47/458c16cf-2c84-7b8e-bb0e-6fdaa833ac18)

#### `public override int GetHashCode()`

Returns the hash code for the current instance.

**Returns:** `Int32` — A 32-bit signed integer that is the hash code for the current instance.

[Docs](https://developer.tekla.com/topic/en/18/47/f11213ce-1c1d-b9bd-8246-24ac0f18a920)

#### `public static Distance Parse(string text)`

Parses a distance from a string representation using the current unit type and culture.

**Parameters:**
- `text` (System.String) — The text to parse.

**Returns:** `Distance` — The representation of the specified distance.

[Docs](https://developer.tekla.com/topic/en/18/47/a41cda91-0ae5-6855-7057-873ef3e16841)

#### `public static Distance Parse(string text, IFormatProvider formatProvider)`

Parses a distance from a string representation using the current unit type and the given culture.

**Parameters:**
- `text` (System.String) — The text to parse.
- `formatProvider` (System.IFormatProvider) — The number format provider.

**Returns:** `Distance` — The representation of the specified distance.

[Docs](https://developer.tekla.com/topic/en/18/47/30331ab4-552e-4df4-0b35-99d041d11f1d)

#### `public static Distance Parse(string text, IFormatProvider formatProvider, Distance.UnitType unitType)`

Parses a distance from a string representation using the given unit type and culture.

**Parameters:**
- `text` (System.String) — The text to parse.
- `formatProvider` (System.IFormatProvider) — The number format provider.
- `unitType` (Tekla.Structures.Datatype.Distance.UnitType) — The unit type of the distance to parse. Determines the parsing process.

**Returns:** `Distance` — The representation of the specified distance.

[Docs](https://developer.tekla.com/topic/en/18/47/7f2527a1-966d-74e5-e167-d16511fb0566)

#### `public string ToDecimalString(IFormatProvider formatProvider)`

Converts the distance to a decimal string representation.

**Parameters:**
- `formatProvider` (System.IFormatProvider) — The number format provider.

**Returns:** `String` — The string representation of the distance.

[Docs](https://developer.tekla.com/topic/en/18/47/b5280892-7714-7bc8-1e4a-7f7522040382)

#### `public string ToDecimalString(string format)`

Converts the distance to a decimal string representation.

**Parameters:**
- `format` (System.String) — The format string.

**Returns:** `String` — The string representation of the distance.

[Docs](https://developer.tekla.com/topic/en/18/47/cd0ee155-46bc-2446-d7b4-085e1c18173c)

#### `public string ToDecimalString(string format, IFormatProvider formatProvider)`

Converts the distance to a decimal string representation.

**Parameters:**
- `format` (System.String) — The format string.
- `formatProvider` (System.IFormatProvider) — The number format provider.

**Returns:** `String` — The string representation of the distance.

[Docs](https://developer.tekla.com/topic/en/18/47/f0623791-0fbb-dc73-74b6-ad1a6bde8dc9)

#### `public string ToDecimalString(string format, IFormatProvider formatProvider, Distance.UnitType unitType)`

Converts the distance to a decimal string representation.

**Parameters:**
- `format` (System.String) — The format string.
- `formatProvider` (System.IFormatProvider) — The number format provider.
- `unitType` (Tekla.Structures.Datatype.Distance.UnitType) — The unit type.

**Returns:** `String` — The string representation of the distance.

[Docs](https://developer.tekla.com/topic/en/18/47/de76014f-1500-125f-6f0a-1f4fd3017cb5)

#### `public string ToDecimalString()`

Converts the distance to a decimal string representation.

**Returns:** `String` — The string representation of the distance.

[Docs](https://developer.tekla.com/topic/en/18/47/4fdd8958-9e34-3223-bc05-807c3fbeb7ae)

#### `public string ToFractionalFeetAndInchesString(IFormatProvider formatProvider)`

Creates a string representation of the distance in feet and inches.

**Parameters:**
- `formatProvider` (System.IFormatProvider) — The number format provider.

**Returns:** `String` — The string representation of the distance.

[Docs](https://developer.tekla.com/topic/en/18/47/a34602fe-6adb-ce21-3b16-5b6258e88506)

#### `public string ToFractionalFeetAndInchesString()`

Creates a string representation of the distance in feet and inches.

**Returns:** `String` — The string representation of the distance.

[Docs](https://developer.tekla.com/topic/en/18/47/d611f8c3-11c1-c318-4bfa-5f933257fe4f)

#### `public string ToFractionalInchesString(IFormatProvider formatProvider)`

Creates a string representation of the distance in inches.

**Parameters:**
- `formatProvider` (System.IFormatProvider) — The number format provider.

**Returns:** `String` — The string representation of the distance.

[Docs](https://developer.tekla.com/topic/en/18/47/735bbd44-6bf8-d628-c205-89d237f74583)

#### `public string ToFractionalInchesString()`

Creates a string representation of the distance in inches.

**Returns:** `String` — The string representation of the distance.

[Docs](https://developer.tekla.com/topic/en/18/47/5ce38b30-c93c-cff8-6cec-60f514689552)

#### `public string ToString(IFormatProvider formatProvider)`

Creates a string representation of the distance.

**Parameters:**
- `formatProvider` (System.IFormatProvider) — The number format provider.

**Returns:** `String` — The string representation of the distance.

[Docs](https://developer.tekla.com/topic/en/18/47/c9c481b3-fc9f-f8dc-23fd-58d8c8d0d568)

#### `public string ToString(string format)`

Creates a string representation of the distance.

**Parameters:**
- `format` (System.String) — The format string.

**Returns:** `String` — The string representation of the distance.

[Docs](https://developer.tekla.com/topic/en/18/47/9d1e329d-d0fc-d9d0-d3b8-62af500b4540)

#### `public string ToString(string format, IFormatProvider formatProvider)`

Creates a string representation of the distance.

**Parameters:**
- `format` (System.String) — The format string.
- `formatProvider` (System.IFormatProvider) — The number format provider.

**Returns:** `String` — The string representation of the distance.

[Docs](https://developer.tekla.com/topic/en/18/47/048a07c8-6092-2995-5539-c2ca71b7cb01)

#### `public string ToString(string format, IFormatProvider formatProvider, Distance.UnitType unitType)`

Creates a string representation of the distance.

**Parameters:**
- `format` (System.String) — The format string.
- `formatProvider` (System.IFormatProvider) — The number format provider.
- `unitType` (Tekla.Structures.Datatype.Distance.UnitType) — The unit type.

**Returns:** `String` — The string representation of the distance.

[Docs](https://developer.tekla.com/topic/en/18/47/fc6e470e-88d7-e078-1bca-ffec1fe32042)

#### `public override string ToString()`

Creates a string representation of the distance.

**Returns:** `String` — The string representation of the distance using CultureInfo.InvariantCulture.

[Docs](https://developer.tekla.com/topic/en/18/47/7981e135-73db-fe09-efcf-6bbc0543262c)

#### `public static bool TryParse(string text, out Distance result)`

Attempts to parse a distance from a string representation using the current units.

**Parameters:**
- `text` (System.String) — The text to parse.
- `result` (Tekla.Structures.Datatype.Distance.) — The output variable for the result.

**Returns:** `Boolean` — A boolean value indicating whether the operation succeeded.

[Docs](https://developer.tekla.com/topic/en/18/47/21abb59b-79ab-c805-bfc0-a1bc27fd4e6d)

#### `public static bool TryParse(string text, IFormatProvider formatProvider, out Distance result)`

Attempts to parse a distance from a string representation using the current units.

**Parameters:**
- `text` (System.String) — The text to parse.
- `formatProvider` (System.IFormatProvider) — The number format provider.
- `result` (Tekla.Structures.Datatype.Distance.) — The output variable for the result.

**Returns:** `Boolean` — A boolean value indicating whether the operation succeeded.

[Docs](https://developer.tekla.com/topic/en/18/47/1b05f47a-26b1-7e05-947e-cf5daf99af2d)

#### `public static bool TryParse(string text, IFormatProvider formatProvider, Distance.UnitType defaultUnitType, out Distance result)`

Attempts to parse a distance from a string representation using the current units.

**Parameters:**
- `text` (System.String) — The text to parse.
- `formatProvider` (System.IFormatProvider) — The number format provider.
- `defaultUnitType` (Tekla.Structures.Datatype.Distance.UnitType) — The default unit type if the unit cannot be determined.
- `result` (Tekla.Structures.Datatype.Distance.) — The output variable for the result.

**Returns:** `Boolean` — A boolean value indicating whether the operation succeeded.

[Docs](https://developer.tekla.com/topic/en/18/47/d5655b28-ad76-fd8b-61ac-68447343ae39)

### Properties
- `CurrentUnitType` (Distance.UnitType, get/set) — Gets or sets the current unit type.
- `Millimeters` (Double, get/set) — Gets or sets the distance in millimeters.
- `UseFractionalFormat` (Boolean, get/set) — Gets or sets a boolean value indicating whether to use the fractional format for US imperial units.
- `UseUnitsInDecimalString` (Boolean, get/set) — Gets or sets a boolean value indicating whether to use units in the decimal string representation.
- `Value` (Double, get/set) — Gets or sets the distance value in current units.

## Distance.UnitType (enum)

The distance units.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/2410add6-9fcc-35cd-ef9e-0468934d8a3e)

### Values
- `Millimeter` = `0` — The millimeter unit type.
- `Centimeter` = `1` — The centimeter unit type.
- `Meter` = `2` — The meter unit type.
- `Inch` = `3` — The inch unit type.
- `Foot` = `4` — The foot unit type.
- `UsSurveyInch` = `5` — The US survey inch unit type.
- `UsSurveyFoot` = `6` — The US survey foot unit type.

## DistanceConverter (class)

The DistanceConverter class converts types to and from the distance type.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/40392776-f460-001c-0eed-dfee702cb672)

### Constructors
- `public DistanceConverter()` — Initializes a new instance of the DistanceConverter class

### Methods
#### `public virtual bool CanConvertFrom(ITypeDescriptorContext context, Type sourceType)`

Checks whether the conversion can be made from the source type to the distance type.

**Parameters:**
- `context` (System.ComponentModel.ITypeDescriptorContext) — The context.
- `sourceType` (System.Type) — The type to convert from.

**Returns:** `Boolean` — True if possible, false if not possible.

[Docs](https://developer.tekla.com/topic/en/18/47/19293faf-2238-0f03-fcb5-aef2c762d440)

#### `public virtual bool CanConvertTo(ITypeDescriptorContext context, Type destinationType)`

Checks whether the conversion can be made from the distance type to the destination type.

**Parameters:**
- `context` (System.ComponentModel.ITypeDescriptorContext) — The context.
- `destinationType` (System.Type) — The type to convert to.

**Returns:** `Boolean` — True if possible, false if not possible.

[Docs](https://developer.tekla.com/topic/en/18/47/f5117fd1-eff2-5925-d971-3b27704b4a1b)

#### `public virtual Object ConvertFrom(ITypeDescriptorContext context, CultureInfo culture, Object value)`

Converts an object from the given culture to the distance type.

**Parameters:**
- `context` (System.ComponentModel.ITypeDescriptorContext) — The context.
- `culture` (System.Globalization.CultureInfo) — The culture.
- `value` (System.Object) — The object to be converted.

**Returns:** `Object` — The new distance object.

[Docs](https://developer.tekla.com/topic/en/18/47/c4b60749-7c53-6d4d-40bb-df1eebed715f)

#### `public virtual Object ConvertTo(ITypeDescriptorContext context, CultureInfo culture, Object value, Type destinationType)`

Converts an object from the distance type to the given destination type.

**Parameters:**
- `context` (System.ComponentModel.ITypeDescriptorContext) — The context.
- `culture` (System.Globalization.CultureInfo) — The target culture.
- `value` (System.Object) — The distance object to be converted.
- `destinationType` (System.Type) — The destination type.

**Returns:** `Object` — The given distance object converted to the new type.

[Docs](https://developer.tekla.com/topic/en/18/47/6291a0b1-0d57-c9d9-069f-a1bcf83f8de2)

## DistanceList (struct)

A list of Distance instances.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/2fbe454f-df7b-4de0-73ba-5d047f97fb14)

### Constructors
- `public DistanceList(IEnumerable<Distance> distanceList)` — Initializes a new instance of the structure.

### Methods
#### `public void Add(Distance item)`

Adds the specified item at the end of the list.

**Parameters:**
- `item` (Tekla.Structures.Datatype.Distance) — The item to be added.

[Docs](https://developer.tekla.com/topic/en/18/47/bf13e3d5-ae24-ff62-204f-175bd3cacc02)

#### `public void Clear()`

Clears the list.

[Docs](https://developer.tekla.com/topic/en/18/47/afc5521f-d3e1-de9a-b868-bae3fda01ee3)

#### `public bool Contains(Distance item)`

Determines whether the specified item is in the list.

**Parameters:**
- `item` (Tekla.Structures.Datatype.Distance) — The item to be matched.

**Returns:** `Boolean` — A boolean value indicating whether the item was found.

[Docs](https://developer.tekla.com/topic/en/18/47/a7eb2564-de96-72bb-885f-a8559eb327d3)

#### `public void CopyTo(Distance[] array, int arrayIndex)`

Copies the contents of the list to an array.

**Parameters:**
- `array` (.Tekla.Structures.Datatype.Distance.) — The target array.
- `arrayIndex` (System.Int32) — The starting index in the array.

[Docs](https://developer.tekla.com/topic/en/18/47/1e7381e6-ec7f-978e-2bf0-944d90d6f568)

#### `public IEnumerator<Distance> GetEnumerator()`

Gets an enumerator for enumeraring through the list.

**Returns:** `IEnumerator<Distance>` — The list enumerator.

[Docs](https://developer.tekla.com/topic/en/18/47/c8334d58-61bd-20a1-7bbc-c802b4530bf4)

#### `public int IndexOf(Distance item)`

Returns the index of the first matching item in the list.

**Parameters:**
- `item` (Tekla.Structures.Datatype.Distance) — The item to be matched.

**Returns:** `Int32` — The index of the first matching item.

[Docs](https://developer.tekla.com/topic/en/18/47/cfebb454-880d-bb2c-c3bc-80da5bdcf887)

#### `public void Insert(int index, Distance item)`

Inserts an item at the specified position.

**Parameters:**
- `index` (System.Int32) — The index of the insertion point.
- `item` (Tekla.Structures.Datatype.Distance) — The item to be inserted.

[Docs](https://developer.tekla.com/topic/en/18/47/4c4fde84-17b9-e6ee-8b5a-d737b8f72762)

#### `public static DistanceList Parse(string input)`

Parses a distance list from a string representation.

**Parameters:**
- `input` (System.String) — The input string.

**Returns:** `DistanceList` — The distance list.

[Docs](https://developer.tekla.com/topic/en/18/47/e98067a1-3b05-1a66-470a-7bceeaa89f26)

#### `public static DistanceList Parse(string input, IFormatProvider formatProvider)`

Parses a distance list from a string representation.

**Parameters:**
- `input` (System.String) — The input string.
- `formatProvider` (System.IFormatProvider) — The format provider.

**Returns:** `DistanceList` — The distance list.

[Docs](https://developer.tekla.com/topic/en/18/47/5323b1c1-d6f2-a324-aff2-c4e46dd56b64)

#### `public static DistanceList Parse(string input, IFormatProvider formatProvider, Distance.UnitType defaultUnitType)`

Parses a distance list from a string representation.

**Parameters:**
- `input` (System.String) — The input string.
- `formatProvider` (System.IFormatProvider) — The format provider.
- `defaultUnitType` (Tekla.Structures.Datatype.Distance.UnitType) — The default unit type.

**Returns:** `DistanceList` — The distance list.

[Docs](https://developer.tekla.com/topic/en/18/47/fb31c67b-d91e-2135-3f35-665be8a14ac4)

#### `public bool Remove(Distance item)`

Removes the specified item.

**Parameters:**
- `item` (Tekla.Structures.Datatype.Distance) — The item to be removed.

**Returns:** `Boolean` — A boolean value indicating whether the item was removed.

[Docs](https://developer.tekla.com/topic/en/18/47/b575565e-2a57-74a6-430a-3cf350d5afcc)

#### `public void RemoveAt(int index)`

Removes an item at the specified position.

**Parameters:**
- `index` (System.Int32) — The index of the item to be removed.

[Docs](https://developer.tekla.com/topic/en/18/47/12c876a0-de2d-2184-26b6-8c577f8c2399)

#### `public Distance[] ToArray()`

Copies the contents of the list into a new array.

**Returns:** `.Distance.` — The array of distances.

[Docs](https://developer.tekla.com/topic/en/18/47/6ef2b2bb-b79d-0ed9-503f-4ad56dc48d86)

#### `public string ToString(IFormatProvider formatProvider)`

Returns the string representation of the distance list.

**Parameters:**
- `formatProvider` (System.IFormatProvider) — The format provider.

**Returns:** `String` — The string representation of the distance list.

[Docs](https://developer.tekla.com/topic/en/18/47/22629370-6513-8468-d624-a238527e203a)

#### `public string ToString(string format, IFormatProvider formatProvider)`

Returns the string representation of the distance list.

**Parameters:**
- `format` (System.String) — The format string.
- `formatProvider` (System.IFormatProvider) — The format provider.

**Returns:** `String` — The string representation of the distance list.

[Docs](https://developer.tekla.com/topic/en/18/47/24c2f2b9-1ac5-2ea2-0b24-a3e4e8721482)

#### `public string ToString(string format, IFormatProvider formatProvider, Distance.UnitType unitType)`

Returns the string representation of the distance list.

**Parameters:**
- `format` (System.String) — The format string.
- `formatProvider` (System.IFormatProvider) — The format provider.
- `unitType` (Tekla.Structures.Datatype.Distance.UnitType) — The distance unit type.

**Returns:** `String` — The string representation of the distance list.

[Docs](https://developer.tekla.com/topic/en/18/47/5dc71ac4-54d5-1b7b-0d0a-f0ddc37512d3)

#### `public override string ToString()`

Returns the string representation of the distance list.

**Returns:** `String` — The string representation of the distance list.

[Docs](https://developer.tekla.com/topic/en/18/47/56e41342-c57e-ce1b-4f39-0fa1ffa9d9a8)

### Properties
- `Count` (Int32, get) — Gets the number of items in the list.
- `IsReadOnly` (Boolean, get) — Gets a boolean value indicating whether the collection is read-only.
- `Item` (object, get/set) — Gets or sets the item at the specified index.

## DistanceListConverter (class)

The DistanceListConverter class converts types to and from the distance list type.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/79b0f524-b30b-9529-2b3a-cc7bedde4289)

### Constructors
- `public DistanceListConverter()` — Initializes a new instance of the DistanceListConverter class

### Methods
#### `public virtual bool CanConvertFrom(ITypeDescriptorContext context, Type sourceType)`

Checks whether the conversion can be made from the source type to the distance list type.

**Parameters:**
- `context` (System.ComponentModel.ITypeDescriptorContext) — The context.
- `sourceType` (System.Type) — The type to convert from.

**Returns:** `Boolean` — True if possible, false if not possible.

[Docs](https://developer.tekla.com/topic/en/18/47/250881a7-3803-8251-d282-682d3d36bbf4)

#### `public virtual bool CanConvertTo(ITypeDescriptorContext context, Type destinationType)`

Checks whether the conversion can be made from the distance list type to the destination type.

**Parameters:**
- `context` (System.ComponentModel.ITypeDescriptorContext) — The context.
- `destinationType` (System.Type) — The type to convert to.

**Returns:** `Boolean` — True if possible, false if not possible.

[Docs](https://developer.tekla.com/topic/en/18/47/80ab11a8-d1a0-bf20-ee5c-37ec607b61a8)

#### `public virtual Object ConvertFrom(ITypeDescriptorContext context, CultureInfo culture, Object value)`

Converts an object from the given culture to the distance list type.

**Parameters:**
- `context` (System.ComponentModel.ITypeDescriptorContext) — The context.
- `culture` (System.Globalization.CultureInfo) — The culture.
- `value` (System.Object) — The object to be converted.

**Returns:** `Object` — The new distance object.

[Docs](https://developer.tekla.com/topic/en/18/47/ea469854-98c0-fe1a-9d60-d55ad38a123f)

#### `public virtual Object ConvertTo(ITypeDescriptorContext context, CultureInfo culture, Object value, Type destinationType)`

Converts an object from the distance list type to the given destination type.

**Parameters:**
- `context` (System.ComponentModel.ITypeDescriptorContext) — The context.
- `culture` (System.Globalization.CultureInfo) — The target culture.
- `value` (System.Object) — The distance object to be converted.
- `destinationType` (System.Type) — The destination type.

**Returns:** `Object` — The given distance object converted to the new type.

[Docs](https://developer.tekla.com/topic/en/18/47/5b0d37bf-9fc8-5611-2ae2-952e4c48aaad)

## Double (struct)

The Double datatype.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/ca14f3fd-a7b9-8563-7a7d-a83aed85d51d)

### Constructors
- `public Double(double value)` — Creates a new double datatype instance.

### Methods
#### `public int CompareTo(Double other)`

Compares the current object with another object of the same type.

**Parameters:**
- `other` (Tekla.Structures.Datatype.Double) — The object to be compared with the current object.

**Returns:** `Int32` — A 32-bit signed integer that indicates the relative order of the objects being compared.The return value has the following meanings:ValueMeaningLess than zeroThe current object is less than the other object.ZeroThe current object is equal to the other object.Greater than zeroThe current object is greater than the other object.

[Docs](https://developer.tekla.com/topic/en/18/47/b08d6a92-be92-cd4a-8350-e676c9c6880c)

#### `public bool Equals(Double other)`

Indicates whether the current object is equal to another object of the same type.

**Parameters:**
- `other` (Tekla.Structures.Datatype.Double) — The object to be compared with the current object.

**Returns:** `Boolean` — True if the current object is equal to the other object; otherwise, false.

[Docs](https://developer.tekla.com/topic/en/18/47/320dd7c1-7183-dd8c-677c-653a8b5cd42c)

#### `public override bool Equals(Object Obj)`

Returns a value that indicates whether the current instance is equal to the given object.

**Parameters:**
- `Obj` (System.Object) — The object to be used for comparing.

**Returns:** `Boolean` — True if the given object has the same value as the current instance.

[Docs](https://developer.tekla.com/topic/en/18/47/60172094-34b3-d3b4-a3c7-639ac7509369)

#### `public override int GetHashCode()`

Returns the hash code for the current instance.

**Returns:** `Int32` — A hash code for the current Double instance.

[Docs](https://developer.tekla.com/topic/en/18/47/dd889a84-a1e3-a0f8-babe-c1f361a973a8)

#### `public XmlSchema GetSchema()`

Gets the XML Schema for the datatype.

**Returns:** `XmlSchema` — The datatype's XML Schema.

[Docs](https://developer.tekla.com/topic/en/18/47/5a05a043-dcfb-bcdb-8d58-51e3f782e168)

#### `public void ReadXml(XmlReader reader)`

Parses a new value from the given reader.

**Parameters:**
- `reader` (System.Xml.XmlReader) — The reader to be used.

[Docs](https://developer.tekla.com/topic/en/18/47/b88641ca-e4d7-28b6-cb2e-8132d8c2e4f0)

#### `public override string ToString()`

Returns the string representation of the object.

**Returns:** `String` — The string representation of the object.

[Docs](https://developer.tekla.com/topic/en/18/47/7f80b0ef-dfc8-3714-4fd0-25a8f8d1d72d)

#### `public void WriteXml(XmlWriter writer)`

Writes the value as XML to the given writer.

**Parameters:**
- `writer` (System.Xml.XmlWriter) — The writer to be used.

[Docs](https://developer.tekla.com/topic/en/18/47/fffa87a4-e597-9339-09eb-e4f6555929c7)

### Properties
- `Value` (Double, get/set) — The value of the double datatype.

## DoubleConverter (class)

The DoubleConverter class converts types to and from the double datatype.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/30f3a075-d5b1-6d35-de48-d8fa193902db)

### Constructors
- `public DoubleConverter()` — Initializes a new instance of the DoubleConverter class

### Methods
#### `public virtual bool CanConvertFrom(ITypeDescriptorContext context, Type sourceType)`

Checks whether the conversion can be made from the source type to the double datatype.

**Parameters:**
- `context` (System.ComponentModel.ITypeDescriptorContext) — The context.
- `sourceType` (System.Type) — The type to convert from.

**Returns:** `Boolean` — True if possible, false if not possible.

[Docs](https://developer.tekla.com/topic/en/18/47/74518166-0416-0497-a7a5-288b483a03fc)

#### `public virtual bool CanConvertTo(ITypeDescriptorContext context, Type destinationType)`

Checks whether the conversion can be made from the double datatype to the destination type.

**Parameters:**
- `context` (System.ComponentModel.ITypeDescriptorContext) — The context.
- `destinationType` (System.Type) — The type to convert to.

**Returns:** `Boolean` — True if possible, false if not possible.

[Docs](https://developer.tekla.com/topic/en/18/47/a9a810f2-8ce1-3433-84b9-172a5a4e404f)

#### `public virtual Object ConvertFrom(ITypeDescriptorContext context, CultureInfo culture, Object value)`

Converts an object from the given culture to the Tekla.Structures.Datatype.Double type.

**Parameters:**
- `context` (System.ComponentModel.ITypeDescriptorContext) — The context.
- `culture` (System.Globalization.CultureInfo) — The culture.
- `value` (System.Object) — The object to be converted.

**Returns:** `Object` — The new double datatype object.

[Docs](https://developer.tekla.com/topic/en/18/47/ed867658-ddc3-7673-ad31-67e255099ac7)

#### `public virtual Object ConvertTo(ITypeDescriptorContext context, CultureInfo culture, Object value, Type destinationType)`

Converts an object from the double datatype to the given destination type.

**Parameters:**
- `context` (System.ComponentModel.ITypeDescriptorContext) — The context.
- `culture` (System.Globalization.CultureInfo) — The target culture.
- `value` (System.Object) — The double datatype object to be converted.
- `destinationType` (System.Type) — The destination type.

**Returns:** `Object` — The given double datatype object converted to the new type.

[Docs](https://developer.tekla.com/topic/en/18/47/53a8a97e-d5a8-0aee-6090-39a9c125dbe7)

## Enum.E. (struct)

The Enum datatype generic.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/6d6e6a90-123e-b7b7-8c1c-28582ecc1119)

### Methods
#### `public XmlSchema GetSchema()`



**Returns:** `XmlSchema` — 

[Docs](https://developer.tekla.com/topic/en/18/47/27ddc2df-2642-c422-aea0-63db5fc74037)

#### `public void ReadXml(XmlReader reader)`



**Parameters:**
- `reader` (System.Xml.XmlReader)

[Docs](https://developer.tekla.com/topic/en/18/47/8cd9e291-2d2d-6178-3366-05fde4a64bc2)

#### `public void WriteXml(XmlWriter writer)`



**Parameters:**
- `writer` (System.Xml.XmlWriter)

[Docs](https://developer.tekla.com/topic/en/18/47/777144d9-9d41-3453-fb21-7af2b6baf9c8)

### Properties
- `Value` (E, get/set) — Gets or sets the value.

## EnumConverter (class)

Type converter for Enum

[Vendor docs](https://developer.tekla.com/topic/en/18/47/dd7d11a6-800a-0ad4-445e-2a071b3fa1a9)

### Constructors
- `public EnumConverter()` — Initializes a new instance of the EnumConverter class

### Methods
#### `public virtual bool CanConvertFrom(ITypeDescriptorContext context, Type sourceType)`



**Parameters:**
- `context` (System.ComponentModel.ITypeDescriptorContext)
- `sourceType` (System.Type)

**Returns:** `Boolean` — 

[Docs](https://developer.tekla.com/topic/en/18/47/0b0bd085-59fe-234a-332e-91c6575ba2c8)

#### `public virtual bool CanConvertTo(ITypeDescriptorContext context, Type destinationType)`



**Parameters:**
- `context` (System.ComponentModel.ITypeDescriptorContext)
- `destinationType` (System.Type)

**Returns:** `Boolean` — 

[Docs](https://developer.tekla.com/topic/en/18/47/7276eb74-bf2f-3369-41b7-63cc47940877)

#### `public virtual Object ConvertFrom(ITypeDescriptorContext context, CultureInfo culture, Object value)`



**Parameters:**
- `context` (System.ComponentModel.ITypeDescriptorContext)
- `culture` (System.Globalization.CultureInfo)
- `value` (System.Object)

**Returns:** `Object` — 

[Docs](https://developer.tekla.com/topic/en/18/47/dc16d26e-bf17-b038-d170-aa132b6b34db)

#### `public virtual Object ConvertTo(ITypeDescriptorContext context, CultureInfo culture, Object value, Type destinationType)`



**Parameters:**
- `context` (System.ComponentModel.ITypeDescriptorContext)
- `culture` (System.Globalization.CultureInfo)
- `value` (System.Object)
- `destinationType` (System.Type)

**Returns:** `Object` — 

[Docs](https://developer.tekla.com/topic/en/18/47/726ddaea-16ca-37a5-75f1-1d4f0fdfe98d)

## IDataType (interface)

The IDataType interface is the root interface for converting all the Tekla.Structures.Datatype.* types to basic types supported by the Tekla Structures Object Dialog tree.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/9dad6c8c-50eb-7195-bdce-0f1e40ff3c26)

## IDoubleDataType (interface)

The IDoubleDataType interface is an interface for the Tekla.Structures.Datatype.* types whose value is a double.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/3efde7c8-1ded-89ec-e787-dea30c5a9dc8)

### Properties
- `Value` (Double, get/set) — The double value of the field.

## IIntDataType (interface)

The IIntDataType interface is an interface for the Tekla.Structures.Datatype.* types whose value is an integer.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/bc1bc7c1-9f61-cd7d-99f7-c672f396f53b)

### Properties
- `Value` (Int32, get/set) — The integer value of the field.

## IStringDataType (interface)

The IStringDataType interface is an interface for the Tekla.Structures.Datatype.* types whose value is a string.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/1e85ced4-506f-5a31-6997-b767a2749d02)

### Properties
- `Value` (String, get/set) — The string value of the field.

## Integer (struct)

The Integer datatype.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/6e7e0138-f3f9-f260-9930-c6b25f7583f5)

### Constructors
- `public Integer(int value)` — Creates a new integer datatype instance.

### Methods
#### `public override bool Equals(Object Obj)`

Returns a value that indicates whether the current instance is equal to the given object.

**Parameters:**
- `Obj` (System.Object) — The object to be used for comparing.

**Returns:** `Boolean` — True if the given object has the same value as the current instance.

[Docs](https://developer.tekla.com/topic/en/18/47/489c427b-05b2-2899-23b8-a3624cdb0da1)

#### `public override int GetHashCode()`

Returns the hash code for the current instance.

**Returns:** `Int32` — A hash code for the current Integer instance.

[Docs](https://developer.tekla.com/topic/en/18/47/e917eafd-fa7d-e5b4-fad3-80b1c3c687e0)

#### `public XmlSchema GetSchema()`

Gets the XML Schema for the datatype.

**Returns:** `XmlSchema` — The datatype's XML Schema.

[Docs](https://developer.tekla.com/topic/en/18/47/83a934e9-65a0-6c88-5f97-62f63585da52)

#### `public void ReadXml(XmlReader reader)`

Parses a new value from the given reader.

**Parameters:**
- `reader` (System.Xml.XmlReader) — The reader to be used.

[Docs](https://developer.tekla.com/topic/en/18/47/2f7b1133-4d5f-175b-7a6f-cdf19e170b80)

#### `public override string ToString()`

Returns the string representation of the object.

**Returns:** `String` — The string representation of the object.

[Docs](https://developer.tekla.com/topic/en/18/47/ba4f28f3-495d-66ed-d106-f2000d8623db)

#### `public void WriteXml(XmlWriter writer)`

Writes the value as XML to the given writer.

**Parameters:**
- `writer` (System.Xml.XmlWriter) — The writer to be used.

[Docs](https://developer.tekla.com/topic/en/18/47/61f951c6-ffaf-b2a1-6ddc-851a5601c3d8)

### Properties
- `Value` (Int32, get/set) — The value of the integer datatype.

## IntegerConverter (class)

The IntegerConverter class converts types to and from the integer datatype.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/83ff6d0a-70a0-8a03-3514-624cf9605045)

### Constructors
- `public IntegerConverter()` — Initializes a new instance of the IntegerConverter class

### Methods
#### `public virtual bool CanConvertFrom(ITypeDescriptorContext context, Type sourceType)`

Checks whether the conversion can be made from the source type to the integer datatype.

**Parameters:**
- `context` (System.ComponentModel.ITypeDescriptorContext) — The context.
- `sourceType` (System.Type) — The type to convert from.

**Returns:** `Boolean` — True if possible, false if not possible.

[Docs](https://developer.tekla.com/topic/en/18/47/ec34b138-b93b-bf83-35e2-54169429e20e)

#### `public virtual bool CanConvertTo(ITypeDescriptorContext context, Type destinationType)`

Checks whether the conversion can be made from the integer datatype to the destination type.

**Parameters:**
- `context` (System.ComponentModel.ITypeDescriptorContext) — The context.
- `destinationType` (System.Type) — The type to convert to.

**Returns:** `Boolean` — True if possible, false if not possible.

[Docs](https://developer.tekla.com/topic/en/18/47/ef525f1e-d054-144f-636d-85ae260b1331)

#### `public virtual Object ConvertFrom(ITypeDescriptorContext context, CultureInfo culture, Object value)`

Converts an object from the given culture to the Tekla.Structures.Datatype.Integer type.

**Parameters:**
- `context` (System.ComponentModel.ITypeDescriptorContext) — The context.
- `culture` (System.Globalization.CultureInfo) — The culture.
- `value` (System.Object) — The object to be converted.

**Returns:** `Object` — The new integer datatype object.

[Docs](https://developer.tekla.com/topic/en/18/47/d035f709-44b7-afc5-cd49-545314828813)

#### `public virtual Object ConvertTo(ITypeDescriptorContext context, CultureInfo culture, Object value, Type destinationType)`

Converts an object from the the integer datatype to the given destination type.

**Parameters:**
- `context` (System.ComponentModel.ITypeDescriptorContext) — The context.
- `culture` (System.Globalization.CultureInfo) — The target culture.
- `value` (System.Object) — The integer datatype object to be converted.
- `destinationType` (System.Type) — The destination type.

**Returns:** `Object` — The given integer datatype object converted to the new type.

[Docs](https://developer.tekla.com/topic/en/18/47/b7f633f9-8cf6-0241-57f1-8009d859b6c9)

## Settings (class)

The Settings class contains the currently active unit settings.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/9485d590-5b71-ebb9-13fb-8f55de9337ab)

### Constructors
- `public Settings()` — Initializes a new instance of the Settings class

### Methods
#### `public static Object GetValue(string name)`

Gets the value of the setting with the given name.

**Parameters:**
- `name` (System.String) — The name of the setting.

**Returns:** `Object` — The value of the given setting.

[Docs](https://developer.tekla.com/topic/en/18/47/0db94c0d-502a-97b2-51df-d351b58524c4)

#### `public static void SetValue(string name, Object value)`

Sets a setting to a value.

**Parameters:**
- `name` (System.String) — The name of the setting.
- `value` (System.Object) — The new value for the setting.

[Docs](https://developer.tekla.com/topic/en/18/47/2903c000-7c5f-df96-9a7f-9556d992d4aa)

#### `public static bool TryGetValue(string name, out Object obj)`

Tries to get the value of the setting with the given name.

**Parameters:**
- `name` (System.String) — The name of the setting.
- `obj` (System.Object.) — The value of the given setting.

**Returns:** `Boolean` — True on success, false otherwise.

[Docs](https://developer.tekla.com/topic/en/18/47/9ebbbc0c-626d-6f2b-2c3f-e58c9509bcf5)

#### `public static bool TryGetValue<T>(string name, out T obj)`

Tries to get the value of the setting with the given name and given object type.

**Parameters:**
- `name` (System.String) — The name of the setting.
- `obj` (T.) — The value of the given setting.

**Returns:** `Boolean` — True on success, false otherwise.

[Docs](https://developer.tekla.com/topic/en/18/47/c48529be-f7c8-5b08-34fe-33491cb04597)

## SettingsProxy (class)

The SettingsProxy class is a proxy for the settings.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/4f296ae4-7b22-7536-4d51-6f0c786b1f24)

### Constructors
- `public SettingsProxy()` — Creates a new settings proxy instance.

### Methods
#### `public Object GetValue(string name)`

Gets the value of the setting with the given name.

**Parameters:**
- `name` (System.String) — The name of the setting.

**Returns:** `Object` — The value of the given setting.

[Docs](https://developer.tekla.com/topic/en/18/47/aff544dc-6a76-16cc-fa9d-00deef9d60b8)

#### `public void SetValue(string name, Object value)`

Sets a setting to a value.

**Parameters:**
- `name` (System.String) — The name of the setting.
- `value` (System.Object) — The new value for the setting.

[Docs](https://developer.tekla.com/topic/en/18/47/2652bc46-1ae5-fcbb-f343-4960697413fc)

#### `public bool TryGetValue(string name, out Object obj)`

Tries to get the value of the setting with the given name.

**Parameters:**
- `name` (System.String) — The name of the setting.
- `obj` (System.Object.) — The value of the given setting.

**Returns:** `Boolean` — True on success, false otherwise.

[Docs](https://developer.tekla.com/topic/en/18/47/8e499dc9-15ec-89fa-013f-82afbab6ec18)

## String (struct)

The String datatype.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/c6f0455d-cb51-99fe-67f7-a6911dbc4728)

### Constructors
- `public String(string text)` — Creates a new string datatype instance.

### Methods
#### `public override bool Equals(Object Obj)`

Returns a value that indicates whether the current instance is equal to the given object.

**Parameters:**
- `Obj` (System.Object) — The object to be used for comparing.

**Returns:** `Boolean` — True if the given object has the same value as the current instance.

[Docs](https://developer.tekla.com/topic/en/18/47/4a48baa2-e08b-c323-9af4-bc85d98374c7)

#### `public override int GetHashCode()`

Returns the hash code for the current instance.

**Returns:** `Int32` — A hash code for the current String instance.

[Docs](https://developer.tekla.com/topic/en/18/47/9b470039-311f-4fef-091e-1020fecbd875)

#### `public XmlSchema GetSchema()`

Gets the XML Schema for the datatype.

**Returns:** `XmlSchema` — The datatype's XML Schema.

[Docs](https://developer.tekla.com/topic/en/18/47/df3587f8-158f-a003-199a-fcd09507d3bc)

#### `public void ReadXml(XmlReader reader)`

Parses a new value from the given reader.

**Parameters:**
- `reader` (System.Xml.XmlReader) — The reader to be used.

[Docs](https://developer.tekla.com/topic/en/18/47/dc6b9b85-747e-b9db-a1ea-90973214c356)

#### `public override string ToString()`

Returns the string representation of the object.

**Returns:** `String` — The string representation of the object.

[Docs](https://developer.tekla.com/topic/en/18/47/81162790-c961-4843-fd25-cf51887f820f)

#### `public void WriteXml(XmlWriter writer)`

Writes the value as XML to the given writer.

**Parameters:**
- `writer` (System.Xml.XmlWriter) — The writer to be used.

[Docs](https://developer.tekla.com/topic/en/18/47/c05d5763-61c6-b80c-87d1-95e9d1b985aa)

### Properties
- `Value` (String, get/set) — The value of the string datatype.

## StringConverter (class)

The StringConverter class converts types to and from the string datatype.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/5c2009b2-b234-6874-1d25-602eccc15f40)

### Constructors
- `public StringConverter()` — Initializes a new instance of the StringConverter class

### Methods
#### `public virtual bool CanConvertFrom(ITypeDescriptorContext context, Type sourceType)`

Checks whether the conversion can be made from the source type to the string datatype.

**Parameters:**
- `context` (System.ComponentModel.ITypeDescriptorContext) — The context.
- `sourceType` (System.Type) — The type to convert from.

**Returns:** `Boolean` — True if possible, false if not possible.

[Docs](https://developer.tekla.com/topic/en/18/47/9210c084-0285-2bc9-ca5e-56bb8145aa11)

#### `public virtual bool CanConvertTo(ITypeDescriptorContext context, Type destinationType)`

Checks whether the conversion can be made from the string datatype to the destination type.

**Parameters:**
- `context` (System.ComponentModel.ITypeDescriptorContext) — The context.
- `destinationType` (System.Type) — The type to convert to.

**Returns:** `Boolean` — True if possible, false if not possible.

[Docs](https://developer.tekla.com/topic/en/18/47/eba9b217-d5ac-841e-78e9-22ddccfbd516)

#### `public virtual Object ConvertFrom(ITypeDescriptorContext context, CultureInfo culture, Object value)`

Converts an object from the given culture to the Tekla.Structures.Datatype.String type.

**Parameters:**
- `context` (System.ComponentModel.ITypeDescriptorContext) — The context.
- `culture` (System.Globalization.CultureInfo) — The culture.
- `value` (System.Object) — The object to be converted.

**Returns:** `Object` — The new string datatype object.

[Docs](https://developer.tekla.com/topic/en/18/47/cdd83c5e-c1b2-749a-5d79-005757351f24)

#### `public virtual Object ConvertTo(ITypeDescriptorContext context, CultureInfo culture, Object value, Type destinationType)`

Converts an object from the string datatype to the given destination type.

**Parameters:**
- `context` (System.ComponentModel.ITypeDescriptorContext) — The context.
- `culture` (System.Globalization.CultureInfo) — The target culture.
- `value` (System.Object) — The string datatype object to be converted.
- `destinationType` (System.Type) — The destination type.

**Returns:** `Object` — The given string datatype object converted to the new type.

[Docs](https://developer.tekla.com/topic/en/18/47/a72e134e-3c4b-dd4a-a341-180a1a312248)

