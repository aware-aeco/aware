---
name: tekla-plugin-sdk-fusion-formatting
description: This skill encodes the tekla-plugin-sdk 2025.0 surface of the Fusion.Formatting namespace — 34 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AngleFormatter, AreaFormatter, AreaPerLengthFormatter, DateTimeFormatter, DensityFormatter, DoubleFormatter, DynamicViscosityFormatter, FlowCoefficientFormatter, and 26 more types.
---

# Fusion.Formatting

Auto-generated from vendor docs for tekla-plugin-sdk 2025.0. 34 types in this namespace.

## AngleFormatter (class)

AngleFormatter class in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.AngleFormatter)

### Constructors
- `AngleFormatter()` — Constructs a new AngleFormatter.
- `AngleFormatter(Fusion.AngleUnit unit)` — Constructs a new AngleFormatter.
- `AngleFormatter(Fusion.AngleUnit unit, Fusion.AngleUnit formatUnit)` — Constructs a new AngleFormatter.
- `AngleFormatter(Fusion.AngleUnit unit, Fusion.AngleUnit formatUnit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new AngleFormatter.
- `AngleFormatter(Fusion.AngleUnit unit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new AngleFormatter.
- `AngleFormatter(string formatString, System.IFormatProvider formatProvider)` — Constructs a new AngleFormatter.

### Methods
#### `static System.Collections.Generic.IDictionary<string, Fusion.Formatting.AngleFormatter> ReadSettings(System.Collections.Generic.IDictionary<string, string> settings)`

ReadSettings method of AngleFormatter.

**Parameters:**
- `settings` (System.Collections.Generic.IDictionary<string, string>)

**Returns:** `System.Collections.Generic.IDictionary<string, Fusion.Formatting.AngleFormatter>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.AngleFormatter.ReadSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

#### `string ToString(Fusion.Angle value, Fusion.Formatting.UnitSymbols symbols)`

ToString method of AngleFormatter.

**Parameters:**
- `value` (Fusion.Angle)
- `symbols` (Fusion.Formatting.UnitSymbols)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.AngleFormatter.ToString%28Fusion.Angle%2CFusion.Formatting.UnitSymbols%29)

#### `bool TryParse(string text, Fusion.Formatting.UnitSymbols symbols, out Fusion.Angle value)`

TryParse method of AngleFormatter.

**Parameters:**
- `text` (string)
- `symbols` (Fusion.Formatting.UnitSymbols)
- `value` (Fusion.Angle)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.AngleFormatter.TryParse%28System.String%2CFusion.Formatting.UnitSymbols%2CFusion.Angle%40%29)

#### `static void WriteSettings(System.Collections.Generic.IDictionary<string, Fusion.Formatting.AngleFormatter> formatters, System.Collections.Generic.IDictionary<string, string> settings)`

WriteSettings method of AngleFormatter.

**Parameters:**
- `formatters` (System.Collections.Generic.IDictionary<string, Fusion.Formatting.AngleFormatter>)
- `settings` (System.Collections.Generic.IDictionary<string, string>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.AngleFormatter.WriteSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CFusion.Formatting.AngleFormatter%7D%2CSystem.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

## AreaFormatter (class)

AreaFormatter class in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.AreaFormatter)

### Constructors
- `AreaFormatter()` — Constructs a new AreaFormatter.
- `AreaFormatter(Fusion.AreaUnit unit)` — Constructs a new AreaFormatter.
- `AreaFormatter(Fusion.AreaUnit unit, Fusion.AreaUnit formatUnit)` — Constructs a new AreaFormatter.
- `AreaFormatter(Fusion.AreaUnit unit, Fusion.AreaUnit formatUnit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new AreaFormatter.
- `AreaFormatter(Fusion.AreaUnit unit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new AreaFormatter.

### Methods
#### `static System.Collections.Generic.IDictionary<string, Fusion.Formatting.AreaFormatter> ReadSettings(System.Collections.Generic.IDictionary<string, string> settings)`

ReadSettings method of AreaFormatter.

**Parameters:**
- `settings` (System.Collections.Generic.IDictionary<string, string>)

**Returns:** `System.Collections.Generic.IDictionary<string, Fusion.Formatting.AreaFormatter>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.AreaFormatter.ReadSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

#### `string ToString(Fusion.Area value, Fusion.Formatting.UnitSymbols symbols)`

ToString method of AreaFormatter.

**Parameters:**
- `value` (Fusion.Area)
- `symbols` (Fusion.Formatting.UnitSymbols)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.AreaFormatter.ToString%28Fusion.Area%2CFusion.Formatting.UnitSymbols%29)

#### `bool TryParse(string text, Fusion.Formatting.UnitSymbols symbols, out Fusion.Area value)`

TryParse method of AreaFormatter.

**Parameters:**
- `text` (string)
- `symbols` (Fusion.Formatting.UnitSymbols)
- `value` (Fusion.Area)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.AreaFormatter.TryParse%28System.String%2CFusion.Formatting.UnitSymbols%2CFusion.Area%40%29)

#### `static void WriteSettings(System.Collections.Generic.IDictionary<string, Fusion.Formatting.AreaFormatter> formatters, System.Collections.Generic.IDictionary<string, string> settings)`

WriteSettings method of AreaFormatter.

**Parameters:**
- `formatters` (System.Collections.Generic.IDictionary<string, Fusion.Formatting.AreaFormatter>)
- `settings` (System.Collections.Generic.IDictionary<string, string>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.AreaFormatter.WriteSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CFusion.Formatting.AreaFormatter%7D%2CSystem.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

## AreaPerLengthFormatter (class)

AreaPerLengthFormatter class in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.AreaPerLengthFormatter)

### Constructors
- `AreaPerLengthFormatter()` — Constructs a new AreaPerLengthFormatter.
- `AreaPerLengthFormatter(Fusion.AreaPerLength.AreaPerLengthUnit unit)` — Constructs a new AreaPerLengthFormatter.
- `AreaPerLengthFormatter(Fusion.AreaPerLength.AreaPerLengthUnit unit, Fusion.AreaPerLength.AreaPerLengthUnit formatUnit)` — Constructs a new AreaPerLengthFormatter.
- `AreaPerLengthFormatter(Fusion.AreaPerLength.AreaPerLengthUnit unit, Fusion.AreaPerLength.AreaPerLengthUnit formatUnit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new AreaPerLengthFormatter.
- `AreaPerLengthFormatter(Fusion.AreaPerLength.AreaPerLengthUnit unit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new AreaPerLengthFormatter.

### Methods
#### `static System.Collections.Generic.IDictionary<string, Fusion.Formatting.AreaPerLengthFormatter> ReadSettings(System.Collections.Generic.IDictionary<string, string> settings)`

ReadSettings method of AreaPerLengthFormatter.

**Parameters:**
- `settings` (System.Collections.Generic.IDictionary<string, string>)

**Returns:** `System.Collections.Generic.IDictionary<string, Fusion.Formatting.AreaPerLengthFormatter>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.AreaPerLengthFormatter.ReadSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

#### `string ToString(Fusion.AreaPerLength value, Fusion.Formatting.UnitSymbols symbols)`

ToString method of AreaPerLengthFormatter.

**Parameters:**
- `value` (Fusion.AreaPerLength)
- `symbols` (Fusion.Formatting.UnitSymbols)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.AreaPerLengthFormatter.ToString%28Fusion.AreaPerLength%2CFusion.Formatting.UnitSymbols%29)

#### `bool TryParse(string text, Fusion.Formatting.UnitSymbols symbols, out Fusion.AreaPerLength value)`

TryParse method of AreaPerLengthFormatter.

**Parameters:**
- `text` (string)
- `symbols` (Fusion.Formatting.UnitSymbols)
- `value` (Fusion.AreaPerLength)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.AreaPerLengthFormatter.TryParse%28System.String%2CFusion.Formatting.UnitSymbols%2CFusion.AreaPerLength%40%29)

#### `static void WriteSettings(System.Collections.Generic.IDictionary<string, Fusion.Formatting.AreaPerLengthFormatter> formatters, System.Collections.Generic.IDictionary<string, string> settings)`

WriteSettings method of AreaPerLengthFormatter.

**Parameters:**
- `formatters` (System.Collections.Generic.IDictionary<string, Fusion.Formatting.AreaPerLengthFormatter>)
- `settings` (System.Collections.Generic.IDictionary<string, string>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.AreaPerLengthFormatter.WriteSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CFusion.Formatting.AreaPerLengthFormatter%7D%2CSystem.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

## DateTimeFormatter (class)

DateTimeFormatter class in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.DateTimeFormatter)

### Constructors
- `DateTimeFormatter()` — Constructs a new DateTimeFormatter.
- `DateTimeFormatter(System.Globalization.DateTimeStyles dateTimeStyle, string formatString, System.IFormatProvider formatProvider)` — Constructs a new DateTimeFormatter.

### Methods
#### `static System.Collections.Generic.IDictionary<string, Fusion.Formatting.DateTimeFormatter> ReadSettings(System.Collections.Generic.IDictionary<string, string> settings)`

ReadSettings method of DateTimeFormatter.

**Parameters:**
- `settings` (System.Collections.Generic.IDictionary<string, string>)

**Returns:** `System.Collections.Generic.IDictionary<string, Fusion.Formatting.DateTimeFormatter>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.DateTimeFormatter.ReadSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

#### `string ToString(System.DateTime value)`

ToString method of DateTimeFormatter.

**Parameters:**
- `value` (System.DateTime)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.DateTimeFormatter.ToString%28System.DateTime%29)

#### `bool TryParse(string text, out System.DateTime value)`

TryParse method of DateTimeFormatter.

**Parameters:**
- `text` (string)
- `value` (System.DateTime)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.DateTimeFormatter.TryParse%28System.String%2CSystem.DateTime%40%29)

#### `static void WriteSettings(System.Collections.Generic.IDictionary<string, Fusion.Formatting.DateTimeFormatter> formatters, System.Collections.Generic.IDictionary<string, string> settings)`

WriteSettings method of DateTimeFormatter.

**Parameters:**
- `formatters` (System.Collections.Generic.IDictionary<string, Fusion.Formatting.DateTimeFormatter>)
- `settings` (System.Collections.Generic.IDictionary<string, string>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.DateTimeFormatter.WriteSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CFusion.Formatting.DateTimeFormatter%7D%2CSystem.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

### Properties
- `DateTimeStyle` (System.Globalization.DateTimeStyles, get) — DateTimeStyle property of DateTimeFormatter.

## DensityFormatter (class)

DensityFormatter class in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.DensityFormatter)

### Constructors
- `DensityFormatter()` — Constructs a new DensityFormatter.
- `DensityFormatter(Fusion.DensityUnit unit)` — Constructs a new DensityFormatter.
- `DensityFormatter(Fusion.DensityUnit unit, Fusion.DensityUnit formatUnit)` — Constructs a new DensityFormatter.
- `DensityFormatter(Fusion.DensityUnit unit, Fusion.DensityUnit formatUnit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new DensityFormatter.

### Methods
#### `string ToString(double value, Fusion.Formatting.UnitSymbols symbols)`

ToString method of DensityFormatter.

**Parameters:**
- `value` (double)
- `symbols` (Fusion.Formatting.UnitSymbols)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.DensityFormatter.ToString%28System.Double%2CFusion.Formatting.UnitSymbols%29)

#### `bool TryParse(string text, Fusion.Formatting.UnitSymbols symbols, out double value)`

TryParse method of DensityFormatter.

**Parameters:**
- `text` (string)
- `symbols` (Fusion.Formatting.UnitSymbols)
- `value` (double)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.DensityFormatter.TryParse%28System.String%2CFusion.Formatting.UnitSymbols%2CSystem.Double%40%29)

## DoubleFormatter (class)

DoubleFormatter class in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.DoubleFormatter)

### Constructors
- `DoubleFormatter()` — Constructs a new DoubleFormatter.
- `DoubleFormatter(System.Globalization.NumberStyles numberStyle, string formatString, System.IFormatProvider formatProvider)` — Constructs a new DoubleFormatter.

### Methods
#### `static System.Collections.Generic.IDictionary<string, Fusion.Formatting.DoubleFormatter> ReadSettings(System.Collections.Generic.IDictionary<string, string> settings)`

ReadSettings method of DoubleFormatter.

**Parameters:**
- `settings` (System.Collections.Generic.IDictionary<string, string>)

**Returns:** `System.Collections.Generic.IDictionary<string, Fusion.Formatting.DoubleFormatter>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.DoubleFormatter.ReadSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

#### `string ToString(double value)`

ToString method of DoubleFormatter.

**Parameters:**
- `value` (double)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.DoubleFormatter.ToString%28System.Double%29)

#### `bool TryParse(string text, out double value)`

TryParse method of DoubleFormatter.

**Parameters:**
- `text` (string)
- `value` (double)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.DoubleFormatter.TryParse%28System.String%2CSystem.Double%40%29)

#### `static void WriteSettings(System.Collections.Generic.IDictionary<string, Fusion.Formatting.DoubleFormatter> formatters, System.Collections.Generic.IDictionary<string, string> settings)`

WriteSettings method of DoubleFormatter.

**Parameters:**
- `formatters` (System.Collections.Generic.IDictionary<string, Fusion.Formatting.DoubleFormatter>)
- `settings` (System.Collections.Generic.IDictionary<string, string>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.DoubleFormatter.WriteSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CFusion.Formatting.DoubleFormatter%7D%2CSystem.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

## DynamicViscosityFormatter (class)

DynamicViscosityFormatter class in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.DynamicViscosityFormatter)

### Constructors
- `DynamicViscosityFormatter()` — Constructs a new DynamicViscosityFormatter.
- `DynamicViscosityFormatter(Fusion.DynamicViscosityUnit unit)` — Constructs a new DynamicViscosityFormatter.
- `DynamicViscosityFormatter(Fusion.DynamicViscosityUnit unit, Fusion.DynamicViscosityUnit formatUnit)` — Constructs a new DynamicViscosityFormatter.
- `DynamicViscosityFormatter(Fusion.DynamicViscosityUnit unit, Fusion.DynamicViscosityUnit formatUnit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new DynamicViscosityFormatter.
- `DynamicViscosityFormatter(Fusion.DynamicViscosityUnit unit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new DynamicViscosityFormatter.

### Methods
#### `static System.Collections.Generic.IDictionary<string, Fusion.Formatting.DynamicViscosityFormatter> ReadSettings(System.Collections.Generic.IDictionary<string, string> settings)`

ReadSettings method of DynamicViscosityFormatter.

**Parameters:**
- `settings` (System.Collections.Generic.IDictionary<string, string>)

**Returns:** `System.Collections.Generic.IDictionary<string, Fusion.Formatting.DynamicViscosityFormatter>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.DynamicViscosityFormatter.ReadSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

#### `string ToString(Fusion.DynamicViscosity value, Fusion.Formatting.UnitSymbols symbols)`

ToString method of DynamicViscosityFormatter.

**Parameters:**
- `value` (Fusion.DynamicViscosity)
- `symbols` (Fusion.Formatting.UnitSymbols)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.DynamicViscosityFormatter.ToString%28Fusion.DynamicViscosity%2CFusion.Formatting.UnitSymbols%29)

#### `bool TryParse(string text, Fusion.Formatting.UnitSymbols symbols, out Fusion.DynamicViscosity value)`

TryParse method of DynamicViscosityFormatter.

**Parameters:**
- `text` (string)
- `symbols` (Fusion.Formatting.UnitSymbols)
- `value` (Fusion.DynamicViscosity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.DynamicViscosityFormatter.TryParse%28System.String%2CFusion.Formatting.UnitSymbols%2CFusion.DynamicViscosity%40%29)

#### `static void WriteSettings(System.Collections.Generic.IDictionary<string, Fusion.Formatting.DynamicViscosityFormatter> formatters, System.Collections.Generic.IDictionary<string, string> settings)`

WriteSettings method of DynamicViscosityFormatter.

**Parameters:**
- `formatters` (System.Collections.Generic.IDictionary<string, Fusion.Formatting.DynamicViscosityFormatter>)
- `settings` (System.Collections.Generic.IDictionary<string, string>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.DynamicViscosityFormatter.WriteSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CFusion.Formatting.DynamicViscosityFormatter%7D%2CSystem.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

## FlowCoefficientFormatter (class)

FlowCoefficientFormatter class in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.FlowCoefficientFormatter)

### Constructors
- `FlowCoefficientFormatter()` — Constructs a new FlowCoefficientFormatter.
- `FlowCoefficientFormatter(Fusion.FlowCoefficientUnit unit)` — Constructs a new FlowCoefficientFormatter.
- `FlowCoefficientFormatter(Fusion.FlowCoefficientUnit unit, Fusion.FlowCoefficientUnit formatUnit)` — Constructs a new FlowCoefficientFormatter.
- `FlowCoefficientFormatter(Fusion.FlowCoefficientUnit unit, Fusion.FlowCoefficientUnit formatUnit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new FlowCoefficientFormatter.
- `FlowCoefficientFormatter(Fusion.FlowCoefficientUnit unit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new FlowCoefficientFormatter.

### Methods
#### `static System.Collections.Generic.IDictionary<string, Fusion.Formatting.FlowCoefficientFormatter> ReadSettings(System.Collections.Generic.IDictionary<string, string> settings)`

ReadSettings method of FlowCoefficientFormatter.

**Parameters:**
- `settings` (System.Collections.Generic.IDictionary<string, string>)

**Returns:** `System.Collections.Generic.IDictionary<string, Fusion.Formatting.FlowCoefficientFormatter>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.FlowCoefficientFormatter.ReadSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

#### `string ToString(Fusion.FlowCoefficient value, Fusion.Formatting.UnitSymbols symbols)`

ToString method of FlowCoefficientFormatter.

**Parameters:**
- `value` (Fusion.FlowCoefficient)
- `symbols` (Fusion.Formatting.UnitSymbols)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.FlowCoefficientFormatter.ToString%28Fusion.FlowCoefficient%2CFusion.Formatting.UnitSymbols%29)

#### `bool TryParse(string text, Fusion.Formatting.UnitSymbols symbols, out Fusion.FlowCoefficient value)`

TryParse method of FlowCoefficientFormatter.

**Parameters:**
- `text` (string)
- `symbols` (Fusion.Formatting.UnitSymbols)
- `value` (Fusion.FlowCoefficient)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.FlowCoefficientFormatter.TryParse%28System.String%2CFusion.Formatting.UnitSymbols%2CFusion.FlowCoefficient%40%29)

#### `static void WriteSettings(System.Collections.Generic.IDictionary<string, Fusion.Formatting.FlowCoefficientFormatter> formatters, System.Collections.Generic.IDictionary<string, string> settings)`

WriteSettings method of FlowCoefficientFormatter.

**Parameters:**
- `formatters` (System.Collections.Generic.IDictionary<string, Fusion.Formatting.FlowCoefficientFormatter>)
- `settings` (System.Collections.Generic.IDictionary<string, string>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.FlowCoefficientFormatter.WriteSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CFusion.Formatting.FlowCoefficientFormatter%7D%2CSystem.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

## ForceFormatter (class)

ForceFormatter class in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.ForceFormatter)

### Constructors
- `ForceFormatter()` — Constructs a new ForceFormatter.
- `ForceFormatter(Fusion.ForceUnit unit)` — Constructs a new ForceFormatter.
- `ForceFormatter(Fusion.ForceUnit unit, Fusion.ForceUnit formatUnit)` — Constructs a new ForceFormatter.
- `ForceFormatter(Fusion.ForceUnit unit, Fusion.ForceUnit formatUnit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new ForceFormatter.
- `ForceFormatter(Fusion.ForceUnit unit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new ForceFormatter.

### Methods
#### `static System.Collections.Generic.IDictionary<string, Fusion.Formatting.ForceFormatter> ReadSettings(System.Collections.Generic.IDictionary<string, string> settings)`

ReadSettings method of ForceFormatter.

**Parameters:**
- `settings` (System.Collections.Generic.IDictionary<string, string>)

**Returns:** `System.Collections.Generic.IDictionary<string, Fusion.Formatting.ForceFormatter>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.ForceFormatter.ReadSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

#### `string ToString(Fusion.Force value, Fusion.Formatting.UnitSymbols symbols)`

ToString method of ForceFormatter.

**Parameters:**
- `value` (Fusion.Force)
- `symbols` (Fusion.Formatting.UnitSymbols)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.ForceFormatter.ToString%28Fusion.Force%2CFusion.Formatting.UnitSymbols%29)

#### `bool TryParse(string text, Fusion.Formatting.UnitSymbols symbols, out Fusion.Force value)`

TryParse method of ForceFormatter.

**Parameters:**
- `text` (string)
- `symbols` (Fusion.Formatting.UnitSymbols)
- `value` (Fusion.Force)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.ForceFormatter.TryParse%28System.String%2CFusion.Formatting.UnitSymbols%2CFusion.Force%40%29)

#### `static void WriteSettings(System.Collections.Generic.IDictionary<string, Fusion.Formatting.ForceFormatter> formatters, System.Collections.Generic.IDictionary<string, string> settings)`

WriteSettings method of ForceFormatter.

**Parameters:**
- `formatters` (System.Collections.Generic.IDictionary<string, Fusion.Formatting.ForceFormatter>)
- `settings` (System.Collections.Generic.IDictionary<string, string>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.ForceFormatter.WriteSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CFusion.Formatting.ForceFormatter%7D%2CSystem.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

## Formatter (class)

Formatter class in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.Formatter)

### Constructors
- `Formatter(string formatString, System.IFormatProvider formatProvider)` — Constructs a new Formatter.

### Properties
- `FormatProvider` (System.IFormatProvider, get) — FormatProvider property of Formatter.
- `FormatString` (string, get) — FormatString property of Formatter.

## Formatter`1 (class)

Formatter`1 class in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.Formatter`1)

### Constructors
- `Formatter`1(string formatString, System.IFormatProvider formatProvider)` — Constructs a new Formatter`1.

### Methods
#### `string ToString(T value)`

ToString method of Formatter`1.

**Parameters:**
- `value` (T)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.Formatter%601.ToString%28%600%29)

#### `string ToString(object value)`

ToString method of Formatter`1.

**Parameters:**
- `value` (object)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.Formatter%601.ToString%28System.Object%29)

#### `bool TryParse(string text, out T value)`

TryParse method of Formatter`1.

**Parameters:**
- `text` (string)
- `value` (T)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.Formatter%601.TryParse%28System.String%2C%600%40%29)

#### `bool TryParse(string text, out object value)`

TryParse method of Formatter`1.

**Parameters:**
- `text` (string)
- `value` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.Formatter%601.TryParse%28System.String%2CSystem.Object%40%29)

## HeatCapacityFormatter (class)

HeatCapacityFormatter class in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.HeatCapacityFormatter)

### Constructors
- `HeatCapacityFormatter()` — Constructs a new HeatCapacityFormatter.
- `HeatCapacityFormatter(Fusion.HeatCapacityUnit unit)` — Constructs a new HeatCapacityFormatter.
- `HeatCapacityFormatter(Fusion.HeatCapacityUnit unit, Fusion.HeatCapacityUnit formatUnit)` — Constructs a new HeatCapacityFormatter.
- `HeatCapacityFormatter(Fusion.HeatCapacityUnit unit, Fusion.HeatCapacityUnit formatUnit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new HeatCapacityFormatter.
- `HeatCapacityFormatter(Fusion.HeatCapacityUnit unit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new HeatCapacityFormatter.

### Methods
#### `static System.Collections.Generic.IDictionary<string, Fusion.Formatting.HeatCapacityFormatter> ReadSettings(System.Collections.Generic.IDictionary<string, string> settings)`

ReadSettings method of HeatCapacityFormatter.

**Parameters:**
- `settings` (System.Collections.Generic.IDictionary<string, string>)

**Returns:** `System.Collections.Generic.IDictionary<string, Fusion.Formatting.HeatCapacityFormatter>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.HeatCapacityFormatter.ReadSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

#### `string ToString(Fusion.HeatCapacity value, Fusion.Formatting.UnitSymbols symbols)`

ToString method of HeatCapacityFormatter.

**Parameters:**
- `value` (Fusion.HeatCapacity)
- `symbols` (Fusion.Formatting.UnitSymbols)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.HeatCapacityFormatter.ToString%28Fusion.HeatCapacity%2CFusion.Formatting.UnitSymbols%29)

#### `bool TryParse(string text, Fusion.Formatting.UnitSymbols symbols, out Fusion.HeatCapacity value)`

TryParse method of HeatCapacityFormatter.

**Parameters:**
- `text` (string)
- `symbols` (Fusion.Formatting.UnitSymbols)
- `value` (Fusion.HeatCapacity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.HeatCapacityFormatter.TryParse%28System.String%2CFusion.Formatting.UnitSymbols%2CFusion.HeatCapacity%40%29)

#### `static void WriteSettings(System.Collections.Generic.IDictionary<string, Fusion.Formatting.HeatCapacityFormatter> formatters, System.Collections.Generic.IDictionary<string, string> settings)`

WriteSettings method of HeatCapacityFormatter.

**Parameters:**
- `formatters` (System.Collections.Generic.IDictionary<string, Fusion.Formatting.HeatCapacityFormatter>)
- `settings` (System.Collections.Generic.IDictionary<string, string>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.HeatCapacityFormatter.WriteSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CFusion.Formatting.HeatCapacityFormatter%7D%2CSystem.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

## IFormatter (interface)

IFormatter interface in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.IFormatter)

### Methods
#### `string ToString(object value)`

ToString method of IFormatter.

**Parameters:**
- `value` (object)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.IFormatter.ToString%28System.Object%29)

#### `bool TryParse(string text, out object value)`

TryParse method of IFormatter.

**Parameters:**
- `text` (string)
- `value` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.IFormatter.TryParse%28System.String%2CSystem.Object%40%29)

## IQuantityFormatter (interface)

IQuantityFormatter interface in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.IQuantityFormatter)

### Methods
#### `string ToString(object value, Fusion.Formatting.UnitSymbols symbols)`

ToString method of IQuantityFormatter.

**Parameters:**
- `value` (object)
- `symbols` (Fusion.Formatting.UnitSymbols)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.IQuantityFormatter.ToString%28System.Object%2CFusion.Formatting.UnitSymbols%29)

#### `bool TryParse(string text, Fusion.Formatting.UnitSymbols symbols, out object value)`

TryParse method of IQuantityFormatter.

**Parameters:**
- `text` (string)
- `symbols` (Fusion.Formatting.UnitSymbols)
- `value` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.IQuantityFormatter.TryParse%28System.String%2CFusion.Formatting.UnitSymbols%2CSystem.Object%40%29)

## InchSymbolPlacement (enum)

InchSymbolPlacement enum in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.InchSymbolPlacement)

### Values
- `AfterInches` = `0`
- `AfterFractions` = `1`

## Int32Formatter (class)

Int32Formatter class in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.Int32Formatter)

### Constructors
- `Int32Formatter()` — Constructs a new Int32Formatter.
- `Int32Formatter(System.Globalization.NumberStyles numberStyle, string formatString, System.IFormatProvider formatProvider)` — Constructs a new Int32Formatter.

### Methods
#### `static System.Collections.Generic.IDictionary<string, Fusion.Formatting.Int32Formatter> ReadSettings(System.Collections.Generic.IDictionary<string, string> settings)`

ReadSettings method of Int32Formatter.

**Parameters:**
- `settings` (System.Collections.Generic.IDictionary<string, string>)

**Returns:** `System.Collections.Generic.IDictionary<string, Fusion.Formatting.Int32Formatter>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.Int32Formatter.ReadSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

#### `string ToString(int value)`

ToString method of Int32Formatter.

**Parameters:**
- `value` (int)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.Int32Formatter.ToString%28System.Int32%29)

#### `bool TryParse(string text, out int value)`

TryParse method of Int32Formatter.

**Parameters:**
- `text` (string)
- `value` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.Int32Formatter.TryParse%28System.String%2CSystem.Int32%40%29)

#### `static void WriteSettings(System.Collections.Generic.IDictionary<string, Fusion.Formatting.Int32Formatter> formatters, System.Collections.Generic.IDictionary<string, string> settings)`

WriteSettings method of Int32Formatter.

**Parameters:**
- `formatters` (System.Collections.Generic.IDictionary<string, Fusion.Formatting.Int32Formatter>)
- `settings` (System.Collections.Generic.IDictionary<string, string>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.Int32Formatter.WriteSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CFusion.Formatting.Int32Formatter%7D%2CSystem.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

## KinematicViscosityFormatter (class)

KinematicViscosityFormatter class in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.KinematicViscosityFormatter)

### Constructors
- `KinematicViscosityFormatter()` — Constructs a new KinematicViscosityFormatter.
- `KinematicViscosityFormatter(Fusion.KinematicViscosityUnit unit)` — Constructs a new KinematicViscosityFormatter.
- `KinematicViscosityFormatter(Fusion.KinematicViscosityUnit unit, Fusion.KinematicViscosityUnit formatUnit)` — Constructs a new KinematicViscosityFormatter.
- `KinematicViscosityFormatter(Fusion.KinematicViscosityUnit unit, Fusion.KinematicViscosityUnit formatUnit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new KinematicViscosityFormatter.
- `KinematicViscosityFormatter(Fusion.KinematicViscosityUnit unit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new KinematicViscosityFormatter.

### Methods
#### `static System.Collections.Generic.IDictionary<string, Fusion.Formatting.KinematicViscosityFormatter> ReadSettings(System.Collections.Generic.IDictionary<string, string> settings)`

ReadSettings method of KinematicViscosityFormatter.

**Parameters:**
- `settings` (System.Collections.Generic.IDictionary<string, string>)

**Returns:** `System.Collections.Generic.IDictionary<string, Fusion.Formatting.KinematicViscosityFormatter>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.KinematicViscosityFormatter.ReadSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

#### `string ToString(Fusion.KinematicViscosity value, Fusion.Formatting.UnitSymbols symbols)`

ToString method of KinematicViscosityFormatter.

**Parameters:**
- `value` (Fusion.KinematicViscosity)
- `symbols` (Fusion.Formatting.UnitSymbols)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.KinematicViscosityFormatter.ToString%28Fusion.KinematicViscosity%2CFusion.Formatting.UnitSymbols%29)

#### `bool TryParse(string text, Fusion.Formatting.UnitSymbols symbols, out Fusion.KinematicViscosity value)`

TryParse method of KinematicViscosityFormatter.

**Parameters:**
- `text` (string)
- `symbols` (Fusion.Formatting.UnitSymbols)
- `value` (Fusion.KinematicViscosity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.KinematicViscosityFormatter.TryParse%28System.String%2CFusion.Formatting.UnitSymbols%2CFusion.KinematicViscosity%40%29)

#### `static void WriteSettings(System.Collections.Generic.IDictionary<string, Fusion.Formatting.KinematicViscosityFormatter> formatters, System.Collections.Generic.IDictionary<string, string> settings)`

WriteSettings method of KinematicViscosityFormatter.

**Parameters:**
- `formatters` (System.Collections.Generic.IDictionary<string, Fusion.Formatting.KinematicViscosityFormatter>)
- `settings` (System.Collections.Generic.IDictionary<string, string>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.KinematicViscosityFormatter.WriteSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CFusion.Formatting.KinematicViscosityFormatter%7D%2CSystem.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

## Length4Formatter (class)

Length4Formatter class in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.Length4Formatter)

### Constructors
- `Length4Formatter()` — Constructs a new Length4Formatter.
- `Length4Formatter(Fusion.Length4Units unit)` — Constructs a new Length4Formatter.
- `Length4Formatter(Fusion.Length4Units unit, Fusion.Length4Units formatUnit)` — Constructs a new Length4Formatter.
- `Length4Formatter(Fusion.Length4Units unit, Fusion.Length4Units formatUnit, string formatString)` — Constructs a new Length4Formatter.
- `Length4Formatter(Fusion.Length4Units unit, Fusion.Length4Units formatUnit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new Length4Formatter.

### Methods
#### `string ToString(double value, Fusion.Formatting.UnitSymbols symbols)`

ToString method of Length4Formatter.

**Parameters:**
- `value` (double)
- `symbols` (Fusion.Formatting.UnitSymbols)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.Length4Formatter.ToString%28System.Double%2CFusion.Formatting.UnitSymbols%29)

#### `bool TryParse(string text, Fusion.Formatting.UnitSymbols symbols, out double value)`

TryParse method of Length4Formatter.

**Parameters:**
- `text` (string)
- `symbols` (Fusion.Formatting.UnitSymbols)
- `value` (double)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.Length4Formatter.TryParse%28System.String%2CFusion.Formatting.UnitSymbols%2CSystem.Double%40%29)

## LengthFormatter (class)

LengthFormatter class in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.LengthFormatter)

### Constructors
- `LengthFormatter()` — Constructs a new LengthFormatter.
- `LengthFormatter(Fusion.LengthUnit unit)` — Constructs a new LengthFormatter.
- `LengthFormatter(Fusion.LengthUnit unit, Fusion.LengthUnit formatUnit)` — Constructs a new LengthFormatter.
- `LengthFormatter(Fusion.LengthUnit unit, Fusion.LengthUnit formatUnit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new LengthFormatter.
- `LengthFormatter(Fusion.LengthUnit unit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new LengthFormatter.

### Methods
#### `static System.Collections.Generic.IDictionary<string, Fusion.Formatting.LengthFormatter> ReadSettings(System.Collections.Generic.IDictionary<string, string> settings)`

ReadSettings method of LengthFormatter.

**Parameters:**
- `settings` (System.Collections.Generic.IDictionary<string, string>)

**Returns:** `System.Collections.Generic.IDictionary<string, Fusion.Formatting.LengthFormatter>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.LengthFormatter.ReadSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

#### `string ToString(Fusion.Length value, Fusion.Formatting.UnitSymbols symbols)`

ToString method of LengthFormatter.

**Parameters:**
- `value` (Fusion.Length)
- `symbols` (Fusion.Formatting.UnitSymbols)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.LengthFormatter.ToString%28Fusion.Length%2CFusion.Formatting.UnitSymbols%29)

#### `bool TryParse(string text, Fusion.Formatting.UnitSymbols symbols, out Fusion.Length value)`

TryParse method of LengthFormatter.

**Parameters:**
- `text` (string)
- `symbols` (Fusion.Formatting.UnitSymbols)
- `value` (Fusion.Length)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.LengthFormatter.TryParse%28System.String%2CFusion.Formatting.UnitSymbols%2CFusion.Length%40%29)

#### `static void WriteSettings(System.Collections.Generic.IDictionary<string, Fusion.Formatting.LengthFormatter> formatters, System.Collections.Generic.IDictionary<string, string> settings)`

WriteSettings method of LengthFormatter.

**Parameters:**
- `formatters` (System.Collections.Generic.IDictionary<string, Fusion.Formatting.LengthFormatter>)
- `settings` (System.Collections.Generic.IDictionary<string, string>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.LengthFormatter.WriteSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CFusion.Formatting.LengthFormatter%7D%2CSystem.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

## LinearDensityFormatter (class)

LinearDensityFormatter class in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.LinearDensityFormatter)

### Constructors
- `LinearDensityFormatter()` — Constructs a new LinearDensityFormatter.
- `LinearDensityFormatter(Fusion.LinearDensity.LinearDensityUnit unit)` — Constructs a new LinearDensityFormatter.
- `LinearDensityFormatter(Fusion.LinearDensity.LinearDensityUnit unit, Fusion.LinearDensity.LinearDensityUnit formatUnit)` — Constructs a new LinearDensityFormatter.
- `LinearDensityFormatter(Fusion.LinearDensity.LinearDensityUnit unit, Fusion.LinearDensity.LinearDensityUnit formatUnit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new LinearDensityFormatter.
- `LinearDensityFormatter(Fusion.LinearDensity.LinearDensityUnit unit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new LinearDensityFormatter.

### Methods
#### `static System.Collections.Generic.IDictionary<string, Fusion.Formatting.LinearDensityFormatter> ReadSettings(System.Collections.Generic.IDictionary<string, string> settings)`

ReadSettings method of LinearDensityFormatter.

**Parameters:**
- `settings` (System.Collections.Generic.IDictionary<string, string>)

**Returns:** `System.Collections.Generic.IDictionary<string, Fusion.Formatting.LinearDensityFormatter>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.LinearDensityFormatter.ReadSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

#### `string ToString(Fusion.LinearDensity value, Fusion.Formatting.UnitSymbols symbols)`

ToString method of LinearDensityFormatter.

**Parameters:**
- `value` (Fusion.LinearDensity)
- `symbols` (Fusion.Formatting.UnitSymbols)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.LinearDensityFormatter.ToString%28Fusion.LinearDensity%2CFusion.Formatting.UnitSymbols%29)

#### `bool TryParse(string text, Fusion.Formatting.UnitSymbols symbols, out Fusion.LinearDensity value)`

TryParse method of LinearDensityFormatter.

**Parameters:**
- `text` (string)
- `symbols` (Fusion.Formatting.UnitSymbols)
- `value` (Fusion.LinearDensity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.LinearDensityFormatter.TryParse%28System.String%2CFusion.Formatting.UnitSymbols%2CFusion.LinearDensity%40%29)

#### `static void WriteSettings(System.Collections.Generic.IDictionary<string, Fusion.Formatting.LinearDensityFormatter> formatters, System.Collections.Generic.IDictionary<string, string> settings)`

WriteSettings method of LinearDensityFormatter.

**Parameters:**
- `formatters` (System.Collections.Generic.IDictionary<string, Fusion.Formatting.LinearDensityFormatter>)
- `settings` (System.Collections.Generic.IDictionary<string, string>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.LinearDensityFormatter.WriteSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CFusion.Formatting.LinearDensityFormatter%7D%2CSystem.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

## MassFlowFormatter (class)

MassFlowFormatter class in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.MassFlowFormatter)

### Constructors
- `MassFlowFormatter()` — Constructs a new MassFlowFormatter.
- `MassFlowFormatter(Fusion.MassFlowUnit unit)` — Constructs a new MassFlowFormatter.
- `MassFlowFormatter(Fusion.MassFlowUnit unit, Fusion.MassFlowUnit formatUnit)` — Constructs a new MassFlowFormatter.
- `MassFlowFormatter(Fusion.MassFlowUnit unit, Fusion.MassFlowUnit formatUnit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new MassFlowFormatter.
- `MassFlowFormatter(Fusion.MassFlowUnit unit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new MassFlowFormatter.

### Methods
#### `static System.Collections.Generic.IDictionary<string, Fusion.Formatting.MassFlowFormatter> ReadSettings(System.Collections.Generic.IDictionary<string, string> settings)`

ReadSettings method of MassFlowFormatter.

**Parameters:**
- `settings` (System.Collections.Generic.IDictionary<string, string>)

**Returns:** `System.Collections.Generic.IDictionary<string, Fusion.Formatting.MassFlowFormatter>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.MassFlowFormatter.ReadSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

#### `string ToString(Fusion.MassFlow value, Fusion.Formatting.UnitSymbols symbols)`

ToString method of MassFlowFormatter.

**Parameters:**
- `value` (Fusion.MassFlow)
- `symbols` (Fusion.Formatting.UnitSymbols)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.MassFlowFormatter.ToString%28Fusion.MassFlow%2CFusion.Formatting.UnitSymbols%29)

#### `bool TryParse(string text, Fusion.Formatting.UnitSymbols symbols, out Fusion.MassFlow value)`

TryParse method of MassFlowFormatter.

**Parameters:**
- `text` (string)
- `symbols` (Fusion.Formatting.UnitSymbols)
- `value` (Fusion.MassFlow)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.MassFlowFormatter.TryParse%28System.String%2CFusion.Formatting.UnitSymbols%2CFusion.MassFlow%40%29)

#### `static void WriteSettings(System.Collections.Generic.IDictionary<string, Fusion.Formatting.MassFlowFormatter> formatters, System.Collections.Generic.IDictionary<string, string> settings)`

WriteSettings method of MassFlowFormatter.

**Parameters:**
- `formatters` (System.Collections.Generic.IDictionary<string, Fusion.Formatting.MassFlowFormatter>)
- `settings` (System.Collections.Generic.IDictionary<string, string>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.MassFlowFormatter.WriteSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CFusion.Formatting.MassFlowFormatter%7D%2CSystem.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

## MassFormatter (class)

MassFormatter class in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.MassFormatter)

### Constructors
- `MassFormatter()` — Constructs a new MassFormatter.
- `MassFormatter(Fusion.MassUnit unit)` — Constructs a new MassFormatter.
- `MassFormatter(Fusion.MassUnit unit, Fusion.MassUnit formatUnit)` — Constructs a new MassFormatter.
- `MassFormatter(Fusion.MassUnit unit, Fusion.MassUnit formatUnit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new MassFormatter.
- `MassFormatter(Fusion.MassUnit unit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new MassFormatter.

### Methods
#### `static System.Collections.Generic.IDictionary<string, Fusion.Formatting.MassFormatter> ReadSettings(System.Collections.Generic.IDictionary<string, string> settings)`

ReadSettings method of MassFormatter.

**Parameters:**
- `settings` (System.Collections.Generic.IDictionary<string, string>)

**Returns:** `System.Collections.Generic.IDictionary<string, Fusion.Formatting.MassFormatter>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.MassFormatter.ReadSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

#### `string ToString(Fusion.Mass value, Fusion.Formatting.UnitSymbols symbols)`

ToString method of MassFormatter.

**Parameters:**
- `value` (Fusion.Mass)
- `symbols` (Fusion.Formatting.UnitSymbols)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.MassFormatter.ToString%28Fusion.Mass%2CFusion.Formatting.UnitSymbols%29)

#### `bool TryParse(string text, Fusion.Formatting.UnitSymbols symbols, out Fusion.Mass value)`

TryParse method of MassFormatter.

**Parameters:**
- `text` (string)
- `symbols` (Fusion.Formatting.UnitSymbols)
- `value` (Fusion.Mass)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.MassFormatter.TryParse%28System.String%2CFusion.Formatting.UnitSymbols%2CFusion.Mass%40%29)

#### `static void WriteSettings(System.Collections.Generic.IDictionary<string, Fusion.Formatting.MassFormatter> formatters, System.Collections.Generic.IDictionary<string, string> settings)`

WriteSettings method of MassFormatter.

**Parameters:**
- `formatters` (System.Collections.Generic.IDictionary<string, Fusion.Formatting.MassFormatter>)
- `settings` (System.Collections.Generic.IDictionary<string, string>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.MassFormatter.WriteSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CFusion.Formatting.MassFormatter%7D%2CSystem.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

## NumberFormatter`1 (class)

NumberFormatter`1 class in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.NumberFormatter`1)

### Constructors
- `NumberFormatter`1(System.Globalization.NumberStyles numberStyle, string formatString, System.IFormatProvider formatProvider)` — Constructs a new NumberFormatter`1.

### Properties
- `NumberStyle` (System.Globalization.NumberStyles, get) — NumberStyle property of NumberFormatter`1.

## PowerFormatter (class)

PowerFormatter class in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.PowerFormatter)

### Constructors
- `PowerFormatter()` — Constructs a new PowerFormatter.
- `PowerFormatter(Fusion.PowerUnit unit)` — Constructs a new PowerFormatter.
- `PowerFormatter(Fusion.PowerUnit unit, Fusion.PowerUnit formatUnit)` — Constructs a new PowerFormatter.
- `PowerFormatter(Fusion.PowerUnit unit, Fusion.PowerUnit formatUnit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new PowerFormatter.
- `PowerFormatter(Fusion.PowerUnit unit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new PowerFormatter.

### Methods
#### `static System.Collections.Generic.IDictionary<string, Fusion.Formatting.PowerFormatter> ReadSettings(System.Collections.Generic.IDictionary<string, string> settings)`

ReadSettings method of PowerFormatter.

**Parameters:**
- `settings` (System.Collections.Generic.IDictionary<string, string>)

**Returns:** `System.Collections.Generic.IDictionary<string, Fusion.Formatting.PowerFormatter>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.PowerFormatter.ReadSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

#### `string ToString(Fusion.Power value, Fusion.Formatting.UnitSymbols symbols)`

ToString method of PowerFormatter.

**Parameters:**
- `value` (Fusion.Power)
- `symbols` (Fusion.Formatting.UnitSymbols)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.PowerFormatter.ToString%28Fusion.Power%2CFusion.Formatting.UnitSymbols%29)

#### `bool TryParse(string text, Fusion.Formatting.UnitSymbols symbols, out Fusion.Power value)`

TryParse method of PowerFormatter.

**Parameters:**
- `text` (string)
- `symbols` (Fusion.Formatting.UnitSymbols)
- `value` (Fusion.Power)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.PowerFormatter.TryParse%28System.String%2CFusion.Formatting.UnitSymbols%2CFusion.Power%40%29)

#### `static void WriteSettings(System.Collections.Generic.IDictionary<string, Fusion.Formatting.PowerFormatter> formatters, System.Collections.Generic.IDictionary<string, string> settings)`

WriteSettings method of PowerFormatter.

**Parameters:**
- `formatters` (System.Collections.Generic.IDictionary<string, Fusion.Formatting.PowerFormatter>)
- `settings` (System.Collections.Generic.IDictionary<string, string>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.PowerFormatter.WriteSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CFusion.Formatting.PowerFormatter%7D%2CSystem.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

## PressureLossFormatter (class)

PressureLossFormatter class in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.PressureLossFormatter)

### Constructors
- `PressureLossFormatter()` — Constructs a new PressureLossFormatter.
- `PressureLossFormatter(Fusion.PressureLossUnit unit)` — Constructs a new PressureLossFormatter.
- `PressureLossFormatter(Fusion.PressureLossUnit unit, Fusion.PressureLossUnit formatUnit)` — Constructs a new PressureLossFormatter.
- `PressureLossFormatter(Fusion.PressureLossUnit unit, Fusion.PressureLossUnit formatUnit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new PressureLossFormatter.
- `PressureLossFormatter(Fusion.PressureLossUnit unit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new PressureLossFormatter.

### Methods
#### `static System.Collections.Generic.IDictionary<string, Fusion.Formatting.PressureLossFormatter> ReadSettings(System.Collections.Generic.IDictionary<string, string> settings)`

ReadSettings method of PressureLossFormatter.

**Parameters:**
- `settings` (System.Collections.Generic.IDictionary<string, string>)

**Returns:** `System.Collections.Generic.IDictionary<string, Fusion.Formatting.PressureLossFormatter>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.PressureLossFormatter.ReadSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

#### `string ToString(Fusion.PressureLoss value, Fusion.Formatting.UnitSymbols symbols)`

ToString method of PressureLossFormatter.

**Parameters:**
- `value` (Fusion.PressureLoss)
- `symbols` (Fusion.Formatting.UnitSymbols)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.PressureLossFormatter.ToString%28Fusion.PressureLoss%2CFusion.Formatting.UnitSymbols%29)

#### `bool TryParse(string text, Fusion.Formatting.UnitSymbols symbols, out Fusion.PressureLoss value)`

TryParse method of PressureLossFormatter.

**Parameters:**
- `text` (string)
- `symbols` (Fusion.Formatting.UnitSymbols)
- `value` (Fusion.PressureLoss)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.PressureLossFormatter.TryParse%28System.String%2CFusion.Formatting.UnitSymbols%2CFusion.PressureLoss%40%29)

#### `static void WriteSettings(System.Collections.Generic.IDictionary<string, Fusion.Formatting.PressureLossFormatter> formatters, System.Collections.Generic.IDictionary<string, string> settings)`

WriteSettings method of PressureLossFormatter.

**Parameters:**
- `formatters` (System.Collections.Generic.IDictionary<string, Fusion.Formatting.PressureLossFormatter>)
- `settings` (System.Collections.Generic.IDictionary<string, string>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.PressureLossFormatter.WriteSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CFusion.Formatting.PressureLossFormatter%7D%2CSystem.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

## QuantityFormatter`2 (class)

QuantityFormatter`2 class in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.QuantityFormatter`2)

### Constructors
- `QuantityFormatter`2(TQuantityUnit unit, TQuantityUnit formatUnit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new QuantityFormatter`2.

### Methods
#### `string ToString(TQuantity value)`

ToString method of QuantityFormatter`2.

**Parameters:**
- `value` (TQuantity)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.QuantityFormatter%602.ToString%28%600%29)

#### `string ToString(TQuantity value, Fusion.Formatting.UnitSymbols symbols)`

ToString method of QuantityFormatter`2.

**Parameters:**
- `value` (TQuantity)
- `symbols` (Fusion.Formatting.UnitSymbols)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.QuantityFormatter%602.ToString%28%600%2CFusion.Formatting.UnitSymbols%29)

#### `string ToString(object value, Fusion.Formatting.UnitSymbols symbols)`

ToString method of QuantityFormatter`2.

**Parameters:**
- `value` (object)
- `symbols` (Fusion.Formatting.UnitSymbols)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.QuantityFormatter%602.ToString%28System.Object%2CFusion.Formatting.UnitSymbols%29)

#### `bool TryParse(string text, Fusion.Formatting.UnitSymbols symbols, out TQuantity value)`

TryParse method of QuantityFormatter`2.

**Parameters:**
- `text` (string)
- `symbols` (Fusion.Formatting.UnitSymbols)
- `value` (TQuantity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.QuantityFormatter%602.TryParse%28System.String%2CFusion.Formatting.UnitSymbols%2C%600%40%29)

#### `bool TryParse(string text, Fusion.Formatting.UnitSymbols symbols, out object value)`

TryParse method of QuantityFormatter`2.

**Parameters:**
- `text` (string)
- `symbols` (Fusion.Formatting.UnitSymbols)
- `value` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.QuantityFormatter%602.TryParse%28System.String%2CFusion.Formatting.UnitSymbols%2CSystem.Object%40%29)

#### `bool TryParse(string text, out TQuantity value)`

TryParse method of QuantityFormatter`2.

**Parameters:**
- `text` (string)
- `value` (TQuantity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.QuantityFormatter%602.TryParse%28System.String%2C%600%40%29)

### Properties
- `FormatUnit` (TQuantityUnit, get) — FormatUnit property of QuantityFormatter`2.
- `Unit` (TQuantityUnit, get) — Unit property of QuantityFormatter`2.

## RatioFormatter (class)

RatioFormatter class in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.RatioFormatter)

### Constructors
- `RatioFormatter()` — Constructs a new RatioFormatter.
- `RatioFormatter(Fusion.RatioUnit unit)` — Constructs a new RatioFormatter.
- `RatioFormatter(Fusion.RatioUnit unit, Fusion.RatioUnit formatUnit)` — Constructs a new RatioFormatter.
- `RatioFormatter(Fusion.RatioUnit unit, Fusion.RatioUnit formatUnit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new RatioFormatter.

### Methods
#### `string ToString(double value, Fusion.Formatting.UnitSymbols symbols)`

ToString method of RatioFormatter.

**Parameters:**
- `value` (double)
- `symbols` (Fusion.Formatting.UnitSymbols)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.RatioFormatter.ToString%28System.Double%2CFusion.Formatting.UnitSymbols%29)

#### `bool TryParse(string text, Fusion.Formatting.UnitSymbols symbols, out double value)`

TryParse method of RatioFormatter.

**Parameters:**
- `text` (string)
- `symbols` (Fusion.Formatting.UnitSymbols)
- `value` (double)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.RatioFormatter.TryParse%28System.String%2CFusion.Formatting.UnitSymbols%2CSystem.Double%40%29)

## TemperatureFormatter (class)

TemperatureFormatter class in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.TemperatureFormatter)

### Constructors
- `TemperatureFormatter()` — Constructs a new TemperatureFormatter.
- `TemperatureFormatter(Fusion.TemperatureUnit unit)` — Constructs a new TemperatureFormatter.
- `TemperatureFormatter(Fusion.TemperatureUnit unit, Fusion.TemperatureUnit formatUnit)` — Constructs a new TemperatureFormatter.
- `TemperatureFormatter(Fusion.TemperatureUnit unit, Fusion.TemperatureUnit formatUnit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new TemperatureFormatter.
- `TemperatureFormatter(Fusion.TemperatureUnit unit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new TemperatureFormatter.

### Methods
#### `static System.Collections.Generic.IDictionary<string, Fusion.Formatting.TemperatureFormatter> ReadSettings(System.Collections.Generic.IDictionary<string, string> settings)`

ReadSettings method of TemperatureFormatter.

**Parameters:**
- `settings` (System.Collections.Generic.IDictionary<string, string>)

**Returns:** `System.Collections.Generic.IDictionary<string, Fusion.Formatting.TemperatureFormatter>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.TemperatureFormatter.ReadSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

#### `string ToString(Fusion.Temperature value, Fusion.Formatting.UnitSymbols symbols)`

ToString method of TemperatureFormatter.

**Parameters:**
- `value` (Fusion.Temperature)
- `symbols` (Fusion.Formatting.UnitSymbols)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.TemperatureFormatter.ToString%28Fusion.Temperature%2CFusion.Formatting.UnitSymbols%29)

#### `bool TryParse(string text, Fusion.Formatting.UnitSymbols symbols, out Fusion.Temperature value)`

TryParse method of TemperatureFormatter.

**Parameters:**
- `text` (string)
- `symbols` (Fusion.Formatting.UnitSymbols)
- `value` (Fusion.Temperature)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.TemperatureFormatter.TryParse%28System.String%2CFusion.Formatting.UnitSymbols%2CFusion.Temperature%40%29)

#### `static void WriteSettings(System.Collections.Generic.IDictionary<string, Fusion.Formatting.TemperatureFormatter> formatters, System.Collections.Generic.IDictionary<string, string> settings)`

WriteSettings method of TemperatureFormatter.

**Parameters:**
- `formatters` (System.Collections.Generic.IDictionary<string, Fusion.Formatting.TemperatureFormatter>)
- `settings` (System.Collections.Generic.IDictionary<string, string>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.TemperatureFormatter.WriteSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CFusion.Formatting.TemperatureFormatter%7D%2CSystem.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

## TimeSpanFormatter (class)

TimeSpanFormatter class in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.TimeSpanFormatter)

### Constructors
- `TimeSpanFormatter()` — Constructs a new TimeSpanFormatter.
- `TimeSpanFormatter(System.Globalization.TimeSpanStyles timeSpanStyle, string formatString, System.IFormatProvider formatProvider)` — Constructs a new TimeSpanFormatter.

### Methods
#### `static System.Collections.Generic.IDictionary<string, Fusion.Formatting.TimeSpanFormatter> ReadSettings(System.Collections.Generic.IDictionary<string, string> settings)`

ReadSettings method of TimeSpanFormatter.

**Parameters:**
- `settings` (System.Collections.Generic.IDictionary<string, string>)

**Returns:** `System.Collections.Generic.IDictionary<string, Fusion.Formatting.TimeSpanFormatter>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.TimeSpanFormatter.ReadSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

#### `string ToString(System.TimeSpan value)`

ToString method of TimeSpanFormatter.

**Parameters:**
- `value` (System.TimeSpan)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.TimeSpanFormatter.ToString%28System.TimeSpan%29)

#### `bool TryParse(string text, out System.TimeSpan value)`

TryParse method of TimeSpanFormatter.

**Parameters:**
- `text` (string)
- `value` (System.TimeSpan)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.TimeSpanFormatter.TryParse%28System.String%2CSystem.TimeSpan%40%29)

#### `static void WriteSettings(System.Collections.Generic.IDictionary<string, Fusion.Formatting.TimeSpanFormatter> formatters, System.Collections.Generic.IDictionary<string, string> settings)`

WriteSettings method of TimeSpanFormatter.

**Parameters:**
- `formatters` (System.Collections.Generic.IDictionary<string, Fusion.Formatting.TimeSpanFormatter>)
- `settings` (System.Collections.Generic.IDictionary<string, string>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.TimeSpanFormatter.WriteSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CFusion.Formatting.TimeSpanFormatter%7D%2CSystem.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

### Properties
- `TimeSpanStyle` (System.Globalization.TimeSpanStyles, get) — TimeSpanStyle property of TimeSpanFormatter.

## UnitSymbols (class)

UnitSymbols class in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.UnitSymbols)

### Constructors
- `UnitSymbols()` — Constructs a new UnitSymbols.

### Properties
- `BtusPerFahrenheitPound` (string, get/set) — BtusPerFahrenheitPound property of UnitSymbols.
- `Celsius` (string, get/set) — Celsius property of UnitSymbols.
- `Centimeter` (string, get/set) — Centimeter property of UnitSymbols.
- `CubicCentimeter` (string, get/set) — CubicCentimeter property of UnitSymbols.
- `CubicFeetPerSecond` (string, get/set) — CubicFeetPerSecond property of UnitSymbols.
- `CubicFoot` (string, get/set) — CubicFoot property of UnitSymbols.
- `CubicInch` (string, get/set) — CubicInch property of UnitSymbols.
- `CubicMeter` (string, get/set) — CubicMeter property of UnitSymbols.
- `CubicMetersPerSecond` (string, get/set) — CubicMetersPerSecond property of UnitSymbols.
- `CubicMillimeter` (string, get/set) — CubicMillimeter property of UnitSymbols.
- `CubicYard` (string, get/set) — CubicYard property of UnitSymbols.
- `Cv` (string, get/set) — Cv property of UnitSymbols.
- `Decanewton` (string, get/set) — Decanewton property of UnitSymbols.
- `DegMinSec_Deg` (string, get/set) — DegMinSec_Deg property of UnitSymbols.
- `DegMinSec_Min` (string, get/set) — DegMinSec_Min property of UnitSymbols.
- `DegMinSec_Sec` (string, get/set) — DegMinSec_Sec property of UnitSymbols.
- `Degree` (string, get/set) — Degree property of UnitSymbols.
- `Fahrenheit` (string, get/set) — Fahrenheit property of UnitSymbols.
- `FeetPerSecond` (string, get/set) — FeetPerSecond property of UnitSymbols.
- `Foot` (string, get/set) — Foot property of UnitSymbols.
- `Gallon` (string, get/set) — Gallon property of UnitSymbols.
- `Gram` (string, get/set) — Gram property of UnitSymbols.
- `HectopascalsPerMeter` (string, get/set) — HectopascalsPerMeter property of UnitSymbols.
- `Inch` (string, get/set) — Inch property of UnitSymbols.
- `InchSymbolPlacement` (Fusion.Formatting.InchSymbolPlacement, get/set) — InchSymbolPlacement property of UnitSymbols.
- `Infinity` (string, get/set) — Infinity property of UnitSymbols.
- `JoulesPerKelvinKilogram` (string, get/set) — JoulesPerKelvinKilogram property of UnitSymbols.
- `Kelvin` (string, get/set) — Kelvin property of UnitSymbols.
- `KiloPound` (string, get/set) — KiloPound property of UnitSymbols.
- `Kilogram` (string, get/set) — Kilogram property of UnitSymbols.
- `KilogramForce` (string, get/set) — KilogramForce property of UnitSymbols.
- `KilogramsPerCubicMeter` (string, get/set) — KilogramsPerCubicMeter property of UnitSymbols.
- `KilogramsPerHour` (string, get/set) — KilogramsPerHour property of UnitSymbols.
- `KilogramsPerSecond` (string, get/set) — KilogramsPerSecond property of UnitSymbols.
- `KilogramsPerSquareMeterSeconds` (string, get/set) — KilogramsPerSquareMeterSeconds property of UnitSymbols.
- `KilojoulesPerKelvinKilogram` (string, get/set) — KilojoulesPerKelvinKilogram property of UnitSymbols.
- `Kilometer` (string, get/set) — Kilometer property of UnitSymbols.
- `Kilonewton` (string, get/set) — Kilonewton property of UnitSymbols.
- `KilonewtonsPerCubicMeter` (string, get/set) — KilonewtonsPerCubicMeter property of UnitSymbols.
- `KilopascalsPerMeter` (string, get/set) — KilopascalsPerMeter property of UnitSymbols.
- `Kilowatts` (string, get/set) — Kilowatts property of UnitSymbols.
- `Kv` (string, get/set) — Kv property of UnitSymbols.
- `Length4Centimeter` (string, get/set) — Length4Centimeter property of UnitSymbols.
- `Length4Inch` (string, get/set) — Length4Inch property of UnitSymbols.
- `Length4Millimeter` (string, get/set) — Length4Millimeter property of UnitSymbols.
- `Liter` (string, get/set) — Liter property of UnitSymbols.
- `LitresPerSecond` (string, get/set) — LitresPerSecond property of UnitSymbols.
- `Meter` (string, get/set) — Meter property of UnitSymbols.
- `MetersPerSecond` (string, get/set) — MetersPerSecond property of UnitSymbols.
- `Mile` (string, get/set) — Mile property of UnitSymbols.
- `Milligram` (string, get/set) — Milligram property of UnitSymbols.
- `Millimeter` (string, get/set) — Millimeter property of UnitSymbols.
- `NaN` (string, get/set) — NaN property of UnitSymbols.
- `NegativeInfinity` (string, get/set) — NegativeInfinity property of UnitSymbols.
- `Newton` (string, get/set) — Newton property of UnitSymbols.
- `NewtonsPerCubicMeter` (string, get/set) — NewtonsPerCubicMeter property of UnitSymbols.
- `Ounce` (string, get/set) — Ounce property of UnitSymbols.
- `PascalSeconds` (string, get/set) — PascalSeconds property of UnitSymbols.
- `PascalsPerMeter` (string, get/set) — PascalsPerMeter property of UnitSymbols.
- `Percent` (string, get/set) — Percent property of UnitSymbols.
- `Permille` (string, get/set) — Permille property of UnitSymbols.
- `PositiveInfinity` (string, get/set) — PositiveInfinity property of UnitSymbols.
- `Pound` (string, get/set) — Pound property of UnitSymbols.
- `PoundForce` (string, get/set) — PoundForce property of UnitSymbols.
- `PoundForcePerCubicFoot` (string, get/set) — PoundForcePerCubicFoot property of UnitSymbols.
- `PoundsPerSecond` (string, get/set) — PoundsPerSecond property of UnitSymbols.
- `PoundsPerSquareFootSeconds` (string, get/set) — PoundsPerSquareFootSeconds property of UnitSymbols.
- `PoundsPerSquareInchPerFoot` (string, get/set) — PoundsPerSquareInchPerFoot property of UnitSymbols.
- `Radian` (string, get/set) — Radian property of UnitSymbols.
- `SeparatorBetweenFeetAndInches` (string, get/set) — SeparatorBetweenFeetAndInches property of UnitSymbols.
- `SeparatorBetweenInchesAndFractions` (string, get/set) — SeparatorBetweenInchesAndFractions property of UnitSymbols.
- `SeparatorBetweenNumberAndUnitSymbol` (string, get/set) — SeparatorBetweenNumberAndUnitSymbol property of UnitSymbols.
- `ShortTon` (string, get/set) — ShortTon property of UnitSymbols.
- `SquareCentimeter` (string, get/set) — SquareCentimeter property of UnitSymbols.
- `SquareFeetPerSecond` (string, get/set) — SquareFeetPerSecond property of UnitSymbols.
- `SquareFoot` (string, get/set) — SquareFoot property of UnitSymbols.
- `SquareInch` (string, get/set) — SquareInch property of UnitSymbols.
- `SquareMeter` (string, get/set) — SquareMeter property of UnitSymbols.
- `SquareMetersPerSecond` (string, get/set) — SquareMetersPerSecond property of UnitSymbols.
- `SquareMillimeter` (string, get/set) — SquareMillimeter property of UnitSymbols.
- `SquareYard` (string, get/set) — SquareYard property of UnitSymbols.
- `Tonne` (string, get/set) — Tonne property of UnitSymbols.
- `TonneCapitalized` (string, get/set) — TonneCapitalized property of UnitSymbols.
- `TonsPerCubicMeter` (string, get/set) — TonsPerCubicMeter property of UnitSymbols.
- `USSurveyFoot` (string, get/set) — USSurveyFoot property of UnitSymbols.
- `USSurveyInch` (string, get/set) — USSurveyInch property of UnitSymbols.
- `USSurveyMile` (string, get/set) — USSurveyMile property of UnitSymbols.
- `USSurveyYard` (string, get/set) — USSurveyYard property of UnitSymbols.
- `WarpingCentimeter` (string, get/set) — WarpingCentimeter property of UnitSymbols.
- `WarpingInch` (string, get/set) — WarpingInch property of UnitSymbols.
- `WarpingMillimeter` (string, get/set) — WarpingMillimeter property of UnitSymbols.
- `Watts` (string, get/set) — Watts property of UnitSymbols.
- `Yard` (string, get/set) — Yard property of UnitSymbols.
- `Zero` (string, get/set) — Zero property of UnitSymbols.

## VelocityFormatter (class)

VelocityFormatter class in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.VelocityFormatter)

### Constructors
- `VelocityFormatter()` — Constructs a new VelocityFormatter.
- `VelocityFormatter(Fusion.VelocityUnit unit)` — Constructs a new VelocityFormatter.
- `VelocityFormatter(Fusion.VelocityUnit unit, Fusion.VelocityUnit formatUnit)` — Constructs a new VelocityFormatter.
- `VelocityFormatter(Fusion.VelocityUnit unit, Fusion.VelocityUnit formatUnit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new VelocityFormatter.
- `VelocityFormatter(Fusion.VelocityUnit unit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new VelocityFormatter.

### Methods
#### `static System.Collections.Generic.IDictionary<string, Fusion.Formatting.VelocityFormatter> ReadSettings(System.Collections.Generic.IDictionary<string, string> settings)`

ReadSettings method of VelocityFormatter.

**Parameters:**
- `settings` (System.Collections.Generic.IDictionary<string, string>)

**Returns:** `System.Collections.Generic.IDictionary<string, Fusion.Formatting.VelocityFormatter>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.VelocityFormatter.ReadSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

#### `string ToString(Fusion.Velocity value, Fusion.Formatting.UnitSymbols symbols)`

ToString method of VelocityFormatter.

**Parameters:**
- `value` (Fusion.Velocity)
- `symbols` (Fusion.Formatting.UnitSymbols)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.VelocityFormatter.ToString%28Fusion.Velocity%2CFusion.Formatting.UnitSymbols%29)

#### `bool TryParse(string text, Fusion.Formatting.UnitSymbols symbols, out Fusion.Velocity value)`

TryParse method of VelocityFormatter.

**Parameters:**
- `text` (string)
- `symbols` (Fusion.Formatting.UnitSymbols)
- `value` (Fusion.Velocity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.VelocityFormatter.TryParse%28System.String%2CFusion.Formatting.UnitSymbols%2CFusion.Velocity%40%29)

#### `static void WriteSettings(System.Collections.Generic.IDictionary<string, Fusion.Formatting.VelocityFormatter> formatters, System.Collections.Generic.IDictionary<string, string> settings)`

WriteSettings method of VelocityFormatter.

**Parameters:**
- `formatters` (System.Collections.Generic.IDictionary<string, Fusion.Formatting.VelocityFormatter>)
- `settings` (System.Collections.Generic.IDictionary<string, string>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.VelocityFormatter.WriteSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CFusion.Formatting.VelocityFormatter%7D%2CSystem.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

## VolumeFlowFormatter (class)

VolumeFlowFormatter class in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.VolumeFlowFormatter)

### Constructors
- `VolumeFlowFormatter()` — Constructs a new VolumeFlowFormatter.
- `VolumeFlowFormatter(Fusion.VolumeFlowUnit unit)` — Constructs a new VolumeFlowFormatter.
- `VolumeFlowFormatter(Fusion.VolumeFlowUnit unit, Fusion.VolumeFlowUnit formatUnit)` — Constructs a new VolumeFlowFormatter.
- `VolumeFlowFormatter(Fusion.VolumeFlowUnit unit, Fusion.VolumeFlowUnit formatUnit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new VolumeFlowFormatter.
- `VolumeFlowFormatter(Fusion.VolumeFlowUnit unit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new VolumeFlowFormatter.

### Methods
#### `static System.Collections.Generic.IDictionary<string, Fusion.Formatting.VolumeFlowFormatter> ReadSettings(System.Collections.Generic.IDictionary<string, string> settings)`

ReadSettings method of VolumeFlowFormatter.

**Parameters:**
- `settings` (System.Collections.Generic.IDictionary<string, string>)

**Returns:** `System.Collections.Generic.IDictionary<string, Fusion.Formatting.VolumeFlowFormatter>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.VolumeFlowFormatter.ReadSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

#### `string ToString(Fusion.VolumeFlow value, Fusion.Formatting.UnitSymbols symbols)`

ToString method of VolumeFlowFormatter.

**Parameters:**
- `value` (Fusion.VolumeFlow)
- `symbols` (Fusion.Formatting.UnitSymbols)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.VolumeFlowFormatter.ToString%28Fusion.VolumeFlow%2CFusion.Formatting.UnitSymbols%29)

#### `bool TryParse(string text, Fusion.Formatting.UnitSymbols symbols, out Fusion.VolumeFlow value)`

TryParse method of VolumeFlowFormatter.

**Parameters:**
- `text` (string)
- `symbols` (Fusion.Formatting.UnitSymbols)
- `value` (Fusion.VolumeFlow)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.VolumeFlowFormatter.TryParse%28System.String%2CFusion.Formatting.UnitSymbols%2CFusion.VolumeFlow%40%29)

#### `static void WriteSettings(System.Collections.Generic.IDictionary<string, Fusion.Formatting.VolumeFlowFormatter> formatters, System.Collections.Generic.IDictionary<string, string> settings)`

WriteSettings method of VolumeFlowFormatter.

**Parameters:**
- `formatters` (System.Collections.Generic.IDictionary<string, Fusion.Formatting.VolumeFlowFormatter>)
- `settings` (System.Collections.Generic.IDictionary<string, string>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.VolumeFlowFormatter.WriteSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CFusion.Formatting.VolumeFlowFormatter%7D%2CSystem.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

## VolumeFormatter (class)

VolumeFormatter class in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.VolumeFormatter)

### Constructors
- `VolumeFormatter()` — Constructs a new VolumeFormatter.
- `VolumeFormatter(Fusion.VolumeUnit unit)` — Constructs a new VolumeFormatter.
- `VolumeFormatter(Fusion.VolumeUnit unit, Fusion.VolumeUnit formatUnit)` — Constructs a new VolumeFormatter.
- `VolumeFormatter(Fusion.VolumeUnit unit, Fusion.VolumeUnit formatUnit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new VolumeFormatter.
- `VolumeFormatter(Fusion.VolumeUnit unit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new VolumeFormatter.

### Methods
#### `static System.Collections.Generic.IDictionary<string, Fusion.Formatting.VolumeFormatter> ReadSettings(System.Collections.Generic.IDictionary<string, string> settings)`

ReadSettings method of VolumeFormatter.

**Parameters:**
- `settings` (System.Collections.Generic.IDictionary<string, string>)

**Returns:** `System.Collections.Generic.IDictionary<string, Fusion.Formatting.VolumeFormatter>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.VolumeFormatter.ReadSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

#### `string ToString(Fusion.Volume value, Fusion.Formatting.UnitSymbols symbols)`

ToString method of VolumeFormatter.

**Parameters:**
- `value` (Fusion.Volume)
- `symbols` (Fusion.Formatting.UnitSymbols)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.VolumeFormatter.ToString%28Fusion.Volume%2CFusion.Formatting.UnitSymbols%29)

#### `bool TryParse(string text, Fusion.Formatting.UnitSymbols symbols, out Fusion.Volume value)`

TryParse method of VolumeFormatter.

**Parameters:**
- `text` (string)
- `symbols` (Fusion.Formatting.UnitSymbols)
- `value` (Fusion.Volume)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.VolumeFormatter.TryParse%28System.String%2CFusion.Formatting.UnitSymbols%2CFusion.Volume%40%29)

#### `static void WriteSettings(System.Collections.Generic.IDictionary<string, Fusion.Formatting.VolumeFormatter> formatters, System.Collections.Generic.IDictionary<string, string> settings)`

WriteSettings method of VolumeFormatter.

**Parameters:**
- `formatters` (System.Collections.Generic.IDictionary<string, Fusion.Formatting.VolumeFormatter>)
- `settings` (System.Collections.Generic.IDictionary<string, string>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.VolumeFormatter.WriteSettings%28System.Collections.Generic.IDictionary%7BSystem.String%2CFusion.Formatting.VolumeFormatter%7D%2CSystem.Collections.Generic.IDictionary%7BSystem.String%2CSystem.String%7D%29)

## WarpingFormatter (class)

WarpingFormatter class in Fusion.Formatting.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Formatting.WarpingFormatter)

### Constructors
- `WarpingFormatter()` — Constructs a new WarpingFormatter.
- `WarpingFormatter(Fusion.WarpingUnit unit)` — Constructs a new WarpingFormatter.
- `WarpingFormatter(Fusion.WarpingUnit unit, Fusion.WarpingUnit formatUnit)` — Constructs a new WarpingFormatter.
- `WarpingFormatter(Fusion.WarpingUnit unit, Fusion.WarpingUnit formatUnit, string formatString)` — Constructs a new WarpingFormatter.
- `WarpingFormatter(Fusion.WarpingUnit unit, Fusion.WarpingUnit formatUnit, string formatString, System.IFormatProvider formatProvider)` — Constructs a new WarpingFormatter.

### Methods
#### `string ToString(double value, Fusion.Formatting.UnitSymbols symbols)`

ToString method of WarpingFormatter.

**Parameters:**
- `value` (double)
- `symbols` (Fusion.Formatting.UnitSymbols)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.WarpingFormatter.ToString%28System.Double%2CFusion.Formatting.UnitSymbols%29)

#### `bool TryParse(string text, Fusion.Formatting.UnitSymbols symbols, out double value)`

TryParse method of WarpingFormatter.

**Parameters:**
- `text` (string)
- `symbols` (Fusion.Formatting.UnitSymbols)
- `value` (double)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Formatting.WarpingFormatter.TryParse%28System.String%2CFusion.Formatting.UnitSymbols%2CSystem.Double%40%29)

