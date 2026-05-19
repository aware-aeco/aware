---
name: allplan-nemall_python_allplansettings
description: This skill encodes the allplan 2024.0 surface of the NemAll_Python_AllplanSettings namespace — 11 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AllplanLocalisationService, AllplanPaths, AllplanVersion, FontProvider, AllplanGlobalSettings, AngleUnits, LengthUnits, PythonPartsSettings, and 3 more types.
---

# NemAll_Python_AllplanSettings

Auto-generated from vendor docs for allplan 2024.0. 11 types in this namespace.

## AllplanGlobalSettings (class)

Helper class for global Allplan settings exposure

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanGlobalSettings/)

### Methods
#### `GetCurrentColorId()`

Get the current used color ID in Allplan

**Remarks:** Get the current used color ID in Allplan

**Returns:** `int` — Current color ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanGlobalSettings/#NemAll_Python_AllplanSettings.AllplanGlobalSettings.GetCurrentColorId)

#### `GetCurrentCommonProperties()`

Get the current common properties

**Remarks:** Get the current common properties

**Returns:** `CommonProperties` — Current common properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanGlobalSettings/#NemAll_Python_AllplanSettings.AllplanGlobalSettings.GetCurrentCommonProperties)

#### `GetCurrentFaceStyleId()`

Get the current used face style ID in Allplan

**Remarks:** Get the current used face style ID in Allplan

**Returns:** `int` — Current face style ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanGlobalSettings/#NemAll_Python_AllplanSettings.AllplanGlobalSettings.GetCurrentFaceStyleId)

#### `GetCurrentFontId()`

Get the current used font ID in Allplan

**Remarks:** Get the current used font ID in Allplan

**Returns:** `int` — Current font ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanGlobalSettings/#NemAll_Python_AllplanSettings.AllplanGlobalSettings.GetCurrentFontId)

#### `GetCurrentHatchId()`

Get the current used hatch ID in Allplan

**Remarks:** Get the current used hatch ID in Allplan

**Returns:** `int` — Current hatch ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanGlobalSettings/#NemAll_Python_AllplanSettings.AllplanGlobalSettings.GetCurrentHatchId)

#### `GetCurrentLayerId()`

Get the current used layer ID in Allplan

**Remarks:** Get the current used layer ID in Allplan

**Returns:** `int` — Current layer ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanGlobalSettings/#NemAll_Python_AllplanSettings.AllplanGlobalSettings.GetCurrentLayerId)

#### `GetCurrentPatternId()`

Get the current used pattern ID in Allplan

**Remarks:** Get the current used pattern ID in Allplan

**Returns:** `int` — Current pattern ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanGlobalSettings/#NemAll_Python_AllplanSettings.AllplanGlobalSettings.GetCurrentPatternId)

#### `GetCurrentPenId()`

Get the current used pen ID in Allplan

**Remarks:** Get the current used pen ID in Allplan

**Returns:** `int` — Current pen ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanGlobalSettings/#NemAll_Python_AllplanSettings.AllplanGlobalSettings.GetCurrentPenId)

#### `GetCurrentStrokeId()`

Get the current used stroke ID in Allplan

**Remarks:** Get the current used stroke ID in Allplan

**Returns:** `int` — Current stroke ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanGlobalSettings/#NemAll_Python_AllplanSettings.AllplanGlobalSettings.GetCurrentStrokeId)

#### `GetOffsetPoint()`

Get the offset point

**Remarks:** Get the offset point

**Returns:** `Point3D` — Offset point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanGlobalSettings/#NemAll_Python_AllplanSettings.AllplanGlobalSettings.GetOffsetPoint)

## AllplanLocalisationService (class)

Class for extraction of Allplan localization settings

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanLocalisationService/)

### Methods
#### `AllplanLanguage()`

Get the installed Allplan language

**Remarks:** Get the installed Allplan language

**Returns:** `str` — Allplans currently used language as a three letter description bul - Bulgarian deu - German eng - English fin - Finnish fra - French grc - Greek hol - Dutch hrv - Croatian ita - Italian pol - Polish rum - Romanian rus - Russian slk - Slovak spa - Spanish svn - Slovenian tch - Czech trk - Turkish ung - Hungarian dan - Danish ser - Serbian mak - Macedonian prt - Portuguese ltu - Lithuanian lva - Latvian est - Estonian ukr - Ukrainian swe - Swedish nor - Norwegian chn - Chinese kor - Korean jpn - Japanese usa - USA - English vie - Vietnamese

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanLocalisationService/#NemAll_Python_AllplanSettings.AllplanLocalisationService.AllplanLanguage)

#### `Language()`

Get the current Allplan language

**Remarks:** Get the current Allplan language

**Returns:** `str` — Allplans currently used language as a two letter description bg - Bulgarian de - German en - English fi - Finnish fr - French el - Greek nl - Dutch hr - Croatian it - Italian pl - Polish ro - Romanian ru - Russian sk - Slovak es - Spanish sl - Slovenian cs - Czech tr - Turkish hu - Hungarian da - Danish sr - Serbian mk - Macedonian pt - Portuguese lt - Lithuanian lv - Latvian et - Estonian uk - Ukrainian sv - Swedish no - Norwegian zh - Chinese ko - Korean ja - Japanese vi - Vietnamese

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanLocalisationService/#NemAll_Python_AllplanSettings.AllplanLocalisationService.Language)

## AllplanPaths (class)

Class for getting Allplan paths

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanPaths/)

### Methods
#### `GetCurPrjDesignPath()`

Get the current project design path

**Remarks:** Get the current project design path

**Returns:** `str` — Current project design path

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanPaths/#NemAll_Python_AllplanSettings.AllplanPaths.GetCurPrjDesignPath)

#### `GetCurPrjPath()`

Get the current project path

**Remarks:** Get the current project path

**Returns:** `str` — Current project path

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanPaths/#NemAll_Python_AllplanSettings.AllplanPaths.GetCurPrjPath)

#### `GetEtcPath()`

Get the Etc path

**Remarks:** Get the Etc path

**Returns:** `str` — Etc path

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanPaths/#NemAll_Python_AllplanSettings.AllplanPaths.GetEtcPath)

#### `GetPathOfApplication()`

Get the path of the application

**Remarks:** Get the path of the application

**Returns:** `str` — Path of the application

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanPaths/#NemAll_Python_AllplanSettings.AllplanPaths.GetPathOfApplication)

#### `GetPrgPath()`

Get the program path

**Remarks:** Get the program path

**Returns:** `str` — Program path

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanPaths/#NemAll_Python_AllplanSettings.AllplanPaths.GetPrgPath)

#### `GetPythonPartsEtcPath()`

Get the PythonParts Etc path

**Remarks:** Get the PythonParts Etc path

**Returns:** `str` — Etc path

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanPaths/#NemAll_Python_AllplanSettings.AllplanPaths.GetPythonPartsEtcPath)

#### `GetStdDesignPath()`

Get the Std design path

**Remarks:** Get the Std design path

**Returns:** `str` — Std design path

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanPaths/#NemAll_Python_AllplanSettings.AllplanPaths.GetStdDesignPath)

#### `GetStdPath()`

Get the Std path

**Remarks:** Get the Std path

**Returns:** `str` — Std path

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanPaths/#NemAll_Python_AllplanSettings.AllplanPaths.GetStdPath)

#### `GetTmpPath()`

Get the Tmp path

**Remarks:** Get the Tmp path

**Returns:** `str` — Tmp path

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanPaths/#NemAll_Python_AllplanSettings.AllplanPaths.GetTmpPath)

#### `GetUsrPath()`

Get the Usr path

**Remarks:** Get the Usr path

**Returns:** `str` — Usr path

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanPaths/#NemAll_Python_AllplanSettings.AllplanPaths.GetUsrPath)

## AllplanVersion (class)

Class for extraction of Allplan version information

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanVersion/)

### Methods
#### `MainReleaseName()`

Get the Allplan main release name Returns: Allplan main release name ('2016', '2017', ...)

**Remarks:** Get the Allplan main release name Returns: Allplan main release name ('2016', '2017', ...)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanVersion/#NemAll_Python_AllplanSettings.AllplanVersion.MainReleaseName)

#### `SubReleaseName()`

Get the Allplan sub release name Returns: Allplan sub release name ('2016.0', '2016.1', ...)

**Remarks:** Get the Allplan sub release name Returns: Allplan sub release name ('2016.0', '2016.1', ...)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanVersion/#NemAll_Python_AllplanSettings.AllplanVersion.SubReleaseName)

#### `Version()`

Get the Allplan version as string Returns: Allplan version 2016.1 2016: Allplan main version .1: Allplan sub version 2016.1.1 2016: Allplan main version .1.1: Allplan sub version

**Remarks:** Get the Allplan version as string Returns: Allplan version 2016.1 2016: Allplan main version .1: Allplan sub version 2016.1.1 2016: Allplan main version .1.1: Allplan sub version

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanVersion/#NemAll_Python_AllplanSettings.AllplanVersion.Version)

#### `WindowsReleaseName()`

Get the Allplan window release name Returns: 'P' for project version or empty string for the normal version

**Remarks:** Get the Allplan window release name Returns: 'P' for project version or empty string for the normal version

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanVersion/#NemAll_Python_AllplanSettings.AllplanVersion.WindowsReleaseName)

## AngleUnits (enum)

Angle units

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AngleUnits/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `AngleUnits` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AngleUnits/#NemAll_Python_AllplanSettings.AngleUnits.__getitem__)

### Values
- `eDeg` = `1`
- `eGon` = `2`
- `eRad` = `0`

## FontProvider (class)

Provides the methods to obtain the list of the fonts

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/)

### Methods
#### `FontMMToPoints(mmValue)`

Converts mm to points

**Remarks:** Converts mm to points

**Parameters:**
- `mmValue` (float)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.FontMMToPoints)

#### `FontMMToTwips(mmValue)`

Font size conversions. It is a font special factor (0.714) being applied in order to associate the mm unit with the real glyph size (like for vector fonts) without sub and superscripts space. So these function do not provide common standard point/twips/millimeter conversions!!! Converts mm to twips

**Remarks:** Font size conversions. It is a font special factor (0.714) being applied in order to associate the mm unit with the real glyph size (like for vector fonts) without sub and superscripts space. So these function do not provide common standard point/twips/millimeter conversions!!! Converts mm to twips

**Parameters:**
- `mmValue` (float)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.FontMMToTwips)

#### `FontPointsToMM(twipsValue)`

Converts points to mm

**Remarks:** Converts points to mm

**Parameters:**
- `twipsValue` (float)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.FontPointsToMM)

#### `FontTwipsToMM(twipsValue)`

Converts twips to mm

**Remarks:** Converts twips to mm

**Parameters:**
- `twipsValue` (float)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.FontTwipsToMM)

#### `GetCharsetList(fontName)`

Returns the list of available charsets for the given font

**Remarks:** Returns the list of available charsets for the given font

**Parameters:**
- `fontName` (str) — Font name

**Returns:** `bool` — tuple(Charsets for the given font,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.GetCharsetList)

#### `GetCharsetList(nFontID)`

Returns the list of available charsets for the given font

**Remarks:** Returns the list of available charsets for the given font

**Parameters:**
- `nFontID` (int) — Font ID

**Returns:** `bool` — tuple(Charsets for the given font,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.GetCharsetList)

#### `GetFirstValidPredefinedFontID()`

Returns ID of the first existing predefined TTF font

**Remarks:** Returns ID of the first existing predefined TTF font

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.GetFirstValidPredefinedFontID)

#### `GetFontID(fontName, bCheckExistence=1)`

Return ID for the given font Return the ID for the given font name. ID is the index to the table m_PreDefinedFontNameList. etval 0 Dialog Font ID etval 1-20 ID of the hardcoded Allplan's fonts ( see table m_PreDefinedFontNameList ) etval 21-63 ID of the hardcoded system's fonts ( see table m_PreDefinedFontNameList ) etval >=SYSTEM_FONT_ID (1000) ID of the system TTF font ( this ID is valid only during Allplan's session ) etval >=NON_EXISTING_FONT_ID(10000) ID of the font that isn't installed nor exists in the predefined table( this ID is valid only during Allplan's session ) etval INVALID_FONT_ID(N_FONT_INTERNAL_MIN_ID - 1) Invalid ID, font with given name doesn't exist

**Remarks:** Return ID for the given font Return the ID for the given font name. ID is the index to the table m_PreDefinedFontNameList. etval 0 Dialog Font ID etval 1-20 ID of the hardcoded Allplan's fonts ( see table m_PreDefinedFontNameList ) etval 21-63 ID of the hardcoded system's fonts ( see table m_PreDefinedFontNameList ) etval >=SYSTEM_FONT_ID (1000) ID of the system TTF font ( this ID is valid only during Allplan's session ) etval >=NON_EXISTING_FONT_ID(10000) ID of the font that isn't installed nor exists in the predefined table( this ID is valid only during Allplan's session ) etval INVALID_FONT_ID(N_FONT_INTERNAL_MIN_ID - 1) Invalid ID, font with given name doesn't exist

**Parameters:**
- `fontName` (str) — Name of the font
- `bCheckExistence` (int) — When bCheckExistence is TRUE then the font with the particular ID has to be installed. In case, that this font is not installed, the function returns the empty string. When bCheckExistence is FALSE then the function returns the name of the font regardless of it is installed or not.

**Returns:** `Return` — int

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.GetFontID)

#### `GetFontName(fontID, bCheckExistence=1)`

Returns the name of the font Returns FontName corresponding to fontID If fontID is invalid then the empty string is returned

**Remarks:** Returns the name of the font Returns FontName corresponding to fontID If fontID is invalid then the empty string is returned

**Parameters:**
- `fontID` (int) — ID of the font
- `bCheckExistence` (int) — When bCheckExistence is TRUE then the font with the particular ID has to be installed. In case, that this font is not installed, the function returns the empty string. When bCheckExistence is FALSE then the function returns the name of the font regardless of it is installed or not.

**Returns:** `str` — Returns FontName. If fontID is invalid then the empty string is returned.

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.GetFontName)

#### `GetNemFontIDs()`

Returns the list of the IDs of all Allplan's fonts

**Remarks:** Returns the list of the IDs of all Allplan's fonts

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.GetNemFontIDs)

#### `GetNemFonts()`

Returns the list of the names of all Allplan's fonts

**Remarks:** Returns the list of the names of all Allplan's fonts

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.GetNemFonts)

#### `GetPredefinedFontIDs()`

Returns the list of then IDs of all installed and predefined TTF fonts

**Remarks:** Returns the list of then IDs of all installed and predefined TTF fonts

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.GetPredefinedFontIDs)

#### `GetPredefinedFonts()`

Returns the list of then names of all installed and predefined TTF fonts

**Remarks:** Returns the list of then names of all installed and predefined TTF fonts

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.GetPredefinedFonts)

#### `GetSystemFontIDs()`

Returns the list of the IDs of all installed TTF fonts

**Remarks:** Returns the list of the IDs of all installed TTF fonts

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.GetSystemFontIDs)

#### `GetSystemFonts()`

Returns the list of the names of all installed TTF fonts

**Remarks:** Returns the list of the names of all installed TTF fonts

**Returns:** `tuple[bool, list[str]]` — The function returns TRUE when at least one font is installed

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.GetSystemFonts)

#### `GetTTFFontSubtype(fontName)`

Returns type of checked TTF font

**Remarks:** Returns type of checked TTF font

**Parameters:**
- `fontName` (str)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.GetTTFFontSubtype)

#### `Instance()`

Get the instance of the font provider

**Remarks:** Get the instance of the font provider

**Returns:** `FontProvider` — Instance of the font provider

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.Instance)

#### `NormalizeFontName(fontName, CharSet=1)`

Removes from the fontname script suffix e.g. Arial CE -> Arial

**Remarks:** Removes from the fontname script suffix e.g. Arial CE -> Arial

**Parameters:**
- `fontName` (str)
- `CharSet` (int)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.NormalizeFontName)

#### `Refresh()`

Reinitialize the fonts It'd be called when some system's font was changed ( added/removed )

**Remarks:** Reinitialize the fonts It'd be called when some system's font was changed ( added/removed )

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/FontProvider/#NemAll_Python_AllplanSettings.FontProvider.Refresh)

## Functions (static-class)

Module-level functions of NemAll_Python_AllplanSettings

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/_functions/)

### Methods
#### `GetAngleUnit()`

Get the angle unit

**Remarks:** Get the angle unit

**Returns:** `AngleUnits` — Angle unit

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/_functions/#NemAll_Python_AllplanSettings.GetAngleUnit)

#### `GetLengthUnit()`

Get the length unit

**Remarks:** Get the length unit

**Returns:** `LengthUnits` — Length unit

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/_functions/#NemAll_Python_AllplanSettings.GetLengthUnit)

## ImperialUnitService (class)

Imperial unit service

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/ImperialUnitService/)

### Methods
#### `ConvertToMM(imperialValue)`

Convert an imperial length unit to mm

**Remarks:** Convert an imperial length unit to mm

**Parameters:**
- `imperialValue` (str) — Imperial value as string

**Returns:** `tuple` — Length in mm

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/ImperialUnitService/#NemAll_Python_AllplanSettings.ImperialUnitService.ConvertToMM)

#### `ConvertTokg(imperialValue)`

Convert an imperial weight unit to kg

**Remarks:** Convert an imperial weight unit to kg

**Parameters:**
- `imperialValue` (str) — Imperial value as string

**Returns:** `tuple` — Weight in kg

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/ImperialUnitService/#NemAll_Python_AllplanSettings.ImperialUnitService.ConvertTokg)

#### `IsImperialUnit()`

Check whether the current input unit is imperial unit

**Remarks:** Check whether the current input unit is imperial unit

**Returns:** `bool` — Imperial unit input: True/False

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/ImperialUnitService/#NemAll_Python_AllplanSettings.ImperialUnitService.IsImperialUnit)

## LengthUnits (enum)

Length units

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/LengthUnits/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `LengthUnits` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/LengthUnits/#NemAll_Python_AllplanSettings.LengthUnits.__getitem__)

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

## PythonPartsSettings (class)

Implementation of the PythonParts settings

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/PythonPartsSettings/)

### Methods
#### `GetInstance()`

Get the instance

**Remarks:** Get the instance

**Returns:** `PythonPartsSettings` — Instance of the singleton

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/PythonPartsSettings/#NemAll_Python_AllplanSettings.PythonPartsSettings.GetInstance)

#### `SetLengthUnit(lengthUnit)`

Set the length unit

**Remarks:** Set the length unit

**Parameters:**
- `lengthUnit` (int) — Length unit: 3 = m

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/PythonPartsSettings/#NemAll_Python_AllplanSettings.PythonPartsSettings.SetLengthUnit)

#### `SetShowFullPreview(showFullPreview)`

Show the full preview

**Remarks:** Show the full preview

**Parameters:**
- `showFullPreview` (bool) — Show full preview: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/PythonPartsSettings/#NemAll_Python_AllplanSettings.PythonPartsSettings.SetShowFullPreview)

### Properties
- `IsShowFullPreview` (bool, get/set) — Get the full preview state Show the full preview

## UnitService (class)

Unit service

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/UnitService/)

### Methods
#### `ConvertToMM(lengthString)`

Convert a value string to mm

**Remarks:** Convert a value string to mm

**Parameters:**
- `lengthString` (str) — Length as string

**Returns:** `tuple` — Length in mm

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/UnitService/#NemAll_Python_AllplanSettings.UnitService.ConvertToMM)

#### `ToLengthUnitString(length)`

Convert a length(mm) to a string by the current length unit

**Remarks:** Convert a length(mm) to a string by the current length unit

**Parameters:**
- `length` (float) — Length in mm

**Returns:** `str` — String with the value in the current length unit

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/UnitService/#NemAll_Python_AllplanSettings.UnitService.ToLengthUnitString)

