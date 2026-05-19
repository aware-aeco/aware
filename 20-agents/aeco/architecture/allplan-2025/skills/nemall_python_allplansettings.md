---
name: allplan-nemall_python_allplansettings
description: This skill encodes the allplan 2025.0 surface of the NemAll_Python_AllplanSettings namespace — 29 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Functions, AllplanPaths, AllplanVersion, AllplanGlobalSettings, AllplanLocalisationService, ImperialUnitService, FontProvider, AngleUnits, and 21 more types.
---

# NemAll_Python_AllplanSettings

Auto-generated from vendor docs for allplan 2025.0. 29 types in this namespace.

## AllplanGlobalSettings (class)

Utility for reading the current global Allplan settings

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanGlobalSettings/)

### Methods
#### `GetCurrentColorId() -> int`

Get the current used color ID in Allplan

**Remarks:** Get the current used color ID in Allplan

**Returns:** `int` — Current color ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanGlobalSettings/#NemAll_Python_AllplanSettings.AllplanGlobalSettings.GetCurrentColorId)

#### `GetCurrentCommonProperties() -> CommonProperties`

Get the current common properties

**Remarks:** Get the current common properties

**Returns:** `CommonProperties` — Current common properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanGlobalSettings/#NemAll_Python_AllplanSettings.AllplanGlobalSettings.GetCurrentCommonProperties)

#### `GetCurrentFaceStyleId() -> int`

Get the current used face style ID in Allplan

**Remarks:** Get the current used face style ID in Allplan

**Returns:** `int` — Current face style ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanGlobalSettings/#NemAll_Python_AllplanSettings.AllplanGlobalSettings.GetCurrentFaceStyleId)

#### `GetCurrentFontId() -> int`

Get the current used font ID in Allplan

**Remarks:** Get the current used font ID in Allplan

**Returns:** `int` — Current font ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanGlobalSettings/#NemAll_Python_AllplanSettings.AllplanGlobalSettings.GetCurrentFontId)

#### `GetCurrentHatchId() -> int`

Get the current used hatch ID in Allplan

**Remarks:** Get the current used hatch ID in Allplan

**Returns:** `int` — Current hatch ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanGlobalSettings/#NemAll_Python_AllplanSettings.AllplanGlobalSettings.GetCurrentHatchId)

#### `GetCurrentLayerId() -> int`

Get the current used layer ID in Allplan

**Remarks:** Get the current used layer ID in Allplan

**Returns:** `int` — Current layer ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanGlobalSettings/#NemAll_Python_AllplanSettings.AllplanGlobalSettings.GetCurrentLayerId)

#### `GetCurrentPatternId() -> int`

Get the current used pattern ID in Allplan

**Remarks:** Get the current used pattern ID in Allplan

**Returns:** `int` — Current pattern ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanGlobalSettings/#NemAll_Python_AllplanSettings.AllplanGlobalSettings.GetCurrentPatternId)

#### `GetCurrentPenId() -> int`

Get the current used pen ID in Allplan

**Remarks:** Get the current used pen ID in Allplan

**Returns:** `int` — Current pen ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanGlobalSettings/#NemAll_Python_AllplanSettings.AllplanGlobalSettings.GetCurrentPenId)

#### `GetCurrentStrokeId() -> int`

Get the current used stroke ID in Allplan

**Remarks:** Get the current used stroke ID in Allplan

**Returns:** `int` — Current stroke ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanGlobalSettings/#NemAll_Python_AllplanSettings.AllplanGlobalSettings.GetCurrentStrokeId)

#### `GetOffsetPoint() -> Point3D`

Get the project offset of the currently active project

**Remarks:** Get the project offset of the currently active project

**Returns:** `Point3D` — Project offset as point

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanGlobalSettings/#NemAll_Python_AllplanSettings.AllplanGlobalSettings.GetOffsetPoint)

## AllplanLocalisationService (class)

Utility for reading the current localization settings in Allplan

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanLocalisationService/)

### Methods
#### `AllplanLanguage() -> str`

Get the installed Allplan language

**Remarks:** Get the installed Allplan language

**Returns:** `str` — Currently used language as a three letter description bul - Bulgarian deu - German eng - English fin - Finnish fra - French grc - Greek hol - Dutch hrv - Croatian ita - Italian pol - Polish rum - Romanian rus - Russian slk - Slovak spa - Spanish svn - Slovenian tch - Czech trk - Turkish ung - Hungarian dan - Danish ser - Serbian mak - Macedonian prt - Portuguese ltu - Lithuanian lva - Latvian est - Estonian ukr - Ukrainian swe - Swedish nor - Norwegian chn - Chinese kor - Korean jpn - Japanese usa - USA - English vie - Vietnamese

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanLocalisationService/#NemAll_Python_AllplanSettings.AllplanLocalisationService.AllplanLanguage)

#### `GetTextResource(textID: int) -> str`

Get a text resource

**Remarks:** Get a text resource

**Parameters:**
- `textID` (int) — Text ID

**Returns:** `str` — Text from the text ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanLocalisationService/#NemAll_Python_AllplanSettings.AllplanLocalisationService.GetTextResource)

#### `Language() -> str`

Get the current Allplan language

**Remarks:** Get the current Allplan language

**Returns:** `str` — Currently used language as a two letter description bg - Bulgarian de - German en - English fi - Finnish fr - French el - Greek nl - Dutch hr - Croatian it - Italian pl - Polish ro - Romanian ru - Russian sk - Slovak es - Spanish sl - Slovenian cs - Czech tr - Turkish hu - Hungarian da - Danish sr - Serbian mk - Macedonian pt - Portuguese lt - Lithuanian lv - Latvian et - Estonian uk - Ukrainian sv - Swedish no - Norwegian zh - Chinese ko - Korean ja - Japanese vi - Vietnamese

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanLocalisationService/#NemAll_Python_AllplanSettings.AllplanLocalisationService.Language)

## AllplanPaths (class)

Class for getting Allplan paths

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanPaths/)

### Methods
#### `GetCurPrjDesignPath() -> str`

Get the current project design path

**Remarks:** Get the current project design path

**Returns:** `str` — Current project design path

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanPaths/#NemAll_Python_AllplanSettings.AllplanPaths.GetCurPrjDesignPath)

#### `GetCurPrjPath() -> str`

Get the current project path

**Remarks:** Get the current project path

**Returns:** `str` — Current project path

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanPaths/#NemAll_Python_AllplanSettings.AllplanPaths.GetCurPrjPath)

#### `GetEtcPath() -> str`

Get the Etc path

**Remarks:** Get the Etc path

**Returns:** `str` — Etc path

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanPaths/#NemAll_Python_AllplanSettings.AllplanPaths.GetEtcPath)

#### `GetPathOfApplication() -> str`

Get the path of the application

**Remarks:** Get the path of the application

**Returns:** `str` — Path of the application

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanPaths/#NemAll_Python_AllplanSettings.AllplanPaths.GetPathOfApplication)

#### `GetPrgPath() -> str`

Get the program path

**Remarks:** Get the program path

**Returns:** `str` — Program path

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanPaths/#NemAll_Python_AllplanSettings.AllplanPaths.GetPrgPath)

#### `GetPythonPartsEtcPath() -> str`

Get the PythonParts Etc path

**Remarks:** Get the PythonParts Etc path

**Returns:** `str` — Etc path

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanPaths/#NemAll_Python_AllplanSettings.AllplanPaths.GetPythonPartsEtcPath)

#### `GetStdDesignPath() -> str`

Get the Std design path

**Remarks:** Get the Std design path

**Returns:** `str` — Std design path

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanPaths/#NemAll_Python_AllplanSettings.AllplanPaths.GetStdDesignPath)

#### `GetStdPath() -> str`

Get the Std path

**Remarks:** Get the Std path

**Returns:** `str` — Std path

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanPaths/#NemAll_Python_AllplanSettings.AllplanPaths.GetStdPath)

#### `GetTmpPath() -> str`

Get the Tmp path

**Remarks:** Get the Tmp path

**Returns:** `str` — Tmp path

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanPaths/#NemAll_Python_AllplanSettings.AllplanPaths.GetTmpPath)

#### `GetUsrPath() -> str`

Get the Usr path

**Remarks:** Get the Usr path

**Returns:** `str` — Usr path

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanPaths/#NemAll_Python_AllplanSettings.AllplanPaths.GetUsrPath)

## AllplanVersion (class)

Class for extraction of Allplan version information

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanVersion/)

### Methods
#### `MainReleaseName() -> str`

Get the Allplan main release name Returns: Allplan main release name ('2016', '2017', ...)

**Remarks:** Get the Allplan main release name Returns: Allplan main release name ('2016', '2017', ...)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanVersion/#NemAll_Python_AllplanSettings.AllplanVersion.MainReleaseName)

#### `SubReleaseName() -> str`

Get the Allplan sub release name Returns: Allplan sub release name ('2016.0', '2016.1', ...)

**Remarks:** Get the Allplan sub release name Returns: Allplan sub release name ('2016.0', '2016.1', ...)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanVersion/#NemAll_Python_AllplanSettings.AllplanVersion.SubReleaseName)

#### `Version() -> str`

Get the Allplan version as string

**Remarks:** Get the Allplan version as string

**Returns:** `str` — Allplan version

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanVersion/#NemAll_Python_AllplanSettings.AllplanVersion.Version)

#### `WindowsReleaseName() -> str`

Get the Allplan window release name Returns: 'P' for project version or empty string for the normal version

**Remarks:** Get the Allplan window release name Returns: 'P' for project version or empty string for the normal version

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanVersion/#NemAll_Python_AllplanSettings.AllplanVersion.WindowsReleaseName)

## AngleUnits (enum)

Angle units

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AngleUnits/)

### Values
- `eDeg` = `1`
- `eGon` = `2`
- `eRad` = `0`

## FontProvider (class)

Provides the methods to obtain the list of the fonts.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/)

### Methods
#### `FontMMToPoints(mmValue: float) -> float`

Converts mm to points

**Remarks:** Converts mm to points

**Parameters:**
- `mmValue` (float) — Value to convert in millimeters

**Returns:** `float` — Value in points

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.FontMMToPoints)

#### `FontMMToTwips(mmValue: float) -> float`

Font size conversions. It is a font special factor (0.714) being applied in order to associate the mm unit with the real glyph size (like for vector fonts) without sub and superscripts space. So these function do not provide common standard point/twips/millimeter conversions!!! Converts mm to twips

**Remarks:** Font size conversions. It is a font special factor (0.714) being applied in order to associate the mm unit with the real glyph size (like for vector fonts) without sub and superscripts space. So these function do not provide common standard point/twips/millimeter conversions!!! Converts mm to twips

**Parameters:**
- `mmValue` (float) — Value to convert in millimeters

**Returns:** `float` — Value in twips

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.FontMMToTwips)

#### `FontPointsToMM(twipsValue: float) -> float`

Converts points to mm

**Remarks:** Converts points to mm

**Parameters:**
- `twipsValue` (float) — Value in points

**Returns:** `float` — Value in mm

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.FontPointsToMM)

#### `FontTwipsToMM(twipsValue: float) -> float`

Converts twips to mm

**Remarks:** Converts twips to mm

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.FontTwipsToMM)

#### `GetCharsetList(nFontID: int) -> tuple[bool, list[int]] | GetCharsetList(fontName: str) -> tuple[bool, list[int]]`

Returns the list of available charsets for the given font

**Remarks:** Returns the list of available charsets for the given font

**Parameters:**
- `nFontID` (int) — Font ID

**Returns:** `bool` — True if at least one Charset for the given font ID exists

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.GetCharsetList)

#### `GetFirstValidPredefinedFontID() -> int`

Returns ID of the first existing predefined TTF font

**Remarks:** Returns ID of the first existing predefined TTF font

**Returns:** `int` — font ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.GetFirstValidPredefinedFontID)

#### `GetFontID(fontName: str, bCheckExistence: int = 1) -> int`

Get font ID for the given font name.

**Remarks:** Get font ID for the given font name.

**Parameters:**
- `fontName` (str) — Name of the font
- `bCheckExistence` (int) — When set to True, the font with the particular ID has to be installed. If it's not the case, the function returns an empty string. When set to False, the function returns the name of the font regardless of whether it is installed or not.

**Returns:** `int` — font ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.GetFontID)

#### `GetFontName(fontID: int, bCheckExistence: int = 1) -> str`

Returns the name of the font Returns FontName corresponding to fontID If fontID is invalid then the empty string is returned

**Remarks:** Returns the name of the font Returns FontName corresponding to fontID If fontID is invalid then the empty string is returned

**Parameters:**
- `fontID` (int) — ID of the font
- `bCheckExistence` (int) — When set to True, the font with the particular ID has to be installed. If it's not the case, the function returns an empty string. When set to False, the function returns the name of the font regardless of whether it is installed or not.

**Returns:** `str` — Font name. If fontID is invalid then the empty string is returned.

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.GetFontName)

#### `GetNemFontIDs() -> tuple[bool, list[int]]`

Returns the list of the IDs of all Allplan's fonts

**Remarks:** Returns the list of the IDs of all Allplan's fonts

**Returns:** `bool` — True when at least one font is installed

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.GetNemFontIDs)

#### `GetNemFonts() -> tuple[bool, list[str]]`

Returns the list of the names of all Allplan's fonts

**Remarks:** Returns the list of the names of all Allplan's fonts

**Returns:** `bool` — True when at least one font is installed

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.GetNemFonts)

#### `GetPredefinedFontIDs() -> tuple[bool, list[int]]`

Returns the list of then IDs of all installed and predefined TTF fonts

**Remarks:** Returns the list of then IDs of all installed and predefined TTF fonts

**Returns:** `bool` — True when at least one font is installed

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.GetPredefinedFontIDs)

#### `GetPredefinedFonts() -> tuple[bool, list[str]]`

Returns the list of then names of all installed and predefined TTF fonts

**Remarks:** Returns the list of then names of all installed and predefined TTF fonts

**Returns:** `bool` — TRUE when at least one font is installed

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.GetPredefinedFonts)

#### `GetSystemFontIDs() -> tuple[bool, list[int]]`

Returns the list of the IDs of all installed TTF fonts

**Remarks:** Returns the list of the IDs of all installed TTF fonts

**Returns:** `bool` — TRUE when at least one font is installed

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.GetSystemFontIDs)

#### `GetSystemFonts() -> tuple[bool, list[str]]`

Returns the list of the names of all installed TTF fonts

**Remarks:** Returns the list of the names of all installed TTF fonts

**Returns:** `bool` — True when at least one font is installed

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.GetSystemFonts)

#### `GetTTFFontSubtype(fontName: str) -> int`

Returns type of checked TTF font

**Remarks:** Returns type of checked TTF font

**Parameters:**
- `fontName` (str) — name of the TTF font

**Returns:** `int` — type of the TTF font

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.GetTTFFontSubtype)

#### `Instance() -> FontProvider`

Get the instance of the font provider

**Remarks:** Get the instance of the font provider

**Returns:** `FontProvider` — Instance of the font provider

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.Instance)

#### `NormalizeFontName(fontName: str, CharSet: int = 1) -> str`

Removes the script suffix from the font name

**Remarks:** Removes the script suffix from the font name

**Parameters:**
- `fontName` (str) — font name
- `CharSet` (int) — Charset

**Returns:** `str` — Font name without suffix

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.NormalizeFontName)

#### `Refresh()`

Reinitialize the fonts It'd be called when some system's font was changed ( added/removed )

**Remarks:** Reinitialize the fonts It'd be called when some system's font was changed ( added/removed )

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.Refresh)

## Functions (static-class)

Module-level functions of NemAll_Python_AllplanSettings

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/)

### Methods
#### `GetAngleUnit() -> AngleUnits`

Get the angle unit

**Remarks:** Get the angle unit

**Returns:** `AngleUnits` — Angle unit

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/#NemAll_Python_AllplanSettings.GetAngleUnit)

#### `GetLengthUnit() -> LengthUnits`

Get the length unit

**Remarks:** Get the length unit

**Returns:** `LengthUnits` — Length unit

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/#NemAll_Python_AllplanSettings.GetLengthUnit)

## ImperialUnitService (class)

Utility class offering methods for conversion from metric into imperial units

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/ImperialUnitService/)

### Methods
#### `ConvertToMM(imperialValue: str) -> tuple`

Convert an imperial length unit to mm

**Remarks:** Convert an imperial length unit to mm

**Parameters:**
- `imperialValue` (str) — Imperial value as string

**Returns:** `tuple` — True when conversion failed, False when it was successful

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/ImperialUnitService/#NemAll_Python_AllplanSettings.ImperialUnitService.ConvertToMM)

#### `ConvertTokg(imperialValue: str) -> tuple`

Convert an imperial weight unit to kg

**Remarks:** Convert an imperial weight unit to kg

**Parameters:**
- `imperialValue` (str) — Imperial value as string

**Returns:** `tuple` — True when conversion failed, False when it was successful

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/ImperialUnitService/#NemAll_Python_AllplanSettings.ImperialUnitService.ConvertTokg)

#### `IsImperialUnit() -> bool`

Check whether the current input unit is imperial unit

**Remarks:** Check whether the current input unit is imperial unit

**Returns:** `bool` — Imperial unit input: True/False

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/ImperialUnitService/#NemAll_Python_AllplanSettings.ImperialUnitService.IsImperialUnit)

## InputAngleSettings (class)

Utility containing functions to control the Allplan settings related to the input angle

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/InputAngleSettings/)

### Methods
#### `GetAngleInputUnit() -> eAngleGradientUnit`

Gets the current setting for unit of angle

**Remarks:** Gets the current setting for unit of angle

**Returns:** `eAngleGradientUnit` — Unit for angle

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/InputAngleSettings/#NemAll_Python_AllplanSettings.InputAngleSettings.GetAngleInputUnit)

#### `GetGradientInputUnit() -> eAngleGradientUnit`

Gets the current setting for Inclination as. If it's set to Angle, the result will be the current Unit of angle setting

**Remarks:** Gets the current setting for Inclination as. If it's set to Angle, the result will be the current Unit of angle setting

**Returns:** `eAngleGradientUnit` — Units for the inclination

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/InputAngleSettings/#NemAll_Python_AllplanSettings.InputAngleSettings.GetGradientInputUnit)

#### `GetProjectAngle(noCache: bool = False) -> float`

Get the project angle

**Remarks:** Get the project angle

**Parameters:**
- `noCache` (bool) — Use no cache: true/false

**Returns:** `float` — Project angle

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/InputAngleSettings/#NemAll_Python_AllplanSettings.InputAngleSettings.GetProjectAngle)

#### `GetSystemAngle() -> float`

Get the system angle (rotation angle of the cursor)

**Remarks:** Get the system angle (rotation angle of the cursor)

**Returns:** `float` — System angle in radian

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/InputAngleSettings/#NemAll_Python_AllplanSettings.InputAngleSettings.GetSystemAngle)

#### `SetProjectAngle(angle: float)`

Set the project angle

**Remarks:** Set the project angle

**Parameters:**
- `angle` (float) — Project angle

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/InputAngleSettings/#NemAll_Python_AllplanSettings.InputAngleSettings.SetProjectAngle)

#### `SetSystemAngle(angle: float)`

Set the system angle

**Remarks:** Set the system angle

**Parameters:**
- `angle` (float) — New system angle

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/InputAngleSettings/#NemAll_Python_AllplanSettings.InputAngleSettings.SetSystemAngle)

## LengthUnits (enum)

Length units

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/LengthUnits/)

### Values
- `eCentimeters` = `1`
- `eDecimeters` = `2`
- `eFeets` = `8`
- `eFeetsAndInchesDecimal` = `9`
- `eFeetsAndInchesExact` = `11`
- `eFeetsAndInchesFractional` = `10`
- `eInchesDecimal` = `5`
- `eInchesExact` = `7`
- `eInchesFractional` = `6`
- `eKilometers` = `4`
- `eMeters` = `3`
- `eMillimeters` = `0`

## PictResDoorSwingType (enum)

Picture resources for the door swing type

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/PictResDoorSwingType/)

### Values
- `eBiFold` = `-1`
- `eDoubleOppositeSwingCircular` = `19751`
- `eDoubleOppositeSwingLinear` = `19753`
- `eDoubleSwingCircular` = `19747`
- `eDoubleSwingLinear` = `19749`
- `eFolding` = `19775`
- `eLifting` = `19777`
- `eLiftingSingleSwingCircular` = `19763`
- `eLiftingSingleSwingLinear` = `19765`
- `eLiftingSliding` = `19771`
- `eNone` = `0`
- `eOneSidedDoubleRevolving` = `19787`
- `eOneSidedRevolving` = `19783`
- `eOneSidedSwingOptional` = `19791`
- `ePendulumDoubleSwingCircular` = `19759`
- `ePendulumDoubleSwingLinear` = `19761`
- `ePendulumSingleSwingCircular` = `19755`
- `ePendulumSingleSwingLinear` = `19757`
- `eRevolving` = `19773`
- `eSingleSwingCircular` = `19743`
- `eSingleSwingLinear` = `19745`
- `eSliding` = `-1`
- `eSlidingDoubleSwing` = `19769`
- `eSlidingSingleSwing` = `19767`
- `eSwing` = `19781`
- `eTwoSidedDoubleRevolving` = `19789`
- `eTwoSidedFolding` = `19779`
- `eTwoSidedRevolving` = `19785`
- `eTwoSidedSwingOptional` = `19793`

## PictResEdgeOffsetType (enum)

Picture resources for edge offset types

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/PictResEdgeOffsetType/)

### Values
- `eMajorValueAtEnd` = `12151`
- `eMajorValueAtStart` = `12147`
- `eStartEqualEnd` = `12149`
- `eZeroAtEnd` = `12153`
- `eZeroAtStart` = `12145`

## PictResPalette (enum)

Picture resources for the palette

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/PictResPalette/)

### Values
- `eCenterOfGravity` = `19407`
- `eCenterOfGravitySelected` = `19405`
- `eClear` = `17025`
- `eHotinfo` = `16205`
- `eRollLeft` = `8543`
- `eRollRight` = `8542`

## PictResParam (enum)

Picture resources for the parameter icons

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/PictResParam/)

### Values
- `eParam01` = `19093`
- `eParam010` = `19111`
- `eParam011` = `19113`
- `eParam012` = `19115`
- `eParam013` = `19117`
- `eParam014` = `19119`
- `eParam015` = `19121`
- `eParam016` = `19123`
- `eParam017` = `19125`
- `eParam018` = `19127`
- `eParam019` = `19129`
- `eParam02` = `19095`
- `eParam020` = `19131`
- `eParam03` = `19097`
- `eParam04` = `19099`
- `eParam05` = `19101`
- `eParam06` = `19103`
- `eParam07` = `19105`
- `eParam08` = `19107`
- `eParam09` = `19109`

## PictResPlaneReferences (enum)

Picture resources for the plan references

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/PictResPlaneReferences/)

### Values
- `eAbsElevationBottom` = `8567`
- `eAbsElevationTop` = `8568`
- `eBottomFixed` = `8576`
- `eBottomPlane` = `8569`
- `eBottomPlaneFromTop` = `8570`
- `eComponentsBottomPlane` = `8574`
- `eComponentsTopPlane` = `8575`
- `eTopFixed` = `8576`
- `eTopPlane` = `8572`
- `eTopPlaneFromBottom` = `8571`

## PictResRevealType (enum)

Picture resources for reveal types

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/PictResRevealType/)

### Values
- `eEmbedded` = `19709`
- `eExterior` = `19713`
- `eInterior` = `19711`
- `eNone` = `17025`

## PictResShapeType (enum)

Picture resources for shape types

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/PictResShapeType/)

### Values
- `eArbitrary` = `14601`
- `eCircle` = `14587`
- `eDiamond` = `14585`
- `ePolygon` = `14599`
- `eRectangle` = `14583`
- `eRegularPolygonCircumscribed` = `14589`
- `eRegularPolygonInscribed` = `14591`
- `eRiseBottomTop` = `14597`
- `eSemiCircle` = `14595`
- `eSemiDiamond` = `14593`

## PictResSillType (enum)

Picture resources for sill types

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/PictResSillType/)

### Values
- `eBothsides` = `19707`
- `eInner` = `19703`
- `eNone` = `19701`
- `eOuter` = `19705`

## PictResTierOffsetType (enum)

Picture resources for tier offset types

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/PictResTierOffsetType/)

### Values
- `eAdvanced` = `16507`
- `eInnerSide` = `16505`
- `eNone` = `16501`
- `eOuterSide` = `16503`
- `eWithInnerFacing` = `16511`
- `eWithOuterFacing` = `16509`

## PictResWallTierCount (enum)

Picture resources for edge offset types

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/PictResWallTierCount/)

### Values
- `eFiveTiers` = `14581`
- `eFourTiers` = `14579`
- `eOneTier` = `14573`
- `eThreeTiers` = `14577`
- `eTwoTiers` = `14575`

## PythonPartsSettings (class)

Implementation of the PythonParts settings

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/PythonPartsSettings/)

### Methods
#### `GetInstance() -> PythonPartsSettings`

Get the instance

**Remarks:** Get the instance

**Returns:** `PythonPartsSettings` — Instance of the singleton

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/PythonPartsSettings/#NemAll_Python_AllplanSettings.PythonPartsSettings.GetInstance)

#### `SetLengthUnit(lengthUnit: int)`

Set the length unit

**Remarks:** Set the length unit

**Parameters:**
- `lengthUnit` (int) — Length unit: 3 = m

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/PythonPartsSettings/#NemAll_Python_AllplanSettings.PythonPartsSettings.SetLengthUnit)

#### `SetShowFullPreview(showFullPreview: bool)`

Show the full preview

**Remarks:** Show the full preview

**Parameters:**
- `showFullPreview` (bool) — Show full preview: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/PythonPartsSettings/#NemAll_Python_AllplanSettings.PythonPartsSettings.SetShowFullPreview)

### Properties
- `ShowFullPreview` (bool, get/set) — Get the full preview state Show the full preview
- `UpdateIdenticalPythonParts` (UpdateIdenticalPythonPartsState, get/set) — Get the update identical PythonParts state

## TextResDoorSwingType (enum)

Text resources for the door swing type

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/TextResDoorSwingType/)

### Values
- `eBiFold` = `-1`
- `eDoubleOppositeSwingCircular` = `36404`
- `eDoubleOppositeSwingLinear` = `36405`
- `eDoubleSwingCircular` = `36402`
- `eDoubleSwingLinear` = `36403`
- `eFolding` = `36417`
- `eLifting` = `36426`
- `eLiftingSingleSwingCircular` = `36410`
- `eLiftingSingleSwingLinear` = `36411`
- `eLiftingSliding` = `36415`
- `eNone` = `36399`
- `eOneSidedDoubleRevolving` = `36422`
- `eOneSidedRevolving` = `36420`
- `eOneSidedSwingOptional` = `36424`
- `ePendulumDoubleSwingCircular` = `36408`
- `ePendulumDoubleSwingLinear` = `36409`
- `ePendulumSingleSwingCircular` = `36406`
- `ePendulumSingleSwingLinear` = `36407`
- `eRevolving` = `36416`
- `eSingleSwingCircular` = `36400`
- `eSingleSwingLinear` = `36401`
- `eSliding` = `-1`
- `eSlidingDoubleSwing` = `36413`
- `eSlidingSingleSwing` = `36412`
- `eSwing` = `36419`
- `eTwoSidedDoubleRevolving` = `36423`
- `eTwoSidedFolding` = `36427`
- `eTwoSidedRevolving` = `36421`
- `eTwoSidedSwingOptional` = `36425`

## TextResRevealType (enum)

Texture resources for reveal types

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/TextResRevealType/)

### Values
- `eEmbedded` = `36386`
- `eExterior` = `36388`
- `eInterior` = `36387`
- `eNone` = `36385`

## TextResShapeType (enum)

Texture resouces for shape types

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/TextResShapeType/)

### Values
- `eArbitrary` = `36391`
- `eCircle` = `36390`
- `eDiamond` = `36389`
- `ePolygon` = `36496`
- `eRectangle` = `36367`
- `eRegularPolygonCircumscribed` = `36440`
- `eRegularPolygonInscribed` = `36439`
- `eRiseBottomTop` = `36370`
- `eSemiCircle` = `36369`
- `eSemiDiamond` = `36368`

## TextResSillType (enum)

Texture resources for sill types

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/TextResSillType/)

### Values
- `eBothsides` = `36384`
- `eInner` = `36382`
- `eNone` = `36381`
- `eOuter` = `36383`

## TextResTierOffsetType (enum)

Text resources for tier offset types

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/TextResTierOffsetType/)

### Values
- `eAdvanced` = `36395`
- `eInnerSide` = `36394`
- `eNone` = `36392`
- `eOuterSide` = `36393`
- `eWithInnerFacing` = `36396`
- `eWithOuterFacing` = `36397`

## UnitService (class)

Unit service

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/UnitService/)

### Methods
#### `ConvertToMM(lengthString: str) -> tuple`

Convert a value string to mm

**Remarks:** Convert a value string to mm

**Parameters:**
- `lengthString` (str) — Length as string

**Returns:** `tuple` — Length in mm

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/UnitService/#NemAll_Python_AllplanSettings.UnitService.ConvertToMM)

#### `ToLengthUnitString(length: float) -> str`

Convert a length(mm) to a string by the current length unit

**Remarks:** Convert a length(mm) to a string by the current length unit

**Parameters:**
- `length` (float) — Length in mm

**Returns:** `str` — String with the value in the current length unit

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/UnitService/#NemAll_Python_AllplanSettings.UnitService.ToLengthUnitString)

## UpdateIdenticalPythonPartsState (enum)

Implementation of the update identical PythonParts state

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/UpdateIdenticalPythonPartsState/)

### Values
- `eDoNotUpdateIdentical` = `2`
- `eDoUpdateIdentical` = `1`
- `eUndefinded` = `0`

## eAngleGradientUnit (enum)

Enumeration of available angle units for input of rotation and inclination

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/eAngleGradientUnit/)

### Values
- `ANGLE_DEGREE` = `1` — Degrees
- `ANGLE_GON` = `3` — Gradians
- `ANGLE_NOTDEF` = `0` — Undefined
- `ANGLE_PERCENT` = `5` — Inclination as %
- `ANGLE_PERCENTAGE` = `4` — Inclination as 1:x
- `ANGLE_PERMILL` = `6` — Inclination as ‰
- `ANGLE_RADIAN` = `2` — Radians

