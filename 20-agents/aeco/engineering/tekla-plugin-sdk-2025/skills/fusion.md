---
name: tekla-plugin-sdk-fusion
description: This skill encodes the tekla-plugin-sdk 2025.0 surface of the Fusion namespace — 125 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Angle, AngleUnit, App, Area, AreaPerLength, AreaUnit, AuthorizationAvailableResponse, AuthorizationResponse, and 117 more types.
---

# Fusion

Auto-generated from vendor docs for tekla-plugin-sdk 2025.0. 125 types in this namespace.

## Angle (struct)

Angle struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Angle)

### Constructors
- `Angle(double degrees)` — Constructs a new Angle.

### Methods
#### `static Fusion.Angle Abs(Fusion.Angle value)`

Abs method of Angle.

**Parameters:**
- `value` (Fusion.Angle)

**Returns:** `Fusion.Angle` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.Abs%28Fusion.Angle%29)

#### `static Fusion.Angle Acos(double cos)`

Acos method of Angle.

**Parameters:**
- `cos` (double)

**Returns:** `Fusion.Angle` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.Acos%28System.Double%29)

#### `static Fusion.Angle Asin(double sin)`

Asin method of Angle.

**Parameters:**
- `sin` (double)

**Returns:** `Fusion.Angle` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.Asin%28System.Double%29)

#### `static Fusion.Angle Atan(double tan)`

Atan method of Angle.

**Parameters:**
- `tan` (double)

**Returns:** `Fusion.Angle` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.Atan%28System.Double%29)

#### `static Fusion.Angle Atan(double y, double x)`

Atan method of Angle.

**Parameters:**
- `y` (double)
- `x` (double)

**Returns:** `Fusion.Angle` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.Atan%28System.Double%2CSystem.Double%29)

#### `int CompareTo(Fusion.Angle other)`

CompareTo method of Angle.

**Parameters:**
- `other` (Fusion.Angle)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.CompareTo%28Fusion.Angle%29)

#### `int CompareTo(object other)`

CompareTo method of Angle.

**Parameters:**
- `other` (object)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.CompareTo%28System.Object%29)

#### `bool Equals(Fusion.Angle other)`

Equals method of Angle.

**Parameters:**
- `other` (Fusion.Angle)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.Equals%28Fusion.Angle%29)

#### `bool Equals(object other)`

Equals method of Angle.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.Equals%28System.Object%29)

#### `static Fusion.Angle FromRadians(double radians)`

FromRadians method of Angle.

**Parameters:**
- `radians` (double)

**Returns:** `Fusion.Angle` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.FromRadians%28System.Double%29)

#### `int GetHashCode()`

GetHashCode method of Angle.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.GetHashCode)

#### `static bool IsInfinity(Fusion.Angle value)`

IsInfinity method of Angle.

**Parameters:**
- `value` (Fusion.Angle)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.IsInfinity%28Fusion.Angle%29)

#### `static bool IsNaN(Fusion.Angle value)`

IsNaN method of Angle.

**Parameters:**
- `value` (Fusion.Angle)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.IsNaN%28Fusion.Angle%29)

#### `static bool IsNegativeInfinity(Fusion.Angle value)`

IsNegativeInfinity method of Angle.

**Parameters:**
- `value` (Fusion.Angle)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.IsNegativeInfinity%28Fusion.Angle%29)

#### `static bool IsPositiveInfinity(Fusion.Angle value)`

IsPositiveInfinity method of Angle.

**Parameters:**
- `value` (Fusion.Angle)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.IsPositiveInfinity%28Fusion.Angle%29)

#### `static Fusion.Angle Max(Fusion.Angle a, Fusion.Angle b)`

Max method of Angle.

**Parameters:**
- `a` (Fusion.Angle)
- `b` (Fusion.Angle)

**Returns:** `Fusion.Angle` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.Max%28Fusion.Angle%2CFusion.Angle%29)

#### `static Fusion.Angle Min(Fusion.Angle a, Fusion.Angle b)`

Min method of Angle.

**Parameters:**
- `a` (Fusion.Angle)
- `b` (Fusion.Angle)

**Returns:** `Fusion.Angle` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.Min%28Fusion.Angle%2CFusion.Angle%29)

#### `bool Near(Fusion.Angle other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of Angle.

**Parameters:**
- `other` (Fusion.Angle)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.Near%28Fusion.Angle%2CSystem.Int32%29)

#### `static Fusion.Angle Round(Fusion.Angle angle, Fusion.Angle precision, Fusion.RoundingMode roundingMode)`

Round method of Angle.

**Parameters:**
- `angle` (Fusion.Angle)
- `precision` (Fusion.Angle)
- `roundingMode` (Fusion.RoundingMode)

**Returns:** `Fusion.Angle` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.Round%28Fusion.Angle%2CFusion.Angle%2CFusion.RoundingMode%29)

#### `static int Sign(Fusion.Angle value)`

Sign method of Angle.

**Parameters:**
- `value` (Fusion.Angle)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.Sign%28Fusion.Angle%29)

#### `double To(Fusion.AngleUnit unit)`

To method of Angle.

**Parameters:**
- `unit` (Fusion.AngleUnit)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.To%28Fusion.AngleUnit%29)

#### `static double ToDegrees(double radians)`

ToDegrees method of Angle.

**Parameters:**
- `radians` (double)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.ToDegrees%28System.Double%29)

#### `static double ToRadians(double degrees)`

ToRadians method of Angle.

**Parameters:**
- `degrees` (double)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.ToRadians%28System.Double%29)

#### `string ToString()`

ToString method of Angle.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of Angle.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.Angle op_Addition(Fusion.Angle a, Fusion.Angle b)`

op_Addition method of Angle.

**Parameters:**
- `a` (Fusion.Angle)
- `b` (Fusion.Angle)

**Returns:** `Fusion.Angle` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.op_Addition%28Fusion.Angle%2CFusion.Angle%29)

#### `static Fusion.Angle op_Division(Fusion.Angle a, double b)`

op_Division method of Angle.

**Parameters:**
- `a` (Fusion.Angle)
- `b` (double)

**Returns:** `Fusion.Angle` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.op_Division%28Fusion.Angle%2CSystem.Double%29)

#### `static double op_Division(Fusion.Angle a, Fusion.Angle b)`

op_Division method of Angle.

**Parameters:**
- `a` (Fusion.Angle)
- `b` (Fusion.Angle)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.op_Division%28Fusion.Angle%2CFusion.Angle%29)

#### `static bool op_Equality(Fusion.Angle a, Fusion.Angle b)`

op_Equality method of Angle.

**Parameters:**
- `a` (Fusion.Angle)
- `b` (Fusion.Angle)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.op_Equality%28Fusion.Angle%2CFusion.Angle%29)

#### `static bool op_GreaterThan(Fusion.Angle a, Fusion.Angle b)`

op_GreaterThan method of Angle.

**Parameters:**
- `a` (Fusion.Angle)
- `b` (Fusion.Angle)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.op_GreaterThan%28Fusion.Angle%2CFusion.Angle%29)

#### `static bool op_GreaterThanOrEqual(Fusion.Angle a, Fusion.Angle b)`

op_GreaterThanOrEqual method of Angle.

**Parameters:**
- `a` (Fusion.Angle)
- `b` (Fusion.Angle)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.op_GreaterThanOrEqual%28Fusion.Angle%2CFusion.Angle%29)

#### `static Fusion.Angle op_Implicit(double degrees)`

op_Implicit method of Angle.

**Parameters:**
- `degrees` (double)

**Returns:** `Fusion.Angle` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.op_Implicit%28System.Double%29~Fusion.Angle)

#### `static bool op_Inequality(Fusion.Angle a, Fusion.Angle b)`

op_Inequality method of Angle.

**Parameters:**
- `a` (Fusion.Angle)
- `b` (Fusion.Angle)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.op_Inequality%28Fusion.Angle%2CFusion.Angle%29)

#### `static bool op_LessThan(Fusion.Angle a, Fusion.Angle b)`

op_LessThan method of Angle.

**Parameters:**
- `a` (Fusion.Angle)
- `b` (Fusion.Angle)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.op_LessThan%28Fusion.Angle%2CFusion.Angle%29)

#### `static bool op_LessThanOrEqual(Fusion.Angle a, Fusion.Angle b)`

op_LessThanOrEqual method of Angle.

**Parameters:**
- `a` (Fusion.Angle)
- `b` (Fusion.Angle)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.op_LessThanOrEqual%28Fusion.Angle%2CFusion.Angle%29)

#### `static Fusion.Angle op_Modulus(Fusion.Angle a, Fusion.Angle b)`

op_Modulus method of Angle.

**Parameters:**
- `a` (Fusion.Angle)
- `b` (Fusion.Angle)

**Returns:** `Fusion.Angle` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.op_Modulus%28Fusion.Angle%2CFusion.Angle%29)

#### `static Fusion.Angle op_Multiply(Fusion.Angle a, double b)`

op_Multiply method of Angle.

**Parameters:**
- `a` (Fusion.Angle)
- `b` (double)

**Returns:** `Fusion.Angle` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.op_Multiply%28Fusion.Angle%2CSystem.Double%29)

#### `static Fusion.Angle op_Multiply(double a, Fusion.Angle b)`

op_Multiply method of Angle.

**Parameters:**
- `a` (double)
- `b` (Fusion.Angle)

**Returns:** `Fusion.Angle` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.op_Multiply%28System.Double%2CFusion.Angle%29)

#### `static Fusion.Angle op_Subtraction(Fusion.Angle a, Fusion.Angle b)`

op_Subtraction method of Angle.

**Parameters:**
- `a` (Fusion.Angle)
- `b` (Fusion.Angle)

**Returns:** `Fusion.Angle` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.op_Subtraction%28Fusion.Angle%2CFusion.Angle%29)

#### `static Fusion.Angle op_UnaryNegation(Fusion.Angle v)`

op_UnaryNegation method of Angle.

**Parameters:**
- `v` (Fusion.Angle)

**Returns:** `Fusion.Angle` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.op_UnaryNegation%28Fusion.Angle%29)

#### `static Fusion.Angle op_UnaryPlus(Fusion.Angle v)`

op_UnaryPlus method of Angle.

**Parameters:**
- `v` (Fusion.Angle)

**Returns:** `Fusion.Angle` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Angle.op_UnaryPlus%28Fusion.Angle%29)

### Properties
- `Cos` (double, get) — Cos property of Angle.
- `Degrees` (double, get) — Degrees property of Angle.
- `Radians` (double, get) — Radians property of Angle.
- `Sin` (double, get) — Sin property of Angle.
- `Tan` (double, get) — Tan property of Angle.

## AngleUnit (enum)

AngleUnit enum in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.AngleUnit)

### Values
- `Degrees` = `1`
- `Radians` = `2`
- `DegMinSec` = `3`

## App (class)

App class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.App)

### Constructors
- `App()` — Constructs a new App.
- `App(Fusion.App.Feature[] features)` — Constructs a new App.

### Methods
#### `static System.IDisposable BlockingOperation()`

BlockingOperation method of App.

**Returns:** `System.IDisposable` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.App.BlockingOperation)

#### `static System.IDisposable Busy(System.Windows.Input.Cursor busyCursor = null)`

Busy method of App.

**Parameters:**
- `busyCursor` (System.Windows.Input.Cursor)

**Returns:** `System.IDisposable` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.App.Busy%28System.Windows.Input.Cursor%29)

#### `System.Windows.FrameworkElement CreateView(string viewName, object parameter)`

CreateView method of App.

**Parameters:**
- `viewName` (string)
- `parameter` (object)

**Returns:** `System.Windows.FrameworkElement` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.App.CreateView%28System.String%2CSystem.Object%29)

#### `System.Windows.Controls.Primitives.Popup CreateViewInPopup(string viewName, object parameter)`

CreateViewInPopup method of App.

**Parameters:**
- `viewName` (string)
- `parameter` (object)

**Returns:** `System.Windows.Controls.Primitives.Popup` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.App.CreateViewInPopup%28System.String%2CSystem.Object%29)

#### `System.Windows.Window CreateViewInWindow(string viewName, object parameter)`

CreateViewInWindow method of App.

**Parameters:**
- `viewName` (string)
- `parameter` (object)

**Returns:** `System.Windows.Window` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.App.CreateViewInWindow%28System.String%2CSystem.Object%29)

#### `System.Threading.Tasks.Task Idle()`

Idle method of App.

**Returns:** `System.Threading.Tasks.Task` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.App.Idle)

#### `static System.Threading.Tasks.Task Invoke(string publishedMethodSignature, object[] arguments)`

Invoke method of App.

**Parameters:**
- `publishedMethodSignature` (string)
- `arguments` (object[])

**Returns:** `System.Threading.Tasks.Task` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.App.Invoke%28System.String%2CSystem.Object%5B%5D%29)

#### `static System.Threading.Tasks.Task<T> Invoke<T>(string publishedMethodSignature, object[] arguments)`

Invoke method of App.

**Parameters:**
- `publishedMethodSignature` (string)
- `arguments` (object[])

**Returns:** `System.Threading.Tasks.Task<T>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.App.Invoke%60%601%28System.String%2CSystem.Object%5B%5D%29)

#### `static void InvokeAsync(System.Action action)`

InvokeAsync method of App.

**Parameters:**
- `action` (System.Action)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.App.InvokeAsync%28System.Action%29)

#### `static System.Threading.Tasks.Task InvokeDomainLogic(System.Action action)`

InvokeDomainLogic method of App.

**Parameters:**
- `action` (System.Action)

**Returns:** `System.Threading.Tasks.Task` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.App.InvokeDomainLogic%28System.Action%29)

#### `static System.Threading.Tasks.Task<T> InvokeDomainLogic<T>(System.Func<T> action)`

InvokeDomainLogic method of App.

**Parameters:**
- `action` (System.Func<T>)

**Returns:** `System.Threading.Tasks.Task<T>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.App.InvokeDomainLogic%60%601%28System.Func%7B%60%600%7D%29)

#### `static System.IDisposable NotBusy()`

NotBusy method of App.

**Returns:** `System.IDisposable` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.App.NotBusy)

#### `System.IO.Stream OpenFile(string path)`

OpenFile method of App.

**Parameters:**
- `path` (string)

**Returns:** `System.IO.Stream` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.App.OpenFile%28System.String%29)

#### `void RaisePropertyChanged(string propertyName = null)`

RaisePropertyChanged method of App.

**Parameters:**
- `propertyName` (string)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.App.RaisePropertyChanged%28System.String%29)

#### `void RaisePropertyChanged(string propertyName1, string propertyName2, string[] propertyNames)`

RaisePropertyChanged method of App.

**Parameters:**
- `propertyName1` (string)
- `propertyName2` (string)
- `propertyNames` (string[])

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.App.RaisePropertyChanged%28System.String%2CSystem.String%2CSystem.String%5B%5D%29)

#### `void Restart()`

Restart method of App.

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.App.Restart)

#### `int Run()`

Run method of App.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.App.Run)

#### `static void Start(Fusion.App application)`

Start method of App.

**Parameters:**
- `application` (Fusion.App)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.App.Start%28Fusion.App%29)

#### `static void Start(Fusion.App application, Fusion.App.SettingsOverrides settingsOverrides)`

Start method of App.

**Parameters:**
- `application` (Fusion.App)
- `settingsOverrides` (Fusion.App.SettingsOverrides)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.App.Start%28Fusion.App%2CFusion.App.SettingsOverrides%29)

### Properties
- `Company` (string, get) — Company property of App.
- `Current` (Fusion.App, get) — Current property of App.
- `DataFolder` (string, get) — DataFolder property of App.
- `DialogOwner` (System.Windows.Window, get) — DialogOwner property of App.
- `Executable` (string, get) — Executable property of App.
- `FeatureFolder` (string, get) — FeatureFolder property of App.
- `Features` (System.Collections.Generic.IEnumerable<Fusion.App.Feature>, get) — Features property of App.
- `Folder` (string, get) — Folder property of App.
- `FullScreenMode` (bool, get/set) — FullScreenMode property of App.
- `Host` (Fusion.IHost, get) — Host property of App.
- `Identifier` (string, get) — Identifier property of App.
- `InstanceIdentifier` (string, get) — InstanceIdentifier property of App.
- `Name` (string, get) — Name property of App.
- `PublishedViews` (System.Collections.Generic.IEnumerable<Fusion.PublishedViewInfo>, get) — PublishedViews property of App.
- `Recovery` (Fusion.IRecovery, get) — Recovery property of App.
- `RecoveryPath` (string, get) — RecoveryPath property of App.
- `RegistryPath` (string, get) — RegistryPath property of App.
- `Session` (Fusion.ISession, get) — Session property of App.
- `SessionPath` (string, get) — SessionPath property of App.
- `Settings` (Fusion.ISettings, get) — Settings property of App.
- `SettingsPath` (string, get) — SettingsPath property of App.
- `TabletMode` (bool, get/set) — TabletMode property of App.
- `TempFolder` (string, get) — TempFolder property of App.
- `Version` (string, get) — Version property of App.
- `WindowOwner` (System.Windows.Window, get) — WindowOwner property of App.

### Events
#### `KeyDown` (`System.Windows.Input.KeyEventHandler`)

**Signature:** `event System.Windows.Input.KeyEventHandler KeyDown`

KeyDown event of App.

[Docs](https://www.nuget.org/packages/TeklaFusion#E%3AFusion.App.KeyDown)

#### `KeyUp` (`System.Windows.Input.KeyEventHandler`)

**Signature:** `event System.Windows.Input.KeyEventHandler KeyUp`

KeyUp event of App.

[Docs](https://www.nuget.org/packages/TeklaFusion#E%3AFusion.App.KeyUp)

#### `PropertyChanged` (`System.ComponentModel.PropertyChangedEventHandler`)

**Signature:** `event System.ComponentModel.PropertyChangedEventHandler PropertyChanged`

PropertyChanged event of App.

[Docs](https://www.nuget.org/packages/TeklaFusion#E%3AFusion.App.PropertyChanged)

## Area (struct)

Area struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Area)

### Constructors
- `Area(double squareMillimeters)` — Constructs a new Area.

### Methods
#### `static Fusion.Area Abs(Fusion.Area area)`

Abs method of Area.

**Parameters:**
- `area` (Fusion.Area)

**Returns:** `Fusion.Area` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.Abs%28Fusion.Area%29)

#### `int CompareTo(Fusion.Area other)`

CompareTo method of Area.

**Parameters:**
- `other` (Fusion.Area)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.CompareTo%28Fusion.Area%29)

#### `int CompareTo(object other)`

CompareTo method of Area.

**Parameters:**
- `other` (object)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.CompareTo%28System.Object%29)

#### `bool Equals(Fusion.Area other)`

Equals method of Area.

**Parameters:**
- `other` (Fusion.Area)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.Equals%28Fusion.Area%29)

#### `bool Equals(object other)`

Equals method of Area.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.Equals%28System.Object%29)

#### `static Fusion.Area From(double area, Fusion.AreaUnit areaUnit)`

From method of Area.

**Parameters:**
- `area` (double)
- `areaUnit` (Fusion.AreaUnit)

**Returns:** `Fusion.Area` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.From%28System.Double%2CFusion.AreaUnit%29)

#### `int GetHashCode()`

GetHashCode method of Area.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.GetHashCode)

#### `static bool IsInfinity(Fusion.Area area)`

IsInfinity method of Area.

**Parameters:**
- `area` (Fusion.Area)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.IsInfinity%28Fusion.Area%29)

#### `static bool IsNaN(Fusion.Area area)`

IsNaN method of Area.

**Parameters:**
- `area` (Fusion.Area)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.IsNaN%28Fusion.Area%29)

#### `static bool IsNegativeInfinity(Fusion.Area area)`

IsNegativeInfinity method of Area.

**Parameters:**
- `area` (Fusion.Area)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.IsNegativeInfinity%28Fusion.Area%29)

#### `static bool IsPositiveInfinity(Fusion.Area area)`

IsPositiveInfinity method of Area.

**Parameters:**
- `area` (Fusion.Area)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.IsPositiveInfinity%28Fusion.Area%29)

#### `static Fusion.Area Max(Fusion.Area a, Fusion.Area b)`

Max method of Area.

**Parameters:**
- `a` (Fusion.Area)
- `b` (Fusion.Area)

**Returns:** `Fusion.Area` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.Max%28Fusion.Area%2CFusion.Area%29)

#### `static Fusion.Area Min(Fusion.Area a, Fusion.Area b)`

Min method of Area.

**Parameters:**
- `a` (Fusion.Area)
- `b` (Fusion.Area)

**Returns:** `Fusion.Area` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.Min%28Fusion.Area%2CFusion.Area%29)

#### `bool Near(Fusion.Area other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of Area.

**Parameters:**
- `other` (Fusion.Area)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.Near%28Fusion.Area%2CSystem.Int32%29)

#### `static Fusion.Area Round(Fusion.Area area, Fusion.Area precision, Fusion.RoundingMode roundingMode)`

Round method of Area.

**Parameters:**
- `area` (Fusion.Area)
- `precision` (Fusion.Area)
- `roundingMode` (Fusion.RoundingMode)

**Returns:** `Fusion.Area` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.Round%28Fusion.Area%2CFusion.Area%2CFusion.RoundingMode%29)

#### `static int Sign(Fusion.Area area)`

Sign method of Area.

**Parameters:**
- `area` (Fusion.Area)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.Sign%28Fusion.Area%29)

#### `double To(Fusion.AreaUnit areaUnit)`

To method of Area.

**Parameters:**
- `areaUnit` (Fusion.AreaUnit)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.To%28Fusion.AreaUnit%29)

#### `string ToString()`

ToString method of Area.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of Area.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.Area op_Addition(Fusion.Area a, Fusion.Area b)`

op_Addition method of Area.

**Parameters:**
- `a` (Fusion.Area)
- `b` (Fusion.Area)

**Returns:** `Fusion.Area` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.op_Addition%28Fusion.Area%2CFusion.Area%29)

#### `static Fusion.Area op_Division(Fusion.Area a, double b)`

op_Division method of Area.

**Parameters:**
- `a` (Fusion.Area)
- `b` (double)

**Returns:** `Fusion.Area` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.op_Division%28Fusion.Area%2CSystem.Double%29)

#### `static double op_Division(Fusion.Area a, Fusion.Area b)`

op_Division method of Area.

**Parameters:**
- `a` (Fusion.Area)
- `b` (Fusion.Area)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.op_Division%28Fusion.Area%2CFusion.Area%29)

#### `static bool op_Equality(Fusion.Area a, Fusion.Area b)`

op_Equality method of Area.

**Parameters:**
- `a` (Fusion.Area)
- `b` (Fusion.Area)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.op_Equality%28Fusion.Area%2CFusion.Area%29)

#### `static bool op_GreaterThan(Fusion.Area a, Fusion.Area b)`

op_GreaterThan method of Area.

**Parameters:**
- `a` (Fusion.Area)
- `b` (Fusion.Area)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.op_GreaterThan%28Fusion.Area%2CFusion.Area%29)

#### `static bool op_GreaterThanOrEqual(Fusion.Area a, Fusion.Area b)`

op_GreaterThanOrEqual method of Area.

**Parameters:**
- `a` (Fusion.Area)
- `b` (Fusion.Area)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.op_GreaterThanOrEqual%28Fusion.Area%2CFusion.Area%29)

#### `static bool op_Inequality(Fusion.Area a, Fusion.Area b)`

op_Inequality method of Area.

**Parameters:**
- `a` (Fusion.Area)
- `b` (Fusion.Area)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.op_Inequality%28Fusion.Area%2CFusion.Area%29)

#### `static bool op_LessThan(Fusion.Area a, Fusion.Area b)`

op_LessThan method of Area.

**Parameters:**
- `a` (Fusion.Area)
- `b` (Fusion.Area)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.op_LessThan%28Fusion.Area%2CFusion.Area%29)

#### `static bool op_LessThanOrEqual(Fusion.Area a, Fusion.Area b)`

op_LessThanOrEqual method of Area.

**Parameters:**
- `a` (Fusion.Area)
- `b` (Fusion.Area)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.op_LessThanOrEqual%28Fusion.Area%2CFusion.Area%29)

#### `static Fusion.Area op_Modulus(Fusion.Area a, Fusion.Area b)`

op_Modulus method of Area.

**Parameters:**
- `a` (Fusion.Area)
- `b` (Fusion.Area)

**Returns:** `Fusion.Area` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.op_Modulus%28Fusion.Area%2CFusion.Area%29)

#### `static Fusion.Area op_Multiply(Fusion.Area a, double b)`

op_Multiply method of Area.

**Parameters:**
- `a` (Fusion.Area)
- `b` (double)

**Returns:** `Fusion.Area` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.op_Multiply%28Fusion.Area%2CSystem.Double%29)

#### `static Fusion.Area op_Multiply(double a, Fusion.Area b)`

op_Multiply method of Area.

**Parameters:**
- `a` (double)
- `b` (Fusion.Area)

**Returns:** `Fusion.Area` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.op_Multiply%28System.Double%2CFusion.Area%29)

#### `static Fusion.Area op_Subtraction(Fusion.Area a, Fusion.Area b)`

op_Subtraction method of Area.

**Parameters:**
- `a` (Fusion.Area)
- `b` (Fusion.Area)

**Returns:** `Fusion.Area` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.op_Subtraction%28Fusion.Area%2CFusion.Area%29)

#### `static Fusion.Area op_UnaryNegation(Fusion.Area a)`

op_UnaryNegation method of Area.

**Parameters:**
- `a` (Fusion.Area)

**Returns:** `Fusion.Area` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Area.op_UnaryNegation%28Fusion.Area%29)

### Properties
- `SquareMillimeters` (double, get) — SquareMillimeters property of Area.

## AreaPerLength (struct)

AreaPerLength struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.AreaPerLength)

### Constructors
- `AreaPerLength(double squareMillimeterPerMeter)` — Constructs a new AreaPerLength.

### Methods
#### `static Fusion.AreaPerLength Abs(Fusion.AreaPerLength areaPerLength)`

Abs method of AreaPerLength.

**Parameters:**
- `areaPerLength` (Fusion.AreaPerLength)

**Returns:** `Fusion.AreaPerLength` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.Abs%28Fusion.AreaPerLength%29)

#### `int CompareTo(Fusion.AreaPerLength other)`

CompareTo method of AreaPerLength.

**Parameters:**
- `other` (Fusion.AreaPerLength)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.CompareTo%28Fusion.AreaPerLength%29)

#### `int CompareTo(object obj)`

CompareTo method of AreaPerLength.

**Parameters:**
- `obj` (object)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.CompareTo%28System.Object%29)

#### `bool Equals(Fusion.AreaPerLength other)`

Equals method of AreaPerLength.

**Parameters:**
- `other` (Fusion.AreaPerLength)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.Equals%28Fusion.AreaPerLength%29)

#### `bool Equals(object obj)`

Equals method of AreaPerLength.

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.Equals%28System.Object%29)

#### `static Fusion.AreaPerLength From(double areaPerUnitLength, Fusion.AreaPerLength.AreaPerLengthUnit areaPerLengthUnit)`

From method of AreaPerLength.

**Parameters:**
- `areaPerUnitLength` (double)
- `areaPerLengthUnit` (Fusion.AreaPerLength.AreaPerLengthUnit)

**Returns:** `Fusion.AreaPerLength` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.From%28System.Double%2CFusion.AreaPerLength.AreaPerLengthUnit%29)

#### `int GetHashCode()`

GetHashCode method of AreaPerLength.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.GetHashCode)

#### `static bool IsInfinity(Fusion.AreaPerLength areaPerLength)`

IsInfinity method of AreaPerLength.

**Parameters:**
- `areaPerLength` (Fusion.AreaPerLength)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.IsInfinity%28Fusion.AreaPerLength%29)

#### `static bool IsNaN(Fusion.AreaPerLength areaPerLength)`

IsNaN method of AreaPerLength.

**Parameters:**
- `areaPerLength` (Fusion.AreaPerLength)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.IsNaN%28Fusion.AreaPerLength%29)

#### `static bool IsNegativeInfinity(Fusion.AreaPerLength areaPerLength)`

IsNegativeInfinity method of AreaPerLength.

**Parameters:**
- `areaPerLength` (Fusion.AreaPerLength)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.IsNegativeInfinity%28Fusion.AreaPerLength%29)

#### `static bool IsPositiveInfinity(Fusion.AreaPerLength areaPerLength)`

IsPositiveInfinity method of AreaPerLength.

**Parameters:**
- `areaPerLength` (Fusion.AreaPerLength)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.IsPositiveInfinity%28Fusion.AreaPerLength%29)

#### `static Fusion.AreaPerLength Max(Fusion.AreaPerLength a, Fusion.AreaPerLength b)`

Max method of AreaPerLength.

**Parameters:**
- `a` (Fusion.AreaPerLength)
- `b` (Fusion.AreaPerLength)

**Returns:** `Fusion.AreaPerLength` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.Max%28Fusion.AreaPerLength%2CFusion.AreaPerLength%29)

#### `static Fusion.AreaPerLength Min(Fusion.AreaPerLength a, Fusion.AreaPerLength b)`

Min method of AreaPerLength.

**Parameters:**
- `a` (Fusion.AreaPerLength)
- `b` (Fusion.AreaPerLength)

**Returns:** `Fusion.AreaPerLength` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.Min%28Fusion.AreaPerLength%2CFusion.AreaPerLength%29)

#### `bool Near(Fusion.AreaPerLength other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of AreaPerLength.

**Parameters:**
- `other` (Fusion.AreaPerLength)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.Near%28Fusion.AreaPerLength%2CSystem.Int32%29)

#### `static Fusion.AreaPerLength Round(Fusion.AreaPerLength areaPerLength, Fusion.AreaPerLength precision, Fusion.RoundingMode roundingMode)`

Round method of AreaPerLength.

**Parameters:**
- `areaPerLength` (Fusion.AreaPerLength)
- `precision` (Fusion.AreaPerLength)
- `roundingMode` (Fusion.RoundingMode)

**Returns:** `Fusion.AreaPerLength` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.Round%28Fusion.AreaPerLength%2CFusion.AreaPerLength%2CFusion.RoundingMode%29)

#### `static int Sign(Fusion.AreaPerLength areaPerLength)`

Sign method of AreaPerLength.

**Parameters:**
- `areaPerLength` (Fusion.AreaPerLength)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.Sign%28Fusion.AreaPerLength%29)

#### `double To(Fusion.AreaPerLength.AreaPerLengthUnit areaPerLengthUnit)`

To method of AreaPerLength.

**Parameters:**
- `areaPerLengthUnit` (Fusion.AreaPerLength.AreaPerLengthUnit)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.To%28Fusion.AreaPerLength.AreaPerLengthUnit%29)

#### `string ToString()`

ToString method of AreaPerLength.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of AreaPerLength.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.AreaPerLength op_Addition(Fusion.AreaPerLength a, Fusion.AreaPerLength b)`

op_Addition method of AreaPerLength.

**Parameters:**
- `a` (Fusion.AreaPerLength)
- `b` (Fusion.AreaPerLength)

**Returns:** `Fusion.AreaPerLength` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.op_Addition%28Fusion.AreaPerLength%2CFusion.AreaPerLength%29)

#### `static Fusion.AreaPerLength op_Division(Fusion.AreaPerLength a, double b)`

op_Division method of AreaPerLength.

**Parameters:**
- `a` (Fusion.AreaPerLength)
- `b` (double)

**Returns:** `Fusion.AreaPerLength` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.op_Division%28Fusion.AreaPerLength%2CSystem.Double%29)

#### `static double op_Division(Fusion.AreaPerLength a, Fusion.AreaPerLength b)`

op_Division method of AreaPerLength.

**Parameters:**
- `a` (Fusion.AreaPerLength)
- `b` (Fusion.AreaPerLength)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.op_Division%28Fusion.AreaPerLength%2CFusion.AreaPerLength%29)

#### `static bool op_Equality(Fusion.AreaPerLength a, Fusion.AreaPerLength b)`

op_Equality method of AreaPerLength.

**Parameters:**
- `a` (Fusion.AreaPerLength)
- `b` (Fusion.AreaPerLength)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.op_Equality%28Fusion.AreaPerLength%2CFusion.AreaPerLength%29)

#### `static bool op_GreaterThan(Fusion.AreaPerLength a, Fusion.AreaPerLength b)`

op_GreaterThan method of AreaPerLength.

**Parameters:**
- `a` (Fusion.AreaPerLength)
- `b` (Fusion.AreaPerLength)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.op_GreaterThan%28Fusion.AreaPerLength%2CFusion.AreaPerLength%29)

#### `static bool op_GreaterThanOrEqual(Fusion.AreaPerLength a, Fusion.AreaPerLength b)`

op_GreaterThanOrEqual method of AreaPerLength.

**Parameters:**
- `a` (Fusion.AreaPerLength)
- `b` (Fusion.AreaPerLength)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.op_GreaterThanOrEqual%28Fusion.AreaPerLength%2CFusion.AreaPerLength%29)

#### `static bool op_Inequality(Fusion.AreaPerLength a, Fusion.AreaPerLength b)`

op_Inequality method of AreaPerLength.

**Parameters:**
- `a` (Fusion.AreaPerLength)
- `b` (Fusion.AreaPerLength)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.op_Inequality%28Fusion.AreaPerLength%2CFusion.AreaPerLength%29)

#### `static bool op_LessThan(Fusion.AreaPerLength a, Fusion.AreaPerLength b)`

op_LessThan method of AreaPerLength.

**Parameters:**
- `a` (Fusion.AreaPerLength)
- `b` (Fusion.AreaPerLength)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.op_LessThan%28Fusion.AreaPerLength%2CFusion.AreaPerLength%29)

#### `static bool op_LessThanOrEqual(Fusion.AreaPerLength a, Fusion.AreaPerLength b)`

op_LessThanOrEqual method of AreaPerLength.

**Parameters:**
- `a` (Fusion.AreaPerLength)
- `b` (Fusion.AreaPerLength)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.op_LessThanOrEqual%28Fusion.AreaPerLength%2CFusion.AreaPerLength%29)

#### `static Fusion.AreaPerLength op_Modulus(Fusion.AreaPerLength a, Fusion.AreaPerLength b)`

op_Modulus method of AreaPerLength.

**Parameters:**
- `a` (Fusion.AreaPerLength)
- `b` (Fusion.AreaPerLength)

**Returns:** `Fusion.AreaPerLength` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.op_Modulus%28Fusion.AreaPerLength%2CFusion.AreaPerLength%29)

#### `static Fusion.AreaPerLength op_Multiply(Fusion.AreaPerLength a, double b)`

op_Multiply method of AreaPerLength.

**Parameters:**
- `a` (Fusion.AreaPerLength)
- `b` (double)

**Returns:** `Fusion.AreaPerLength` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.op_Multiply%28Fusion.AreaPerLength%2CSystem.Double%29)

#### `static Fusion.AreaPerLength op_Multiply(double a, Fusion.AreaPerLength b)`

op_Multiply method of AreaPerLength.

**Parameters:**
- `a` (double)
- `b` (Fusion.AreaPerLength)

**Returns:** `Fusion.AreaPerLength` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.op_Multiply%28System.Double%2CFusion.AreaPerLength%29)

#### `static Fusion.AreaPerLength op_Subtraction(Fusion.AreaPerLength a, Fusion.AreaPerLength b)`

op_Subtraction method of AreaPerLength.

**Parameters:**
- `a` (Fusion.AreaPerLength)
- `b` (Fusion.AreaPerLength)

**Returns:** `Fusion.AreaPerLength` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.op_Subtraction%28Fusion.AreaPerLength%2CFusion.AreaPerLength%29)

#### `static Fusion.AreaPerLength op_UnaryNegation(Fusion.AreaPerLength a)`

op_UnaryNegation method of AreaPerLength.

**Parameters:**
- `a` (Fusion.AreaPerLength)

**Returns:** `Fusion.AreaPerLength` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.AreaPerLength.op_UnaryNegation%28Fusion.AreaPerLength%29)

### Properties
- `SquareMillimetersPerMeter` (double, get) — SquareMillimetersPerMeter property of AreaPerLength.

## AreaUnit (enum)

AreaUnit enum in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.AreaUnit)

### Values
- `SquareMillimeter` = `0`
- `SquareCentimeter` = `1`
- `SquareMeter` = `2`
- `SquareInch` = `3`
- `SquareFoot` = `4`
- `SquareYard` = `5`

## AuthorizationAvailableResponse (class)

AuthorizationAvailableResponse class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.AuthorizationAvailableResponse)

### Constructors
- `AuthorizationAvailableResponse(string action, Fusion.AuthorizationResult result)` — Constructs a new AuthorizationAvailableResponse.
- `AuthorizationAvailableResponse(string action, Fusion.AuthorizationResult result, string responseText)` — Constructs a new AuthorizationAvailableResponse.

### Properties
- `Action` (string, get) — Action property of AuthorizationAvailableResponse.
- `ResponseText` (string, get) — ResponseText property of AuthorizationAvailableResponse.
- `Result` (Fusion.AuthorizationResult, get) — Result property of AuthorizationAvailableResponse.

## AuthorizationResponse (class)

AuthorizationResponse class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.AuthorizationResponse)

### Constructors
- `AuthorizationResponse(Fusion.AuthorizationResult result, Fusion.AuthorizationToken authorizationToken)` — Constructs a new AuthorizationResponse.
- `AuthorizationResponse(Fusion.AuthorizationResult result, Fusion.AuthorizationToken authorizationToken, string responseText)` — Constructs a new AuthorizationResponse.

### Properties
- `ResponseText` (string, get) — ResponseText property of AuthorizationResponse.
- `Result` (Fusion.AuthorizationResult, get) — Result property of AuthorizationResponse.
- `Token` (Fusion.AuthorizationToken, get) — Token property of AuthorizationResponse.

## AuthorizationResult (enum)

AuthorizationResult enum in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.AuthorizationResult)

### Values
- `Success` = `0`
- `Timeout` = `1`
- `AccessDenied` = `2`
- `AccessExpired` = `3`

## AuthorizationToken (class)

AuthorizationToken class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.AuthorizationToken)

### Constructors
- `AuthorizationToken(string principal, System.DateTime? expires, System.DateTime? renewableUntil, string action)` — Constructs a new AuthorizationToken.

### Properties
- `Action` (string, get) — Action property of AuthorizationToken.
- `RenewableUntil` (System.DateTime?, get) — RenewableUntil property of AuthorizationToken.

## BindingSource (class)

BindingSource class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.BindingSource)

### Methods
#### `void RaisePropertyChanged(string propertyName = null)`

RaisePropertyChanged method of BindingSource.

**Parameters:**
- `propertyName` (string)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.BindingSource.RaisePropertyChanged%28System.String%29)

#### `void RaisePropertyChanged(string propertyName1, string propertyName2, string[] propertyNames)`

RaisePropertyChanged method of BindingSource.

**Parameters:**
- `propertyName1` (string)
- `propertyName2` (string)
- `propertyNames` (string[])

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.BindingSource.RaisePropertyChanged%28System.String%2CSystem.String%2CSystem.String%5B%5D%29)

### Events
#### `PropertyChanged` (`System.ComponentModel.PropertyChangedEventHandler`)

**Signature:** `event System.ComponentModel.PropertyChangedEventHandler PropertyChanged`

PropertyChanged event of BindingSource.

[Docs](https://www.nuget.org/packages/TeklaFusion#E%3AFusion.BindingSource.PropertyChanged)

## BoundingBox (struct)

BoundingBox struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.BoundingBox)

### Constructors
- `BoundingBox(Fusion.Float3 minimum, Fusion.Float3 maximum)` — Constructs a new BoundingBox.

### Methods
#### `bool Contains(Fusion.BoundingBox boundingBox)`

Contains method of BoundingBox.

**Parameters:**
- `boundingBox` (Fusion.BoundingBox)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.BoundingBox.Contains%28Fusion.BoundingBox%29)

#### `bool Contains(Fusion.Float3 point)`

Contains method of BoundingBox.

**Parameters:**
- `point` (Fusion.Float3)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.BoundingBox.Contains%28Fusion.Float3%29)

#### `bool Contains(Fusion.Triangle triangle)`

Contains method of BoundingBox.

**Parameters:**
- `triangle` (Fusion.Triangle)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.BoundingBox.Contains%28Fusion.Triangle%29)

#### `bool Contains(Fusion.TriangleMesh triangleMesh)`

Contains method of BoundingBox.

**Parameters:**
- `triangleMesh` (Fusion.TriangleMesh)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.BoundingBox.Contains%28Fusion.TriangleMesh%29)

#### `bool Equals(Fusion.BoundingBox other)`

Equals method of BoundingBox.

**Parameters:**
- `other` (Fusion.BoundingBox)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.BoundingBox.Equals%28Fusion.BoundingBox%29)

#### `bool Equals(object other)`

Equals method of BoundingBox.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.BoundingBox.Equals%28System.Object%29)

#### `static Fusion.BoundingBox FromPoints(System.Collections.Generic.IEnumerable<Fusion.Float3> points)`

FromPoints method of BoundingBox.

**Parameters:**
- `points` (System.Collections.Generic.IEnumerable<Fusion.Float3>)

**Returns:** `Fusion.BoundingBox` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.BoundingBox.FromPoints%28System.Collections.Generic.IEnumerable%7BFusion.Float3%7D%29)

#### `int GetHashCode()`

GetHashCode method of BoundingBox.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.BoundingBox.GetHashCode)

#### `bool Intersects(Fusion.BoundingBox boundingBox)`

Intersects method of BoundingBox.

**Parameters:**
- `boundingBox` (Fusion.BoundingBox)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.BoundingBox.Intersects%28Fusion.BoundingBox%29)

#### `static Fusion.BoundingBox Merge(Fusion.BoundingBox a, Fusion.BoundingBox b)`

Merge method of BoundingBox.

**Parameters:**
- `a` (Fusion.BoundingBox)
- `b` (Fusion.BoundingBox)

**Returns:** `Fusion.BoundingBox` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.BoundingBox.Merge%28Fusion.BoundingBox%2CFusion.BoundingBox%29)

#### `bool Near(Fusion.BoundingBox other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of BoundingBox.

**Parameters:**
- `other` (Fusion.BoundingBox)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.BoundingBox.Near%28Fusion.BoundingBox%2CSystem.Int32%29)

#### `static bool op_Equality(Fusion.BoundingBox a, Fusion.BoundingBox b)`

op_Equality method of BoundingBox.

**Parameters:**
- `a` (Fusion.BoundingBox)
- `b` (Fusion.BoundingBox)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.BoundingBox.op_Equality%28Fusion.BoundingBox%2CFusion.BoundingBox%29)

#### `static bool op_Inequality(Fusion.BoundingBox a, Fusion.BoundingBox b)`

op_Inequality method of BoundingBox.

**Parameters:**
- `a` (Fusion.BoundingBox)
- `b` (Fusion.BoundingBox)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.BoundingBox.op_Inequality%28Fusion.BoundingBox%2CFusion.BoundingBox%29)

### Properties
- `Center` (Fusion.Float3, get) — Center property of BoundingBox.
- `Empty` (Fusion.BoundingBox, get) — Empty property of BoundingBox.
- `IsEmpty` (bool, get) — IsEmpty property of BoundingBox.
- `Radius` (float, get) — Radius property of BoundingBox.

## BufferedTraceListener (class)

BufferedTraceListener class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.BufferedTraceListener)

### Constructors
- `BufferedTraceListener()` — Constructs a new BufferedTraceListener.

### Methods
#### `System.Collections.Generic.IEnumerable<string> Take(int count)`

Take method of BufferedTraceListener.

**Parameters:**
- `count` (int)

**Returns:** `System.Collections.Generic.IEnumerable<string>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.BufferedTraceListener.Take%28System.Int32%29)

#### `void Write(string message)`

Write method of BufferedTraceListener.

**Parameters:**
- `message` (string)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.BufferedTraceListener.Write%28System.String%29)

#### `void WriteLine(string message)`

WriteLine method of BufferedTraceListener.

**Parameters:**
- `message` (string)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.BufferedTraceListener.WriteLine%28System.String%29)

### Properties
- `BufferSize` (int, get/set) — BufferSize property of BufferedTraceListener.
- `IsThreadSafe` (bool, get) — IsThreadSafe property of BufferedTraceListener.

## Camera (struct)

Camera struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Camera)

### Constructors
- `Camera(Fusion.Float3 position, Fusion.Float3 offset, Fusion.Angle pitch, Fusion.Angle yaw, Fusion.Angle fieldOfView, float scaleFactor)` — Constructs a new Camera.

### Methods
#### `bool Equals(Fusion.Camera other)`

Equals method of Camera.

**Parameters:**
- `other` (Fusion.Camera)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Camera.Equals%28Fusion.Camera%29)

#### `bool Equals(object other)`

Equals method of Camera.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Camera.Equals%28System.Object%29)

#### `int GetHashCode()`

GetHashCode method of Camera.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Camera.GetHashCode)

#### `static Fusion.Camera Interpolate(Fusion.Camera a, Fusion.Camera b, float t)`

Interpolate method of Camera.

**Parameters:**
- `a` (Fusion.Camera)
- `b` (Fusion.Camera)
- `t` (float)

**Returns:** `Fusion.Camera` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Camera.Interpolate%28Fusion.Camera%2CFusion.Camera%2CSystem.Single%29)

#### `bool Near(Fusion.Camera other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of Camera.

**Parameters:**
- `other` (Fusion.Camera)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Camera.Near%28Fusion.Camera%2CSystem.Int32%29)

#### `Fusion.Float4x4 ToMatrix()`

ToMatrix method of Camera.

**Returns:** `Fusion.Float4x4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Camera.ToMatrix)

#### `static bool op_Equality(Fusion.Camera a, Fusion.Camera b)`

op_Equality method of Camera.

**Parameters:**
- `a` (Fusion.Camera)
- `b` (Fusion.Camera)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Camera.op_Equality%28Fusion.Camera%2CFusion.Camera%29)

#### `static bool op_Inequality(Fusion.Camera a, Fusion.Camera b)`

op_Inequality method of Camera.

**Parameters:**
- `a` (Fusion.Camera)
- `b` (Fusion.Camera)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Camera.op_Inequality%28Fusion.Camera%2CFusion.Camera%29)

### Properties
- `IsPerspective` (bool, get) — IsPerspective property of Camera.

## Command (class)

Command class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Command)

### Constructors
- `Command(System.Action execute)` — Constructs a new Command.
- `Command(System.Action execute, System.Func<bool> canExecute)` — Constructs a new Command.
- `Command(System.Action<object> execute)` — Constructs a new Command.
- `Command(System.Action<object> execute, System.Func<object, bool> canExecute)` — Constructs a new Command.

### Methods
#### `bool CanExecute(object parameter)`

CanExecute method of Command.

**Parameters:**
- `parameter` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Command.CanExecute%28System.Object%29)

#### `void ChangeCanExecute()`

ChangeCanExecute method of Command.

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Command.ChangeCanExecute)

#### `void Execute(object parameter)`

Execute method of Command.

**Parameters:**
- `parameter` (object)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Command.Execute%28System.Object%29)

### Events
#### `CanExecuteChanged` (`System.EventHandler`)

**Signature:** `event System.EventHandler CanExecuteChanged`

CanExecuteChanged event of Command.

[Docs](https://www.nuget.org/packages/TeklaFusion#E%3AFusion.Command.CanExecuteChanged)

## CommandHandler (class)

CommandHandler class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.CommandHandler)

### Constructors
- `CommandHandler(Fusion.ViewModel target)` — Constructs a new CommandHandler.

### Methods
#### `System.Windows.Input.ICommand GetCommand(string commandName)`

GetCommand method of CommandHandler.

**Parameters:**
- `commandName` (string)

**Returns:** `System.Windows.Input.ICommand` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.CommandHandler.GetCommand%28System.String%29)

#### `static Fusion.CommandHandler GetCommandHandler(System.Windows.DependencyObject element)`

GetCommandHandler method of CommandHandler.

**Parameters:**
- `element` (System.Windows.DependencyObject)

**Returns:** `Fusion.CommandHandler` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.CommandHandler.GetCommandHandler%28System.Windows.DependencyObject%29)

#### `static void SetCommandHandler(System.Windows.DependencyObject element, Fusion.CommandHandler commandHandler)`

SetCommandHandler method of CommandHandler.

**Parameters:**
- `element` (System.Windows.DependencyObject)
- `commandHandler` (Fusion.CommandHandler)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.CommandHandler.SetCommandHandler%28System.Windows.DependencyObject%2CFusion.CommandHandler%29)

### Properties
- `Target` (Fusion.ViewModel, get) — Target property of CommandHandler.

## CommandHandlerAttribute (class)

CommandHandlerAttribute class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.CommandHandlerAttribute)

### Constructors
- `CommandHandlerAttribute()` — Constructs a new CommandHandlerAttribute.

### Properties
- `MeasurePerformance` (bool, get/set) — MeasurePerformance property of CommandHandlerAttribute.

## Command`1 (class)

Command`1 class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Command`1)

### Constructors
- `Command`1(System.Action<T> execute)` — Constructs a new Command`1.
- `Command`1(System.Action<T> execute, System.Func<T, bool> canExecute)` — Constructs a new Command`1.

## DensityUnit (enum)

DensityUnit enum in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.DensityUnit)

### Values
- `KilogramsPerCubicMeter` = `0`
- `TonsPerCubicMeter` = `1`
- `NewtonsPerCubicMeter` = `2`
- `KilonewtonsPerCubicMeter` = `3`
- `PoundForcePerCubicFoot` = `4`

## DependencyAttribute (class)

DependencyAttribute class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.DependencyAttribute)

### Constructors
- `DependencyAttribute()` — Constructs a new DependencyAttribute.

## Disposable (class)

Disposable class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Disposable)

### Constructors
- `Disposable(System.Action disposed)` — Constructs a new Disposable.

### Methods
#### `void Dispose()`

Dispose method of Disposable.

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Disposable.Dispose)

#### `static Fusion.Disposable op_Addition(Fusion.Disposable a, System.IDisposable b)`

op_Addition method of Disposable.

**Parameters:**
- `a` (Fusion.Disposable)
- `b` (System.IDisposable)

**Returns:** `Fusion.Disposable` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Disposable.op_Addition%28Fusion.Disposable%2CSystem.IDisposable%29)

## Double2 (struct)

Double2 struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Double2)

### Constructors
- `Double2(double all)` — Constructs a new Double2.
- `Double2(double x, double y)` — Constructs a new Double2.

### Methods
#### `static Fusion.Double2 Abs(Fusion.Double2 value)`

Abs method of Double2.

**Parameters:**
- `value` (Fusion.Double2)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.Abs%28Fusion.Double2%29)

#### `static Fusion.Double2 Ceiling(Fusion.Double2 value)`

Ceiling method of Double2.

**Parameters:**
- `value` (Fusion.Double2)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.Ceiling%28Fusion.Double2%29)

#### `bool Equals(Fusion.Double2 other)`

Equals method of Double2.

**Parameters:**
- `other` (Fusion.Double2)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.Equals%28Fusion.Double2%29)

#### `bool Equals(object other)`

Equals method of Double2.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.Equals%28System.Object%29)

#### `static Fusion.Double2 Floor(Fusion.Double2 value)`

Floor method of Double2.

**Parameters:**
- `value` (Fusion.Double2)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.Floor%28Fusion.Double2%29)

#### `int GetHashCode()`

GetHashCode method of Double2.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.GetHashCode)

#### `static bool IsInfinity(Fusion.Double2 value)`

IsInfinity method of Double2.

**Parameters:**
- `value` (Fusion.Double2)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.IsInfinity%28Fusion.Double2%29)

#### `static bool IsNaN(Fusion.Double2 value)`

IsNaN method of Double2.

**Parameters:**
- `value` (Fusion.Double2)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.IsNaN%28Fusion.Double2%29)

#### `static bool IsNegativeInfinity(Fusion.Double2 value)`

IsNegativeInfinity method of Double2.

**Parameters:**
- `value` (Fusion.Double2)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.IsNegativeInfinity%28Fusion.Double2%29)

#### `static bool IsPositiveInfinity(Fusion.Double2 value)`

IsPositiveInfinity method of Double2.

**Parameters:**
- `value` (Fusion.Double2)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.IsPositiveInfinity%28Fusion.Double2%29)

#### `static Fusion.Double2 Max(Fusion.Double2 a, Fusion.Double2 b)`

Max method of Double2.

**Parameters:**
- `a` (Fusion.Double2)
- `b` (Fusion.Double2)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.Max%28Fusion.Double2%2CFusion.Double2%29)

#### `static Fusion.Double2 Min(Fusion.Double2 a, Fusion.Double2 b)`

Min method of Double2.

**Parameters:**
- `a` (Fusion.Double2)
- `b` (Fusion.Double2)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.Min%28Fusion.Double2%2CFusion.Double2%29)

#### `bool Near(Fusion.Double2 other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of Double2.

**Parameters:**
- `other` (Fusion.Double2)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.Near%28Fusion.Double2%2CSystem.Int32%29)

#### `static Fusion.Double2 Pow(Fusion.Double2 a, Fusion.Double2 b)`

Pow method of Double2.

**Parameters:**
- `a` (Fusion.Double2)
- `b` (Fusion.Double2)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.Pow%28Fusion.Double2%2CFusion.Double2%29)

#### `static Fusion.Double2 Pow(Fusion.Double2 a, double b)`

Pow method of Double2.

**Parameters:**
- `a` (Fusion.Double2)
- `b` (double)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.Pow%28Fusion.Double2%2CSystem.Double%29)

#### `static Fusion.Double2 Pow(double a, Fusion.Double2 b)`

Pow method of Double2.

**Parameters:**
- `a` (double)
- `b` (Fusion.Double2)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.Pow%28System.Double%2CFusion.Double2%29)

#### `static Fusion.Double2 Round(Fusion.Double2 value)`

Round method of Double2.

**Parameters:**
- `value` (Fusion.Double2)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.Round%28Fusion.Double2%29)

#### `static Fusion.Double2 Round(Fusion.Double2 value, System.MidpointRounding midpointRounding)`

Round method of Double2.

**Parameters:**
- `value` (Fusion.Double2)
- `midpointRounding` (System.MidpointRounding)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.Round%28Fusion.Double2%2CSystem.MidpointRounding%29)

#### `static Fusion.Double2 Round(Fusion.Double2 value, int digits)`

Round method of Double2.

**Parameters:**
- `value` (Fusion.Double2)
- `digits` (int)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.Round%28Fusion.Double2%2CSystem.Int32%29)

#### `static Fusion.Double2 Round(Fusion.Double2 value, int digits, System.MidpointRounding midpointRounding)`

Round method of Double2.

**Parameters:**
- `value` (Fusion.Double2)
- `digits` (int)
- `midpointRounding` (System.MidpointRounding)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.Round%28Fusion.Double2%2CSystem.Int32%2CSystem.MidpointRounding%29)

#### `static Fusion.Int2 Sign(Fusion.Double2 value)`

Sign method of Double2.

**Parameters:**
- `value` (Fusion.Double2)

**Returns:** `Fusion.Int2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.Sign%28Fusion.Double2%29)

#### `static Fusion.Double2 Sqrt(Fusion.Double2 value)`

Sqrt method of Double2.

**Parameters:**
- `value` (Fusion.Double2)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.Sqrt%28Fusion.Double2%29)

#### `string ToString()`

ToString method of Double2.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of Double2.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.Double2 Truncate(Fusion.Double2 value)`

Truncate method of Double2.

**Parameters:**
- `value` (Fusion.Double2)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.Truncate%28Fusion.Double2%29)

#### `static Fusion.Double2 op_Addition(Fusion.Double2 a, Fusion.Double2 b)`

op_Addition method of Double2.

**Parameters:**
- `a` (Fusion.Double2)
- `b` (Fusion.Double2)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.op_Addition%28Fusion.Double2%2CFusion.Double2%29)

#### `static Fusion.Double2 op_Addition(Fusion.Double2 a, double b)`

op_Addition method of Double2.

**Parameters:**
- `a` (Fusion.Double2)
- `b` (double)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.op_Addition%28Fusion.Double2%2CSystem.Double%29)

#### `static Fusion.Double2 op_Addition(double a, Fusion.Double2 b)`

op_Addition method of Double2.

**Parameters:**
- `a` (double)
- `b` (Fusion.Double2)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.op_Addition%28System.Double%2CFusion.Double2%29)

#### `static Fusion.Double2 op_Division(Fusion.Double2 a, Fusion.Double2 b)`

op_Division method of Double2.

**Parameters:**
- `a` (Fusion.Double2)
- `b` (Fusion.Double2)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.op_Division%28Fusion.Double2%2CFusion.Double2%29)

#### `static Fusion.Double2 op_Division(Fusion.Double2 a, double b)`

op_Division method of Double2.

**Parameters:**
- `a` (Fusion.Double2)
- `b` (double)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.op_Division%28Fusion.Double2%2CSystem.Double%29)

#### `static Fusion.Double2 op_Division(double a, Fusion.Double2 b)`

op_Division method of Double2.

**Parameters:**
- `a` (double)
- `b` (Fusion.Double2)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.op_Division%28System.Double%2CFusion.Double2%29)

#### `static bool op_Equality(Fusion.Double2 a, Fusion.Double2 b)`

op_Equality method of Double2.

**Parameters:**
- `a` (Fusion.Double2)
- `b` (Fusion.Double2)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.op_Equality%28Fusion.Double2%2CFusion.Double2%29)

#### `static Fusion.Double2 op_Implicit(Fusion.Float2 v)`

op_Implicit method of Double2.

**Parameters:**
- `v` (Fusion.Float2)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.op_Implicit%28Fusion.Float2%29~Fusion.Double2)

#### `static Fusion.Double2 op_Implicit(Fusion.Half2 v)`

op_Implicit method of Double2.

**Parameters:**
- `v` (Fusion.Half2)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.op_Implicit%28Fusion.Half2%29~Fusion.Double2)

#### `static Fusion.Double2 op_Implicit(Fusion.Int2 v)`

op_Implicit method of Double2.

**Parameters:**
- `v` (Fusion.Int2)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.op_Implicit%28Fusion.Int2%29~Fusion.Double2)

#### `static Fusion.Double2 op_Implicit(Fusion.Short2 v)`

op_Implicit method of Double2.

**Parameters:**
- `v` (Fusion.Short2)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.op_Implicit%28Fusion.Short2%29~Fusion.Double2)

#### `static Fusion.Double2 op_Implicit(Fusion.UShort2 v)`

op_Implicit method of Double2.

**Parameters:**
- `v` (Fusion.UShort2)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.op_Implicit%28Fusion.UShort2%29~Fusion.Double2)

#### `static bool op_Inequality(Fusion.Double2 a, Fusion.Double2 b)`

op_Inequality method of Double2.

**Parameters:**
- `a` (Fusion.Double2)
- `b` (Fusion.Double2)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.op_Inequality%28Fusion.Double2%2CFusion.Double2%29)

#### `static Fusion.Double2 op_Modulus(Fusion.Double2 a, Fusion.Double2 b)`

op_Modulus method of Double2.

**Parameters:**
- `a` (Fusion.Double2)
- `b` (Fusion.Double2)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.op_Modulus%28Fusion.Double2%2CFusion.Double2%29)

#### `static Fusion.Double2 op_Modulus(Fusion.Double2 a, double b)`

op_Modulus method of Double2.

**Parameters:**
- `a` (Fusion.Double2)
- `b` (double)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.op_Modulus%28Fusion.Double2%2CSystem.Double%29)

#### `static Fusion.Double2 op_Modulus(double a, Fusion.Double2 b)`

op_Modulus method of Double2.

**Parameters:**
- `a` (double)
- `b` (Fusion.Double2)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.op_Modulus%28System.Double%2CFusion.Double2%29)

#### `static Fusion.Double2 op_Multiply(Fusion.Double2 a, Fusion.Double2 b)`

op_Multiply method of Double2.

**Parameters:**
- `a` (Fusion.Double2)
- `b` (Fusion.Double2)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.op_Multiply%28Fusion.Double2%2CFusion.Double2%29)

#### `static Fusion.Double2 op_Multiply(Fusion.Double2 a, double b)`

op_Multiply method of Double2.

**Parameters:**
- `a` (Fusion.Double2)
- `b` (double)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.op_Multiply%28Fusion.Double2%2CSystem.Double%29)

#### `static Fusion.Double2 op_Multiply(double a, Fusion.Double2 b)`

op_Multiply method of Double2.

**Parameters:**
- `a` (double)
- `b` (Fusion.Double2)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.op_Multiply%28System.Double%2CFusion.Double2%29)

#### `static Fusion.Double2 op_Subtraction(Fusion.Double2 a, Fusion.Double2 b)`

op_Subtraction method of Double2.

**Parameters:**
- `a` (Fusion.Double2)
- `b` (Fusion.Double2)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.op_Subtraction%28Fusion.Double2%2CFusion.Double2%29)

#### `static Fusion.Double2 op_Subtraction(Fusion.Double2 a, double b)`

op_Subtraction method of Double2.

**Parameters:**
- `a` (Fusion.Double2)
- `b` (double)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.op_Subtraction%28Fusion.Double2%2CSystem.Double%29)

#### `static Fusion.Double2 op_Subtraction(double a, Fusion.Double2 b)`

op_Subtraction method of Double2.

**Parameters:**
- `a` (double)
- `b` (Fusion.Double2)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.op_Subtraction%28System.Double%2CFusion.Double2%29)

#### `static Fusion.Double2 op_UnaryNegation(Fusion.Double2 v)`

op_UnaryNegation method of Double2.

**Parameters:**
- `v` (Fusion.Double2)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.op_UnaryNegation%28Fusion.Double2%29)

#### `static Fusion.Double2 op_UnaryPlus(Fusion.Double2 v)`

op_UnaryPlus method of Double2.

**Parameters:**
- `v` (Fusion.Double2)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double2.op_UnaryPlus%28Fusion.Double2%29)

### Properties
- `Item` (double, get/set) — Item property of Double2.

## Double3 (struct)

Double3 struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Double3)

### Constructors
- `Double3(Fusion.Double2 xy, double z)` — Constructs a new Double3.
- `Double3(double all)` — Constructs a new Double3.
- `Double3(double x, Fusion.Double2 yz)` — Constructs a new Double3.
- `Double3(double x, double y, double z)` — Constructs a new Double3.

### Methods
#### `static Fusion.Double3 Abs(Fusion.Double3 value)`

Abs method of Double3.

**Parameters:**
- `value` (Fusion.Double3)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.Abs%28Fusion.Double3%29)

#### `static Fusion.Double3 Ceiling(Fusion.Double3 value)`

Ceiling method of Double3.

**Parameters:**
- `value` (Fusion.Double3)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.Ceiling%28Fusion.Double3%29)

#### `bool Equals(Fusion.Double3 other)`

Equals method of Double3.

**Parameters:**
- `other` (Fusion.Double3)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.Equals%28Fusion.Double3%29)

#### `bool Equals(object other)`

Equals method of Double3.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.Equals%28System.Object%29)

#### `static Fusion.Double3 Floor(Fusion.Double3 value)`

Floor method of Double3.

**Parameters:**
- `value` (Fusion.Double3)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.Floor%28Fusion.Double3%29)

#### `int GetHashCode()`

GetHashCode method of Double3.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.GetHashCode)

#### `static bool IsInfinity(Fusion.Double3 value)`

IsInfinity method of Double3.

**Parameters:**
- `value` (Fusion.Double3)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.IsInfinity%28Fusion.Double3%29)

#### `static bool IsNaN(Fusion.Double3 value)`

IsNaN method of Double3.

**Parameters:**
- `value` (Fusion.Double3)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.IsNaN%28Fusion.Double3%29)

#### `static bool IsNegativeInfinity(Fusion.Double3 value)`

IsNegativeInfinity method of Double3.

**Parameters:**
- `value` (Fusion.Double3)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.IsNegativeInfinity%28Fusion.Double3%29)

#### `static bool IsPositiveInfinity(Fusion.Double3 value)`

IsPositiveInfinity method of Double3.

**Parameters:**
- `value` (Fusion.Double3)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.IsPositiveInfinity%28Fusion.Double3%29)

#### `static Fusion.Double3 Max(Fusion.Double3 a, Fusion.Double3 b)`

Max method of Double3.

**Parameters:**
- `a` (Fusion.Double3)
- `b` (Fusion.Double3)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.Max%28Fusion.Double3%2CFusion.Double3%29)

#### `static Fusion.Double3 Min(Fusion.Double3 a, Fusion.Double3 b)`

Min method of Double3.

**Parameters:**
- `a` (Fusion.Double3)
- `b` (Fusion.Double3)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.Min%28Fusion.Double3%2CFusion.Double3%29)

#### `bool Near(Fusion.Double3 other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of Double3.

**Parameters:**
- `other` (Fusion.Double3)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.Near%28Fusion.Double3%2CSystem.Int32%29)

#### `static Fusion.Double3 Pow(Fusion.Double3 a, Fusion.Double3 b)`

Pow method of Double3.

**Parameters:**
- `a` (Fusion.Double3)
- `b` (Fusion.Double3)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.Pow%28Fusion.Double3%2CFusion.Double3%29)

#### `static Fusion.Double3 Pow(Fusion.Double3 a, double b)`

Pow method of Double3.

**Parameters:**
- `a` (Fusion.Double3)
- `b` (double)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.Pow%28Fusion.Double3%2CSystem.Double%29)

#### `static Fusion.Double3 Pow(double a, Fusion.Double3 b)`

Pow method of Double3.

**Parameters:**
- `a` (double)
- `b` (Fusion.Double3)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.Pow%28System.Double%2CFusion.Double3%29)

#### `static Fusion.Double3 Round(Fusion.Double3 value)`

Round method of Double3.

**Parameters:**
- `value` (Fusion.Double3)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.Round%28Fusion.Double3%29)

#### `static Fusion.Double3 Round(Fusion.Double3 value, System.MidpointRounding midpointRounding)`

Round method of Double3.

**Parameters:**
- `value` (Fusion.Double3)
- `midpointRounding` (System.MidpointRounding)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.Round%28Fusion.Double3%2CSystem.MidpointRounding%29)

#### `static Fusion.Double3 Round(Fusion.Double3 value, int digits)`

Round method of Double3.

**Parameters:**
- `value` (Fusion.Double3)
- `digits` (int)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.Round%28Fusion.Double3%2CSystem.Int32%29)

#### `static Fusion.Double3 Round(Fusion.Double3 value, int digits, System.MidpointRounding midpointRounding)`

Round method of Double3.

**Parameters:**
- `value` (Fusion.Double3)
- `digits` (int)
- `midpointRounding` (System.MidpointRounding)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.Round%28Fusion.Double3%2CSystem.Int32%2CSystem.MidpointRounding%29)

#### `static Fusion.Int3 Sign(Fusion.Double3 value)`

Sign method of Double3.

**Parameters:**
- `value` (Fusion.Double3)

**Returns:** `Fusion.Int3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.Sign%28Fusion.Double3%29)

#### `static Fusion.Double3 Sqrt(Fusion.Double3 value)`

Sqrt method of Double3.

**Parameters:**
- `value` (Fusion.Double3)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.Sqrt%28Fusion.Double3%29)

#### `string ToString()`

ToString method of Double3.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of Double3.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.Double3 Truncate(Fusion.Double3 value)`

Truncate method of Double3.

**Parameters:**
- `value` (Fusion.Double3)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.Truncate%28Fusion.Double3%29)

#### `static Fusion.Double3 op_Addition(Fusion.Double3 a, Fusion.Double3 b)`

op_Addition method of Double3.

**Parameters:**
- `a` (Fusion.Double3)
- `b` (Fusion.Double3)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.op_Addition%28Fusion.Double3%2CFusion.Double3%29)

#### `static Fusion.Double3 op_Addition(Fusion.Double3 a, double b)`

op_Addition method of Double3.

**Parameters:**
- `a` (Fusion.Double3)
- `b` (double)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.op_Addition%28Fusion.Double3%2CSystem.Double%29)

#### `static Fusion.Double3 op_Addition(double a, Fusion.Double3 b)`

op_Addition method of Double3.

**Parameters:**
- `a` (double)
- `b` (Fusion.Double3)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.op_Addition%28System.Double%2CFusion.Double3%29)

#### `static Fusion.Double3 op_Division(Fusion.Double3 a, Fusion.Double3 b)`

op_Division method of Double3.

**Parameters:**
- `a` (Fusion.Double3)
- `b` (Fusion.Double3)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.op_Division%28Fusion.Double3%2CFusion.Double3%29)

#### `static Fusion.Double3 op_Division(Fusion.Double3 a, double b)`

op_Division method of Double3.

**Parameters:**
- `a` (Fusion.Double3)
- `b` (double)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.op_Division%28Fusion.Double3%2CSystem.Double%29)

#### `static Fusion.Double3 op_Division(double a, Fusion.Double3 b)`

op_Division method of Double3.

**Parameters:**
- `a` (double)
- `b` (Fusion.Double3)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.op_Division%28System.Double%2CFusion.Double3%29)

#### `static bool op_Equality(Fusion.Double3 a, Fusion.Double3 b)`

op_Equality method of Double3.

**Parameters:**
- `a` (Fusion.Double3)
- `b` (Fusion.Double3)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.op_Equality%28Fusion.Double3%2CFusion.Double3%29)

#### `static Fusion.Double3 op_Implicit(Fusion.Float3 v)`

op_Implicit method of Double3.

**Parameters:**
- `v` (Fusion.Float3)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.op_Implicit%28Fusion.Float3%29~Fusion.Double3)

#### `static Fusion.Double3 op_Implicit(Fusion.Half3 v)`

op_Implicit method of Double3.

**Parameters:**
- `v` (Fusion.Half3)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.op_Implicit%28Fusion.Half3%29~Fusion.Double3)

#### `static Fusion.Double3 op_Implicit(Fusion.Int3 v)`

op_Implicit method of Double3.

**Parameters:**
- `v` (Fusion.Int3)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.op_Implicit%28Fusion.Int3%29~Fusion.Double3)

#### `static Fusion.Double3 op_Implicit(Fusion.Short3 v)`

op_Implicit method of Double3.

**Parameters:**
- `v` (Fusion.Short3)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.op_Implicit%28Fusion.Short3%29~Fusion.Double3)

#### `static Fusion.Double3 op_Implicit(Fusion.UShort3 v)`

op_Implicit method of Double3.

**Parameters:**
- `v` (Fusion.UShort3)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.op_Implicit%28Fusion.UShort3%29~Fusion.Double3)

#### `static bool op_Inequality(Fusion.Double3 a, Fusion.Double3 b)`

op_Inequality method of Double3.

**Parameters:**
- `a` (Fusion.Double3)
- `b` (Fusion.Double3)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.op_Inequality%28Fusion.Double3%2CFusion.Double3%29)

#### `static Fusion.Double3 op_Modulus(Fusion.Double3 a, Fusion.Double3 b)`

op_Modulus method of Double3.

**Parameters:**
- `a` (Fusion.Double3)
- `b` (Fusion.Double3)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.op_Modulus%28Fusion.Double3%2CFusion.Double3%29)

#### `static Fusion.Double3 op_Modulus(Fusion.Double3 a, double b)`

op_Modulus method of Double3.

**Parameters:**
- `a` (Fusion.Double3)
- `b` (double)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.op_Modulus%28Fusion.Double3%2CSystem.Double%29)

#### `static Fusion.Double3 op_Modulus(double a, Fusion.Double3 b)`

op_Modulus method of Double3.

**Parameters:**
- `a` (double)
- `b` (Fusion.Double3)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.op_Modulus%28System.Double%2CFusion.Double3%29)

#### `static Fusion.Double3 op_Multiply(Fusion.Double3 a, Fusion.Double3 b)`

op_Multiply method of Double3.

**Parameters:**
- `a` (Fusion.Double3)
- `b` (Fusion.Double3)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.op_Multiply%28Fusion.Double3%2CFusion.Double3%29)

#### `static Fusion.Double3 op_Multiply(Fusion.Double3 a, double b)`

op_Multiply method of Double3.

**Parameters:**
- `a` (Fusion.Double3)
- `b` (double)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.op_Multiply%28Fusion.Double3%2CSystem.Double%29)

#### `static Fusion.Double3 op_Multiply(double a, Fusion.Double3 b)`

op_Multiply method of Double3.

**Parameters:**
- `a` (double)
- `b` (Fusion.Double3)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.op_Multiply%28System.Double%2CFusion.Double3%29)

#### `static Fusion.Double3 op_Subtraction(Fusion.Double3 a, Fusion.Double3 b)`

op_Subtraction method of Double3.

**Parameters:**
- `a` (Fusion.Double3)
- `b` (Fusion.Double3)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.op_Subtraction%28Fusion.Double3%2CFusion.Double3%29)

#### `static Fusion.Double3 op_Subtraction(Fusion.Double3 a, double b)`

op_Subtraction method of Double3.

**Parameters:**
- `a` (Fusion.Double3)
- `b` (double)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.op_Subtraction%28Fusion.Double3%2CSystem.Double%29)

#### `static Fusion.Double3 op_Subtraction(double a, Fusion.Double3 b)`

op_Subtraction method of Double3.

**Parameters:**
- `a` (double)
- `b` (Fusion.Double3)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.op_Subtraction%28System.Double%2CFusion.Double3%29)

#### `static Fusion.Double3 op_UnaryNegation(Fusion.Double3 v)`

op_UnaryNegation method of Double3.

**Parameters:**
- `v` (Fusion.Double3)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.op_UnaryNegation%28Fusion.Double3%29)

#### `static Fusion.Double3 op_UnaryPlus(Fusion.Double3 v)`

op_UnaryPlus method of Double3.

**Parameters:**
- `v` (Fusion.Double3)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double3.op_UnaryPlus%28Fusion.Double3%29)

### Properties
- `Item` (double, get/set) — Item property of Double3.
- `XY` (Fusion.Double2, get) — XY property of Double3.

## Double4 (struct)

Double4 struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Double4)

### Constructors
- `Double4(Fusion.Double2 xy, Fusion.Double2 zw)` — Constructs a new Double4.
- `Double4(Fusion.Double2 xy, double z, double w)` — Constructs a new Double4.
- `Double4(Fusion.Double3 xyz, double w)` — Constructs a new Double4.
- `Double4(double all)` — Constructs a new Double4.
- `Double4(double x, Fusion.Double2 yz, double w)` — Constructs a new Double4.
- `Double4(double x, Fusion.Double3 yzw)` — Constructs a new Double4.
- `Double4(double x, double y, Fusion.Double2 zw)` — Constructs a new Double4.
- `Double4(double x, double y, double z, double w)` — Constructs a new Double4.

### Methods
#### `static Fusion.Double4 Abs(Fusion.Double4 value)`

Abs method of Double4.

**Parameters:**
- `value` (Fusion.Double4)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.Abs%28Fusion.Double4%29)

#### `static Fusion.Double4 Ceiling(Fusion.Double4 value)`

Ceiling method of Double4.

**Parameters:**
- `value` (Fusion.Double4)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.Ceiling%28Fusion.Double4%29)

#### `bool Equals(Fusion.Double4 other)`

Equals method of Double4.

**Parameters:**
- `other` (Fusion.Double4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.Equals%28Fusion.Double4%29)

#### `bool Equals(object other)`

Equals method of Double4.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.Equals%28System.Object%29)

#### `static Fusion.Double4 Floor(Fusion.Double4 value)`

Floor method of Double4.

**Parameters:**
- `value` (Fusion.Double4)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.Floor%28Fusion.Double4%29)

#### `int GetHashCode()`

GetHashCode method of Double4.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.GetHashCode)

#### `static bool IsInfinity(Fusion.Double4 value)`

IsInfinity method of Double4.

**Parameters:**
- `value` (Fusion.Double4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.IsInfinity%28Fusion.Double4%29)

#### `static bool IsNaN(Fusion.Double4 value)`

IsNaN method of Double4.

**Parameters:**
- `value` (Fusion.Double4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.IsNaN%28Fusion.Double4%29)

#### `static bool IsNegativeInfinity(Fusion.Double4 value)`

IsNegativeInfinity method of Double4.

**Parameters:**
- `value` (Fusion.Double4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.IsNegativeInfinity%28Fusion.Double4%29)

#### `static bool IsPositiveInfinity(Fusion.Double4 value)`

IsPositiveInfinity method of Double4.

**Parameters:**
- `value` (Fusion.Double4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.IsPositiveInfinity%28Fusion.Double4%29)

#### `static Fusion.Double4 Max(Fusion.Double4 a, Fusion.Double4 b)`

Max method of Double4.

**Parameters:**
- `a` (Fusion.Double4)
- `b` (Fusion.Double4)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.Max%28Fusion.Double4%2CFusion.Double4%29)

#### `static Fusion.Double4 Min(Fusion.Double4 a, Fusion.Double4 b)`

Min method of Double4.

**Parameters:**
- `a` (Fusion.Double4)
- `b` (Fusion.Double4)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.Min%28Fusion.Double4%2CFusion.Double4%29)

#### `bool Near(Fusion.Double4 other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of Double4.

**Parameters:**
- `other` (Fusion.Double4)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.Near%28Fusion.Double4%2CSystem.Int32%29)

#### `static Fusion.Double4 Pow(Fusion.Double4 a, Fusion.Double4 b)`

Pow method of Double4.

**Parameters:**
- `a` (Fusion.Double4)
- `b` (Fusion.Double4)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.Pow%28Fusion.Double4%2CFusion.Double4%29)

#### `static Fusion.Double4 Pow(Fusion.Double4 a, double b)`

Pow method of Double4.

**Parameters:**
- `a` (Fusion.Double4)
- `b` (double)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.Pow%28Fusion.Double4%2CSystem.Double%29)

#### `static Fusion.Double4 Pow(double a, Fusion.Double4 b)`

Pow method of Double4.

**Parameters:**
- `a` (double)
- `b` (Fusion.Double4)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.Pow%28System.Double%2CFusion.Double4%29)

#### `static Fusion.Double4 Round(Fusion.Double4 value)`

Round method of Double4.

**Parameters:**
- `value` (Fusion.Double4)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.Round%28Fusion.Double4%29)

#### `static Fusion.Double4 Round(Fusion.Double4 value, System.MidpointRounding midpointRounding)`

Round method of Double4.

**Parameters:**
- `value` (Fusion.Double4)
- `midpointRounding` (System.MidpointRounding)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.Round%28Fusion.Double4%2CSystem.MidpointRounding%29)

#### `static Fusion.Double4 Round(Fusion.Double4 value, int digits)`

Round method of Double4.

**Parameters:**
- `value` (Fusion.Double4)
- `digits` (int)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.Round%28Fusion.Double4%2CSystem.Int32%29)

#### `static Fusion.Double4 Round(Fusion.Double4 value, int digits, System.MidpointRounding midpointRounding)`

Round method of Double4.

**Parameters:**
- `value` (Fusion.Double4)
- `digits` (int)
- `midpointRounding` (System.MidpointRounding)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.Round%28Fusion.Double4%2CSystem.Int32%2CSystem.MidpointRounding%29)

#### `static Fusion.Int4 Sign(Fusion.Double4 value)`

Sign method of Double4.

**Parameters:**
- `value` (Fusion.Double4)

**Returns:** `Fusion.Int4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.Sign%28Fusion.Double4%29)

#### `static Fusion.Double4 Sqrt(Fusion.Double4 value)`

Sqrt method of Double4.

**Parameters:**
- `value` (Fusion.Double4)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.Sqrt%28Fusion.Double4%29)

#### `string ToString()`

ToString method of Double4.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of Double4.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.Double4 Truncate(Fusion.Double4 value)`

Truncate method of Double4.

**Parameters:**
- `value` (Fusion.Double4)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.Truncate%28Fusion.Double4%29)

#### `static Fusion.Double4 op_Addition(Fusion.Double4 a, Fusion.Double4 b)`

op_Addition method of Double4.

**Parameters:**
- `a` (Fusion.Double4)
- `b` (Fusion.Double4)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.op_Addition%28Fusion.Double4%2CFusion.Double4%29)

#### `static Fusion.Double4 op_Addition(Fusion.Double4 a, double b)`

op_Addition method of Double4.

**Parameters:**
- `a` (Fusion.Double4)
- `b` (double)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.op_Addition%28Fusion.Double4%2CSystem.Double%29)

#### `static Fusion.Double4 op_Addition(double a, Fusion.Double4 b)`

op_Addition method of Double4.

**Parameters:**
- `a` (double)
- `b` (Fusion.Double4)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.op_Addition%28System.Double%2CFusion.Double4%29)

#### `static Fusion.Double4 op_Division(Fusion.Double4 a, Fusion.Double4 b)`

op_Division method of Double4.

**Parameters:**
- `a` (Fusion.Double4)
- `b` (Fusion.Double4)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.op_Division%28Fusion.Double4%2CFusion.Double4%29)

#### `static Fusion.Double4 op_Division(Fusion.Double4 a, double b)`

op_Division method of Double4.

**Parameters:**
- `a` (Fusion.Double4)
- `b` (double)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.op_Division%28Fusion.Double4%2CSystem.Double%29)

#### `static Fusion.Double4 op_Division(double a, Fusion.Double4 b)`

op_Division method of Double4.

**Parameters:**
- `a` (double)
- `b` (Fusion.Double4)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.op_Division%28System.Double%2CFusion.Double4%29)

#### `static bool op_Equality(Fusion.Double4 a, Fusion.Double4 b)`

op_Equality method of Double4.

**Parameters:**
- `a` (Fusion.Double4)
- `b` (Fusion.Double4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.op_Equality%28Fusion.Double4%2CFusion.Double4%29)

#### `static Fusion.Double4 op_Implicit(Fusion.Float4 v)`

op_Implicit method of Double4.

**Parameters:**
- `v` (Fusion.Float4)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.op_Implicit%28Fusion.Float4%29~Fusion.Double4)

#### `static Fusion.Double4 op_Implicit(Fusion.Half4 v)`

op_Implicit method of Double4.

**Parameters:**
- `v` (Fusion.Half4)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.op_Implicit%28Fusion.Half4%29~Fusion.Double4)

#### `static Fusion.Double4 op_Implicit(Fusion.Int4 v)`

op_Implicit method of Double4.

**Parameters:**
- `v` (Fusion.Int4)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.op_Implicit%28Fusion.Int4%29~Fusion.Double4)

#### `static Fusion.Double4 op_Implicit(Fusion.RGBA v)`

op_Implicit method of Double4.

**Parameters:**
- `v` (Fusion.RGBA)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.op_Implicit%28Fusion.RGBA%29~Fusion.Double4)

#### `static Fusion.Double4 op_Implicit(Fusion.Short4 v)`

op_Implicit method of Double4.

**Parameters:**
- `v` (Fusion.Short4)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.op_Implicit%28Fusion.Short4%29~Fusion.Double4)

#### `static Fusion.Double4 op_Implicit(Fusion.UShort4 v)`

op_Implicit method of Double4.

**Parameters:**
- `v` (Fusion.UShort4)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.op_Implicit%28Fusion.UShort4%29~Fusion.Double4)

#### `static bool op_Inequality(Fusion.Double4 a, Fusion.Double4 b)`

op_Inequality method of Double4.

**Parameters:**
- `a` (Fusion.Double4)
- `b` (Fusion.Double4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.op_Inequality%28Fusion.Double4%2CFusion.Double4%29)

#### `static Fusion.Double4 op_Modulus(Fusion.Double4 a, Fusion.Double4 b)`

op_Modulus method of Double4.

**Parameters:**
- `a` (Fusion.Double4)
- `b` (Fusion.Double4)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.op_Modulus%28Fusion.Double4%2CFusion.Double4%29)

#### `static Fusion.Double4 op_Modulus(Fusion.Double4 a, double b)`

op_Modulus method of Double4.

**Parameters:**
- `a` (Fusion.Double4)
- `b` (double)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.op_Modulus%28Fusion.Double4%2CSystem.Double%29)

#### `static Fusion.Double4 op_Modulus(double a, Fusion.Double4 b)`

op_Modulus method of Double4.

**Parameters:**
- `a` (double)
- `b` (Fusion.Double4)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.op_Modulus%28System.Double%2CFusion.Double4%29)

#### `static Fusion.Double4 op_Multiply(Fusion.Double4 a, Fusion.Double4 b)`

op_Multiply method of Double4.

**Parameters:**
- `a` (Fusion.Double4)
- `b` (Fusion.Double4)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.op_Multiply%28Fusion.Double4%2CFusion.Double4%29)

#### `static Fusion.Double4 op_Multiply(Fusion.Double4 a, double b)`

op_Multiply method of Double4.

**Parameters:**
- `a` (Fusion.Double4)
- `b` (double)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.op_Multiply%28Fusion.Double4%2CSystem.Double%29)

#### `static Fusion.Double4 op_Multiply(double a, Fusion.Double4 b)`

op_Multiply method of Double4.

**Parameters:**
- `a` (double)
- `b` (Fusion.Double4)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.op_Multiply%28System.Double%2CFusion.Double4%29)

#### `static Fusion.Double4 op_Subtraction(Fusion.Double4 a, Fusion.Double4 b)`

op_Subtraction method of Double4.

**Parameters:**
- `a` (Fusion.Double4)
- `b` (Fusion.Double4)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.op_Subtraction%28Fusion.Double4%2CFusion.Double4%29)

#### `static Fusion.Double4 op_Subtraction(Fusion.Double4 a, double b)`

op_Subtraction method of Double4.

**Parameters:**
- `a` (Fusion.Double4)
- `b` (double)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.op_Subtraction%28Fusion.Double4%2CSystem.Double%29)

#### `static Fusion.Double4 op_Subtraction(double a, Fusion.Double4 b)`

op_Subtraction method of Double4.

**Parameters:**
- `a` (double)
- `b` (Fusion.Double4)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.op_Subtraction%28System.Double%2CFusion.Double4%29)

#### `static Fusion.Double4 op_UnaryNegation(Fusion.Double4 v)`

op_UnaryNegation method of Double4.

**Parameters:**
- `v` (Fusion.Double4)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.op_UnaryNegation%28Fusion.Double4%29)

#### `static Fusion.Double4 op_UnaryPlus(Fusion.Double4 v)`

op_UnaryPlus method of Double4.

**Parameters:**
- `v` (Fusion.Double4)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Double4.op_UnaryPlus%28Fusion.Double4%29)

### Properties
- `Item` (double, get/set) — Item property of Double4.
- `XY` (Fusion.Double2, get) — XY property of Double4.
- `XYZ` (Fusion.Double3, get) — XYZ property of Double4.

## DynamicViscosity (struct)

DynamicViscosity struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.DynamicViscosity)

### Constructors
- `DynamicViscosity(double pascalSeconds)` — Constructs a new DynamicViscosity.

### Methods
#### `static Fusion.DynamicViscosity Abs(Fusion.DynamicViscosity dynamicViscosity)`

Abs method of DynamicViscosity.

**Parameters:**
- `dynamicViscosity` (Fusion.DynamicViscosity)

**Returns:** `Fusion.DynamicViscosity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.Abs%28Fusion.DynamicViscosity%29)

#### `int CompareTo(Fusion.DynamicViscosity other)`

CompareTo method of DynamicViscosity.

**Parameters:**
- `other` (Fusion.DynamicViscosity)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.CompareTo%28Fusion.DynamicViscosity%29)

#### `int CompareTo(object other)`

CompareTo method of DynamicViscosity.

**Parameters:**
- `other` (object)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.CompareTo%28System.Object%29)

#### `bool Equals(Fusion.DynamicViscosity other)`

Equals method of DynamicViscosity.

**Parameters:**
- `other` (Fusion.DynamicViscosity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.Equals%28Fusion.DynamicViscosity%29)

#### `bool Equals(object other)`

Equals method of DynamicViscosity.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.Equals%28System.Object%29)

#### `static Fusion.DynamicViscosity From(double dynamicViscosity, Fusion.DynamicViscosityUnit dynamicViscosityUnit)`

From method of DynamicViscosity.

**Parameters:**
- `dynamicViscosity` (double)
- `dynamicViscosityUnit` (Fusion.DynamicViscosityUnit)

**Returns:** `Fusion.DynamicViscosity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.From%28System.Double%2CFusion.DynamicViscosityUnit%29)

#### `int GetHashCode()`

GetHashCode method of DynamicViscosity.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.GetHashCode)

#### `static bool IsInfinity(Fusion.DynamicViscosity dynamicViscosity)`

IsInfinity method of DynamicViscosity.

**Parameters:**
- `dynamicViscosity` (Fusion.DynamicViscosity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.IsInfinity%28Fusion.DynamicViscosity%29)

#### `static bool IsNaN(Fusion.DynamicViscosity dynamicViscosity)`

IsNaN method of DynamicViscosity.

**Parameters:**
- `dynamicViscosity` (Fusion.DynamicViscosity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.IsNaN%28Fusion.DynamicViscosity%29)

#### `static bool IsNegativeInfinity(Fusion.DynamicViscosity dynamicViscosity)`

IsNegativeInfinity method of DynamicViscosity.

**Parameters:**
- `dynamicViscosity` (Fusion.DynamicViscosity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.IsNegativeInfinity%28Fusion.DynamicViscosity%29)

#### `static bool IsPositiveInfinity(Fusion.DynamicViscosity dynamicViscosity)`

IsPositiveInfinity method of DynamicViscosity.

**Parameters:**
- `dynamicViscosity` (Fusion.DynamicViscosity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.IsPositiveInfinity%28Fusion.DynamicViscosity%29)

#### `static Fusion.DynamicViscosity Max(Fusion.DynamicViscosity a, Fusion.DynamicViscosity b)`

Max method of DynamicViscosity.

**Parameters:**
- `a` (Fusion.DynamicViscosity)
- `b` (Fusion.DynamicViscosity)

**Returns:** `Fusion.DynamicViscosity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.Max%28Fusion.DynamicViscosity%2CFusion.DynamicViscosity%29)

#### `static Fusion.DynamicViscosity Min(Fusion.DynamicViscosity a, Fusion.DynamicViscosity b)`

Min method of DynamicViscosity.

**Parameters:**
- `a` (Fusion.DynamicViscosity)
- `b` (Fusion.DynamicViscosity)

**Returns:** `Fusion.DynamicViscosity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.Min%28Fusion.DynamicViscosity%2CFusion.DynamicViscosity%29)

#### `bool Near(Fusion.DynamicViscosity other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of DynamicViscosity.

**Parameters:**
- `other` (Fusion.DynamicViscosity)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.Near%28Fusion.DynamicViscosity%2CSystem.Int32%29)

#### `static Fusion.DynamicViscosity Round(Fusion.DynamicViscosity dynamicViscosity, Fusion.DynamicViscosity precision, Fusion.RoundingMode roundingMode)`

Round method of DynamicViscosity.

**Parameters:**
- `dynamicViscosity` (Fusion.DynamicViscosity)
- `precision` (Fusion.DynamicViscosity)
- `roundingMode` (Fusion.RoundingMode)

**Returns:** `Fusion.DynamicViscosity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.Round%28Fusion.DynamicViscosity%2CFusion.DynamicViscosity%2CFusion.RoundingMode%29)

#### `static int Sign(Fusion.DynamicViscosity dynamicViscosity)`

Sign method of DynamicViscosity.

**Parameters:**
- `dynamicViscosity` (Fusion.DynamicViscosity)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.Sign%28Fusion.DynamicViscosity%29)

#### `double To(Fusion.DynamicViscosityUnit dynamicViscosityUnit)`

To method of DynamicViscosity.

**Parameters:**
- `dynamicViscosityUnit` (Fusion.DynamicViscosityUnit)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.To%28Fusion.DynamicViscosityUnit%29)

#### `string ToString()`

ToString method of DynamicViscosity.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of DynamicViscosity.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.DynamicViscosity op_Addition(Fusion.DynamicViscosity a, Fusion.DynamicViscosity b)`

op_Addition method of DynamicViscosity.

**Parameters:**
- `a` (Fusion.DynamicViscosity)
- `b` (Fusion.DynamicViscosity)

**Returns:** `Fusion.DynamicViscosity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.op_Addition%28Fusion.DynamicViscosity%2CFusion.DynamicViscosity%29)

#### `static Fusion.DynamicViscosity op_Division(Fusion.DynamicViscosity a, double b)`

op_Division method of DynamicViscosity.

**Parameters:**
- `a` (Fusion.DynamicViscosity)
- `b` (double)

**Returns:** `Fusion.DynamicViscosity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.op_Division%28Fusion.DynamicViscosity%2CSystem.Double%29)

#### `static double op_Division(Fusion.DynamicViscosity a, Fusion.DynamicViscosity b)`

op_Division method of DynamicViscosity.

**Parameters:**
- `a` (Fusion.DynamicViscosity)
- `b` (Fusion.DynamicViscosity)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.op_Division%28Fusion.DynamicViscosity%2CFusion.DynamicViscosity%29)

#### `static bool op_Equality(Fusion.DynamicViscosity a, Fusion.DynamicViscosity b)`

op_Equality method of DynamicViscosity.

**Parameters:**
- `a` (Fusion.DynamicViscosity)
- `b` (Fusion.DynamicViscosity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.op_Equality%28Fusion.DynamicViscosity%2CFusion.DynamicViscosity%29)

#### `static bool op_GreaterThan(Fusion.DynamicViscosity a, Fusion.DynamicViscosity b)`

op_GreaterThan method of DynamicViscosity.

**Parameters:**
- `a` (Fusion.DynamicViscosity)
- `b` (Fusion.DynamicViscosity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.op_GreaterThan%28Fusion.DynamicViscosity%2CFusion.DynamicViscosity%29)

#### `static bool op_GreaterThanOrEqual(Fusion.DynamicViscosity a, Fusion.DynamicViscosity b)`

op_GreaterThanOrEqual method of DynamicViscosity.

**Parameters:**
- `a` (Fusion.DynamicViscosity)
- `b` (Fusion.DynamicViscosity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.op_GreaterThanOrEqual%28Fusion.DynamicViscosity%2CFusion.DynamicViscosity%29)

#### `static bool op_Inequality(Fusion.DynamicViscosity a, Fusion.DynamicViscosity b)`

op_Inequality method of DynamicViscosity.

**Parameters:**
- `a` (Fusion.DynamicViscosity)
- `b` (Fusion.DynamicViscosity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.op_Inequality%28Fusion.DynamicViscosity%2CFusion.DynamicViscosity%29)

#### `static bool op_LessThan(Fusion.DynamicViscosity a, Fusion.DynamicViscosity b)`

op_LessThan method of DynamicViscosity.

**Parameters:**
- `a` (Fusion.DynamicViscosity)
- `b` (Fusion.DynamicViscosity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.op_LessThan%28Fusion.DynamicViscosity%2CFusion.DynamicViscosity%29)

#### `static bool op_LessThanOrEqual(Fusion.DynamicViscosity a, Fusion.DynamicViscosity b)`

op_LessThanOrEqual method of DynamicViscosity.

**Parameters:**
- `a` (Fusion.DynamicViscosity)
- `b` (Fusion.DynamicViscosity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.op_LessThanOrEqual%28Fusion.DynamicViscosity%2CFusion.DynamicViscosity%29)

#### `static Fusion.DynamicViscosity op_Modulus(Fusion.DynamicViscosity a, Fusion.DynamicViscosity b)`

op_Modulus method of DynamicViscosity.

**Parameters:**
- `a` (Fusion.DynamicViscosity)
- `b` (Fusion.DynamicViscosity)

**Returns:** `Fusion.DynamicViscosity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.op_Modulus%28Fusion.DynamicViscosity%2CFusion.DynamicViscosity%29)

#### `static Fusion.DynamicViscosity op_Multiply(Fusion.DynamicViscosity a, double b)`

op_Multiply method of DynamicViscosity.

**Parameters:**
- `a` (Fusion.DynamicViscosity)
- `b` (double)

**Returns:** `Fusion.DynamicViscosity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.op_Multiply%28Fusion.DynamicViscosity%2CSystem.Double%29)

#### `static Fusion.DynamicViscosity op_Multiply(double a, Fusion.DynamicViscosity b)`

op_Multiply method of DynamicViscosity.

**Parameters:**
- `a` (double)
- `b` (Fusion.DynamicViscosity)

**Returns:** `Fusion.DynamicViscosity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.op_Multiply%28System.Double%2CFusion.DynamicViscosity%29)

#### `static Fusion.DynamicViscosity op_Subtraction(Fusion.DynamicViscosity a, Fusion.DynamicViscosity b)`

op_Subtraction method of DynamicViscosity.

**Parameters:**
- `a` (Fusion.DynamicViscosity)
- `b` (Fusion.DynamicViscosity)

**Returns:** `Fusion.DynamicViscosity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.op_Subtraction%28Fusion.DynamicViscosity%2CFusion.DynamicViscosity%29)

#### `static Fusion.DynamicViscosity op_UnaryNegation(Fusion.DynamicViscosity a)`

op_UnaryNegation method of DynamicViscosity.

**Parameters:**
- `a` (Fusion.DynamicViscosity)

**Returns:** `Fusion.DynamicViscosity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.DynamicViscosity.op_UnaryNegation%28Fusion.DynamicViscosity%29)

### Properties
- `PascalSeconds` (double, get) — PascalSeconds property of DynamicViscosity.

## DynamicViscosityUnit (enum)

DynamicViscosityUnit enum in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.DynamicViscosityUnit)

### Values
- `PascalSeconds` = `0`
- `KilogramsPerSquareMeterSeconds` = `1`
- `PoundsPerSquareFootSeconds` = `2`

## Extensions (static-class)

Extensions static class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Extensions)

### Methods
#### `static void Activate(System.Windows.Controls.Primitives.Popup popup)`

Activate method of Extensions.

**Parameters:**
- `popup` (System.Windows.Controls.Primitives.Popup)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Activate%28System.Windows.Controls.Primitives.Popup%29)

#### `static void AddRange<T>(System.Collections.Generic.ICollection<T> target, System.Collections.Generic.IEnumerable<T> source)`

AddRange method of Extensions.

**Parameters:**
- `target` (System.Collections.Generic.ICollection<T>)
- `source` (System.Collections.Generic.IEnumerable<T>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.AddRange%60%601%28System.Collections.Generic.ICollection%7B%60%600%7D%2CSystem.Collections.Generic.IEnumerable%7B%60%600%7D%29)

#### `static Fusion.ReadOnlySet<T> AsReadOnly<T>(System.Collections.Generic.ISet<T> instance)`

AsReadOnly method of Extensions.

**Parameters:**
- `instance` (System.Collections.Generic.ISet<T>)

**Returns:** `Fusion.ReadOnlySet<T>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.AsReadOnly%60%601%28System.Collections.Generic.ISet%7B%60%600%7D%29)

#### `static System.Collections.ObjectModel.ReadOnlyCollection<T> AsReadOnly<T>(System.Collections.Generic.IList<T> collection)`

AsReadOnly method of Extensions.

**Parameters:**
- `collection` (System.Collections.Generic.IList<T>)

**Returns:** `System.Collections.ObjectModel.ReadOnlyCollection<T>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.AsReadOnly%60%601%28System.Collections.Generic.IList%7B%60%600%7D%29)

#### `static System.Collections.ObjectModel.ReadOnlyDictionary<TKey, TValue> AsReadOnly<TKey, TValue>(System.Collections.Generic.IDictionary<TKey, TValue> instance)`

AsReadOnly method of Extensions.

**Parameters:**
- `instance` (System.Collections.Generic.IDictionary<TKey, TValue>)

**Returns:** `System.Collections.ObjectModel.ReadOnlyDictionary<TKey, TValue>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.AsReadOnly%60%602%28System.Collections.Generic.IDictionary%7B%60%600%2C%60%601%7D%29)

#### `static System.Collections.ObjectModel.ReadOnlyObservableCollection<T> AsReadOnly<T>(System.Collections.ObjectModel.ObservableCollection<T> collection)`

AsReadOnly method of Extensions.

**Parameters:**
- `collection` (System.Collections.ObjectModel.ObservableCollection<T>)

**Returns:** `System.Collections.ObjectModel.ReadOnlyObservableCollection<T>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.AsReadOnly%60%601%28System.Collections.ObjectModel.ObservableCollection%7B%60%600%7D%29)

#### `static void CopyToSession(System.Net.CookieContainer cookieContainer, System.Func<System.Net.Cookie, bool> where = null)`

CopyToSession method of Extensions.

**Parameters:**
- `cookieContainer` (System.Net.CookieContainer)
- `where` (System.Func<System.Net.Cookie, bool>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.CopyToSession%28System.Net.CookieContainer%2CSystem.Func%7BSystem.Net.Cookie%2CSystem.Boolean%7D%29)

#### `static void DisableShortcuts(System.Windows.Window window)`

DisableShortcuts method of Extensions.

**Parameters:**
- `window` (System.Windows.Window)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.DisableShortcuts%28System.Windows.Window%29)

#### `static void Discard<T>(System.Collections.Generic.Stack<T> stack, int count)`

Discard method of Extensions.

**Parameters:**
- `stack` (System.Collections.Generic.Stack<T>)
- `count` (int)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Discard%60%601%28System.Collections.Generic.Stack%7B%60%600%7D%2CSystem.Int32%29)

#### `static TElement FindChild<TElement>(System.Windows.Application application)`

FindChild method of Extensions.

**Parameters:**
- `application` (System.Windows.Application)

**Returns:** `TElement` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.FindChild%60%601%28System.Windows.Application%29)

#### `static TElement FindChild<TElement>(System.Windows.Application application, System.Func<TElement, bool> where)`

FindChild method of Extensions.

**Parameters:**
- `application` (System.Windows.Application)
- `where` (System.Func<TElement, bool>)

**Returns:** `TElement` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.FindChild%60%601%28System.Windows.Application%2CSystem.Func%7B%60%600%2CSystem.Boolean%7D%29)

#### `static TElement FindChild<TElement>(System.Windows.DependencyObject element)`

FindChild method of Extensions.

**Parameters:**
- `element` (System.Windows.DependencyObject)

**Returns:** `TElement` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.FindChild%60%601%28System.Windows.DependencyObject%29)

#### `static TElement FindChild<TElement>(System.Windows.DependencyObject element, System.Func<TElement, bool> where)`

FindChild method of Extensions.

**Parameters:**
- `element` (System.Windows.DependencyObject)
- `where` (System.Func<TElement, bool>)

**Returns:** `TElement` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.FindChild%60%601%28System.Windows.DependencyObject%2CSystem.Func%7B%60%600%2CSystem.Boolean%7D%29)

#### `static System.Windows.DependencyObject FindLogicalParent(System.Windows.DependencyObject element)`

FindLogicalParent method of Extensions.

**Parameters:**
- `element` (System.Windows.DependencyObject)

**Returns:** `System.Windows.DependencyObject` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.FindLogicalParent%28System.Windows.DependencyObject%29)

#### `static TElement FindLogicalParent<TElement>(System.Windows.DependencyObject element)`

FindLogicalParent method of Extensions.

**Parameters:**
- `element` (System.Windows.DependencyObject)

**Returns:** `TElement` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.FindLogicalParent%60%601%28System.Windows.DependencyObject%29)

#### `static TElement FindLogicalParent<TElement>(System.Windows.DependencyObject element, System.Func<TElement, bool> where)`

FindLogicalParent method of Extensions.

**Parameters:**
- `element` (System.Windows.DependencyObject)
- `where` (System.Func<TElement, bool>)

**Returns:** `TElement` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.FindLogicalParent%60%601%28System.Windows.DependencyObject%2CSystem.Func%7B%60%600%2CSystem.Boolean%7D%29)

#### `static TElement FindName<TElement>(System.Windows.FrameworkElement element, string name)`

FindName method of Extensions.

**Parameters:**
- `element` (System.Windows.FrameworkElement)
- `name` (string)

**Returns:** `TElement` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.FindName%60%601%28System.Windows.FrameworkElement%2CSystem.String%29)

#### `static System.Windows.DependencyObject FindParent(System.Windows.DependencyObject element)`

FindParent method of Extensions.

**Parameters:**
- `element` (System.Windows.DependencyObject)

**Returns:** `System.Windows.DependencyObject` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.FindParent%28System.Windows.DependencyObject%29)

#### `static TElement FindParent<TElement>(System.Windows.DependencyObject element)`

FindParent method of Extensions.

**Parameters:**
- `element` (System.Windows.DependencyObject)

**Returns:** `TElement` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.FindParent%60%601%28System.Windows.DependencyObject%29)

#### `static TElement FindParent<TElement>(System.Windows.DependencyObject element, System.Func<TElement, bool> where)`

FindParent method of Extensions.

**Parameters:**
- `element` (System.Windows.DependencyObject)
- `where` (System.Func<TElement, bool>)

**Returns:** `TElement` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.FindParent%60%601%28System.Windows.DependencyObject%2CSystem.Func%7B%60%600%2CSystem.Boolean%7D%29)

#### `static System.Windows.Controls.Primitives.Popup FindParentPopup(System.Windows.DependencyObject element)`

FindParentPopup method of Extensions.

**Parameters:**
- `element` (System.Windows.DependencyObject)

**Returns:** `System.Windows.Controls.Primitives.Popup` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.FindParentPopup%28System.Windows.DependencyObject%29)

#### `static System.Windows.Window FindWindow(System.Windows.Application application, System.Func<System.Windows.Window, bool> where)`

FindWindow method of Extensions.

**Parameters:**
- `application` (System.Windows.Application)
- `where` (System.Func<System.Windows.Window, bool>)

**Returns:** `System.Windows.Window` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.FindWindow%28System.Windows.Application%2CSystem.Func%7BSystem.Windows.Window%2CSystem.Boolean%7D%29)

#### `static TWindow FindWindow<TWindow>(System.Windows.Application application, System.Func<TWindow, bool> where)`

FindWindow method of Extensions.

**Parameters:**
- `application` (System.Windows.Application)
- `where` (System.Func<TWindow, bool>)

**Returns:** `TWindow` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.FindWindow%60%601%28System.Windows.Application%2CSystem.Func%7B%60%600%2CSystem.Boolean%7D%29)

#### `static void ForEach<T1, T2, T3, T4, T5, T6, T7, T8>(System.Collections.Generic.IEnumerable<System.Tuple<T1, T2, T3, T4, T5, T6, T7, T8>> sequence, System.Action<T1, T2, T3, T4, T5, T6, T7, T8> action)`

ForEach method of Extensions.

**Parameters:**
- `sequence` (System.Collections.Generic.IEnumerable<System.Tuple<T1, T2, T3, T4, T5, T6, T7, T8>>)
- `action` (System.Action<T1, T2, T3, T4, T5, T6, T7, T8>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ForEach%60%608%28System.Collections.Generic.IEnumerable%7BSystem.Tuple%7B%60%600%2C%60%601%2C%60%602%2C%60%603%2C%60%604%2C%60%605%2C%60%606%2C%60%607%7D%7D%2CSystem.Action%7B%60%600%2C%60%601%2C%60%602%2C%60%603%2C%60%604%2C%60%605%2C%60%606%2C%60%607%7D%29)

#### `static void ForEach<T1, T2, T3, T4, T5, T6, T7>(System.Collections.Generic.IEnumerable<System.Tuple<T1, T2, T3, T4, T5, T6, T7>> sequence, System.Action<T1, T2, T3, T4, T5, T6, T7> action)`

ForEach method of Extensions.

**Parameters:**
- `sequence` (System.Collections.Generic.IEnumerable<System.Tuple<T1, T2, T3, T4, T5, T6, T7>>)
- `action` (System.Action<T1, T2, T3, T4, T5, T6, T7>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ForEach%60%607%28System.Collections.Generic.IEnumerable%7BSystem.Tuple%7B%60%600%2C%60%601%2C%60%602%2C%60%603%2C%60%604%2C%60%605%2C%60%606%7D%7D%2CSystem.Action%7B%60%600%2C%60%601%2C%60%602%2C%60%603%2C%60%604%2C%60%605%2C%60%606%7D%29)

#### `static void ForEach<T1, T2, T3, T4, T5, T6>(System.Collections.Generic.IEnumerable<System.Tuple<T1, T2, T3, T4, T5, T6>> sequence, System.Action<T1, T2, T3, T4, T5, T6> action)`

ForEach method of Extensions.

**Parameters:**
- `sequence` (System.Collections.Generic.IEnumerable<System.Tuple<T1, T2, T3, T4, T5, T6>>)
- `action` (System.Action<T1, T2, T3, T4, T5, T6>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ForEach%60%606%28System.Collections.Generic.IEnumerable%7BSystem.Tuple%7B%60%600%2C%60%601%2C%60%602%2C%60%603%2C%60%604%2C%60%605%7D%7D%2CSystem.Action%7B%60%600%2C%60%601%2C%60%602%2C%60%603%2C%60%604%2C%60%605%7D%29)

#### `static void ForEach<T1, T2, T3, T4, T5>(System.Collections.Generic.IEnumerable<System.Tuple<T1, T2, T3, T4, T5>> sequence, System.Action<T1, T2, T3, T4, T5> action)`

ForEach method of Extensions.

**Parameters:**
- `sequence` (System.Collections.Generic.IEnumerable<System.Tuple<T1, T2, T3, T4, T5>>)
- `action` (System.Action<T1, T2, T3, T4, T5>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ForEach%60%605%28System.Collections.Generic.IEnumerable%7BSystem.Tuple%7B%60%600%2C%60%601%2C%60%602%2C%60%603%2C%60%604%7D%7D%2CSystem.Action%7B%60%600%2C%60%601%2C%60%602%2C%60%603%2C%60%604%7D%29)

#### `static void ForEach<T1, T2, T3, T4>(System.Collections.Generic.IEnumerable<System.Tuple<T1, T2, T3, T4>> sequence, System.Action<T1, T2, T3, T4> action)`

ForEach method of Extensions.

**Parameters:**
- `sequence` (System.Collections.Generic.IEnumerable<System.Tuple<T1, T2, T3, T4>>)
- `action` (System.Action<T1, T2, T3, T4>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ForEach%60%604%28System.Collections.Generic.IEnumerable%7BSystem.Tuple%7B%60%600%2C%60%601%2C%60%602%2C%60%603%7D%7D%2CSystem.Action%7B%60%600%2C%60%601%2C%60%602%2C%60%603%7D%29)

#### `static void ForEach<T1, T2, T3>(System.Collections.Generic.IEnumerable<System.Tuple<T1, T2, T3>> sequence, System.Action<T1, T2, T3> action)`

ForEach method of Extensions.

**Parameters:**
- `sequence` (System.Collections.Generic.IEnumerable<System.Tuple<T1, T2, T3>>)
- `action` (System.Action<T1, T2, T3>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ForEach%60%603%28System.Collections.Generic.IEnumerable%7BSystem.Tuple%7B%60%600%2C%60%601%2C%60%602%7D%7D%2CSystem.Action%7B%60%600%2C%60%601%2C%60%602%7D%29)

#### `static void ForEach<T1, T2>(System.Collections.Generic.IEnumerable<System.Tuple<T1, T2>> sequence, System.Action<T1, T2> action)`

ForEach method of Extensions.

**Parameters:**
- `sequence` (System.Collections.Generic.IEnumerable<System.Tuple<T1, T2>>)
- `action` (System.Action<T1, T2>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ForEach%60%602%28System.Collections.Generic.IEnumerable%7BSystem.Tuple%7B%60%600%2C%60%601%7D%7D%2CSystem.Action%7B%60%600%2C%60%601%7D%29)

#### `static void ForEach<T1>(System.Collections.Generic.IEnumerable<System.Tuple<T1>> sequence, System.Action<T1> action)`

ForEach method of Extensions.

**Parameters:**
- `sequence` (System.Collections.Generic.IEnumerable<System.Tuple<T1>>)
- `action` (System.Action<T1>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ForEach%60%601%28System.Collections.Generic.IEnumerable%7BSystem.Tuple%7B%60%600%7D%7D%2CSystem.Action%7B%60%600%7D%29)

#### `static void ForEach<T>(System.Collections.Generic.IEnumerable<T> sequence, System.Action<T> action)`

ForEach method of Extensions.

**Parameters:**
- `sequence` (System.Collections.Generic.IEnumerable<T>)
- `action` (System.Action<T>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ForEach%60%601%28System.Collections.Generic.IEnumerable%7B%60%600%7D%2CSystem.Action%7B%60%600%7D%29)

#### `static void ForEach<TKey, TValue>(System.Collections.Generic.IEnumerable<System.Collections.Generic.KeyValuePair<TKey, TValue>> sequence, System.Action<TKey, TValue> action)`

ForEach method of Extensions.

**Parameters:**
- `sequence` (System.Collections.Generic.IEnumerable<System.Collections.Generic.KeyValuePair<TKey, TValue>>)
- `action` (System.Action<TKey, TValue>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ForEach%60%602%28System.Collections.Generic.IEnumerable%7BSystem.Collections.Generic.KeyValuePair%7B%60%600%2C%60%601%7D%7D%2CSystem.Action%7B%60%600%2C%60%601%7D%29)

#### `static string FormatInvariant(string format, object[] data)`

FormatInvariant method of Extensions.

**Parameters:**
- `format` (string)
- `data` (object[])

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.FormatInvariant%28System.String%2CSystem.Object%5B%5D%29)

#### `static System.Collections.Generic.IEnumerable<System.Net.Cookie> GetCookies(System.Net.CookieContainer cookieContainer)`

GetCookies method of Extensions.

**Parameters:**
- `cookieContainer` (System.Net.CookieContainer)

**Returns:** `System.Collections.Generic.IEnumerable<System.Net.Cookie>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.GetCookies%28System.Net.CookieContainer%29)

#### `static int GetDepth(System.Windows.Controls.TreeViewItem element)`

GetDepth method of Extensions.

**Parameters:**
- `element` (System.Windows.Controls.TreeViewItem)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.GetDepth%28System.Windows.Controls.TreeViewItem%29)

#### `static string GetMD5Hash(System.Drawing.Bitmap bitmap)`

GetMD5Hash method of Extensions.

**Parameters:**
- `bitmap` (System.Drawing.Bitmap)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.GetMD5Hash%28System.Drawing.Bitmap%29)

#### `static string GetMD5Hash(System.IO.Stream stream)`

GetMD5Hash method of Extensions.

**Parameters:**
- `stream` (System.IO.Stream)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.GetMD5Hash%28System.IO.Stream%29)

#### `static string GetMD5Hash(byte[] array)`

GetMD5Hash method of Extensions.

**Parameters:**
- `array` (byte[])

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.GetMD5Hash%28System.Byte%5B%5D%29)

#### `static string GetMD5Hash(string text)`

GetMD5Hash method of Extensions.

**Parameters:**
- `text` (string)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.GetMD5Hash%28System.String%29)

#### `static System.Windows.Point GetMousePosition(System.Windows.Media.Visual visual)`

GetMousePosition method of Extensions.

**Parameters:**
- `visual` (System.Windows.Media.Visual)

**Returns:** `System.Windows.Point` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.GetMousePosition%28System.Windows.Media.Visual%29)

#### `static System.Windows.Point GetMousePositionInScreenCoordinates(System.Windows.Media.Visual visual)`

GetMousePositionInScreenCoordinates method of Extensions.

**Parameters:**
- `visual` (System.Windows.Media.Visual)

**Returns:** `System.Windows.Point` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.GetMousePositionInScreenCoordinates%28System.Windows.Media.Visual%29)

#### `static TValue GetOrAdd<TKey, TValue>(System.Collections.Generic.IDictionary<TKey, TValue> dictionary, TKey key, System.Func<TKey, TValue> valueFactory)`

GetOrAdd method of Extensions.

**Parameters:**
- `dictionary` (System.Collections.Generic.IDictionary<TKey, TValue>)
- `key` (TKey)
- `valueFactory` (System.Func<TKey, TValue>)

**Returns:** `TValue` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.GetOrAdd%60%602%28System.Collections.Generic.IDictionary%7B%60%600%2C%60%601%7D%2C%60%600%2CSystem.Func%7B%60%600%2C%60%601%7D%29)

#### `static System.Windows.Size GetPixelScaleFactors(System.Windows.Media.Visual visual)`

GetPixelScaleFactors method of Extensions.

**Parameters:**
- `visual` (System.Windows.Media.Visual)

**Returns:** `System.Windows.Size` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.GetPixelScaleFactors%28System.Windows.Media.Visual%29)

#### `static System.Windows.ResourceDictionary GetResourceDictionary(System.Reflection.Assembly assembly, string resourcePath)`

GetResourceDictionary method of Extensions.

**Parameters:**
- `assembly` (System.Reflection.Assembly)
- `resourcePath` (string)

**Returns:** `System.Windows.ResourceDictionary` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.GetResourceDictionary%28System.Reflection.Assembly%2CSystem.String%29)

#### `static string GetSignature(System.Reflection.MethodInfo method, bool ignoreAccessModifier = false, bool ignoreStaticModifier = false)`

GetSignature method of Extensions.

**Parameters:**
- `method` (System.Reflection.MethodInfo)
- `ignoreAccessModifier` (bool)
- `ignoreStaticModifier` (bool)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.GetSignature%28System.Reflection.MethodInfo%2CSystem.Boolean%2CSystem.Boolean%29)

#### `static string GetSignature(System.Type delegateType, bool ignoreAccessModifier = false, bool ignoreStaticModifier = false)`

GetSignature method of Extensions.

**Parameters:**
- `delegateType` (System.Type)
- `ignoreAccessModifier` (bool)
- `ignoreStaticModifier` (bool)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.GetSignature%28System.Type%2CSystem.Boolean%2CSystem.Boolean%29)

#### `static TElement GetValue<TElement>(System.Collections.Generic.IList<TElement> list, int index, System.Func<int, TElement> valueFactory)`

GetValue method of Extensions.

**Parameters:**
- `list` (System.Collections.Generic.IList<TElement>)
- `index` (int)
- `valueFactory` (System.Func<int, TElement>)

**Returns:** `TElement` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.GetValue%60%601%28System.Collections.Generic.IList%7B%60%600%7D%2CSystem.Int32%2CSystem.Func%7BSystem.Int32%2C%60%600%7D%29)

#### `static TValue GetValue<TKey, TValue>(System.Collections.Generic.IDictionary<TKey, TValue> dictionary, TKey key, System.Func<TKey, TValue> valueFactory)`

GetValue method of Extensions.

**Parameters:**
- `dictionary` (System.Collections.Generic.IDictionary<TKey, TValue>)
- `key` (TKey)
- `valueFactory` (System.Func<TKey, TValue>)

**Returns:** `TValue` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.GetValue%60%602%28System.Collections.Generic.IDictionary%7B%60%600%2C%60%601%7D%2C%60%600%2CSystem.Func%7B%60%600%2C%60%601%7D%29)

#### `static TElement GetValueOrDefault<TElement>(System.Collections.Generic.IList<TElement> list, int index)`

GetValueOrDefault method of Extensions.

**Parameters:**
- `list` (System.Collections.Generic.IList<TElement>)
- `index` (int)

**Returns:** `TElement` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.GetValueOrDefault%60%601%28System.Collections.Generic.IList%7B%60%600%7D%2CSystem.Int32%29)

#### `static TElement GetValueOrDefault<TElement>(System.Collections.Generic.IList<TElement> list, int index, TElement defaultValue)`

GetValueOrDefault method of Extensions.

**Parameters:**
- `list` (System.Collections.Generic.IList<TElement>)
- `index` (int)
- `defaultValue` (TElement)

**Returns:** `TElement` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.GetValueOrDefault%60%601%28System.Collections.Generic.IList%7B%60%600%7D%2CSystem.Int32%2C%60%600%29)

#### `static TValue GetValueOrDefault<TKey, TValue>(System.Collections.Generic.IDictionary<TKey, TValue> dictionary, TKey key)`

GetValueOrDefault method of Extensions.

**Parameters:**
- `dictionary` (System.Collections.Generic.IDictionary<TKey, TValue>)
- `key` (TKey)

**Returns:** `TValue` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.GetValueOrDefault%60%602%28System.Collections.Generic.IDictionary%7B%60%600%2C%60%601%7D%2C%60%600%29)

#### `static TValue GetValueOrDefault<TKey, TValue>(System.Collections.Generic.IDictionary<TKey, TValue> dictionary, TKey key, TValue defaultValue)`

GetValueOrDefault method of Extensions.

**Parameters:**
- `dictionary` (System.Collections.Generic.IDictionary<TKey, TValue>)
- `key` (TKey)
- `defaultValue` (TValue)

**Returns:** `TValue` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.GetValueOrDefault%60%602%28System.Collections.Generic.IDictionary%7B%60%600%2C%60%601%7D%2C%60%600%2C%60%601%29)

#### `static void GlobalActivate(System.Windows.Window w)`

GlobalActivate method of Extensions.

**Parameters:**
- `w` (System.Windows.Window)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.GlobalActivate%28System.Windows.Window%29)

#### `static bool InsertAfter<T>(System.Collections.Generic.IList<T> list, T item, T after)`

InsertAfter method of Extensions.

**Parameters:**
- `list` (System.Collections.Generic.IList<T>)
- `item` (T)
- `after` (T)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.InsertAfter%60%601%28System.Collections.Generic.IList%7B%60%600%7D%2C%60%600%2C%60%600%29)

#### `static bool InsertBefore<T>(System.Collections.Generic.IList<T> list, T item, T before)`

InsertBefore method of Extensions.

**Parameters:**
- `list` (System.Collections.Generic.IList<T>)
- `item` (T)
- `before` (T)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.InsertBefore%60%601%28System.Collections.Generic.IList%7B%60%600%7D%2C%60%600%2C%60%600%29)

#### `static void Keep<T>(System.Collections.Generic.Stack<T> stack, int count)`

Keep method of Extensions.

**Parameters:**
- `stack` (System.Collections.Generic.Stack<T>)
- `count` (int)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Keep%60%601%28System.Collections.Generic.Stack%7B%60%600%7D%2CSystem.Int32%29)

#### `static bool Match(System.Windows.Input.KeyEventArgs keyEventArgs, System.Windows.Input.Key key, System.Windows.Input.ModifierKeys modifierKeys = 0)`

Match method of Extensions.

**Parameters:**
- `keyEventArgs` (System.Windows.Input.KeyEventArgs)
- `key` (System.Windows.Input.Key)
- `modifierKeys` (System.Windows.Input.ModifierKeys)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Match%28System.Windows.Input.KeyEventArgs%2CSystem.Windows.Input.Key%2CSystem.Windows.Input.ModifierKeys%29)

#### `static TSource MaxBy<TSource, TKey>(System.Collections.Generic.IEnumerable<TSource> source, System.Func<TSource, TKey> selector)`

MaxBy method of Extensions.

**Parameters:**
- `source` (System.Collections.Generic.IEnumerable<TSource>)
- `selector` (System.Func<TSource, TKey>)

**Returns:** `TSource` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.MaxBy%60%602%28System.Collections.Generic.IEnumerable%7B%60%600%7D%2CSystem.Func%7B%60%600%2C%60%601%7D%29)

#### `static TSource MaxBy<TSource, TKey>(System.Collections.Generic.IEnumerable<TSource> source, System.Func<TSource, TKey> selector, System.Collections.Generic.IComparer<TKey> comparer)`

MaxBy method of Extensions.

**Parameters:**
- `source` (System.Collections.Generic.IEnumerable<TSource>)
- `selector` (System.Func<TSource, TKey>)
- `comparer` (System.Collections.Generic.IComparer<TKey>)

**Returns:** `TSource` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.MaxBy%60%602%28System.Collections.Generic.IEnumerable%7B%60%600%7D%2CSystem.Func%7B%60%600%2C%60%601%7D%2CSystem.Collections.Generic.IComparer%7B%60%601%7D%29)

#### `static TSource MinBy<TSource, TKey>(System.Collections.Generic.IEnumerable<TSource> source, System.Func<TSource, TKey> selector)`

MinBy method of Extensions.

**Parameters:**
- `source` (System.Collections.Generic.IEnumerable<TSource>)
- `selector` (System.Func<TSource, TKey>)

**Returns:** `TSource` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.MinBy%60%602%28System.Collections.Generic.IEnumerable%7B%60%600%7D%2CSystem.Func%7B%60%600%2C%60%601%7D%29)

#### `static TSource MinBy<TSource, TKey>(System.Collections.Generic.IEnumerable<TSource> source, System.Func<TSource, TKey> selector, System.Collections.Generic.IComparer<TKey> comparer)`

MinBy method of Extensions.

**Parameters:**
- `source` (System.Collections.Generic.IEnumerable<TSource>)
- `selector` (System.Func<TSource, TKey>)
- `comparer` (System.Collections.Generic.IComparer<TKey>)

**Returns:** `TSource` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.MinBy%60%602%28System.Collections.Generic.IEnumerable%7B%60%600%7D%2CSystem.Func%7B%60%600%2C%60%601%7D%2CSystem.Collections.Generic.IComparer%7B%60%601%7D%29)

#### `static bool Near(double value1, double value2, int allowedErrorInUnitsInLastPlace = 10)`

Near method of Extensions.

**Parameters:**
- `value1` (double)
- `value2` (double)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Near%28System.Double%2CSystem.Double%2CSystem.Int32%29)

#### `static bool Near(float value1, float value2, int allowedErrorInUnitsInLastPlace = 10)`

Near method of Extensions.

**Parameters:**
- `value1` (float)
- `value2` (float)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Near%28System.Single%2CSystem.Single%2CSystem.Int32%29)

#### `static System.Windows.Input.CommandBinding OnCommand(System.Windows.UIElement element, System.Windows.Input.RoutedCommand command, System.Action<object> executed, System.Func<object, bool> canExecute = null)`

OnCommand method of Extensions.

**Parameters:**
- `element` (System.Windows.UIElement)
- `command` (System.Windows.Input.RoutedCommand)
- `executed` (System.Action<object>)
- `canExecute` (System.Func<object, bool>)

**Returns:** `System.Windows.Input.CommandBinding` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.OnCommand%28System.Windows.UIElement%2CSystem.Windows.Input.RoutedCommand%2CSystem.Action%7BSystem.Object%7D%2CSystem.Func%7BSystem.Object%2CSystem.Boolean%7D%29)

#### `static bool ParseAsBoolean(string text, bool defaultValue)`

ParseAsBoolean method of Extensions.

**Parameters:**
- `text` (string)
- `defaultValue` (bool)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ParseAsBoolean%28System.String%2CSystem.Boolean%29)

#### `static bool? ParseAsBoolean(string text)`

ParseAsBoolean method of Extensions.

**Parameters:**
- `text` (string)

**Returns:** `bool?` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ParseAsBoolean%28System.String%29)

#### `static double ParseAsDouble(string text, double defaultValue, System.Globalization.NumberStyles numberStyle = 167, System.IFormatProvider formatProvider = null)`

ParseAsDouble method of Extensions.

**Parameters:**
- `text` (string)
- `defaultValue` (double)
- `numberStyle` (System.Globalization.NumberStyles)
- `formatProvider` (System.IFormatProvider)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ParseAsDouble%28System.String%2CSystem.Double%2CSystem.Globalization.NumberStyles%2CSystem.IFormatProvider%29)

#### `static double? ParseAsDouble(string text, System.Globalization.NumberStyles numberStyle = 167, System.IFormatProvider formatProvider = null)`

ParseAsDouble method of Extensions.

**Parameters:**
- `text` (string)
- `numberStyle` (System.Globalization.NumberStyles)
- `formatProvider` (System.IFormatProvider)

**Returns:** `double?` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ParseAsDouble%28System.String%2CSystem.Globalization.NumberStyles%2CSystem.IFormatProvider%29)

#### `static TEnum ParseAsEnum<TEnum>(string text, TEnum defaultValue, bool ignoreCase = false)`

ParseAsEnum method of Extensions.

**Parameters:**
- `text` (string)
- `defaultValue` (TEnum)
- `ignoreCase` (bool)

**Returns:** `TEnum` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ParseAsEnum%60%601%28System.String%2C%60%600%2CSystem.Boolean%29)

#### `static TEnum? ParseAsEnum<TEnum>(string text, bool ignoreCase = false)`

ParseAsEnum method of Extensions.

**Parameters:**
- `text` (string)
- `ignoreCase` (bool)

**Returns:** `TEnum?` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ParseAsEnum%60%601%28System.String%2CSystem.Boolean%29)

#### `static int ParseAsInt32(string text, int defaultValue, System.Globalization.NumberStyles numberStyle = 7, System.IFormatProvider formatProvider = null)`

ParseAsInt32 method of Extensions.

**Parameters:**
- `text` (string)
- `defaultValue` (int)
- `numberStyle` (System.Globalization.NumberStyles)
- `formatProvider` (System.IFormatProvider)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ParseAsInt32%28System.String%2CSystem.Int32%2CSystem.Globalization.NumberStyles%2CSystem.IFormatProvider%29)

#### `static int? ParseAsInt32(string text, System.Globalization.NumberStyles numberStyle = 7, System.IFormatProvider formatProvider = null)`

ParseAsInt32 method of Extensions.

**Parameters:**
- `text` (string)
- `numberStyle` (System.Globalization.NumberStyles)
- `formatProvider` (System.IFormatProvider)

**Returns:** `int?` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ParseAsInt32%28System.String%2CSystem.Globalization.NumberStyles%2CSystem.IFormatProvider%29)

#### `static System.Exception PreserveStackTrace(System.Exception exception)`

PreserveStackTrace method of Extensions.

**Parameters:**
- `exception` (System.Exception)

**Returns:** `System.Exception` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.PreserveStackTrace%28System.Exception%29)

#### `static void Raise(System.Collections.Specialized.NotifyCollectionChangedEventHandler eventHandler, object sender)`

Raise method of Extensions.

**Parameters:**
- `eventHandler` (System.Collections.Specialized.NotifyCollectionChangedEventHandler)
- `sender` (object)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Raise%28System.Collections.Specialized.NotifyCollectionChangedEventHandler%2CSystem.Object%29)

#### `static void Raise(System.ComponentModel.PropertyChangedEventHandler eventHandler, object sender, string propertyName)`

Raise method of Extensions.

**Parameters:**
- `eventHandler` (System.ComponentModel.PropertyChangedEventHandler)
- `sender` (object)
- `propertyName` (string)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Raise%28System.ComponentModel.PropertyChangedEventHandler%2CSystem.Object%2CSystem.String%29)

#### `static void Raise(System.EventHandler eventHandler, object sender)`

Raise method of Extensions.

**Parameters:**
- `eventHandler` (System.EventHandler)
- `sender` (object)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Raise%28System.EventHandler%2CSystem.Object%29)

#### `static void Raise(System.EventHandler eventHandler, object sender, System.EventArgs eventArgs)`

Raise method of Extensions.

**Parameters:**
- `eventHandler` (System.EventHandler)
- `sender` (object)
- `eventArgs` (System.EventArgs)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Raise%28System.EventHandler%2CSystem.Object%2CSystem.EventArgs%29)

#### `static void Raise<TEventArgs>(System.EventHandler<TEventArgs> eventHandler, object sender, System.Func<TEventArgs> eventArgsFactory)`

Raise method of Extensions.

**Parameters:**
- `eventHandler` (System.EventHandler<TEventArgs>)
- `sender` (object)
- `eventArgsFactory` (System.Func<TEventArgs>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Raise%60%601%28System.EventHandler%7B%60%600%7D%2CSystem.Object%2CSystem.Func%7B%60%600%7D%29)

#### `static void Raise<TEventArgs>(System.EventHandler<TEventArgs> eventHandler, object sender, TEventArgs eventArgs)`

Raise method of Extensions.

**Parameters:**
- `eventHandler` (System.EventHandler<TEventArgs>)
- `sender` (object)
- `eventArgs` (TEventArgs)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Raise%60%601%28System.EventHandler%7B%60%600%7D%2CSystem.Object%2C%60%600%29)

#### `static Fusion.Angle ReadAngle(System.IO.BinaryReader reader)`

ReadAngle method of Extensions.

**Parameters:**
- `reader` (System.IO.BinaryReader)

**Returns:** `Fusion.Angle` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ReadAngle%28System.IO.BinaryReader%29)

#### `static Fusion.Area ReadArea(System.IO.BinaryReader reader)`

ReadArea method of Extensions.

**Parameters:**
- `reader` (System.IO.BinaryReader)

**Returns:** `Fusion.Area` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ReadArea%28System.IO.BinaryReader%29)

#### `static Fusion.Double2 ReadDouble2(System.IO.BinaryReader reader)`

ReadDouble2 method of Extensions.

**Parameters:**
- `reader` (System.IO.BinaryReader)

**Returns:** `Fusion.Double2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ReadDouble2%28System.IO.BinaryReader%29)

#### `static Fusion.Double3 ReadDouble3(System.IO.BinaryReader reader)`

ReadDouble3 method of Extensions.

**Parameters:**
- `reader` (System.IO.BinaryReader)

**Returns:** `Fusion.Double3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ReadDouble3%28System.IO.BinaryReader%29)

#### `static Fusion.Double4 ReadDouble4(System.IO.BinaryReader reader)`

ReadDouble4 method of Extensions.

**Parameters:**
- `reader` (System.IO.BinaryReader)

**Returns:** `Fusion.Double4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ReadDouble4%28System.IO.BinaryReader%29)

#### `static Fusion.Float2 ReadFloat2(System.IO.BinaryReader reader)`

ReadFloat2 method of Extensions.

**Parameters:**
- `reader` (System.IO.BinaryReader)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ReadFloat2%28System.IO.BinaryReader%29)

#### `static Fusion.Float3 ReadFloat3(System.IO.BinaryReader reader)`

ReadFloat3 method of Extensions.

**Parameters:**
- `reader` (System.IO.BinaryReader)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ReadFloat3%28System.IO.BinaryReader%29)

#### `static Fusion.Float4 ReadFloat4(System.IO.BinaryReader reader)`

ReadFloat4 method of Extensions.

**Parameters:**
- `reader` (System.IO.BinaryReader)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ReadFloat4%28System.IO.BinaryReader%29)

#### `static Fusion.Float4x4 ReadFloat4x4(System.IO.BinaryReader reader)`

ReadFloat4x4 method of Extensions.

**Parameters:**
- `reader` (System.IO.BinaryReader)

**Returns:** `Fusion.Float4x4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ReadFloat4x4%28System.IO.BinaryReader%29)

#### `static Fusion.FourCC ReadFourCC(System.IO.BinaryReader reader)`

ReadFourCC method of Extensions.

**Parameters:**
- `reader` (System.IO.BinaryReader)

**Returns:** `Fusion.FourCC` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ReadFourCC%28System.IO.BinaryReader%29)

#### `static System.Guid ReadGuid(System.IO.BinaryReader reader)`

ReadGuid method of Extensions.

**Parameters:**
- `reader` (System.IO.BinaryReader)

**Returns:** `System.Guid` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ReadGuid%28System.IO.BinaryReader%29)

#### `static Fusion.Int2 ReadInt2(System.IO.BinaryReader reader)`

ReadInt2 method of Extensions.

**Parameters:**
- `reader` (System.IO.BinaryReader)

**Returns:** `Fusion.Int2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ReadInt2%28System.IO.BinaryReader%29)

#### `static Fusion.Int3 ReadInt3(System.IO.BinaryReader reader)`

ReadInt3 method of Extensions.

**Parameters:**
- `reader` (System.IO.BinaryReader)

**Returns:** `Fusion.Int3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ReadInt3%28System.IO.BinaryReader%29)

#### `static Fusion.Int4 ReadInt4(System.IO.BinaryReader reader)`

ReadInt4 method of Extensions.

**Parameters:**
- `reader` (System.IO.BinaryReader)

**Returns:** `Fusion.Int4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ReadInt4%28System.IO.BinaryReader%29)

#### `static Fusion.Length ReadLength(System.IO.BinaryReader reader)`

ReadLength method of Extensions.

**Parameters:**
- `reader` (System.IO.BinaryReader)

**Returns:** `Fusion.Length` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ReadLength%28System.IO.BinaryReader%29)

#### `static Fusion.Mass ReadMass(System.IO.BinaryReader reader)`

ReadMass method of Extensions.

**Parameters:**
- `reader` (System.IO.BinaryReader)

**Returns:** `Fusion.Mass` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ReadMass%28System.IO.BinaryReader%29)

#### `static int ReadPackedInt32(System.IO.BinaryReader reader)`

ReadPackedInt32 method of Extensions.

**Parameters:**
- `reader` (System.IO.BinaryReader)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ReadPackedInt32%28System.IO.BinaryReader%29)

#### `static Fusion.RGBA ReadRGBA(System.IO.BinaryReader reader)`

ReadRGBA method of Extensions.

**Parameters:**
- `reader` (System.IO.BinaryReader)

**Returns:** `Fusion.RGBA` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ReadRGBA%28System.IO.BinaryReader%29)

#### `static Fusion.Short2 ReadShort2(System.IO.BinaryReader reader)`

ReadShort2 method of Extensions.

**Parameters:**
- `reader` (System.IO.BinaryReader)

**Returns:** `Fusion.Short2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ReadShort2%28System.IO.BinaryReader%29)

#### `static Fusion.Short3 ReadShort3(System.IO.BinaryReader reader)`

ReadShort3 method of Extensions.

**Parameters:**
- `reader` (System.IO.BinaryReader)

**Returns:** `Fusion.Short3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ReadShort3%28System.IO.BinaryReader%29)

#### `static Fusion.Short4 ReadShort4(System.IO.BinaryReader reader)`

ReadShort4 method of Extensions.

**Parameters:**
- `reader` (System.IO.BinaryReader)

**Returns:** `Fusion.Short4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ReadShort4%28System.IO.BinaryReader%29)

#### `static Fusion.UShort2 ReadUShort2(System.IO.BinaryReader reader)`

ReadUShort2 method of Extensions.

**Parameters:**
- `reader` (System.IO.BinaryReader)

**Returns:** `Fusion.UShort2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ReadUShort2%28System.IO.BinaryReader%29)

#### `static Fusion.UShort3 ReadUShort3(System.IO.BinaryReader reader)`

ReadUShort3 method of Extensions.

**Parameters:**
- `reader` (System.IO.BinaryReader)

**Returns:** `Fusion.UShort3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ReadUShort3%28System.IO.BinaryReader%29)

#### `static Fusion.UShort4 ReadUShort4(System.IO.BinaryReader reader)`

ReadUShort4 method of Extensions.

**Parameters:**
- `reader` (System.IO.BinaryReader)

**Returns:** `Fusion.UShort4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ReadUShort4%28System.IO.BinaryReader%29)

#### `static Fusion.Volume ReadVolume(System.IO.BinaryReader reader)`

ReadVolume method of Extensions.

**Parameters:**
- `reader` (System.IO.BinaryReader)

**Returns:** `Fusion.Volume` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ReadVolume%28System.IO.BinaryReader%29)

#### `static bool RecursiveUpdate(System.Windows.DependencyObject dependencyObject, System.Action recursiveUpdate)`

RecursiveUpdate method of Extensions.

**Parameters:**
- `dependencyObject` (System.Windows.DependencyObject)
- `recursiveUpdate` (System.Action)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.RecursiveUpdate%28System.Windows.DependencyObject%2CSystem.Action%29)

#### `static T[] RemoveAll<T>(System.Collections.Generic.ICollection<T> collection)`

RemoveAll method of Extensions.

**Parameters:**
- `collection` (System.Collections.Generic.ICollection<T>)

**Returns:** `T[]` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.RemoveAll%60%601%28System.Collections.Generic.ICollection%7B%60%600%7D%29)

#### `static void SetFilter(Microsoft.Win32.FileDialog dialog, Fusion.FileType[] fileTypes)`

SetFilter method of Extensions.

**Parameters:**
- `dialog` (Microsoft.Win32.FileDialog)
- `fileTypes` (Fusion.FileType[])

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.SetFilter%28Microsoft.Win32.FileDialog%2CFusion.FileType%5B%5D%29)

#### `static void SimulateClick(System.Windows.Controls.Primitives.ButtonBase button)`

SimulateClick method of Extensions.

**Parameters:**
- `button` (System.Windows.Controls.Primitives.ButtonBase)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.SimulateClick%28System.Windows.Controls.Primitives.ButtonBase%29)

#### `static void SizeToContentWorkaround(System.Windows.Window window)`

SizeToContentWorkaround method of Extensions.

**Parameters:**
- `window` (System.Windows.Window)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.SizeToContentWorkaround%28System.Windows.Window%29)

#### `static void SuppressScriptErrors(System.Windows.Controls.WebBrowser webBrowser, bool suppress)`

SuppressScriptErrors method of Extensions.

**Parameters:**
- `webBrowser` (System.Windows.Controls.WebBrowser)
- `suppress` (bool)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.SuppressScriptErrors%28System.Windows.Controls.WebBrowser%2CSystem.Boolean%29)

#### `static void TemplatedPart<TPart>(System.Windows.Controls.Control control, string partName, System.Action<TPart> withPart)`

TemplatedPart method of Extensions.

**Parameters:**
- `control` (System.Windows.Controls.Control)
- `partName` (string)
- `withPart` (System.Action<TPart>)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.TemplatedPart%60%601%28System.Windows.Controls.Control%2CSystem.String%2CSystem.Action%7B%60%600%7D%29)

#### `static System.Collections.BitArray ToBitArray(byte[] array)`

ToBitArray method of Extensions.

**Parameters:**
- `array` (byte[])

**Returns:** `System.Collections.BitArray` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ToBitArray%28System.Byte%5B%5D%29)

#### `static System.Drawing.Bitmap ToBitmap(byte[] array)`

ToBitmap method of Extensions.

**Parameters:**
- `array` (byte[])

**Returns:** `System.Drawing.Bitmap` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ToBitmap%28System.Byte%5B%5D%29)

#### `static byte[] ToByteArray(System.Collections.BitArray bitArray)`

ToByteArray method of Extensions.

**Parameters:**
- `bitArray` (System.Collections.BitArray)

**Returns:** `byte[]` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ToByteArray%28System.Collections.BitArray%29)

#### `static byte[] ToByteArray(System.Drawing.Bitmap bitmap)`

ToByteArray method of Extensions.

**Parameters:**
- `bitmap` (System.Drawing.Bitmap)

**Returns:** `byte[]` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ToByteArray%28System.Drawing.Bitmap%29)

#### `static System.Collections.Generic.HashSet<T> ToHashSet<T>(System.Collections.Generic.IEnumerable<T> source)`

ToHashSet method of Extensions.

**Parameters:**
- `source` (System.Collections.Generic.IEnumerable<T>)

**Returns:** `System.Collections.Generic.HashSet<T>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ToHashSet%60%601%28System.Collections.Generic.IEnumerable%7B%60%600%7D%29)

#### `static System.Collections.Generic.HashSet<T> ToHashSet<T>(System.Collections.Generic.IEnumerable<T> source, System.Collections.Generic.IEqualityComparer<T> comparer)`

ToHashSet method of Extensions.

**Parameters:**
- `source` (System.Collections.Generic.IEnumerable<T>)
- `comparer` (System.Collections.Generic.IEqualityComparer<T>)

**Returns:** `System.Collections.Generic.HashSet<T>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.ToHashSet%60%601%28System.Collections.Generic.IEnumerable%7B%60%600%7D%2CSystem.Collections.Generic.IEqualityComparer%7B%60%600%7D%29)

#### `static string TrimEnd(string text, int characters)`

TrimEnd method of Extensions.

**Parameters:**
- `text` (string)
- `characters` (int)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.TrimEnd%28System.String%2CSystem.Int32%29)

#### `static string TrimStart(string text, int characters)`

TrimStart method of Extensions.

**Parameters:**
- `text` (string)
- `characters` (int)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.TrimStart%28System.String%2CSystem.Int32%29)

#### `static bool TryAdd<TKey, TValue>(System.Collections.Generic.IDictionary<TKey, TValue> dictionary, TKey key, System.Func<TKey, TValue> valueFactory)`

TryAdd method of Extensions.

**Parameters:**
- `dictionary` (System.Collections.Generic.IDictionary<TKey, TValue>)
- `key` (TKey)
- `valueFactory` (System.Func<TKey, TValue>)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.TryAdd%60%602%28System.Collections.Generic.IDictionary%7B%60%600%2C%60%601%7D%2C%60%600%2CSystem.Func%7B%60%600%2C%60%601%7D%29)

#### `static bool TryAdd<TKey, TValue>(System.Collections.Generic.IDictionary<TKey, TValue> dictionary, TKey key, TValue value)`

TryAdd method of Extensions.

**Parameters:**
- `dictionary` (System.Collections.Generic.IDictionary<TKey, TValue>)
- `key` (TKey)
- `value` (TValue)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.TryAdd%60%602%28System.Collections.Generic.IDictionary%7B%60%600%2C%60%601%7D%2C%60%600%2C%60%601%29)

#### `static bool TryRemove<TKey, TValue>(System.Collections.Generic.IDictionary<TKey, TValue> dictionary, TKey key, out TValue removedValue)`

TryRemove method of Extensions.

**Parameters:**
- `dictionary` (System.Collections.Generic.IDictionary<TKey, TValue>)
- `key` (TKey)
- `removedValue` (TValue)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.TryRemove%60%602%28System.Collections.Generic.IDictionary%7B%60%600%2C%60%601%7D%2C%60%600%2C%60%601%40%29)

#### `static bool TryUpdate<TKey, TValue>(System.Collections.Generic.IDictionary<TKey, TValue> dictionary, TKey key, System.Func<TKey, TValue, TValue> updateValue)`

TryUpdate method of Extensions.

**Parameters:**
- `dictionary` (System.Collections.Generic.IDictionary<TKey, TValue>)
- `key` (TKey)
- `updateValue` (System.Func<TKey, TValue, TValue>)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.TryUpdate%60%602%28System.Collections.Generic.IDictionary%7B%60%600%2C%60%601%7D%2C%60%600%2CSystem.Func%7B%60%600%2C%60%601%2C%60%601%7D%29)

#### `static bool TryUpdate<TKey, TValue>(System.Collections.Generic.IDictionary<TKey, TValue> dictionary, TKey key, TValue value)`

TryUpdate method of Extensions.

**Parameters:**
- `dictionary` (System.Collections.Generic.IDictionary<TKey, TValue>)
- `key` (TKey)
- `value` (TValue)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.TryUpdate%60%602%28System.Collections.Generic.IDictionary%7B%60%600%2C%60%601%7D%2C%60%600%2C%60%601%29)

#### `static bool TryUpdate<TKey, TValue>(System.Collections.Generic.IDictionary<TKey, TValue> dictionary, TKey key, TValue value, out TValue replaced)`

TryUpdate method of Extensions.

**Parameters:**
- `dictionary` (System.Collections.Generic.IDictionary<TKey, TValue>)
- `key` (TKey)
- `value` (TValue)
- `replaced` (TValue)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.TryUpdate%60%602%28System.Collections.Generic.IDictionary%7B%60%600%2C%60%601%7D%2C%60%600%2C%60%601%2C%60%601%40%29)

#### `static System.Collections.Generic.IDictionary<TKey, TValue> WithValue<TKey, TValue>(System.Collections.Generic.IDictionary<TKey, TValue> dictionary, TKey key, System.Action<TValue> withValue)`

WithValue method of Extensions.

**Parameters:**
- `dictionary` (System.Collections.Generic.IDictionary<TKey, TValue>)
- `key` (TKey)
- `withValue` (System.Action<TValue>)

**Returns:** `System.Collections.Generic.IDictionary<TKey, TValue>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.WithValue%60%602%28System.Collections.Generic.IDictionary%7B%60%600%2C%60%601%7D%2C%60%600%2CSystem.Action%7B%60%601%7D%29)

#### `static void Write(System.IO.BinaryWriter writer, Fusion.Angle value)`

Write method of Extensions.

**Parameters:**
- `writer` (System.IO.BinaryWriter)
- `value` (Fusion.Angle)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Write%28System.IO.BinaryWriter%2CFusion.Angle%29)

#### `static void Write(System.IO.BinaryWriter writer, Fusion.Area value)`

Write method of Extensions.

**Parameters:**
- `writer` (System.IO.BinaryWriter)
- `value` (Fusion.Area)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Write%28System.IO.BinaryWriter%2CFusion.Area%29)

#### `static void Write(System.IO.BinaryWriter writer, Fusion.Double2 value)`

Write method of Extensions.

**Parameters:**
- `writer` (System.IO.BinaryWriter)
- `value` (Fusion.Double2)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Write%28System.IO.BinaryWriter%2CFusion.Double2%29)

#### `static void Write(System.IO.BinaryWriter writer, Fusion.Double3 value)`

Write method of Extensions.

**Parameters:**
- `writer` (System.IO.BinaryWriter)
- `value` (Fusion.Double3)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Write%28System.IO.BinaryWriter%2CFusion.Double3%29)

#### `static void Write(System.IO.BinaryWriter writer, Fusion.Double4 value)`

Write method of Extensions.

**Parameters:**
- `writer` (System.IO.BinaryWriter)
- `value` (Fusion.Double4)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Write%28System.IO.BinaryWriter%2CFusion.Double4%29)

#### `static void Write(System.IO.BinaryWriter writer, Fusion.Float2 value)`

Write method of Extensions.

**Parameters:**
- `writer` (System.IO.BinaryWriter)
- `value` (Fusion.Float2)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Write%28System.IO.BinaryWriter%2CFusion.Float2%29)

#### `static void Write(System.IO.BinaryWriter writer, Fusion.Float3 value)`

Write method of Extensions.

**Parameters:**
- `writer` (System.IO.BinaryWriter)
- `value` (Fusion.Float3)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Write%28System.IO.BinaryWriter%2CFusion.Float3%29)

#### `static void Write(System.IO.BinaryWriter writer, Fusion.Float4 value)`

Write method of Extensions.

**Parameters:**
- `writer` (System.IO.BinaryWriter)
- `value` (Fusion.Float4)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Write%28System.IO.BinaryWriter%2CFusion.Float4%29)

#### `static void Write(System.IO.BinaryWriter writer, Fusion.Float4x4 value)`

Write method of Extensions.

**Parameters:**
- `writer` (System.IO.BinaryWriter)
- `value` (Fusion.Float4x4)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Write%28System.IO.BinaryWriter%2CFusion.Float4x4%29)

#### `static void Write(System.IO.BinaryWriter writer, Fusion.FourCC value)`

Write method of Extensions.

**Parameters:**
- `writer` (System.IO.BinaryWriter)
- `value` (Fusion.FourCC)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Write%28System.IO.BinaryWriter%2CFusion.FourCC%29)

#### `static void Write(System.IO.BinaryWriter writer, Fusion.Int2 value)`

Write method of Extensions.

**Parameters:**
- `writer` (System.IO.BinaryWriter)
- `value` (Fusion.Int2)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Write%28System.IO.BinaryWriter%2CFusion.Int2%29)

#### `static void Write(System.IO.BinaryWriter writer, Fusion.Int3 value)`

Write method of Extensions.

**Parameters:**
- `writer` (System.IO.BinaryWriter)
- `value` (Fusion.Int3)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Write%28System.IO.BinaryWriter%2CFusion.Int3%29)

#### `static void Write(System.IO.BinaryWriter writer, Fusion.Int4 value)`

Write method of Extensions.

**Parameters:**
- `writer` (System.IO.BinaryWriter)
- `value` (Fusion.Int4)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Write%28System.IO.BinaryWriter%2CFusion.Int4%29)

#### `static void Write(System.IO.BinaryWriter writer, Fusion.Length value)`

Write method of Extensions.

**Parameters:**
- `writer` (System.IO.BinaryWriter)
- `value` (Fusion.Length)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Write%28System.IO.BinaryWriter%2CFusion.Length%29)

#### `static void Write(System.IO.BinaryWriter writer, Fusion.Mass value)`

Write method of Extensions.

**Parameters:**
- `writer` (System.IO.BinaryWriter)
- `value` (Fusion.Mass)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Write%28System.IO.BinaryWriter%2CFusion.Mass%29)

#### `static void Write(System.IO.BinaryWriter writer, Fusion.RGBA value)`

Write method of Extensions.

**Parameters:**
- `writer` (System.IO.BinaryWriter)
- `value` (Fusion.RGBA)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Write%28System.IO.BinaryWriter%2CFusion.RGBA%29)

#### `static void Write(System.IO.BinaryWriter writer, Fusion.Short2 value)`

Write method of Extensions.

**Parameters:**
- `writer` (System.IO.BinaryWriter)
- `value` (Fusion.Short2)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Write%28System.IO.BinaryWriter%2CFusion.Short2%29)

#### `static void Write(System.IO.BinaryWriter writer, Fusion.Short3 value)`

Write method of Extensions.

**Parameters:**
- `writer` (System.IO.BinaryWriter)
- `value` (Fusion.Short3)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Write%28System.IO.BinaryWriter%2CFusion.Short3%29)

#### `static void Write(System.IO.BinaryWriter writer, Fusion.Short4 value)`

Write method of Extensions.

**Parameters:**
- `writer` (System.IO.BinaryWriter)
- `value` (Fusion.Short4)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Write%28System.IO.BinaryWriter%2CFusion.Short4%29)

#### `static void Write(System.IO.BinaryWriter writer, Fusion.UShort2 value)`

Write method of Extensions.

**Parameters:**
- `writer` (System.IO.BinaryWriter)
- `value` (Fusion.UShort2)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Write%28System.IO.BinaryWriter%2CFusion.UShort2%29)

#### `static void Write(System.IO.BinaryWriter writer, Fusion.UShort3 value)`

Write method of Extensions.

**Parameters:**
- `writer` (System.IO.BinaryWriter)
- `value` (Fusion.UShort3)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Write%28System.IO.BinaryWriter%2CFusion.UShort3%29)

#### `static void Write(System.IO.BinaryWriter writer, Fusion.UShort4 value)`

Write method of Extensions.

**Parameters:**
- `writer` (System.IO.BinaryWriter)
- `value` (Fusion.UShort4)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Write%28System.IO.BinaryWriter%2CFusion.UShort4%29)

#### `static void Write(System.IO.BinaryWriter writer, Fusion.Volume value)`

Write method of Extensions.

**Parameters:**
- `writer` (System.IO.BinaryWriter)
- `value` (Fusion.Volume)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Write%28System.IO.BinaryWriter%2CFusion.Volume%29)

#### `static void Write(System.IO.BinaryWriter writer, System.Guid value)`

Write method of Extensions.

**Parameters:**
- `writer` (System.IO.BinaryWriter)
- `value` (System.Guid)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.Write%28System.IO.BinaryWriter%2CSystem.Guid%29)

#### `static void WritePackedInt32(System.IO.BinaryWriter writer, int value)`

WritePackedInt32 method of Extensions.

**Parameters:**
- `writer` (System.IO.BinaryWriter)
- `value` (int)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Extensions.WritePackedInt32%28System.IO.BinaryWriter%2CSystem.Int32%29)

## FileType (class)

FileType class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.FileType)

### Constructors
- `FileType(string description, string[] extensions)` — Constructs a new FileType.

### Properties
- `Description` (string, get) — Description property of FileType.
- `Extensions` (string[], get) — Extensions property of FileType.

## Float2 (struct)

Float2 struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Float2)

### Constructors
- `Float2(float all)` — Constructs a new Float2.
- `Float2(float x, float y)` — Constructs a new Float2.

### Methods
#### `static Fusion.Float2 Abs(Fusion.Float2 value)`

Abs method of Float2.

**Parameters:**
- `value` (Fusion.Float2)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.Abs%28Fusion.Float2%29)

#### `static Fusion.Float2 Ceiling(Fusion.Float2 value)`

Ceiling method of Float2.

**Parameters:**
- `value` (Fusion.Float2)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.Ceiling%28Fusion.Float2%29)

#### `bool Equals(Fusion.Float2 other)`

Equals method of Float2.

**Parameters:**
- `other` (Fusion.Float2)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.Equals%28Fusion.Float2%29)

#### `bool Equals(object other)`

Equals method of Float2.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.Equals%28System.Object%29)

#### `static Fusion.Float2 Floor(Fusion.Float2 value)`

Floor method of Float2.

**Parameters:**
- `value` (Fusion.Float2)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.Floor%28Fusion.Float2%29)

#### `int GetHashCode()`

GetHashCode method of Float2.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.GetHashCode)

#### `static bool IsInfinity(Fusion.Float2 value)`

IsInfinity method of Float2.

**Parameters:**
- `value` (Fusion.Float2)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.IsInfinity%28Fusion.Float2%29)

#### `static bool IsNaN(Fusion.Float2 value)`

IsNaN method of Float2.

**Parameters:**
- `value` (Fusion.Float2)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.IsNaN%28Fusion.Float2%29)

#### `static bool IsNegativeInfinity(Fusion.Float2 value)`

IsNegativeInfinity method of Float2.

**Parameters:**
- `value` (Fusion.Float2)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.IsNegativeInfinity%28Fusion.Float2%29)

#### `static bool IsPositiveInfinity(Fusion.Float2 value)`

IsPositiveInfinity method of Float2.

**Parameters:**
- `value` (Fusion.Float2)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.IsPositiveInfinity%28Fusion.Float2%29)

#### `static Fusion.Float2 Max(Fusion.Float2 a, Fusion.Float2 b)`

Max method of Float2.

**Parameters:**
- `a` (Fusion.Float2)
- `b` (Fusion.Float2)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.Max%28Fusion.Float2%2CFusion.Float2%29)

#### `static Fusion.Float2 Min(Fusion.Float2 a, Fusion.Float2 b)`

Min method of Float2.

**Parameters:**
- `a` (Fusion.Float2)
- `b` (Fusion.Float2)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.Min%28Fusion.Float2%2CFusion.Float2%29)

#### `bool Near(Fusion.Float2 other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of Float2.

**Parameters:**
- `other` (Fusion.Float2)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.Near%28Fusion.Float2%2CSystem.Int32%29)

#### `static Fusion.Float2 Pow(Fusion.Float2 a, Fusion.Float2 b)`

Pow method of Float2.

**Parameters:**
- `a` (Fusion.Float2)
- `b` (Fusion.Float2)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.Pow%28Fusion.Float2%2CFusion.Float2%29)

#### `static Fusion.Float2 Pow(Fusion.Float2 a, float b)`

Pow method of Float2.

**Parameters:**
- `a` (Fusion.Float2)
- `b` (float)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.Pow%28Fusion.Float2%2CSystem.Single%29)

#### `static Fusion.Float2 Pow(float a, Fusion.Float2 b)`

Pow method of Float2.

**Parameters:**
- `a` (float)
- `b` (Fusion.Float2)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.Pow%28System.Single%2CFusion.Float2%29)

#### `static Fusion.Float2 Round(Fusion.Float2 value)`

Round method of Float2.

**Parameters:**
- `value` (Fusion.Float2)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.Round%28Fusion.Float2%29)

#### `static Fusion.Float2 Round(Fusion.Float2 value, System.MidpointRounding midpointRounding)`

Round method of Float2.

**Parameters:**
- `value` (Fusion.Float2)
- `midpointRounding` (System.MidpointRounding)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.Round%28Fusion.Float2%2CSystem.MidpointRounding%29)

#### `static Fusion.Float2 Round(Fusion.Float2 value, int digits)`

Round method of Float2.

**Parameters:**
- `value` (Fusion.Float2)
- `digits` (int)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.Round%28Fusion.Float2%2CSystem.Int32%29)

#### `static Fusion.Float2 Round(Fusion.Float2 value, int digits, System.MidpointRounding midpointRounding)`

Round method of Float2.

**Parameters:**
- `value` (Fusion.Float2)
- `digits` (int)
- `midpointRounding` (System.MidpointRounding)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.Round%28Fusion.Float2%2CSystem.Int32%2CSystem.MidpointRounding%29)

#### `static Fusion.Int2 Sign(Fusion.Float2 value)`

Sign method of Float2.

**Parameters:**
- `value` (Fusion.Float2)

**Returns:** `Fusion.Int2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.Sign%28Fusion.Float2%29)

#### `static Fusion.Float2 Sqrt(Fusion.Float2 value)`

Sqrt method of Float2.

**Parameters:**
- `value` (Fusion.Float2)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.Sqrt%28Fusion.Float2%29)

#### `string ToString()`

ToString method of Float2.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of Float2.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.Float2 Truncate(Fusion.Float2 value)`

Truncate method of Float2.

**Parameters:**
- `value` (Fusion.Float2)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.Truncate%28Fusion.Float2%29)

#### `static Fusion.Float2 op_Addition(Fusion.Float2 a, Fusion.Float2 b)`

op_Addition method of Float2.

**Parameters:**
- `a` (Fusion.Float2)
- `b` (Fusion.Float2)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.op_Addition%28Fusion.Float2%2CFusion.Float2%29)

#### `static Fusion.Float2 op_Addition(Fusion.Float2 a, float b)`

op_Addition method of Float2.

**Parameters:**
- `a` (Fusion.Float2)
- `b` (float)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.op_Addition%28Fusion.Float2%2CSystem.Single%29)

#### `static Fusion.Float2 op_Addition(float a, Fusion.Float2 b)`

op_Addition method of Float2.

**Parameters:**
- `a` (float)
- `b` (Fusion.Float2)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.op_Addition%28System.Single%2CFusion.Float2%29)

#### `static Fusion.Float2 op_Division(Fusion.Float2 a, Fusion.Float2 b)`

op_Division method of Float2.

**Parameters:**
- `a` (Fusion.Float2)
- `b` (Fusion.Float2)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.op_Division%28Fusion.Float2%2CFusion.Float2%29)

#### `static Fusion.Float2 op_Division(Fusion.Float2 a, float b)`

op_Division method of Float2.

**Parameters:**
- `a` (Fusion.Float2)
- `b` (float)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.op_Division%28Fusion.Float2%2CSystem.Single%29)

#### `static Fusion.Float2 op_Division(float a, Fusion.Float2 b)`

op_Division method of Float2.

**Parameters:**
- `a` (float)
- `b` (Fusion.Float2)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.op_Division%28System.Single%2CFusion.Float2%29)

#### `static bool op_Equality(Fusion.Float2 a, Fusion.Float2 b)`

op_Equality method of Float2.

**Parameters:**
- `a` (Fusion.Float2)
- `b` (Fusion.Float2)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.op_Equality%28Fusion.Float2%2CFusion.Float2%29)

#### `static Fusion.Float2 op_Explicit(Fusion.Double2 v)`

op_Explicit method of Float2.

**Parameters:**
- `v` (Fusion.Double2)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.op_Explicit%28Fusion.Double2%29~Fusion.Float2)

#### `static Fusion.Float2 op_Implicit(Fusion.Half2 v)`

op_Implicit method of Float2.

**Parameters:**
- `v` (Fusion.Half2)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.op_Implicit%28Fusion.Half2%29~Fusion.Float2)

#### `static Fusion.Float2 op_Implicit(Fusion.Int2 v)`

op_Implicit method of Float2.

**Parameters:**
- `v` (Fusion.Int2)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.op_Implicit%28Fusion.Int2%29~Fusion.Float2)

#### `static Fusion.Float2 op_Implicit(Fusion.Short2 v)`

op_Implicit method of Float2.

**Parameters:**
- `v` (Fusion.Short2)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.op_Implicit%28Fusion.Short2%29~Fusion.Float2)

#### `static Fusion.Float2 op_Implicit(Fusion.UShort2 v)`

op_Implicit method of Float2.

**Parameters:**
- `v` (Fusion.UShort2)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.op_Implicit%28Fusion.UShort2%29~Fusion.Float2)

#### `static bool op_Inequality(Fusion.Float2 a, Fusion.Float2 b)`

op_Inequality method of Float2.

**Parameters:**
- `a` (Fusion.Float2)
- `b` (Fusion.Float2)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.op_Inequality%28Fusion.Float2%2CFusion.Float2%29)

#### `static Fusion.Float2 op_Modulus(Fusion.Float2 a, Fusion.Float2 b)`

op_Modulus method of Float2.

**Parameters:**
- `a` (Fusion.Float2)
- `b` (Fusion.Float2)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.op_Modulus%28Fusion.Float2%2CFusion.Float2%29)

#### `static Fusion.Float2 op_Modulus(Fusion.Float2 a, float b)`

op_Modulus method of Float2.

**Parameters:**
- `a` (Fusion.Float2)
- `b` (float)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.op_Modulus%28Fusion.Float2%2CSystem.Single%29)

#### `static Fusion.Float2 op_Modulus(float a, Fusion.Float2 b)`

op_Modulus method of Float2.

**Parameters:**
- `a` (float)
- `b` (Fusion.Float2)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.op_Modulus%28System.Single%2CFusion.Float2%29)

#### `static Fusion.Float2 op_Multiply(Fusion.Float2 a, Fusion.Float2 b)`

op_Multiply method of Float2.

**Parameters:**
- `a` (Fusion.Float2)
- `b` (Fusion.Float2)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.op_Multiply%28Fusion.Float2%2CFusion.Float2%29)

#### `static Fusion.Float2 op_Multiply(Fusion.Float2 a, float b)`

op_Multiply method of Float2.

**Parameters:**
- `a` (Fusion.Float2)
- `b` (float)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.op_Multiply%28Fusion.Float2%2CSystem.Single%29)

#### `static Fusion.Float2 op_Multiply(float a, Fusion.Float2 b)`

op_Multiply method of Float2.

**Parameters:**
- `a` (float)
- `b` (Fusion.Float2)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.op_Multiply%28System.Single%2CFusion.Float2%29)

#### `static Fusion.Float2 op_Subtraction(Fusion.Float2 a, Fusion.Float2 b)`

op_Subtraction method of Float2.

**Parameters:**
- `a` (Fusion.Float2)
- `b` (Fusion.Float2)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.op_Subtraction%28Fusion.Float2%2CFusion.Float2%29)

#### `static Fusion.Float2 op_Subtraction(Fusion.Float2 a, float b)`

op_Subtraction method of Float2.

**Parameters:**
- `a` (Fusion.Float2)
- `b` (float)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.op_Subtraction%28Fusion.Float2%2CSystem.Single%29)

#### `static Fusion.Float2 op_Subtraction(float a, Fusion.Float2 b)`

op_Subtraction method of Float2.

**Parameters:**
- `a` (float)
- `b` (Fusion.Float2)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.op_Subtraction%28System.Single%2CFusion.Float2%29)

#### `static Fusion.Float2 op_UnaryNegation(Fusion.Float2 v)`

op_UnaryNegation method of Float2.

**Parameters:**
- `v` (Fusion.Float2)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.op_UnaryNegation%28Fusion.Float2%29)

#### `static Fusion.Float2 op_UnaryPlus(Fusion.Float2 v)`

op_UnaryPlus method of Float2.

**Parameters:**
- `v` (Fusion.Float2)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float2.op_UnaryPlus%28Fusion.Float2%29)

### Properties
- `Item` (float, get/set) — Item property of Float2.

## Float3 (struct)

Float3 struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Float3)

### Constructors
- `Float3(Fusion.Float2 xy, float z)` — Constructs a new Float3.
- `Float3(float all)` — Constructs a new Float3.
- `Float3(float x, Fusion.Float2 yz)` — Constructs a new Float3.
- `Float3(float x, float y, float z)` — Constructs a new Float3.

### Methods
#### `static Fusion.Float3 Abs(Fusion.Float3 value)`

Abs method of Float3.

**Parameters:**
- `value` (Fusion.Float3)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.Abs%28Fusion.Float3%29)

#### `static Fusion.Float3 Ceiling(Fusion.Float3 value)`

Ceiling method of Float3.

**Parameters:**
- `value` (Fusion.Float3)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.Ceiling%28Fusion.Float3%29)

#### `bool Equals(Fusion.Float3 other)`

Equals method of Float3.

**Parameters:**
- `other` (Fusion.Float3)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.Equals%28Fusion.Float3%29)

#### `bool Equals(object other)`

Equals method of Float3.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.Equals%28System.Object%29)

#### `static Fusion.Float3 Floor(Fusion.Float3 value)`

Floor method of Float3.

**Parameters:**
- `value` (Fusion.Float3)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.Floor%28Fusion.Float3%29)

#### `int GetHashCode()`

GetHashCode method of Float3.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.GetHashCode)

#### `static bool IsInfinity(Fusion.Float3 value)`

IsInfinity method of Float3.

**Parameters:**
- `value` (Fusion.Float3)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.IsInfinity%28Fusion.Float3%29)

#### `static bool IsNaN(Fusion.Float3 value)`

IsNaN method of Float3.

**Parameters:**
- `value` (Fusion.Float3)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.IsNaN%28Fusion.Float3%29)

#### `static bool IsNegativeInfinity(Fusion.Float3 value)`

IsNegativeInfinity method of Float3.

**Parameters:**
- `value` (Fusion.Float3)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.IsNegativeInfinity%28Fusion.Float3%29)

#### `static bool IsPositiveInfinity(Fusion.Float3 value)`

IsPositiveInfinity method of Float3.

**Parameters:**
- `value` (Fusion.Float3)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.IsPositiveInfinity%28Fusion.Float3%29)

#### `static Fusion.Float3 Max(Fusion.Float3 a, Fusion.Float3 b)`

Max method of Float3.

**Parameters:**
- `a` (Fusion.Float3)
- `b` (Fusion.Float3)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.Max%28Fusion.Float3%2CFusion.Float3%29)

#### `static Fusion.Float3 Min(Fusion.Float3 a, Fusion.Float3 b)`

Min method of Float3.

**Parameters:**
- `a` (Fusion.Float3)
- `b` (Fusion.Float3)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.Min%28Fusion.Float3%2CFusion.Float3%29)

#### `bool Near(Fusion.Float3 other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of Float3.

**Parameters:**
- `other` (Fusion.Float3)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.Near%28Fusion.Float3%2CSystem.Int32%29)

#### `static Fusion.Float3 Pow(Fusion.Float3 a, Fusion.Float3 b)`

Pow method of Float3.

**Parameters:**
- `a` (Fusion.Float3)
- `b` (Fusion.Float3)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.Pow%28Fusion.Float3%2CFusion.Float3%29)

#### `static Fusion.Float3 Pow(Fusion.Float3 a, float b)`

Pow method of Float3.

**Parameters:**
- `a` (Fusion.Float3)
- `b` (float)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.Pow%28Fusion.Float3%2CSystem.Single%29)

#### `static Fusion.Float3 Pow(float a, Fusion.Float3 b)`

Pow method of Float3.

**Parameters:**
- `a` (float)
- `b` (Fusion.Float3)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.Pow%28System.Single%2CFusion.Float3%29)

#### `static Fusion.Float3 Round(Fusion.Float3 value)`

Round method of Float3.

**Parameters:**
- `value` (Fusion.Float3)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.Round%28Fusion.Float3%29)

#### `static Fusion.Float3 Round(Fusion.Float3 value, System.MidpointRounding midpointRounding)`

Round method of Float3.

**Parameters:**
- `value` (Fusion.Float3)
- `midpointRounding` (System.MidpointRounding)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.Round%28Fusion.Float3%2CSystem.MidpointRounding%29)

#### `static Fusion.Float3 Round(Fusion.Float3 value, int digits)`

Round method of Float3.

**Parameters:**
- `value` (Fusion.Float3)
- `digits` (int)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.Round%28Fusion.Float3%2CSystem.Int32%29)

#### `static Fusion.Float3 Round(Fusion.Float3 value, int digits, System.MidpointRounding midpointRounding)`

Round method of Float3.

**Parameters:**
- `value` (Fusion.Float3)
- `digits` (int)
- `midpointRounding` (System.MidpointRounding)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.Round%28Fusion.Float3%2CSystem.Int32%2CSystem.MidpointRounding%29)

#### `static Fusion.Int3 Sign(Fusion.Float3 value)`

Sign method of Float3.

**Parameters:**
- `value` (Fusion.Float3)

**Returns:** `Fusion.Int3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.Sign%28Fusion.Float3%29)

#### `static Fusion.Float3 Sqrt(Fusion.Float3 value)`

Sqrt method of Float3.

**Parameters:**
- `value` (Fusion.Float3)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.Sqrt%28Fusion.Float3%29)

#### `string ToString()`

ToString method of Float3.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of Float3.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.Float3 Truncate(Fusion.Float3 value)`

Truncate method of Float3.

**Parameters:**
- `value` (Fusion.Float3)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.Truncate%28Fusion.Float3%29)

#### `static Fusion.Float3 op_Addition(Fusion.Float3 a, Fusion.Float3 b)`

op_Addition method of Float3.

**Parameters:**
- `a` (Fusion.Float3)
- `b` (Fusion.Float3)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.op_Addition%28Fusion.Float3%2CFusion.Float3%29)

#### `static Fusion.Float3 op_Addition(Fusion.Float3 a, float b)`

op_Addition method of Float3.

**Parameters:**
- `a` (Fusion.Float3)
- `b` (float)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.op_Addition%28Fusion.Float3%2CSystem.Single%29)

#### `static Fusion.Float3 op_Addition(float a, Fusion.Float3 b)`

op_Addition method of Float3.

**Parameters:**
- `a` (float)
- `b` (Fusion.Float3)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.op_Addition%28System.Single%2CFusion.Float3%29)

#### `static Fusion.Float3 op_Division(Fusion.Float3 a, Fusion.Float3 b)`

op_Division method of Float3.

**Parameters:**
- `a` (Fusion.Float3)
- `b` (Fusion.Float3)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.op_Division%28Fusion.Float3%2CFusion.Float3%29)

#### `static Fusion.Float3 op_Division(Fusion.Float3 a, float b)`

op_Division method of Float3.

**Parameters:**
- `a` (Fusion.Float3)
- `b` (float)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.op_Division%28Fusion.Float3%2CSystem.Single%29)

#### `static Fusion.Float3 op_Division(float a, Fusion.Float3 b)`

op_Division method of Float3.

**Parameters:**
- `a` (float)
- `b` (Fusion.Float3)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.op_Division%28System.Single%2CFusion.Float3%29)

#### `static bool op_Equality(Fusion.Float3 a, Fusion.Float3 b)`

op_Equality method of Float3.

**Parameters:**
- `a` (Fusion.Float3)
- `b` (Fusion.Float3)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.op_Equality%28Fusion.Float3%2CFusion.Float3%29)

#### `static Fusion.Float3 op_Explicit(Fusion.Double3 v)`

op_Explicit method of Float3.

**Parameters:**
- `v` (Fusion.Double3)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.op_Explicit%28Fusion.Double3%29~Fusion.Float3)

#### `static Fusion.Float3 op_Implicit(Fusion.Half3 v)`

op_Implicit method of Float3.

**Parameters:**
- `v` (Fusion.Half3)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.op_Implicit%28Fusion.Half3%29~Fusion.Float3)

#### `static Fusion.Float3 op_Implicit(Fusion.Int3 v)`

op_Implicit method of Float3.

**Parameters:**
- `v` (Fusion.Int3)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.op_Implicit%28Fusion.Int3%29~Fusion.Float3)

#### `static Fusion.Float3 op_Implicit(Fusion.Short3 v)`

op_Implicit method of Float3.

**Parameters:**
- `v` (Fusion.Short3)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.op_Implicit%28Fusion.Short3%29~Fusion.Float3)

#### `static Fusion.Float3 op_Implicit(Fusion.UShort3 v)`

op_Implicit method of Float3.

**Parameters:**
- `v` (Fusion.UShort3)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.op_Implicit%28Fusion.UShort3%29~Fusion.Float3)

#### `static bool op_Inequality(Fusion.Float3 a, Fusion.Float3 b)`

op_Inequality method of Float3.

**Parameters:**
- `a` (Fusion.Float3)
- `b` (Fusion.Float3)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.op_Inequality%28Fusion.Float3%2CFusion.Float3%29)

#### `static Fusion.Float3 op_Modulus(Fusion.Float3 a, Fusion.Float3 b)`

op_Modulus method of Float3.

**Parameters:**
- `a` (Fusion.Float3)
- `b` (Fusion.Float3)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.op_Modulus%28Fusion.Float3%2CFusion.Float3%29)

#### `static Fusion.Float3 op_Modulus(Fusion.Float3 a, float b)`

op_Modulus method of Float3.

**Parameters:**
- `a` (Fusion.Float3)
- `b` (float)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.op_Modulus%28Fusion.Float3%2CSystem.Single%29)

#### `static Fusion.Float3 op_Modulus(float a, Fusion.Float3 b)`

op_Modulus method of Float3.

**Parameters:**
- `a` (float)
- `b` (Fusion.Float3)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.op_Modulus%28System.Single%2CFusion.Float3%29)

#### `static Fusion.Float3 op_Multiply(Fusion.Float3 a, Fusion.Float3 b)`

op_Multiply method of Float3.

**Parameters:**
- `a` (Fusion.Float3)
- `b` (Fusion.Float3)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.op_Multiply%28Fusion.Float3%2CFusion.Float3%29)

#### `static Fusion.Float3 op_Multiply(Fusion.Float3 a, float b)`

op_Multiply method of Float3.

**Parameters:**
- `a` (Fusion.Float3)
- `b` (float)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.op_Multiply%28Fusion.Float3%2CSystem.Single%29)

#### `static Fusion.Float3 op_Multiply(float a, Fusion.Float3 b)`

op_Multiply method of Float3.

**Parameters:**
- `a` (float)
- `b` (Fusion.Float3)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.op_Multiply%28System.Single%2CFusion.Float3%29)

#### `static Fusion.Float3 op_Subtraction(Fusion.Float3 a, Fusion.Float3 b)`

op_Subtraction method of Float3.

**Parameters:**
- `a` (Fusion.Float3)
- `b` (Fusion.Float3)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.op_Subtraction%28Fusion.Float3%2CFusion.Float3%29)

#### `static Fusion.Float3 op_Subtraction(Fusion.Float3 a, float b)`

op_Subtraction method of Float3.

**Parameters:**
- `a` (Fusion.Float3)
- `b` (float)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.op_Subtraction%28Fusion.Float3%2CSystem.Single%29)

#### `static Fusion.Float3 op_Subtraction(float a, Fusion.Float3 b)`

op_Subtraction method of Float3.

**Parameters:**
- `a` (float)
- `b` (Fusion.Float3)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.op_Subtraction%28System.Single%2CFusion.Float3%29)

#### `static Fusion.Float3 op_UnaryNegation(Fusion.Float3 v)`

op_UnaryNegation method of Float3.

**Parameters:**
- `v` (Fusion.Float3)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.op_UnaryNegation%28Fusion.Float3%29)

#### `static Fusion.Float3 op_UnaryPlus(Fusion.Float3 v)`

op_UnaryPlus method of Float3.

**Parameters:**
- `v` (Fusion.Float3)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float3.op_UnaryPlus%28Fusion.Float3%29)

### Properties
- `Item` (float, get/set) — Item property of Float3.
- `XY` (Fusion.Float2, get) — XY property of Float3.

## Float4 (struct)

Float4 struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Float4)

### Constructors
- `Float4(Fusion.Float2 xy, Fusion.Float2 zw)` — Constructs a new Float4.
- `Float4(Fusion.Float2 xy, float z, float w)` — Constructs a new Float4.
- `Float4(Fusion.Float3 xyz, float w)` — Constructs a new Float4.
- `Float4(float all)` — Constructs a new Float4.
- `Float4(float x, Fusion.Float2 yz, float w)` — Constructs a new Float4.
- `Float4(float x, Fusion.Float3 yzw)` — Constructs a new Float4.
- `Float4(float x, float y, Fusion.Float2 zw)` — Constructs a new Float4.
- `Float4(float x, float y, float z, float w)` — Constructs a new Float4.

### Methods
#### `static Fusion.Float4 Abs(Fusion.Float4 value)`

Abs method of Float4.

**Parameters:**
- `value` (Fusion.Float4)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.Abs%28Fusion.Float4%29)

#### `static Fusion.Float4 Ceiling(Fusion.Float4 value)`

Ceiling method of Float4.

**Parameters:**
- `value` (Fusion.Float4)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.Ceiling%28Fusion.Float4%29)

#### `bool Equals(Fusion.Float4 other)`

Equals method of Float4.

**Parameters:**
- `other` (Fusion.Float4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.Equals%28Fusion.Float4%29)

#### `bool Equals(object other)`

Equals method of Float4.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.Equals%28System.Object%29)

#### `static Fusion.Float4 Floor(Fusion.Float4 value)`

Floor method of Float4.

**Parameters:**
- `value` (Fusion.Float4)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.Floor%28Fusion.Float4%29)

#### `int GetHashCode()`

GetHashCode method of Float4.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.GetHashCode)

#### `static bool IsInfinity(Fusion.Float4 value)`

IsInfinity method of Float4.

**Parameters:**
- `value` (Fusion.Float4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.IsInfinity%28Fusion.Float4%29)

#### `static bool IsNaN(Fusion.Float4 value)`

IsNaN method of Float4.

**Parameters:**
- `value` (Fusion.Float4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.IsNaN%28Fusion.Float4%29)

#### `static bool IsNegativeInfinity(Fusion.Float4 value)`

IsNegativeInfinity method of Float4.

**Parameters:**
- `value` (Fusion.Float4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.IsNegativeInfinity%28Fusion.Float4%29)

#### `static bool IsPositiveInfinity(Fusion.Float4 value)`

IsPositiveInfinity method of Float4.

**Parameters:**
- `value` (Fusion.Float4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.IsPositiveInfinity%28Fusion.Float4%29)

#### `static Fusion.Float4 Max(Fusion.Float4 a, Fusion.Float4 b)`

Max method of Float4.

**Parameters:**
- `a` (Fusion.Float4)
- `b` (Fusion.Float4)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.Max%28Fusion.Float4%2CFusion.Float4%29)

#### `static Fusion.Float4 Min(Fusion.Float4 a, Fusion.Float4 b)`

Min method of Float4.

**Parameters:**
- `a` (Fusion.Float4)
- `b` (Fusion.Float4)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.Min%28Fusion.Float4%2CFusion.Float4%29)

#### `bool Near(Fusion.Float4 other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of Float4.

**Parameters:**
- `other` (Fusion.Float4)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.Near%28Fusion.Float4%2CSystem.Int32%29)

#### `static Fusion.Float4 Pow(Fusion.Float4 a, Fusion.Float4 b)`

Pow method of Float4.

**Parameters:**
- `a` (Fusion.Float4)
- `b` (Fusion.Float4)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.Pow%28Fusion.Float4%2CFusion.Float4%29)

#### `static Fusion.Float4 Pow(Fusion.Float4 a, float b)`

Pow method of Float4.

**Parameters:**
- `a` (Fusion.Float4)
- `b` (float)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.Pow%28Fusion.Float4%2CSystem.Single%29)

#### `static Fusion.Float4 Pow(float a, Fusion.Float4 b)`

Pow method of Float4.

**Parameters:**
- `a` (float)
- `b` (Fusion.Float4)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.Pow%28System.Single%2CFusion.Float4%29)

#### `static Fusion.Float4 Round(Fusion.Float4 value)`

Round method of Float4.

**Parameters:**
- `value` (Fusion.Float4)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.Round%28Fusion.Float4%29)

#### `static Fusion.Float4 Round(Fusion.Float4 value, System.MidpointRounding midpointRounding)`

Round method of Float4.

**Parameters:**
- `value` (Fusion.Float4)
- `midpointRounding` (System.MidpointRounding)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.Round%28Fusion.Float4%2CSystem.MidpointRounding%29)

#### `static Fusion.Float4 Round(Fusion.Float4 value, int digits)`

Round method of Float4.

**Parameters:**
- `value` (Fusion.Float4)
- `digits` (int)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.Round%28Fusion.Float4%2CSystem.Int32%29)

#### `static Fusion.Float4 Round(Fusion.Float4 value, int digits, System.MidpointRounding midpointRounding)`

Round method of Float4.

**Parameters:**
- `value` (Fusion.Float4)
- `digits` (int)
- `midpointRounding` (System.MidpointRounding)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.Round%28Fusion.Float4%2CSystem.Int32%2CSystem.MidpointRounding%29)

#### `static Fusion.Int4 Sign(Fusion.Float4 value)`

Sign method of Float4.

**Parameters:**
- `value` (Fusion.Float4)

**Returns:** `Fusion.Int4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.Sign%28Fusion.Float4%29)

#### `static Fusion.Float4 Sqrt(Fusion.Float4 value)`

Sqrt method of Float4.

**Parameters:**
- `value` (Fusion.Float4)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.Sqrt%28Fusion.Float4%29)

#### `string ToString()`

ToString method of Float4.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of Float4.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.Float4 Truncate(Fusion.Float4 value)`

Truncate method of Float4.

**Parameters:**
- `value` (Fusion.Float4)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.Truncate%28Fusion.Float4%29)

#### `static Fusion.Float4 op_Addition(Fusion.Float4 a, Fusion.Float4 b)`

op_Addition method of Float4.

**Parameters:**
- `a` (Fusion.Float4)
- `b` (Fusion.Float4)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.op_Addition%28Fusion.Float4%2CFusion.Float4%29)

#### `static Fusion.Float4 op_Addition(Fusion.Float4 a, float b)`

op_Addition method of Float4.

**Parameters:**
- `a` (Fusion.Float4)
- `b` (float)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.op_Addition%28Fusion.Float4%2CSystem.Single%29)

#### `static Fusion.Float4 op_Addition(float a, Fusion.Float4 b)`

op_Addition method of Float4.

**Parameters:**
- `a` (float)
- `b` (Fusion.Float4)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.op_Addition%28System.Single%2CFusion.Float4%29)

#### `static Fusion.Float4 op_Division(Fusion.Float4 a, Fusion.Float4 b)`

op_Division method of Float4.

**Parameters:**
- `a` (Fusion.Float4)
- `b` (Fusion.Float4)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.op_Division%28Fusion.Float4%2CFusion.Float4%29)

#### `static Fusion.Float4 op_Division(Fusion.Float4 a, float b)`

op_Division method of Float4.

**Parameters:**
- `a` (Fusion.Float4)
- `b` (float)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.op_Division%28Fusion.Float4%2CSystem.Single%29)

#### `static Fusion.Float4 op_Division(float a, Fusion.Float4 b)`

op_Division method of Float4.

**Parameters:**
- `a` (float)
- `b` (Fusion.Float4)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.op_Division%28System.Single%2CFusion.Float4%29)

#### `static bool op_Equality(Fusion.Float4 a, Fusion.Float4 b)`

op_Equality method of Float4.

**Parameters:**
- `a` (Fusion.Float4)
- `b` (Fusion.Float4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.op_Equality%28Fusion.Float4%2CFusion.Float4%29)

#### `static Fusion.Float4 op_Explicit(Fusion.Double4 v)`

op_Explicit method of Float4.

**Parameters:**
- `v` (Fusion.Double4)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.op_Explicit%28Fusion.Double4%29~Fusion.Float4)

#### `static Fusion.Float4 op_Implicit(Fusion.Half4 v)`

op_Implicit method of Float4.

**Parameters:**
- `v` (Fusion.Half4)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.op_Implicit%28Fusion.Half4%29~Fusion.Float4)

#### `static Fusion.Float4 op_Implicit(Fusion.Int4 v)`

op_Implicit method of Float4.

**Parameters:**
- `v` (Fusion.Int4)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.op_Implicit%28Fusion.Int4%29~Fusion.Float4)

#### `static Fusion.Float4 op_Implicit(Fusion.RGBA v)`

op_Implicit method of Float4.

**Parameters:**
- `v` (Fusion.RGBA)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.op_Implicit%28Fusion.RGBA%29~Fusion.Float4)

#### `static Fusion.Float4 op_Implicit(Fusion.Short4 v)`

op_Implicit method of Float4.

**Parameters:**
- `v` (Fusion.Short4)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.op_Implicit%28Fusion.Short4%29~Fusion.Float4)

#### `static Fusion.Float4 op_Implicit(Fusion.UShort4 v)`

op_Implicit method of Float4.

**Parameters:**
- `v` (Fusion.UShort4)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.op_Implicit%28Fusion.UShort4%29~Fusion.Float4)

#### `static bool op_Inequality(Fusion.Float4 a, Fusion.Float4 b)`

op_Inequality method of Float4.

**Parameters:**
- `a` (Fusion.Float4)
- `b` (Fusion.Float4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.op_Inequality%28Fusion.Float4%2CFusion.Float4%29)

#### `static Fusion.Float4 op_Modulus(Fusion.Float4 a, Fusion.Float4 b)`

op_Modulus method of Float4.

**Parameters:**
- `a` (Fusion.Float4)
- `b` (Fusion.Float4)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.op_Modulus%28Fusion.Float4%2CFusion.Float4%29)

#### `static Fusion.Float4 op_Modulus(Fusion.Float4 a, float b)`

op_Modulus method of Float4.

**Parameters:**
- `a` (Fusion.Float4)
- `b` (float)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.op_Modulus%28Fusion.Float4%2CSystem.Single%29)

#### `static Fusion.Float4 op_Modulus(float a, Fusion.Float4 b)`

op_Modulus method of Float4.

**Parameters:**
- `a` (float)
- `b` (Fusion.Float4)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.op_Modulus%28System.Single%2CFusion.Float4%29)

#### `static Fusion.Float4 op_Multiply(Fusion.Float4 a, Fusion.Float4 b)`

op_Multiply method of Float4.

**Parameters:**
- `a` (Fusion.Float4)
- `b` (Fusion.Float4)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.op_Multiply%28Fusion.Float4%2CFusion.Float4%29)

#### `static Fusion.Float4 op_Multiply(Fusion.Float4 a, float b)`

op_Multiply method of Float4.

**Parameters:**
- `a` (Fusion.Float4)
- `b` (float)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.op_Multiply%28Fusion.Float4%2CSystem.Single%29)

#### `static Fusion.Float4 op_Multiply(float a, Fusion.Float4 b)`

op_Multiply method of Float4.

**Parameters:**
- `a` (float)
- `b` (Fusion.Float4)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.op_Multiply%28System.Single%2CFusion.Float4%29)

#### `static Fusion.Float4 op_Subtraction(Fusion.Float4 a, Fusion.Float4 b)`

op_Subtraction method of Float4.

**Parameters:**
- `a` (Fusion.Float4)
- `b` (Fusion.Float4)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.op_Subtraction%28Fusion.Float4%2CFusion.Float4%29)

#### `static Fusion.Float4 op_Subtraction(Fusion.Float4 a, float b)`

op_Subtraction method of Float4.

**Parameters:**
- `a` (Fusion.Float4)
- `b` (float)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.op_Subtraction%28Fusion.Float4%2CSystem.Single%29)

#### `static Fusion.Float4 op_Subtraction(float a, Fusion.Float4 b)`

op_Subtraction method of Float4.

**Parameters:**
- `a` (float)
- `b` (Fusion.Float4)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.op_Subtraction%28System.Single%2CFusion.Float4%29)

#### `static Fusion.Float4 op_UnaryNegation(Fusion.Float4 v)`

op_UnaryNegation method of Float4.

**Parameters:**
- `v` (Fusion.Float4)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.op_UnaryNegation%28Fusion.Float4%29)

#### `static Fusion.Float4 op_UnaryPlus(Fusion.Float4 v)`

op_UnaryPlus method of Float4.

**Parameters:**
- `v` (Fusion.Float4)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4.op_UnaryPlus%28Fusion.Float4%29)

### Properties
- `Item` (float, get/set) — Item property of Float4.
- `XY` (Fusion.Float2, get) — XY property of Float4.
- `XYZ` (Fusion.Float3, get) — XYZ property of Float4.

## Float4x4 (struct)

Float4x4 struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Float4x4)

### Constructors
- `Float4x4(Fusion.Float4 col1, Fusion.Float4 col2, Fusion.Float4 col3, Fusion.Float4 col4)` — Constructs a new Float4x4.
- `Float4x4(float diagonal)` — Constructs a new Float4x4.

### Methods
#### `bool Equals(Fusion.Float4x4 other)`

Equals method of Float4x4.

**Parameters:**
- `other` (Fusion.Float4x4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4x4.Equals%28Fusion.Float4x4%29)

#### `bool Equals(object other)`

Equals method of Float4x4.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4x4.Equals%28System.Object%29)

#### `int GetHashCode()`

GetHashCode method of Float4x4.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4x4.GetHashCode)

#### `bool Near(Fusion.Float4x4 other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of Float4x4.

**Parameters:**
- `other` (Fusion.Float4x4)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4x4.Near%28Fusion.Float4x4%2CSystem.Int32%29)

#### `string ToString()`

ToString method of Float4x4.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4x4.ToString)

#### `static bool op_Equality(Fusion.Float4x4 a, Fusion.Float4x4 b)`

op_Equality method of Float4x4.

**Parameters:**
- `a` (Fusion.Float4x4)
- `b` (Fusion.Float4x4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4x4.op_Equality%28Fusion.Float4x4%2CFusion.Float4x4%29)

#### `static bool op_Inequality(Fusion.Float4x4 a, Fusion.Float4x4 b)`

op_Inequality method of Float4x4.

**Parameters:**
- `a` (Fusion.Float4x4)
- `b` (Fusion.Float4x4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4x4.op_Inequality%28Fusion.Float4x4%2CFusion.Float4x4%29)

#### `static Fusion.Float4x4 op_Multiply(Fusion.Float4x4 a, Fusion.Float4x4 b)`

op_Multiply method of Float4x4.

**Parameters:**
- `a` (Fusion.Float4x4)
- `b` (Fusion.Float4x4)

**Returns:** `Fusion.Float4x4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Float4x4.op_Multiply%28Fusion.Float4x4%2CFusion.Float4x4%29)

### Properties
- `Column1` (Fusion.Float4, get) — Column1 property of Float4x4.
- `Column2` (Fusion.Float4, get) — Column2 property of Float4x4.
- `Column3` (Fusion.Float4, get) — Column3 property of Float4x4.
- `Column4` (Fusion.Float4, get) — Column4 property of Float4x4.
- `Item` (float, get/set) — Item property of Float4x4.
- `Row1` (Fusion.Float4, get) — Row1 property of Float4x4.
- `Row2` (Fusion.Float4, get) — Row2 property of Float4x4.
- `Row3` (Fusion.Float4, get) — Row3 property of Float4x4.
- `Row4` (Fusion.Float4, get) — Row4 property of Float4x4.

## FlowCoefficient (struct)

FlowCoefficient struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.FlowCoefficient)

### Constructors
- `FlowCoefficient(double kv)` — Constructs a new FlowCoefficient.

### Methods
#### `static Fusion.FlowCoefficient Abs(Fusion.FlowCoefficient flowCoefficient)`

Abs method of FlowCoefficient.

**Parameters:**
- `flowCoefficient` (Fusion.FlowCoefficient)

**Returns:** `Fusion.FlowCoefficient` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.Abs%28Fusion.FlowCoefficient%29)

#### `int CompareTo(Fusion.FlowCoefficient other)`

CompareTo method of FlowCoefficient.

**Parameters:**
- `other` (Fusion.FlowCoefficient)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.CompareTo%28Fusion.FlowCoefficient%29)

#### `int CompareTo(object other)`

CompareTo method of FlowCoefficient.

**Parameters:**
- `other` (object)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.CompareTo%28System.Object%29)

#### `bool Equals(Fusion.FlowCoefficient other)`

Equals method of FlowCoefficient.

**Parameters:**
- `other` (Fusion.FlowCoefficient)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.Equals%28Fusion.FlowCoefficient%29)

#### `bool Equals(object other)`

Equals method of FlowCoefficient.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.Equals%28System.Object%29)

#### `static Fusion.FlowCoefficient From(double flowCoefficient, Fusion.FlowCoefficientUnit flowCoefficientUnit)`

From method of FlowCoefficient.

**Parameters:**
- `flowCoefficient` (double)
- `flowCoefficientUnit` (Fusion.FlowCoefficientUnit)

**Returns:** `Fusion.FlowCoefficient` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.From%28System.Double%2CFusion.FlowCoefficientUnit%29)

#### `int GetHashCode()`

GetHashCode method of FlowCoefficient.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.GetHashCode)

#### `static bool IsInfinity(Fusion.FlowCoefficient flowCoefficient)`

IsInfinity method of FlowCoefficient.

**Parameters:**
- `flowCoefficient` (Fusion.FlowCoefficient)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.IsInfinity%28Fusion.FlowCoefficient%29)

#### `static bool IsNaN(Fusion.FlowCoefficient flowCoefficient)`

IsNaN method of FlowCoefficient.

**Parameters:**
- `flowCoefficient` (Fusion.FlowCoefficient)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.IsNaN%28Fusion.FlowCoefficient%29)

#### `static bool IsNegativeInfinity(Fusion.FlowCoefficient flowCoefficient)`

IsNegativeInfinity method of FlowCoefficient.

**Parameters:**
- `flowCoefficient` (Fusion.FlowCoefficient)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.IsNegativeInfinity%28Fusion.FlowCoefficient%29)

#### `static bool IsPositiveInfinity(Fusion.FlowCoefficient flowCoefficient)`

IsPositiveInfinity method of FlowCoefficient.

**Parameters:**
- `flowCoefficient` (Fusion.FlowCoefficient)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.IsPositiveInfinity%28Fusion.FlowCoefficient%29)

#### `static Fusion.FlowCoefficient Max(Fusion.FlowCoefficient a, Fusion.FlowCoefficient b)`

Max method of FlowCoefficient.

**Parameters:**
- `a` (Fusion.FlowCoefficient)
- `b` (Fusion.FlowCoefficient)

**Returns:** `Fusion.FlowCoefficient` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.Max%28Fusion.FlowCoefficient%2CFusion.FlowCoefficient%29)

#### `static Fusion.FlowCoefficient Min(Fusion.FlowCoefficient a, Fusion.FlowCoefficient b)`

Min method of FlowCoefficient.

**Parameters:**
- `a` (Fusion.FlowCoefficient)
- `b` (Fusion.FlowCoefficient)

**Returns:** `Fusion.FlowCoefficient` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.Min%28Fusion.FlowCoefficient%2CFusion.FlowCoefficient%29)

#### `bool Near(Fusion.FlowCoefficient other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of FlowCoefficient.

**Parameters:**
- `other` (Fusion.FlowCoefficient)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.Near%28Fusion.FlowCoefficient%2CSystem.Int32%29)

#### `static Fusion.FlowCoefficient Round(Fusion.FlowCoefficient flowCoefficient, Fusion.FlowCoefficient precision, Fusion.RoundingMode roundingMode)`

Round method of FlowCoefficient.

**Parameters:**
- `flowCoefficient` (Fusion.FlowCoefficient)
- `precision` (Fusion.FlowCoefficient)
- `roundingMode` (Fusion.RoundingMode)

**Returns:** `Fusion.FlowCoefficient` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.Round%28Fusion.FlowCoefficient%2CFusion.FlowCoefficient%2CFusion.RoundingMode%29)

#### `static int Sign(Fusion.FlowCoefficient flowCoefficient)`

Sign method of FlowCoefficient.

**Parameters:**
- `flowCoefficient` (Fusion.FlowCoefficient)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.Sign%28Fusion.FlowCoefficient%29)

#### `double To(Fusion.FlowCoefficientUnit flowCoefficientUnit)`

To method of FlowCoefficient.

**Parameters:**
- `flowCoefficientUnit` (Fusion.FlowCoefficientUnit)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.To%28Fusion.FlowCoefficientUnit%29)

#### `string ToString()`

ToString method of FlowCoefficient.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of FlowCoefficient.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.FlowCoefficient op_Addition(Fusion.FlowCoefficient a, Fusion.FlowCoefficient b)`

op_Addition method of FlowCoefficient.

**Parameters:**
- `a` (Fusion.FlowCoefficient)
- `b` (Fusion.FlowCoefficient)

**Returns:** `Fusion.FlowCoefficient` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.op_Addition%28Fusion.FlowCoefficient%2CFusion.FlowCoefficient%29)

#### `static Fusion.FlowCoefficient op_Division(Fusion.FlowCoefficient a, double b)`

op_Division method of FlowCoefficient.

**Parameters:**
- `a` (Fusion.FlowCoefficient)
- `b` (double)

**Returns:** `Fusion.FlowCoefficient` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.op_Division%28Fusion.FlowCoefficient%2CSystem.Double%29)

#### `static double op_Division(Fusion.FlowCoefficient a, Fusion.FlowCoefficient b)`

op_Division method of FlowCoefficient.

**Parameters:**
- `a` (Fusion.FlowCoefficient)
- `b` (Fusion.FlowCoefficient)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.op_Division%28Fusion.FlowCoefficient%2CFusion.FlowCoefficient%29)

#### `static bool op_Equality(Fusion.FlowCoefficient a, Fusion.FlowCoefficient b)`

op_Equality method of FlowCoefficient.

**Parameters:**
- `a` (Fusion.FlowCoefficient)
- `b` (Fusion.FlowCoefficient)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.op_Equality%28Fusion.FlowCoefficient%2CFusion.FlowCoefficient%29)

#### `static bool op_GreaterThan(Fusion.FlowCoefficient a, Fusion.FlowCoefficient b)`

op_GreaterThan method of FlowCoefficient.

**Parameters:**
- `a` (Fusion.FlowCoefficient)
- `b` (Fusion.FlowCoefficient)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.op_GreaterThan%28Fusion.FlowCoefficient%2CFusion.FlowCoefficient%29)

#### `static bool op_GreaterThanOrEqual(Fusion.FlowCoefficient a, Fusion.FlowCoefficient b)`

op_GreaterThanOrEqual method of FlowCoefficient.

**Parameters:**
- `a` (Fusion.FlowCoefficient)
- `b` (Fusion.FlowCoefficient)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.op_GreaterThanOrEqual%28Fusion.FlowCoefficient%2CFusion.FlowCoefficient%29)

#### `static bool op_Inequality(Fusion.FlowCoefficient a, Fusion.FlowCoefficient b)`

op_Inequality method of FlowCoefficient.

**Parameters:**
- `a` (Fusion.FlowCoefficient)
- `b` (Fusion.FlowCoefficient)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.op_Inequality%28Fusion.FlowCoefficient%2CFusion.FlowCoefficient%29)

#### `static bool op_LessThan(Fusion.FlowCoefficient a, Fusion.FlowCoefficient b)`

op_LessThan method of FlowCoefficient.

**Parameters:**
- `a` (Fusion.FlowCoefficient)
- `b` (Fusion.FlowCoefficient)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.op_LessThan%28Fusion.FlowCoefficient%2CFusion.FlowCoefficient%29)

#### `static bool op_LessThanOrEqual(Fusion.FlowCoefficient a, Fusion.FlowCoefficient b)`

op_LessThanOrEqual method of FlowCoefficient.

**Parameters:**
- `a` (Fusion.FlowCoefficient)
- `b` (Fusion.FlowCoefficient)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.op_LessThanOrEqual%28Fusion.FlowCoefficient%2CFusion.FlowCoefficient%29)

#### `static Fusion.FlowCoefficient op_Modulus(Fusion.FlowCoefficient a, Fusion.FlowCoefficient b)`

op_Modulus method of FlowCoefficient.

**Parameters:**
- `a` (Fusion.FlowCoefficient)
- `b` (Fusion.FlowCoefficient)

**Returns:** `Fusion.FlowCoefficient` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.op_Modulus%28Fusion.FlowCoefficient%2CFusion.FlowCoefficient%29)

#### `static Fusion.FlowCoefficient op_Multiply(Fusion.FlowCoefficient a, double b)`

op_Multiply method of FlowCoefficient.

**Parameters:**
- `a` (Fusion.FlowCoefficient)
- `b` (double)

**Returns:** `Fusion.FlowCoefficient` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.op_Multiply%28Fusion.FlowCoefficient%2CSystem.Double%29)

#### `static Fusion.FlowCoefficient op_Multiply(double a, Fusion.FlowCoefficient b)`

op_Multiply method of FlowCoefficient.

**Parameters:**
- `a` (double)
- `b` (Fusion.FlowCoefficient)

**Returns:** `Fusion.FlowCoefficient` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.op_Multiply%28System.Double%2CFusion.FlowCoefficient%29)

#### `static Fusion.FlowCoefficient op_Subtraction(Fusion.FlowCoefficient a, Fusion.FlowCoefficient b)`

op_Subtraction method of FlowCoefficient.

**Parameters:**
- `a` (Fusion.FlowCoefficient)
- `b` (Fusion.FlowCoefficient)

**Returns:** `Fusion.FlowCoefficient` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.op_Subtraction%28Fusion.FlowCoefficient%2CFusion.FlowCoefficient%29)

#### `static Fusion.FlowCoefficient op_UnaryNegation(Fusion.FlowCoefficient a)`

op_UnaryNegation method of FlowCoefficient.

**Parameters:**
- `a` (Fusion.FlowCoefficient)

**Returns:** `Fusion.FlowCoefficient` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FlowCoefficient.op_UnaryNegation%28Fusion.FlowCoefficient%29)

### Properties
- `Kv` (double, get) — Kv property of FlowCoefficient.

## FlowCoefficientUnit (enum)

FlowCoefficientUnit enum in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.FlowCoefficientUnit)

### Values
- `Kv` = `0`
- `Cv` = `1`

## Force (struct)

Force struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Force)

### Constructors
- `Force(double newtons)` — Constructs a new Force.

### Methods
#### `static Fusion.Force Abs(Fusion.Force force)`

Abs method of Force.

**Parameters:**
- `force` (Fusion.Force)

**Returns:** `Fusion.Force` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.Abs%28Fusion.Force%29)

#### `int CompareTo(Fusion.Force other)`

CompareTo method of Force.

**Parameters:**
- `other` (Fusion.Force)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.CompareTo%28Fusion.Force%29)

#### `int CompareTo(object other)`

CompareTo method of Force.

**Parameters:**
- `other` (object)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.CompareTo%28System.Object%29)

#### `bool Equals(Fusion.Force other)`

Equals method of Force.

**Parameters:**
- `other` (Fusion.Force)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.Equals%28Fusion.Force%29)

#### `bool Equals(object other)`

Equals method of Force.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.Equals%28System.Object%29)

#### `static Fusion.Force From(double force, Fusion.ForceUnit forceUnit)`

From method of Force.

**Parameters:**
- `force` (double)
- `forceUnit` (Fusion.ForceUnit)

**Returns:** `Fusion.Force` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.From%28System.Double%2CFusion.ForceUnit%29)

#### `int GetHashCode()`

GetHashCode method of Force.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.GetHashCode)

#### `static bool IsInfinity(Fusion.Force force)`

IsInfinity method of Force.

**Parameters:**
- `force` (Fusion.Force)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.IsInfinity%28Fusion.Force%29)

#### `static bool IsNaN(Fusion.Force force)`

IsNaN method of Force.

**Parameters:**
- `force` (Fusion.Force)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.IsNaN%28Fusion.Force%29)

#### `static bool IsNegativeInfinity(Fusion.Force force)`

IsNegativeInfinity method of Force.

**Parameters:**
- `force` (Fusion.Force)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.IsNegativeInfinity%28Fusion.Force%29)

#### `static bool IsPositiveInfinity(Fusion.Force force)`

IsPositiveInfinity method of Force.

**Parameters:**
- `force` (Fusion.Force)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.IsPositiveInfinity%28Fusion.Force%29)

#### `static Fusion.Force Max(Fusion.Force a, Fusion.Force b)`

Max method of Force.

**Parameters:**
- `a` (Fusion.Force)
- `b` (Fusion.Force)

**Returns:** `Fusion.Force` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.Max%28Fusion.Force%2CFusion.Force%29)

#### `static Fusion.Force Min(Fusion.Force a, Fusion.Force b)`

Min method of Force.

**Parameters:**
- `a` (Fusion.Force)
- `b` (Fusion.Force)

**Returns:** `Fusion.Force` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.Min%28Fusion.Force%2CFusion.Force%29)

#### `bool Near(Fusion.Force other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of Force.

**Parameters:**
- `other` (Fusion.Force)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.Near%28Fusion.Force%2CSystem.Int32%29)

#### `static Fusion.Force Round(Fusion.Force force, Fusion.Force precision, Fusion.RoundingMode roundingMode)`

Round method of Force.

**Parameters:**
- `force` (Fusion.Force)
- `precision` (Fusion.Force)
- `roundingMode` (Fusion.RoundingMode)

**Returns:** `Fusion.Force` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.Round%28Fusion.Force%2CFusion.Force%2CFusion.RoundingMode%29)

#### `static int Sign(Fusion.Force force)`

Sign method of Force.

**Parameters:**
- `force` (Fusion.Force)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.Sign%28Fusion.Force%29)

#### `double To(Fusion.ForceUnit forceUnit)`

To method of Force.

**Parameters:**
- `forceUnit` (Fusion.ForceUnit)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.To%28Fusion.ForceUnit%29)

#### `string ToString()`

ToString method of Force.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of Force.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.Force op_Addition(Fusion.Force a, Fusion.Force b)`

op_Addition method of Force.

**Parameters:**
- `a` (Fusion.Force)
- `b` (Fusion.Force)

**Returns:** `Fusion.Force` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.op_Addition%28Fusion.Force%2CFusion.Force%29)

#### `static Fusion.Force op_Division(Fusion.Force a, double b)`

op_Division method of Force.

**Parameters:**
- `a` (Fusion.Force)
- `b` (double)

**Returns:** `Fusion.Force` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.op_Division%28Fusion.Force%2CSystem.Double%29)

#### `static double op_Division(Fusion.Force a, Fusion.Force b)`

op_Division method of Force.

**Parameters:**
- `a` (Fusion.Force)
- `b` (Fusion.Force)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.op_Division%28Fusion.Force%2CFusion.Force%29)

#### `static bool op_Equality(Fusion.Force a, Fusion.Force b)`

op_Equality method of Force.

**Parameters:**
- `a` (Fusion.Force)
- `b` (Fusion.Force)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.op_Equality%28Fusion.Force%2CFusion.Force%29)

#### `static bool op_GreaterThan(Fusion.Force a, Fusion.Force b)`

op_GreaterThan method of Force.

**Parameters:**
- `a` (Fusion.Force)
- `b` (Fusion.Force)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.op_GreaterThan%28Fusion.Force%2CFusion.Force%29)

#### `static bool op_GreaterThanOrEqual(Fusion.Force a, Fusion.Force b)`

op_GreaterThanOrEqual method of Force.

**Parameters:**
- `a` (Fusion.Force)
- `b` (Fusion.Force)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.op_GreaterThanOrEqual%28Fusion.Force%2CFusion.Force%29)

#### `static bool op_Inequality(Fusion.Force a, Fusion.Force b)`

op_Inequality method of Force.

**Parameters:**
- `a` (Fusion.Force)
- `b` (Fusion.Force)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.op_Inequality%28Fusion.Force%2CFusion.Force%29)

#### `static bool op_LessThan(Fusion.Force a, Fusion.Force b)`

op_LessThan method of Force.

**Parameters:**
- `a` (Fusion.Force)
- `b` (Fusion.Force)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.op_LessThan%28Fusion.Force%2CFusion.Force%29)

#### `static bool op_LessThanOrEqual(Fusion.Force a, Fusion.Force b)`

op_LessThanOrEqual method of Force.

**Parameters:**
- `a` (Fusion.Force)
- `b` (Fusion.Force)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.op_LessThanOrEqual%28Fusion.Force%2CFusion.Force%29)

#### `static Fusion.Force op_Modulus(Fusion.Force a, Fusion.Force b)`

op_Modulus method of Force.

**Parameters:**
- `a` (Fusion.Force)
- `b` (Fusion.Force)

**Returns:** `Fusion.Force` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.op_Modulus%28Fusion.Force%2CFusion.Force%29)

#### `static Fusion.Force op_Multiply(Fusion.Force a, double b)`

op_Multiply method of Force.

**Parameters:**
- `a` (Fusion.Force)
- `b` (double)

**Returns:** `Fusion.Force` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.op_Multiply%28Fusion.Force%2CSystem.Double%29)

#### `static Fusion.Force op_Multiply(double a, Fusion.Force b)`

op_Multiply method of Force.

**Parameters:**
- `a` (double)
- `b` (Fusion.Force)

**Returns:** `Fusion.Force` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.op_Multiply%28System.Double%2CFusion.Force%29)

#### `static Fusion.Force op_Subtraction(Fusion.Force a, Fusion.Force b)`

op_Subtraction method of Force.

**Parameters:**
- `a` (Fusion.Force)
- `b` (Fusion.Force)

**Returns:** `Fusion.Force` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.op_Subtraction%28Fusion.Force%2CFusion.Force%29)

#### `static Fusion.Force op_UnaryNegation(Fusion.Force a)`

op_UnaryNegation method of Force.

**Parameters:**
- `a` (Fusion.Force)

**Returns:** `Fusion.Force` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Force.op_UnaryNegation%28Fusion.Force%29)

### Properties
- `Newtons` (double, get) — Newtons property of Force.

## ForceUnit (enum)

ForceUnit enum in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.ForceUnit)

### Values
- `Newton` = `0`
- `PoundForce` = `1`
- `KiloPound` = `2`
- `KilogramForce` = `3`
- `TonneForce` = `4`
- `Kilonewton` = `6`
- `Decanewton` = `7`

## FormattedResourcesAttribute (class)

FormattedResourcesAttribute class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.FormattedResourcesAttribute)

### Constructors
- `FormattedResourcesAttribute(string resourcePath)` — Constructs a new FormattedResourcesAttribute.

## FourCC (struct)

FourCC struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.FourCC)

### Constructors
- `FourCC(char c1, char c2, char c3, char c4)` — Constructs a new FourCC.
- `FourCC(string code)` — Constructs a new FourCC.

### Methods
#### `bool Equals(Fusion.FourCC other)`

Equals method of FourCC.

**Parameters:**
- `other` (Fusion.FourCC)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FourCC.Equals%28Fusion.FourCC%29)

#### `bool Equals(object other)`

Equals method of FourCC.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FourCC.Equals%28System.Object%29)

#### `int GetHashCode()`

GetHashCode method of FourCC.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FourCC.GetHashCode)

#### `string ToString()`

ToString method of FourCC.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FourCC.ToString)

#### `static bool op_Equality(Fusion.FourCC a, Fusion.FourCC b)`

op_Equality method of FourCC.

**Parameters:**
- `a` (Fusion.FourCC)
- `b` (Fusion.FourCC)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FourCC.op_Equality%28Fusion.FourCC%2CFusion.FourCC%29)

#### `static bool op_Inequality(Fusion.FourCC a, Fusion.FourCC b)`

op_Inequality method of FourCC.

**Parameters:**
- `a` (Fusion.FourCC)
- `b` (Fusion.FourCC)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.FourCC.op_Inequality%28Fusion.FourCC%2CFusion.FourCC%29)

## Half (struct)

Half struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Half)

### Constructors
- `Half(float value)` — Constructs a new Half.

### Methods
#### `static Fusion.Half Abs(Fusion.Half value)`

Abs method of Half.

**Parameters:**
- `value` (Fusion.Half)

**Returns:** `Fusion.Half` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.Abs%28Fusion.Half%29)

#### `int CompareTo(Fusion.Half other)`

CompareTo method of Half.

**Parameters:**
- `other` (Fusion.Half)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.CompareTo%28Fusion.Half%29)

#### `int CompareTo(object other)`

CompareTo method of Half.

**Parameters:**
- `other` (object)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.CompareTo%28System.Object%29)

#### `bool Equals(Fusion.Half other)`

Equals method of Half.

**Parameters:**
- `other` (Fusion.Half)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.Equals%28Fusion.Half%29)

#### `bool Equals(object other)`

Equals method of Half.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.Equals%28System.Object%29)

#### `int GetHashCode()`

GetHashCode method of Half.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.GetHashCode)

#### `static bool IsInfinity(Fusion.Half value)`

IsInfinity method of Half.

**Parameters:**
- `value` (Fusion.Half)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.IsInfinity%28Fusion.Half%29)

#### `static bool IsNaN(Fusion.Half value)`

IsNaN method of Half.

**Parameters:**
- `value` (Fusion.Half)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.IsNaN%28Fusion.Half%29)

#### `static bool IsNegativeInfinity(Fusion.Half value)`

IsNegativeInfinity method of Half.

**Parameters:**
- `value` (Fusion.Half)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.IsNegativeInfinity%28Fusion.Half%29)

#### `static bool IsPositiveInfinity(Fusion.Half value)`

IsPositiveInfinity method of Half.

**Parameters:**
- `value` (Fusion.Half)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.IsPositiveInfinity%28Fusion.Half%29)

#### `static Fusion.Half Max(Fusion.Half a, Fusion.Half b)`

Max method of Half.

**Parameters:**
- `a` (Fusion.Half)
- `b` (Fusion.Half)

**Returns:** `Fusion.Half` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.Max%28Fusion.Half%2CFusion.Half%29)

#### `static Fusion.Half Min(Fusion.Half a, Fusion.Half b)`

Min method of Half.

**Parameters:**
- `a` (Fusion.Half)
- `b` (Fusion.Half)

**Returns:** `Fusion.Half` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.Min%28Fusion.Half%2CFusion.Half%29)

#### `bool Near(Fusion.Half other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of Half.

**Parameters:**
- `other` (Fusion.Half)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.Near%28Fusion.Half%2CSystem.Int32%29)

#### `static int Sign(Fusion.Half value)`

Sign method of Half.

**Parameters:**
- `value` (Fusion.Half)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.Sign%28Fusion.Half%29)

#### `string ToString()`

ToString method of Half.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of Half.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.Half op_Addition(Fusion.Half a, Fusion.Half b)`

op_Addition method of Half.

**Parameters:**
- `a` (Fusion.Half)
- `b` (Fusion.Half)

**Returns:** `Fusion.Half` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_Addition%28Fusion.Half%2CFusion.Half%29)

#### `static Fusion.Half op_Decrement(Fusion.Half v)`

op_Decrement method of Half.

**Parameters:**
- `v` (Fusion.Half)

**Returns:** `Fusion.Half` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_Decrement%28Fusion.Half%29)

#### `static Fusion.Half op_Division(Fusion.Half a, Fusion.Half b)`

op_Division method of Half.

**Parameters:**
- `a` (Fusion.Half)
- `b` (Fusion.Half)

**Returns:** `Fusion.Half` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_Division%28Fusion.Half%2CFusion.Half%29)

#### `static bool op_Equality(Fusion.Half a, Fusion.Half b)`

op_Equality method of Half.

**Parameters:**
- `a` (Fusion.Half)
- `b` (Fusion.Half)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_Equality%28Fusion.Half%2CFusion.Half%29)

#### `static Fusion.Half op_Explicit(decimal value)`

op_Explicit method of Half.

**Parameters:**
- `value` (decimal)

**Returns:** `Fusion.Half` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_Explicit%28System.Decimal%29~Fusion.Half)

#### `static Fusion.Half op_Explicit(double value)`

op_Explicit method of Half.

**Parameters:**
- `value` (double)

**Returns:** `Fusion.Half` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_Explicit%28System.Double%29~Fusion.Half)

#### `static Fusion.Half op_Explicit(float value)`

op_Explicit method of Half.

**Parameters:**
- `value` (float)

**Returns:** `Fusion.Half` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_Explicit%28System.Single%29~Fusion.Half)

#### `static byte op_Explicit(Fusion.Half value)`

op_Explicit method of Half.

**Parameters:**
- `value` (Fusion.Half)

**Returns:** `byte` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_Explicit%28Fusion.Half%29~System.Byte)

#### `static char op_Explicit(Fusion.Half value)`

op_Explicit method of Half.

**Parameters:**
- `value` (Fusion.Half)

**Returns:** `char` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_Explicit%28Fusion.Half%29~System.Char)

#### `static int op_Explicit(Fusion.Half value)`

op_Explicit method of Half.

**Parameters:**
- `value` (Fusion.Half)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_Explicit%28Fusion.Half%29~System.Int32)

#### `static long op_Explicit(Fusion.Half value)`

op_Explicit method of Half.

**Parameters:**
- `value` (Fusion.Half)

**Returns:** `long` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_Explicit%28Fusion.Half%29~System.Int64)

#### `static sbyte op_Explicit(Fusion.Half value)`

op_Explicit method of Half.

**Parameters:**
- `value` (Fusion.Half)

**Returns:** `sbyte` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_Explicit%28Fusion.Half%29~System.SByte)

#### `static short op_Explicit(Fusion.Half value)`

op_Explicit method of Half.

**Parameters:**
- `value` (Fusion.Half)

**Returns:** `short` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_Explicit%28Fusion.Half%29~System.Int16)

#### `static uint op_Explicit(Fusion.Half value)`

op_Explicit method of Half.

**Parameters:**
- `value` (Fusion.Half)

**Returns:** `uint` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_Explicit%28Fusion.Half%29~System.UInt32)

#### `static ulong op_Explicit(Fusion.Half value)`

op_Explicit method of Half.

**Parameters:**
- `value` (Fusion.Half)

**Returns:** `ulong` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_Explicit%28Fusion.Half%29~System.UInt64)

#### `static ushort op_Explicit(Fusion.Half value)`

op_Explicit method of Half.

**Parameters:**
- `value` (Fusion.Half)

**Returns:** `ushort` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_Explicit%28Fusion.Half%29~System.UInt16)

#### `static bool op_GreaterThan(Fusion.Half a, Fusion.Half b)`

op_GreaterThan method of Half.

**Parameters:**
- `a` (Fusion.Half)
- `b` (Fusion.Half)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_GreaterThan%28Fusion.Half%2CFusion.Half%29)

#### `static bool op_GreaterThanOrEqual(Fusion.Half a, Fusion.Half b)`

op_GreaterThanOrEqual method of Half.

**Parameters:**
- `a` (Fusion.Half)
- `b` (Fusion.Half)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_GreaterThanOrEqual%28Fusion.Half%2CFusion.Half%29)

#### `static Fusion.Half op_Implicit(byte value)`

op_Implicit method of Half.

**Parameters:**
- `value` (byte)

**Returns:** `Fusion.Half` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_Implicit%28System.Byte%29~Fusion.Half)

#### `static Fusion.Half op_Implicit(int value)`

op_Implicit method of Half.

**Parameters:**
- `value` (int)

**Returns:** `Fusion.Half` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_Implicit%28System.Int32%29~Fusion.Half)

#### `static Fusion.Half op_Implicit(long value)`

op_Implicit method of Half.

**Parameters:**
- `value` (long)

**Returns:** `Fusion.Half` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_Implicit%28System.Int64%29~Fusion.Half)

#### `static Fusion.Half op_Implicit(sbyte value)`

op_Implicit method of Half.

**Parameters:**
- `value` (sbyte)

**Returns:** `Fusion.Half` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_Implicit%28System.SByte%29~Fusion.Half)

#### `static Fusion.Half op_Implicit(short value)`

op_Implicit method of Half.

**Parameters:**
- `value` (short)

**Returns:** `Fusion.Half` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_Implicit%28System.Int16%29~Fusion.Half)

#### `static Fusion.Half op_Implicit(uint value)`

op_Implicit method of Half.

**Parameters:**
- `value` (uint)

**Returns:** `Fusion.Half` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_Implicit%28System.UInt32%29~Fusion.Half)

#### `static Fusion.Half op_Implicit(ulong value)`

op_Implicit method of Half.

**Parameters:**
- `value` (ulong)

**Returns:** `Fusion.Half` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_Implicit%28System.UInt64%29~Fusion.Half)

#### `static Fusion.Half op_Implicit(ushort value)`

op_Implicit method of Half.

**Parameters:**
- `value` (ushort)

**Returns:** `Fusion.Half` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_Implicit%28System.UInt16%29~Fusion.Half)

#### `static decimal op_Implicit(Fusion.Half value)`

op_Implicit method of Half.

**Parameters:**
- `value` (Fusion.Half)

**Returns:** `decimal` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_Implicit%28Fusion.Half%29~System.Decimal)

#### `static double op_Implicit(Fusion.Half value)`

op_Implicit method of Half.

**Parameters:**
- `value` (Fusion.Half)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_Implicit%28Fusion.Half%29~System.Double)

#### `static float op_Implicit(Fusion.Half value)`

op_Implicit method of Half.

**Parameters:**
- `value` (Fusion.Half)

**Returns:** `float` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_Implicit%28Fusion.Half%29~System.Single)

#### `static Fusion.Half op_Increment(Fusion.Half v)`

op_Increment method of Half.

**Parameters:**
- `v` (Fusion.Half)

**Returns:** `Fusion.Half` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_Increment%28Fusion.Half%29)

#### `static bool op_Inequality(Fusion.Half a, Fusion.Half b)`

op_Inequality method of Half.

**Parameters:**
- `a` (Fusion.Half)
- `b` (Fusion.Half)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_Inequality%28Fusion.Half%2CFusion.Half%29)

#### `static bool op_LessThan(Fusion.Half a, Fusion.Half b)`

op_LessThan method of Half.

**Parameters:**
- `a` (Fusion.Half)
- `b` (Fusion.Half)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_LessThan%28Fusion.Half%2CFusion.Half%29)

#### `static bool op_LessThanOrEqual(Fusion.Half a, Fusion.Half b)`

op_LessThanOrEqual method of Half.

**Parameters:**
- `a` (Fusion.Half)
- `b` (Fusion.Half)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_LessThanOrEqual%28Fusion.Half%2CFusion.Half%29)

#### `static Fusion.Half op_Modulus(Fusion.Half a, Fusion.Half b)`

op_Modulus method of Half.

**Parameters:**
- `a` (Fusion.Half)
- `b` (Fusion.Half)

**Returns:** `Fusion.Half` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_Modulus%28Fusion.Half%2CFusion.Half%29)

#### `static Fusion.Half op_Multiply(Fusion.Half a, Fusion.Half b)`

op_Multiply method of Half.

**Parameters:**
- `a` (Fusion.Half)
- `b` (Fusion.Half)

**Returns:** `Fusion.Half` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_Multiply%28Fusion.Half%2CFusion.Half%29)

#### `static Fusion.Half op_Subtraction(Fusion.Half a, Fusion.Half b)`

op_Subtraction method of Half.

**Parameters:**
- `a` (Fusion.Half)
- `b` (Fusion.Half)

**Returns:** `Fusion.Half` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_Subtraction%28Fusion.Half%2CFusion.Half%29)

#### `static Fusion.Half op_UnaryNegation(Fusion.Half v)`

op_UnaryNegation method of Half.

**Parameters:**
- `v` (Fusion.Half)

**Returns:** `Fusion.Half` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_UnaryNegation%28Fusion.Half%29)

#### `static Fusion.Half op_UnaryPlus(Fusion.Half v)`

op_UnaryPlus method of Half.

**Parameters:**
- `v` (Fusion.Half)

**Returns:** `Fusion.Half` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half.op_UnaryPlus%28Fusion.Half%29)

## Half2 (struct)

Half2 struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Half2)

### Constructors
- `Half2(Fusion.Half all)` — Constructs a new Half2.
- `Half2(Fusion.Half x, Fusion.Half y)` — Constructs a new Half2.

### Methods
#### `static Fusion.Half2 Abs(Fusion.Half2 value)`

Abs method of Half2.

**Parameters:**
- `value` (Fusion.Half2)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.Abs%28Fusion.Half2%29)

#### `static Fusion.Half2 Ceiling(Fusion.Half2 value)`

Ceiling method of Half2.

**Parameters:**
- `value` (Fusion.Half2)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.Ceiling%28Fusion.Half2%29)

#### `bool Equals(Fusion.Half2 other)`

Equals method of Half2.

**Parameters:**
- `other` (Fusion.Half2)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.Equals%28Fusion.Half2%29)

#### `bool Equals(object other)`

Equals method of Half2.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.Equals%28System.Object%29)

#### `static Fusion.Half2 Floor(Fusion.Half2 value)`

Floor method of Half2.

**Parameters:**
- `value` (Fusion.Half2)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.Floor%28Fusion.Half2%29)

#### `int GetHashCode()`

GetHashCode method of Half2.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.GetHashCode)

#### `static bool IsInfinity(Fusion.Half2 value)`

IsInfinity method of Half2.

**Parameters:**
- `value` (Fusion.Half2)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.IsInfinity%28Fusion.Half2%29)

#### `static bool IsNaN(Fusion.Half2 value)`

IsNaN method of Half2.

**Parameters:**
- `value` (Fusion.Half2)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.IsNaN%28Fusion.Half2%29)

#### `static bool IsNegativeInfinity(Fusion.Half2 value)`

IsNegativeInfinity method of Half2.

**Parameters:**
- `value` (Fusion.Half2)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.IsNegativeInfinity%28Fusion.Half2%29)

#### `static bool IsPositiveInfinity(Fusion.Half2 value)`

IsPositiveInfinity method of Half2.

**Parameters:**
- `value` (Fusion.Half2)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.IsPositiveInfinity%28Fusion.Half2%29)

#### `static Fusion.Half2 Max(Fusion.Half2 a, Fusion.Half2 b)`

Max method of Half2.

**Parameters:**
- `a` (Fusion.Half2)
- `b` (Fusion.Half2)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.Max%28Fusion.Half2%2CFusion.Half2%29)

#### `static Fusion.Half2 Min(Fusion.Half2 a, Fusion.Half2 b)`

Min method of Half2.

**Parameters:**
- `a` (Fusion.Half2)
- `b` (Fusion.Half2)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.Min%28Fusion.Half2%2CFusion.Half2%29)

#### `bool Near(Fusion.Half2 other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of Half2.

**Parameters:**
- `other` (Fusion.Half2)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.Near%28Fusion.Half2%2CSystem.Int32%29)

#### `static Fusion.Half2 Pow(Fusion.Half a, Fusion.Half2 b)`

Pow method of Half2.

**Parameters:**
- `a` (Fusion.Half)
- `b` (Fusion.Half2)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.Pow%28Fusion.Half%2CFusion.Half2%29)

#### `static Fusion.Half2 Pow(Fusion.Half2 a, Fusion.Half b)`

Pow method of Half2.

**Parameters:**
- `a` (Fusion.Half2)
- `b` (Fusion.Half)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.Pow%28Fusion.Half2%2CFusion.Half%29)

#### `static Fusion.Half2 Pow(Fusion.Half2 a, Fusion.Half2 b)`

Pow method of Half2.

**Parameters:**
- `a` (Fusion.Half2)
- `b` (Fusion.Half2)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.Pow%28Fusion.Half2%2CFusion.Half2%29)

#### `static Fusion.Half2 Round(Fusion.Half2 value)`

Round method of Half2.

**Parameters:**
- `value` (Fusion.Half2)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.Round%28Fusion.Half2%29)

#### `static Fusion.Half2 Round(Fusion.Half2 value, System.MidpointRounding midpointRounding)`

Round method of Half2.

**Parameters:**
- `value` (Fusion.Half2)
- `midpointRounding` (System.MidpointRounding)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.Round%28Fusion.Half2%2CSystem.MidpointRounding%29)

#### `static Fusion.Half2 Round(Fusion.Half2 value, int digits)`

Round method of Half2.

**Parameters:**
- `value` (Fusion.Half2)
- `digits` (int)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.Round%28Fusion.Half2%2CSystem.Int32%29)

#### `static Fusion.Half2 Round(Fusion.Half2 value, int digits, System.MidpointRounding midpointRounding)`

Round method of Half2.

**Parameters:**
- `value` (Fusion.Half2)
- `digits` (int)
- `midpointRounding` (System.MidpointRounding)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.Round%28Fusion.Half2%2CSystem.Int32%2CSystem.MidpointRounding%29)

#### `static Fusion.Int2 Sign(Fusion.Half2 value)`

Sign method of Half2.

**Parameters:**
- `value` (Fusion.Half2)

**Returns:** `Fusion.Int2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.Sign%28Fusion.Half2%29)

#### `static Fusion.Half2 Sqrt(Fusion.Half2 value)`

Sqrt method of Half2.

**Parameters:**
- `value` (Fusion.Half2)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.Sqrt%28Fusion.Half2%29)

#### `string ToString()`

ToString method of Half2.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of Half2.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.Half2 Truncate(Fusion.Half2 value)`

Truncate method of Half2.

**Parameters:**
- `value` (Fusion.Half2)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.Truncate%28Fusion.Half2%29)

#### `static Fusion.Half2 op_Addition(Fusion.Half a, Fusion.Half2 b)`

op_Addition method of Half2.

**Parameters:**
- `a` (Fusion.Half)
- `b` (Fusion.Half2)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.op_Addition%28Fusion.Half%2CFusion.Half2%29)

#### `static Fusion.Half2 op_Addition(Fusion.Half2 a, Fusion.Half b)`

op_Addition method of Half2.

**Parameters:**
- `a` (Fusion.Half2)
- `b` (Fusion.Half)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.op_Addition%28Fusion.Half2%2CFusion.Half%29)

#### `static Fusion.Half2 op_Addition(Fusion.Half2 a, Fusion.Half2 b)`

op_Addition method of Half2.

**Parameters:**
- `a` (Fusion.Half2)
- `b` (Fusion.Half2)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.op_Addition%28Fusion.Half2%2CFusion.Half2%29)

#### `static Fusion.Half2 op_Division(Fusion.Half a, Fusion.Half2 b)`

op_Division method of Half2.

**Parameters:**
- `a` (Fusion.Half)
- `b` (Fusion.Half2)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.op_Division%28Fusion.Half%2CFusion.Half2%29)

#### `static Fusion.Half2 op_Division(Fusion.Half2 a, Fusion.Half b)`

op_Division method of Half2.

**Parameters:**
- `a` (Fusion.Half2)
- `b` (Fusion.Half)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.op_Division%28Fusion.Half2%2CFusion.Half%29)

#### `static Fusion.Half2 op_Division(Fusion.Half2 a, Fusion.Half2 b)`

op_Division method of Half2.

**Parameters:**
- `a` (Fusion.Half2)
- `b` (Fusion.Half2)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.op_Division%28Fusion.Half2%2CFusion.Half2%29)

#### `static bool op_Equality(Fusion.Half2 a, Fusion.Half2 b)`

op_Equality method of Half2.

**Parameters:**
- `a` (Fusion.Half2)
- `b` (Fusion.Half2)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.op_Equality%28Fusion.Half2%2CFusion.Half2%29)

#### `static Fusion.Half2 op_Explicit(Fusion.Double2 v)`

op_Explicit method of Half2.

**Parameters:**
- `v` (Fusion.Double2)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.op_Explicit%28Fusion.Double2%29~Fusion.Half2)

#### `static Fusion.Half2 op_Explicit(Fusion.Float2 v)`

op_Explicit method of Half2.

**Parameters:**
- `v` (Fusion.Float2)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.op_Explicit%28Fusion.Float2%29~Fusion.Half2)

#### `static Fusion.Half2 op_Explicit(Fusion.Int2 v)`

op_Explicit method of Half2.

**Parameters:**
- `v` (Fusion.Int2)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.op_Explicit%28Fusion.Int2%29~Fusion.Half2)

#### `static Fusion.Half2 op_Explicit(Fusion.Short2 v)`

op_Explicit method of Half2.

**Parameters:**
- `v` (Fusion.Short2)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.op_Explicit%28Fusion.Short2%29~Fusion.Half2)

#### `static Fusion.Half2 op_Explicit(Fusion.UShort2 v)`

op_Explicit method of Half2.

**Parameters:**
- `v` (Fusion.UShort2)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.op_Explicit%28Fusion.UShort2%29~Fusion.Half2)

#### `static bool op_Inequality(Fusion.Half2 a, Fusion.Half2 b)`

op_Inequality method of Half2.

**Parameters:**
- `a` (Fusion.Half2)
- `b` (Fusion.Half2)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.op_Inequality%28Fusion.Half2%2CFusion.Half2%29)

#### `static Fusion.Half2 op_Modulus(Fusion.Half a, Fusion.Half2 b)`

op_Modulus method of Half2.

**Parameters:**
- `a` (Fusion.Half)
- `b` (Fusion.Half2)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.op_Modulus%28Fusion.Half%2CFusion.Half2%29)

#### `static Fusion.Half2 op_Modulus(Fusion.Half2 a, Fusion.Half b)`

op_Modulus method of Half2.

**Parameters:**
- `a` (Fusion.Half2)
- `b` (Fusion.Half)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.op_Modulus%28Fusion.Half2%2CFusion.Half%29)

#### `static Fusion.Half2 op_Modulus(Fusion.Half2 a, Fusion.Half2 b)`

op_Modulus method of Half2.

**Parameters:**
- `a` (Fusion.Half2)
- `b` (Fusion.Half2)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.op_Modulus%28Fusion.Half2%2CFusion.Half2%29)

#### `static Fusion.Half2 op_Multiply(Fusion.Half a, Fusion.Half2 b)`

op_Multiply method of Half2.

**Parameters:**
- `a` (Fusion.Half)
- `b` (Fusion.Half2)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.op_Multiply%28Fusion.Half%2CFusion.Half2%29)

#### `static Fusion.Half2 op_Multiply(Fusion.Half2 a, Fusion.Half b)`

op_Multiply method of Half2.

**Parameters:**
- `a` (Fusion.Half2)
- `b` (Fusion.Half)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.op_Multiply%28Fusion.Half2%2CFusion.Half%29)

#### `static Fusion.Half2 op_Multiply(Fusion.Half2 a, Fusion.Half2 b)`

op_Multiply method of Half2.

**Parameters:**
- `a` (Fusion.Half2)
- `b` (Fusion.Half2)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.op_Multiply%28Fusion.Half2%2CFusion.Half2%29)

#### `static Fusion.Half2 op_Subtraction(Fusion.Half a, Fusion.Half2 b)`

op_Subtraction method of Half2.

**Parameters:**
- `a` (Fusion.Half)
- `b` (Fusion.Half2)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.op_Subtraction%28Fusion.Half%2CFusion.Half2%29)

#### `static Fusion.Half2 op_Subtraction(Fusion.Half2 a, Fusion.Half b)`

op_Subtraction method of Half2.

**Parameters:**
- `a` (Fusion.Half2)
- `b` (Fusion.Half)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.op_Subtraction%28Fusion.Half2%2CFusion.Half%29)

#### `static Fusion.Half2 op_Subtraction(Fusion.Half2 a, Fusion.Half2 b)`

op_Subtraction method of Half2.

**Parameters:**
- `a` (Fusion.Half2)
- `b` (Fusion.Half2)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.op_Subtraction%28Fusion.Half2%2CFusion.Half2%29)

#### `static Fusion.Half2 op_UnaryNegation(Fusion.Half2 v)`

op_UnaryNegation method of Half2.

**Parameters:**
- `v` (Fusion.Half2)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.op_UnaryNegation%28Fusion.Half2%29)

#### `static Fusion.Half2 op_UnaryPlus(Fusion.Half2 v)`

op_UnaryPlus method of Half2.

**Parameters:**
- `v` (Fusion.Half2)

**Returns:** `Fusion.Half2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half2.op_UnaryPlus%28Fusion.Half2%29)

### Properties
- `Item` (Fusion.Half, get/set) — Item property of Half2.

## Half3 (struct)

Half3 struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Half3)

### Constructors
- `Half3(Fusion.Half all)` — Constructs a new Half3.
- `Half3(Fusion.Half x, Fusion.Half y, Fusion.Half z)` — Constructs a new Half3.
- `Half3(Fusion.Half x, Fusion.Half2 yz)` — Constructs a new Half3.
- `Half3(Fusion.Half2 xy, Fusion.Half z)` — Constructs a new Half3.

### Methods
#### `static Fusion.Half3 Abs(Fusion.Half3 value)`

Abs method of Half3.

**Parameters:**
- `value` (Fusion.Half3)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.Abs%28Fusion.Half3%29)

#### `static Fusion.Half3 Ceiling(Fusion.Half3 value)`

Ceiling method of Half3.

**Parameters:**
- `value` (Fusion.Half3)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.Ceiling%28Fusion.Half3%29)

#### `bool Equals(Fusion.Half3 other)`

Equals method of Half3.

**Parameters:**
- `other` (Fusion.Half3)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.Equals%28Fusion.Half3%29)

#### `bool Equals(object other)`

Equals method of Half3.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.Equals%28System.Object%29)

#### `static Fusion.Half3 Floor(Fusion.Half3 value)`

Floor method of Half3.

**Parameters:**
- `value` (Fusion.Half3)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.Floor%28Fusion.Half3%29)

#### `int GetHashCode()`

GetHashCode method of Half3.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.GetHashCode)

#### `static bool IsInfinity(Fusion.Half3 value)`

IsInfinity method of Half3.

**Parameters:**
- `value` (Fusion.Half3)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.IsInfinity%28Fusion.Half3%29)

#### `static bool IsNaN(Fusion.Half3 value)`

IsNaN method of Half3.

**Parameters:**
- `value` (Fusion.Half3)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.IsNaN%28Fusion.Half3%29)

#### `static bool IsNegativeInfinity(Fusion.Half3 value)`

IsNegativeInfinity method of Half3.

**Parameters:**
- `value` (Fusion.Half3)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.IsNegativeInfinity%28Fusion.Half3%29)

#### `static bool IsPositiveInfinity(Fusion.Half3 value)`

IsPositiveInfinity method of Half3.

**Parameters:**
- `value` (Fusion.Half3)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.IsPositiveInfinity%28Fusion.Half3%29)

#### `static Fusion.Half3 Max(Fusion.Half3 a, Fusion.Half3 b)`

Max method of Half3.

**Parameters:**
- `a` (Fusion.Half3)
- `b` (Fusion.Half3)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.Max%28Fusion.Half3%2CFusion.Half3%29)

#### `static Fusion.Half3 Min(Fusion.Half3 a, Fusion.Half3 b)`

Min method of Half3.

**Parameters:**
- `a` (Fusion.Half3)
- `b` (Fusion.Half3)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.Min%28Fusion.Half3%2CFusion.Half3%29)

#### `bool Near(Fusion.Half3 other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of Half3.

**Parameters:**
- `other` (Fusion.Half3)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.Near%28Fusion.Half3%2CSystem.Int32%29)

#### `static Fusion.Half3 Pow(Fusion.Half a, Fusion.Half3 b)`

Pow method of Half3.

**Parameters:**
- `a` (Fusion.Half)
- `b` (Fusion.Half3)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.Pow%28Fusion.Half%2CFusion.Half3%29)

#### `static Fusion.Half3 Pow(Fusion.Half3 a, Fusion.Half b)`

Pow method of Half3.

**Parameters:**
- `a` (Fusion.Half3)
- `b` (Fusion.Half)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.Pow%28Fusion.Half3%2CFusion.Half%29)

#### `static Fusion.Half3 Pow(Fusion.Half3 a, Fusion.Half3 b)`

Pow method of Half3.

**Parameters:**
- `a` (Fusion.Half3)
- `b` (Fusion.Half3)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.Pow%28Fusion.Half3%2CFusion.Half3%29)

#### `static Fusion.Half3 Round(Fusion.Half3 value)`

Round method of Half3.

**Parameters:**
- `value` (Fusion.Half3)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.Round%28Fusion.Half3%29)

#### `static Fusion.Half3 Round(Fusion.Half3 value, System.MidpointRounding midpointRounding)`

Round method of Half3.

**Parameters:**
- `value` (Fusion.Half3)
- `midpointRounding` (System.MidpointRounding)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.Round%28Fusion.Half3%2CSystem.MidpointRounding%29)

#### `static Fusion.Half3 Round(Fusion.Half3 value, int digits)`

Round method of Half3.

**Parameters:**
- `value` (Fusion.Half3)
- `digits` (int)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.Round%28Fusion.Half3%2CSystem.Int32%29)

#### `static Fusion.Half3 Round(Fusion.Half3 value, int digits, System.MidpointRounding midpointRounding)`

Round method of Half3.

**Parameters:**
- `value` (Fusion.Half3)
- `digits` (int)
- `midpointRounding` (System.MidpointRounding)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.Round%28Fusion.Half3%2CSystem.Int32%2CSystem.MidpointRounding%29)

#### `static Fusion.Int3 Sign(Fusion.Half3 value)`

Sign method of Half3.

**Parameters:**
- `value` (Fusion.Half3)

**Returns:** `Fusion.Int3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.Sign%28Fusion.Half3%29)

#### `static Fusion.Half3 Sqrt(Fusion.Half3 value)`

Sqrt method of Half3.

**Parameters:**
- `value` (Fusion.Half3)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.Sqrt%28Fusion.Half3%29)

#### `string ToString()`

ToString method of Half3.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of Half3.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.Half3 Truncate(Fusion.Half3 value)`

Truncate method of Half3.

**Parameters:**
- `value` (Fusion.Half3)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.Truncate%28Fusion.Half3%29)

#### `static Fusion.Half3 op_Addition(Fusion.Half a, Fusion.Half3 b)`

op_Addition method of Half3.

**Parameters:**
- `a` (Fusion.Half)
- `b` (Fusion.Half3)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.op_Addition%28Fusion.Half%2CFusion.Half3%29)

#### `static Fusion.Half3 op_Addition(Fusion.Half3 a, Fusion.Half b)`

op_Addition method of Half3.

**Parameters:**
- `a` (Fusion.Half3)
- `b` (Fusion.Half)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.op_Addition%28Fusion.Half3%2CFusion.Half%29)

#### `static Fusion.Half3 op_Addition(Fusion.Half3 a, Fusion.Half3 b)`

op_Addition method of Half3.

**Parameters:**
- `a` (Fusion.Half3)
- `b` (Fusion.Half3)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.op_Addition%28Fusion.Half3%2CFusion.Half3%29)

#### `static Fusion.Half3 op_Division(Fusion.Half a, Fusion.Half3 b)`

op_Division method of Half3.

**Parameters:**
- `a` (Fusion.Half)
- `b` (Fusion.Half3)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.op_Division%28Fusion.Half%2CFusion.Half3%29)

#### `static Fusion.Half3 op_Division(Fusion.Half3 a, Fusion.Half b)`

op_Division method of Half3.

**Parameters:**
- `a` (Fusion.Half3)
- `b` (Fusion.Half)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.op_Division%28Fusion.Half3%2CFusion.Half%29)

#### `static Fusion.Half3 op_Division(Fusion.Half3 a, Fusion.Half3 b)`

op_Division method of Half3.

**Parameters:**
- `a` (Fusion.Half3)
- `b` (Fusion.Half3)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.op_Division%28Fusion.Half3%2CFusion.Half3%29)

#### `static bool op_Equality(Fusion.Half3 a, Fusion.Half3 b)`

op_Equality method of Half3.

**Parameters:**
- `a` (Fusion.Half3)
- `b` (Fusion.Half3)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.op_Equality%28Fusion.Half3%2CFusion.Half3%29)

#### `static Fusion.Half3 op_Explicit(Fusion.Double3 v)`

op_Explicit method of Half3.

**Parameters:**
- `v` (Fusion.Double3)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.op_Explicit%28Fusion.Double3%29~Fusion.Half3)

#### `static Fusion.Half3 op_Explicit(Fusion.Float3 v)`

op_Explicit method of Half3.

**Parameters:**
- `v` (Fusion.Float3)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.op_Explicit%28Fusion.Float3%29~Fusion.Half3)

#### `static Fusion.Half3 op_Explicit(Fusion.Int3 v)`

op_Explicit method of Half3.

**Parameters:**
- `v` (Fusion.Int3)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.op_Explicit%28Fusion.Int3%29~Fusion.Half3)

#### `static Fusion.Half3 op_Explicit(Fusion.Short3 v)`

op_Explicit method of Half3.

**Parameters:**
- `v` (Fusion.Short3)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.op_Explicit%28Fusion.Short3%29~Fusion.Half3)

#### `static Fusion.Half3 op_Explicit(Fusion.UShort3 v)`

op_Explicit method of Half3.

**Parameters:**
- `v` (Fusion.UShort3)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.op_Explicit%28Fusion.UShort3%29~Fusion.Half3)

#### `static bool op_Inequality(Fusion.Half3 a, Fusion.Half3 b)`

op_Inequality method of Half3.

**Parameters:**
- `a` (Fusion.Half3)
- `b` (Fusion.Half3)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.op_Inequality%28Fusion.Half3%2CFusion.Half3%29)

#### `static Fusion.Half3 op_Modulus(Fusion.Half a, Fusion.Half3 b)`

op_Modulus method of Half3.

**Parameters:**
- `a` (Fusion.Half)
- `b` (Fusion.Half3)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.op_Modulus%28Fusion.Half%2CFusion.Half3%29)

#### `static Fusion.Half3 op_Modulus(Fusion.Half3 a, Fusion.Half b)`

op_Modulus method of Half3.

**Parameters:**
- `a` (Fusion.Half3)
- `b` (Fusion.Half)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.op_Modulus%28Fusion.Half3%2CFusion.Half%29)

#### `static Fusion.Half3 op_Modulus(Fusion.Half3 a, Fusion.Half3 b)`

op_Modulus method of Half3.

**Parameters:**
- `a` (Fusion.Half3)
- `b` (Fusion.Half3)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.op_Modulus%28Fusion.Half3%2CFusion.Half3%29)

#### `static Fusion.Half3 op_Multiply(Fusion.Half a, Fusion.Half3 b)`

op_Multiply method of Half3.

**Parameters:**
- `a` (Fusion.Half)
- `b` (Fusion.Half3)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.op_Multiply%28Fusion.Half%2CFusion.Half3%29)

#### `static Fusion.Half3 op_Multiply(Fusion.Half3 a, Fusion.Half b)`

op_Multiply method of Half3.

**Parameters:**
- `a` (Fusion.Half3)
- `b` (Fusion.Half)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.op_Multiply%28Fusion.Half3%2CFusion.Half%29)

#### `static Fusion.Half3 op_Multiply(Fusion.Half3 a, Fusion.Half3 b)`

op_Multiply method of Half3.

**Parameters:**
- `a` (Fusion.Half3)
- `b` (Fusion.Half3)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.op_Multiply%28Fusion.Half3%2CFusion.Half3%29)

#### `static Fusion.Half3 op_Subtraction(Fusion.Half a, Fusion.Half3 b)`

op_Subtraction method of Half3.

**Parameters:**
- `a` (Fusion.Half)
- `b` (Fusion.Half3)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.op_Subtraction%28Fusion.Half%2CFusion.Half3%29)

#### `static Fusion.Half3 op_Subtraction(Fusion.Half3 a, Fusion.Half b)`

op_Subtraction method of Half3.

**Parameters:**
- `a` (Fusion.Half3)
- `b` (Fusion.Half)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.op_Subtraction%28Fusion.Half3%2CFusion.Half%29)

#### `static Fusion.Half3 op_Subtraction(Fusion.Half3 a, Fusion.Half3 b)`

op_Subtraction method of Half3.

**Parameters:**
- `a` (Fusion.Half3)
- `b` (Fusion.Half3)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.op_Subtraction%28Fusion.Half3%2CFusion.Half3%29)

#### `static Fusion.Half3 op_UnaryNegation(Fusion.Half3 v)`

op_UnaryNegation method of Half3.

**Parameters:**
- `v` (Fusion.Half3)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.op_UnaryNegation%28Fusion.Half3%29)

#### `static Fusion.Half3 op_UnaryPlus(Fusion.Half3 v)`

op_UnaryPlus method of Half3.

**Parameters:**
- `v` (Fusion.Half3)

**Returns:** `Fusion.Half3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half3.op_UnaryPlus%28Fusion.Half3%29)

### Properties
- `Item` (Fusion.Half, get/set) — Item property of Half3.
- `XY` (Fusion.Half2, get) — XY property of Half3.

## Half4 (struct)

Half4 struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Half4)

### Constructors
- `Half4(Fusion.Half all)` — Constructs a new Half4.
- `Half4(Fusion.Half x, Fusion.Half y, Fusion.Half z, Fusion.Half w)` — Constructs a new Half4.
- `Half4(Fusion.Half x, Fusion.Half y, Fusion.Half2 zw)` — Constructs a new Half4.
- `Half4(Fusion.Half x, Fusion.Half2 yz, Fusion.Half w)` — Constructs a new Half4.
- `Half4(Fusion.Half x, Fusion.Half3 yzw)` — Constructs a new Half4.
- `Half4(Fusion.Half2 xy, Fusion.Half z, Fusion.Half w)` — Constructs a new Half4.
- `Half4(Fusion.Half2 xy, Fusion.Half2 zw)` — Constructs a new Half4.
- `Half4(Fusion.Half3 xyz, Fusion.Half w)` — Constructs a new Half4.

### Methods
#### `static Fusion.Half4 Abs(Fusion.Half4 value)`

Abs method of Half4.

**Parameters:**
- `value` (Fusion.Half4)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.Abs%28Fusion.Half4%29)

#### `static Fusion.Half4 Ceiling(Fusion.Half4 value)`

Ceiling method of Half4.

**Parameters:**
- `value` (Fusion.Half4)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.Ceiling%28Fusion.Half4%29)

#### `bool Equals(Fusion.Half4 other)`

Equals method of Half4.

**Parameters:**
- `other` (Fusion.Half4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.Equals%28Fusion.Half4%29)

#### `bool Equals(object other)`

Equals method of Half4.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.Equals%28System.Object%29)

#### `static Fusion.Half4 Floor(Fusion.Half4 value)`

Floor method of Half4.

**Parameters:**
- `value` (Fusion.Half4)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.Floor%28Fusion.Half4%29)

#### `int GetHashCode()`

GetHashCode method of Half4.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.GetHashCode)

#### `static bool IsInfinity(Fusion.Half4 value)`

IsInfinity method of Half4.

**Parameters:**
- `value` (Fusion.Half4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.IsInfinity%28Fusion.Half4%29)

#### `static bool IsNaN(Fusion.Half4 value)`

IsNaN method of Half4.

**Parameters:**
- `value` (Fusion.Half4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.IsNaN%28Fusion.Half4%29)

#### `static bool IsNegativeInfinity(Fusion.Half4 value)`

IsNegativeInfinity method of Half4.

**Parameters:**
- `value` (Fusion.Half4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.IsNegativeInfinity%28Fusion.Half4%29)

#### `static bool IsPositiveInfinity(Fusion.Half4 value)`

IsPositiveInfinity method of Half4.

**Parameters:**
- `value` (Fusion.Half4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.IsPositiveInfinity%28Fusion.Half4%29)

#### `static Fusion.Half4 Max(Fusion.Half4 a, Fusion.Half4 b)`

Max method of Half4.

**Parameters:**
- `a` (Fusion.Half4)
- `b` (Fusion.Half4)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.Max%28Fusion.Half4%2CFusion.Half4%29)

#### `static Fusion.Half4 Min(Fusion.Half4 a, Fusion.Half4 b)`

Min method of Half4.

**Parameters:**
- `a` (Fusion.Half4)
- `b` (Fusion.Half4)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.Min%28Fusion.Half4%2CFusion.Half4%29)

#### `bool Near(Fusion.Half4 other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of Half4.

**Parameters:**
- `other` (Fusion.Half4)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.Near%28Fusion.Half4%2CSystem.Int32%29)

#### `static Fusion.Half4 Pow(Fusion.Half a, Fusion.Half4 b)`

Pow method of Half4.

**Parameters:**
- `a` (Fusion.Half)
- `b` (Fusion.Half4)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.Pow%28Fusion.Half%2CFusion.Half4%29)

#### `static Fusion.Half4 Pow(Fusion.Half4 a, Fusion.Half b)`

Pow method of Half4.

**Parameters:**
- `a` (Fusion.Half4)
- `b` (Fusion.Half)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.Pow%28Fusion.Half4%2CFusion.Half%29)

#### `static Fusion.Half4 Pow(Fusion.Half4 a, Fusion.Half4 b)`

Pow method of Half4.

**Parameters:**
- `a` (Fusion.Half4)
- `b` (Fusion.Half4)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.Pow%28Fusion.Half4%2CFusion.Half4%29)

#### `static Fusion.Half4 Round(Fusion.Half4 value)`

Round method of Half4.

**Parameters:**
- `value` (Fusion.Half4)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.Round%28Fusion.Half4%29)

#### `static Fusion.Half4 Round(Fusion.Half4 value, System.MidpointRounding midpointRounding)`

Round method of Half4.

**Parameters:**
- `value` (Fusion.Half4)
- `midpointRounding` (System.MidpointRounding)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.Round%28Fusion.Half4%2CSystem.MidpointRounding%29)

#### `static Fusion.Half4 Round(Fusion.Half4 value, int digits)`

Round method of Half4.

**Parameters:**
- `value` (Fusion.Half4)
- `digits` (int)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.Round%28Fusion.Half4%2CSystem.Int32%29)

#### `static Fusion.Half4 Round(Fusion.Half4 value, int digits, System.MidpointRounding midpointRounding)`

Round method of Half4.

**Parameters:**
- `value` (Fusion.Half4)
- `digits` (int)
- `midpointRounding` (System.MidpointRounding)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.Round%28Fusion.Half4%2CSystem.Int32%2CSystem.MidpointRounding%29)

#### `static Fusion.Int4 Sign(Fusion.Half4 value)`

Sign method of Half4.

**Parameters:**
- `value` (Fusion.Half4)

**Returns:** `Fusion.Int4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.Sign%28Fusion.Half4%29)

#### `static Fusion.Half4 Sqrt(Fusion.Half4 value)`

Sqrt method of Half4.

**Parameters:**
- `value` (Fusion.Half4)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.Sqrt%28Fusion.Half4%29)

#### `string ToString()`

ToString method of Half4.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of Half4.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.Half4 Truncate(Fusion.Half4 value)`

Truncate method of Half4.

**Parameters:**
- `value` (Fusion.Half4)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.Truncate%28Fusion.Half4%29)

#### `static Fusion.Half4 op_Addition(Fusion.Half a, Fusion.Half4 b)`

op_Addition method of Half4.

**Parameters:**
- `a` (Fusion.Half)
- `b` (Fusion.Half4)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.op_Addition%28Fusion.Half%2CFusion.Half4%29)

#### `static Fusion.Half4 op_Addition(Fusion.Half4 a, Fusion.Half b)`

op_Addition method of Half4.

**Parameters:**
- `a` (Fusion.Half4)
- `b` (Fusion.Half)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.op_Addition%28Fusion.Half4%2CFusion.Half%29)

#### `static Fusion.Half4 op_Addition(Fusion.Half4 a, Fusion.Half4 b)`

op_Addition method of Half4.

**Parameters:**
- `a` (Fusion.Half4)
- `b` (Fusion.Half4)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.op_Addition%28Fusion.Half4%2CFusion.Half4%29)

#### `static Fusion.Half4 op_Division(Fusion.Half a, Fusion.Half4 b)`

op_Division method of Half4.

**Parameters:**
- `a` (Fusion.Half)
- `b` (Fusion.Half4)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.op_Division%28Fusion.Half%2CFusion.Half4%29)

#### `static Fusion.Half4 op_Division(Fusion.Half4 a, Fusion.Half b)`

op_Division method of Half4.

**Parameters:**
- `a` (Fusion.Half4)
- `b` (Fusion.Half)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.op_Division%28Fusion.Half4%2CFusion.Half%29)

#### `static Fusion.Half4 op_Division(Fusion.Half4 a, Fusion.Half4 b)`

op_Division method of Half4.

**Parameters:**
- `a` (Fusion.Half4)
- `b` (Fusion.Half4)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.op_Division%28Fusion.Half4%2CFusion.Half4%29)

#### `static bool op_Equality(Fusion.Half4 a, Fusion.Half4 b)`

op_Equality method of Half4.

**Parameters:**
- `a` (Fusion.Half4)
- `b` (Fusion.Half4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.op_Equality%28Fusion.Half4%2CFusion.Half4%29)

#### `static Fusion.Half4 op_Explicit(Fusion.Double4 v)`

op_Explicit method of Half4.

**Parameters:**
- `v` (Fusion.Double4)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.op_Explicit%28Fusion.Double4%29~Fusion.Half4)

#### `static Fusion.Half4 op_Explicit(Fusion.Float4 v)`

op_Explicit method of Half4.

**Parameters:**
- `v` (Fusion.Float4)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.op_Explicit%28Fusion.Float4%29~Fusion.Half4)

#### `static Fusion.Half4 op_Explicit(Fusion.Int4 v)`

op_Explicit method of Half4.

**Parameters:**
- `v` (Fusion.Int4)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.op_Explicit%28Fusion.Int4%29~Fusion.Half4)

#### `static Fusion.Half4 op_Explicit(Fusion.Short4 v)`

op_Explicit method of Half4.

**Parameters:**
- `v` (Fusion.Short4)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.op_Explicit%28Fusion.Short4%29~Fusion.Half4)

#### `static Fusion.Half4 op_Explicit(Fusion.UShort4 v)`

op_Explicit method of Half4.

**Parameters:**
- `v` (Fusion.UShort4)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.op_Explicit%28Fusion.UShort4%29~Fusion.Half4)

#### `static Fusion.Half4 op_Implicit(Fusion.RGBA v)`

op_Implicit method of Half4.

**Parameters:**
- `v` (Fusion.RGBA)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.op_Implicit%28Fusion.RGBA%29~Fusion.Half4)

#### `static bool op_Inequality(Fusion.Half4 a, Fusion.Half4 b)`

op_Inequality method of Half4.

**Parameters:**
- `a` (Fusion.Half4)
- `b` (Fusion.Half4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.op_Inequality%28Fusion.Half4%2CFusion.Half4%29)

#### `static Fusion.Half4 op_Modulus(Fusion.Half a, Fusion.Half4 b)`

op_Modulus method of Half4.

**Parameters:**
- `a` (Fusion.Half)
- `b` (Fusion.Half4)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.op_Modulus%28Fusion.Half%2CFusion.Half4%29)

#### `static Fusion.Half4 op_Modulus(Fusion.Half4 a, Fusion.Half b)`

op_Modulus method of Half4.

**Parameters:**
- `a` (Fusion.Half4)
- `b` (Fusion.Half)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.op_Modulus%28Fusion.Half4%2CFusion.Half%29)

#### `static Fusion.Half4 op_Modulus(Fusion.Half4 a, Fusion.Half4 b)`

op_Modulus method of Half4.

**Parameters:**
- `a` (Fusion.Half4)
- `b` (Fusion.Half4)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.op_Modulus%28Fusion.Half4%2CFusion.Half4%29)

#### `static Fusion.Half4 op_Multiply(Fusion.Half a, Fusion.Half4 b)`

op_Multiply method of Half4.

**Parameters:**
- `a` (Fusion.Half)
- `b` (Fusion.Half4)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.op_Multiply%28Fusion.Half%2CFusion.Half4%29)

#### `static Fusion.Half4 op_Multiply(Fusion.Half4 a, Fusion.Half b)`

op_Multiply method of Half4.

**Parameters:**
- `a` (Fusion.Half4)
- `b` (Fusion.Half)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.op_Multiply%28Fusion.Half4%2CFusion.Half%29)

#### `static Fusion.Half4 op_Multiply(Fusion.Half4 a, Fusion.Half4 b)`

op_Multiply method of Half4.

**Parameters:**
- `a` (Fusion.Half4)
- `b` (Fusion.Half4)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.op_Multiply%28Fusion.Half4%2CFusion.Half4%29)

#### `static Fusion.Half4 op_Subtraction(Fusion.Half a, Fusion.Half4 b)`

op_Subtraction method of Half4.

**Parameters:**
- `a` (Fusion.Half)
- `b` (Fusion.Half4)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.op_Subtraction%28Fusion.Half%2CFusion.Half4%29)

#### `static Fusion.Half4 op_Subtraction(Fusion.Half4 a, Fusion.Half b)`

op_Subtraction method of Half4.

**Parameters:**
- `a` (Fusion.Half4)
- `b` (Fusion.Half)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.op_Subtraction%28Fusion.Half4%2CFusion.Half%29)

#### `static Fusion.Half4 op_Subtraction(Fusion.Half4 a, Fusion.Half4 b)`

op_Subtraction method of Half4.

**Parameters:**
- `a` (Fusion.Half4)
- `b` (Fusion.Half4)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.op_Subtraction%28Fusion.Half4%2CFusion.Half4%29)

#### `static Fusion.Half4 op_UnaryNegation(Fusion.Half4 v)`

op_UnaryNegation method of Half4.

**Parameters:**
- `v` (Fusion.Half4)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.op_UnaryNegation%28Fusion.Half4%29)

#### `static Fusion.Half4 op_UnaryPlus(Fusion.Half4 v)`

op_UnaryPlus method of Half4.

**Parameters:**
- `v` (Fusion.Half4)

**Returns:** `Fusion.Half4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Half4.op_UnaryPlus%28Fusion.Half4%29)

### Properties
- `Item` (Fusion.Half, get/set) — Item property of Half4.
- `XY` (Fusion.Half2, get) — XY property of Half4.
- `XYZ` (Fusion.Half3, get) — XYZ property of Half4.

## HashCode (static-class)

HashCode static class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.HashCode)

### Methods
#### `static int GetHashCode(int hash, int[] hashes)`

GetHashCode method of HashCode.

**Parameters:**
- `hash` (int)
- `hashes` (int[])

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HashCode.GetHashCode%28System.Int32%2CSystem.Int32%5B%5D%29)

#### `static int GetHashCode(int hash1, int hash2)`

GetHashCode method of HashCode.

**Parameters:**
- `hash1` (int)
- `hash2` (int)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HashCode.GetHashCode%28System.Int32%2CSystem.Int32%29)

#### `static int GetHashCode(int hash1, int hash2, int hash3)`

GetHashCode method of HashCode.

**Parameters:**
- `hash1` (int)
- `hash2` (int)
- `hash3` (int)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HashCode.GetHashCode%28System.Int32%2CSystem.Int32%2CSystem.Int32%29)

#### `static int GetHashCode(int hash1, int hash2, int hash3, int hash4)`

GetHashCode method of HashCode.

**Parameters:**
- `hash1` (int)
- `hash2` (int)
- `hash3` (int)
- `hash4` (int)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HashCode.GetHashCode%28System.Int32%2CSystem.Int32%2CSystem.Int32%2CSystem.Int32%29)

#### `static int GetHashCode(object instance)`

GetHashCode method of HashCode.

**Parameters:**
- `instance` (object)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HashCode.GetHashCode%28System.Object%29)

## HeatCapacity (struct)

HeatCapacity struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.HeatCapacity)

### Constructors
- `HeatCapacity(double joulesPerKelvinKilogram)` — Constructs a new HeatCapacity.

### Methods
#### `static Fusion.HeatCapacity Abs(Fusion.HeatCapacity heatCapacity)`

Abs method of HeatCapacity.

**Parameters:**
- `heatCapacity` (Fusion.HeatCapacity)

**Returns:** `Fusion.HeatCapacity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.Abs%28Fusion.HeatCapacity%29)

#### `int CompareTo(Fusion.HeatCapacity other)`

CompareTo method of HeatCapacity.

**Parameters:**
- `other` (Fusion.HeatCapacity)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.CompareTo%28Fusion.HeatCapacity%29)

#### `int CompareTo(object other)`

CompareTo method of HeatCapacity.

**Parameters:**
- `other` (object)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.CompareTo%28System.Object%29)

#### `bool Equals(Fusion.HeatCapacity other)`

Equals method of HeatCapacity.

**Parameters:**
- `other` (Fusion.HeatCapacity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.Equals%28Fusion.HeatCapacity%29)

#### `bool Equals(object other)`

Equals method of HeatCapacity.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.Equals%28System.Object%29)

#### `static Fusion.HeatCapacity From(double heatCapacity, Fusion.HeatCapacityUnit heatCapacityUnit)`

From method of HeatCapacity.

**Parameters:**
- `heatCapacity` (double)
- `heatCapacityUnit` (Fusion.HeatCapacityUnit)

**Returns:** `Fusion.HeatCapacity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.From%28System.Double%2CFusion.HeatCapacityUnit%29)

#### `int GetHashCode()`

GetHashCode method of HeatCapacity.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.GetHashCode)

#### `static bool IsInfinity(Fusion.HeatCapacity heatCapacity)`

IsInfinity method of HeatCapacity.

**Parameters:**
- `heatCapacity` (Fusion.HeatCapacity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.IsInfinity%28Fusion.HeatCapacity%29)

#### `static bool IsNaN(Fusion.HeatCapacity heatCapacity)`

IsNaN method of HeatCapacity.

**Parameters:**
- `heatCapacity` (Fusion.HeatCapacity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.IsNaN%28Fusion.HeatCapacity%29)

#### `static bool IsNegativeInfinity(Fusion.HeatCapacity heatCapacity)`

IsNegativeInfinity method of HeatCapacity.

**Parameters:**
- `heatCapacity` (Fusion.HeatCapacity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.IsNegativeInfinity%28Fusion.HeatCapacity%29)

#### `static bool IsPositiveInfinity(Fusion.HeatCapacity heatCapacity)`

IsPositiveInfinity method of HeatCapacity.

**Parameters:**
- `heatCapacity` (Fusion.HeatCapacity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.IsPositiveInfinity%28Fusion.HeatCapacity%29)

#### `static Fusion.HeatCapacity Max(Fusion.HeatCapacity a, Fusion.HeatCapacity b)`

Max method of HeatCapacity.

**Parameters:**
- `a` (Fusion.HeatCapacity)
- `b` (Fusion.HeatCapacity)

**Returns:** `Fusion.HeatCapacity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.Max%28Fusion.HeatCapacity%2CFusion.HeatCapacity%29)

#### `static Fusion.HeatCapacity Min(Fusion.HeatCapacity a, Fusion.HeatCapacity b)`

Min method of HeatCapacity.

**Parameters:**
- `a` (Fusion.HeatCapacity)
- `b` (Fusion.HeatCapacity)

**Returns:** `Fusion.HeatCapacity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.Min%28Fusion.HeatCapacity%2CFusion.HeatCapacity%29)

#### `bool Near(Fusion.HeatCapacity other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of HeatCapacity.

**Parameters:**
- `other` (Fusion.HeatCapacity)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.Near%28Fusion.HeatCapacity%2CSystem.Int32%29)

#### `static Fusion.HeatCapacity Round(Fusion.HeatCapacity heatCapacity, Fusion.HeatCapacity precision, Fusion.RoundingMode roundingMode)`

Round method of HeatCapacity.

**Parameters:**
- `heatCapacity` (Fusion.HeatCapacity)
- `precision` (Fusion.HeatCapacity)
- `roundingMode` (Fusion.RoundingMode)

**Returns:** `Fusion.HeatCapacity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.Round%28Fusion.HeatCapacity%2CFusion.HeatCapacity%2CFusion.RoundingMode%29)

#### `static int Sign(Fusion.HeatCapacity heatCapacity)`

Sign method of HeatCapacity.

**Parameters:**
- `heatCapacity` (Fusion.HeatCapacity)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.Sign%28Fusion.HeatCapacity%29)

#### `double To(Fusion.HeatCapacityUnit heatCapacityUnit)`

To method of HeatCapacity.

**Parameters:**
- `heatCapacityUnit` (Fusion.HeatCapacityUnit)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.To%28Fusion.HeatCapacityUnit%29)

#### `string ToString()`

ToString method of HeatCapacity.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of HeatCapacity.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.HeatCapacity op_Addition(Fusion.HeatCapacity a, Fusion.HeatCapacity b)`

op_Addition method of HeatCapacity.

**Parameters:**
- `a` (Fusion.HeatCapacity)
- `b` (Fusion.HeatCapacity)

**Returns:** `Fusion.HeatCapacity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.op_Addition%28Fusion.HeatCapacity%2CFusion.HeatCapacity%29)

#### `static Fusion.HeatCapacity op_Division(Fusion.HeatCapacity a, double b)`

op_Division method of HeatCapacity.

**Parameters:**
- `a` (Fusion.HeatCapacity)
- `b` (double)

**Returns:** `Fusion.HeatCapacity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.op_Division%28Fusion.HeatCapacity%2CSystem.Double%29)

#### `static double op_Division(Fusion.HeatCapacity a, Fusion.HeatCapacity b)`

op_Division method of HeatCapacity.

**Parameters:**
- `a` (Fusion.HeatCapacity)
- `b` (Fusion.HeatCapacity)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.op_Division%28Fusion.HeatCapacity%2CFusion.HeatCapacity%29)

#### `static bool op_Equality(Fusion.HeatCapacity a, Fusion.HeatCapacity b)`

op_Equality method of HeatCapacity.

**Parameters:**
- `a` (Fusion.HeatCapacity)
- `b` (Fusion.HeatCapacity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.op_Equality%28Fusion.HeatCapacity%2CFusion.HeatCapacity%29)

#### `static bool op_GreaterThan(Fusion.HeatCapacity a, Fusion.HeatCapacity b)`

op_GreaterThan method of HeatCapacity.

**Parameters:**
- `a` (Fusion.HeatCapacity)
- `b` (Fusion.HeatCapacity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.op_GreaterThan%28Fusion.HeatCapacity%2CFusion.HeatCapacity%29)

#### `static bool op_GreaterThanOrEqual(Fusion.HeatCapacity a, Fusion.HeatCapacity b)`

op_GreaterThanOrEqual method of HeatCapacity.

**Parameters:**
- `a` (Fusion.HeatCapacity)
- `b` (Fusion.HeatCapacity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.op_GreaterThanOrEqual%28Fusion.HeatCapacity%2CFusion.HeatCapacity%29)

#### `static bool op_Inequality(Fusion.HeatCapacity a, Fusion.HeatCapacity b)`

op_Inequality method of HeatCapacity.

**Parameters:**
- `a` (Fusion.HeatCapacity)
- `b` (Fusion.HeatCapacity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.op_Inequality%28Fusion.HeatCapacity%2CFusion.HeatCapacity%29)

#### `static bool op_LessThan(Fusion.HeatCapacity a, Fusion.HeatCapacity b)`

op_LessThan method of HeatCapacity.

**Parameters:**
- `a` (Fusion.HeatCapacity)
- `b` (Fusion.HeatCapacity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.op_LessThan%28Fusion.HeatCapacity%2CFusion.HeatCapacity%29)

#### `static bool op_LessThanOrEqual(Fusion.HeatCapacity a, Fusion.HeatCapacity b)`

op_LessThanOrEqual method of HeatCapacity.

**Parameters:**
- `a` (Fusion.HeatCapacity)
- `b` (Fusion.HeatCapacity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.op_LessThanOrEqual%28Fusion.HeatCapacity%2CFusion.HeatCapacity%29)

#### `static Fusion.HeatCapacity op_Modulus(Fusion.HeatCapacity a, Fusion.HeatCapacity b)`

op_Modulus method of HeatCapacity.

**Parameters:**
- `a` (Fusion.HeatCapacity)
- `b` (Fusion.HeatCapacity)

**Returns:** `Fusion.HeatCapacity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.op_Modulus%28Fusion.HeatCapacity%2CFusion.HeatCapacity%29)

#### `static Fusion.HeatCapacity op_Multiply(Fusion.HeatCapacity a, double b)`

op_Multiply method of HeatCapacity.

**Parameters:**
- `a` (Fusion.HeatCapacity)
- `b` (double)

**Returns:** `Fusion.HeatCapacity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.op_Multiply%28Fusion.HeatCapacity%2CSystem.Double%29)

#### `static Fusion.HeatCapacity op_Multiply(double a, Fusion.HeatCapacity b)`

op_Multiply method of HeatCapacity.

**Parameters:**
- `a` (double)
- `b` (Fusion.HeatCapacity)

**Returns:** `Fusion.HeatCapacity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.op_Multiply%28System.Double%2CFusion.HeatCapacity%29)

#### `static Fusion.HeatCapacity op_Subtraction(Fusion.HeatCapacity a, Fusion.HeatCapacity b)`

op_Subtraction method of HeatCapacity.

**Parameters:**
- `a` (Fusion.HeatCapacity)
- `b` (Fusion.HeatCapacity)

**Returns:** `Fusion.HeatCapacity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.op_Subtraction%28Fusion.HeatCapacity%2CFusion.HeatCapacity%29)

#### `static Fusion.HeatCapacity op_UnaryNegation(Fusion.HeatCapacity a)`

op_UnaryNegation method of HeatCapacity.

**Parameters:**
- `a` (Fusion.HeatCapacity)

**Returns:** `Fusion.HeatCapacity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.HeatCapacity.op_UnaryNegation%28Fusion.HeatCapacity%29)

### Properties
- `JoulesPerKelvinKilogram` (double, get) — JoulesPerKelvinKilogram property of HeatCapacity.

## HeatCapacityUnit (enum)

HeatCapacityUnit enum in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.HeatCapacityUnit)

### Values
- `JoulesPerKelvinKilogram` = `0`
- `BtusPerFahrenheitPound` = `1`
- `KilojoulesPerKelvinKilogram` = `2`

## ICommandObserver (interface)

ICommandObserver interface in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.ICommandObserver)

### Methods
#### `void Executed(Fusion.ViewModel viewModel, string commandName, object parameter)`

Executed method of ICommandObserver.

**Parameters:**
- `viewModel` (Fusion.ViewModel)
- `commandName` (string)
- `parameter` (object)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.ICommandObserver.Executed%28Fusion.ViewModel%2CSystem.String%2CSystem.Object%29)

#### `void Executing(Fusion.ViewModel viewModel, string commandName, object parameter)`

Executing method of ICommandObserver.

**Parameters:**
- `viewModel` (Fusion.ViewModel)
- `commandName` (string)
- `parameter` (object)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.ICommandObserver.Executing%28Fusion.ViewModel%2CSystem.String%2CSystem.Object%29)

#### `void HandlerMissing(Fusion.ViewModel viewModel, string commandName)`

HandlerMissing method of ICommandObserver.

**Parameters:**
- `viewModel` (Fusion.ViewModel)
- `commandName` (string)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.ICommandObserver.HandlerMissing%28Fusion.ViewModel%2CSystem.String%29)

#### `void UnhandledException(Fusion.ViewModel viewModel, string commandName, System.Exception exception)`

UnhandledException method of ICommandObserver.

**Parameters:**
- `viewModel` (Fusion.ViewModel)
- `commandName` (string)
- `exception` (System.Exception)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.ICommandObserver.UnhandledException%28Fusion.ViewModel%2CSystem.String%2CSystem.Exception%29)

## IHelpProvider (interface)

IHelpProvider interface in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.IHelpProvider)

### Methods
#### `void RequestHelp(string topic)`

RequestHelp method of IHelpProvider.

**Parameters:**
- `topic` (string)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHelpProvider.RequestHelp%28System.String%29)

## IHost (interface)

IHost interface in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.IHost)

### Methods
#### `System.Threading.Tasks.Task Invoke<TPublishedMethodDelegate>(System.Action<TPublishedMethodDelegate> invokeMethod)`

Invoke method of IHost.

**Parameters:**
- `invokeMethod` (System.Action<TPublishedMethodDelegate>)

**Returns:** `System.Threading.Tasks.Task` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHost.Invoke%60%601%28System.Action%7B%60%600%7D%29)

#### `System.Threading.Tasks.Task Invoke<TPublishedMethodDelegate>(System.Func<TPublishedMethodDelegate, System.Threading.Tasks.Task> invokeMethod)`

Invoke method of IHost.

**Parameters:**
- `invokeMethod` (System.Func<TPublishedMethodDelegate, System.Threading.Tasks.Task>)

**Returns:** `System.Threading.Tasks.Task` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHost.Invoke%60%601%28System.Func%7B%60%600%2CSystem.Threading.Tasks.Task%7D%29)

#### `System.Threading.Tasks.Task<TResult> Invoke<TPublishedMethodDelegate, TResult>(System.Func<TPublishedMethodDelegate, System.Threading.Tasks.Task<TResult>> invokeMethod)`

Invoke method of IHost.

**Parameters:**
- `invokeMethod` (System.Func<TPublishedMethodDelegate, System.Threading.Tasks.Task<TResult>>)

**Returns:** `System.Threading.Tasks.Task<TResult>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHost.Invoke%60%602%28System.Func%7B%60%600%2CSystem.Threading.Tasks.Task%7B%60%601%7D%7D%29)

#### `System.Threading.Tasks.Task<TResult> Invoke<TPublishedMethodDelegate, TResult>(System.Func<TPublishedMethodDelegate, TResult> invokeMethod)`

Invoke method of IHost.

**Parameters:**
- `invokeMethod` (System.Func<TPublishedMethodDelegate, TResult>)

**Returns:** `System.Threading.Tasks.Task<TResult>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHost.Invoke%60%602%28System.Func%7B%60%600%2C%60%601%7D%29)

#### `System.Threading.Tasks.Task InvokeDomainLogic(System.Action action)`

InvokeDomainLogic method of IHost.

**Parameters:**
- `action` (System.Action)

**Returns:** `System.Threading.Tasks.Task` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHost.InvokeDomainLogic%28System.Action%29)

#### `System.Threading.Tasks.Task<T> InvokeDomainLogic<T>(System.Func<T> action)`

InvokeDomainLogic method of IHost.

**Parameters:**
- `action` (System.Func<T>)

**Returns:** `System.Threading.Tasks.Task<T>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHost.InvokeDomainLogic%60%601%28System.Func%7B%60%600%7D%29)

#### `void Restart()`

Restart method of IHost.

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHost.Restart)

#### `void Shutdown()`

Shutdown method of IHost.

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHost.Shutdown)

#### `void StartExternalProgram(string path, string[] extraCommandLineArguments)`

StartExternalProgram method of IHost.

**Parameters:**
- `path` (string)
- `extraCommandLineArguments` (string[])

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHost.StartExternalProgram%28System.String%2CSystem.String%5B%5D%29)

#### `void StartExternalService(Fusion.RemoteObject service)`

StartExternalService method of IHost.

**Parameters:**
- `service` (Fusion.RemoteObject)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHost.StartExternalService%28Fusion.RemoteObject%29)

### Properties
- `DataFolder` (string, get) — DataFolder property of IHost.
- `Diagnostics` (Fusion.IHostDiagnostics, get) — Diagnostics property of IHost.
- `Recovery` (Fusion.IRecovery, get) — Recovery property of IHost.
- `Session` (Fusion.ISession, get) — Session property of IHost.
- `Settings` (Fusion.ISettings, get) — Settings property of IHost.
- `TempFolder` (string, get) — TempFolder property of IHost.
- `UI` (Fusion.IHostUI, get) — UI property of IHost.
- `User` (Fusion.IHostUser, get) — User property of IHost.

## IHostDiagnostics (interface)

IHostDiagnostics interface in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.IHostDiagnostics)

### Methods
#### `void Error(string format, object[] args)`

Error method of IHostDiagnostics.

**Parameters:**
- `format` (string)
- `args` (object[])

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHostDiagnostics.Error%28System.String%2CSystem.Object%5B%5D%29)

#### `void Information(string format, object[] args)`

Information method of IHostDiagnostics.

**Parameters:**
- `format` (string)
- `args` (object[])

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHostDiagnostics.Information%28System.String%2CSystem.Object%5B%5D%29)

#### `System.IDisposable MeasurePerformance(string format, object[] args)`

MeasurePerformance method of IHostDiagnostics.

**Parameters:**
- `format` (string)
- `args` (object[])

**Returns:** `System.IDisposable` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHostDiagnostics.MeasurePerformance%28System.String%2CSystem.Object%5B%5D%29)

#### `void Warning(string format, object[] args)`

Warning method of IHostDiagnostics.

**Parameters:**
- `format` (string)
- `args` (object[])

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHostDiagnostics.Warning%28System.String%2CSystem.Object%5B%5D%29)

## IHostUI (interface)

IHostUI interface in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.IHostUI)

### Methods
#### `string GetLocalizedText(string resourceKey)`

GetLocalizedText method of IHostUI.

**Parameters:**
- `resourceKey` (string)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHostUI.GetLocalizedText%28System.String%29)

#### `object GetResource(object resourceKey)`

GetResource method of IHostUI.

**Parameters:**
- `resourceKey` (object)

**Returns:** `object` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHostUI.GetResource%28System.Object%29)

#### `void OpenBackgroundMessageDialog(string text, string caption = null, string icon = null, string button = null)`

OpenBackgroundMessageDialog method of IHostUI.

**Parameters:**
- `text` (string)
- `caption` (string)
- `icon` (string)
- `button` (string)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHostUI.OpenBackgroundMessageDialog%28System.String%2CSystem.String%2CSystem.String%2CSystem.String%29)

#### `void RegisterCommandObserver(Fusion.ICommandObserver commandObserver)`

RegisterCommandObserver method of IHostUI.

**Parameters:**
- `commandObserver` (Fusion.ICommandObserver)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHostUI.RegisterCommandObserver%28Fusion.ICommandObserver%29)

#### `void RegisterKeyboardShortcut(string gesture, System.Action action)`

RegisterKeyboardShortcut method of IHostUI.

**Parameters:**
- `gesture` (string)
- `action` (System.Action)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHostUI.RegisterKeyboardShortcut%28System.String%2CSystem.Action%29)

#### `Fusion.SessionCredential ShowCredentialDialog(string text, string caption)`

ShowCredentialDialog method of IHostUI.

**Parameters:**
- `text` (string)
- `caption` (string)

**Returns:** `Fusion.SessionCredential` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHostUI.ShowCredentialDialog%28System.String%2CSystem.String%29)

#### `void ShowDiagnosticWindow()`

ShowDiagnosticWindow method of IHostUI.

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHostUI.ShowDiagnosticWindow)

#### `object ShowDialog(string viewName, object parameter)`

ShowDialog method of IHostUI.

**Parameters:**
- `viewName` (string)
- `parameter` (object)

**Returns:** `object` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHostUI.ShowDialog%28System.String%2CSystem.Object%29)

#### `object ShowDialogWithStartupLocation(string viewName, object parameter, System.Windows.WindowStartupLocation windowStartupLocation)`

ShowDialogWithStartupLocation method of IHostUI.

**Parameters:**
- `viewName` (string)
- `parameter` (object)
- `windowStartupLocation` (System.Windows.WindowStartupLocation)

**Returns:** `object` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHostUI.ShowDialogWithStartupLocation%28System.String%2CSystem.Object%2CSystem.Windows.WindowStartupLocation%29)

#### `object ShowDialogWithStartupLocationAndOwner(string viewName, object parameter, System.Windows.WindowStartupLocation windowStartupLocation, nint dialogOwner)`

ShowDialogWithStartupLocationAndOwner method of IHostUI.

**Parameters:**
- `viewName` (string)
- `parameter` (object)
- `windowStartupLocation` (System.Windows.WindowStartupLocation)
- `dialogOwner` (nint)

**Returns:** `object` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHostUI.ShowDialogWithStartupLocationAndOwner%28System.String%2CSystem.Object%2CSystem.Windows.WindowStartupLocation%2CSystem.IntPtr%29)

#### `string ShowFolderPickerDialog(string caption, string initialFolder)`

ShowFolderPickerDialog method of IHostUI.

**Parameters:**
- `caption` (string)
- `initialFolder` (string)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHostUI.ShowFolderPickerDialog%28System.String%2CSystem.String%29)

#### `string ShowMessageDialog(string text, string caption = null, string icon = null, string[] buttons = null, string defaultButton = null, string cancelButton = null)`

ShowMessageDialog method of IHostUI.

**Parameters:**
- `text` (string)
- `caption` (string)
- `icon` (string)
- `buttons` (string[])
- `defaultButton` (string)
- `cancelButton` (string)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHostUI.ShowMessageDialog%28System.String%2CSystem.String%2CSystem.String%2CSystem.String%5B%5D%2CSystem.String%2CSystem.String%29)

#### `string ShowMessageDialogWithOnTop(string text, string caption = null, string icon = null, string[] buttons = null, string defaultButton = null, string cancelButton = null, bool modal = true, bool onTop = false)`

ShowMessageDialogWithOnTop method of IHostUI.

**Parameters:**
- `text` (string)
- `caption` (string)
- `icon` (string)
- `buttons` (string[])
- `defaultButton` (string)
- `cancelButton` (string)
- `modal` (bool)
- `onTop` (bool)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHostUI.ShowMessageDialogWithOnTop%28System.String%2CSystem.String%2CSystem.String%2CSystem.String%5B%5D%2CSystem.String%2CSystem.String%2CSystem.Boolean%2CSystem.Boolean%29)

#### `string[] ShowOpenFilePickerDialog(string caption, string initialFileName, string initialFolder, string[] customPlaces, bool allowMultiple, Fusion.FileType[] fileTypes)`

ShowOpenFilePickerDialog method of IHostUI.

**Parameters:**
- `caption` (string)
- `initialFileName` (string)
- `initialFolder` (string)
- `customPlaces` (string[])
- `allowMultiple` (bool)
- `fileTypes` (Fusion.FileType[])

**Returns:** `string[]` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHostUI.ShowOpenFilePickerDialog%28System.String%2CSystem.String%2CSystem.String%2CSystem.String%5B%5D%2CSystem.Boolean%2CFusion.FileType%5B%5D%29)

#### `string[] ShowOpenFilePickerDialog(string caption, string initialFolder, string[] customPlaces, bool allowMultiple, Fusion.FileType[] fileTypes)`

ShowOpenFilePickerDialog method of IHostUI.

**Parameters:**
- `caption` (string)
- `initialFolder` (string)
- `customPlaces` (string[])
- `allowMultiple` (bool)
- `fileTypes` (Fusion.FileType[])

**Returns:** `string[]` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHostUI.ShowOpenFilePickerDialog%28System.String%2CSystem.String%2CSystem.String%5B%5D%2CSystem.Boolean%2CFusion.FileType%5B%5D%29)

#### `bool ShowProgressDialog(System.Action<Fusion.Progress> backgroundOperation)`

ShowProgressDialog method of IHostUI.

**Parameters:**
- `backgroundOperation` (System.Action<Fusion.Progress>)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHostUI.ShowProgressDialog%28System.Action%7BFusion.Progress%7D%29)

#### `string ShowSaveFilePickerDialog(string caption, string initialFileName, string initialFolder, string[] customPlaces, Fusion.FileType[] fileTypes)`

ShowSaveFilePickerDialog method of IHostUI.

**Parameters:**
- `caption` (string)
- `initialFileName` (string)
- `initialFolder` (string)
- `customPlaces` (string[])
- `fileTypes` (Fusion.FileType[])

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHostUI.ShowSaveFilePickerDialog%28System.String%2CSystem.String%2CSystem.String%2CSystem.String%5B%5D%2CFusion.FileType%5B%5D%29)

#### `string ShowSaveFilePickerDialog(string caption, string initialFolder, string[] customPlaces, Fusion.FileType[] fileTypes)`

ShowSaveFilePickerDialog method of IHostUI.

**Parameters:**
- `caption` (string)
- `initialFolder` (string)
- `customPlaces` (string[])
- `fileTypes` (Fusion.FileType[])

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHostUI.ShowSaveFilePickerDialog%28System.String%2CSystem.String%2CSystem.String%5B%5D%2CFusion.FileType%5B%5D%29)

#### `Fusion.IWindowController ShowWindow(string viewName, object parameter)`

ShowWindow method of IHostUI.

**Parameters:**
- `viewName` (string)
- `parameter` (object)

**Returns:** `Fusion.IWindowController` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHostUI.ShowWindow%28System.String%2CSystem.Object%29)

#### `Fusion.IWindowController ShowWindowWithStartupLocation(string viewName, object parameter, System.Windows.WindowStartupLocation windowStartupLocation)`

ShowWindowWithStartupLocation method of IHostUI.

**Parameters:**
- `viewName` (string)
- `parameter` (object)
- `windowStartupLocation` (System.Windows.WindowStartupLocation)

**Returns:** `Fusion.IWindowController` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IHostUI.ShowWindowWithStartupLocation%28System.String%2CSystem.Object%2CSystem.Windows.WindowStartupLocation%29)

### Properties
- `FullScreenMode` (bool, get/set) — FullScreenMode property of IHostUI.
- `Help` (Fusion.IHelpProvider, get/set) — Help property of IHostUI.
- `PublishedViews` (System.Collections.Generic.IEnumerable<Fusion.PublishedViewInfo>, get) — PublishedViews property of IHostUI.
- `Resources` (System.Collections.IDictionary, get) — Resources property of IHostUI.
- `TabletMode` (bool, get/set) — TabletMode property of IHostUI.

## IHostUser (interface)

IHostUser interface in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.IHostUser)

### Properties
- `Authentication` (Fusion.IUserAuthentication, get) — Authentication property of IHostUser.
- `Authorization` (Fusion.IUserAuthorization, get) — Authorization property of IHostUser.
- `Cookies` (System.Net.CookieContainer, get) — Cookies property of IHostUser.
- `Credentials` (System.Net.ICredentials, get) — Credentials property of IHostUser.
- `Profile` (Fusion.IUserProfile, get) — Profile property of IHostUser.

## IKeyValueStore (interface)

IKeyValueStore interface in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.IKeyValueStore)

### Properties
- `Count` (int, get) — Count property of IKeyValueStore.
- `Item` (string, get/set) — Item property of IKeyValueStore.
- `Keys` (System.Collections.Generic.IEnumerable<string>, get) — Keys property of IKeyValueStore.

## INavigationController (interface)

INavigationController interface in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.INavigationController)

### Methods
#### `void Navigate(string viewName, object parameter)`

Navigate method of INavigationController.

**Parameters:**
- `viewName` (string)
- `parameter` (object)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.INavigationController.Navigate%28System.String%2CSystem.Object%29)

#### `void NavigateBack()`

NavigateBack method of INavigationController.

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.INavigationController.NavigateBack)

#### `void NavigateHome()`

NavigateHome method of INavigationController.

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.INavigationController.NavigateHome)

### Properties
- `CanNavigateBack` (bool, get) — CanNavigateBack property of INavigationController.

## INear`1 (interface)

INear`1 interface in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.INear`1)

### Methods
#### `bool Near(T other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of INear`1.

**Parameters:**
- `other` (T)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.INear%601.Near%28%600%2CSystem.Int32%29)

## IPopupController (interface)

IPopupController interface in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.IPopupController)

### Methods
#### `void Close()`

Close method of IPopupController.

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IPopupController.Close)

## IRecovery (interface)

IRecovery interface in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.IRecovery)

### Properties
- `Recover` (bool, get) — Recover property of IRecovery.

## ISession (interface)

ISession interface in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.ISession)

## ISettings (interface)

ISettings interface in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.ISettings)

### Properties
- `Culture` (string, get/set) — Culture property of ISettings.
- `EnableExceptionHandling` (bool, get/set) — EnableExceptionHandling property of ISettings.
- `EnableMessageLogging` (bool, get/set) — EnableMessageLogging property of ISettings.
- `EnableRecoveryAndRestart` (bool, get/set) — EnableRecoveryAndRestart property of ISettings.
- `EnableSingleInstanceMode` (bool, get/set) — EnableSingleInstanceMode property of ISettings.
- `EnableWindowGhosting` (bool, get/set) — EnableWindowGhosting property of ISettings.
- `FeatureFolder` (string, get/set) — FeatureFolder property of ISettings.
- `LocalizationMode` (Fusion.LocalizationMode, get/set) — LocalizationMode property of ISettings.
- `Theme` (string, get/set) — Theme property of ISettings.

## IUserAuthorization (interface)

IUserAuthorization interface in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.IUserAuthorization)

### Methods
#### `System.Threading.Tasks.Task<System.Collections.Generic.IReadOnlyList<Fusion.AuthorizationAvailableResponse>> IsAuthorizationAvailable(Fusion.AuthenticationToken authenticationToken, string[] actions)`

IsAuthorizationAvailable method of IUserAuthorization.

**Parameters:**
- `authenticationToken` (Fusion.AuthenticationToken)
- `actions` (string[])

**Returns:** `System.Threading.Tasks.Task<System.Collections.Generic.IReadOnlyList<Fusion.AuthorizationAvailableResponse>>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IUserAuthorization.IsAuthorizationAvailable%28Fusion.AuthenticationToken%2CSystem.String%5B%5D%29)

#### `System.Threading.Tasks.Task<Fusion.AuthorizationResponse> RequestRenew(Fusion.AuthorizationToken authorizationToken, System.TimeSpan duration, Fusion.AuthenticationToken authenticationToken)`

RequestRenew method of IUserAuthorization.

**Parameters:**
- `authorizationToken` (Fusion.AuthorizationToken)
- `duration` (System.TimeSpan)
- `authenticationToken` (Fusion.AuthenticationToken)

**Returns:** `System.Threading.Tasks.Task<Fusion.AuthorizationResponse>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IUserAuthorization.RequestRenew%28Fusion.AuthorizationToken%2CSystem.TimeSpan%2CFusion.AuthenticationToken%29)

#### `System.Threading.Tasks.Task<System.Collections.Generic.IReadOnlyList<Fusion.AuthorizationResponse>> RequestRenewTokens(System.Collections.Generic.IReadOnlyList<Fusion.AuthorizationToken> authorizationTokens, System.TimeSpan duration, Fusion.AuthenticationToken authenticationToken)`

RequestRenewTokens method of IUserAuthorization.

**Parameters:**
- `authorizationTokens` (System.Collections.Generic.IReadOnlyList<Fusion.AuthorizationToken>)
- `duration` (System.TimeSpan)
- `authenticationToken` (Fusion.AuthenticationToken)

**Returns:** `System.Threading.Tasks.Task<System.Collections.Generic.IReadOnlyList<Fusion.AuthorizationResponse>>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IUserAuthorization.RequestRenewTokens%28System.Collections.Generic.IReadOnlyList%7BFusion.AuthorizationToken%7D%2CSystem.TimeSpan%2CFusion.AuthenticationToken%29)

#### `System.Threading.Tasks.Task<Fusion.AuthorizationResponse> RequestToken(Fusion.AuthenticationToken authenticationToken, System.TimeSpan duration, string action)`

RequestToken method of IUserAuthorization.

**Parameters:**
- `authenticationToken` (Fusion.AuthenticationToken)
- `duration` (System.TimeSpan)
- `action` (string)

**Returns:** `System.Threading.Tasks.Task<Fusion.AuthorizationResponse>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IUserAuthorization.RequestToken%28Fusion.AuthenticationToken%2CSystem.TimeSpan%2CSystem.String%29)

#### `System.Threading.Tasks.Task<System.Collections.Generic.IReadOnlyList<Fusion.AuthorizationResponse>> RequestTokens(Fusion.AuthenticationToken authenticationToken, System.TimeSpan duration, string[] actions)`

RequestTokens method of IUserAuthorization.

**Parameters:**
- `authenticationToken` (Fusion.AuthenticationToken)
- `duration` (System.TimeSpan)
- `actions` (string[])

**Returns:** `System.Threading.Tasks.Task<System.Collections.Generic.IReadOnlyList<Fusion.AuthorizationResponse>>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IUserAuthorization.RequestTokens%28Fusion.AuthenticationToken%2CSystem.TimeSpan%2CSystem.String%5B%5D%29)

## IUserProfile (interface)

IUserProfile interface in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.IUserProfile)

### Methods
#### `System.Threading.Tasks.Task<System.Collections.Generic.IDictionary<string, string>> GetProfile(Fusion.AuthenticationToken authenticationToken, string principal, string[] properties)`

GetProfile method of IUserProfile.

**Parameters:**
- `authenticationToken` (Fusion.AuthenticationToken)
- `principal` (string)
- `properties` (string[])

**Returns:** `System.Threading.Tasks.Task<System.Collections.Generic.IDictionary<string, string>>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IUserProfile.GetProfile%28Fusion.AuthenticationToken%2CSystem.String%2CSystem.String%5B%5D%29)

#### `System.Threading.Tasks.Task<System.Uri> GetProfileLink(Fusion.AuthenticationToken authenticationToken, string principal)`

GetProfileLink method of IUserProfile.

**Parameters:**
- `authenticationToken` (Fusion.AuthenticationToken)
- `principal` (string)

**Returns:** `System.Threading.Tasks.Task<System.Uri>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IUserProfile.GetProfileLink%28Fusion.AuthenticationToken%2CSystem.String%29)

#### `System.Threading.Tasks.Task<System.Collections.Generic.IDictionary<string, System.Collections.Generic.IDictionary<string, string>>> GetProfiles(Fusion.AuthenticationToken authenticationToken, string[] properties)`

GetProfiles method of IUserProfile.

**Parameters:**
- `authenticationToken` (Fusion.AuthenticationToken)
- `properties` (string[])

**Returns:** `System.Threading.Tasks.Task<System.Collections.Generic.IDictionary<string, System.Collections.Generic.IDictionary<string, string>>>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IUserProfile.GetProfiles%28Fusion.AuthenticationToken%2CSystem.String%5B%5D%29)

## IWindowController (interface)

IWindowController interface in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.IWindowController)

### Methods
#### `void Activate()`

Activate method of IWindowController.

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IWindowController.Activate)

#### `void BringToTop()`

BringToTop method of IWindowController.

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IWindowController.BringToTop)

#### `void Close()`

Close method of IWindowController.

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IWindowController.Close)

#### `void Close(object dialogResult)`

Close method of IWindowController.

**Parameters:**
- `dialogResult` (object)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IWindowController.Close%28System.Object%29)

#### `void SetOwnedWindowClosedCallback(System.Action action)`

SetOwnedWindowClosedCallback method of IWindowController.

**Parameters:**
- `action` (System.Action)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IWindowController.SetOwnedWindowClosedCallback%28System.Action%29)

#### `void UpdateLayout()`

UpdateLayout method of IWindowController.

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.IWindowController.UpdateLayout)

### Properties
- `HasOwnedWindow` (bool, get) — HasOwnedWindow property of IWindowController.
- `Height` (double, get/set) — Height property of IWindowController.
- `IsOpen` (bool, get) — IsOpen property of IWindowController.
- `Left` (double, get/set) — Left property of IWindowController.
- `Top` (double, get/set) — Top property of IWindowController.
- `Width` (double, get/set) — Width property of IWindowController.

## Int2 (struct)

Int2 struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Int2)

### Constructors
- `Int2(int all)` — Constructs a new Int2.
- `Int2(int x, int y)` — Constructs a new Int2.

### Methods
#### `static Fusion.Int2 Abs(Fusion.Int2 value)`

Abs method of Int2.

**Parameters:**
- `value` (Fusion.Int2)

**Returns:** `Fusion.Int2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.Abs%28Fusion.Int2%29)

#### `bool Equals(Fusion.Int2 other)`

Equals method of Int2.

**Parameters:**
- `other` (Fusion.Int2)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.Equals%28Fusion.Int2%29)

#### `bool Equals(object other)`

Equals method of Int2.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.Equals%28System.Object%29)

#### `int GetHashCode()`

GetHashCode method of Int2.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.GetHashCode)

#### `static Fusion.Int2 Max(Fusion.Int2 a, Fusion.Int2 b)`

Max method of Int2.

**Parameters:**
- `a` (Fusion.Int2)
- `b` (Fusion.Int2)

**Returns:** `Fusion.Int2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.Max%28Fusion.Int2%2CFusion.Int2%29)

#### `static Fusion.Int2 Min(Fusion.Int2 a, Fusion.Int2 b)`

Min method of Int2.

**Parameters:**
- `a` (Fusion.Int2)
- `b` (Fusion.Int2)

**Returns:** `Fusion.Int2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.Min%28Fusion.Int2%2CFusion.Int2%29)

#### `static Fusion.Int2 Sign(Fusion.Int2 value)`

Sign method of Int2.

**Parameters:**
- `value` (Fusion.Int2)

**Returns:** `Fusion.Int2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.Sign%28Fusion.Int2%29)

#### `string ToString()`

ToString method of Int2.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of Int2.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.Int2 op_Addition(Fusion.Int2 a, Fusion.Int2 b)`

op_Addition method of Int2.

**Parameters:**
- `a` (Fusion.Int2)
- `b` (Fusion.Int2)

**Returns:** `Fusion.Int2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.op_Addition%28Fusion.Int2%2CFusion.Int2%29)

#### `static Fusion.Int2 op_Addition(Fusion.Int2 a, int b)`

op_Addition method of Int2.

**Parameters:**
- `a` (Fusion.Int2)
- `b` (int)

**Returns:** `Fusion.Int2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.op_Addition%28Fusion.Int2%2CSystem.Int32%29)

#### `static Fusion.Int2 op_Addition(int a, Fusion.Int2 b)`

op_Addition method of Int2.

**Parameters:**
- `a` (int)
- `b` (Fusion.Int2)

**Returns:** `Fusion.Int2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.op_Addition%28System.Int32%2CFusion.Int2%29)

#### `static Fusion.Int2 op_Division(Fusion.Int2 a, Fusion.Int2 b)`

op_Division method of Int2.

**Parameters:**
- `a` (Fusion.Int2)
- `b` (Fusion.Int2)

**Returns:** `Fusion.Int2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.op_Division%28Fusion.Int2%2CFusion.Int2%29)

#### `static Fusion.Int2 op_Division(Fusion.Int2 a, int b)`

op_Division method of Int2.

**Parameters:**
- `a` (Fusion.Int2)
- `b` (int)

**Returns:** `Fusion.Int2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.op_Division%28Fusion.Int2%2CSystem.Int32%29)

#### `static Fusion.Int2 op_Division(int a, Fusion.Int2 b)`

op_Division method of Int2.

**Parameters:**
- `a` (int)
- `b` (Fusion.Int2)

**Returns:** `Fusion.Int2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.op_Division%28System.Int32%2CFusion.Int2%29)

#### `static bool op_Equality(Fusion.Int2 a, Fusion.Int2 b)`

op_Equality method of Int2.

**Parameters:**
- `a` (Fusion.Int2)
- `b` (Fusion.Int2)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.op_Equality%28Fusion.Int2%2CFusion.Int2%29)

#### `static Fusion.Int2 op_Explicit(Fusion.Double2 v)`

op_Explicit method of Int2.

**Parameters:**
- `v` (Fusion.Double2)

**Returns:** `Fusion.Int2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.op_Explicit%28Fusion.Double2%29~Fusion.Int2)

#### `static Fusion.Int2 op_Explicit(Fusion.Float2 v)`

op_Explicit method of Int2.

**Parameters:**
- `v` (Fusion.Float2)

**Returns:** `Fusion.Int2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.op_Explicit%28Fusion.Float2%29~Fusion.Int2)

#### `static Fusion.Int2 op_Explicit(Fusion.Half2 v)`

op_Explicit method of Int2.

**Parameters:**
- `v` (Fusion.Half2)

**Returns:** `Fusion.Int2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.op_Explicit%28Fusion.Half2%29~Fusion.Int2)

#### `static Fusion.Int2 op_Implicit(Fusion.Short2 v)`

op_Implicit method of Int2.

**Parameters:**
- `v` (Fusion.Short2)

**Returns:** `Fusion.Int2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.op_Implicit%28Fusion.Short2%29~Fusion.Int2)

#### `static Fusion.Int2 op_Implicit(Fusion.UShort2 v)`

op_Implicit method of Int2.

**Parameters:**
- `v` (Fusion.UShort2)

**Returns:** `Fusion.Int2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.op_Implicit%28Fusion.UShort2%29~Fusion.Int2)

#### `static bool op_Inequality(Fusion.Int2 a, Fusion.Int2 b)`

op_Inequality method of Int2.

**Parameters:**
- `a` (Fusion.Int2)
- `b` (Fusion.Int2)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.op_Inequality%28Fusion.Int2%2CFusion.Int2%29)

#### `static Fusion.Int2 op_Modulus(Fusion.Int2 a, Fusion.Int2 b)`

op_Modulus method of Int2.

**Parameters:**
- `a` (Fusion.Int2)
- `b` (Fusion.Int2)

**Returns:** `Fusion.Int2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.op_Modulus%28Fusion.Int2%2CFusion.Int2%29)

#### `static Fusion.Int2 op_Modulus(Fusion.Int2 a, int b)`

op_Modulus method of Int2.

**Parameters:**
- `a` (Fusion.Int2)
- `b` (int)

**Returns:** `Fusion.Int2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.op_Modulus%28Fusion.Int2%2CSystem.Int32%29)

#### `static Fusion.Int2 op_Modulus(int a, Fusion.Int2 b)`

op_Modulus method of Int2.

**Parameters:**
- `a` (int)
- `b` (Fusion.Int2)

**Returns:** `Fusion.Int2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.op_Modulus%28System.Int32%2CFusion.Int2%29)

#### `static Fusion.Int2 op_Multiply(Fusion.Int2 a, Fusion.Int2 b)`

op_Multiply method of Int2.

**Parameters:**
- `a` (Fusion.Int2)
- `b` (Fusion.Int2)

**Returns:** `Fusion.Int2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.op_Multiply%28Fusion.Int2%2CFusion.Int2%29)

#### `static Fusion.Int2 op_Multiply(Fusion.Int2 a, int b)`

op_Multiply method of Int2.

**Parameters:**
- `a` (Fusion.Int2)
- `b` (int)

**Returns:** `Fusion.Int2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.op_Multiply%28Fusion.Int2%2CSystem.Int32%29)

#### `static Fusion.Int2 op_Multiply(int a, Fusion.Int2 b)`

op_Multiply method of Int2.

**Parameters:**
- `a` (int)
- `b` (Fusion.Int2)

**Returns:** `Fusion.Int2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.op_Multiply%28System.Int32%2CFusion.Int2%29)

#### `static Fusion.Int2 op_Subtraction(Fusion.Int2 a, Fusion.Int2 b)`

op_Subtraction method of Int2.

**Parameters:**
- `a` (Fusion.Int2)
- `b` (Fusion.Int2)

**Returns:** `Fusion.Int2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.op_Subtraction%28Fusion.Int2%2CFusion.Int2%29)

#### `static Fusion.Int2 op_Subtraction(Fusion.Int2 a, int b)`

op_Subtraction method of Int2.

**Parameters:**
- `a` (Fusion.Int2)
- `b` (int)

**Returns:** `Fusion.Int2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.op_Subtraction%28Fusion.Int2%2CSystem.Int32%29)

#### `static Fusion.Int2 op_Subtraction(int a, Fusion.Int2 b)`

op_Subtraction method of Int2.

**Parameters:**
- `a` (int)
- `b` (Fusion.Int2)

**Returns:** `Fusion.Int2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.op_Subtraction%28System.Int32%2CFusion.Int2%29)

#### `static Fusion.Int2 op_UnaryNegation(Fusion.Int2 v)`

op_UnaryNegation method of Int2.

**Parameters:**
- `v` (Fusion.Int2)

**Returns:** `Fusion.Int2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.op_UnaryNegation%28Fusion.Int2%29)

#### `static Fusion.Int2 op_UnaryPlus(Fusion.Int2 v)`

op_UnaryPlus method of Int2.

**Parameters:**
- `v` (Fusion.Int2)

**Returns:** `Fusion.Int2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int2.op_UnaryPlus%28Fusion.Int2%29)

### Properties
- `Item` (int, get/set) — Item property of Int2.

## Int3 (struct)

Int3 struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Int3)

### Constructors
- `Int3(Fusion.Int2 xy, int z)` — Constructs a new Int3.
- `Int3(int all)` — Constructs a new Int3.
- `Int3(int x, Fusion.Int2 yz)` — Constructs a new Int3.
- `Int3(int x, int y, int z)` — Constructs a new Int3.

### Methods
#### `static Fusion.Int3 Abs(Fusion.Int3 value)`

Abs method of Int3.

**Parameters:**
- `value` (Fusion.Int3)

**Returns:** `Fusion.Int3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.Abs%28Fusion.Int3%29)

#### `bool Equals(Fusion.Int3 other)`

Equals method of Int3.

**Parameters:**
- `other` (Fusion.Int3)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.Equals%28Fusion.Int3%29)

#### `bool Equals(object other)`

Equals method of Int3.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.Equals%28System.Object%29)

#### `int GetHashCode()`

GetHashCode method of Int3.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.GetHashCode)

#### `static Fusion.Int3 Max(Fusion.Int3 a, Fusion.Int3 b)`

Max method of Int3.

**Parameters:**
- `a` (Fusion.Int3)
- `b` (Fusion.Int3)

**Returns:** `Fusion.Int3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.Max%28Fusion.Int3%2CFusion.Int3%29)

#### `static Fusion.Int3 Min(Fusion.Int3 a, Fusion.Int3 b)`

Min method of Int3.

**Parameters:**
- `a` (Fusion.Int3)
- `b` (Fusion.Int3)

**Returns:** `Fusion.Int3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.Min%28Fusion.Int3%2CFusion.Int3%29)

#### `static Fusion.Int3 Sign(Fusion.Int3 value)`

Sign method of Int3.

**Parameters:**
- `value` (Fusion.Int3)

**Returns:** `Fusion.Int3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.Sign%28Fusion.Int3%29)

#### `string ToString()`

ToString method of Int3.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of Int3.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.Int3 op_Addition(Fusion.Int3 a, Fusion.Int3 b)`

op_Addition method of Int3.

**Parameters:**
- `a` (Fusion.Int3)
- `b` (Fusion.Int3)

**Returns:** `Fusion.Int3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.op_Addition%28Fusion.Int3%2CFusion.Int3%29)

#### `static Fusion.Int3 op_Addition(Fusion.Int3 a, int b)`

op_Addition method of Int3.

**Parameters:**
- `a` (Fusion.Int3)
- `b` (int)

**Returns:** `Fusion.Int3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.op_Addition%28Fusion.Int3%2CSystem.Int32%29)

#### `static Fusion.Int3 op_Addition(int a, Fusion.Int3 b)`

op_Addition method of Int3.

**Parameters:**
- `a` (int)
- `b` (Fusion.Int3)

**Returns:** `Fusion.Int3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.op_Addition%28System.Int32%2CFusion.Int3%29)

#### `static Fusion.Int3 op_Division(Fusion.Int3 a, Fusion.Int3 b)`

op_Division method of Int3.

**Parameters:**
- `a` (Fusion.Int3)
- `b` (Fusion.Int3)

**Returns:** `Fusion.Int3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.op_Division%28Fusion.Int3%2CFusion.Int3%29)

#### `static Fusion.Int3 op_Division(Fusion.Int3 a, int b)`

op_Division method of Int3.

**Parameters:**
- `a` (Fusion.Int3)
- `b` (int)

**Returns:** `Fusion.Int3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.op_Division%28Fusion.Int3%2CSystem.Int32%29)

#### `static Fusion.Int3 op_Division(int a, Fusion.Int3 b)`

op_Division method of Int3.

**Parameters:**
- `a` (int)
- `b` (Fusion.Int3)

**Returns:** `Fusion.Int3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.op_Division%28System.Int32%2CFusion.Int3%29)

#### `static bool op_Equality(Fusion.Int3 a, Fusion.Int3 b)`

op_Equality method of Int3.

**Parameters:**
- `a` (Fusion.Int3)
- `b` (Fusion.Int3)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.op_Equality%28Fusion.Int3%2CFusion.Int3%29)

#### `static Fusion.Int3 op_Explicit(Fusion.Double3 v)`

op_Explicit method of Int3.

**Parameters:**
- `v` (Fusion.Double3)

**Returns:** `Fusion.Int3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.op_Explicit%28Fusion.Double3%29~Fusion.Int3)

#### `static Fusion.Int3 op_Explicit(Fusion.Float3 v)`

op_Explicit method of Int3.

**Parameters:**
- `v` (Fusion.Float3)

**Returns:** `Fusion.Int3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.op_Explicit%28Fusion.Float3%29~Fusion.Int3)

#### `static Fusion.Int3 op_Explicit(Fusion.Half3 v)`

op_Explicit method of Int3.

**Parameters:**
- `v` (Fusion.Half3)

**Returns:** `Fusion.Int3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.op_Explicit%28Fusion.Half3%29~Fusion.Int3)

#### `static Fusion.Int3 op_Implicit(Fusion.Short3 v)`

op_Implicit method of Int3.

**Parameters:**
- `v` (Fusion.Short3)

**Returns:** `Fusion.Int3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.op_Implicit%28Fusion.Short3%29~Fusion.Int3)

#### `static Fusion.Int3 op_Implicit(Fusion.UShort3 v)`

op_Implicit method of Int3.

**Parameters:**
- `v` (Fusion.UShort3)

**Returns:** `Fusion.Int3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.op_Implicit%28Fusion.UShort3%29~Fusion.Int3)

#### `static bool op_Inequality(Fusion.Int3 a, Fusion.Int3 b)`

op_Inequality method of Int3.

**Parameters:**
- `a` (Fusion.Int3)
- `b` (Fusion.Int3)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.op_Inequality%28Fusion.Int3%2CFusion.Int3%29)

#### `static Fusion.Int3 op_Modulus(Fusion.Int3 a, Fusion.Int3 b)`

op_Modulus method of Int3.

**Parameters:**
- `a` (Fusion.Int3)
- `b` (Fusion.Int3)

**Returns:** `Fusion.Int3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.op_Modulus%28Fusion.Int3%2CFusion.Int3%29)

#### `static Fusion.Int3 op_Modulus(Fusion.Int3 a, int b)`

op_Modulus method of Int3.

**Parameters:**
- `a` (Fusion.Int3)
- `b` (int)

**Returns:** `Fusion.Int3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.op_Modulus%28Fusion.Int3%2CSystem.Int32%29)

#### `static Fusion.Int3 op_Modulus(int a, Fusion.Int3 b)`

op_Modulus method of Int3.

**Parameters:**
- `a` (int)
- `b` (Fusion.Int3)

**Returns:** `Fusion.Int3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.op_Modulus%28System.Int32%2CFusion.Int3%29)

#### `static Fusion.Int3 op_Multiply(Fusion.Int3 a, Fusion.Int3 b)`

op_Multiply method of Int3.

**Parameters:**
- `a` (Fusion.Int3)
- `b` (Fusion.Int3)

**Returns:** `Fusion.Int3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.op_Multiply%28Fusion.Int3%2CFusion.Int3%29)

#### `static Fusion.Int3 op_Multiply(Fusion.Int3 a, int b)`

op_Multiply method of Int3.

**Parameters:**
- `a` (Fusion.Int3)
- `b` (int)

**Returns:** `Fusion.Int3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.op_Multiply%28Fusion.Int3%2CSystem.Int32%29)

#### `static Fusion.Int3 op_Multiply(int a, Fusion.Int3 b)`

op_Multiply method of Int3.

**Parameters:**
- `a` (int)
- `b` (Fusion.Int3)

**Returns:** `Fusion.Int3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.op_Multiply%28System.Int32%2CFusion.Int3%29)

#### `static Fusion.Int3 op_Subtraction(Fusion.Int3 a, Fusion.Int3 b)`

op_Subtraction method of Int3.

**Parameters:**
- `a` (Fusion.Int3)
- `b` (Fusion.Int3)

**Returns:** `Fusion.Int3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.op_Subtraction%28Fusion.Int3%2CFusion.Int3%29)

#### `static Fusion.Int3 op_Subtraction(Fusion.Int3 a, int b)`

op_Subtraction method of Int3.

**Parameters:**
- `a` (Fusion.Int3)
- `b` (int)

**Returns:** `Fusion.Int3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.op_Subtraction%28Fusion.Int3%2CSystem.Int32%29)

#### `static Fusion.Int3 op_Subtraction(int a, Fusion.Int3 b)`

op_Subtraction method of Int3.

**Parameters:**
- `a` (int)
- `b` (Fusion.Int3)

**Returns:** `Fusion.Int3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.op_Subtraction%28System.Int32%2CFusion.Int3%29)

#### `static Fusion.Int3 op_UnaryNegation(Fusion.Int3 v)`

op_UnaryNegation method of Int3.

**Parameters:**
- `v` (Fusion.Int3)

**Returns:** `Fusion.Int3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.op_UnaryNegation%28Fusion.Int3%29)

#### `static Fusion.Int3 op_UnaryPlus(Fusion.Int3 v)`

op_UnaryPlus method of Int3.

**Parameters:**
- `v` (Fusion.Int3)

**Returns:** `Fusion.Int3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int3.op_UnaryPlus%28Fusion.Int3%29)

### Properties
- `Item` (int, get/set) — Item property of Int3.
- `XY` (Fusion.Int2, get) — XY property of Int3.

## Int4 (struct)

Int4 struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Int4)

### Constructors
- `Int4(Fusion.Int2 xy, Fusion.Int2 zw)` — Constructs a new Int4.
- `Int4(Fusion.Int2 xy, int z, int w)` — Constructs a new Int4.
- `Int4(Fusion.Int3 xyz, int w)` — Constructs a new Int4.
- `Int4(int all)` — Constructs a new Int4.
- `Int4(int x, Fusion.Int2 yz, int w)` — Constructs a new Int4.
- `Int4(int x, Fusion.Int3 yzw)` — Constructs a new Int4.
- `Int4(int x, int y, Fusion.Int2 zw)` — Constructs a new Int4.
- `Int4(int x, int y, int z, int w)` — Constructs a new Int4.

### Methods
#### `static Fusion.Int4 Abs(Fusion.Int4 value)`

Abs method of Int4.

**Parameters:**
- `value` (Fusion.Int4)

**Returns:** `Fusion.Int4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.Abs%28Fusion.Int4%29)

#### `bool Equals(Fusion.Int4 other)`

Equals method of Int4.

**Parameters:**
- `other` (Fusion.Int4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.Equals%28Fusion.Int4%29)

#### `bool Equals(object other)`

Equals method of Int4.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.Equals%28System.Object%29)

#### `int GetHashCode()`

GetHashCode method of Int4.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.GetHashCode)

#### `static Fusion.Int4 Max(Fusion.Int4 a, Fusion.Int4 b)`

Max method of Int4.

**Parameters:**
- `a` (Fusion.Int4)
- `b` (Fusion.Int4)

**Returns:** `Fusion.Int4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.Max%28Fusion.Int4%2CFusion.Int4%29)

#### `static Fusion.Int4 Min(Fusion.Int4 a, Fusion.Int4 b)`

Min method of Int4.

**Parameters:**
- `a` (Fusion.Int4)
- `b` (Fusion.Int4)

**Returns:** `Fusion.Int4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.Min%28Fusion.Int4%2CFusion.Int4%29)

#### `static Fusion.Int4 Sign(Fusion.Int4 value)`

Sign method of Int4.

**Parameters:**
- `value` (Fusion.Int4)

**Returns:** `Fusion.Int4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.Sign%28Fusion.Int4%29)

#### `string ToString()`

ToString method of Int4.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of Int4.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.Int4 op_Addition(Fusion.Int4 a, Fusion.Int4 b)`

op_Addition method of Int4.

**Parameters:**
- `a` (Fusion.Int4)
- `b` (Fusion.Int4)

**Returns:** `Fusion.Int4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.op_Addition%28Fusion.Int4%2CFusion.Int4%29)

#### `static Fusion.Int4 op_Addition(Fusion.Int4 a, int b)`

op_Addition method of Int4.

**Parameters:**
- `a` (Fusion.Int4)
- `b` (int)

**Returns:** `Fusion.Int4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.op_Addition%28Fusion.Int4%2CSystem.Int32%29)

#### `static Fusion.Int4 op_Addition(int a, Fusion.Int4 b)`

op_Addition method of Int4.

**Parameters:**
- `a` (int)
- `b` (Fusion.Int4)

**Returns:** `Fusion.Int4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.op_Addition%28System.Int32%2CFusion.Int4%29)

#### `static Fusion.Int4 op_Division(Fusion.Int4 a, Fusion.Int4 b)`

op_Division method of Int4.

**Parameters:**
- `a` (Fusion.Int4)
- `b` (Fusion.Int4)

**Returns:** `Fusion.Int4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.op_Division%28Fusion.Int4%2CFusion.Int4%29)

#### `static Fusion.Int4 op_Division(Fusion.Int4 a, int b)`

op_Division method of Int4.

**Parameters:**
- `a` (Fusion.Int4)
- `b` (int)

**Returns:** `Fusion.Int4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.op_Division%28Fusion.Int4%2CSystem.Int32%29)

#### `static Fusion.Int4 op_Division(int a, Fusion.Int4 b)`

op_Division method of Int4.

**Parameters:**
- `a` (int)
- `b` (Fusion.Int4)

**Returns:** `Fusion.Int4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.op_Division%28System.Int32%2CFusion.Int4%29)

#### `static bool op_Equality(Fusion.Int4 a, Fusion.Int4 b)`

op_Equality method of Int4.

**Parameters:**
- `a` (Fusion.Int4)
- `b` (Fusion.Int4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.op_Equality%28Fusion.Int4%2CFusion.Int4%29)

#### `static Fusion.Int4 op_Explicit(Fusion.Double4 v)`

op_Explicit method of Int4.

**Parameters:**
- `v` (Fusion.Double4)

**Returns:** `Fusion.Int4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.op_Explicit%28Fusion.Double4%29~Fusion.Int4)

#### `static Fusion.Int4 op_Explicit(Fusion.Float4 v)`

op_Explicit method of Int4.

**Parameters:**
- `v` (Fusion.Float4)

**Returns:** `Fusion.Int4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.op_Explicit%28Fusion.Float4%29~Fusion.Int4)

#### `static Fusion.Int4 op_Explicit(Fusion.Half4 v)`

op_Explicit method of Int4.

**Parameters:**
- `v` (Fusion.Half4)

**Returns:** `Fusion.Int4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.op_Explicit%28Fusion.Half4%29~Fusion.Int4)

#### `static Fusion.Int4 op_Implicit(Fusion.RGBA v)`

op_Implicit method of Int4.

**Parameters:**
- `v` (Fusion.RGBA)

**Returns:** `Fusion.Int4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.op_Implicit%28Fusion.RGBA%29~Fusion.Int4)

#### `static Fusion.Int4 op_Implicit(Fusion.Short4 v)`

op_Implicit method of Int4.

**Parameters:**
- `v` (Fusion.Short4)

**Returns:** `Fusion.Int4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.op_Implicit%28Fusion.Short4%29~Fusion.Int4)

#### `static Fusion.Int4 op_Implicit(Fusion.UShort4 v)`

op_Implicit method of Int4.

**Parameters:**
- `v` (Fusion.UShort4)

**Returns:** `Fusion.Int4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.op_Implicit%28Fusion.UShort4%29~Fusion.Int4)

#### `static bool op_Inequality(Fusion.Int4 a, Fusion.Int4 b)`

op_Inequality method of Int4.

**Parameters:**
- `a` (Fusion.Int4)
- `b` (Fusion.Int4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.op_Inequality%28Fusion.Int4%2CFusion.Int4%29)

#### `static Fusion.Int4 op_Modulus(Fusion.Int4 a, Fusion.Int4 b)`

op_Modulus method of Int4.

**Parameters:**
- `a` (Fusion.Int4)
- `b` (Fusion.Int4)

**Returns:** `Fusion.Int4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.op_Modulus%28Fusion.Int4%2CFusion.Int4%29)

#### `static Fusion.Int4 op_Modulus(Fusion.Int4 a, int b)`

op_Modulus method of Int4.

**Parameters:**
- `a` (Fusion.Int4)
- `b` (int)

**Returns:** `Fusion.Int4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.op_Modulus%28Fusion.Int4%2CSystem.Int32%29)

#### `static Fusion.Int4 op_Modulus(int a, Fusion.Int4 b)`

op_Modulus method of Int4.

**Parameters:**
- `a` (int)
- `b` (Fusion.Int4)

**Returns:** `Fusion.Int4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.op_Modulus%28System.Int32%2CFusion.Int4%29)

#### `static Fusion.Int4 op_Multiply(Fusion.Int4 a, Fusion.Int4 b)`

op_Multiply method of Int4.

**Parameters:**
- `a` (Fusion.Int4)
- `b` (Fusion.Int4)

**Returns:** `Fusion.Int4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.op_Multiply%28Fusion.Int4%2CFusion.Int4%29)

#### `static Fusion.Int4 op_Multiply(Fusion.Int4 a, int b)`

op_Multiply method of Int4.

**Parameters:**
- `a` (Fusion.Int4)
- `b` (int)

**Returns:** `Fusion.Int4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.op_Multiply%28Fusion.Int4%2CSystem.Int32%29)

#### `static Fusion.Int4 op_Multiply(int a, Fusion.Int4 b)`

op_Multiply method of Int4.

**Parameters:**
- `a` (int)
- `b` (Fusion.Int4)

**Returns:** `Fusion.Int4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.op_Multiply%28System.Int32%2CFusion.Int4%29)

#### `static Fusion.Int4 op_Subtraction(Fusion.Int4 a, Fusion.Int4 b)`

op_Subtraction method of Int4.

**Parameters:**
- `a` (Fusion.Int4)
- `b` (Fusion.Int4)

**Returns:** `Fusion.Int4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.op_Subtraction%28Fusion.Int4%2CFusion.Int4%29)

#### `static Fusion.Int4 op_Subtraction(Fusion.Int4 a, int b)`

op_Subtraction method of Int4.

**Parameters:**
- `a` (Fusion.Int4)
- `b` (int)

**Returns:** `Fusion.Int4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.op_Subtraction%28Fusion.Int4%2CSystem.Int32%29)

#### `static Fusion.Int4 op_Subtraction(int a, Fusion.Int4 b)`

op_Subtraction method of Int4.

**Parameters:**
- `a` (int)
- `b` (Fusion.Int4)

**Returns:** `Fusion.Int4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.op_Subtraction%28System.Int32%2CFusion.Int4%29)

#### `static Fusion.Int4 op_UnaryNegation(Fusion.Int4 v)`

op_UnaryNegation method of Int4.

**Parameters:**
- `v` (Fusion.Int4)

**Returns:** `Fusion.Int4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.op_UnaryNegation%28Fusion.Int4%29)

#### `static Fusion.Int4 op_UnaryPlus(Fusion.Int4 v)`

op_UnaryPlus method of Int4.

**Parameters:**
- `v` (Fusion.Int4)

**Returns:** `Fusion.Int4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Int4.op_UnaryPlus%28Fusion.Int4%29)

### Properties
- `Item` (int, get/set) — Item property of Int4.
- `XY` (Fusion.Int2, get) — XY property of Int4.
- `XYZ` (Fusion.Int3, get) — XYZ property of Int4.

## InvokeIn (enum)

InvokeIn enum in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.InvokeIn)

### Values
- `Default` = `0`
- `Background` = `1`
- `DomainLogic` = `2`
- `UI` = `3`

## ItemsSource`1 (class)

ItemsSource`1 class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.ItemsSource`1)

### Constructors
- `ItemsSource`1(System.Collections.Generic.IEnumerable<T> itemsSource = null)` — Constructs a new ItemsSource`1.

### Methods
#### `System.Collections.Generic.IEnumerator<T> GetEnumerator()`

GetEnumerator method of ItemsSource`1.

**Returns:** `System.Collections.Generic.IEnumerator<T>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.ItemsSource%601.GetEnumerator)

#### `void Refresh()`

Refresh method of ItemsSource`1.

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.ItemsSource%601.Refresh)

### Properties
- `Items` (System.Collections.Generic.IEnumerable<T>, get/set) — Items property of ItemsSource`1.

### Events
#### `CollectionChanged` (`System.Collections.Specialized.NotifyCollectionChangedEventHandler`)

**Signature:** `event System.Collections.Specialized.NotifyCollectionChangedEventHandler CollectionChanged`

CollectionChanged event of ItemsSource`1.

[Docs](https://www.nuget.org/packages/TeklaFusion#E%3AFusion.ItemsSource%601.CollectionChanged)

## KinematicViscosity (struct)

KinematicViscosity struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.KinematicViscosity)

### Constructors
- `KinematicViscosity(double squareMetersPerSecond)` — Constructs a new KinematicViscosity.

### Methods
#### `static Fusion.KinematicViscosity Abs(Fusion.KinematicViscosity kinematicViscosity)`

Abs method of KinematicViscosity.

**Parameters:**
- `kinematicViscosity` (Fusion.KinematicViscosity)

**Returns:** `Fusion.KinematicViscosity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.Abs%28Fusion.KinematicViscosity%29)

#### `int CompareTo(Fusion.KinematicViscosity other)`

CompareTo method of KinematicViscosity.

**Parameters:**
- `other` (Fusion.KinematicViscosity)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.CompareTo%28Fusion.KinematicViscosity%29)

#### `int CompareTo(object other)`

CompareTo method of KinematicViscosity.

**Parameters:**
- `other` (object)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.CompareTo%28System.Object%29)

#### `bool Equals(Fusion.KinematicViscosity other)`

Equals method of KinematicViscosity.

**Parameters:**
- `other` (Fusion.KinematicViscosity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.Equals%28Fusion.KinematicViscosity%29)

#### `bool Equals(object other)`

Equals method of KinematicViscosity.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.Equals%28System.Object%29)

#### `static Fusion.KinematicViscosity From(double kinematicViscosity, Fusion.KinematicViscosityUnit kinematicViscosityUnit)`

From method of KinematicViscosity.

**Parameters:**
- `kinematicViscosity` (double)
- `kinematicViscosityUnit` (Fusion.KinematicViscosityUnit)

**Returns:** `Fusion.KinematicViscosity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.From%28System.Double%2CFusion.KinematicViscosityUnit%29)

#### `int GetHashCode()`

GetHashCode method of KinematicViscosity.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.GetHashCode)

#### `static bool IsInfinity(Fusion.KinematicViscosity kinematicViscosity)`

IsInfinity method of KinematicViscosity.

**Parameters:**
- `kinematicViscosity` (Fusion.KinematicViscosity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.IsInfinity%28Fusion.KinematicViscosity%29)

#### `static bool IsNaN(Fusion.KinematicViscosity kinematicViscosity)`

IsNaN method of KinematicViscosity.

**Parameters:**
- `kinematicViscosity` (Fusion.KinematicViscosity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.IsNaN%28Fusion.KinematicViscosity%29)

#### `static bool IsNegativeInfinity(Fusion.KinematicViscosity kinematicViscosity)`

IsNegativeInfinity method of KinematicViscosity.

**Parameters:**
- `kinematicViscosity` (Fusion.KinematicViscosity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.IsNegativeInfinity%28Fusion.KinematicViscosity%29)

#### `static bool IsPositiveInfinity(Fusion.KinematicViscosity kinematicViscosity)`

IsPositiveInfinity method of KinematicViscosity.

**Parameters:**
- `kinematicViscosity` (Fusion.KinematicViscosity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.IsPositiveInfinity%28Fusion.KinematicViscosity%29)

#### `static Fusion.KinematicViscosity Max(Fusion.KinematicViscosity a, Fusion.KinematicViscosity b)`

Max method of KinematicViscosity.

**Parameters:**
- `a` (Fusion.KinematicViscosity)
- `b` (Fusion.KinematicViscosity)

**Returns:** `Fusion.KinematicViscosity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.Max%28Fusion.KinematicViscosity%2CFusion.KinematicViscosity%29)

#### `static Fusion.KinematicViscosity Min(Fusion.KinematicViscosity a, Fusion.KinematicViscosity b)`

Min method of KinematicViscosity.

**Parameters:**
- `a` (Fusion.KinematicViscosity)
- `b` (Fusion.KinematicViscosity)

**Returns:** `Fusion.KinematicViscosity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.Min%28Fusion.KinematicViscosity%2CFusion.KinematicViscosity%29)

#### `bool Near(Fusion.KinematicViscosity other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of KinematicViscosity.

**Parameters:**
- `other` (Fusion.KinematicViscosity)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.Near%28Fusion.KinematicViscosity%2CSystem.Int32%29)

#### `static Fusion.KinematicViscosity Round(Fusion.KinematicViscosity kinematicViscosity, Fusion.KinematicViscosity precision, Fusion.RoundingMode roundingMode)`

Round method of KinematicViscosity.

**Parameters:**
- `kinematicViscosity` (Fusion.KinematicViscosity)
- `precision` (Fusion.KinematicViscosity)
- `roundingMode` (Fusion.RoundingMode)

**Returns:** `Fusion.KinematicViscosity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.Round%28Fusion.KinematicViscosity%2CFusion.KinematicViscosity%2CFusion.RoundingMode%29)

#### `static int Sign(Fusion.KinematicViscosity kinematicViscosity)`

Sign method of KinematicViscosity.

**Parameters:**
- `kinematicViscosity` (Fusion.KinematicViscosity)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.Sign%28Fusion.KinematicViscosity%29)

#### `double To(Fusion.KinematicViscosityUnit kinematicViscosityUnit)`

To method of KinematicViscosity.

**Parameters:**
- `kinematicViscosityUnit` (Fusion.KinematicViscosityUnit)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.To%28Fusion.KinematicViscosityUnit%29)

#### `string ToString()`

ToString method of KinematicViscosity.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of KinematicViscosity.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.KinematicViscosity op_Addition(Fusion.KinematicViscosity a, Fusion.KinematicViscosity b)`

op_Addition method of KinematicViscosity.

**Parameters:**
- `a` (Fusion.KinematicViscosity)
- `b` (Fusion.KinematicViscosity)

**Returns:** `Fusion.KinematicViscosity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.op_Addition%28Fusion.KinematicViscosity%2CFusion.KinematicViscosity%29)

#### `static Fusion.KinematicViscosity op_Division(Fusion.KinematicViscosity a, double b)`

op_Division method of KinematicViscosity.

**Parameters:**
- `a` (Fusion.KinematicViscosity)
- `b` (double)

**Returns:** `Fusion.KinematicViscosity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.op_Division%28Fusion.KinematicViscosity%2CSystem.Double%29)

#### `static double op_Division(Fusion.KinematicViscosity a, Fusion.KinematicViscosity b)`

op_Division method of KinematicViscosity.

**Parameters:**
- `a` (Fusion.KinematicViscosity)
- `b` (Fusion.KinematicViscosity)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.op_Division%28Fusion.KinematicViscosity%2CFusion.KinematicViscosity%29)

#### `static bool op_Equality(Fusion.KinematicViscosity a, Fusion.KinematicViscosity b)`

op_Equality method of KinematicViscosity.

**Parameters:**
- `a` (Fusion.KinematicViscosity)
- `b` (Fusion.KinematicViscosity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.op_Equality%28Fusion.KinematicViscosity%2CFusion.KinematicViscosity%29)

#### `static bool op_GreaterThan(Fusion.KinematicViscosity a, Fusion.KinematicViscosity b)`

op_GreaterThan method of KinematicViscosity.

**Parameters:**
- `a` (Fusion.KinematicViscosity)
- `b` (Fusion.KinematicViscosity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.op_GreaterThan%28Fusion.KinematicViscosity%2CFusion.KinematicViscosity%29)

#### `static bool op_GreaterThanOrEqual(Fusion.KinematicViscosity a, Fusion.KinematicViscosity b)`

op_GreaterThanOrEqual method of KinematicViscosity.

**Parameters:**
- `a` (Fusion.KinematicViscosity)
- `b` (Fusion.KinematicViscosity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.op_GreaterThanOrEqual%28Fusion.KinematicViscosity%2CFusion.KinematicViscosity%29)

#### `static bool op_Inequality(Fusion.KinematicViscosity a, Fusion.KinematicViscosity b)`

op_Inequality method of KinematicViscosity.

**Parameters:**
- `a` (Fusion.KinematicViscosity)
- `b` (Fusion.KinematicViscosity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.op_Inequality%28Fusion.KinematicViscosity%2CFusion.KinematicViscosity%29)

#### `static bool op_LessThan(Fusion.KinematicViscosity a, Fusion.KinematicViscosity b)`

op_LessThan method of KinematicViscosity.

**Parameters:**
- `a` (Fusion.KinematicViscosity)
- `b` (Fusion.KinematicViscosity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.op_LessThan%28Fusion.KinematicViscosity%2CFusion.KinematicViscosity%29)

#### `static bool op_LessThanOrEqual(Fusion.KinematicViscosity a, Fusion.KinematicViscosity b)`

op_LessThanOrEqual method of KinematicViscosity.

**Parameters:**
- `a` (Fusion.KinematicViscosity)
- `b` (Fusion.KinematicViscosity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.op_LessThanOrEqual%28Fusion.KinematicViscosity%2CFusion.KinematicViscosity%29)

#### `static Fusion.KinematicViscosity op_Modulus(Fusion.KinematicViscosity a, Fusion.KinematicViscosity b)`

op_Modulus method of KinematicViscosity.

**Parameters:**
- `a` (Fusion.KinematicViscosity)
- `b` (Fusion.KinematicViscosity)

**Returns:** `Fusion.KinematicViscosity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.op_Modulus%28Fusion.KinematicViscosity%2CFusion.KinematicViscosity%29)

#### `static Fusion.KinematicViscosity op_Multiply(Fusion.KinematicViscosity a, double b)`

op_Multiply method of KinematicViscosity.

**Parameters:**
- `a` (Fusion.KinematicViscosity)
- `b` (double)

**Returns:** `Fusion.KinematicViscosity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.op_Multiply%28Fusion.KinematicViscosity%2CSystem.Double%29)

#### `static Fusion.KinematicViscosity op_Multiply(double a, Fusion.KinematicViscosity b)`

op_Multiply method of KinematicViscosity.

**Parameters:**
- `a` (double)
- `b` (Fusion.KinematicViscosity)

**Returns:** `Fusion.KinematicViscosity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.op_Multiply%28System.Double%2CFusion.KinematicViscosity%29)

#### `static Fusion.KinematicViscosity op_Subtraction(Fusion.KinematicViscosity a, Fusion.KinematicViscosity b)`

op_Subtraction method of KinematicViscosity.

**Parameters:**
- `a` (Fusion.KinematicViscosity)
- `b` (Fusion.KinematicViscosity)

**Returns:** `Fusion.KinematicViscosity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.op_Subtraction%28Fusion.KinematicViscosity%2CFusion.KinematicViscosity%29)

#### `static Fusion.KinematicViscosity op_UnaryNegation(Fusion.KinematicViscosity a)`

op_UnaryNegation method of KinematicViscosity.

**Parameters:**
- `a` (Fusion.KinematicViscosity)

**Returns:** `Fusion.KinematicViscosity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.KinematicViscosity.op_UnaryNegation%28Fusion.KinematicViscosity%29)

### Properties
- `SquareMetersPerSecond` (double, get) — SquareMetersPerSecond property of KinematicViscosity.

## KinematicViscosityUnit (enum)

KinematicViscosityUnit enum in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.KinematicViscosityUnit)

### Values
- `SquareMetersPerSecond` = `0`
- `SquareFeetPerSecond` = `1`

## Length (struct)

Length struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Length)

### Constructors
- `Length(double millimeters)` — Constructs a new Length.

### Methods
#### `static Fusion.Length Abs(Fusion.Length length)`

Abs method of Length.

**Parameters:**
- `length` (Fusion.Length)

**Returns:** `Fusion.Length` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.Abs%28Fusion.Length%29)

#### `int CompareTo(Fusion.Length other)`

CompareTo method of Length.

**Parameters:**
- `other` (Fusion.Length)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.CompareTo%28Fusion.Length%29)

#### `int CompareTo(object other)`

CompareTo method of Length.

**Parameters:**
- `other` (object)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.CompareTo%28System.Object%29)

#### `bool Equals(Fusion.Length other)`

Equals method of Length.

**Parameters:**
- `other` (Fusion.Length)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.Equals%28Fusion.Length%29)

#### `bool Equals(object other)`

Equals method of Length.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.Equals%28System.Object%29)

#### `static Fusion.Length From(double length, Fusion.LengthUnit lengthUnit)`

From method of Length.

**Parameters:**
- `length` (double)
- `lengthUnit` (Fusion.LengthUnit)

**Returns:** `Fusion.Length` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.From%28System.Double%2CFusion.LengthUnit%29)

#### `int GetHashCode()`

GetHashCode method of Length.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.GetHashCode)

#### `static bool IsInfinity(Fusion.Length length)`

IsInfinity method of Length.

**Parameters:**
- `length` (Fusion.Length)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.IsInfinity%28Fusion.Length%29)

#### `static bool IsNaN(Fusion.Length length)`

IsNaN method of Length.

**Parameters:**
- `length` (Fusion.Length)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.IsNaN%28Fusion.Length%29)

#### `static bool IsNegativeInfinity(Fusion.Length length)`

IsNegativeInfinity method of Length.

**Parameters:**
- `length` (Fusion.Length)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.IsNegativeInfinity%28Fusion.Length%29)

#### `static bool IsPositiveInfinity(Fusion.Length length)`

IsPositiveInfinity method of Length.

**Parameters:**
- `length` (Fusion.Length)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.IsPositiveInfinity%28Fusion.Length%29)

#### `static Fusion.Length Max(Fusion.Length a, Fusion.Length b)`

Max method of Length.

**Parameters:**
- `a` (Fusion.Length)
- `b` (Fusion.Length)

**Returns:** `Fusion.Length` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.Max%28Fusion.Length%2CFusion.Length%29)

#### `static Fusion.Length Min(Fusion.Length a, Fusion.Length b)`

Min method of Length.

**Parameters:**
- `a` (Fusion.Length)
- `b` (Fusion.Length)

**Returns:** `Fusion.Length` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.Min%28Fusion.Length%2CFusion.Length%29)

#### `bool Near(Fusion.Length other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of Length.

**Parameters:**
- `other` (Fusion.Length)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.Near%28Fusion.Length%2CSystem.Int32%29)

#### `static Fusion.Length Round(Fusion.Length length, Fusion.Length precision, Fusion.RoundingMode roundingMode)`

Round method of Length.

**Parameters:**
- `length` (Fusion.Length)
- `precision` (Fusion.Length)
- `roundingMode` (Fusion.RoundingMode)

**Returns:** `Fusion.Length` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.Round%28Fusion.Length%2CFusion.Length%2CFusion.RoundingMode%29)

#### `static int Sign(Fusion.Length length)`

Sign method of Length.

**Parameters:**
- `length` (Fusion.Length)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.Sign%28Fusion.Length%29)

#### `double To(Fusion.LengthUnit lengthUnit)`

To method of Length.

**Parameters:**
- `lengthUnit` (Fusion.LengthUnit)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.To%28Fusion.LengthUnit%29)

#### `string ToString()`

ToString method of Length.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of Length.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.Length op_Addition(Fusion.Length a, Fusion.Length b)`

op_Addition method of Length.

**Parameters:**
- `a` (Fusion.Length)
- `b` (Fusion.Length)

**Returns:** `Fusion.Length` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.op_Addition%28Fusion.Length%2CFusion.Length%29)

#### `static Fusion.Length op_Division(Fusion.Length a, double b)`

op_Division method of Length.

**Parameters:**
- `a` (Fusion.Length)
- `b` (double)

**Returns:** `Fusion.Length` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.op_Division%28Fusion.Length%2CSystem.Double%29)

#### `static double op_Division(Fusion.Length a, Fusion.Length b)`

op_Division method of Length.

**Parameters:**
- `a` (Fusion.Length)
- `b` (Fusion.Length)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.op_Division%28Fusion.Length%2CFusion.Length%29)

#### `static bool op_Equality(Fusion.Length a, Fusion.Length b)`

op_Equality method of Length.

**Parameters:**
- `a` (Fusion.Length)
- `b` (Fusion.Length)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.op_Equality%28Fusion.Length%2CFusion.Length%29)

#### `static bool op_GreaterThan(Fusion.Length a, Fusion.Length b)`

op_GreaterThan method of Length.

**Parameters:**
- `a` (Fusion.Length)
- `b` (Fusion.Length)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.op_GreaterThan%28Fusion.Length%2CFusion.Length%29)

#### `static bool op_GreaterThanOrEqual(Fusion.Length a, Fusion.Length b)`

op_GreaterThanOrEqual method of Length.

**Parameters:**
- `a` (Fusion.Length)
- `b` (Fusion.Length)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.op_GreaterThanOrEqual%28Fusion.Length%2CFusion.Length%29)

#### `static bool op_Inequality(Fusion.Length a, Fusion.Length b)`

op_Inequality method of Length.

**Parameters:**
- `a` (Fusion.Length)
- `b` (Fusion.Length)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.op_Inequality%28Fusion.Length%2CFusion.Length%29)

#### `static bool op_LessThan(Fusion.Length a, Fusion.Length b)`

op_LessThan method of Length.

**Parameters:**
- `a` (Fusion.Length)
- `b` (Fusion.Length)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.op_LessThan%28Fusion.Length%2CFusion.Length%29)

#### `static bool op_LessThanOrEqual(Fusion.Length a, Fusion.Length b)`

op_LessThanOrEqual method of Length.

**Parameters:**
- `a` (Fusion.Length)
- `b` (Fusion.Length)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.op_LessThanOrEqual%28Fusion.Length%2CFusion.Length%29)

#### `static Fusion.Length op_Modulus(Fusion.Length a, Fusion.Length b)`

op_Modulus method of Length.

**Parameters:**
- `a` (Fusion.Length)
- `b` (Fusion.Length)

**Returns:** `Fusion.Length` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.op_Modulus%28Fusion.Length%2CFusion.Length%29)

#### `static Fusion.Length op_Multiply(Fusion.Length a, double b)`

op_Multiply method of Length.

**Parameters:**
- `a` (Fusion.Length)
- `b` (double)

**Returns:** `Fusion.Length` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.op_Multiply%28Fusion.Length%2CSystem.Double%29)

#### `static Fusion.Length op_Multiply(double a, Fusion.Length b)`

op_Multiply method of Length.

**Parameters:**
- `a` (double)
- `b` (Fusion.Length)

**Returns:** `Fusion.Length` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.op_Multiply%28System.Double%2CFusion.Length%29)

#### `static Fusion.Length op_Subtraction(Fusion.Length a, Fusion.Length b)`

op_Subtraction method of Length.

**Parameters:**
- `a` (Fusion.Length)
- `b` (Fusion.Length)

**Returns:** `Fusion.Length` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.op_Subtraction%28Fusion.Length%2CFusion.Length%29)

#### `static Fusion.Length op_UnaryNegation(Fusion.Length a)`

op_UnaryNegation method of Length.

**Parameters:**
- `a` (Fusion.Length)

**Returns:** `Fusion.Length` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Length.op_UnaryNegation%28Fusion.Length%29)

### Properties
- `Millimeters` (double, get) — Millimeters property of Length.

## Length4Units (enum)

Length4Units enum in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Length4Units)

### Values
- `Millimeter` = `0`
- `Centimeter` = `1`
- `Inch` = `2`

## LengthUnit (enum)

LengthUnit enum in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.LengthUnit)

### Values
- `Millimeter` = `0`
- `Centimeter` = `1`
- `Meter` = `2`
- `Inch` = `3`
- `Foot` = `4`
- `Yard` = `5`
- `Kilometer` = `6`
- `Mile` = `7`
- `USSurveyFoot` = `8`
- `USSurveyInch` = `9`
- `USSurveyYard` = `10`
- `USSurveyMile` = `11`

## LinearDensity (struct)

LinearDensity struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.LinearDensity)

### Constructors
- `LinearDensity(double kilogramPerMeter)` — Constructs a new LinearDensity.

### Methods
#### `static Fusion.LinearDensity Abs(Fusion.LinearDensity linearDensity)`

Abs method of LinearDensity.

**Parameters:**
- `linearDensity` (Fusion.LinearDensity)

**Returns:** `Fusion.LinearDensity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.Abs%28Fusion.LinearDensity%29)

#### `int CompareTo(Fusion.LinearDensity obj)`

CompareTo method of LinearDensity.

**Parameters:**
- `obj` (Fusion.LinearDensity)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.CompareTo%28Fusion.LinearDensity%29)

#### `int CompareTo(object obj)`

CompareTo method of LinearDensity.

**Parameters:**
- `obj` (object)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.CompareTo%28System.Object%29)

#### `bool Equals(Fusion.LinearDensity obj)`

Equals method of LinearDensity.

**Parameters:**
- `obj` (Fusion.LinearDensity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.Equals%28Fusion.LinearDensity%29)

#### `bool Equals(object obj)`

Equals method of LinearDensity.

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.Equals%28System.Object%29)

#### `static Fusion.LinearDensity From(double linearDensity, Fusion.LinearDensity.LinearDensityUnit massPerLengthUnit)`

From method of LinearDensity.

**Parameters:**
- `linearDensity` (double)
- `massPerLengthUnit` (Fusion.LinearDensity.LinearDensityUnit)

**Returns:** `Fusion.LinearDensity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.From%28System.Double%2CFusion.LinearDensity.LinearDensityUnit%29)

#### `int GetHashCode()`

GetHashCode method of LinearDensity.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.GetHashCode)

#### `static bool IsInfinity(Fusion.LinearDensity linearDensity)`

IsInfinity method of LinearDensity.

**Parameters:**
- `linearDensity` (Fusion.LinearDensity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.IsInfinity%28Fusion.LinearDensity%29)

#### `static bool IsNaN(Fusion.LinearDensity linearDensity)`

IsNaN method of LinearDensity.

**Parameters:**
- `linearDensity` (Fusion.LinearDensity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.IsNaN%28Fusion.LinearDensity%29)

#### `static bool IsNegativeInfinity(Fusion.LinearDensity linearDensity)`

IsNegativeInfinity method of LinearDensity.

**Parameters:**
- `linearDensity` (Fusion.LinearDensity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.IsNegativeInfinity%28Fusion.LinearDensity%29)

#### `static bool IsPositiveInfinity(Fusion.LinearDensity linearDensity)`

IsPositiveInfinity method of LinearDensity.

**Parameters:**
- `linearDensity` (Fusion.LinearDensity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.IsPositiveInfinity%28Fusion.LinearDensity%29)

#### `static Fusion.LinearDensity Max(Fusion.LinearDensity a, Fusion.LinearDensity b)`

Max method of LinearDensity.

**Parameters:**
- `a` (Fusion.LinearDensity)
- `b` (Fusion.LinearDensity)

**Returns:** `Fusion.LinearDensity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.Max%28Fusion.LinearDensity%2CFusion.LinearDensity%29)

#### `static Fusion.LinearDensity Min(Fusion.LinearDensity a, Fusion.LinearDensity b)`

Min method of LinearDensity.

**Parameters:**
- `a` (Fusion.LinearDensity)
- `b` (Fusion.LinearDensity)

**Returns:** `Fusion.LinearDensity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.Min%28Fusion.LinearDensity%2CFusion.LinearDensity%29)

#### `bool Near(Fusion.LinearDensity other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of LinearDensity.

**Parameters:**
- `other` (Fusion.LinearDensity)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.Near%28Fusion.LinearDensity%2CSystem.Int32%29)

#### `static Fusion.LinearDensity Round(Fusion.LinearDensity linearDensity, Fusion.LinearDensity precision, Fusion.RoundingMode roundingMode)`

Round method of LinearDensity.

**Parameters:**
- `linearDensity` (Fusion.LinearDensity)
- `precision` (Fusion.LinearDensity)
- `roundingMode` (Fusion.RoundingMode)

**Returns:** `Fusion.LinearDensity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.Round%28Fusion.LinearDensity%2CFusion.LinearDensity%2CFusion.RoundingMode%29)

#### `static int Sign(Fusion.LinearDensity linearDensity)`

Sign method of LinearDensity.

**Parameters:**
- `linearDensity` (Fusion.LinearDensity)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.Sign%28Fusion.LinearDensity%29)

#### `double To(Fusion.LinearDensity.LinearDensityUnit linearDensityUnit)`

To method of LinearDensity.

**Parameters:**
- `linearDensityUnit` (Fusion.LinearDensity.LinearDensityUnit)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.To%28Fusion.LinearDensity.LinearDensityUnit%29)

#### `string ToString()`

ToString method of LinearDensity.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of LinearDensity.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.LinearDensity op_Addition(Fusion.LinearDensity a, Fusion.LinearDensity b)`

op_Addition method of LinearDensity.

**Parameters:**
- `a` (Fusion.LinearDensity)
- `b` (Fusion.LinearDensity)

**Returns:** `Fusion.LinearDensity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.op_Addition%28Fusion.LinearDensity%2CFusion.LinearDensity%29)

#### `static Fusion.LinearDensity op_Division(Fusion.LinearDensity a, double b)`

op_Division method of LinearDensity.

**Parameters:**
- `a` (Fusion.LinearDensity)
- `b` (double)

**Returns:** `Fusion.LinearDensity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.op_Division%28Fusion.LinearDensity%2CSystem.Double%29)

#### `static double op_Division(Fusion.LinearDensity a, Fusion.LinearDensity b)`

op_Division method of LinearDensity.

**Parameters:**
- `a` (Fusion.LinearDensity)
- `b` (Fusion.LinearDensity)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.op_Division%28Fusion.LinearDensity%2CFusion.LinearDensity%29)

#### `static bool op_Equality(Fusion.LinearDensity a, Fusion.LinearDensity b)`

op_Equality method of LinearDensity.

**Parameters:**
- `a` (Fusion.LinearDensity)
- `b` (Fusion.LinearDensity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.op_Equality%28Fusion.LinearDensity%2CFusion.LinearDensity%29)

#### `static bool op_GreaterThan(Fusion.LinearDensity a, Fusion.LinearDensity b)`

op_GreaterThan method of LinearDensity.

**Parameters:**
- `a` (Fusion.LinearDensity)
- `b` (Fusion.LinearDensity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.op_GreaterThan%28Fusion.LinearDensity%2CFusion.LinearDensity%29)

#### `static bool op_GreaterThanOrEqual(Fusion.LinearDensity a, Fusion.LinearDensity b)`

op_GreaterThanOrEqual method of LinearDensity.

**Parameters:**
- `a` (Fusion.LinearDensity)
- `b` (Fusion.LinearDensity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.op_GreaterThanOrEqual%28Fusion.LinearDensity%2CFusion.LinearDensity%29)

#### `static bool op_Inequality(Fusion.LinearDensity a, Fusion.LinearDensity b)`

op_Inequality method of LinearDensity.

**Parameters:**
- `a` (Fusion.LinearDensity)
- `b` (Fusion.LinearDensity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.op_Inequality%28Fusion.LinearDensity%2CFusion.LinearDensity%29)

#### `static bool op_LessThan(Fusion.LinearDensity a, Fusion.LinearDensity b)`

op_LessThan method of LinearDensity.

**Parameters:**
- `a` (Fusion.LinearDensity)
- `b` (Fusion.LinearDensity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.op_LessThan%28Fusion.LinearDensity%2CFusion.LinearDensity%29)

#### `static bool op_LessThanOrEqual(Fusion.LinearDensity a, Fusion.LinearDensity b)`

op_LessThanOrEqual method of LinearDensity.

**Parameters:**
- `a` (Fusion.LinearDensity)
- `b` (Fusion.LinearDensity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.op_LessThanOrEqual%28Fusion.LinearDensity%2CFusion.LinearDensity%29)

#### `static Fusion.LinearDensity op_Modulus(Fusion.LinearDensity a, Fusion.LinearDensity b)`

op_Modulus method of LinearDensity.

**Parameters:**
- `a` (Fusion.LinearDensity)
- `b` (Fusion.LinearDensity)

**Returns:** `Fusion.LinearDensity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.op_Modulus%28Fusion.LinearDensity%2CFusion.LinearDensity%29)

#### `static Fusion.LinearDensity op_Multiply(Fusion.LinearDensity a, double b)`

op_Multiply method of LinearDensity.

**Parameters:**
- `a` (Fusion.LinearDensity)
- `b` (double)

**Returns:** `Fusion.LinearDensity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.op_Multiply%28Fusion.LinearDensity%2CSystem.Double%29)

#### `static Fusion.LinearDensity op_Multiply(double a, Fusion.LinearDensity b)`

op_Multiply method of LinearDensity.

**Parameters:**
- `a` (double)
- `b` (Fusion.LinearDensity)

**Returns:** `Fusion.LinearDensity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.op_Multiply%28System.Double%2CFusion.LinearDensity%29)

#### `static Fusion.LinearDensity op_Subtraction(Fusion.LinearDensity a, Fusion.LinearDensity b)`

op_Subtraction method of LinearDensity.

**Parameters:**
- `a` (Fusion.LinearDensity)
- `b` (Fusion.LinearDensity)

**Returns:** `Fusion.LinearDensity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.op_Subtraction%28Fusion.LinearDensity%2CFusion.LinearDensity%29)

#### `static Fusion.LinearDensity op_UnaryNegation(Fusion.LinearDensity a)`

op_UnaryNegation method of LinearDensity.

**Parameters:**
- `a` (Fusion.LinearDensity)

**Returns:** `Fusion.LinearDensity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.LinearDensity.op_UnaryNegation%28Fusion.LinearDensity%29)

### Properties
- `KilogramsPerMeter` (double, get) — KilogramsPerMeter property of LinearDensity.

## LocalizationMode (enum)

LocalizationMode enum in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.LocalizationMode)

### Values
- `Normal` = `0`
- `Keys` = `1`
- `Brackets` = `2`
- `ShortString` = `3`
- `LongStrings` = `4`
- `UnicodeTest` = `5`

## LocalizedResourcesAttribute (class)

LocalizedResourcesAttribute class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.LocalizedResourcesAttribute)

### Constructors
- `LocalizedResourcesAttribute(string resourcePath)` — Constructs a new LocalizedResourcesAttribute.

## Mass (struct)

Mass struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Mass)

### Constructors
- `Mass(double kilograms)` — Constructs a new Mass.

### Methods
#### `static Fusion.Mass Abs(Fusion.Mass mass)`

Abs method of Mass.

**Parameters:**
- `mass` (Fusion.Mass)

**Returns:** `Fusion.Mass` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.Abs%28Fusion.Mass%29)

#### `int CompareTo(Fusion.Mass other)`

CompareTo method of Mass.

**Parameters:**
- `other` (Fusion.Mass)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.CompareTo%28Fusion.Mass%29)

#### `int CompareTo(object other)`

CompareTo method of Mass.

**Parameters:**
- `other` (object)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.CompareTo%28System.Object%29)

#### `bool Equals(Fusion.Mass other)`

Equals method of Mass.

**Parameters:**
- `other` (Fusion.Mass)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.Equals%28Fusion.Mass%29)

#### `bool Equals(object other)`

Equals method of Mass.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.Equals%28System.Object%29)

#### `static Fusion.Mass From(double mass, Fusion.MassUnit massUnit)`

From method of Mass.

**Parameters:**
- `mass` (double)
- `massUnit` (Fusion.MassUnit)

**Returns:** `Fusion.Mass` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.From%28System.Double%2CFusion.MassUnit%29)

#### `int GetHashCode()`

GetHashCode method of Mass.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.GetHashCode)

#### `static bool IsInfinity(Fusion.Mass mass)`

IsInfinity method of Mass.

**Parameters:**
- `mass` (Fusion.Mass)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.IsInfinity%28Fusion.Mass%29)

#### `static bool IsNaN(Fusion.Mass mass)`

IsNaN method of Mass.

**Parameters:**
- `mass` (Fusion.Mass)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.IsNaN%28Fusion.Mass%29)

#### `static bool IsNegativeInfinity(Fusion.Mass mass)`

IsNegativeInfinity method of Mass.

**Parameters:**
- `mass` (Fusion.Mass)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.IsNegativeInfinity%28Fusion.Mass%29)

#### `static bool IsPositiveInfinity(Fusion.Mass mass)`

IsPositiveInfinity method of Mass.

**Parameters:**
- `mass` (Fusion.Mass)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.IsPositiveInfinity%28Fusion.Mass%29)

#### `static Fusion.Mass Max(Fusion.Mass a, Fusion.Mass b)`

Max method of Mass.

**Parameters:**
- `a` (Fusion.Mass)
- `b` (Fusion.Mass)

**Returns:** `Fusion.Mass` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.Max%28Fusion.Mass%2CFusion.Mass%29)

#### `static Fusion.Mass Min(Fusion.Mass a, Fusion.Mass b)`

Min method of Mass.

**Parameters:**
- `a` (Fusion.Mass)
- `b` (Fusion.Mass)

**Returns:** `Fusion.Mass` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.Min%28Fusion.Mass%2CFusion.Mass%29)

#### `bool Near(Fusion.Mass other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of Mass.

**Parameters:**
- `other` (Fusion.Mass)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.Near%28Fusion.Mass%2CSystem.Int32%29)

#### `static Fusion.Mass Round(Fusion.Mass mass, Fusion.Mass precision, Fusion.RoundingMode roundingMode)`

Round method of Mass.

**Parameters:**
- `mass` (Fusion.Mass)
- `precision` (Fusion.Mass)
- `roundingMode` (Fusion.RoundingMode)

**Returns:** `Fusion.Mass` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.Round%28Fusion.Mass%2CFusion.Mass%2CFusion.RoundingMode%29)

#### `static int Sign(Fusion.Mass mass)`

Sign method of Mass.

**Parameters:**
- `mass` (Fusion.Mass)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.Sign%28Fusion.Mass%29)

#### `double To(Fusion.MassUnit massUnit)`

To method of Mass.

**Parameters:**
- `massUnit` (Fusion.MassUnit)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.To%28Fusion.MassUnit%29)

#### `string ToString()`

ToString method of Mass.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of Mass.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.Mass op_Addition(Fusion.Mass a, Fusion.Mass b)`

op_Addition method of Mass.

**Parameters:**
- `a` (Fusion.Mass)
- `b` (Fusion.Mass)

**Returns:** `Fusion.Mass` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.op_Addition%28Fusion.Mass%2CFusion.Mass%29)

#### `static Fusion.Mass op_Division(Fusion.Mass a, double b)`

op_Division method of Mass.

**Parameters:**
- `a` (Fusion.Mass)
- `b` (double)

**Returns:** `Fusion.Mass` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.op_Division%28Fusion.Mass%2CSystem.Double%29)

#### `static double op_Division(Fusion.Mass a, Fusion.Mass b)`

op_Division method of Mass.

**Parameters:**
- `a` (Fusion.Mass)
- `b` (Fusion.Mass)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.op_Division%28Fusion.Mass%2CFusion.Mass%29)

#### `static bool op_Equality(Fusion.Mass a, Fusion.Mass b)`

op_Equality method of Mass.

**Parameters:**
- `a` (Fusion.Mass)
- `b` (Fusion.Mass)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.op_Equality%28Fusion.Mass%2CFusion.Mass%29)

#### `static bool op_GreaterThan(Fusion.Mass a, Fusion.Mass b)`

op_GreaterThan method of Mass.

**Parameters:**
- `a` (Fusion.Mass)
- `b` (Fusion.Mass)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.op_GreaterThan%28Fusion.Mass%2CFusion.Mass%29)

#### `static bool op_GreaterThanOrEqual(Fusion.Mass a, Fusion.Mass b)`

op_GreaterThanOrEqual method of Mass.

**Parameters:**
- `a` (Fusion.Mass)
- `b` (Fusion.Mass)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.op_GreaterThanOrEqual%28Fusion.Mass%2CFusion.Mass%29)

#### `static bool op_Inequality(Fusion.Mass a, Fusion.Mass b)`

op_Inequality method of Mass.

**Parameters:**
- `a` (Fusion.Mass)
- `b` (Fusion.Mass)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.op_Inequality%28Fusion.Mass%2CFusion.Mass%29)

#### `static bool op_LessThan(Fusion.Mass a, Fusion.Mass b)`

op_LessThan method of Mass.

**Parameters:**
- `a` (Fusion.Mass)
- `b` (Fusion.Mass)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.op_LessThan%28Fusion.Mass%2CFusion.Mass%29)

#### `static bool op_LessThanOrEqual(Fusion.Mass a, Fusion.Mass b)`

op_LessThanOrEqual method of Mass.

**Parameters:**
- `a` (Fusion.Mass)
- `b` (Fusion.Mass)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.op_LessThanOrEqual%28Fusion.Mass%2CFusion.Mass%29)

#### `static Fusion.Mass op_Modulus(Fusion.Mass a, Fusion.Mass b)`

op_Modulus method of Mass.

**Parameters:**
- `a` (Fusion.Mass)
- `b` (Fusion.Mass)

**Returns:** `Fusion.Mass` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.op_Modulus%28Fusion.Mass%2CFusion.Mass%29)

#### `static Fusion.Mass op_Multiply(Fusion.Mass a, double b)`

op_Multiply method of Mass.

**Parameters:**
- `a` (Fusion.Mass)
- `b` (double)

**Returns:** `Fusion.Mass` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.op_Multiply%28Fusion.Mass%2CSystem.Double%29)

#### `static Fusion.Mass op_Multiply(double a, Fusion.Mass b)`

op_Multiply method of Mass.

**Parameters:**
- `a` (double)
- `b` (Fusion.Mass)

**Returns:** `Fusion.Mass` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.op_Multiply%28System.Double%2CFusion.Mass%29)

#### `static Fusion.Mass op_Subtraction(Fusion.Mass a, Fusion.Mass b)`

op_Subtraction method of Mass.

**Parameters:**
- `a` (Fusion.Mass)
- `b` (Fusion.Mass)

**Returns:** `Fusion.Mass` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.op_Subtraction%28Fusion.Mass%2CFusion.Mass%29)

#### `static Fusion.Mass op_UnaryNegation(Fusion.Mass a)`

op_UnaryNegation method of Mass.

**Parameters:**
- `a` (Fusion.Mass)

**Returns:** `Fusion.Mass` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Mass.op_UnaryNegation%28Fusion.Mass%29)

### Properties
- `Kilograms` (double, get) — Kilograms property of Mass.

## MassFlow (struct)

MassFlow struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.MassFlow)

### Constructors
- `MassFlow(double kilogramsPerSecond)` — Constructs a new MassFlow.

### Methods
#### `static Fusion.MassFlow Abs(Fusion.MassFlow massFlow)`

Abs method of MassFlow.

**Parameters:**
- `massFlow` (Fusion.MassFlow)

**Returns:** `Fusion.MassFlow` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.Abs%28Fusion.MassFlow%29)

#### `int CompareTo(Fusion.MassFlow other)`

CompareTo method of MassFlow.

**Parameters:**
- `other` (Fusion.MassFlow)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.CompareTo%28Fusion.MassFlow%29)

#### `int CompareTo(object other)`

CompareTo method of MassFlow.

**Parameters:**
- `other` (object)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.CompareTo%28System.Object%29)

#### `bool Equals(Fusion.MassFlow other)`

Equals method of MassFlow.

**Parameters:**
- `other` (Fusion.MassFlow)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.Equals%28Fusion.MassFlow%29)

#### `bool Equals(object other)`

Equals method of MassFlow.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.Equals%28System.Object%29)

#### `static Fusion.MassFlow From(double massFlow, Fusion.MassFlowUnit massFlowUnit)`

From method of MassFlow.

**Parameters:**
- `massFlow` (double)
- `massFlowUnit` (Fusion.MassFlowUnit)

**Returns:** `Fusion.MassFlow` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.From%28System.Double%2CFusion.MassFlowUnit%29)

#### `int GetHashCode()`

GetHashCode method of MassFlow.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.GetHashCode)

#### `static bool IsInfinity(Fusion.MassFlow massFlow)`

IsInfinity method of MassFlow.

**Parameters:**
- `massFlow` (Fusion.MassFlow)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.IsInfinity%28Fusion.MassFlow%29)

#### `static bool IsNaN(Fusion.MassFlow massFlow)`

IsNaN method of MassFlow.

**Parameters:**
- `massFlow` (Fusion.MassFlow)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.IsNaN%28Fusion.MassFlow%29)

#### `static bool IsNegativeInfinity(Fusion.MassFlow massFlow)`

IsNegativeInfinity method of MassFlow.

**Parameters:**
- `massFlow` (Fusion.MassFlow)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.IsNegativeInfinity%28Fusion.MassFlow%29)

#### `static bool IsPositiveInfinity(Fusion.MassFlow massFlow)`

IsPositiveInfinity method of MassFlow.

**Parameters:**
- `massFlow` (Fusion.MassFlow)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.IsPositiveInfinity%28Fusion.MassFlow%29)

#### `static Fusion.MassFlow Max(Fusion.MassFlow a, Fusion.MassFlow b)`

Max method of MassFlow.

**Parameters:**
- `a` (Fusion.MassFlow)
- `b` (Fusion.MassFlow)

**Returns:** `Fusion.MassFlow` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.Max%28Fusion.MassFlow%2CFusion.MassFlow%29)

#### `static Fusion.MassFlow Min(Fusion.MassFlow a, Fusion.MassFlow b)`

Min method of MassFlow.

**Parameters:**
- `a` (Fusion.MassFlow)
- `b` (Fusion.MassFlow)

**Returns:** `Fusion.MassFlow` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.Min%28Fusion.MassFlow%2CFusion.MassFlow%29)

#### `bool Near(Fusion.MassFlow other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of MassFlow.

**Parameters:**
- `other` (Fusion.MassFlow)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.Near%28Fusion.MassFlow%2CSystem.Int32%29)

#### `static Fusion.MassFlow Round(Fusion.MassFlow massFlow, Fusion.MassFlow precision, Fusion.RoundingMode roundingMode)`

Round method of MassFlow.

**Parameters:**
- `massFlow` (Fusion.MassFlow)
- `precision` (Fusion.MassFlow)
- `roundingMode` (Fusion.RoundingMode)

**Returns:** `Fusion.MassFlow` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.Round%28Fusion.MassFlow%2CFusion.MassFlow%2CFusion.RoundingMode%29)

#### `static int Sign(Fusion.MassFlow massFlow)`

Sign method of MassFlow.

**Parameters:**
- `massFlow` (Fusion.MassFlow)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.Sign%28Fusion.MassFlow%29)

#### `double To(Fusion.MassFlowUnit massFlowUnit)`

To method of MassFlow.

**Parameters:**
- `massFlowUnit` (Fusion.MassFlowUnit)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.To%28Fusion.MassFlowUnit%29)

#### `string ToString()`

ToString method of MassFlow.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of MassFlow.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.MassFlow op_Addition(Fusion.MassFlow a, Fusion.MassFlow b)`

op_Addition method of MassFlow.

**Parameters:**
- `a` (Fusion.MassFlow)
- `b` (Fusion.MassFlow)

**Returns:** `Fusion.MassFlow` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.op_Addition%28Fusion.MassFlow%2CFusion.MassFlow%29)

#### `static Fusion.MassFlow op_Division(Fusion.MassFlow a, double b)`

op_Division method of MassFlow.

**Parameters:**
- `a` (Fusion.MassFlow)
- `b` (double)

**Returns:** `Fusion.MassFlow` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.op_Division%28Fusion.MassFlow%2CSystem.Double%29)

#### `static double op_Division(Fusion.MassFlow a, Fusion.MassFlow b)`

op_Division method of MassFlow.

**Parameters:**
- `a` (Fusion.MassFlow)
- `b` (Fusion.MassFlow)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.op_Division%28Fusion.MassFlow%2CFusion.MassFlow%29)

#### `static bool op_Equality(Fusion.MassFlow a, Fusion.MassFlow b)`

op_Equality method of MassFlow.

**Parameters:**
- `a` (Fusion.MassFlow)
- `b` (Fusion.MassFlow)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.op_Equality%28Fusion.MassFlow%2CFusion.MassFlow%29)

#### `static bool op_GreaterThan(Fusion.MassFlow a, Fusion.MassFlow b)`

op_GreaterThan method of MassFlow.

**Parameters:**
- `a` (Fusion.MassFlow)
- `b` (Fusion.MassFlow)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.op_GreaterThan%28Fusion.MassFlow%2CFusion.MassFlow%29)

#### `static bool op_GreaterThanOrEqual(Fusion.MassFlow a, Fusion.MassFlow b)`

op_GreaterThanOrEqual method of MassFlow.

**Parameters:**
- `a` (Fusion.MassFlow)
- `b` (Fusion.MassFlow)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.op_GreaterThanOrEqual%28Fusion.MassFlow%2CFusion.MassFlow%29)

#### `static bool op_Inequality(Fusion.MassFlow a, Fusion.MassFlow b)`

op_Inequality method of MassFlow.

**Parameters:**
- `a` (Fusion.MassFlow)
- `b` (Fusion.MassFlow)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.op_Inequality%28Fusion.MassFlow%2CFusion.MassFlow%29)

#### `static bool op_LessThan(Fusion.MassFlow a, Fusion.MassFlow b)`

op_LessThan method of MassFlow.

**Parameters:**
- `a` (Fusion.MassFlow)
- `b` (Fusion.MassFlow)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.op_LessThan%28Fusion.MassFlow%2CFusion.MassFlow%29)

#### `static bool op_LessThanOrEqual(Fusion.MassFlow a, Fusion.MassFlow b)`

op_LessThanOrEqual method of MassFlow.

**Parameters:**
- `a` (Fusion.MassFlow)
- `b` (Fusion.MassFlow)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.op_LessThanOrEqual%28Fusion.MassFlow%2CFusion.MassFlow%29)

#### `static Fusion.MassFlow op_Modulus(Fusion.MassFlow a, Fusion.MassFlow b)`

op_Modulus method of MassFlow.

**Parameters:**
- `a` (Fusion.MassFlow)
- `b` (Fusion.MassFlow)

**Returns:** `Fusion.MassFlow` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.op_Modulus%28Fusion.MassFlow%2CFusion.MassFlow%29)

#### `static Fusion.MassFlow op_Multiply(Fusion.MassFlow a, double b)`

op_Multiply method of MassFlow.

**Parameters:**
- `a` (Fusion.MassFlow)
- `b` (double)

**Returns:** `Fusion.MassFlow` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.op_Multiply%28Fusion.MassFlow%2CSystem.Double%29)

#### `static Fusion.MassFlow op_Multiply(double a, Fusion.MassFlow b)`

op_Multiply method of MassFlow.

**Parameters:**
- `a` (double)
- `b` (Fusion.MassFlow)

**Returns:** `Fusion.MassFlow` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.op_Multiply%28System.Double%2CFusion.MassFlow%29)

#### `static Fusion.MassFlow op_Subtraction(Fusion.MassFlow a, Fusion.MassFlow b)`

op_Subtraction method of MassFlow.

**Parameters:**
- `a` (Fusion.MassFlow)
- `b` (Fusion.MassFlow)

**Returns:** `Fusion.MassFlow` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.op_Subtraction%28Fusion.MassFlow%2CFusion.MassFlow%29)

#### `static Fusion.MassFlow op_UnaryNegation(Fusion.MassFlow a)`

op_UnaryNegation method of MassFlow.

**Parameters:**
- `a` (Fusion.MassFlow)

**Returns:** `Fusion.MassFlow` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.MassFlow.op_UnaryNegation%28Fusion.MassFlow%29)

### Properties
- `KilogramsPerSecond` (double, get) — KilogramsPerSecond property of MassFlow.

## MassFlowUnit (enum)

MassFlowUnit enum in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.MassFlowUnit)

### Values
- `KilogramsPerSecond` = `0`
- `PoundsPerSecond` = `1`
- `KilogramsPerHour` = `2`

## MassUnit (enum)

MassUnit enum in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.MassUnit)

### Values
- `Kilogram` = `0`
- `Tonne` = `1`
- `Pound` = `2`
- `ShortTon` = `3`
- `Ounce` = `4`
- `Milligram` = `5`
- `Gram` = `6`

## Matrix (static-class)

Matrix static class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Matrix)

### Methods
#### `static Fusion.Float4x4 Camera(Fusion.Float3 cameraPosition, Fusion.Float3 cameraUpVector, Fusion.Float3 cameraForwardVector)`

Camera method of Matrix.

**Parameters:**
- `cameraPosition` (Fusion.Float3)
- `cameraUpVector` (Fusion.Float3)
- `cameraForwardVector` (Fusion.Float3)

**Returns:** `Fusion.Float4x4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Matrix.Camera%28Fusion.Float3%2CFusion.Float3%2CFusion.Float3%29)

#### `static Fusion.Float4x4 Invert(Fusion.Float4x4 matrix)`

Invert method of Matrix.

**Parameters:**
- `matrix` (Fusion.Float4x4)

**Returns:** `Fusion.Float4x4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Matrix.Invert%28Fusion.Float4x4%29)

#### `static Fusion.Float4x4 InvertAffine(Fusion.Float4x4 matrix)`

InvertAffine method of Matrix.

**Parameters:**
- `matrix` (Fusion.Float4x4)

**Returns:** `Fusion.Float4x4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Matrix.InvertAffine%28Fusion.Float4x4%29)

#### `static Fusion.Float4x4 LookAt(Fusion.Float3 targetPosition, Fusion.Float3 cameraPosition, Fusion.Float3 cameraUpVector)`

LookAt method of Matrix.

**Parameters:**
- `targetPosition` (Fusion.Float3)
- `cameraPosition` (Fusion.Float3)
- `cameraUpVector` (Fusion.Float3)

**Returns:** `Fusion.Float4x4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Matrix.LookAt%28Fusion.Float3%2CFusion.Float3%2CFusion.Float3%29)

#### `static Fusion.Float4x4 Object(Fusion.Float3 objectPosition, Fusion.Float3 objectUpVector, Fusion.Float3 objectForwardVector)`

Object method of Matrix.

**Parameters:**
- `objectPosition` (Fusion.Float3)
- `objectUpVector` (Fusion.Float3)
- `objectForwardVector` (Fusion.Float3)

**Returns:** `Fusion.Float4x4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Matrix.Object%28Fusion.Float3%2CFusion.Float3%2CFusion.Float3%29)

#### `static Fusion.Float4x4 Projection(float width, float height, Fusion.Angle verticalViewAngle, float scale, float minimumDistance, float maximumDistance)`

Projection method of Matrix.

**Parameters:**
- `width` (float)
- `height` (float)
- `verticalViewAngle` (Fusion.Angle)
- `scale` (float)
- `minimumDistance` (float)
- `maximumDistance` (float)

**Returns:** `Fusion.Float4x4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Matrix.Projection%28System.Single%2CSystem.Single%2CFusion.Angle%2CSystem.Single%2CSystem.Single%2CSystem.Single%29)

#### `static Fusion.Float4x4 Rotate(Fusion.Angle angle, Fusion.Float3 axis)`

Rotate method of Matrix.

**Parameters:**
- `angle` (Fusion.Angle)
- `axis` (Fusion.Float3)

**Returns:** `Fusion.Float4x4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Matrix.Rotate%28Fusion.Angle%2CFusion.Float3%29)

#### `static Fusion.Float4x4 Rotate(Fusion.Angle yaw, Fusion.Angle pitch, Fusion.Angle roll)`

Rotate method of Matrix.

**Parameters:**
- `yaw` (Fusion.Angle)
- `pitch` (Fusion.Angle)
- `roll` (Fusion.Angle)

**Returns:** `Fusion.Float4x4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Matrix.Rotate%28Fusion.Angle%2CFusion.Angle%2CFusion.Angle%29)

#### `static Fusion.Float4x4 RotateX(Fusion.Angle angle)`

RotateX method of Matrix.

**Parameters:**
- `angle` (Fusion.Angle)

**Returns:** `Fusion.Float4x4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Matrix.RotateX%28Fusion.Angle%29)

#### `static Fusion.Float4x4 RotateY(Fusion.Angle angle)`

RotateY method of Matrix.

**Parameters:**
- `angle` (Fusion.Angle)

**Returns:** `Fusion.Float4x4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Matrix.RotateY%28Fusion.Angle%29)

#### `static Fusion.Float4x4 RotateZ(Fusion.Angle angle)`

RotateZ method of Matrix.

**Parameters:**
- `angle` (Fusion.Angle)

**Returns:** `Fusion.Float4x4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Matrix.RotateZ%28Fusion.Angle%29)

#### `static Fusion.Float4 Transform(Fusion.Float4x4 matrix, Fusion.Float4 vector)`

Transform method of Matrix.

**Parameters:**
- `matrix` (Fusion.Float4x4)
- `vector` (Fusion.Float4)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Matrix.Transform%28Fusion.Float4x4%2CFusion.Float4%29)

#### `static Fusion.Float4x4 Translate(Fusion.Float3 translation)`

Translate method of Matrix.

**Parameters:**
- `translation` (Fusion.Float3)

**Returns:** `Fusion.Float4x4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Matrix.Translate%28Fusion.Float3%29)

#### `static Fusion.Float4x4 Translate(float x, float y, float z)`

Translate method of Matrix.

**Parameters:**
- `x` (float)
- `y` (float)
- `z` (float)

**Returns:** `Fusion.Float4x4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Matrix.Translate%28System.Single%2CSystem.Single%2CSystem.Single%29)

#### `static Fusion.Float4x4 Transpose(Fusion.Float4x4 matrix)`

Transpose method of Matrix.

**Parameters:**
- `matrix` (Fusion.Float4x4)

**Returns:** `Fusion.Float4x4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Matrix.Transpose%28Fusion.Float4x4%29)

## Maybe`1 (struct)

Maybe`1 struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Maybe`1)

### Constructors
- `Maybe`1(T value)` — Constructs a new Maybe`1.

### Methods
#### `bool Equals(Fusion.Maybe<T> other)`

Equals method of Maybe`1.

**Parameters:**
- `other` (Fusion.Maybe<T>)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Maybe%601.Equals%28Fusion.Maybe%7B%600%7D%29)

#### `bool Equals(object other)`

Equals method of Maybe`1.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Maybe%601.Equals%28System.Object%29)

#### `int GetHashCode()`

GetHashCode method of Maybe`1.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Maybe%601.GetHashCode)

#### `string ToString()`

ToString method of Maybe`1.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Maybe%601.ToString)

#### `static bool op_Equality(Fusion.Maybe<T> a, Fusion.Maybe<T> b)`

op_Equality method of Maybe`1.

**Parameters:**
- `a` (Fusion.Maybe<T>)
- `b` (Fusion.Maybe<T>)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Maybe%601.op_Equality%28Fusion.Maybe%7B%600%7D%2CFusion.Maybe%7B%600%7D%29)

#### `static bool op_Equality(Fusion.Maybe<T> a, T b)`

op_Equality method of Maybe`1.

**Parameters:**
- `a` (Fusion.Maybe<T>)
- `b` (T)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Maybe%601.op_Equality%28Fusion.Maybe%7B%600%7D%2C%600%29)

#### `static bool op_Equality(T a, Fusion.Maybe<T> b)`

op_Equality method of Maybe`1.

**Parameters:**
- `a` (T)
- `b` (Fusion.Maybe<T>)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Maybe%601.op_Equality%28%600%2CFusion.Maybe%7B%600%7D%29)

#### `static bool op_Inequality(Fusion.Maybe<T> a, Fusion.Maybe<T> b)`

op_Inequality method of Maybe`1.

**Parameters:**
- `a` (Fusion.Maybe<T>)
- `b` (Fusion.Maybe<T>)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Maybe%601.op_Inequality%28Fusion.Maybe%7B%600%7D%2CFusion.Maybe%7B%600%7D%29)

#### `static bool op_Inequality(Fusion.Maybe<T> a, T b)`

op_Inequality method of Maybe`1.

**Parameters:**
- `a` (Fusion.Maybe<T>)
- `b` (T)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Maybe%601.op_Inequality%28Fusion.Maybe%7B%600%7D%2C%600%29)

#### `static bool op_Inequality(T a, Fusion.Maybe<T> b)`

op_Inequality method of Maybe`1.

**Parameters:**
- `a` (T)
- `b` (Fusion.Maybe<T>)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Maybe%601.op_Inequality%28%600%2CFusion.Maybe%7B%600%7D%29)

### Properties
- `HasValue` (bool, get) — HasValue property of Maybe`1.
- `Value` (T, get) — Value property of Maybe`1.

## NavigationViewModel (class)

NavigationViewModel class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.NavigationViewModel)

### Methods
#### `bool NavigatingBack()`

NavigatingBack method of NavigationViewModel.

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.NavigationViewModel.NavigatingBack)

#### `void NavigatingHere()`

NavigatingHere method of NavigationViewModel.

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.NavigationViewModel.NavigatingHere)

### Properties
- `Navigation` (Fusion.INavigationController, get/set) — Navigation property of NavigationViewModel.

## PerformanceMetric (class)

PerformanceMetric class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.PerformanceMetric)

### Constructors
- `PerformanceMetric()` — Constructs a new PerformanceMetric.

### Methods
#### `string ToString()`

ToString method of PerformanceMetric.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PerformanceMetric.ToString)

### Properties
- `Average` (System.TimeSpan, get) — Average property of PerformanceMetric.
- `Maximum` (System.TimeSpan, get/set) — Maximum property of PerformanceMetric.
- `Minimum` (System.TimeSpan, get/set) — Minimum property of PerformanceMetric.
- `Operation` (string, get/set) — Operation property of PerformanceMetric.
- `Samples` (int, get/set) — Samples property of PerformanceMetric.
- `Target` (string, get/set) — Target property of PerformanceMetric.
- `Total` (System.TimeSpan, get/set) — Total property of PerformanceMetric.

## Plane (struct)

Plane struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Plane)

### Constructors
- `Plane(Fusion.Float3 normal, float distance)` — Constructs a new Plane.
- `Plane(float x, float y, float z, float distance)` — Constructs a new Plane.

### Methods
#### `float DistanceTo(Fusion.Float3 point)`

DistanceTo method of Plane.

**Parameters:**
- `point` (Fusion.Float3)

**Returns:** `float` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Plane.DistanceTo%28Fusion.Float3%29)

#### `bool Equals(Fusion.Plane other)`

Equals method of Plane.

**Parameters:**
- `other` (Fusion.Plane)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Plane.Equals%28Fusion.Plane%29)

#### `bool Equals(object other)`

Equals method of Plane.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Plane.Equals%28System.Object%29)

#### `int GetHashCode()`

GetHashCode method of Plane.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Plane.GetHashCode)

#### `bool Near(Fusion.Plane other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of Plane.

**Parameters:**
- `other` (Fusion.Plane)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Plane.Near%28Fusion.Plane%2CSystem.Int32%29)

#### `static bool op_Equality(Fusion.Plane a, Fusion.Plane b)`

op_Equality method of Plane.

**Parameters:**
- `a` (Fusion.Plane)
- `b` (Fusion.Plane)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Plane.op_Equality%28Fusion.Plane%2CFusion.Plane%29)

#### `static bool op_Inequality(Fusion.Plane a, Fusion.Plane b)`

op_Inequality method of Plane.

**Parameters:**
- `a` (Fusion.Plane)
- `b` (Fusion.Plane)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Plane.op_Inequality%28Fusion.Plane%2CFusion.Plane%29)

## PopupViewModel (class)

PopupViewModel class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.PopupViewModel)

### Methods
#### `void PopupClosed()`

PopupClosed method of PopupViewModel.

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PopupViewModel.PopupClosed)

#### `void PopupOpened()`

PopupOpened method of PopupViewModel.

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PopupViewModel.PopupOpened)

### Properties
- `Popup` (Fusion.IPopupController, get/set) — Popup property of PopupViewModel.

## Power (struct)

Power struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Power)

### Constructors
- `Power(double watts)` — Constructs a new Power.

### Methods
#### `static Fusion.Power Abs(Fusion.Power power)`

Abs method of Power.

**Parameters:**
- `power` (Fusion.Power)

**Returns:** `Fusion.Power` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.Abs%28Fusion.Power%29)

#### `int CompareTo(Fusion.Power other)`

CompareTo method of Power.

**Parameters:**
- `other` (Fusion.Power)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.CompareTo%28Fusion.Power%29)

#### `int CompareTo(object other)`

CompareTo method of Power.

**Parameters:**
- `other` (object)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.CompareTo%28System.Object%29)

#### `bool Equals(Fusion.Power other)`

Equals method of Power.

**Parameters:**
- `other` (Fusion.Power)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.Equals%28Fusion.Power%29)

#### `bool Equals(object other)`

Equals method of Power.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.Equals%28System.Object%29)

#### `static Fusion.Power From(double power, Fusion.PowerUnit powerUnit)`

From method of Power.

**Parameters:**
- `power` (double)
- `powerUnit` (Fusion.PowerUnit)

**Returns:** `Fusion.Power` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.From%28System.Double%2CFusion.PowerUnit%29)

#### `int GetHashCode()`

GetHashCode method of Power.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.GetHashCode)

#### `static bool IsInfinity(Fusion.Power power)`

IsInfinity method of Power.

**Parameters:**
- `power` (Fusion.Power)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.IsInfinity%28Fusion.Power%29)

#### `static bool IsNaN(Fusion.Power power)`

IsNaN method of Power.

**Parameters:**
- `power` (Fusion.Power)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.IsNaN%28Fusion.Power%29)

#### `static bool IsNegativeInfinity(Fusion.Power power)`

IsNegativeInfinity method of Power.

**Parameters:**
- `power` (Fusion.Power)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.IsNegativeInfinity%28Fusion.Power%29)

#### `static bool IsPositiveInfinity(Fusion.Power power)`

IsPositiveInfinity method of Power.

**Parameters:**
- `power` (Fusion.Power)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.IsPositiveInfinity%28Fusion.Power%29)

#### `static Fusion.Power Max(Fusion.Power a, Fusion.Power b)`

Max method of Power.

**Parameters:**
- `a` (Fusion.Power)
- `b` (Fusion.Power)

**Returns:** `Fusion.Power` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.Max%28Fusion.Power%2CFusion.Power%29)

#### `static Fusion.Power Min(Fusion.Power a, Fusion.Power b)`

Min method of Power.

**Parameters:**
- `a` (Fusion.Power)
- `b` (Fusion.Power)

**Returns:** `Fusion.Power` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.Min%28Fusion.Power%2CFusion.Power%29)

#### `bool Near(Fusion.Power other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of Power.

**Parameters:**
- `other` (Fusion.Power)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.Near%28Fusion.Power%2CSystem.Int32%29)

#### `static Fusion.Power Round(Fusion.Power power, Fusion.Power precision, Fusion.RoundingMode roundingMode)`

Round method of Power.

**Parameters:**
- `power` (Fusion.Power)
- `precision` (Fusion.Power)
- `roundingMode` (Fusion.RoundingMode)

**Returns:** `Fusion.Power` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.Round%28Fusion.Power%2CFusion.Power%2CFusion.RoundingMode%29)

#### `static int Sign(Fusion.Power power)`

Sign method of Power.

**Parameters:**
- `power` (Fusion.Power)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.Sign%28Fusion.Power%29)

#### `double To(Fusion.PowerUnit powerUnit)`

To method of Power.

**Parameters:**
- `powerUnit` (Fusion.PowerUnit)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.To%28Fusion.PowerUnit%29)

#### `string ToString()`

ToString method of Power.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of Power.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.Power op_Addition(Fusion.Power a, Fusion.Power b)`

op_Addition method of Power.

**Parameters:**
- `a` (Fusion.Power)
- `b` (Fusion.Power)

**Returns:** `Fusion.Power` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.op_Addition%28Fusion.Power%2CFusion.Power%29)

#### `static Fusion.Power op_Division(Fusion.Power a, double b)`

op_Division method of Power.

**Parameters:**
- `a` (Fusion.Power)
- `b` (double)

**Returns:** `Fusion.Power` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.op_Division%28Fusion.Power%2CSystem.Double%29)

#### `static double op_Division(Fusion.Power a, Fusion.Power b)`

op_Division method of Power.

**Parameters:**
- `a` (Fusion.Power)
- `b` (Fusion.Power)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.op_Division%28Fusion.Power%2CFusion.Power%29)

#### `static bool op_Equality(Fusion.Power a, Fusion.Power b)`

op_Equality method of Power.

**Parameters:**
- `a` (Fusion.Power)
- `b` (Fusion.Power)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.op_Equality%28Fusion.Power%2CFusion.Power%29)

#### `static bool op_GreaterThan(Fusion.Power a, Fusion.Power b)`

op_GreaterThan method of Power.

**Parameters:**
- `a` (Fusion.Power)
- `b` (Fusion.Power)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.op_GreaterThan%28Fusion.Power%2CFusion.Power%29)

#### `static bool op_GreaterThanOrEqual(Fusion.Power a, Fusion.Power b)`

op_GreaterThanOrEqual method of Power.

**Parameters:**
- `a` (Fusion.Power)
- `b` (Fusion.Power)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.op_GreaterThanOrEqual%28Fusion.Power%2CFusion.Power%29)

#### `static bool op_Inequality(Fusion.Power a, Fusion.Power b)`

op_Inequality method of Power.

**Parameters:**
- `a` (Fusion.Power)
- `b` (Fusion.Power)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.op_Inequality%28Fusion.Power%2CFusion.Power%29)

#### `static bool op_LessThan(Fusion.Power a, Fusion.Power b)`

op_LessThan method of Power.

**Parameters:**
- `a` (Fusion.Power)
- `b` (Fusion.Power)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.op_LessThan%28Fusion.Power%2CFusion.Power%29)

#### `static bool op_LessThanOrEqual(Fusion.Power a, Fusion.Power b)`

op_LessThanOrEqual method of Power.

**Parameters:**
- `a` (Fusion.Power)
- `b` (Fusion.Power)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.op_LessThanOrEqual%28Fusion.Power%2CFusion.Power%29)

#### `static Fusion.Power op_Modulus(Fusion.Power a, Fusion.Power b)`

op_Modulus method of Power.

**Parameters:**
- `a` (Fusion.Power)
- `b` (Fusion.Power)

**Returns:** `Fusion.Power` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.op_Modulus%28Fusion.Power%2CFusion.Power%29)

#### `static Fusion.Power op_Multiply(Fusion.Power a, double b)`

op_Multiply method of Power.

**Parameters:**
- `a` (Fusion.Power)
- `b` (double)

**Returns:** `Fusion.Power` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.op_Multiply%28Fusion.Power%2CSystem.Double%29)

#### `static Fusion.Power op_Multiply(double a, Fusion.Power b)`

op_Multiply method of Power.

**Parameters:**
- `a` (double)
- `b` (Fusion.Power)

**Returns:** `Fusion.Power` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.op_Multiply%28System.Double%2CFusion.Power%29)

#### `static Fusion.Power op_Subtraction(Fusion.Power a, Fusion.Power b)`

op_Subtraction method of Power.

**Parameters:**
- `a` (Fusion.Power)
- `b` (Fusion.Power)

**Returns:** `Fusion.Power` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.op_Subtraction%28Fusion.Power%2CFusion.Power%29)

#### `static Fusion.Power op_UnaryNegation(Fusion.Power a)`

op_UnaryNegation method of Power.

**Parameters:**
- `a` (Fusion.Power)

**Returns:** `Fusion.Power` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Power.op_UnaryNegation%28Fusion.Power%29)

### Properties
- `Watts` (double, get) — Watts property of Power.

## PowerUnit (enum)

PowerUnit enum in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.PowerUnit)

### Values
- `Watts` = `0`
- `Kilowatts` = `1`

## PressureLoss (struct)

PressureLoss struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.PressureLoss)

### Constructors
- `PressureLoss(double kilopascalsPerMeter)` — Constructs a new PressureLoss.

### Methods
#### `static Fusion.PressureLoss Abs(Fusion.PressureLoss pressureLoss)`

Abs method of PressureLoss.

**Parameters:**
- `pressureLoss` (Fusion.PressureLoss)

**Returns:** `Fusion.PressureLoss` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.Abs%28Fusion.PressureLoss%29)

#### `int CompareTo(Fusion.PressureLoss other)`

CompareTo method of PressureLoss.

**Parameters:**
- `other` (Fusion.PressureLoss)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.CompareTo%28Fusion.PressureLoss%29)

#### `int CompareTo(object other)`

CompareTo method of PressureLoss.

**Parameters:**
- `other` (object)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.CompareTo%28System.Object%29)

#### `bool Equals(Fusion.PressureLoss other)`

Equals method of PressureLoss.

**Parameters:**
- `other` (Fusion.PressureLoss)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.Equals%28Fusion.PressureLoss%29)

#### `bool Equals(object other)`

Equals method of PressureLoss.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.Equals%28System.Object%29)

#### `static Fusion.PressureLoss From(double pressureLoss, Fusion.PressureLossUnit pressureLossUnit)`

From method of PressureLoss.

**Parameters:**
- `pressureLoss` (double)
- `pressureLossUnit` (Fusion.PressureLossUnit)

**Returns:** `Fusion.PressureLoss` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.From%28System.Double%2CFusion.PressureLossUnit%29)

#### `int GetHashCode()`

GetHashCode method of PressureLoss.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.GetHashCode)

#### `static bool IsInfinity(Fusion.PressureLoss pressureLoss)`

IsInfinity method of PressureLoss.

**Parameters:**
- `pressureLoss` (Fusion.PressureLoss)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.IsInfinity%28Fusion.PressureLoss%29)

#### `static bool IsNaN(Fusion.PressureLoss pressureLoss)`

IsNaN method of PressureLoss.

**Parameters:**
- `pressureLoss` (Fusion.PressureLoss)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.IsNaN%28Fusion.PressureLoss%29)

#### `static bool IsNegativeInfinity(Fusion.PressureLoss pressureLoss)`

IsNegativeInfinity method of PressureLoss.

**Parameters:**
- `pressureLoss` (Fusion.PressureLoss)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.IsNegativeInfinity%28Fusion.PressureLoss%29)

#### `static bool IsPositiveInfinity(Fusion.PressureLoss pressureLoss)`

IsPositiveInfinity method of PressureLoss.

**Parameters:**
- `pressureLoss` (Fusion.PressureLoss)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.IsPositiveInfinity%28Fusion.PressureLoss%29)

#### `static Fusion.PressureLoss Max(Fusion.PressureLoss a, Fusion.PressureLoss b)`

Max method of PressureLoss.

**Parameters:**
- `a` (Fusion.PressureLoss)
- `b` (Fusion.PressureLoss)

**Returns:** `Fusion.PressureLoss` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.Max%28Fusion.PressureLoss%2CFusion.PressureLoss%29)

#### `static Fusion.PressureLoss Min(Fusion.PressureLoss a, Fusion.PressureLoss b)`

Min method of PressureLoss.

**Parameters:**
- `a` (Fusion.PressureLoss)
- `b` (Fusion.PressureLoss)

**Returns:** `Fusion.PressureLoss` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.Min%28Fusion.PressureLoss%2CFusion.PressureLoss%29)

#### `bool Near(Fusion.PressureLoss other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of PressureLoss.

**Parameters:**
- `other` (Fusion.PressureLoss)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.Near%28Fusion.PressureLoss%2CSystem.Int32%29)

#### `static Fusion.PressureLoss Round(Fusion.PressureLoss pressureLoss, Fusion.PressureLoss precision, Fusion.RoundingMode roundingMode)`

Round method of PressureLoss.

**Parameters:**
- `pressureLoss` (Fusion.PressureLoss)
- `precision` (Fusion.PressureLoss)
- `roundingMode` (Fusion.RoundingMode)

**Returns:** `Fusion.PressureLoss` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.Round%28Fusion.PressureLoss%2CFusion.PressureLoss%2CFusion.RoundingMode%29)

#### `static int Sign(Fusion.PressureLoss pressureLoss)`

Sign method of PressureLoss.

**Parameters:**
- `pressureLoss` (Fusion.PressureLoss)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.Sign%28Fusion.PressureLoss%29)

#### `double To(Fusion.PressureLossUnit pressureLossUnit)`

To method of PressureLoss.

**Parameters:**
- `pressureLossUnit` (Fusion.PressureLossUnit)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.To%28Fusion.PressureLossUnit%29)

#### `string ToString()`

ToString method of PressureLoss.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of PressureLoss.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.PressureLoss op_Addition(Fusion.PressureLoss a, Fusion.PressureLoss b)`

op_Addition method of PressureLoss.

**Parameters:**
- `a` (Fusion.PressureLoss)
- `b` (Fusion.PressureLoss)

**Returns:** `Fusion.PressureLoss` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.op_Addition%28Fusion.PressureLoss%2CFusion.PressureLoss%29)

#### `static Fusion.PressureLoss op_Division(Fusion.PressureLoss a, double b)`

op_Division method of PressureLoss.

**Parameters:**
- `a` (Fusion.PressureLoss)
- `b` (double)

**Returns:** `Fusion.PressureLoss` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.op_Division%28Fusion.PressureLoss%2CSystem.Double%29)

#### `static double op_Division(Fusion.PressureLoss a, Fusion.PressureLoss b)`

op_Division method of PressureLoss.

**Parameters:**
- `a` (Fusion.PressureLoss)
- `b` (Fusion.PressureLoss)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.op_Division%28Fusion.PressureLoss%2CFusion.PressureLoss%29)

#### `static bool op_Equality(Fusion.PressureLoss a, Fusion.PressureLoss b)`

op_Equality method of PressureLoss.

**Parameters:**
- `a` (Fusion.PressureLoss)
- `b` (Fusion.PressureLoss)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.op_Equality%28Fusion.PressureLoss%2CFusion.PressureLoss%29)

#### `static bool op_GreaterThan(Fusion.PressureLoss a, Fusion.PressureLoss b)`

op_GreaterThan method of PressureLoss.

**Parameters:**
- `a` (Fusion.PressureLoss)
- `b` (Fusion.PressureLoss)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.op_GreaterThan%28Fusion.PressureLoss%2CFusion.PressureLoss%29)

#### `static bool op_GreaterThanOrEqual(Fusion.PressureLoss a, Fusion.PressureLoss b)`

op_GreaterThanOrEqual method of PressureLoss.

**Parameters:**
- `a` (Fusion.PressureLoss)
- `b` (Fusion.PressureLoss)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.op_GreaterThanOrEqual%28Fusion.PressureLoss%2CFusion.PressureLoss%29)

#### `static bool op_Inequality(Fusion.PressureLoss a, Fusion.PressureLoss b)`

op_Inequality method of PressureLoss.

**Parameters:**
- `a` (Fusion.PressureLoss)
- `b` (Fusion.PressureLoss)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.op_Inequality%28Fusion.PressureLoss%2CFusion.PressureLoss%29)

#### `static bool op_LessThan(Fusion.PressureLoss a, Fusion.PressureLoss b)`

op_LessThan method of PressureLoss.

**Parameters:**
- `a` (Fusion.PressureLoss)
- `b` (Fusion.PressureLoss)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.op_LessThan%28Fusion.PressureLoss%2CFusion.PressureLoss%29)

#### `static bool op_LessThanOrEqual(Fusion.PressureLoss a, Fusion.PressureLoss b)`

op_LessThanOrEqual method of PressureLoss.

**Parameters:**
- `a` (Fusion.PressureLoss)
- `b` (Fusion.PressureLoss)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.op_LessThanOrEqual%28Fusion.PressureLoss%2CFusion.PressureLoss%29)

#### `static Fusion.PressureLoss op_Modulus(Fusion.PressureLoss a, Fusion.PressureLoss b)`

op_Modulus method of PressureLoss.

**Parameters:**
- `a` (Fusion.PressureLoss)
- `b` (Fusion.PressureLoss)

**Returns:** `Fusion.PressureLoss` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.op_Modulus%28Fusion.PressureLoss%2CFusion.PressureLoss%29)

#### `static Fusion.PressureLoss op_Multiply(Fusion.PressureLoss a, double b)`

op_Multiply method of PressureLoss.

**Parameters:**
- `a` (Fusion.PressureLoss)
- `b` (double)

**Returns:** `Fusion.PressureLoss` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.op_Multiply%28Fusion.PressureLoss%2CSystem.Double%29)

#### `static Fusion.PressureLoss op_Multiply(double a, Fusion.PressureLoss b)`

op_Multiply method of PressureLoss.

**Parameters:**
- `a` (double)
- `b` (Fusion.PressureLoss)

**Returns:** `Fusion.PressureLoss` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.op_Multiply%28System.Double%2CFusion.PressureLoss%29)

#### `static Fusion.PressureLoss op_Subtraction(Fusion.PressureLoss a, Fusion.PressureLoss b)`

op_Subtraction method of PressureLoss.

**Parameters:**
- `a` (Fusion.PressureLoss)
- `b` (Fusion.PressureLoss)

**Returns:** `Fusion.PressureLoss` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.op_Subtraction%28Fusion.PressureLoss%2CFusion.PressureLoss%29)

#### `static Fusion.PressureLoss op_UnaryNegation(Fusion.PressureLoss a)`

op_UnaryNegation method of PressureLoss.

**Parameters:**
- `a` (Fusion.PressureLoss)

**Returns:** `Fusion.PressureLoss` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PressureLoss.op_UnaryNegation%28Fusion.PressureLoss%29)

### Properties
- `KilopascalsPerMeter` (double, get) — KilopascalsPerMeter property of PressureLoss.

## PressureLossUnit (enum)

PressureLossUnit enum in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.PressureLossUnit)

### Values
- `KilopascalsPerMeter` = `0`
- `PoundsPerSquareInchPerFoot` = `1`
- `PascalsPerMeter` = `2`
- `HectopascalsPerMeter` = `3`

## Progress (class)

Progress class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Progress)

### Constructors
- `Progress()` — Constructs a new Progress.
- `Progress(System.Threading.CancellationTokenSource parentCancellationTokenSource)` — Constructs a new Progress.

### Methods
#### `void Complete()`

Complete method of Progress.

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Progress.Complete)

#### `void Failed(System.Exception exception)`

Failed method of Progress.

**Parameters:**
- `exception` (System.Exception)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Progress.Failed%28System.Exception%29)

### Properties
- `Completed` (System.Threading.Tasks.Task, get) — Completed property of Progress.

## ProgressBase (class)

ProgressBase class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.ProgressBase)

### Methods
#### `void Cancel()`

Cancel method of ProgressBase.

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.ProgressBase.Cancel)

#### `static Fusion.ProgressBase.Update Combine(Fusion.ProgressBase.Update update, int operationIndex, int operationCount)`

Combine method of ProgressBase.

**Parameters:**
- `update` (Fusion.ProgressBase.Update)
- `operationIndex` (int)
- `operationCount` (int)

**Returns:** `Fusion.ProgressBase.Update` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.ProgressBase.Combine%28Fusion.ProgressBase.Update%2CSystem.Int32%2CSystem.Int32%29)

#### `static Fusion.ProgressBase.Update Combine(string text, Fusion.ProgressBase.Update[] updates)`

Combine method of ProgressBase.

**Parameters:**
- `text` (string)
- `updates` (Fusion.ProgressBase.Update[])

**Returns:** `Fusion.ProgressBase.Update` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.ProgressBase.Combine%28System.String%2CFusion.ProgressBase.Update%5B%5D%29)

#### `System.Runtime.Remoting.ObjRef CreateObjRef(System.Type requestedType)`

CreateObjRef method of ProgressBase.

**Parameters:**
- `requestedType` (System.Type)

**Returns:** `System.Runtime.Remoting.ObjRef` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.ProgressBase.CreateObjRef%28System.Type%29)

#### `void Dispose()`

Dispose method of ProgressBase.

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.ProgressBase.Dispose)

#### `object GetLifetimeService()`

GetLifetimeService method of ProgressBase.

**Returns:** `object` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.ProgressBase.GetLifetimeService)

#### `object InitializeLifetimeService()`

InitializeLifetimeService method of ProgressBase.

**Returns:** `object` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.ProgressBase.InitializeLifetimeService)

#### `void Report(Fusion.ProgressBase.Update update)`

Report method of ProgressBase.

**Parameters:**
- `update` (Fusion.ProgressBase.Update)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.ProgressBase.Report%28Fusion.ProgressBase.Update%29)

#### `void Report(string text)`

Report method of ProgressBase.

**Parameters:**
- `text` (string)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.ProgressBase.Report%28System.String%29)

#### `void Report(string text, bool cancelable)`

Report method of ProgressBase.

**Parameters:**
- `text` (string)
- `cancelable` (bool)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.ProgressBase.Report%28System.String%2CSystem.Boolean%29)

#### `void Report(string text, double progressPercentage)`

Report method of ProgressBase.

**Parameters:**
- `text` (string)
- `progressPercentage` (double)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.ProgressBase.Report%28System.String%2CSystem.Double%29)

#### `void Report(string text, double progressPercentage, bool cancelable)`

Report method of ProgressBase.

**Parameters:**
- `text` (string)
- `progressPercentage` (double)
- `cancelable` (bool)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.ProgressBase.Report%28System.String%2CSystem.Double%2CSystem.Boolean%29)

### Properties
- `Cancelable` (bool, get) — Cancelable property of ProgressBase.
- `CancellationToken` (System.Threading.CancellationToken, get) — CancellationToken property of ProgressBase.
- `CancellationTokenSource` (System.Threading.CancellationTokenSource, get) — CancellationTokenSource property of ProgressBase.
- `IsCancellationRequested` (bool, get) — IsCancellationRequested property of ProgressBase.
- `ProgressPercentage` (double?, get) — ProgressPercentage property of ProgressBase.
- `Text` (string, get) — Text property of ProgressBase.

### Events
#### `ProgressChanged` (`System.EventHandler<Fusion.ProgressBase.Update>`)

**Signature:** `event System.EventHandler<Fusion.ProgressBase.Update> ProgressChanged`

ProgressChanged event of ProgressBase.

[Docs](https://www.nuget.org/packages/TeklaFusion#E%3AFusion.ProgressBase.ProgressChanged)

#### `PropertyChanged` (`System.ComponentModel.PropertyChangedEventHandler`)

**Signature:** `event System.ComponentModel.PropertyChangedEventHandler PropertyChanged`

PropertyChanged event of ProgressBase.

[Docs](https://www.nuget.org/packages/TeklaFusion#E%3AFusion.ProgressBase.PropertyChanged)

## Progress`1 (class)

Progress`1 class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Progress`1)

### Constructors
- `Progress`1()` — Constructs a new Progress`1.
- `Progress`1(System.Threading.CancellationTokenSource parentCancellationTokenSource)` — Constructs a new Progress`1.

### Methods
#### `void Complete(TResult result)`

Complete method of Progress`1.

**Parameters:**
- `result` (TResult)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Progress%601.Complete%28%600%29)

#### `void Failed(System.Exception exception)`

Failed method of Progress`1.

**Parameters:**
- `exception` (System.Exception)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Progress%601.Failed%28System.Exception%29)

### Properties
- `Completed` (System.Threading.Tasks.Task<TResult>, get) — Completed property of Progress`1.

## PublishedMethodAttribute (class)

PublishedMethodAttribute class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.PublishedMethodAttribute)

### Constructors
- `PublishedMethodAttribute()` — Constructs a new PublishedMethodAttribute.
- `PublishedMethodAttribute(Fusion.InvokeIn invokeIn)` — Constructs a new PublishedMethodAttribute.

### Properties
- `Documentation` (string, get/set) — Documentation property of PublishedMethodAttribute.
- `InvokeIn` (Fusion.InvokeIn, get) — InvokeIn property of PublishedMethodAttribute.

## PublishedViewAttribute (class)

PublishedViewAttribute class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.PublishedViewAttribute)

### Constructors
- `PublishedViewAttribute(string viewName)` — Constructs a new PublishedViewAttribute.

### Properties
- `Documentation` (string, get/set) — Documentation property of PublishedViewAttribute.
- `ViewName` (string, get) — ViewName property of PublishedViewAttribute.
- `ViewType` (System.Type, get/set) — ViewType property of PublishedViewAttribute.

## PublishedViewInfo (class)

PublishedViewInfo class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.PublishedViewInfo)

### Constructors
- `PublishedViewInfo(System.Func<object, Fusion.ViewModel> factoryMethod, Fusion.PublishedViewAttribute attribute, Fusion.PublishedViewMetadataAttribute[] metadataAttributes = null)` — Constructs a new PublishedViewInfo.

### Methods
#### `TValue GetMetadata<TValue>(object key, TValue defaultValue)`

GetMetadata method of PublishedViewInfo.

**Parameters:**
- `key` (object)
- `defaultValue` (TValue)

**Returns:** `TValue` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PublishedViewInfo.GetMetadata%60%601%28System.Object%2C%60%600%29)

#### `bool HasMetadata(object key)`

HasMetadata method of PublishedViewInfo.

**Parameters:**
- `key` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PublishedViewInfo.HasMetadata%28System.Object%29)

#### `bool HasMetadata(object key, object expectedValue)`

HasMetadata method of PublishedViewInfo.

**Parameters:**
- `key` (object)
- `expectedValue` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.PublishedViewInfo.HasMetadata%28System.Object%2CSystem.Object%29)

### Properties
- `Documentation` (string, get) — Documentation property of PublishedViewInfo.
- `Metadata` (System.Collections.ObjectModel.ReadOnlyDictionary<object, object>, get) — Metadata property of PublishedViewInfo.
- `ViewName` (string, get) — ViewName property of PublishedViewInfo.
- `ViewType` (System.Type, get) — ViewType property of PublishedViewInfo.

## PublishedViewMetadataAttribute (class)

PublishedViewMetadataAttribute class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.PublishedViewMetadataAttribute)

### Constructors
- `PublishedViewMetadataAttribute(object key, object value)` — Constructs a new PublishedViewMetadataAttribute.

### Properties
- `Key` (object, get) — Key property of PublishedViewMetadataAttribute.
- `Value` (object, get) — Value property of PublishedViewMetadataAttribute.

## RGBA (struct)

RGBA struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.RGBA)

### Constructors
- `RGBA(Fusion.RGBA rgb, byte a)` — Constructs a new RGBA.
- `RGBA(byte all)` — Constructs a new RGBA.
- `RGBA(byte r, byte g, byte b)` — Constructs a new RGBA.
- `RGBA(byte r, byte g, byte b, byte a)` — Constructs a new RGBA.
- `RGBA(byte rgb, byte a)` — Constructs a new RGBA.

### Methods
#### `bool Equals(Fusion.RGBA other)`

Equals method of RGBA.

**Parameters:**
- `other` (Fusion.RGBA)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.RGBA.Equals%28Fusion.RGBA%29)

#### `bool Equals(object other)`

Equals method of RGBA.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.RGBA.Equals%28System.Object%29)

#### `int GetHashCode()`

GetHashCode method of RGBA.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.RGBA.GetHashCode)

#### `string ToString()`

ToString method of RGBA.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.RGBA.ToString)

#### `static bool op_Equality(Fusion.RGBA a, Fusion.RGBA b)`

op_Equality method of RGBA.

**Parameters:**
- `a` (Fusion.RGBA)
- `b` (Fusion.RGBA)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.RGBA.op_Equality%28Fusion.RGBA%2CFusion.RGBA%29)

#### `static Fusion.RGBA op_Explicit(Fusion.Double4 v)`

op_Explicit method of RGBA.

**Parameters:**
- `v` (Fusion.Double4)

**Returns:** `Fusion.RGBA` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.RGBA.op_Explicit%28Fusion.Double4%29~Fusion.RGBA)

#### `static Fusion.RGBA op_Explicit(Fusion.Float4 v)`

op_Explicit method of RGBA.

**Parameters:**
- `v` (Fusion.Float4)

**Returns:** `Fusion.RGBA` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.RGBA.op_Explicit%28Fusion.Float4%29~Fusion.RGBA)

#### `static Fusion.RGBA op_Explicit(Fusion.Int4 v)`

op_Explicit method of RGBA.

**Parameters:**
- `v` (Fusion.Int4)

**Returns:** `Fusion.RGBA` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.RGBA.op_Explicit%28Fusion.Int4%29~Fusion.RGBA)

#### `static Fusion.RGBA op_Explicit(Fusion.Short4 v)`

op_Explicit method of RGBA.

**Parameters:**
- `v` (Fusion.Short4)

**Returns:** `Fusion.RGBA` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.RGBA.op_Explicit%28Fusion.Short4%29~Fusion.RGBA)

#### `static Fusion.RGBA op_Explicit(Fusion.UShort4 v)`

op_Explicit method of RGBA.

**Parameters:**
- `v` (Fusion.UShort4)

**Returns:** `Fusion.RGBA` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.RGBA.op_Explicit%28Fusion.UShort4%29~Fusion.RGBA)

#### `static bool op_Inequality(Fusion.RGBA a, Fusion.RGBA b)`

op_Inequality method of RGBA.

**Parameters:**
- `a` (Fusion.RGBA)
- `b` (Fusion.RGBA)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.RGBA.op_Inequality%28Fusion.RGBA%2CFusion.RGBA%29)

### Properties
- `Item` (byte, get/set) — Item property of RGBA.

## Random`1 (class)

Random`1 class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Random`1)

### Constructors
- `Random`1(System.Collections.Generic.IList<TElement> elements)` — Constructs a new Random`1.
- `Random`1(System.Collections.Generic.IList<TElement> elements, int seed)` — Constructs a new Random`1.

### Methods
#### `System.Collections.Generic.IEnumerator<TElement> GetEnumerator()`

GetEnumerator method of Random`1.

**Returns:** `System.Collections.Generic.IEnumerator<TElement>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Random%601.GetEnumerator)

#### `TElement Next()`

Next method of Random`1.

**Returns:** `TElement` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Random%601.Next)

## RatioUnit (enum)

RatioUnit enum in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.RatioUnit)

### Values
- `None` = `0`
- `Percent` = `1`
- `Permille` = `2`

## Ray (struct)

Ray struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Ray)

### Constructors
- `Ray(Fusion.Float3 origin, Fusion.Float3 direction)` — Constructs a new Ray.

### Methods
#### `bool Equals(Fusion.Ray other)`

Equals method of Ray.

**Parameters:**
- `other` (Fusion.Ray)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Ray.Equals%28Fusion.Ray%29)

#### `bool Equals(object other)`

Equals method of Ray.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Ray.Equals%28System.Object%29)

#### `int GetHashCode()`

GetHashCode method of Ray.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Ray.GetHashCode)

#### `float? Intersects(Fusion.BoundingBox boundingBox)`

Intersects method of Ray.

**Parameters:**
- `boundingBox` (Fusion.BoundingBox)

**Returns:** `float?` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Ray.Intersects%28Fusion.BoundingBox%29)

#### `float? Intersects(Fusion.Triangle triangle)`

Intersects method of Ray.

**Parameters:**
- `triangle` (Fusion.Triangle)

**Returns:** `float?` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Ray.Intersects%28Fusion.Triangle%29)

#### `float? Intersects(Fusion.TriangleMesh triangleMesh)`

Intersects method of Ray.

**Parameters:**
- `triangleMesh` (Fusion.TriangleMesh)

**Returns:** `float?` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Ray.Intersects%28Fusion.TriangleMesh%29)

#### `float? Intersects(Fusion.TriangleMesh triangleMesh, Fusion.Plane[] clipPlanes)`

Intersects method of Ray.

**Parameters:**
- `triangleMesh` (Fusion.TriangleMesh)
- `clipPlanes` (Fusion.Plane[])

**Returns:** `float?` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Ray.Intersects%28Fusion.TriangleMesh%2CFusion.Plane%5B%5D%29)

#### `bool Near(Fusion.Ray other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of Ray.

**Parameters:**
- `other` (Fusion.Ray)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Ray.Near%28Fusion.Ray%2CSystem.Int32%29)

#### `static bool op_Equality(Fusion.Ray a, Fusion.Ray b)`

op_Equality method of Ray.

**Parameters:**
- `a` (Fusion.Ray)
- `b` (Fusion.Ray)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Ray.op_Equality%28Fusion.Ray%2CFusion.Ray%29)

#### `static bool op_Inequality(Fusion.Ray a, Fusion.Ray b)`

op_Inequality method of Ray.

**Parameters:**
- `a` (Fusion.Ray)
- `b` (Fusion.Ray)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Ray.op_Inequality%28Fusion.Ray%2CFusion.Ray%29)

#### `static Fusion.Float3 op_Multiply(Fusion.Ray a, float b)`

op_Multiply method of Ray.

**Parameters:**
- `a` (Fusion.Ray)
- `b` (float)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Ray.op_Multiply%28Fusion.Ray%2CSystem.Single%29)

#### `static Fusion.Float3 op_Multiply(float a, Fusion.Ray b)`

op_Multiply method of Ray.

**Parameters:**
- `a` (float)
- `b` (Fusion.Ray)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Ray.op_Multiply%28System.Single%2CFusion.Ray%29)

## ReadOnlySet`1 (class)

ReadOnlySet`1 class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.ReadOnlySet`1)

### Constructors
- `ReadOnlySet`1(System.Collections.Generic.ISet<T> set)` — Constructs a new ReadOnlySet`1.

### Methods
#### `bool Contains(T item)`

Contains method of ReadOnlySet`1.

**Parameters:**
- `item` (T)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.ReadOnlySet%601.Contains%28%600%29)

#### `void CopyTo(T[] array, int arrayIndex)`

CopyTo method of ReadOnlySet`1.

**Parameters:**
- `array` (T[])
- `arrayIndex` (int)

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.ReadOnlySet%601.CopyTo%28%600%5B%5D%2CSystem.Int32%29)

#### `System.Collections.Generic.IEnumerator<T> GetEnumerator()`

GetEnumerator method of ReadOnlySet`1.

**Returns:** `System.Collections.Generic.IEnumerator<T>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.ReadOnlySet%601.GetEnumerator)

#### `bool IsProperSubsetOf(System.Collections.Generic.IEnumerable<T> other)`

IsProperSubsetOf method of ReadOnlySet`1.

**Parameters:**
- `other` (System.Collections.Generic.IEnumerable<T>)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.ReadOnlySet%601.IsProperSubsetOf%28System.Collections.Generic.IEnumerable%7B%600%7D%29)

#### `bool IsProperSupersetOf(System.Collections.Generic.IEnumerable<T> other)`

IsProperSupersetOf method of ReadOnlySet`1.

**Parameters:**
- `other` (System.Collections.Generic.IEnumerable<T>)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.ReadOnlySet%601.IsProperSupersetOf%28System.Collections.Generic.IEnumerable%7B%600%7D%29)

#### `bool IsSubsetOf(System.Collections.Generic.IEnumerable<T> other)`

IsSubsetOf method of ReadOnlySet`1.

**Parameters:**
- `other` (System.Collections.Generic.IEnumerable<T>)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.ReadOnlySet%601.IsSubsetOf%28System.Collections.Generic.IEnumerable%7B%600%7D%29)

#### `bool IsSupersetOf(System.Collections.Generic.IEnumerable<T> other)`

IsSupersetOf method of ReadOnlySet`1.

**Parameters:**
- `other` (System.Collections.Generic.IEnumerable<T>)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.ReadOnlySet%601.IsSupersetOf%28System.Collections.Generic.IEnumerable%7B%600%7D%29)

#### `bool Overlaps(System.Collections.Generic.IEnumerable<T> other)`

Overlaps method of ReadOnlySet`1.

**Parameters:**
- `other` (System.Collections.Generic.IEnumerable<T>)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.ReadOnlySet%601.Overlaps%28System.Collections.Generic.IEnumerable%7B%600%7D%29)

#### `bool SetEquals(System.Collections.Generic.IEnumerable<T> other)`

SetEquals method of ReadOnlySet`1.

**Parameters:**
- `other` (System.Collections.Generic.IEnumerable<T>)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.ReadOnlySet%601.SetEquals%28System.Collections.Generic.IEnumerable%7B%600%7D%29)

### Properties
- `Count` (int, get) — Count property of ReadOnlySet`1.

## ResourcesAttribute (class)

ResourcesAttribute class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.ResourcesAttribute)

### Constructors
- `ResourcesAttribute(string resourcePath)` — Constructs a new ResourcesAttribute.

### Properties
- `Culture` (string, get/set) — Culture property of ResourcesAttribute.
- `ResourcePath` (string, get) — ResourcePath property of ResourcesAttribute.
- `Theme` (string, get/set) — Theme property of ResourcesAttribute.

## RoundingMode (enum)

RoundingMode enum in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.RoundingMode)

### Values
- `Nearest` = `0`
- `Up` = `1`
- `Down` = `2`

## SharedResourcesAttribute (class)

SharedResourcesAttribute class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.SharedResourcesAttribute)

### Constructors
- `SharedResourcesAttribute(string resourcePath)` — Constructs a new SharedResourcesAttribute.

## Short2 (struct)

Short2 struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Short2)

### Constructors
- `Short2(short all)` — Constructs a new Short2.
- `Short2(short x, short y)` — Constructs a new Short2.

### Methods
#### `static Fusion.Short2 Abs(Fusion.Short2 value)`

Abs method of Short2.

**Parameters:**
- `value` (Fusion.Short2)

**Returns:** `Fusion.Short2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.Abs%28Fusion.Short2%29)

#### `bool Equals(Fusion.Short2 other)`

Equals method of Short2.

**Parameters:**
- `other` (Fusion.Short2)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.Equals%28Fusion.Short2%29)

#### `bool Equals(object other)`

Equals method of Short2.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.Equals%28System.Object%29)

#### `int GetHashCode()`

GetHashCode method of Short2.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.GetHashCode)

#### `static Fusion.Short2 Max(Fusion.Short2 a, Fusion.Short2 b)`

Max method of Short2.

**Parameters:**
- `a` (Fusion.Short2)
- `b` (Fusion.Short2)

**Returns:** `Fusion.Short2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.Max%28Fusion.Short2%2CFusion.Short2%29)

#### `static Fusion.Short2 Min(Fusion.Short2 a, Fusion.Short2 b)`

Min method of Short2.

**Parameters:**
- `a` (Fusion.Short2)
- `b` (Fusion.Short2)

**Returns:** `Fusion.Short2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.Min%28Fusion.Short2%2CFusion.Short2%29)

#### `static Fusion.Int2 Sign(Fusion.Short2 value)`

Sign method of Short2.

**Parameters:**
- `value` (Fusion.Short2)

**Returns:** `Fusion.Int2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.Sign%28Fusion.Short2%29)

#### `string ToString()`

ToString method of Short2.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of Short2.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.Short2 op_Addition(Fusion.Short2 a, Fusion.Short2 b)`

op_Addition method of Short2.

**Parameters:**
- `a` (Fusion.Short2)
- `b` (Fusion.Short2)

**Returns:** `Fusion.Short2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.op_Addition%28Fusion.Short2%2CFusion.Short2%29)

#### `static Fusion.Short2 op_Addition(Fusion.Short2 a, short b)`

op_Addition method of Short2.

**Parameters:**
- `a` (Fusion.Short2)
- `b` (short)

**Returns:** `Fusion.Short2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.op_Addition%28Fusion.Short2%2CSystem.Int16%29)

#### `static Fusion.Short2 op_Addition(short a, Fusion.Short2 b)`

op_Addition method of Short2.

**Parameters:**
- `a` (short)
- `b` (Fusion.Short2)

**Returns:** `Fusion.Short2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.op_Addition%28System.Int16%2CFusion.Short2%29)

#### `static Fusion.Short2 op_Division(Fusion.Short2 a, Fusion.Short2 b)`

op_Division method of Short2.

**Parameters:**
- `a` (Fusion.Short2)
- `b` (Fusion.Short2)

**Returns:** `Fusion.Short2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.op_Division%28Fusion.Short2%2CFusion.Short2%29)

#### `static Fusion.Short2 op_Division(Fusion.Short2 a, short b)`

op_Division method of Short2.

**Parameters:**
- `a` (Fusion.Short2)
- `b` (short)

**Returns:** `Fusion.Short2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.op_Division%28Fusion.Short2%2CSystem.Int16%29)

#### `static Fusion.Short2 op_Division(short a, Fusion.Short2 b)`

op_Division method of Short2.

**Parameters:**
- `a` (short)
- `b` (Fusion.Short2)

**Returns:** `Fusion.Short2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.op_Division%28System.Int16%2CFusion.Short2%29)

#### `static bool op_Equality(Fusion.Short2 a, Fusion.Short2 b)`

op_Equality method of Short2.

**Parameters:**
- `a` (Fusion.Short2)
- `b` (Fusion.Short2)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.op_Equality%28Fusion.Short2%2CFusion.Short2%29)

#### `static Fusion.Short2 op_Explicit(Fusion.Double2 v)`

op_Explicit method of Short2.

**Parameters:**
- `v` (Fusion.Double2)

**Returns:** `Fusion.Short2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.op_Explicit%28Fusion.Double2%29~Fusion.Short2)

#### `static Fusion.Short2 op_Explicit(Fusion.Float2 v)`

op_Explicit method of Short2.

**Parameters:**
- `v` (Fusion.Float2)

**Returns:** `Fusion.Short2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.op_Explicit%28Fusion.Float2%29~Fusion.Short2)

#### `static Fusion.Short2 op_Explicit(Fusion.Half2 v)`

op_Explicit method of Short2.

**Parameters:**
- `v` (Fusion.Half2)

**Returns:** `Fusion.Short2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.op_Explicit%28Fusion.Half2%29~Fusion.Short2)

#### `static Fusion.Short2 op_Explicit(Fusion.Int2 v)`

op_Explicit method of Short2.

**Parameters:**
- `v` (Fusion.Int2)

**Returns:** `Fusion.Short2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.op_Explicit%28Fusion.Int2%29~Fusion.Short2)

#### `static Fusion.Short2 op_Explicit(Fusion.UShort2 v)`

op_Explicit method of Short2.

**Parameters:**
- `v` (Fusion.UShort2)

**Returns:** `Fusion.Short2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.op_Explicit%28Fusion.UShort2%29~Fusion.Short2)

#### `static bool op_Inequality(Fusion.Short2 a, Fusion.Short2 b)`

op_Inequality method of Short2.

**Parameters:**
- `a` (Fusion.Short2)
- `b` (Fusion.Short2)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.op_Inequality%28Fusion.Short2%2CFusion.Short2%29)

#### `static Fusion.Short2 op_Modulus(Fusion.Short2 a, Fusion.Short2 b)`

op_Modulus method of Short2.

**Parameters:**
- `a` (Fusion.Short2)
- `b` (Fusion.Short2)

**Returns:** `Fusion.Short2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.op_Modulus%28Fusion.Short2%2CFusion.Short2%29)

#### `static Fusion.Short2 op_Modulus(Fusion.Short2 a, short b)`

op_Modulus method of Short2.

**Parameters:**
- `a` (Fusion.Short2)
- `b` (short)

**Returns:** `Fusion.Short2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.op_Modulus%28Fusion.Short2%2CSystem.Int16%29)

#### `static Fusion.Short2 op_Modulus(short a, Fusion.Short2 b)`

op_Modulus method of Short2.

**Parameters:**
- `a` (short)
- `b` (Fusion.Short2)

**Returns:** `Fusion.Short2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.op_Modulus%28System.Int16%2CFusion.Short2%29)

#### `static Fusion.Short2 op_Multiply(Fusion.Short2 a, Fusion.Short2 b)`

op_Multiply method of Short2.

**Parameters:**
- `a` (Fusion.Short2)
- `b` (Fusion.Short2)

**Returns:** `Fusion.Short2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.op_Multiply%28Fusion.Short2%2CFusion.Short2%29)

#### `static Fusion.Short2 op_Multiply(Fusion.Short2 a, short b)`

op_Multiply method of Short2.

**Parameters:**
- `a` (Fusion.Short2)
- `b` (short)

**Returns:** `Fusion.Short2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.op_Multiply%28Fusion.Short2%2CSystem.Int16%29)

#### `static Fusion.Short2 op_Multiply(short a, Fusion.Short2 b)`

op_Multiply method of Short2.

**Parameters:**
- `a` (short)
- `b` (Fusion.Short2)

**Returns:** `Fusion.Short2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.op_Multiply%28System.Int16%2CFusion.Short2%29)

#### `static Fusion.Short2 op_Subtraction(Fusion.Short2 a, Fusion.Short2 b)`

op_Subtraction method of Short2.

**Parameters:**
- `a` (Fusion.Short2)
- `b` (Fusion.Short2)

**Returns:** `Fusion.Short2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.op_Subtraction%28Fusion.Short2%2CFusion.Short2%29)

#### `static Fusion.Short2 op_Subtraction(Fusion.Short2 a, short b)`

op_Subtraction method of Short2.

**Parameters:**
- `a` (Fusion.Short2)
- `b` (short)

**Returns:** `Fusion.Short2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.op_Subtraction%28Fusion.Short2%2CSystem.Int16%29)

#### `static Fusion.Short2 op_Subtraction(short a, Fusion.Short2 b)`

op_Subtraction method of Short2.

**Parameters:**
- `a` (short)
- `b` (Fusion.Short2)

**Returns:** `Fusion.Short2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.op_Subtraction%28System.Int16%2CFusion.Short2%29)

#### `static Fusion.Short2 op_UnaryNegation(Fusion.Short2 v)`

op_UnaryNegation method of Short2.

**Parameters:**
- `v` (Fusion.Short2)

**Returns:** `Fusion.Short2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.op_UnaryNegation%28Fusion.Short2%29)

#### `static Fusion.Short2 op_UnaryPlus(Fusion.Short2 v)`

op_UnaryPlus method of Short2.

**Parameters:**
- `v` (Fusion.Short2)

**Returns:** `Fusion.Short2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short2.op_UnaryPlus%28Fusion.Short2%29)

### Properties
- `Item` (short, get/set) — Item property of Short2.

## Short3 (struct)

Short3 struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Short3)

### Constructors
- `Short3(Fusion.Short2 xy, short z)` — Constructs a new Short3.
- `Short3(short all)` — Constructs a new Short3.
- `Short3(short x, Fusion.Short2 yz)` — Constructs a new Short3.
- `Short3(short x, short y, short z)` — Constructs a new Short3.

### Methods
#### `static Fusion.Short3 Abs(Fusion.Short3 value)`

Abs method of Short3.

**Parameters:**
- `value` (Fusion.Short3)

**Returns:** `Fusion.Short3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.Abs%28Fusion.Short3%29)

#### `bool Equals(Fusion.Short3 other)`

Equals method of Short3.

**Parameters:**
- `other` (Fusion.Short3)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.Equals%28Fusion.Short3%29)

#### `bool Equals(object other)`

Equals method of Short3.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.Equals%28System.Object%29)

#### `int GetHashCode()`

GetHashCode method of Short3.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.GetHashCode)

#### `static Fusion.Short3 Max(Fusion.Short3 a, Fusion.Short3 b)`

Max method of Short3.

**Parameters:**
- `a` (Fusion.Short3)
- `b` (Fusion.Short3)

**Returns:** `Fusion.Short3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.Max%28Fusion.Short3%2CFusion.Short3%29)

#### `static Fusion.Short3 Min(Fusion.Short3 a, Fusion.Short3 b)`

Min method of Short3.

**Parameters:**
- `a` (Fusion.Short3)
- `b` (Fusion.Short3)

**Returns:** `Fusion.Short3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.Min%28Fusion.Short3%2CFusion.Short3%29)

#### `static Fusion.Int3 Sign(Fusion.Short3 value)`

Sign method of Short3.

**Parameters:**
- `value` (Fusion.Short3)

**Returns:** `Fusion.Int3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.Sign%28Fusion.Short3%29)

#### `string ToString()`

ToString method of Short3.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of Short3.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.Short3 op_Addition(Fusion.Short3 a, Fusion.Short3 b)`

op_Addition method of Short3.

**Parameters:**
- `a` (Fusion.Short3)
- `b` (Fusion.Short3)

**Returns:** `Fusion.Short3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.op_Addition%28Fusion.Short3%2CFusion.Short3%29)

#### `static Fusion.Short3 op_Addition(Fusion.Short3 a, short b)`

op_Addition method of Short3.

**Parameters:**
- `a` (Fusion.Short3)
- `b` (short)

**Returns:** `Fusion.Short3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.op_Addition%28Fusion.Short3%2CSystem.Int16%29)

#### `static Fusion.Short3 op_Addition(short a, Fusion.Short3 b)`

op_Addition method of Short3.

**Parameters:**
- `a` (short)
- `b` (Fusion.Short3)

**Returns:** `Fusion.Short3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.op_Addition%28System.Int16%2CFusion.Short3%29)

#### `static Fusion.Short3 op_Division(Fusion.Short3 a, Fusion.Short3 b)`

op_Division method of Short3.

**Parameters:**
- `a` (Fusion.Short3)
- `b` (Fusion.Short3)

**Returns:** `Fusion.Short3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.op_Division%28Fusion.Short3%2CFusion.Short3%29)

#### `static Fusion.Short3 op_Division(Fusion.Short3 a, short b)`

op_Division method of Short3.

**Parameters:**
- `a` (Fusion.Short3)
- `b` (short)

**Returns:** `Fusion.Short3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.op_Division%28Fusion.Short3%2CSystem.Int16%29)

#### `static Fusion.Short3 op_Division(short a, Fusion.Short3 b)`

op_Division method of Short3.

**Parameters:**
- `a` (short)
- `b` (Fusion.Short3)

**Returns:** `Fusion.Short3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.op_Division%28System.Int16%2CFusion.Short3%29)

#### `static bool op_Equality(Fusion.Short3 a, Fusion.Short3 b)`

op_Equality method of Short3.

**Parameters:**
- `a` (Fusion.Short3)
- `b` (Fusion.Short3)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.op_Equality%28Fusion.Short3%2CFusion.Short3%29)

#### `static Fusion.Short3 op_Explicit(Fusion.Double3 v)`

op_Explicit method of Short3.

**Parameters:**
- `v` (Fusion.Double3)

**Returns:** `Fusion.Short3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.op_Explicit%28Fusion.Double3%29~Fusion.Short3)

#### `static Fusion.Short3 op_Explicit(Fusion.Float3 v)`

op_Explicit method of Short3.

**Parameters:**
- `v` (Fusion.Float3)

**Returns:** `Fusion.Short3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.op_Explicit%28Fusion.Float3%29~Fusion.Short3)

#### `static Fusion.Short3 op_Explicit(Fusion.Half3 v)`

op_Explicit method of Short3.

**Parameters:**
- `v` (Fusion.Half3)

**Returns:** `Fusion.Short3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.op_Explicit%28Fusion.Half3%29~Fusion.Short3)

#### `static Fusion.Short3 op_Explicit(Fusion.Int3 v)`

op_Explicit method of Short3.

**Parameters:**
- `v` (Fusion.Int3)

**Returns:** `Fusion.Short3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.op_Explicit%28Fusion.Int3%29~Fusion.Short3)

#### `static Fusion.Short3 op_Explicit(Fusion.UShort3 v)`

op_Explicit method of Short3.

**Parameters:**
- `v` (Fusion.UShort3)

**Returns:** `Fusion.Short3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.op_Explicit%28Fusion.UShort3%29~Fusion.Short3)

#### `static bool op_Inequality(Fusion.Short3 a, Fusion.Short3 b)`

op_Inequality method of Short3.

**Parameters:**
- `a` (Fusion.Short3)
- `b` (Fusion.Short3)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.op_Inequality%28Fusion.Short3%2CFusion.Short3%29)

#### `static Fusion.Short3 op_Modulus(Fusion.Short3 a, Fusion.Short3 b)`

op_Modulus method of Short3.

**Parameters:**
- `a` (Fusion.Short3)
- `b` (Fusion.Short3)

**Returns:** `Fusion.Short3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.op_Modulus%28Fusion.Short3%2CFusion.Short3%29)

#### `static Fusion.Short3 op_Modulus(Fusion.Short3 a, short b)`

op_Modulus method of Short3.

**Parameters:**
- `a` (Fusion.Short3)
- `b` (short)

**Returns:** `Fusion.Short3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.op_Modulus%28Fusion.Short3%2CSystem.Int16%29)

#### `static Fusion.Short3 op_Modulus(short a, Fusion.Short3 b)`

op_Modulus method of Short3.

**Parameters:**
- `a` (short)
- `b` (Fusion.Short3)

**Returns:** `Fusion.Short3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.op_Modulus%28System.Int16%2CFusion.Short3%29)

#### `static Fusion.Short3 op_Multiply(Fusion.Short3 a, Fusion.Short3 b)`

op_Multiply method of Short3.

**Parameters:**
- `a` (Fusion.Short3)
- `b` (Fusion.Short3)

**Returns:** `Fusion.Short3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.op_Multiply%28Fusion.Short3%2CFusion.Short3%29)

#### `static Fusion.Short3 op_Multiply(Fusion.Short3 a, short b)`

op_Multiply method of Short3.

**Parameters:**
- `a` (Fusion.Short3)
- `b` (short)

**Returns:** `Fusion.Short3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.op_Multiply%28Fusion.Short3%2CSystem.Int16%29)

#### `static Fusion.Short3 op_Multiply(short a, Fusion.Short3 b)`

op_Multiply method of Short3.

**Parameters:**
- `a` (short)
- `b` (Fusion.Short3)

**Returns:** `Fusion.Short3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.op_Multiply%28System.Int16%2CFusion.Short3%29)

#### `static Fusion.Short3 op_Subtraction(Fusion.Short3 a, Fusion.Short3 b)`

op_Subtraction method of Short3.

**Parameters:**
- `a` (Fusion.Short3)
- `b` (Fusion.Short3)

**Returns:** `Fusion.Short3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.op_Subtraction%28Fusion.Short3%2CFusion.Short3%29)

#### `static Fusion.Short3 op_Subtraction(Fusion.Short3 a, short b)`

op_Subtraction method of Short3.

**Parameters:**
- `a` (Fusion.Short3)
- `b` (short)

**Returns:** `Fusion.Short3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.op_Subtraction%28Fusion.Short3%2CSystem.Int16%29)

#### `static Fusion.Short3 op_Subtraction(short a, Fusion.Short3 b)`

op_Subtraction method of Short3.

**Parameters:**
- `a` (short)
- `b` (Fusion.Short3)

**Returns:** `Fusion.Short3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.op_Subtraction%28System.Int16%2CFusion.Short3%29)

#### `static Fusion.Short3 op_UnaryNegation(Fusion.Short3 v)`

op_UnaryNegation method of Short3.

**Parameters:**
- `v` (Fusion.Short3)

**Returns:** `Fusion.Short3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.op_UnaryNegation%28Fusion.Short3%29)

#### `static Fusion.Short3 op_UnaryPlus(Fusion.Short3 v)`

op_UnaryPlus method of Short3.

**Parameters:**
- `v` (Fusion.Short3)

**Returns:** `Fusion.Short3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short3.op_UnaryPlus%28Fusion.Short3%29)

### Properties
- `Item` (short, get/set) — Item property of Short3.
- `XY` (Fusion.Short2, get) — XY property of Short3.

## Short4 (struct)

Short4 struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Short4)

### Constructors
- `Short4(Fusion.Short2 xy, Fusion.Short2 zw)` — Constructs a new Short4.
- `Short4(Fusion.Short2 xy, short z, short w)` — Constructs a new Short4.
- `Short4(Fusion.Short3 xyz, short w)` — Constructs a new Short4.
- `Short4(short all)` — Constructs a new Short4.
- `Short4(short x, Fusion.Short2 yz, short w)` — Constructs a new Short4.
- `Short4(short x, Fusion.Short3 yzw)` — Constructs a new Short4.
- `Short4(short x, short y, Fusion.Short2 zw)` — Constructs a new Short4.
- `Short4(short x, short y, short z, short w)` — Constructs a new Short4.

### Methods
#### `static Fusion.Short4 Abs(Fusion.Short4 value)`

Abs method of Short4.

**Parameters:**
- `value` (Fusion.Short4)

**Returns:** `Fusion.Short4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.Abs%28Fusion.Short4%29)

#### `bool Equals(Fusion.Short4 other)`

Equals method of Short4.

**Parameters:**
- `other` (Fusion.Short4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.Equals%28Fusion.Short4%29)

#### `bool Equals(object other)`

Equals method of Short4.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.Equals%28System.Object%29)

#### `int GetHashCode()`

GetHashCode method of Short4.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.GetHashCode)

#### `static Fusion.Short4 Max(Fusion.Short4 a, Fusion.Short4 b)`

Max method of Short4.

**Parameters:**
- `a` (Fusion.Short4)
- `b` (Fusion.Short4)

**Returns:** `Fusion.Short4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.Max%28Fusion.Short4%2CFusion.Short4%29)

#### `static Fusion.Short4 Min(Fusion.Short4 a, Fusion.Short4 b)`

Min method of Short4.

**Parameters:**
- `a` (Fusion.Short4)
- `b` (Fusion.Short4)

**Returns:** `Fusion.Short4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.Min%28Fusion.Short4%2CFusion.Short4%29)

#### `static Fusion.Int4 Sign(Fusion.Short4 value)`

Sign method of Short4.

**Parameters:**
- `value` (Fusion.Short4)

**Returns:** `Fusion.Int4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.Sign%28Fusion.Short4%29)

#### `string ToString()`

ToString method of Short4.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of Short4.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.Short4 op_Addition(Fusion.Short4 a, Fusion.Short4 b)`

op_Addition method of Short4.

**Parameters:**
- `a` (Fusion.Short4)
- `b` (Fusion.Short4)

**Returns:** `Fusion.Short4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.op_Addition%28Fusion.Short4%2CFusion.Short4%29)

#### `static Fusion.Short4 op_Addition(Fusion.Short4 a, short b)`

op_Addition method of Short4.

**Parameters:**
- `a` (Fusion.Short4)
- `b` (short)

**Returns:** `Fusion.Short4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.op_Addition%28Fusion.Short4%2CSystem.Int16%29)

#### `static Fusion.Short4 op_Addition(short a, Fusion.Short4 b)`

op_Addition method of Short4.

**Parameters:**
- `a` (short)
- `b` (Fusion.Short4)

**Returns:** `Fusion.Short4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.op_Addition%28System.Int16%2CFusion.Short4%29)

#### `static Fusion.Short4 op_Division(Fusion.Short4 a, Fusion.Short4 b)`

op_Division method of Short4.

**Parameters:**
- `a` (Fusion.Short4)
- `b` (Fusion.Short4)

**Returns:** `Fusion.Short4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.op_Division%28Fusion.Short4%2CFusion.Short4%29)

#### `static Fusion.Short4 op_Division(Fusion.Short4 a, short b)`

op_Division method of Short4.

**Parameters:**
- `a` (Fusion.Short4)
- `b` (short)

**Returns:** `Fusion.Short4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.op_Division%28Fusion.Short4%2CSystem.Int16%29)

#### `static Fusion.Short4 op_Division(short a, Fusion.Short4 b)`

op_Division method of Short4.

**Parameters:**
- `a` (short)
- `b` (Fusion.Short4)

**Returns:** `Fusion.Short4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.op_Division%28System.Int16%2CFusion.Short4%29)

#### `static bool op_Equality(Fusion.Short4 a, Fusion.Short4 b)`

op_Equality method of Short4.

**Parameters:**
- `a` (Fusion.Short4)
- `b` (Fusion.Short4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.op_Equality%28Fusion.Short4%2CFusion.Short4%29)

#### `static Fusion.Short4 op_Explicit(Fusion.Double4 v)`

op_Explicit method of Short4.

**Parameters:**
- `v` (Fusion.Double4)

**Returns:** `Fusion.Short4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.op_Explicit%28Fusion.Double4%29~Fusion.Short4)

#### `static Fusion.Short4 op_Explicit(Fusion.Float4 v)`

op_Explicit method of Short4.

**Parameters:**
- `v` (Fusion.Float4)

**Returns:** `Fusion.Short4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.op_Explicit%28Fusion.Float4%29~Fusion.Short4)

#### `static Fusion.Short4 op_Explicit(Fusion.Half4 v)`

op_Explicit method of Short4.

**Parameters:**
- `v` (Fusion.Half4)

**Returns:** `Fusion.Short4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.op_Explicit%28Fusion.Half4%29~Fusion.Short4)

#### `static Fusion.Short4 op_Explicit(Fusion.Int4 v)`

op_Explicit method of Short4.

**Parameters:**
- `v` (Fusion.Int4)

**Returns:** `Fusion.Short4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.op_Explicit%28Fusion.Int4%29~Fusion.Short4)

#### `static Fusion.Short4 op_Explicit(Fusion.UShort4 v)`

op_Explicit method of Short4.

**Parameters:**
- `v` (Fusion.UShort4)

**Returns:** `Fusion.Short4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.op_Explicit%28Fusion.UShort4%29~Fusion.Short4)

#### `static Fusion.Short4 op_Implicit(Fusion.RGBA v)`

op_Implicit method of Short4.

**Parameters:**
- `v` (Fusion.RGBA)

**Returns:** `Fusion.Short4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.op_Implicit%28Fusion.RGBA%29~Fusion.Short4)

#### `static bool op_Inequality(Fusion.Short4 a, Fusion.Short4 b)`

op_Inequality method of Short4.

**Parameters:**
- `a` (Fusion.Short4)
- `b` (Fusion.Short4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.op_Inequality%28Fusion.Short4%2CFusion.Short4%29)

#### `static Fusion.Short4 op_Modulus(Fusion.Short4 a, Fusion.Short4 b)`

op_Modulus method of Short4.

**Parameters:**
- `a` (Fusion.Short4)
- `b` (Fusion.Short4)

**Returns:** `Fusion.Short4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.op_Modulus%28Fusion.Short4%2CFusion.Short4%29)

#### `static Fusion.Short4 op_Modulus(Fusion.Short4 a, short b)`

op_Modulus method of Short4.

**Parameters:**
- `a` (Fusion.Short4)
- `b` (short)

**Returns:** `Fusion.Short4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.op_Modulus%28Fusion.Short4%2CSystem.Int16%29)

#### `static Fusion.Short4 op_Modulus(short a, Fusion.Short4 b)`

op_Modulus method of Short4.

**Parameters:**
- `a` (short)
- `b` (Fusion.Short4)

**Returns:** `Fusion.Short4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.op_Modulus%28System.Int16%2CFusion.Short4%29)

#### `static Fusion.Short4 op_Multiply(Fusion.Short4 a, Fusion.Short4 b)`

op_Multiply method of Short4.

**Parameters:**
- `a` (Fusion.Short4)
- `b` (Fusion.Short4)

**Returns:** `Fusion.Short4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.op_Multiply%28Fusion.Short4%2CFusion.Short4%29)

#### `static Fusion.Short4 op_Multiply(Fusion.Short4 a, short b)`

op_Multiply method of Short4.

**Parameters:**
- `a` (Fusion.Short4)
- `b` (short)

**Returns:** `Fusion.Short4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.op_Multiply%28Fusion.Short4%2CSystem.Int16%29)

#### `static Fusion.Short4 op_Multiply(short a, Fusion.Short4 b)`

op_Multiply method of Short4.

**Parameters:**
- `a` (short)
- `b` (Fusion.Short4)

**Returns:** `Fusion.Short4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.op_Multiply%28System.Int16%2CFusion.Short4%29)

#### `static Fusion.Short4 op_Subtraction(Fusion.Short4 a, Fusion.Short4 b)`

op_Subtraction method of Short4.

**Parameters:**
- `a` (Fusion.Short4)
- `b` (Fusion.Short4)

**Returns:** `Fusion.Short4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.op_Subtraction%28Fusion.Short4%2CFusion.Short4%29)

#### `static Fusion.Short4 op_Subtraction(Fusion.Short4 a, short b)`

op_Subtraction method of Short4.

**Parameters:**
- `a` (Fusion.Short4)
- `b` (short)

**Returns:** `Fusion.Short4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.op_Subtraction%28Fusion.Short4%2CSystem.Int16%29)

#### `static Fusion.Short4 op_Subtraction(short a, Fusion.Short4 b)`

op_Subtraction method of Short4.

**Parameters:**
- `a` (short)
- `b` (Fusion.Short4)

**Returns:** `Fusion.Short4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.op_Subtraction%28System.Int16%2CFusion.Short4%29)

#### `static Fusion.Short4 op_UnaryNegation(Fusion.Short4 v)`

op_UnaryNegation method of Short4.

**Parameters:**
- `v` (Fusion.Short4)

**Returns:** `Fusion.Short4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.op_UnaryNegation%28Fusion.Short4%29)

#### `static Fusion.Short4 op_UnaryPlus(Fusion.Short4 v)`

op_UnaryPlus method of Short4.

**Parameters:**
- `v` (Fusion.Short4)

**Returns:** `Fusion.Short4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Short4.op_UnaryPlus%28Fusion.Short4%29)

### Properties
- `Item` (short, get/set) — Item property of Short4.
- `XY` (Fusion.Short2, get) — XY property of Short4.
- `XYZ` (Fusion.Short3, get) — XYZ property of Short4.

## Temperature (struct)

Temperature struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Temperature)

### Constructors
- `Temperature(double kelvin)` — Constructs a new Temperature.

### Methods
#### `static Fusion.Temperature Abs(Fusion.Temperature temperature)`

Abs method of Temperature.

**Parameters:**
- `temperature` (Fusion.Temperature)

**Returns:** `Fusion.Temperature` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.Abs%28Fusion.Temperature%29)

#### `int CompareTo(Fusion.Temperature other)`

CompareTo method of Temperature.

**Parameters:**
- `other` (Fusion.Temperature)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.CompareTo%28Fusion.Temperature%29)

#### `int CompareTo(object other)`

CompareTo method of Temperature.

**Parameters:**
- `other` (object)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.CompareTo%28System.Object%29)

#### `bool Equals(Fusion.Temperature other)`

Equals method of Temperature.

**Parameters:**
- `other` (Fusion.Temperature)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.Equals%28Fusion.Temperature%29)

#### `bool Equals(object other)`

Equals method of Temperature.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.Equals%28System.Object%29)

#### `static Fusion.Temperature From(double temperature, Fusion.TemperatureUnit temperatureUnit)`

From method of Temperature.

**Parameters:**
- `temperature` (double)
- `temperatureUnit` (Fusion.TemperatureUnit)

**Returns:** `Fusion.Temperature` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.From%28System.Double%2CFusion.TemperatureUnit%29)

#### `int GetHashCode()`

GetHashCode method of Temperature.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.GetHashCode)

#### `static bool IsInfinity(Fusion.Temperature temperature)`

IsInfinity method of Temperature.

**Parameters:**
- `temperature` (Fusion.Temperature)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.IsInfinity%28Fusion.Temperature%29)

#### `static bool IsNaN(Fusion.Temperature temperature)`

IsNaN method of Temperature.

**Parameters:**
- `temperature` (Fusion.Temperature)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.IsNaN%28Fusion.Temperature%29)

#### `static bool IsNegativeInfinity(Fusion.Temperature temperature)`

IsNegativeInfinity method of Temperature.

**Parameters:**
- `temperature` (Fusion.Temperature)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.IsNegativeInfinity%28Fusion.Temperature%29)

#### `static bool IsPositiveInfinity(Fusion.Temperature temperature)`

IsPositiveInfinity method of Temperature.

**Parameters:**
- `temperature` (Fusion.Temperature)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.IsPositiveInfinity%28Fusion.Temperature%29)

#### `static Fusion.Temperature Max(Fusion.Temperature a, Fusion.Temperature b)`

Max method of Temperature.

**Parameters:**
- `a` (Fusion.Temperature)
- `b` (Fusion.Temperature)

**Returns:** `Fusion.Temperature` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.Max%28Fusion.Temperature%2CFusion.Temperature%29)

#### `static Fusion.Temperature Min(Fusion.Temperature a, Fusion.Temperature b)`

Min method of Temperature.

**Parameters:**
- `a` (Fusion.Temperature)
- `b` (Fusion.Temperature)

**Returns:** `Fusion.Temperature` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.Min%28Fusion.Temperature%2CFusion.Temperature%29)

#### `bool Near(Fusion.Temperature other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of Temperature.

**Parameters:**
- `other` (Fusion.Temperature)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.Near%28Fusion.Temperature%2CSystem.Int32%29)

#### `static Fusion.Temperature Round(Fusion.Temperature temperature, Fusion.Temperature precision, Fusion.RoundingMode roundingMode)`

Round method of Temperature.

**Parameters:**
- `temperature` (Fusion.Temperature)
- `precision` (Fusion.Temperature)
- `roundingMode` (Fusion.RoundingMode)

**Returns:** `Fusion.Temperature` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.Round%28Fusion.Temperature%2CFusion.Temperature%2CFusion.RoundingMode%29)

#### `static int Sign(Fusion.Temperature temperature)`

Sign method of Temperature.

**Parameters:**
- `temperature` (Fusion.Temperature)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.Sign%28Fusion.Temperature%29)

#### `double To(Fusion.TemperatureUnit temperatureUnit)`

To method of Temperature.

**Parameters:**
- `temperatureUnit` (Fusion.TemperatureUnit)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.To%28Fusion.TemperatureUnit%29)

#### `string ToString()`

ToString method of Temperature.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of Temperature.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.Temperature op_Addition(Fusion.Temperature a, Fusion.Temperature b)`

op_Addition method of Temperature.

**Parameters:**
- `a` (Fusion.Temperature)
- `b` (Fusion.Temperature)

**Returns:** `Fusion.Temperature` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.op_Addition%28Fusion.Temperature%2CFusion.Temperature%29)

#### `static Fusion.Temperature op_Division(Fusion.Temperature a, double b)`

op_Division method of Temperature.

**Parameters:**
- `a` (Fusion.Temperature)
- `b` (double)

**Returns:** `Fusion.Temperature` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.op_Division%28Fusion.Temperature%2CSystem.Double%29)

#### `static double op_Division(Fusion.Temperature a, Fusion.Temperature b)`

op_Division method of Temperature.

**Parameters:**
- `a` (Fusion.Temperature)
- `b` (Fusion.Temperature)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.op_Division%28Fusion.Temperature%2CFusion.Temperature%29)

#### `static bool op_Equality(Fusion.Temperature a, Fusion.Temperature b)`

op_Equality method of Temperature.

**Parameters:**
- `a` (Fusion.Temperature)
- `b` (Fusion.Temperature)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.op_Equality%28Fusion.Temperature%2CFusion.Temperature%29)

#### `static bool op_GreaterThan(Fusion.Temperature a, Fusion.Temperature b)`

op_GreaterThan method of Temperature.

**Parameters:**
- `a` (Fusion.Temperature)
- `b` (Fusion.Temperature)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.op_GreaterThan%28Fusion.Temperature%2CFusion.Temperature%29)

#### `static bool op_GreaterThanOrEqual(Fusion.Temperature a, Fusion.Temperature b)`

op_GreaterThanOrEqual method of Temperature.

**Parameters:**
- `a` (Fusion.Temperature)
- `b` (Fusion.Temperature)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.op_GreaterThanOrEqual%28Fusion.Temperature%2CFusion.Temperature%29)

#### `static bool op_Inequality(Fusion.Temperature a, Fusion.Temperature b)`

op_Inequality method of Temperature.

**Parameters:**
- `a` (Fusion.Temperature)
- `b` (Fusion.Temperature)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.op_Inequality%28Fusion.Temperature%2CFusion.Temperature%29)

#### `static bool op_LessThan(Fusion.Temperature a, Fusion.Temperature b)`

op_LessThan method of Temperature.

**Parameters:**
- `a` (Fusion.Temperature)
- `b` (Fusion.Temperature)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.op_LessThan%28Fusion.Temperature%2CFusion.Temperature%29)

#### `static bool op_LessThanOrEqual(Fusion.Temperature a, Fusion.Temperature b)`

op_LessThanOrEqual method of Temperature.

**Parameters:**
- `a` (Fusion.Temperature)
- `b` (Fusion.Temperature)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.op_LessThanOrEqual%28Fusion.Temperature%2CFusion.Temperature%29)

#### `static Fusion.Temperature op_Modulus(Fusion.Temperature a, Fusion.Temperature b)`

op_Modulus method of Temperature.

**Parameters:**
- `a` (Fusion.Temperature)
- `b` (Fusion.Temperature)

**Returns:** `Fusion.Temperature` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.op_Modulus%28Fusion.Temperature%2CFusion.Temperature%29)

#### `static Fusion.Temperature op_Multiply(Fusion.Temperature a, double b)`

op_Multiply method of Temperature.

**Parameters:**
- `a` (Fusion.Temperature)
- `b` (double)

**Returns:** `Fusion.Temperature` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.op_Multiply%28Fusion.Temperature%2CSystem.Double%29)

#### `static Fusion.Temperature op_Multiply(double a, Fusion.Temperature b)`

op_Multiply method of Temperature.

**Parameters:**
- `a` (double)
- `b` (Fusion.Temperature)

**Returns:** `Fusion.Temperature` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.op_Multiply%28System.Double%2CFusion.Temperature%29)

#### `static Fusion.Temperature op_Subtraction(Fusion.Temperature a, Fusion.Temperature b)`

op_Subtraction method of Temperature.

**Parameters:**
- `a` (Fusion.Temperature)
- `b` (Fusion.Temperature)

**Returns:** `Fusion.Temperature` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.op_Subtraction%28Fusion.Temperature%2CFusion.Temperature%29)

#### `static Fusion.Temperature op_UnaryNegation(Fusion.Temperature a)`

op_UnaryNegation method of Temperature.

**Parameters:**
- `a` (Fusion.Temperature)

**Returns:** `Fusion.Temperature` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Temperature.op_UnaryNegation%28Fusion.Temperature%29)

### Properties
- `Kelvin` (double, get) — Kelvin property of Temperature.

## TemperatureUnit (enum)

TemperatureUnit enum in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.TemperatureUnit)

### Values
- `Kelvin` = `0`
- `Celsius` = `1`
- `Fahrenheit` = `2`

## ThemeResourcesAttribute (class)

ThemeResourcesAttribute class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.ThemeResourcesAttribute)

### Constructors
- `ThemeResourcesAttribute(string resourcePath)` — Constructs a new ThemeResourcesAttribute.

## ThreadingAttribute (class)

ThreadingAttribute class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.ThreadingAttribute)

### Constructors
- `ThreadingAttribute(string threadingInformation)` — Constructs a new ThreadingAttribute.

### Properties
- `ThreadingInformation` (string, get) — ThreadingInformation property of ThreadingAttribute.

## Triangle (struct)

Triangle struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Triangle)

### Constructors
- `Triangle(Fusion.Float3 v1, Fusion.Float3 v2, Fusion.Float3 v3)` — Constructs a new Triangle.

### Methods
#### `bool Equals(Fusion.Triangle other)`

Equals method of Triangle.

**Parameters:**
- `other` (Fusion.Triangle)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Triangle.Equals%28Fusion.Triangle%29)

#### `bool Equals(object other)`

Equals method of Triangle.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Triangle.Equals%28System.Object%29)

#### `int GetHashCode()`

GetHashCode method of Triangle.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Triangle.GetHashCode)

#### `bool Near(Fusion.Triangle other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of Triangle.

**Parameters:**
- `other` (Fusion.Triangle)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Triangle.Near%28Fusion.Triangle%2CSystem.Int32%29)

#### `static bool op_Equality(Fusion.Triangle a, Fusion.Triangle b)`

op_Equality method of Triangle.

**Parameters:**
- `a` (Fusion.Triangle)
- `b` (Fusion.Triangle)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Triangle.op_Equality%28Fusion.Triangle%2CFusion.Triangle%29)

#### `static bool op_Inequality(Fusion.Triangle a, Fusion.Triangle b)`

op_Inequality method of Triangle.

**Parameters:**
- `a` (Fusion.Triangle)
- `b` (Fusion.Triangle)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Triangle.op_Inequality%28Fusion.Triangle%2CFusion.Triangle%29)

### Properties
- `Center` (Fusion.Float3, get) — Center property of Triangle.
- `Normal` (Fusion.Float3, get) — Normal property of Triangle.

## TriangleMesh (class)

TriangleMesh class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.TriangleMesh)

### Constructors
- `TriangleMesh(System.Collections.Generic.IReadOnlyList<Fusion.Float3> points, System.Collections.Generic.IReadOnlyList<Fusion.UShort3> faces)` — Constructs a new TriangleMesh.

### Methods
#### `bool Equals(Fusion.TriangleMesh other)`

Equals method of TriangleMesh.

**Parameters:**
- `other` (Fusion.TriangleMesh)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.TriangleMesh.Equals%28Fusion.TriangleMesh%29)

#### `bool Equals(object other)`

Equals method of TriangleMesh.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.TriangleMesh.Equals%28System.Object%29)

#### `System.Collections.Generic.IEnumerator<Fusion.Triangle> GetEnumerator()`

GetEnumerator method of TriangleMesh.

**Returns:** `System.Collections.Generic.IEnumerator<Fusion.Triangle>` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.TriangleMesh.GetEnumerator)

#### `int GetHashCode()`

GetHashCode method of TriangleMesh.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.TriangleMesh.GetHashCode)

### Properties
- `Faces` (System.Collections.Generic.IReadOnlyList<Fusion.UShort3>, get) — Faces property of TriangleMesh.
- `Points` (System.Collections.Generic.IReadOnlyList<Fusion.Float3>, get) — Points property of TriangleMesh.

## UShort2 (struct)

UShort2 struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.UShort2)

### Constructors
- `UShort2(ushort all)` — Constructs a new UShort2.
- `UShort2(ushort x, ushort y)` — Constructs a new UShort2.

### Methods
#### `bool Equals(Fusion.UShort2 other)`

Equals method of UShort2.

**Parameters:**
- `other` (Fusion.UShort2)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort2.Equals%28Fusion.UShort2%29)

#### `bool Equals(object other)`

Equals method of UShort2.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort2.Equals%28System.Object%29)

#### `int GetHashCode()`

GetHashCode method of UShort2.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort2.GetHashCode)

#### `static Fusion.UShort2 Max(Fusion.UShort2 a, Fusion.UShort2 b)`

Max method of UShort2.

**Parameters:**
- `a` (Fusion.UShort2)
- `b` (Fusion.UShort2)

**Returns:** `Fusion.UShort2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort2.Max%28Fusion.UShort2%2CFusion.UShort2%29)

#### `static Fusion.UShort2 Min(Fusion.UShort2 a, Fusion.UShort2 b)`

Min method of UShort2.

**Parameters:**
- `a` (Fusion.UShort2)
- `b` (Fusion.UShort2)

**Returns:** `Fusion.UShort2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort2.Min%28Fusion.UShort2%2CFusion.UShort2%29)

#### `string ToString()`

ToString method of UShort2.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort2.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of UShort2.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort2.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.UShort2 op_Addition(Fusion.UShort2 a, Fusion.UShort2 b)`

op_Addition method of UShort2.

**Parameters:**
- `a` (Fusion.UShort2)
- `b` (Fusion.UShort2)

**Returns:** `Fusion.UShort2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort2.op_Addition%28Fusion.UShort2%2CFusion.UShort2%29)

#### `static Fusion.UShort2 op_Addition(Fusion.UShort2 a, ushort b)`

op_Addition method of UShort2.

**Parameters:**
- `a` (Fusion.UShort2)
- `b` (ushort)

**Returns:** `Fusion.UShort2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort2.op_Addition%28Fusion.UShort2%2CSystem.UInt16%29)

#### `static Fusion.UShort2 op_Addition(ushort a, Fusion.UShort2 b)`

op_Addition method of UShort2.

**Parameters:**
- `a` (ushort)
- `b` (Fusion.UShort2)

**Returns:** `Fusion.UShort2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort2.op_Addition%28System.UInt16%2CFusion.UShort2%29)

#### `static Fusion.UShort2 op_Division(Fusion.UShort2 a, Fusion.UShort2 b)`

op_Division method of UShort2.

**Parameters:**
- `a` (Fusion.UShort2)
- `b` (Fusion.UShort2)

**Returns:** `Fusion.UShort2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort2.op_Division%28Fusion.UShort2%2CFusion.UShort2%29)

#### `static Fusion.UShort2 op_Division(Fusion.UShort2 a, ushort b)`

op_Division method of UShort2.

**Parameters:**
- `a` (Fusion.UShort2)
- `b` (ushort)

**Returns:** `Fusion.UShort2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort2.op_Division%28Fusion.UShort2%2CSystem.UInt16%29)

#### `static Fusion.UShort2 op_Division(ushort a, Fusion.UShort2 b)`

op_Division method of UShort2.

**Parameters:**
- `a` (ushort)
- `b` (Fusion.UShort2)

**Returns:** `Fusion.UShort2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort2.op_Division%28System.UInt16%2CFusion.UShort2%29)

#### `static bool op_Equality(Fusion.UShort2 a, Fusion.UShort2 b)`

op_Equality method of UShort2.

**Parameters:**
- `a` (Fusion.UShort2)
- `b` (Fusion.UShort2)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort2.op_Equality%28Fusion.UShort2%2CFusion.UShort2%29)

#### `static Fusion.UShort2 op_Explicit(Fusion.Double2 v)`

op_Explicit method of UShort2.

**Parameters:**
- `v` (Fusion.Double2)

**Returns:** `Fusion.UShort2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort2.op_Explicit%28Fusion.Double2%29~Fusion.UShort2)

#### `static Fusion.UShort2 op_Explicit(Fusion.Float2 v)`

op_Explicit method of UShort2.

**Parameters:**
- `v` (Fusion.Float2)

**Returns:** `Fusion.UShort2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort2.op_Explicit%28Fusion.Float2%29~Fusion.UShort2)

#### `static Fusion.UShort2 op_Explicit(Fusion.Half2 v)`

op_Explicit method of UShort2.

**Parameters:**
- `v` (Fusion.Half2)

**Returns:** `Fusion.UShort2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort2.op_Explicit%28Fusion.Half2%29~Fusion.UShort2)

#### `static Fusion.UShort2 op_Explicit(Fusion.Int2 v)`

op_Explicit method of UShort2.

**Parameters:**
- `v` (Fusion.Int2)

**Returns:** `Fusion.UShort2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort2.op_Explicit%28Fusion.Int2%29~Fusion.UShort2)

#### `static Fusion.UShort2 op_Explicit(Fusion.Short2 v)`

op_Explicit method of UShort2.

**Parameters:**
- `v` (Fusion.Short2)

**Returns:** `Fusion.UShort2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort2.op_Explicit%28Fusion.Short2%29~Fusion.UShort2)

#### `static bool op_Inequality(Fusion.UShort2 a, Fusion.UShort2 b)`

op_Inequality method of UShort2.

**Parameters:**
- `a` (Fusion.UShort2)
- `b` (Fusion.UShort2)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort2.op_Inequality%28Fusion.UShort2%2CFusion.UShort2%29)

#### `static Fusion.UShort2 op_Modulus(Fusion.UShort2 a, Fusion.UShort2 b)`

op_Modulus method of UShort2.

**Parameters:**
- `a` (Fusion.UShort2)
- `b` (Fusion.UShort2)

**Returns:** `Fusion.UShort2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort2.op_Modulus%28Fusion.UShort2%2CFusion.UShort2%29)

#### `static Fusion.UShort2 op_Modulus(Fusion.UShort2 a, ushort b)`

op_Modulus method of UShort2.

**Parameters:**
- `a` (Fusion.UShort2)
- `b` (ushort)

**Returns:** `Fusion.UShort2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort2.op_Modulus%28Fusion.UShort2%2CSystem.UInt16%29)

#### `static Fusion.UShort2 op_Modulus(ushort a, Fusion.UShort2 b)`

op_Modulus method of UShort2.

**Parameters:**
- `a` (ushort)
- `b` (Fusion.UShort2)

**Returns:** `Fusion.UShort2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort2.op_Modulus%28System.UInt16%2CFusion.UShort2%29)

#### `static Fusion.UShort2 op_Multiply(Fusion.UShort2 a, Fusion.UShort2 b)`

op_Multiply method of UShort2.

**Parameters:**
- `a` (Fusion.UShort2)
- `b` (Fusion.UShort2)

**Returns:** `Fusion.UShort2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort2.op_Multiply%28Fusion.UShort2%2CFusion.UShort2%29)

#### `static Fusion.UShort2 op_Multiply(Fusion.UShort2 a, ushort b)`

op_Multiply method of UShort2.

**Parameters:**
- `a` (Fusion.UShort2)
- `b` (ushort)

**Returns:** `Fusion.UShort2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort2.op_Multiply%28Fusion.UShort2%2CSystem.UInt16%29)

#### `static Fusion.UShort2 op_Multiply(ushort a, Fusion.UShort2 b)`

op_Multiply method of UShort2.

**Parameters:**
- `a` (ushort)
- `b` (Fusion.UShort2)

**Returns:** `Fusion.UShort2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort2.op_Multiply%28System.UInt16%2CFusion.UShort2%29)

#### `static Fusion.UShort2 op_Subtraction(Fusion.UShort2 a, Fusion.UShort2 b)`

op_Subtraction method of UShort2.

**Parameters:**
- `a` (Fusion.UShort2)
- `b` (Fusion.UShort2)

**Returns:** `Fusion.UShort2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort2.op_Subtraction%28Fusion.UShort2%2CFusion.UShort2%29)

#### `static Fusion.UShort2 op_Subtraction(Fusion.UShort2 a, ushort b)`

op_Subtraction method of UShort2.

**Parameters:**
- `a` (Fusion.UShort2)
- `b` (ushort)

**Returns:** `Fusion.UShort2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort2.op_Subtraction%28Fusion.UShort2%2CSystem.UInt16%29)

#### `static Fusion.UShort2 op_Subtraction(ushort a, Fusion.UShort2 b)`

op_Subtraction method of UShort2.

**Parameters:**
- `a` (ushort)
- `b` (Fusion.UShort2)

**Returns:** `Fusion.UShort2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort2.op_Subtraction%28System.UInt16%2CFusion.UShort2%29)

#### `static Fusion.UShort2 op_UnaryNegation(Fusion.UShort2 v)`

op_UnaryNegation method of UShort2.

**Parameters:**
- `v` (Fusion.UShort2)

**Returns:** `Fusion.UShort2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort2.op_UnaryNegation%28Fusion.UShort2%29)

#### `static Fusion.UShort2 op_UnaryPlus(Fusion.UShort2 v)`

op_UnaryPlus method of UShort2.

**Parameters:**
- `v` (Fusion.UShort2)

**Returns:** `Fusion.UShort2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort2.op_UnaryPlus%28Fusion.UShort2%29)

### Properties
- `Item` (ushort, get/set) — Item property of UShort2.

## UShort3 (struct)

UShort3 struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.UShort3)

### Constructors
- `UShort3(Fusion.UShort2 xy, ushort z)` — Constructs a new UShort3.
- `UShort3(ushort all)` — Constructs a new UShort3.
- `UShort3(ushort x, Fusion.UShort2 yz)` — Constructs a new UShort3.
- `UShort3(ushort x, ushort y, ushort z)` — Constructs a new UShort3.

### Methods
#### `bool Equals(Fusion.UShort3 other)`

Equals method of UShort3.

**Parameters:**
- `other` (Fusion.UShort3)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort3.Equals%28Fusion.UShort3%29)

#### `bool Equals(object other)`

Equals method of UShort3.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort3.Equals%28System.Object%29)

#### `int GetHashCode()`

GetHashCode method of UShort3.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort3.GetHashCode)

#### `static Fusion.UShort3 Max(Fusion.UShort3 a, Fusion.UShort3 b)`

Max method of UShort3.

**Parameters:**
- `a` (Fusion.UShort3)
- `b` (Fusion.UShort3)

**Returns:** `Fusion.UShort3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort3.Max%28Fusion.UShort3%2CFusion.UShort3%29)

#### `static Fusion.UShort3 Min(Fusion.UShort3 a, Fusion.UShort3 b)`

Min method of UShort3.

**Parameters:**
- `a` (Fusion.UShort3)
- `b` (Fusion.UShort3)

**Returns:** `Fusion.UShort3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort3.Min%28Fusion.UShort3%2CFusion.UShort3%29)

#### `string ToString()`

ToString method of UShort3.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort3.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of UShort3.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort3.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.UShort3 op_Addition(Fusion.UShort3 a, Fusion.UShort3 b)`

op_Addition method of UShort3.

**Parameters:**
- `a` (Fusion.UShort3)
- `b` (Fusion.UShort3)

**Returns:** `Fusion.UShort3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort3.op_Addition%28Fusion.UShort3%2CFusion.UShort3%29)

#### `static Fusion.UShort3 op_Addition(Fusion.UShort3 a, ushort b)`

op_Addition method of UShort3.

**Parameters:**
- `a` (Fusion.UShort3)
- `b` (ushort)

**Returns:** `Fusion.UShort3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort3.op_Addition%28Fusion.UShort3%2CSystem.UInt16%29)

#### `static Fusion.UShort3 op_Addition(ushort a, Fusion.UShort3 b)`

op_Addition method of UShort3.

**Parameters:**
- `a` (ushort)
- `b` (Fusion.UShort3)

**Returns:** `Fusion.UShort3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort3.op_Addition%28System.UInt16%2CFusion.UShort3%29)

#### `static Fusion.UShort3 op_Division(Fusion.UShort3 a, Fusion.UShort3 b)`

op_Division method of UShort3.

**Parameters:**
- `a` (Fusion.UShort3)
- `b` (Fusion.UShort3)

**Returns:** `Fusion.UShort3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort3.op_Division%28Fusion.UShort3%2CFusion.UShort3%29)

#### `static Fusion.UShort3 op_Division(Fusion.UShort3 a, ushort b)`

op_Division method of UShort3.

**Parameters:**
- `a` (Fusion.UShort3)
- `b` (ushort)

**Returns:** `Fusion.UShort3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort3.op_Division%28Fusion.UShort3%2CSystem.UInt16%29)

#### `static Fusion.UShort3 op_Division(ushort a, Fusion.UShort3 b)`

op_Division method of UShort3.

**Parameters:**
- `a` (ushort)
- `b` (Fusion.UShort3)

**Returns:** `Fusion.UShort3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort3.op_Division%28System.UInt16%2CFusion.UShort3%29)

#### `static bool op_Equality(Fusion.UShort3 a, Fusion.UShort3 b)`

op_Equality method of UShort3.

**Parameters:**
- `a` (Fusion.UShort3)
- `b` (Fusion.UShort3)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort3.op_Equality%28Fusion.UShort3%2CFusion.UShort3%29)

#### `static Fusion.UShort3 op_Explicit(Fusion.Double3 v)`

op_Explicit method of UShort3.

**Parameters:**
- `v` (Fusion.Double3)

**Returns:** `Fusion.UShort3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort3.op_Explicit%28Fusion.Double3%29~Fusion.UShort3)

#### `static Fusion.UShort3 op_Explicit(Fusion.Float3 v)`

op_Explicit method of UShort3.

**Parameters:**
- `v` (Fusion.Float3)

**Returns:** `Fusion.UShort3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort3.op_Explicit%28Fusion.Float3%29~Fusion.UShort3)

#### `static Fusion.UShort3 op_Explicit(Fusion.Half3 v)`

op_Explicit method of UShort3.

**Parameters:**
- `v` (Fusion.Half3)

**Returns:** `Fusion.UShort3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort3.op_Explicit%28Fusion.Half3%29~Fusion.UShort3)

#### `static Fusion.UShort3 op_Explicit(Fusion.Int3 v)`

op_Explicit method of UShort3.

**Parameters:**
- `v` (Fusion.Int3)

**Returns:** `Fusion.UShort3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort3.op_Explicit%28Fusion.Int3%29~Fusion.UShort3)

#### `static Fusion.UShort3 op_Explicit(Fusion.Short3 v)`

op_Explicit method of UShort3.

**Parameters:**
- `v` (Fusion.Short3)

**Returns:** `Fusion.UShort3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort3.op_Explicit%28Fusion.Short3%29~Fusion.UShort3)

#### `static bool op_Inequality(Fusion.UShort3 a, Fusion.UShort3 b)`

op_Inequality method of UShort3.

**Parameters:**
- `a` (Fusion.UShort3)
- `b` (Fusion.UShort3)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort3.op_Inequality%28Fusion.UShort3%2CFusion.UShort3%29)

#### `static Fusion.UShort3 op_Modulus(Fusion.UShort3 a, Fusion.UShort3 b)`

op_Modulus method of UShort3.

**Parameters:**
- `a` (Fusion.UShort3)
- `b` (Fusion.UShort3)

**Returns:** `Fusion.UShort3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort3.op_Modulus%28Fusion.UShort3%2CFusion.UShort3%29)

#### `static Fusion.UShort3 op_Modulus(Fusion.UShort3 a, ushort b)`

op_Modulus method of UShort3.

**Parameters:**
- `a` (Fusion.UShort3)
- `b` (ushort)

**Returns:** `Fusion.UShort3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort3.op_Modulus%28Fusion.UShort3%2CSystem.UInt16%29)

#### `static Fusion.UShort3 op_Modulus(ushort a, Fusion.UShort3 b)`

op_Modulus method of UShort3.

**Parameters:**
- `a` (ushort)
- `b` (Fusion.UShort3)

**Returns:** `Fusion.UShort3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort3.op_Modulus%28System.UInt16%2CFusion.UShort3%29)

#### `static Fusion.UShort3 op_Multiply(Fusion.UShort3 a, Fusion.UShort3 b)`

op_Multiply method of UShort3.

**Parameters:**
- `a` (Fusion.UShort3)
- `b` (Fusion.UShort3)

**Returns:** `Fusion.UShort3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort3.op_Multiply%28Fusion.UShort3%2CFusion.UShort3%29)

#### `static Fusion.UShort3 op_Multiply(Fusion.UShort3 a, ushort b)`

op_Multiply method of UShort3.

**Parameters:**
- `a` (Fusion.UShort3)
- `b` (ushort)

**Returns:** `Fusion.UShort3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort3.op_Multiply%28Fusion.UShort3%2CSystem.UInt16%29)

#### `static Fusion.UShort3 op_Multiply(ushort a, Fusion.UShort3 b)`

op_Multiply method of UShort3.

**Parameters:**
- `a` (ushort)
- `b` (Fusion.UShort3)

**Returns:** `Fusion.UShort3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort3.op_Multiply%28System.UInt16%2CFusion.UShort3%29)

#### `static Fusion.UShort3 op_Subtraction(Fusion.UShort3 a, Fusion.UShort3 b)`

op_Subtraction method of UShort3.

**Parameters:**
- `a` (Fusion.UShort3)
- `b` (Fusion.UShort3)

**Returns:** `Fusion.UShort3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort3.op_Subtraction%28Fusion.UShort3%2CFusion.UShort3%29)

#### `static Fusion.UShort3 op_Subtraction(Fusion.UShort3 a, ushort b)`

op_Subtraction method of UShort3.

**Parameters:**
- `a` (Fusion.UShort3)
- `b` (ushort)

**Returns:** `Fusion.UShort3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort3.op_Subtraction%28Fusion.UShort3%2CSystem.UInt16%29)

#### `static Fusion.UShort3 op_Subtraction(ushort a, Fusion.UShort3 b)`

op_Subtraction method of UShort3.

**Parameters:**
- `a` (ushort)
- `b` (Fusion.UShort3)

**Returns:** `Fusion.UShort3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort3.op_Subtraction%28System.UInt16%2CFusion.UShort3%29)

#### `static Fusion.UShort3 op_UnaryNegation(Fusion.UShort3 v)`

op_UnaryNegation method of UShort3.

**Parameters:**
- `v` (Fusion.UShort3)

**Returns:** `Fusion.UShort3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort3.op_UnaryNegation%28Fusion.UShort3%29)

#### `static Fusion.UShort3 op_UnaryPlus(Fusion.UShort3 v)`

op_UnaryPlus method of UShort3.

**Parameters:**
- `v` (Fusion.UShort3)

**Returns:** `Fusion.UShort3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort3.op_UnaryPlus%28Fusion.UShort3%29)

### Properties
- `Item` (ushort, get/set) — Item property of UShort3.
- `XY` (Fusion.UShort2, get) — XY property of UShort3.

## UShort4 (struct)

UShort4 struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.UShort4)

### Constructors
- `UShort4(Fusion.UShort2 xy, Fusion.UShort2 zw)` — Constructs a new UShort4.
- `UShort4(Fusion.UShort2 xy, ushort z, ushort w)` — Constructs a new UShort4.
- `UShort4(Fusion.UShort3 xyz, ushort w)` — Constructs a new UShort4.
- `UShort4(ushort all)` — Constructs a new UShort4.
- `UShort4(ushort x, Fusion.UShort2 yz, ushort w)` — Constructs a new UShort4.
- `UShort4(ushort x, Fusion.UShort3 yzw)` — Constructs a new UShort4.
- `UShort4(ushort x, ushort y, Fusion.UShort2 zw)` — Constructs a new UShort4.
- `UShort4(ushort x, ushort y, ushort z, ushort w)` — Constructs a new UShort4.

### Methods
#### `bool Equals(Fusion.UShort4 other)`

Equals method of UShort4.

**Parameters:**
- `other` (Fusion.UShort4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort4.Equals%28Fusion.UShort4%29)

#### `bool Equals(object other)`

Equals method of UShort4.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort4.Equals%28System.Object%29)

#### `int GetHashCode()`

GetHashCode method of UShort4.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort4.GetHashCode)

#### `static Fusion.UShort4 Max(Fusion.UShort4 a, Fusion.UShort4 b)`

Max method of UShort4.

**Parameters:**
- `a` (Fusion.UShort4)
- `b` (Fusion.UShort4)

**Returns:** `Fusion.UShort4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort4.Max%28Fusion.UShort4%2CFusion.UShort4%29)

#### `static Fusion.UShort4 Min(Fusion.UShort4 a, Fusion.UShort4 b)`

Min method of UShort4.

**Parameters:**
- `a` (Fusion.UShort4)
- `b` (Fusion.UShort4)

**Returns:** `Fusion.UShort4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort4.Min%28Fusion.UShort4%2CFusion.UShort4%29)

#### `string ToString()`

ToString method of UShort4.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort4.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of UShort4.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort4.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.UShort4 op_Addition(Fusion.UShort4 a, Fusion.UShort4 b)`

op_Addition method of UShort4.

**Parameters:**
- `a` (Fusion.UShort4)
- `b` (Fusion.UShort4)

**Returns:** `Fusion.UShort4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort4.op_Addition%28Fusion.UShort4%2CFusion.UShort4%29)

#### `static Fusion.UShort4 op_Addition(Fusion.UShort4 a, ushort b)`

op_Addition method of UShort4.

**Parameters:**
- `a` (Fusion.UShort4)
- `b` (ushort)

**Returns:** `Fusion.UShort4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort4.op_Addition%28Fusion.UShort4%2CSystem.UInt16%29)

#### `static Fusion.UShort4 op_Addition(ushort a, Fusion.UShort4 b)`

op_Addition method of UShort4.

**Parameters:**
- `a` (ushort)
- `b` (Fusion.UShort4)

**Returns:** `Fusion.UShort4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort4.op_Addition%28System.UInt16%2CFusion.UShort4%29)

#### `static Fusion.UShort4 op_Division(Fusion.UShort4 a, Fusion.UShort4 b)`

op_Division method of UShort4.

**Parameters:**
- `a` (Fusion.UShort4)
- `b` (Fusion.UShort4)

**Returns:** `Fusion.UShort4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort4.op_Division%28Fusion.UShort4%2CFusion.UShort4%29)

#### `static Fusion.UShort4 op_Division(Fusion.UShort4 a, ushort b)`

op_Division method of UShort4.

**Parameters:**
- `a` (Fusion.UShort4)
- `b` (ushort)

**Returns:** `Fusion.UShort4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort4.op_Division%28Fusion.UShort4%2CSystem.UInt16%29)

#### `static Fusion.UShort4 op_Division(ushort a, Fusion.UShort4 b)`

op_Division method of UShort4.

**Parameters:**
- `a` (ushort)
- `b` (Fusion.UShort4)

**Returns:** `Fusion.UShort4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort4.op_Division%28System.UInt16%2CFusion.UShort4%29)

#### `static bool op_Equality(Fusion.UShort4 a, Fusion.UShort4 b)`

op_Equality method of UShort4.

**Parameters:**
- `a` (Fusion.UShort4)
- `b` (Fusion.UShort4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort4.op_Equality%28Fusion.UShort4%2CFusion.UShort4%29)

#### `static Fusion.UShort4 op_Explicit(Fusion.Double4 v)`

op_Explicit method of UShort4.

**Parameters:**
- `v` (Fusion.Double4)

**Returns:** `Fusion.UShort4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort4.op_Explicit%28Fusion.Double4%29~Fusion.UShort4)

#### `static Fusion.UShort4 op_Explicit(Fusion.Float4 v)`

op_Explicit method of UShort4.

**Parameters:**
- `v` (Fusion.Float4)

**Returns:** `Fusion.UShort4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort4.op_Explicit%28Fusion.Float4%29~Fusion.UShort4)

#### `static Fusion.UShort4 op_Explicit(Fusion.Half4 v)`

op_Explicit method of UShort4.

**Parameters:**
- `v` (Fusion.Half4)

**Returns:** `Fusion.UShort4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort4.op_Explicit%28Fusion.Half4%29~Fusion.UShort4)

#### `static Fusion.UShort4 op_Explicit(Fusion.Int4 v)`

op_Explicit method of UShort4.

**Parameters:**
- `v` (Fusion.Int4)

**Returns:** `Fusion.UShort4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort4.op_Explicit%28Fusion.Int4%29~Fusion.UShort4)

#### `static Fusion.UShort4 op_Explicit(Fusion.Short4 v)`

op_Explicit method of UShort4.

**Parameters:**
- `v` (Fusion.Short4)

**Returns:** `Fusion.UShort4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort4.op_Explicit%28Fusion.Short4%29~Fusion.UShort4)

#### `static Fusion.UShort4 op_Implicit(Fusion.RGBA v)`

op_Implicit method of UShort4.

**Parameters:**
- `v` (Fusion.RGBA)

**Returns:** `Fusion.UShort4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort4.op_Implicit%28Fusion.RGBA%29~Fusion.UShort4)

#### `static bool op_Inequality(Fusion.UShort4 a, Fusion.UShort4 b)`

op_Inequality method of UShort4.

**Parameters:**
- `a` (Fusion.UShort4)
- `b` (Fusion.UShort4)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort4.op_Inequality%28Fusion.UShort4%2CFusion.UShort4%29)

#### `static Fusion.UShort4 op_Modulus(Fusion.UShort4 a, Fusion.UShort4 b)`

op_Modulus method of UShort4.

**Parameters:**
- `a` (Fusion.UShort4)
- `b` (Fusion.UShort4)

**Returns:** `Fusion.UShort4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort4.op_Modulus%28Fusion.UShort4%2CFusion.UShort4%29)

#### `static Fusion.UShort4 op_Modulus(Fusion.UShort4 a, ushort b)`

op_Modulus method of UShort4.

**Parameters:**
- `a` (Fusion.UShort4)
- `b` (ushort)

**Returns:** `Fusion.UShort4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort4.op_Modulus%28Fusion.UShort4%2CSystem.UInt16%29)

#### `static Fusion.UShort4 op_Modulus(ushort a, Fusion.UShort4 b)`

op_Modulus method of UShort4.

**Parameters:**
- `a` (ushort)
- `b` (Fusion.UShort4)

**Returns:** `Fusion.UShort4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort4.op_Modulus%28System.UInt16%2CFusion.UShort4%29)

#### `static Fusion.UShort4 op_Multiply(Fusion.UShort4 a, Fusion.UShort4 b)`

op_Multiply method of UShort4.

**Parameters:**
- `a` (Fusion.UShort4)
- `b` (Fusion.UShort4)

**Returns:** `Fusion.UShort4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort4.op_Multiply%28Fusion.UShort4%2CFusion.UShort4%29)

#### `static Fusion.UShort4 op_Multiply(Fusion.UShort4 a, ushort b)`

op_Multiply method of UShort4.

**Parameters:**
- `a` (Fusion.UShort4)
- `b` (ushort)

**Returns:** `Fusion.UShort4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort4.op_Multiply%28Fusion.UShort4%2CSystem.UInt16%29)

#### `static Fusion.UShort4 op_Multiply(ushort a, Fusion.UShort4 b)`

op_Multiply method of UShort4.

**Parameters:**
- `a` (ushort)
- `b` (Fusion.UShort4)

**Returns:** `Fusion.UShort4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort4.op_Multiply%28System.UInt16%2CFusion.UShort4%29)

#### `static Fusion.UShort4 op_Subtraction(Fusion.UShort4 a, Fusion.UShort4 b)`

op_Subtraction method of UShort4.

**Parameters:**
- `a` (Fusion.UShort4)
- `b` (Fusion.UShort4)

**Returns:** `Fusion.UShort4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort4.op_Subtraction%28Fusion.UShort4%2CFusion.UShort4%29)

#### `static Fusion.UShort4 op_Subtraction(Fusion.UShort4 a, ushort b)`

op_Subtraction method of UShort4.

**Parameters:**
- `a` (Fusion.UShort4)
- `b` (ushort)

**Returns:** `Fusion.UShort4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort4.op_Subtraction%28Fusion.UShort4%2CSystem.UInt16%29)

#### `static Fusion.UShort4 op_Subtraction(ushort a, Fusion.UShort4 b)`

op_Subtraction method of UShort4.

**Parameters:**
- `a` (ushort)
- `b` (Fusion.UShort4)

**Returns:** `Fusion.UShort4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort4.op_Subtraction%28System.UInt16%2CFusion.UShort4%29)

#### `static Fusion.UShort4 op_UnaryNegation(Fusion.UShort4 v)`

op_UnaryNegation method of UShort4.

**Parameters:**
- `v` (Fusion.UShort4)

**Returns:** `Fusion.UShort4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort4.op_UnaryNegation%28Fusion.UShort4%29)

#### `static Fusion.UShort4 op_UnaryPlus(Fusion.UShort4 v)`

op_UnaryPlus method of UShort4.

**Parameters:**
- `v` (Fusion.UShort4)

**Returns:** `Fusion.UShort4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.UShort4.op_UnaryPlus%28Fusion.UShort4%29)

### Properties
- `Item` (ushort, get/set) — Item property of UShort4.
- `XY` (Fusion.UShort2, get) — XY property of UShort4.
- `XYZ` (Fusion.UShort3, get) — XYZ property of UShort4.

## Vector (static-class)

Vector static class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Vector)

### Methods
#### `static Fusion.Float3 Cross(Fusion.Float3 a, Fusion.Float3 b)`

Cross method of Vector.

**Parameters:**
- `a` (Fusion.Float3)
- `b` (Fusion.Float3)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Vector.Cross%28Fusion.Float3%2CFusion.Float3%29)

#### `static float Dot(Fusion.Float2 a, Fusion.Float2 b)`

Dot method of Vector.

**Parameters:**
- `a` (Fusion.Float2)
- `b` (Fusion.Float2)

**Returns:** `float` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Vector.Dot%28Fusion.Float2%2CFusion.Float2%29)

#### `static float Dot(Fusion.Float3 a, Fusion.Float3 b)`

Dot method of Vector.

**Parameters:**
- `a` (Fusion.Float3)
- `b` (Fusion.Float3)

**Returns:** `float` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Vector.Dot%28Fusion.Float3%2CFusion.Float3%29)

#### `static float Dot(Fusion.Float4 a, Fusion.Float4 b)`

Dot method of Vector.

**Parameters:**
- `a` (Fusion.Float4)
- `b` (Fusion.Float4)

**Returns:** `float` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Vector.Dot%28Fusion.Float4%2CFusion.Float4%29)

#### `static float Magnitude(Fusion.Float2 vector)`

Magnitude method of Vector.

**Parameters:**
- `vector` (Fusion.Float2)

**Returns:** `float` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Vector.Magnitude%28Fusion.Float2%29)

#### `static float Magnitude(Fusion.Float3 vector)`

Magnitude method of Vector.

**Parameters:**
- `vector` (Fusion.Float3)

**Returns:** `float` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Vector.Magnitude%28Fusion.Float3%29)

#### `static float Magnitude(Fusion.Float4 vector)`

Magnitude method of Vector.

**Parameters:**
- `vector` (Fusion.Float4)

**Returns:** `float` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Vector.Magnitude%28Fusion.Float4%29)

#### `static float MagnitudeSquared(Fusion.Float2 vector)`

MagnitudeSquared method of Vector.

**Parameters:**
- `vector` (Fusion.Float2)

**Returns:** `float` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Vector.MagnitudeSquared%28Fusion.Float2%29)

#### `static float MagnitudeSquared(Fusion.Float3 vector)`

MagnitudeSquared method of Vector.

**Parameters:**
- `vector` (Fusion.Float3)

**Returns:** `float` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Vector.MagnitudeSquared%28Fusion.Float3%29)

#### `static float MagnitudeSquared(Fusion.Float4 vector)`

MagnitudeSquared method of Vector.

**Parameters:**
- `vector` (Fusion.Float4)

**Returns:** `float` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Vector.MagnitudeSquared%28Fusion.Float4%29)

#### `static Fusion.Float2 Normalize(Fusion.Float2 vector)`

Normalize method of Vector.

**Parameters:**
- `vector` (Fusion.Float2)

**Returns:** `Fusion.Float2` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Vector.Normalize%28Fusion.Float2%29)

#### `static Fusion.Float3 Normalize(Fusion.Float3 vector)`

Normalize method of Vector.

**Parameters:**
- `vector` (Fusion.Float3)

**Returns:** `Fusion.Float3` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Vector.Normalize%28Fusion.Float3%29)

#### `static Fusion.Float4 Normalize(Fusion.Float4 vector)`

Normalize method of Vector.

**Parameters:**
- `vector` (Fusion.Float4)

**Returns:** `Fusion.Float4` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Vector.Normalize%28Fusion.Float4%29)

## Velocity (struct)

Velocity struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Velocity)

### Constructors
- `Velocity(double metersPerSecond)` — Constructs a new Velocity.

### Methods
#### `static Fusion.Velocity Abs(Fusion.Velocity velocity)`

Abs method of Velocity.

**Parameters:**
- `velocity` (Fusion.Velocity)

**Returns:** `Fusion.Velocity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.Abs%28Fusion.Velocity%29)

#### `int CompareTo(Fusion.Velocity other)`

CompareTo method of Velocity.

**Parameters:**
- `other` (Fusion.Velocity)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.CompareTo%28Fusion.Velocity%29)

#### `int CompareTo(object other)`

CompareTo method of Velocity.

**Parameters:**
- `other` (object)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.CompareTo%28System.Object%29)

#### `bool Equals(Fusion.Velocity other)`

Equals method of Velocity.

**Parameters:**
- `other` (Fusion.Velocity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.Equals%28Fusion.Velocity%29)

#### `bool Equals(object other)`

Equals method of Velocity.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.Equals%28System.Object%29)

#### `static Fusion.Velocity From(double velocity, Fusion.VelocityUnit velocityUnit)`

From method of Velocity.

**Parameters:**
- `velocity` (double)
- `velocityUnit` (Fusion.VelocityUnit)

**Returns:** `Fusion.Velocity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.From%28System.Double%2CFusion.VelocityUnit%29)

#### `int GetHashCode()`

GetHashCode method of Velocity.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.GetHashCode)

#### `static bool IsInfinity(Fusion.Velocity velocity)`

IsInfinity method of Velocity.

**Parameters:**
- `velocity` (Fusion.Velocity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.IsInfinity%28Fusion.Velocity%29)

#### `static bool IsNaN(Fusion.Velocity velocity)`

IsNaN method of Velocity.

**Parameters:**
- `velocity` (Fusion.Velocity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.IsNaN%28Fusion.Velocity%29)

#### `static bool IsNegativeInfinity(Fusion.Velocity velocity)`

IsNegativeInfinity method of Velocity.

**Parameters:**
- `velocity` (Fusion.Velocity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.IsNegativeInfinity%28Fusion.Velocity%29)

#### `static bool IsPositiveInfinity(Fusion.Velocity velocity)`

IsPositiveInfinity method of Velocity.

**Parameters:**
- `velocity` (Fusion.Velocity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.IsPositiveInfinity%28Fusion.Velocity%29)

#### `static Fusion.Velocity Max(Fusion.Velocity a, Fusion.Velocity b)`

Max method of Velocity.

**Parameters:**
- `a` (Fusion.Velocity)
- `b` (Fusion.Velocity)

**Returns:** `Fusion.Velocity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.Max%28Fusion.Velocity%2CFusion.Velocity%29)

#### `static Fusion.Velocity Min(Fusion.Velocity a, Fusion.Velocity b)`

Min method of Velocity.

**Parameters:**
- `a` (Fusion.Velocity)
- `b` (Fusion.Velocity)

**Returns:** `Fusion.Velocity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.Min%28Fusion.Velocity%2CFusion.Velocity%29)

#### `bool Near(Fusion.Velocity other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of Velocity.

**Parameters:**
- `other` (Fusion.Velocity)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.Near%28Fusion.Velocity%2CSystem.Int32%29)

#### `static Fusion.Velocity Round(Fusion.Velocity velocity, Fusion.Velocity precision, Fusion.RoundingMode roundingMode)`

Round method of Velocity.

**Parameters:**
- `velocity` (Fusion.Velocity)
- `precision` (Fusion.Velocity)
- `roundingMode` (Fusion.RoundingMode)

**Returns:** `Fusion.Velocity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.Round%28Fusion.Velocity%2CFusion.Velocity%2CFusion.RoundingMode%29)

#### `static int Sign(Fusion.Velocity velocity)`

Sign method of Velocity.

**Parameters:**
- `velocity` (Fusion.Velocity)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.Sign%28Fusion.Velocity%29)

#### `double To(Fusion.VelocityUnit velocityUnit)`

To method of Velocity.

**Parameters:**
- `velocityUnit` (Fusion.VelocityUnit)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.To%28Fusion.VelocityUnit%29)

#### `string ToString()`

ToString method of Velocity.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of Velocity.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.Velocity op_Addition(Fusion.Velocity a, Fusion.Velocity b)`

op_Addition method of Velocity.

**Parameters:**
- `a` (Fusion.Velocity)
- `b` (Fusion.Velocity)

**Returns:** `Fusion.Velocity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.op_Addition%28Fusion.Velocity%2CFusion.Velocity%29)

#### `static Fusion.Velocity op_Division(Fusion.Velocity a, double b)`

op_Division method of Velocity.

**Parameters:**
- `a` (Fusion.Velocity)
- `b` (double)

**Returns:** `Fusion.Velocity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.op_Division%28Fusion.Velocity%2CSystem.Double%29)

#### `static double op_Division(Fusion.Velocity a, Fusion.Velocity b)`

op_Division method of Velocity.

**Parameters:**
- `a` (Fusion.Velocity)
- `b` (Fusion.Velocity)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.op_Division%28Fusion.Velocity%2CFusion.Velocity%29)

#### `static bool op_Equality(Fusion.Velocity a, Fusion.Velocity b)`

op_Equality method of Velocity.

**Parameters:**
- `a` (Fusion.Velocity)
- `b` (Fusion.Velocity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.op_Equality%28Fusion.Velocity%2CFusion.Velocity%29)

#### `static bool op_GreaterThan(Fusion.Velocity a, Fusion.Velocity b)`

op_GreaterThan method of Velocity.

**Parameters:**
- `a` (Fusion.Velocity)
- `b` (Fusion.Velocity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.op_GreaterThan%28Fusion.Velocity%2CFusion.Velocity%29)

#### `static bool op_GreaterThanOrEqual(Fusion.Velocity a, Fusion.Velocity b)`

op_GreaterThanOrEqual method of Velocity.

**Parameters:**
- `a` (Fusion.Velocity)
- `b` (Fusion.Velocity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.op_GreaterThanOrEqual%28Fusion.Velocity%2CFusion.Velocity%29)

#### `static bool op_Inequality(Fusion.Velocity a, Fusion.Velocity b)`

op_Inequality method of Velocity.

**Parameters:**
- `a` (Fusion.Velocity)
- `b` (Fusion.Velocity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.op_Inequality%28Fusion.Velocity%2CFusion.Velocity%29)

#### `static bool op_LessThan(Fusion.Velocity a, Fusion.Velocity b)`

op_LessThan method of Velocity.

**Parameters:**
- `a` (Fusion.Velocity)
- `b` (Fusion.Velocity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.op_LessThan%28Fusion.Velocity%2CFusion.Velocity%29)

#### `static bool op_LessThanOrEqual(Fusion.Velocity a, Fusion.Velocity b)`

op_LessThanOrEqual method of Velocity.

**Parameters:**
- `a` (Fusion.Velocity)
- `b` (Fusion.Velocity)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.op_LessThanOrEqual%28Fusion.Velocity%2CFusion.Velocity%29)

#### `static Fusion.Velocity op_Modulus(Fusion.Velocity a, Fusion.Velocity b)`

op_Modulus method of Velocity.

**Parameters:**
- `a` (Fusion.Velocity)
- `b` (Fusion.Velocity)

**Returns:** `Fusion.Velocity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.op_Modulus%28Fusion.Velocity%2CFusion.Velocity%29)

#### `static Fusion.Velocity op_Multiply(Fusion.Velocity a, double b)`

op_Multiply method of Velocity.

**Parameters:**
- `a` (Fusion.Velocity)
- `b` (double)

**Returns:** `Fusion.Velocity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.op_Multiply%28Fusion.Velocity%2CSystem.Double%29)

#### `static Fusion.Velocity op_Multiply(double a, Fusion.Velocity b)`

op_Multiply method of Velocity.

**Parameters:**
- `a` (double)
- `b` (Fusion.Velocity)

**Returns:** `Fusion.Velocity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.op_Multiply%28System.Double%2CFusion.Velocity%29)

#### `static Fusion.Velocity op_Subtraction(Fusion.Velocity a, Fusion.Velocity b)`

op_Subtraction method of Velocity.

**Parameters:**
- `a` (Fusion.Velocity)
- `b` (Fusion.Velocity)

**Returns:** `Fusion.Velocity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.op_Subtraction%28Fusion.Velocity%2CFusion.Velocity%29)

#### `static Fusion.Velocity op_UnaryNegation(Fusion.Velocity a)`

op_UnaryNegation method of Velocity.

**Parameters:**
- `a` (Fusion.Velocity)

**Returns:** `Fusion.Velocity` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Velocity.op_UnaryNegation%28Fusion.Velocity%29)

### Properties
- `MetersPerSecond` (double, get) — MetersPerSecond property of Velocity.

## VelocityUnit (enum)

VelocityUnit enum in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.VelocityUnit)

### Values
- `MetersPerSecond` = `0`
- `FeetPerSecond` = `1`

## ViewModel (class)

ViewModel class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.ViewModel)

### Properties
- `Host` (Fusion.IHost, get/set) — Host property of ViewModel.

## Volume (struct)

Volume struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.Volume)

### Constructors
- `Volume(double cubicMillimeters)` — Constructs a new Volume.

### Methods
#### `static Fusion.Volume Abs(Fusion.Volume volume)`

Abs method of Volume.

**Parameters:**
- `volume` (Fusion.Volume)

**Returns:** `Fusion.Volume` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.Abs%28Fusion.Volume%29)

#### `int CompareTo(Fusion.Volume other)`

CompareTo method of Volume.

**Parameters:**
- `other` (Fusion.Volume)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.CompareTo%28Fusion.Volume%29)

#### `int CompareTo(object other)`

CompareTo method of Volume.

**Parameters:**
- `other` (object)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.CompareTo%28System.Object%29)

#### `bool Equals(Fusion.Volume other)`

Equals method of Volume.

**Parameters:**
- `other` (Fusion.Volume)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.Equals%28Fusion.Volume%29)

#### `bool Equals(object other)`

Equals method of Volume.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.Equals%28System.Object%29)

#### `static Fusion.Volume From(double volume, Fusion.VolumeUnit volumeUnit)`

From method of Volume.

**Parameters:**
- `volume` (double)
- `volumeUnit` (Fusion.VolumeUnit)

**Returns:** `Fusion.Volume` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.From%28System.Double%2CFusion.VolumeUnit%29)

#### `int GetHashCode()`

GetHashCode method of Volume.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.GetHashCode)

#### `static bool IsInfinity(Fusion.Volume volume)`

IsInfinity method of Volume.

**Parameters:**
- `volume` (Fusion.Volume)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.IsInfinity%28Fusion.Volume%29)

#### `static bool IsNaN(Fusion.Volume volume)`

IsNaN method of Volume.

**Parameters:**
- `volume` (Fusion.Volume)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.IsNaN%28Fusion.Volume%29)

#### `static bool IsNegativeInfinity(Fusion.Volume volume)`

IsNegativeInfinity method of Volume.

**Parameters:**
- `volume` (Fusion.Volume)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.IsNegativeInfinity%28Fusion.Volume%29)

#### `static bool IsPositiveInfinity(Fusion.Volume volume)`

IsPositiveInfinity method of Volume.

**Parameters:**
- `volume` (Fusion.Volume)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.IsPositiveInfinity%28Fusion.Volume%29)

#### `static Fusion.Volume Max(Fusion.Volume a, Fusion.Volume b)`

Max method of Volume.

**Parameters:**
- `a` (Fusion.Volume)
- `b` (Fusion.Volume)

**Returns:** `Fusion.Volume` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.Max%28Fusion.Volume%2CFusion.Volume%29)

#### `static Fusion.Volume Min(Fusion.Volume a, Fusion.Volume b)`

Min method of Volume.

**Parameters:**
- `a` (Fusion.Volume)
- `b` (Fusion.Volume)

**Returns:** `Fusion.Volume` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.Min%28Fusion.Volume%2CFusion.Volume%29)

#### `bool Near(Fusion.Volume other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of Volume.

**Parameters:**
- `other` (Fusion.Volume)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.Near%28Fusion.Volume%2CSystem.Int32%29)

#### `static Fusion.Volume Round(Fusion.Volume volume, Fusion.Volume precision, Fusion.RoundingMode roundingMode)`

Round method of Volume.

**Parameters:**
- `volume` (Fusion.Volume)
- `precision` (Fusion.Volume)
- `roundingMode` (Fusion.RoundingMode)

**Returns:** `Fusion.Volume` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.Round%28Fusion.Volume%2CFusion.Volume%2CFusion.RoundingMode%29)

#### `static int Sign(Fusion.Volume volume)`

Sign method of Volume.

**Parameters:**
- `volume` (Fusion.Volume)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.Sign%28Fusion.Volume%29)

#### `double To(Fusion.VolumeUnit volumeUnit)`

To method of Volume.

**Parameters:**
- `volumeUnit` (Fusion.VolumeUnit)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.To%28Fusion.VolumeUnit%29)

#### `string ToString()`

ToString method of Volume.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of Volume.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.Volume op_Addition(Fusion.Volume a, Fusion.Volume b)`

op_Addition method of Volume.

**Parameters:**
- `a` (Fusion.Volume)
- `b` (Fusion.Volume)

**Returns:** `Fusion.Volume` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.op_Addition%28Fusion.Volume%2CFusion.Volume%29)

#### `static Fusion.Volume op_Division(Fusion.Volume a, double b)`

op_Division method of Volume.

**Parameters:**
- `a` (Fusion.Volume)
- `b` (double)

**Returns:** `Fusion.Volume` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.op_Division%28Fusion.Volume%2CSystem.Double%29)

#### `static double op_Division(Fusion.Volume a, Fusion.Volume b)`

op_Division method of Volume.

**Parameters:**
- `a` (Fusion.Volume)
- `b` (Fusion.Volume)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.op_Division%28Fusion.Volume%2CFusion.Volume%29)

#### `static bool op_Equality(Fusion.Volume a, Fusion.Volume b)`

op_Equality method of Volume.

**Parameters:**
- `a` (Fusion.Volume)
- `b` (Fusion.Volume)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.op_Equality%28Fusion.Volume%2CFusion.Volume%29)

#### `static bool op_GreaterThan(Fusion.Volume a, Fusion.Volume b)`

op_GreaterThan method of Volume.

**Parameters:**
- `a` (Fusion.Volume)
- `b` (Fusion.Volume)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.op_GreaterThan%28Fusion.Volume%2CFusion.Volume%29)

#### `static bool op_GreaterThanOrEqual(Fusion.Volume a, Fusion.Volume b)`

op_GreaterThanOrEqual method of Volume.

**Parameters:**
- `a` (Fusion.Volume)
- `b` (Fusion.Volume)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.op_GreaterThanOrEqual%28Fusion.Volume%2CFusion.Volume%29)

#### `static bool op_Inequality(Fusion.Volume a, Fusion.Volume b)`

op_Inequality method of Volume.

**Parameters:**
- `a` (Fusion.Volume)
- `b` (Fusion.Volume)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.op_Inequality%28Fusion.Volume%2CFusion.Volume%29)

#### `static bool op_LessThan(Fusion.Volume a, Fusion.Volume b)`

op_LessThan method of Volume.

**Parameters:**
- `a` (Fusion.Volume)
- `b` (Fusion.Volume)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.op_LessThan%28Fusion.Volume%2CFusion.Volume%29)

#### `static bool op_LessThanOrEqual(Fusion.Volume a, Fusion.Volume b)`

op_LessThanOrEqual method of Volume.

**Parameters:**
- `a` (Fusion.Volume)
- `b` (Fusion.Volume)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.op_LessThanOrEqual%28Fusion.Volume%2CFusion.Volume%29)

#### `static Fusion.Volume op_Modulus(Fusion.Volume a, Fusion.Volume b)`

op_Modulus method of Volume.

**Parameters:**
- `a` (Fusion.Volume)
- `b` (Fusion.Volume)

**Returns:** `Fusion.Volume` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.op_Modulus%28Fusion.Volume%2CFusion.Volume%29)

#### `static Fusion.Volume op_Multiply(Fusion.Volume a, double b)`

op_Multiply method of Volume.

**Parameters:**
- `a` (Fusion.Volume)
- `b` (double)

**Returns:** `Fusion.Volume` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.op_Multiply%28Fusion.Volume%2CSystem.Double%29)

#### `static Fusion.Volume op_Multiply(double a, Fusion.Volume b)`

op_Multiply method of Volume.

**Parameters:**
- `a` (double)
- `b` (Fusion.Volume)

**Returns:** `Fusion.Volume` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.op_Multiply%28System.Double%2CFusion.Volume%29)

#### `static Fusion.Volume op_Subtraction(Fusion.Volume a, Fusion.Volume b)`

op_Subtraction method of Volume.

**Parameters:**
- `a` (Fusion.Volume)
- `b` (Fusion.Volume)

**Returns:** `Fusion.Volume` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.op_Subtraction%28Fusion.Volume%2CFusion.Volume%29)

#### `static Fusion.Volume op_UnaryNegation(Fusion.Volume a)`

op_UnaryNegation method of Volume.

**Parameters:**
- `a` (Fusion.Volume)

**Returns:** `Fusion.Volume` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.Volume.op_UnaryNegation%28Fusion.Volume%29)

### Properties
- `CubicMillimeters` (double, get) — CubicMillimeters property of Volume.

## VolumeFlow (struct)

VolumeFlow struct in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.VolumeFlow)

### Constructors
- `VolumeFlow(double cubicMetersPerSecond)` — Constructs a new VolumeFlow.

### Methods
#### `static Fusion.VolumeFlow Abs(Fusion.VolumeFlow volumeFlow)`

Abs method of VolumeFlow.

**Parameters:**
- `volumeFlow` (Fusion.VolumeFlow)

**Returns:** `Fusion.VolumeFlow` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.Abs%28Fusion.VolumeFlow%29)

#### `int CompareTo(Fusion.VolumeFlow other)`

CompareTo method of VolumeFlow.

**Parameters:**
- `other` (Fusion.VolumeFlow)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.CompareTo%28Fusion.VolumeFlow%29)

#### `int CompareTo(object other)`

CompareTo method of VolumeFlow.

**Parameters:**
- `other` (object)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.CompareTo%28System.Object%29)

#### `bool Equals(Fusion.VolumeFlow other)`

Equals method of VolumeFlow.

**Parameters:**
- `other` (Fusion.VolumeFlow)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.Equals%28Fusion.VolumeFlow%29)

#### `bool Equals(object other)`

Equals method of VolumeFlow.

**Parameters:**
- `other` (object)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.Equals%28System.Object%29)

#### `static Fusion.VolumeFlow From(double volumeFlow, Fusion.VolumeFlowUnit volumeFlowUnit)`

From method of VolumeFlow.

**Parameters:**
- `volumeFlow` (double)
- `volumeFlowUnit` (Fusion.VolumeFlowUnit)

**Returns:** `Fusion.VolumeFlow` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.From%28System.Double%2CFusion.VolumeFlowUnit%29)

#### `int GetHashCode()`

GetHashCode method of VolumeFlow.

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.GetHashCode)

#### `static bool IsInfinity(Fusion.VolumeFlow volumeFlow)`

IsInfinity method of VolumeFlow.

**Parameters:**
- `volumeFlow` (Fusion.VolumeFlow)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.IsInfinity%28Fusion.VolumeFlow%29)

#### `static bool IsNaN(Fusion.VolumeFlow volumeFlow)`

IsNaN method of VolumeFlow.

**Parameters:**
- `volumeFlow` (Fusion.VolumeFlow)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.IsNaN%28Fusion.VolumeFlow%29)

#### `static bool IsNegativeInfinity(Fusion.VolumeFlow volumeFlow)`

IsNegativeInfinity method of VolumeFlow.

**Parameters:**
- `volumeFlow` (Fusion.VolumeFlow)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.IsNegativeInfinity%28Fusion.VolumeFlow%29)

#### `static bool IsPositiveInfinity(Fusion.VolumeFlow volumeFlow)`

IsPositiveInfinity method of VolumeFlow.

**Parameters:**
- `volumeFlow` (Fusion.VolumeFlow)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.IsPositiveInfinity%28Fusion.VolumeFlow%29)

#### `static Fusion.VolumeFlow Max(Fusion.VolumeFlow a, Fusion.VolumeFlow b)`

Max method of VolumeFlow.

**Parameters:**
- `a` (Fusion.VolumeFlow)
- `b` (Fusion.VolumeFlow)

**Returns:** `Fusion.VolumeFlow` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.Max%28Fusion.VolumeFlow%2CFusion.VolumeFlow%29)

#### `static Fusion.VolumeFlow Min(Fusion.VolumeFlow a, Fusion.VolumeFlow b)`

Min method of VolumeFlow.

**Parameters:**
- `a` (Fusion.VolumeFlow)
- `b` (Fusion.VolumeFlow)

**Returns:** `Fusion.VolumeFlow` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.Min%28Fusion.VolumeFlow%2CFusion.VolumeFlow%29)

#### `bool Near(Fusion.VolumeFlow other, int allowedErrorInUnitsInLastPlace = 10)`

Near method of VolumeFlow.

**Parameters:**
- `other` (Fusion.VolumeFlow)
- `allowedErrorInUnitsInLastPlace` (int)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.Near%28Fusion.VolumeFlow%2CSystem.Int32%29)

#### `static Fusion.VolumeFlow Round(Fusion.VolumeFlow volumeFlow, Fusion.VolumeFlow precision, Fusion.RoundingMode roundingMode)`

Round method of VolumeFlow.

**Parameters:**
- `volumeFlow` (Fusion.VolumeFlow)
- `precision` (Fusion.VolumeFlow)
- `roundingMode` (Fusion.RoundingMode)

**Returns:** `Fusion.VolumeFlow` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.Round%28Fusion.VolumeFlow%2CFusion.VolumeFlow%2CFusion.RoundingMode%29)

#### `static int Sign(Fusion.VolumeFlow volumeFlow)`

Sign method of VolumeFlow.

**Parameters:**
- `volumeFlow` (Fusion.VolumeFlow)

**Returns:** `int` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.Sign%28Fusion.VolumeFlow%29)

#### `double To(Fusion.VolumeFlowUnit volumeFlowUnit)`

To method of VolumeFlow.

**Parameters:**
- `volumeFlowUnit` (Fusion.VolumeFlowUnit)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.To%28Fusion.VolumeFlowUnit%29)

#### `string ToString()`

ToString method of VolumeFlow.

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.ToString)

#### `string ToString(string format, System.IFormatProvider formatProvider)`

ToString method of VolumeFlow.

**Parameters:**
- `format` (string)
- `formatProvider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.ToString%28System.String%2CSystem.IFormatProvider%29)

#### `static Fusion.VolumeFlow op_Addition(Fusion.VolumeFlow a, Fusion.VolumeFlow b)`

op_Addition method of VolumeFlow.

**Parameters:**
- `a` (Fusion.VolumeFlow)
- `b` (Fusion.VolumeFlow)

**Returns:** `Fusion.VolumeFlow` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.op_Addition%28Fusion.VolumeFlow%2CFusion.VolumeFlow%29)

#### `static Fusion.VolumeFlow op_Division(Fusion.VolumeFlow a, double b)`

op_Division method of VolumeFlow.

**Parameters:**
- `a` (Fusion.VolumeFlow)
- `b` (double)

**Returns:** `Fusion.VolumeFlow` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.op_Division%28Fusion.VolumeFlow%2CSystem.Double%29)

#### `static double op_Division(Fusion.VolumeFlow a, Fusion.VolumeFlow b)`

op_Division method of VolumeFlow.

**Parameters:**
- `a` (Fusion.VolumeFlow)
- `b` (Fusion.VolumeFlow)

**Returns:** `double` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.op_Division%28Fusion.VolumeFlow%2CFusion.VolumeFlow%29)

#### `static bool op_Equality(Fusion.VolumeFlow a, Fusion.VolumeFlow b)`

op_Equality method of VolumeFlow.

**Parameters:**
- `a` (Fusion.VolumeFlow)
- `b` (Fusion.VolumeFlow)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.op_Equality%28Fusion.VolumeFlow%2CFusion.VolumeFlow%29)

#### `static bool op_GreaterThan(Fusion.VolumeFlow a, Fusion.VolumeFlow b)`

op_GreaterThan method of VolumeFlow.

**Parameters:**
- `a` (Fusion.VolumeFlow)
- `b` (Fusion.VolumeFlow)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.op_GreaterThan%28Fusion.VolumeFlow%2CFusion.VolumeFlow%29)

#### `static bool op_GreaterThanOrEqual(Fusion.VolumeFlow a, Fusion.VolumeFlow b)`

op_GreaterThanOrEqual method of VolumeFlow.

**Parameters:**
- `a` (Fusion.VolumeFlow)
- `b` (Fusion.VolumeFlow)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.op_GreaterThanOrEqual%28Fusion.VolumeFlow%2CFusion.VolumeFlow%29)

#### `static bool op_Inequality(Fusion.VolumeFlow a, Fusion.VolumeFlow b)`

op_Inequality method of VolumeFlow.

**Parameters:**
- `a` (Fusion.VolumeFlow)
- `b` (Fusion.VolumeFlow)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.op_Inequality%28Fusion.VolumeFlow%2CFusion.VolumeFlow%29)

#### `static bool op_LessThan(Fusion.VolumeFlow a, Fusion.VolumeFlow b)`

op_LessThan method of VolumeFlow.

**Parameters:**
- `a` (Fusion.VolumeFlow)
- `b` (Fusion.VolumeFlow)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.op_LessThan%28Fusion.VolumeFlow%2CFusion.VolumeFlow%29)

#### `static bool op_LessThanOrEqual(Fusion.VolumeFlow a, Fusion.VolumeFlow b)`

op_LessThanOrEqual method of VolumeFlow.

**Parameters:**
- `a` (Fusion.VolumeFlow)
- `b` (Fusion.VolumeFlow)

**Returns:** `bool` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.op_LessThanOrEqual%28Fusion.VolumeFlow%2CFusion.VolumeFlow%29)

#### `static Fusion.VolumeFlow op_Modulus(Fusion.VolumeFlow a, Fusion.VolumeFlow b)`

op_Modulus method of VolumeFlow.

**Parameters:**
- `a` (Fusion.VolumeFlow)
- `b` (Fusion.VolumeFlow)

**Returns:** `Fusion.VolumeFlow` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.op_Modulus%28Fusion.VolumeFlow%2CFusion.VolumeFlow%29)

#### `static Fusion.VolumeFlow op_Multiply(Fusion.VolumeFlow a, double b)`

op_Multiply method of VolumeFlow.

**Parameters:**
- `a` (Fusion.VolumeFlow)
- `b` (double)

**Returns:** `Fusion.VolumeFlow` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.op_Multiply%28Fusion.VolumeFlow%2CSystem.Double%29)

#### `static Fusion.VolumeFlow op_Multiply(double a, Fusion.VolumeFlow b)`

op_Multiply method of VolumeFlow.

**Parameters:**
- `a` (double)
- `b` (Fusion.VolumeFlow)

**Returns:** `Fusion.VolumeFlow` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.op_Multiply%28System.Double%2CFusion.VolumeFlow%29)

#### `static Fusion.VolumeFlow op_Subtraction(Fusion.VolumeFlow a, Fusion.VolumeFlow b)`

op_Subtraction method of VolumeFlow.

**Parameters:**
- `a` (Fusion.VolumeFlow)
- `b` (Fusion.VolumeFlow)

**Returns:** `Fusion.VolumeFlow` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.op_Subtraction%28Fusion.VolumeFlow%2CFusion.VolumeFlow%29)

#### `static Fusion.VolumeFlow op_UnaryNegation(Fusion.VolumeFlow a)`

op_UnaryNegation method of VolumeFlow.

**Parameters:**
- `a` (Fusion.VolumeFlow)

**Returns:** `Fusion.VolumeFlow` — 

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.VolumeFlow.op_UnaryNegation%28Fusion.VolumeFlow%29)

### Properties
- `CubicMetersPerSecond` (double, get) — CubicMetersPerSecond property of VolumeFlow.

## VolumeFlowUnit (enum)

VolumeFlowUnit enum in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.VolumeFlowUnit)

### Values
- `CubicMetersPerSecond` = `0`
- `CubicFeetPerSecond` = `1`
- `LitresPerSecond` = `2`

## VolumeUnit (enum)

VolumeUnit enum in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.VolumeUnit)

### Values
- `CubicMillimeter` = `0`
- `CubicCentimeter` = `1`
- `CubicMeter` = `2`
- `CubicInch` = `3`
- `CubicFoot` = `4`
- `CubicYard` = `5`
- `Liter` = `6`
- `Gallon` = `7`

## WarpingUnit (enum)

WarpingUnit enum in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.WarpingUnit)

### Values
- `Millimeter` = `0`
- `Centimeter` = `1`
- `Inch` = `2`

## WindowViewModel (class)

WindowViewModel class in Fusion.

[Vendor docs](https://www.nuget.org/packages/TeklaFusion#T:Fusion.WindowViewModel)

### Methods
#### `void WindowClosed()`

WindowClosed method of WindowViewModel.

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.WindowViewModel.WindowClosed)

#### `void WindowOpened()`

WindowOpened method of WindowViewModel.

[Docs](https://www.nuget.org/packages/TeklaFusion#M%3AFusion.WindowViewModel.WindowOpened)

### Properties
- `Window` (Fusion.IWindowController, get/set) — Window property of WindowViewModel.

