---
name: tedds-tekla-structural-interopassemblies-teddscalc
description: This skill encodes the tedds 25.0 surface of the Tekla.Structural.InteropAssemblies.TeddsCalc namespace — 11 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: CalcErrorType, CalcProgressEvent, CalcStatus, CalcValueFormat, CalcValueType, ICalcValue, ICalculator, ICalculatorExEvents, and 3 more types.
---

# Tekla.Structural.InteropAssemblies.TeddsCalc

Auto-generated from vendor docs for tedds 25.0. 11 types in this namespace.

## CalcErrorType (enum)

Error types

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-interopassemblies-teddscalc-calcerrortype-41846)

### Values
- `Expression` = `1` — Expresso expression error - can continue, interrrupt or abort
- `Filter` = `4` — Filter error - cannot continue
- `System` = `0` — System error - cannot continue
- `Units` = `3` — Units library error - can continue only
- `Variables` = `2` — Variables library error - can continue only

## CalcProgressEvent (enum)

Progress bar modes

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-interopassemblies-teddscalc-calcprogressevent-41847)

### Values
- `ProgressAddOutput` = `6` — Add item to output log
- `ProgressClearOutput` = `7` — Clear output log
- `ProgressDisable` = `3` — Disable
- `ProgressEnable` = `4` — Enable
- `ProgressFinished` = `8` — Notify finish
- `ProgressGetStatus` = `5` — Get state
- `ProgressHide` = `12` — Hide progress log
- `ProgressReset` = `0` — Reset position to 0
- `ProgressSetDestroyOnFinish` = `9` — Set flag to destroy on finish
- `ProgressSetHideOnFinish` = `10` — Set flag to hide on finish
- `ProgressSetPos` = `1` — Set position
- `ProgressSetText` = `2` — Set text
- `ProgressShow` = `11` — Show progress log

## CalcStatus (enum)

Calculate status codes. Values less than zero are considered failures, values greater than zero are considered successful.

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-interopassemblies-teddscalc-calcstatus-41848)

### Values
- `Aborted` = `-1` — Calculating was aborted
- `Interrupted` = `2` — The user interrupted the calculation process therefore the state of the results and output is indeterminate
- `NotInitialized` = `0` — The Tedds Calculator has not been initialized
- `Ok` = `1` — Calculating completed successfully

## CalcValueFormat (enum)

Formats for converting numeric values to a string

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-interopassemblies-teddscalc-calcvalueformat-41849)

### Values
- `Engineering` = `3` — Engineering format
- `Fixed` = `0` — Fixed format
- `General` = `2` — General format
- `InputOutput` = `4` — Data Input/Output format for interoperability with other software
- `Scientific` = `1` — Scientific format
- `UserInput` = `5` — User interface input format

## CalcValueType (enum)

Value data types

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-interopassemblies-teddscalc-calcvaluetype-41850)

### Values
- `Expression` = `3` — Sequence of characters which can be calculated
- `Number` = `1` — Numerical value including unit dimensions
- `String` = `2` — Sequence of characters

## ICalcValue (interface)

Interface for an ICalcValue, the fundamental data type used by the calculator for input and output values

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-interopassemblies-teddscalc-icalcvalue-41851)

### Methods
#### `bool ToBool()`

Returns the objects value converted to a boolean. Only applicable for objects of type Number.

**Returns:** `bool` — true if the object contains a non zero value; otherwise false

[Docs](https://developer.tekla.com/topic/en/20/45/b4fdc83e0046694954d62505f12c88689326d8fa91a1934fa6045addf7e17d76)

#### `double ToDouble(string units)`

Returns the objects value converted to a double. Only applicable for CalcValue objects of type Number.

**Parameters:**
- `units` (string) — Units to use for the conversion, if omitted the value will be returned in base units.

**Returns:** `double` — The objects numeric value in the specified units

[Docs](https://developer.tekla.com/topic/en/20/45/0e4047330c7daa66f6d692e0d6fd0bf7a0c5406bf847ad3b2446a715a20657db)

#### `int ToInt()`

Returns the objects value converted to an integer. Only applicable for CalcValue objects of type Number.

**Returns:** `int` — The objects value as an integer

[Docs](https://developer.tekla.com/topic/en/20/45/9bcdc3eede78155e5168291a89eb96ce32646f90006eecd99409e7e8c7584126)

#### `string ToString(string units, CalcValueFormat format = CalcValueFormat.Fixed, int precision = 3)`

Returns the objects value represented as a string.

**Parameters:**
- `units` (string) — Units to use for converting the numeric value, if omitted the value will be represented in base units.
- `format` (CalcValueFormat) — format Expression text format to use for representing the numerical value (E = Engineering, F = Fixed, G = General, S = Scientific, O = Data Input/Output, U = User input)
- `precision` (int) — Numeric precision to use

**Returns:** `string` — The objects value as a string

[Docs](https://developer.tekla.com/topic/en/20/45/a85846735f25776e4707ab06a57ec3b5dbcc30d13e6e3ceccf954bb55ac109e9)

### Properties
- `Degrees` (double, get) — Returns the linear degrees dimension of the object. Only applicable for objects of type Number.
- `IsDimensionless` (bool, get) — Returns whether the units of a numeric object has dimensions. Only applicable for objects of type Number.
- `Length` (double, get) — Returns the length dimension of the object. Only applicable for objects of type Number.
- `Mass` (double, get) — Returns the mass dimension of the object. Only applicable for objects of type Number.
- `Time` (double, get) — Returns the time dimension of the object. Only applicable for objects of type Number.
- `Type` (CalcValueType, get) — Returns the objects data type

## ICalculator (interface)

Interface for the main Tedds Calculator

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-interopassemblies-teddscalc-icalculator-41852)

### Methods
#### `void CopyOutputToClipboard(OutputFormat format = OutputFormat.Rtf)`

Copies the output item to the Windows clipboard in the specified format

**Parameters:**
- `format` (OutputFormat) — Output format, the default is Microsoft Rich Text Format (RTF)

[Docs](https://developer.tekla.com/topic/en/20/45/7459dca22bddaf39178a117918019b0c30050bf7acb67db38adba29948b9afcd)

#### `string GetOutput(OutputFormat format = OutputFormat.Rtf)`

Returns the documented output from the calculator in the specified format.

**Remarks:** In order to obtain output the calculator must have been initialized with RTF, using InitializeCalc or Initialize. If the scenario you are implementing requires both input and output then you will typically initialize the Calculator object twice, the first time to create the input variables for the calculation and the second time to initialise the calculator using those input variables to start a specific calculation. When the output is requested in PDF format the resulting string must be processed correctly. PDF is a binary format and as such the string should not be modified because it is likely to contain invalid characters and null terminators which will not be processed correctly by most string processing functions. The string should be converted to raw bytes before further processing or saving to disk.

**Parameters:**
- `format` (OutputFormat) — Output format, the default is Microsoft Rich Text Format

**Returns:** `string` — Output documentation.

[Docs](https://developer.tekla.com/topic/en/20/45/ca45d8f35efea7491dd22f92bf3f1930264669cef90c8c5fa6e46c1f2518d015)

#### `string GetVariables(VariableFormat format = VariableFormat.Xml)`

Returns all the variables defined in the calculator excluding temporary variables

**Parameters:**
- `format` (VariableFormat) — Format in which to return the variables, the default is XML

**Returns:** `string` — A string which defines the variables and their associated values and attributes. Temporary variables are excluded, if you require the temporary variables use GetVariables2

[Docs](https://developer.tekla.com/topic/en/20/45/c6ba2c4dba023815ffb9c2a21ed88b88b96e2ed4b4dac49d14a89be26cf54f3f)

#### `string GetVariables2(VariableFormat format = VariableFormat.Xml, bool includeTemporaryVariables = false)`

Returns the current Document and Calc Section variables in the specified data format

**Parameters:**
- `format` (VariableFormat) — Format in which to return the variables, the default is XML
- `includeTemporaryVariables` (bool) — Determines whether to include temporary variables

**Returns:** `string` — A string which defines the variables and their associated values and attributes

[Docs](https://developer.tekla.com/topic/en/20/45/dad1d22b7a396d2fb9774f8182fe2cd9422ce37cfb055455f8b0a8d40d3ebcc1)

#### `string GetVariables3(VariableFormat format = VariableFormat.Xml, bool includeTemporaryVariables = false, bool includeModifiedAttribute = false)`

Returns the current Document and Calc Section variables in the specified data format

**Parameters:**
- `format` (VariableFormat) — Format in which to return the variables, the default is XML
- `includeTemporaryVariables` (bool) — Determines whether to include temporary variables
- `includeModifiedAttribute` (bool) — Determines whether to include the modified attribute for each variable

**Returns:** `string` — A string which defines the variables and their associated values and attributes

[Docs](https://developer.tekla.com/topic/en/20/45/fd6733c2fade95aa5a08a45363589f1daa68f1ab5221b76c6d30a86ade441688)

#### `void Initialize(string inputRtf, string variables, string sectionName, int sectionId = 0, int pageWidth = 180, int pageHeight = 260)`

Initializes the calculator to start calculating

**Remarks:** The initialize method is often used without any parameters, the optional parameters are required to run an existing calculation or to retrieve the final output, however also consider using InitializeCalc. The inputRtf is required if you want to calculate an existing RTF source, for example if you wanted to integrate Tedds with an alternative word processor you would get the RTF from the document and pass it as this parameter so that Tedds would calculate the expressions in the RTF. The inputRtf is also required if you want to obtain output RTF once calculating is finished. Output is obtained by using the GetOutput method but if no inputRtf has been specified to the Initialize function (or the InitializeCalc function) then GetOutput will not return any output. If the calculator has been used to perform a calculation and you want to save the variables so that they can be used for re-calculating then you can use the optional variables parameter to initialise the calculator with the previously saved variables. The variables from a previous calculation are retrieved by using the GetVariables function.

**Parameters:**
- `inputRtf` (string) — Optional input formatted in Microsoft Rich Text Format(RTF) which will be calculated
- `variables` (string) — Xml string of the variables to initialize the calculator with, usually created by GetVariables
- `sectionName` (string) — Calc Section name
- `sectionId` (int) — Calc Section unique numeric identifier
- `pageWidth` (int) — Output page width in twips, default is 180 twips
- `pageHeight` (int) — Output page height in twips, default is 260 twips

[Docs](https://developer.tekla.com/topic/en/20/45/3ab2a6a338f894c330b63592f689966056f69beb09047175f6e40986902c5841)

#### `void InitializeCalc(string calcFileName, string calcItemName, string variables, string sectionName, int sectionId = 0, int pageWidth = 180, int pageHeight = 260)`

Initializes the calculator and starts calculating using the specified Calc Library Item

**Remarks:** The fileName and itemName are required and determine the Calc Item in the specified Calc Library which will be calculated. If you want to retrieve the document output when calculating is finished the calc item you calculate must be in the RTF format. Output is obtained by using the GetOutput method. If the calculator has been used to perform a calculation and you want to save the variables so that they can be used at a later date for re-calculating then you can use the optional variables parameter to initialise the calculator with the previously saved variables.The variables from a previous calculation are retrieved by using the GetVariables method.

**Parameters:**
- `calcFileName` (string) — File name of the Calc Library file which contains the calc item to calculate
- `calcItemName` (string) — Item name of the Calc Item to calculate
- `variables` (string) — Xml string of the variables to initialize the calculator instance with
- `sectionName` (string) — Initial section name
- `sectionId` (int) — Initial section identifier
- `pageWidth` (int) — Output page width in twips, default is 180 twips
- `pageHeight` (int) — Output page height in twips, default is 260 twips

[Docs](https://developer.tekla.com/topic/en/20/45/4ea1d7cff39c130aecc6be90103a3e4900815e9b473cb3713fbd77ef9c115700)

#### `void SetForegroundWindow(int WindowHandle)`

Executes the Windows API function SetForegroundWindow from the Tedds calculator process.

**Parameters:**
- `WindowHandle` (int) — Handle of window to make the foreground window

[Docs](https://developer.tekla.com/topic/en/20/45/a4d66c046b6b69511f04dc3a35fe80aacc19569cf0fe72682802b18bb1aa82aa)

#### `void SetOwnerWindow(int ownerWindowHandle)`

Sets which window owns the Tedds calculator's main window

**Remarks:** The main Tedds calculator window is always hidden but acts as the parent window for all other windows displayed by the Tedds calculator. The window handle may be required to ensure that the Calculator process is brought to the foreground when the calling processes expects the Tedds calculator to show a user interface.

**Parameters:**
- `ownerWindowHandle` (int) — Window handle of the window which will be the owner of the Tedds calculator's main window

[Docs](https://developer.tekla.com/topic/en/20/45/00b41f48802759a1cb72ea566e7961c29ab524ee3df4237a2b1d6a631d40cca0)

#### `void SetVariables(string variables, VariableFormat format = VariableFormat.Xml)`

Assign the specified variables to the calculator instance

**Remarks:** The Calculator currently supports one format for defining variables which uses XML. The XML schema is not documented and is subject to change, however the Tedds Calculator itself can be used to generate the required XML. To create the XML string create an instance of the XML calculator and initialize the calculator using Initialize, omitting all of the optional parameters. For each variable you wish to assign in the XML use Functions.SetVar or Functions.SetVarExpr. Once all the required variables have been assigned use @ref GetVariables to retrieve an XML string of all the variables which have been defined. This XML string can then be used with SetVariables although normally the variables would be initialized using one of the Initialize or InitializeCalc functions.

**Parameters:**
- `variables` (string) — Variables and their associated values and attributes in the specified data format
- `format` (VariableFormat) — Format of the variables, the default is XML

[Docs](https://developer.tekla.com/topic/en/20/45/174504dc6f5330c8dce44c982607f92bea731c0b66eb45a7205c08bf21173782)

### Properties
- `CalculatingErrorEvents` (bool, get/set) — Determines whether error events are fired to the event listeners
- `CalculatingProgressEvents` (bool, get/set) — Determines whether progress events are fired to the event listeners
- `Functions` (IFunctions, get) — Returns an object supporting the IFunctions interface which is used to access the internal Tedds calculator functions
- `Status` (CalcStatus, get) — Returns the status of the Tedds calculator when the last evaluation finished.
- `UndefinedVariableEvents` (bool, get/set) — Determines whether undefined variable events are fired to the event listeners
- `VersionLibraryMajor` (int, get) — Returns the major version number of the Tedds Engineering Library
- `VersionLibraryMinor` (int, get) — Returns the minor version number of the Tedds Engineering Library
- `VersionMajor` (int, get) — Returns the major version number of the Tedds software
- `VersionMinor` (int, get) — Returns the minor version number of the Tedds software
- `WindowHandle` (int, get) — Returns the Tedds calculator's main window handle

## ICalculatorExEvents (interface)

Interface for Calculator events.

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-interopassemblies-teddscalc-icalculatorexevents-41853)

### Methods
#### `void CalculatingError(CalcErrorType errorType, uint errorCode, string context, string message, string expression, uint options, ref CalcStatus status)`

Calculating error event

**Parameters:**
- `errorType` (CalcErrorType) — Type of error
- `errorCode` (uint) — Error code identifier
- `context` (string) — Context of error
- `message` (string) — Error message
- `expression` (string) — Expression which caused error
- `options` (uint) — Error handler option flags
- `status` (CalcStatus) — Calculation status, to stop calculation process return Aborted or Interrupted

[Docs](https://developer.tekla.com/topic/en/20/45/c4360bc2c3c5e2e9d0c653895e0e3fdacea13c19280b1fa12cefa7a9e0395dba)

#### `void CalculatingProgress(CalcProgressEvent progressEvent, uint progressValue, string text, ref CalcStatus status)`

Calculating progress event

**Parameters:**
- `progressEvent` (CalcProgressEvent) — Progress event type
- `progressValue` (uint) — Progress value for the current event
- `text` (string) — Text for the current event
- `status` (CalcStatus) — Calculation status after the event has been processed, the caller can change the state to stop the calculation process

[Docs](https://developer.tekla.com/topic/en/20/45/d38ce4e7579dc46cf0472a9676b6bed654649e1707ef6ab3b5d986e6da89700f)

#### `void UndefinedVariable(string variableName, out string value)`

Undefined variable event

**Remarks:** The undefined variable event can be used to provide a hook into the data model or API of another software system. If a variable is referred to in your calculations which doesn't exist then your event handler will be called and you have the opportunity to provide a value for the referenced name. Note that once a value has been provided the variable will then exist in Tedds so the undefined event will not fire again for that variable unless it is explicitly deleted.

**Parameters:**
- `variableName` (string) — Name of undefined variable
- `value` (string) — Value to be assigned to variable

[Docs](https://developer.tekla.com/topic/en/20/45/f29fc3008f656adc287c7f03ed166ee3b54ecc5de60323d34228921aed6cd5b8)

#### `void UndefinedVariable2(string variableName, out string value, out string attributes)`

Undefined variable event

**Remarks:** The undefined variable event can be used to provide a hook into the data model or API of another software system. If a variable is referred to in your calculations which doesn't exist then your event handler will be called and you have the opportunity to provide a value for the referenced name. Note that once a value has been provided the variable will then exist in Tedds so the undefined event will not fire again for that variable unless it is explicitly deleted.

**Parameters:**
- `variableName` (string) — Name of undefined variable
- `value` (string) — Value to be assigned to variable
- `attributes` (string) — Attributes to be assigned to variable

[Docs](https://developer.tekla.com/topic/en/20/45/bfd3e49e17020062b9d18a5b43246ba9381130d53f97d6f94fe7d2190dbff63a)

## IFunctions (interface)

Interface for executing calculation commands

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-interopassemblies-teddscalc-ifunctions-41854)

### Methods
#### `int AddSection()`

Add a new variable section using the Tedds 'AddSection' function. For information on Tedds calculation functions, use Tedds for Word and refer to the Tedds engineering library index at "Writing your own custom calculations\Calculation writing documentation\Tedds functions"

**Returns:** `int` — Returns the unique identifier of the new variable section

[Docs](https://developer.tekla.com/topic/en/20/45/cf6f8832e7b6969398c9ceea55831ef219bfb40e6df7f228337dc225c5b6e61b)

#### `ICalcValue Eval(string Expression)`

Returns the result of calculating an expression using the Tedds 'Eval' function. For information on Tedds calculation functions, use Tedds for Word and refer to the Tedds engineering library index at "Writing your own custom calculations\Calculation writing documentation\Tedds functions"

**Parameters:**
- `Expression` (string) — Expression to calculate

**Returns:** `ICalcValue` — Result of expression

[Docs](https://developer.tekla.com/topic/en/20/45/f12c48fb7e6b02853b7fd3e115ef93ba3d6485cbbde1c3e4caa508c64c053fc4)

#### `ICalcValue GetVar(string name)`

Returns the value of a named variable using the Tedds 'GetVar' function

**Parameters:**
- `name` (string) — Name of the variable

**Returns:** `ICalcValue` — Variables value

[Docs](https://developer.tekla.com/topic/en/20/45/06724dd15ce9e7ecae0e434ae49c424386e8db35cf88628ed181367f1568c5c5)

#### `bool SetSection(int sectionId, string name)`

Set the active variable section using the Tedds 'SetSection' function. For information on Tedds calculation functions, use Tedds for Word and refer to the Tedds engineering library index at "Writing your own custom calculations\Calculation writing documentation\Tedds functions"

**Parameters:**
- `sectionId` (int) — Unique numeric identifier of the section which is to become the active section
- `name` (string) — Display name of the section

**Returns:** `bool` — true if successful; otherwise false

[Docs](https://developer.tekla.com/topic/en/20/45/4f0bba553d273c2ad4ca659bda6b5a4d2e56cbea2d65e54802a66a82cef76a59)

#### `ICalcValue SetVar(string name, object value, string units = "", string attributes = "")`

Assigns a value to the named variable using the Tedds 'SetVar' function. For information on Tedds calculation functions, use Tedds for Word and refer to the Tedds engineering library index at "Writing your own custom calculations\Calculation writing documentation\Tedds functions"

**Parameters:**
- `name` (string) — Name of the variable
- `value` (object) — Value to assign
- `units` (string) — Units of the assigned value, only applicable if the value is numeric.
- `attributes` (string) — Variable attribute flags (h = hidden, t = temporary, e = expression, r = readonly)

**Returns:** `ICalcValue` — Actual value of variable

[Docs](https://developer.tekla.com/topic/en/20/45/355af9947c940067d8dec66613858a8b7a2e3e3a8413f309dad0abaf4fe6e968)

#### `ICalcValue SetVarExpr(string name, string value, string attributes = "e")`

Assigns an expression value to the named variable using the Tedds 'SetVar' function. For information on Tedds calculation functions, use Tedds for Word and refer to the Tedds engineering library index at "Writing your own custom calculations\Calculation writing documentation\Tedds functions"

**Parameters:**
- `name` (string) — Name of the variable
- `value` (string) — Expression to assign
- `attributes` (string) — attributes Variable attribute flags (h = hidden, t = temporary, e = expression, r = readonly)

**Returns:** `ICalcValue` — numerical error code

[Docs](https://developer.tekla.com/topic/en/20/45/59fc94f41fcb37cd176f6ba197b309e77d3c9589bca3de51a838caf6d37a9cb3)

#### `bool VarExists(string name)`

Returns true if the named variable already exists using the Tedds 'VarExists' function. For information on Tedds calculation functions, use Tedds for Word and refer to the Tedds engineering library index at "Writing your own custom calculations\Calculation writing documentation\Tedds functions"

**Parameters:**
- `name` (string) — Name of the variable

**Returns:** `bool` — true if the variable already exists; otherwise false

[Docs](https://developer.tekla.com/topic/en/20/45/9ec27b9571900c755ad66a85fd2f3330299740800e8ac6c407eb68205c40f53e)

## OutputFormat (enum)

Document output formats

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-interopassemblies-teddscalc-outputformat-41855)

### Values
- `Pdf` = `2` — Adobe Portable Document Format
- `Rtf` = `1` — Microsoft Rich Text Format

## VariableFormat (enum)

Data formats for variables, currently only XML format is supported

[Vendor docs](https://developer.tekla.com/doc/tekla-tedds/2025/tekla-structural-interopassemblies-teddscalc-variableformat-41856)

### Values
- `Xml` = `1` — XML format

