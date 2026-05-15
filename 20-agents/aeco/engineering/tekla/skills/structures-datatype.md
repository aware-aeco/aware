---
name: tekla-structures-datatype
description: Tekla Open API - Tekla.Structures.Datatype namespace. Distance, Angle, unit conversion, millimeters, inches, feet, degrees, radians, measurement units, parsing, formatting, serialization. This skill should be used when working with Tekla measurement units, distances, angles, or unit conversions.
---

# Tekla.Structures.Datatype API Reference (v2025)

## Overview

Unit-aware data types for distances, angles, and primitives used throughout the Tekla API. These types handle unit conversion (mm/cm/m/in/ft, degrees/radians) and support parsing from locale-aware strings.

## Core Types

### Angle
> Provides support for angles using either the current Tekla Structures units, or user-defined units.

**Constructors:** None public (use static factory methods)

**Properties:**
- `double CurrentUnitValue { get; }` - value in the current unit
- `double Degrees { get; }` - value in degrees
- `double Radians { get; }` - value in radians

**Static Properties:**
- `Angle.UnitType CurrentUnitType { get; set; }`
- `int DecimalPlaces { get; set; }`

**Factory Methods:**
- `static Angle FromCurrentUnit(double angle)`
- `static Angle FromDegrees(double degrees)`
- `static Angle FromRadians(double radians)`

**Parse Methods:**
- `static Angle Parse(string text)`
- `static Angle Parse(string text, IFormatProvider formatProvider)`
- `static Angle Parse(string text, IFormatProvider formatProvider, UnitType unitType)`
- `static bool TryParse(string text, out Angle result)`
- `static bool TryParse(string text, IFormatProvider formatProvider, out Angle result)`
- `static bool TryParse(string text, IFormatProvider formatProvider, UnitType unitType, out Angle result)`

### Angle.UnitType (Enum)
| Value | Int | Description |
|-------|-----|-------------|
| Degrees | 0 | 0-360 range |
| Radians | 1 | 0-2*PI range |

---

### Distance
> The Distance datatype. Supports mm, cm, m, inch, foot, US survey units.

**Constructors:**
- `Distance(double millimeters)`
- `Distance(double distance, UnitType unitType)`

**Properties:**
- `double Millimeters { get; set; }` - distance in millimeters
- `double Value { get; set; }` - distance in current units

**Static Properties:**
- `Distance.UnitType CurrentUnitType { get; set; }`
- `bool UseFractionalFormat { get; set; }`
- `bool UseUnitsInDecimalString { get; set; }`

**Methods:**
- `int CompareTo(Distance other)`
- `double ConvertTo(UnitType unitType)`
- `string ToDecimalString()` / `ToDecimalString(string format)` / `ToDecimalString(IFormatProvider, UnitType)`
- `string ToFractionalFeetAndInchesString()` / `ToFractionalFeetAndInchesString(IFormatProvider)`
- `string ToFractionalInchesString()` / `ToFractionalInchesString(IFormatProvider)`

**Static Factory Methods:**
- `static Distance FromDecimalString(string text)` / with `IFormatProvider` / with `IFormatProvider, UnitType`
- `static Distance FromFractionalFeetAndInchesString(string text)` / with `IFormatProvider, UnitType`
- `static Distance Parse(string text)` / with `IFormatProvider` / with `IFormatProvider, UnitType`
- `static bool TryParse(string text, out Distance result)` / with `IFormatProvider` / with `IFormatProvider, UnitType`

### Distance.UnitType (Enum)
| Value | Int | Description |
|-------|-----|-------------|
| Millimeter | 0 | Millimeter unit |
| Centimeter | 1 | Centimeter unit |
| Meter | 2 | Meter unit |
| Inch | 3 | Inch unit |
| Foot | 4 | Foot unit |
| UsSurveyInch | 5 | US survey inch |
| UsSurveyFoot | 6 | US survey foot |

---

### DistanceList
> List of Distance values with parsing and serialization support.

**Constructors:** `DistanceList(IEnumerable<Distance> distanceList)`

**Properties:**
- `int Count { get; }`
- `bool IsReadOnly { get; }`
- `Distance this[int index] { get; set; }`

**Methods:**
- `void Add(Distance item)`, `void Clear()`, `bool Contains(Distance item)`
- `void Insert(int index, Distance item)`, `bool Remove(Distance item)`, `void RemoveAt(int index)`
- `int IndexOf(Distance item)`, `void CopyTo(Distance[] array, int arrayIndex)`
- `Distance[] ToArray()`
- `IEnumerator<Distance> GetEnumerator()`

**Static Parse Methods:**
- `static DistanceList Parse(string input)` / with `IFormatProvider` / with `IFormatProvider, UnitType`

---

### AngleList (Static)
> Methods to handle lists of angles.

- `static List<Angle> Parse(string text)` / with `IFormatProvider` / with `IFormatProvider, UnitType`
- `static string ToString(IEnumerable<Angle> angleList)` / with `UnitType` / with `string format, IFormatProvider`

## Primitive Wrappers

These wrap basic types for Tekla's XML serialization system:

| Type | Value Property | Description |
|------|---------------|-------------|
| `Boolean` (sealed) | `bool Value { get; set; }` | Boolean datatype wrapper |
| `Double` (sealed) | `double Value { get; set; }` | Double datatype wrapper |
| `Integer` (sealed) | `int Value { get; set; }` | Integer datatype wrapper |
| `String` (sealed) | `string Value { get; set; }` | String datatype wrapper |
| `Enum<E>` (sealed) | `E Value { get; set; }` | Generic enum wrapper |

Each has a constructor taking the wrapped value and implements `IDataType` + `IXmlSerializable`.

## Interfaces

| Interface | Property | Description |
|-----------|----------|-------------|
| `IDataType` | (none) | Root interface for all Tekla datatype wrappers |
| `IDoubleDataType` : IDataType | `double Value { get; set; }` | Double-valued types |
| `IIntDataType` : IDataType | `int Value { get; set; }` | Integer-valued types |
| `IStringDataType` : IDataType | `string Value { get; set; }` | String-valued types |

## Settings

### Settings (Static Methods)
> Contains the currently active unit settings.

- `static object GetValue(string name)`
- `static void SetValue(string name, object value)`
- `static bool TryGetValue(string name, out object obj)`
- `static bool TryGetValue<T>(string name, out T obj)`

## Type Converters

All converters extend `TypeConverter` and support `CanConvertFrom`, `CanConvertTo`, `ConvertFrom`, `ConvertTo`:

| Converter | Converts |
|-----------|----------|
| `BooleanConverter` | Boolean type |
| `DistanceConverter` | Distance type |
| `DistanceListConverter` | DistanceList type |
| `DoubleConverter` | Double type |
| `EnumConverter` | Enum type |
| `IntegerConverter` | Integer type |
| `StringConverter` | String type |
