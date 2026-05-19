---
name: allplan-nemall_python_ifw_input
description: This skill encodes the allplan 2024.0 surface of the NemAll_Python_IFW_Input namespace — 36 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AddMsgInfo, BuildingElementInputControls, CoordinateInput, CNOI_DocumentWrapper, CoordinateInputMode, CoordinateInputResult, ElementHandleType, ElementSelect, and 28 more types.
---

# NemAll_Python_IFW_Input

Auto-generated from vendor docs for allplan 2024.0. 36 types in this namespace.

## AddMsgInfo (class)

(No description provided in vendor docs for NemAll_Python_IFW_Input.AddMsgInfo.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/AddMsgInfo/)

### Constructors
- `AddMsgInfo()` — Initialize

## BuildingElementInputControls (class)

Implementation of the building element input controls

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/BuildingElementInputControls/)

### Constructors
- `BuildingElementInputControls()` — Initialize
- `BuildingElementInputControls(element)` — Copy constructor

### Methods
#### `CloseControls()`

Close the input controls

**Remarks:** Close the input controls

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/BuildingElementInputControls/#NemAll_Python_IFW_Input.BuildingElementInputControls.CloseControls)

#### `CreateControls(handlePropList, insertionMat, viewProj, bUpdateControls, assoRefObj)`

Create the controls

**Remarks:** Create the controls

**Parameters:**
- `handlePropList` (object) — List with the handle properties
- `insertionMat` (Matrix3D) — Transformation matrix
- `viewProj` (ViewWorldProjection) — View world projection
- `bUpdateControls` (bool) — Update the controls: true/false
- `assoRefObj` (object) — Reference element for the drawing inside the associative views

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/BuildingElementInputControls/#NemAll_Python_IFW_Input.BuildingElementInputControls.CreateControls)

## CNOI_DocumentWrapper (class)

(No description provided in vendor docs for NemAll_Python_IFW_Input.CNOI_DocumentWrapper.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CNOI_DocumentWrapper/)

### Constructors
- `CNOI_DocumentWrapper()` — Initialize

## CoordinateInput (class)

(No description provided in vendor docs for NemAll_Python_IFW_Input.CoordinateInput.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/)

### Constructors
- `CoordinateInput()` — Initialize

### Methods
#### `AddGeometryFromPreviewElements(arg2)`

Add the geometry elements from the preview elements for the point and element search

**Remarks:** Add the geometry elements from the preview elements for the point and element search

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.AddGeometryFromPreviewElements)

#### `CancelInput()`

Explicit cancel of the input function

**Remarks:** Explicit cancel of the input function

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.CancelInput)

#### `GetAssocViewFromPoint(arg2, arg3, arg4)`



[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.GetAssocViewFromPoint)

#### `GetCurrentPoint()`

Get and mark the current input point

**Remarks:** Get and mark the current input point

**Returns:** `CoordinateInputResult` — Current input point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.GetCurrentPoint)

#### `GetCurrentPoint(bStartPnt)`

Get and mark the current input point End point input is possible by a distance input to the input point of the last input step

**Remarks:** Get and mark the current input point End point input is possible by a distance input to the input point of the last input step

**Parameters:**
- `bStartPnt` (bool) — Starting point is active true/false.
- `End` (object) — point input is possible by a distance input to the input point of the last input step

**Returns:** `CoordinateInputResult` — Current input point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.GetCurrentPoint)

#### `GetCurrentPoint(startPnt)`

Get and mark the current input point

**Remarks:** Get and mark the current input point

**Parameters:**
- `startPnt` (Point3D) — Starting point End point input is possible by a distance input to the start point

**Returns:** `CoordinateInputResult` — Current input point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.GetCurrentPoint)

#### `GetCurrentPoint(startPnt, bStartPnt)`

Get and mark the current input point

**Remarks:** Get and mark the current input point

**Parameters:**
- `startPnt` (Point3D) — Starting point
- `bStartPnt` (bool) — Starting point is active End point input is possible by a distance input to the start point

**Returns:** `CoordinateInputResult` — Current input point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.GetCurrentPoint)

#### `GetInputControlIntValue()`

Get the integer value from the value input control

**Remarks:** Get the integer value from the value input control

**Returns:** `int` — Integer value from the value input control

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.GetInputControlIntValue)

#### `GetInputControlValue()`

Get the double value from the value input control

**Remarks:** Get the double value from the value input control

**Returns:** `float` — Double value from the value input control

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.GetInputControlValue)

#### `GetInputPoint(mouseMsg, pnt, pMsgInfo)`

Get the current input point

**Remarks:** Get the current input point

**Parameters:**
- `mouseMsg` (int) — Mouse message WM_xxx
- `pnt` (Point2D) — Cursor point (view coordinate)
- `pMsgInfo` (AddMsgInfo) — Additional message info

**Returns:** `CoordinateInputResult` — Current input point result

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.GetInputPoint)

#### `GetInputPoint(mouseMsg, pnt, pMsgInfo, bStartPnt)`

Get the current input point End point input is possible by a distance input to the input point of the last input step

**Remarks:** Get the current input point End point input is possible by a distance input to the input point of the last input step

**Parameters:**
- `mouseMsg` (int) — Mouse message WM_xxx
- `pnt` (Point2D) — Cursor point (view coordinate)
- `pMsgInfo` (AddMsgInfo) — Additional message info
- `bStartPnt` (bool) — Starting point is active End point input is possible by a distance input to the input point of the last input step

**Returns:** `CoordinateInputResult` — Current input point result

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.GetInputPoint)

#### `GetInputPoint(mouseMsg, pnt, pMsgInfo, startPnt, bStartPnt)`

Get the current input point End point input is possible by a distance input to the start point

**Remarks:** Get the current input point End point input is possible by a distance input to the start point

**Parameters:**
- `mouseMsg` (int) — Mouse message WM_xxx
- `pnt` (Point2D) — Cursor point (view coordinate)
- `pMsgInfo` (AddMsgInfo) — Additional message info
- `startPnt` (Point3D) — Starting point
- `bStartPnt` (bool) — Starting point is active End point input is possible by a distance input to the start point

**Returns:** `CoordinateInputResult` — Current input point result

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.GetInputPoint)

#### `GetSelectedElement()`

Get the selected element The function can be used in case of eIdentMode = MODE_TEXTPOINT, SelectGeometryElement, SelectElement, ...

**Remarks:** Get the selected element The function can be used in case of eIdentMode = MODE_TEXTPOINT, SelectGeometryElement, SelectElement, ...

**Returns:** `BaseElementAdapter` — Selected element

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.GetSelectedElement)

#### `GetSelectedGeometryElement()`



[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.GetSelectedGeometryElement)

#### `InitFirstElementInput(text, identMode=CoordinateInputMode)`

Initialize the coordinate input for an element (point, line, ...) input as free point input. The coordinate input toolbar shows only the request string. The input is allowed in each document.

**Remarks:** Initialize the coordinate input for an element (point, line, ...) input as free point input. The coordinate input toolbar shows only the request string. The input is allowed in each document.

**Parameters:**
- `text` (InputStringConvert) — Request string as resource ID, CAllstring, TCHAR or CString
- `identMode` (CoordinateInputMode) — Identification mode

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.InitFirstElementInput)

#### `InitFirstElementValueInput(text, ctrlData, identMode=CoordinateInputMode)`

Initialize the coordinate input for an element (point, line, ...) input as free point input. The coordinate input toolbar shows only the request string and the value input controls. The input is allowed only in the document from the last input

**Remarks:** Initialize the coordinate input for an element (point, line, ...) input as free point input. The coordinate input toolbar shows only the request string and the value input controls. The input is allowed only in the document from the last input

**Parameters:**
- `text` (InputStringConvert) — Request string as resource ID, CAllstring, TCHAR or CString
- `ctrlData` (ValueInputControlData) — Input control data as eValueInputControlType or ValueInputControlData object
- `identMode` (CoordinateInputMode) — Identification mode

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.InitFirstElementValueInput)

#### `InitFirstPointInput(text, identMode=CoordinateInputMode, inputViewData=InputViewData)`

Initialize the coordinate input for a first point input. The input is allowed in each document Sample: InitFirstPointInput(IDC_FIRST_POINT); InitFirstPointInput(CAllString(32128));

**Remarks:** Initialize the coordinate input for a first point input. The input is allowed in each document Sample: InitFirstPointInput(IDC_FIRST_POINT); InitFirstPointInput(CAllString(32128));

**Parameters:**
- `text` (InputStringConvert) — Request string as resource ID, CAllstring, TCHAR or CString
- `identMode` (CoordinateInputMode) — Identification mode

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.InitFirstPointInput)

#### `InitFirstPointValueInput(text, ctrlData, identMode=CoordinateInputMode)`

Initialize the coordinate input for a first point and value input. The input is allowed in each document Sample: InitFirstPointValueInput(IDC_FIRST_POINT,ValueInputControlData::COORDINATE_EDIT); InitFirstPointValueInput(CAllString(32128),ValueInputControlData(ValueInputControlData::DOUBLE_EDIT,20.,0.,1000.));

**Remarks:** Initialize the coordinate input for a first point and value input. The input is allowed in each document Sample: InitFirstPointValueInput(IDC_FIRST_POINT,ValueInputControlData::COORDINATE_EDIT); InitFirstPointValueInput(CAllString(32128),ValueInputControlData(ValueInputControlData::DOUBLE_EDIT,20.,0.,1000.));

**Parameters:**
- `text` (InputStringConvert) — Request string as resource ID, CAllstring, TCHAR or CString
- `ctrlData` (ValueInputControlData) — Input control data as eValueInputControlType or ValueInputControlData object
- `identMode` (CoordinateInputMode) — Identification mode

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.InitFirstPointValueInput)

#### `InitNextElementInput(text, identMode=CoordinateInputMode)`

Initialize the coordinate input for an element (point, line, ...) input as free point input. The coordinate input toolbar shows only the request string. The input is allowed only in the document from the last input

**Remarks:** Initialize the coordinate input for an element (point, line, ...) input as free point input. The coordinate input toolbar shows only the request string. The input is allowed only in the document from the last input

**Parameters:**
- `text` (InputStringConvert) — Request string as resource ID, CAllstring, TCHAR or CString
- `identMode` (CoordinateInputMode) — Identification mode

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.InitNextElementInput)

#### `InitNextElementValueInput(text, ctrlData, identMode=CoordinateInputMode)`

Initialize the coordinate input for an element (point, line, ...) input as free point input. The coordinate input toolbar shows only the request string and the value input controls. The input is allowed only in the document from the last input

**Remarks:** Initialize the coordinate input for an element (point, line, ...) input as free point input. The coordinate input toolbar shows only the request string and the value input controls. The input is allowed only in the document from the last input

**Parameters:**
- `text` (InputStringConvert) — Request string as resource ID, CAllstring, TCHAR or CString
- `ctrlData` (ValueInputControlData) — Input control data as eValueInputControlType or ValueInputControlData object
- `identMode` (CoordinateInputMode) — Identification mode

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.InitNextElementValueInput)

#### `InitNextPointInput(text, identMode=CoordinateInputMode)`

Initialize the coordinate input for a next point input. The input is allowed only in the document from the last input

**Remarks:** Initialize the coordinate input for a next point input. The input is allowed only in the document from the last input

**Parameters:**
- `text` (InputStringConvert) — Request string as resource ID, CAllstring, TCHAR or CString
- `identMode` (CoordinateInputMode) — Identification mode

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.InitNextPointInput)

#### `InitNextPointValueInput(text, ctrlData, identMode=CoordinateInputMode)`

Initialize the coordinate input for a first point and value input The input is allowed only in the document from the last input

**Remarks:** Initialize the coordinate input for a first point and value input The input is allowed only in the document from the last input

**Parameters:**
- `text` (InputStringConvert) — Request string as resource ID, CAllstring, TCHAR or CString
- `ctrlData` (ValueInputControlData) — Input control data as eValueInputControlType or ValueInputControlData object
- `identMode` (CoordinateInputMode) — Identification mode

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.InitNextPointValueInput)

#### `InitValueInput(text, ctrlData)`

Initialize the value input

**Remarks:** Initialize the value input

**Parameters:**
- `text` (InputStringConvert) — Request string as resource ID, CAllstring, TCHAR or CString
- `ctrlData` (ValueInputControlData) — Input control data as eValueInputControlType or ValueInputControlData object

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.InitValueInput)

#### `IsEmptyValueInputControl()`

Check, whether there is no input inside the input control

**Remarks:** Check, whether there is no input inside the input control

**Returns:** `bool` — Input control is empty: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.IsEmptyValueInputControl)

#### `IsMouseMove(mouseMsg)`

Check on mouse move

**Remarks:** Check on mouse move

**Parameters:**
- `mouseMsg` (int) — Mouse message WM_xxx

**Returns:** `bool` — Mouse move: true false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.IsMouseMove)

#### `IsValueInputControl()`

Check, whether a value input control exists

**Remarks:** Check, whether a value input control exists

**Returns:** `bool` — A value input control exists: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.IsValueInputControl)

#### `IsValueInputControl(id)`

Check, whether the ID belongs to the value input control (from the coordinate input dialog)

**Remarks:** Check, whether the ID belongs to the value input control (from the coordinate input dialog)

**Parameters:**
- `id` (int) — ID to check

**Returns:** `bool` — ID belongs to the value input control (from the coordinate input dialog): true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.IsValueInputControl)

#### `IsValueInputControlInput(bIdentPoint)`

Check, whether an input inside the value input control is done and the value should be used

**Remarks:** Check, whether an input inside the value input control is done and the value should be used

**Parameters:**
- `bIdentPoint` (bool) — Identification point has higher priority: true/false

**Returns:** `bool` — Check, whether the input value inside the added control

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.IsValueInputControlInput)

#### `SelectElement(mouseMsg, pnt, pMsgInfo, bHighlight, bSelAlways, bAllowCenter)`

Select an element if no identification point exists. Use the filter set by SetElementFilter

**Remarks:** Select an element if no identification point exists. Use the filter set by SetElementFilter

**Parameters:**
- `mouseMsg` (int) — Mouse message WM_xxx
- `pnt` (Point2D) — Cursor point (view coordinate)
- `pMsgInfo` (AddMsgInfo) — Additional message info
- `bHighlight` (bool) — Highlight the selected element
- `bSelAlways` (bool) — true: Select always
- `bAllowCenter` (bool) — Allow element identification by center point

**Returns:** `bool` — Element is selected: true false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.SelectElement)

#### `SelectElement(mouseMsg, pnt, pMsgInfo, bHighlight, bSelAlways, bAllowCenter, selectSetting)`

Select an element if no identification point exists. Use the filter set by SetElementFilter

**Remarks:** Select an element if no identification point exists. Use the filter set by SetElementFilter

**Parameters:**
- `mouseMsg` (int) — Mouse message WM_xxx
- `pnt` (Point2D) — Cursor point (view coordinate)
- `pMsgInfo` (AddMsgInfo) — Additional message info
- `bHighlight` (bool) — Highlight the selected element
- `bSelAlways` (bool) — true: Select always
- `bAllowCenter` (bool) — Allow element identification by center point
- `selectSetting` (ElementSelectFilterSetting) — Element selection filter

**Returns:** `bool` — Element is selected: true false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.SelectElement)

#### `SelectGeometryElement(mouseMsg, pnt, pMsgInfo, bHighlightCompleteElement)`

Select a base geometry element. Use the filter set by SetGeometryElementFilter and SetGeometryFilter

**Remarks:** Select a base geometry element. Use the filter set by SetGeometryElementFilter and SetGeometryFilter

**Parameters:**
- `mouseMsg` (int) — Mouse message WM_xxx
- `pnt` (Point2D) — Cursor point (view coordinate)
- `pMsgInfo` (AddMsgInfo) — Additional message info
- `bHighlightCompleteElement` (bool) — true = highlight the complete element, false = highlight only the selected geometry part

**Returns:** `bool` — Element was found: true false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.SelectGeometryElement)

#### `SelectPolyhedronFace(arg2, arg3, arg4)`

.. deprecated:: since Allplan 2023-1-0 use FaceSelectService::SelectPolyhedronFace

**Remarks:** .. deprecated:: since Allplan 2023-1-0 use FaceSelectService::SelectPolyhedronFace

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.SelectPolyhedronFace)

#### `SelectWallFace(arg2, arg3, arg4)`

.. deprecated:: since Allplan 2023-1-0 use FaceSelectService::SelectWallFace

**Remarks:** .. deprecated:: since Allplan 2023-1-0 use FaceSelectService::SelectWallFace

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.SelectWallFace)

#### `SelectWallFaceInUVS(arg2, arg3, arg4)`

.. deprecated:: since Allplan 2023-1-0 use FaceSelectService::SelectWallFaceInUVS

**Remarks:** .. deprecated:: since Allplan 2023-1-0 use FaceSelectService::SelectWallFaceInUVS

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.SelectWallFaceInUVS)

#### `SetAbscissaLine(arg2, arg3)`



[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.SetAbscissaLine)

#### `SetGeometryFilter(geoFilter)`

Set the geometry element selection filter

**Remarks:** Set the geometry element selection filter

**Parameters:**
- `geoFilter` (SnoopElementGeometryFilter) — Geometry element filter

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.SetGeometryFilter)

#### `SetInputPlane(plane)`

Set the input plane The input point will be transformed to the input plane: true/false

**Remarks:** Set the input plane The input point will be transformed to the input plane: true/false

**Parameters:**
- `plane` (Plane3D) — Input plane

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.SetInputPlane)

#### `SetInputText(text)`

Set the input text

**Remarks:** Set the input text

**Parameters:**
- `text` (InputStringConvert) — Request string as resource ID, CAllstring, TCHAR or CString

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.SetInputText)

#### `SetProjectionBase0(setProjectionBase0)`

Set the projection base of the coordinate selection

**Remarks:** Set the projection base of the coordinate selection

**Parameters:**
- `wallsetProjectionBase0` (object) — True: the input plane normal coordinate is set to 0
- `False` (object) — : the input plane normal coordinate is used from the selected point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.SetProjectionBase0)

## CoordinateInputMode (class)

(No description provided in vendor docs for NemAll_Python_IFW_Input.CoordinateInputMode.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInputMode/)

### Constructors
- `CoordinateInputMode()` — Initialize
- `CoordinateInputMode(identMode, drawPointSymbol)` — Constructor, sets the local coordinate input to false

## CoordinateInputResult (class)

(No description provided in vendor docs for NemAll_Python_IFW_Input.CoordinateInputResult.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInputResult/)

### Constructors
- `CoordinateInputResult()` — Initialize

### Methods
#### `GetPoint()`



[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInputResult/#NemAll_Python_IFW_Input.CoordinateInputResult.GetPoint)

## ElementHandleType (class)

Element handle type

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementHandleType/)

### Constructors
- `ElementHandleType()` — Initialize

### Methods
#### `__repr__()`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementHandleType/#NemAll_Python_IFW_Input.ElementHandleType.__repr__)

## ElementSelect (class)

(No description provided in vendor docs for NemAll_Python_IFW_Input.ElementSelect.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelect/)

### Constructors
- `ElementSelect()` — Initialize

### Methods
#### `InitSelection(arg2)`



[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelect/#NemAll_Python_IFW_Input.ElementSelect.InitSelection)

#### `IsMouseMove(arg2)`



[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelect/#NemAll_Python_IFW_Input.ElementSelect.IsMouseMove)

## ElementSelectFilterSetting (class)

Settings class to control the object selection

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelectFilterSetting/)

### Constructors
- `ElementSelectFilterSetting()` — Initialize
- `ElementSelectFilterSetting(bSnoopAllElements)` — Constructor default values: Query is empty
- `ElementSelectFilterSetting(filter, bSnoopAllElements)` — Constructor default value for eObjectSelectType == SEL_ALL
- `ElementSelectFilterSetting(filter, documentSnoopType, layerSnoopType)` — Constructor default value for eObjectSelectType == SEL_ALL
- `ElementSelectFilterSetting(selectSetting)` — Copy Constructor

### Methods
#### `Clear()`

Reset the settings

**Remarks:** Reset the settings

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelectFilterSetting/#NemAll_Python_IFW_Input.ElementSelectFilterSetting.Clear)

#### `GetLayerSelectType()`

Get the layer selection type

**Remarks:** Get the layer selection type

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelectFilterSetting/#NemAll_Python_IFW_Input.ElementSelectFilterSetting.GetLayerSelectType)

#### `IsBaseClassType(typeID)`

Check, whether the element type is a base class type

**Remarks:** Check, whether the element type is a base class type

**Parameters:**
- `typeID` (GUID) — Element type ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelectFilterSetting/#NemAll_Python_IFW_Input.ElementSelectFilterSetting.IsBaseClassType)

#### `IsClear()`

Get the clear state

**Remarks:** Get the clear state

**Returns:** `bool` — true, if the members contain default values

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelectFilterSetting/#NemAll_Python_IFW_Input.ElementSelectFilterSetting.IsClear)

#### `IsPointSelect()`

Get the point select state

**Remarks:** Get the point select state

**Returns:** `bool` — Point selection is active: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelectFilterSetting/#NemAll_Python_IFW_Input.ElementSelectFilterSetting.IsPointSelect)

#### `IsSelectPassiveInfoElement()`

Get the selection state of a passive info element

**Remarks:** Get the selection state of a passive info element

**Returns:** `bool` — Allow to select passive info element if no active element was found: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelectFilterSetting/#NemAll_Python_IFW_Input.ElementSelectFilterSetting.IsSelectPassiveInfoElement)

#### `SelectPassiveInfoElement()`

Allow to select passive info element if no active element was found

**Remarks:** Allow to select passive info element if no active element was found

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelectFilterSetting/#NemAll_Python_IFW_Input.ElementSelectFilterSetting.SelectPassiveInfoElement)

#### `SetArchitectureFilterQuery(positive=True)`

Set a prefabricated filter for all architecture elements

**Remarks:** Set a prefabricated filter for all architecture elements

**Parameters:**
- `positive` (bool) — if the filter will true for architecture elements otherwise the filter will be false for architecture elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelectFilterSetting/#NemAll_Python_IFW_Input.ElementSelectFilterSetting.SetArchitectureFilterQuery)

#### `SetAssoFilterQuery(positive=True)`

Set a prefabricated filter for all associative view elements

**Remarks:** Set a prefabricated filter for all associative view elements

**Parameters:**
- `positive` (bool) — if the filter will true for associative view elements otherwise the filter will be false for associative view elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelectFilterSetting/#NemAll_Python_IFW_Input.ElementSelectFilterSetting.SetAssoFilterQuery)

#### `SetDocumentLayerFilter(bSnoopAllElements)`

Set the document and layer filter

**Remarks:** Set the document and layer filter

**Parameters:**
- `bSnoopAllElements` (bool) — Snoop all elements: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelectFilterSetting/#NemAll_Python_IFW_Input.ElementSelectFilterSetting.SetDocumentLayerFilter)

#### `SetDocumentSelectType(documentSnoopType)`

Set the selection mode for the document (active, passive or all documents)

**Remarks:** Set the selection mode for the document (active, passive or all documents)

**Parameters:**
- `documentSnoopType` (eDocumentSnoopType) — Selection mode for the document (active, passive or all documents)

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelectFilterSetting/#NemAll_Python_IFW_Input.ElementSelectFilterSetting.SetDocumentSelectType)

#### `SetLayerSelectType(layerSnoopType)`

Set the layer selection type

**Remarks:** Set the layer selection type

**Parameters:**
- `layerSnoopType` (eLayerSnoopType) — Type of the allowed layers for the selection

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelectFilterSetting/#NemAll_Python_IFW_Input.ElementSelectFilterSetting.SetLayerSelectType)

#### `SetPointSelect()`

Set the point select state

**Remarks:** Set the point select state

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelectFilterSetting/#NemAll_Python_IFW_Input.ElementSelectFilterSetting.SetPointSelect)

### Properties
- `LayerSelectType` (eLayerSnoopType, get/set) — Get the layer selection type

## HandleService (class)

(No description provided in vendor docs for NemAll_Python_IFW_Input.HandleService.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/HandleService/)

### Constructors
- `HandleService()` — Initialize

### Methods
#### `AddHandles(doc, handlePropList, insertionMat, assoRefObj)`

Add the handles

**Remarks:** Add the handles

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `handlePropList` (object) — Handle properties list
- `insertionMat` (Matrix3D) — Transformation matrix
- `assoRefObj` (object) — Reference element for the drawing inside the associative views

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/HandleService/#NemAll_Python_IFW_Input.HandleService.AddHandles)

#### `DeleteToolTipText()`

Delete the tool tip text

**Remarks:** Delete the tool tip text

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/HandleService/#NemAll_Python_IFW_Input.HandleService.DeleteToolTipText)

#### `DrawHandles()`

Draw the handles

**Remarks:** Draw the handles

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/HandleService/#NemAll_Python_IFW_Input.HandleService.DrawHandles)

#### `RemoveHandles()`

Remove the handles

**Remarks:** Remove the handles

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/HandleService/#NemAll_Python_IFW_Input.HandleService.RemoveHandles)

#### `SelectHandle(pnt, viewProj)`

Select a handle

**Remarks:** Select a handle

**Parameters:**
- `pnt` (Point2D) — Cursor point
- `viewProj` (ViewWorldProjection) — View world projection

**Returns:** `tuple` — Handle index (-1 = no selection) , world to associative view matrix

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/HandleService/#NemAll_Python_IFW_Input.HandleService.SelectHandle)

#### `ShowToolTipText(text)`

Show the tool tip text

**Remarks:** Show the tool tip text

**Parameters:**
- `text` (str) — Text

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/HandleService/#NemAll_Python_IFW_Input.HandleService.ShowToolTipText)

## HighlightService (class)

(No description provided in vendor docs for NemAll_Python_IFW_Input.HighlightService.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/HighlightService/)

### Methods
#### `CancelAllHighlightedElements(documentID)`

Cancel the highlight of all elements

**Remarks:** Cancel the highlight of all elements

**Parameters:**
- `documentID` (int) — document ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/HighlightService/#NemAll_Python_IFW_Input.HighlightService.CancelAllHighlightedElements)

#### `HighlightElements(eleList)`

Highlight the elements

**Remarks:** Highlight the elements

**Parameters:**
- `eleList` (BaseElementAdapterList) — Element list as BaseElementAdapterList

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/HighlightService/#NemAll_Python_IFW_Input.HighlightService.HighlightElements)

## InputFunctionStarter (class)

(No description provided in vendor docs for NemAll_Python_IFW_Input.InputFunctionStarter.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/InputFunctionStarter/)

### Methods
#### `RemoveFunction()`

Remove the current input function from the input function stack

**Remarks:** Remove the current input function from the input function stack

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/InputFunctionStarter/#NemAll_Python_IFW_Input.InputFunctionStarter.RemoveFunction)

#### `StartElementSelect(text, selectSetting, postSel, markSelectedElements, selectionMode=eSelectGeometry)`

Start the element selection A standard element selection will be started as overloaded function. The function will be removed if the selection is finished and elements are selected.

**Remarks:** Start the element selection A standard element selection will be started as overloaded function. The function will be removed if the selection is finished and elements are selected.

**Parameters:**
- `text` (str) — Request string as resource ID, CAllstring, TCHAR or CString
- `selectSetting` (ElementSelectFilterSetting) — Filter setting
- `postSel` (PostElementSelection) — Post element selection
- `markSelectedElements` (bool) — Mark the selected elements: True/False
- `selectionMode` (SelectionMode) — Selection mode

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/InputFunctionStarter/#NemAll_Python_IFW_Input.InputFunctionStarter.StartElementSelect)

## InputStringConvert (class)

(No description provided in vendor docs for NemAll_Python_IFW_Input.InputStringConvert.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/InputStringConvert/)

### Constructors
- `InputStringConvert(arg2)` — Initialize
- `InputStringConvert(arg2)` — Construct the InputStringConvert by a std::wstring

### Methods
#### `GetString()`

Get the input string

**Remarks:** Get the input string

**Returns:** `str` — Input string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/InputStringConvert/#NemAll_Python_IFW_Input.InputStringConvert.GetString)

## InputViewData (class)

(No description provided in vendor docs for NemAll_Python_IFW_Input.InputViewData.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/InputViewData/)

### Constructors
- `InputViewData()` — Initialize

## InputViewDocumentData (class)

(No description provided in vendor docs for NemAll_Python_IFW_Input.InputViewDocumentData.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/InputViewDocumentData/)

### Constructors
- `InputViewDocumentData()` — Initialize

### Methods
#### `EnableAssistWndClick(bEnable)`

Enable/disable a click inside the assist window

**Remarks:** Enable/disable a click inside the assist window

**Parameters:**
- `bEnable` (bool) — Enable a click inside the assist window: 1/0

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/InputViewDocumentData/#NemAll_Python_IFW_Input.InputViewDocumentData.EnableAssistWndClick)

#### `GetActiveViewDocument()`

Get the active view document

**Remarks:** Get the active view document

**Returns:** `DocumentAdapter` — active view document

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/InputViewDocumentData/#NemAll_Python_IFW_Input.InputViewDocumentData.GetActiveViewDocument)

#### `GetInputViewDocument()`

Get the input view document

**Remarks:** Get the input view document

**Returns:** `DocumentAdapter` — Input view document

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/InputViewDocumentData/#NemAll_Python_IFW_Input.InputViewDocumentData.GetInputViewDocument)

#### `GetInputViewDocumentID()`

Get the document ID of the current input view.

**Remarks:** Get the document ID of the current input view.

**Returns:** `int` — Document ID of the current input view

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/InputViewDocumentData/#NemAll_Python_IFW_Input.InputViewDocumentData.GetInputViewDocumentID)

#### `GetViewWorldProjection()`

Get the view-world projection object

**Remarks:** Get the view-world projection object

**Returns:** `ViewWorldProjection` — View-world projection object

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/InputViewDocumentData/#NemAll_Python_IFW_Input.InputViewDocumentData.GetViewWorldProjection)

## LCS_Flags (enum)

eStandard: standard coordinate system symbol eSmall : small coordinate system symbol eHoverX : the x arrow will be black eHoverY : the y arrow will be black eHoverZ : the z arrow will be black

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/LCS_Flags/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `LCS_Flags` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/LCS_Flags/#NemAll_Python_IFW_Input.LCS_Flags.__getitem__)

### Values
- `eHoverX` = `2`
- `eHoverY` = `4`
- `eHoverZ` = `8`
- `eSmall` = `1`
- `eStandard` = `0`

## PolygonInput (class)

Implementation of the polygon input

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PolygonInput/)

### Constructors
- `PolygonInput(coordInput, bZCoord, multiPolygon)` — Args: coordInput: Coordinate input object bZCoord: Z-coordinate input state multiPolygon: Multi polygon with openings, ...

### Methods
#### `ExecuteInput(mouseMsg, pnt, pMsgInfo)`

Execute the input

**Remarks:** Execute the input

**Parameters:**
- `mouseMsg` (int) — Mouse message
- `pnt` (Point2D) — View input point
- `pMsgInfo` (AddMsgInfo) — Additional message info

**Returns:** `int` — execution state

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PolygonInput/#NemAll_Python_IFW_Input.PolygonInput.ExecuteInput)

#### `GetPolygon()`

get the final polygon

**Remarks:** get the final polygon

**Returns:** `Polygon3D` — final polygon

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PolygonInput/#NemAll_Python_IFW_Input.PolygonInput.GetPolygon)

#### `GetPreviewPolygon()`

get the preview polygon

**Remarks:** get the preview polygon

**Returns:** `Polygon3D` — preview polygon

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PolygonInput/#NemAll_Python_IFW_Input.PolygonInput.GetPreviewPolygon)

#### `StartNewInput()`

Start new input

**Remarks:** Start new input

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PolygonInput/#NemAll_Python_IFW_Input.PolygonInput.StartNewInput)

## PolylineInput (class)

Implementation of the polyline input

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PolylineInput/)

### Constructors
- `PolylineInput(coordInput, bZCoord)` — Args: coordInput: Coordinate input object bZCoord: Z-coordinate input state

### Methods
#### `ExecuteInput(mouseMsg, pnt, pMsgInfo)`

Execute the input

**Remarks:** Execute the input

**Parameters:**
- `mouseMsg` (int) — Mouse message
- `pnt` (Point2D) — View input point
- `pMsgInfo` (AddMsgInfo) — Additional message info

**Returns:** `int` — execution state

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PolylineInput/#NemAll_Python_IFW_Input.PolylineInput.ExecuteInput)

#### `GetPolyline()`

get the final polyline

**Remarks:** get the final polyline

**Returns:** `Polyline3D` — final polyline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PolylineInput/#NemAll_Python_IFW_Input.PolylineInput.GetPolyline)

#### `GetPreviewPolyline()`

get the preview polyline

**Remarks:** get the preview polyline

**Returns:** `Polyline3D` — preview polyline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PolylineInput/#NemAll_Python_IFW_Input.PolylineInput.GetPreviewPolyline)

#### `StartNewInput()`

Start new input

**Remarks:** Start new input

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PolylineInput/#NemAll_Python_IFW_Input.PolylineInput.StartNewInput)

## PostElementSelection (class)

(No description provided in vendor docs for NemAll_Python_IFW_Input.PostElementSelection.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PostElementSelection/)

### Constructors
- `PostElementSelection()` — Initialize

### Methods
#### `GetSelectedElements(doc)`

Get the selected elements

**Remarks:** Get the selected elements

**Parameters:**
- `doc` (DocumentAdapter) — Document

**Returns:** `BaseElementAdapterList` — Selected elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PostElementSelection/#NemAll_Python_IFW_Input.PostElementSelection.GetSelectedElements)

## PreviewSymbolBuilder (class)

Implementation of the preview symbol builder

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/)

### Methods
#### `ArrowSymbol(pnt, bDrawIso, viewProj, colorVariant, rotationAngle, allWindows=True)`

Create a small arrow symbol

**Remarks:** Create a small arrow symbol

**Parameters:**
- `pnt` (Point3D) — Arrowhead point
- `bDrawIso` (bool) — Draw the arrow symbol inside the isometric view: true/false
- `viewProj` (ViewWorldProjection) — View world projection data
- `colorVariant` (ARGB) — Color of the preview
- `rotationAngle` (float) — Rotation angle of the rectangle
- `allWindows` (bool) — Show symbol in all windows

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.ArrowSymbol)

#### `ArrowSymbol(pnt, bDrawIso, viewProj, colorVariant, widthVariant, rotationAngle, allWindows=True)`

Create an arrow symbol

**Remarks:** Create an arrow symbol

**Parameters:**
- `pnt` (Point3D) — Arrowhead point
- `bDrawIso` (bool) — Draw the arrow symbol inside the isometric view: true/false
- `viewProj` (ViewWorldProjection) — View world projection data
- `colorVariant` (ARGB) — Color of the preview
- `widthVariant` (int) — Width of the symbol
- `rotationAngle` (float) — Rotation angle of the rectangle
- `allWindows` (bool) — Show symbol in all windows

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.ArrowSymbol)

#### `Circle3DSymbol(refPnt, circle, viewProj, colorVariant, linePattern, lineWidth)`

Create a 3D circle symbol preview

**Remarks:** Create a 3D circle symbol preview

**Parameters:**
- `refPnt` (Point3D) — Reference point
- `circle` (Arc3D) — 3D circle
- `viewProj` (ViewWorldProjection) — View world projection data
- `colorVariant` (ARGB) — Color of the preview
- `linePattern` (int) — Line pattern
- `lineWidth` (float) — Width of the line

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.Circle3DSymbol)

#### `CircleSymbol(pnt, bDrawIso, viewProj, colorVariant, radius)`

Create a circle symbol

**Remarks:** Create a circle symbol

**Parameters:**
- `pnt` (Point3D) — Center point
- `bDrawIso` (bool) — Draw the circle symbol inside the isometric view: true/false
- `viewProj` (ViewWorldProjection) — View world projection data
- `colorVariant` (ARGB) — Color of the preview
- `radius` (int) — Radius of the circle in pixel

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.CircleSymbol)

#### `CoordCrossSymbol(axisPlacement, armLength, viewProj)`

Draw the coordinate cross symbol

**Remarks:** Draw the coordinate cross symbol

**Parameters:**
- `axisPlacement` (AxisPlacement3D) — Axis placement
- `armLength` (int) — Length of the symbol arms
- `viewProj` (ViewWorldProjection) — View world projection

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.CoordCrossSymbol)

#### `CoordCrossSymbol(plane, armLength, viewProj)`

Draw the coordinate cross symbol

**Remarks:** Draw the coordinate cross symbol

**Parameters:**
- `plane` (Plane3D) — Plane
- `armLength` (int) — Length of the symbol arms
- `viewProj` (ViewWorldProjection) — View world projection

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.CoordCrossSymbol)

#### `CrossSymbol(pnt, bDrawIso, viewProj, colorVariant, widthVariant)`

Create a cross symbol

**Remarks:** Create a cross symbol

**Parameters:**
- `pnt` (Point3D) — Cross center point
- `bDrawIso` (bool) — Draw the cross symbol inside the isometric view: true/false
- `viewProj` (ViewWorldProjection) — View world projection data
- `colorVariant` (ARGB) — Color of the preview
- `widthVariant` (int) — Width of the symbol

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.CrossSymbol)

#### `FilledRectangleSymbol(pnt, bDrawIso, viewProj, colorVariant, widthVariant, rotationAngle)`

Create a filled rectangle symbol

**Remarks:** Create a filled rectangle symbol

**Parameters:**
- `pnt` (Point3D) — Rectangle center point
- `bDrawIso` (bool) — Draw the rectangle symbol inside the isometric view: true/false
- `viewProj` (ViewWorldProjection) — View world projection data
- `colorVariant` (ARGB) — Color of the preview
- `widthVariant` (int) — Width of the symbol
- `rotationAngle` (float) — Rotation angle of the rectangle

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.FilledRectangleSymbol)

#### `Line3DSymbol(refPnt, line, viewProj, colorVariant, linePattern, lineWidth)`

Create a 3D line symbol preview

**Remarks:** Create a 3D line symbol preview

**Parameters:**
- `refPnt` (Point3D) — Reference point
- `line` (Line3D) — 3D line
- `viewProj` (ViewWorldProjection) — View world projection data
- `colorVariant` (ARGB) — Color of the preview
- `linePattern` (int) — Line pattern
- `lineWidth` (int) — Line width

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.Line3DSymbol)

#### `LocalCoordinateSystem(coordSystemMatrix, flags, maxSize)`

Create a symbol for a local coordinate system

**Remarks:** Create a symbol for a local coordinate system

**Parameters:**
- `coordSystemMatrix` (Matrix3D) — Matrix of the coordinate system
- `flags` (LCS_Flags) — Coordinate system flags
- `maxSize` (float) — Max size of the coordinate system

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.LocalCoordinateSystem)

#### `MarkSymbol(pnt, bDrawIso, viewProj, colorVariant, widthVariant)`

Create a mark symbol (drawn an x)

**Remarks:** Create a mark symbol (drawn an x)

**Parameters:**
- `pnt` (Point3D) — Mark center point
- `bDrawIso` (bool) — Draw the mark symbol inside the isometric view: true/false
- `viewProj` (ViewWorldProjection) — View world projection data
- `colorVariant` (ARGB) — Color of the preview
- `widthVariant` (int) — Width of the symbol

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.MarkSymbol)

#### `OffsetPointSymbols(refPnt, offPnt, bDrawIso, viewProj, colorVariant, widthVariant, refPntAngle, offPntAngle)`

Create the symbols for an offset point

**Remarks:** Create the symbols for an offset point

**Parameters:**
- `refPnt` (Point3D) — Reference point
- `offPnt` (Point3D) — Offset point
- `bDrawIso` (bool) — Draw the arrow symbol inside the isometric view: true/false
- `viewProj` (ViewWorldProjection) — View world projection data
- `colorVariant` (ARGB) — Color of the preview
- `widthVariant` (int) — Width of the symbol
- `refPntAngle` (float) — Angle at the reference point
- `offPntAngle` (float) — Angle at the offset point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.OffsetPointSymbols)

#### `OrthogonalSymbol(pnt, bDrawIso, viewProj, colorVariant, widthVariant)`

Create an orthogonal symbol

**Remarks:** Create an orthogonal symbol

**Parameters:**
- `pnt` (Point3D) — Center point
- `bDrawIso` (bool) — Draw the circle symbol inside the isometric view: true/false
- `viewProj` (ViewWorldProjection) — View world projection data
- `colorVariant` (ARGB) — Color of the preview
- `widthVariant` (int) — Width of the symbol

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.OrthogonalSymbol)

#### `ParallelSymbol(pnt, bDrawIso, viewProj, colorVariant, widthVariant)`

Create a parallel symbol

**Remarks:** Create a parallel symbol

**Parameters:**
- `pnt` (Point3D) — Center point
- `bDrawIso` (bool) — Draw the circle symbol inside the isometric view: true/false
- `viewProj` (ViewWorldProjection) — View world projection data
- `colorVariant` (ARGB) — Color of the preview
- `widthVariant` (int) — Width of the symbol

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.ParallelSymbol)

#### `Polyline2DSymbol(refPnt, polyline, viewProj, colorVariant, linePattern)`

Create a 2D polyline symbol preview

**Remarks:** Create a 2D polyline symbol preview

**Parameters:**
- `refPnt` (Point3D) — Reference point
- `polyline` (Polyline2D) — Polyline
- `viewProj` (ViewWorldProjection) — View world projection
- `colorVariant` (ARGB) — Color of the preview
- `linePattern` (int) — Line pattern

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.Polyline2DSymbol)

#### `Polyline3DSymbol(polyline, viewProj, colorVariant, linePattern)`

Create a 3D polyline symbol preview

**Remarks:** Create a 3D polyline symbol preview

**Parameters:**
- `polyline` (Polyline3D) — 3D Polyline
- `viewProj` (ViewWorldProjection) — View world projection data
- `colorVariant` (ARGB) — Color of the preview
- `linePattern` (int) — Line pattern

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.Polyline3DSymbol)

#### `Polyline3DSymbol(refPnt, polyline, viewProj, colorVariant, linePattern)`

Create a 3D polyline symbol preview

**Remarks:** Create a 3D polyline symbol preview

**Parameters:**
- `refPnt` (Point3D) — Reference point
- `polyline` (Polyline3D) — Polyline
- `viewProj` (ViewWorldProjection) — View world projection
- `colorVariant` (ARGB) — Color of the preview
- `linePattern` (int) — Line pattern

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.Polyline3DSymbol)

#### `TrackLine(line, bDrawIso, viewProj, colorVariant, trackLineType)`

Create a track line

**Remarks:** Create a track line

**Parameters:**
- `line` (Line3D) — Track line
- `bDrawIso` (bool) — Draw the circle symbol inside the isometric view: true/false
- `viewProj` (ViewWorldProjection) — View world projection data
- `colorVariant` (ARGB) — Color of the preview
- `trackLineType` (eTrackLineType) — Type of the track line

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.TrackLine)

#### `TrackMarkSymbol(pnt, bDrawIso, viewProj, colorVariant, widthVariant)`

Create a track mark symbol

**Remarks:** Create a track mark symbol

**Parameters:**
- `pnt` (Point3D) — Center point
- `bDrawIso` (bool) — Draw the circle symbol inside the isometric view: true/false
- `viewProj` (ViewWorldProjection) — View world projection data
- `colorVariant` (ARGB) — Color of the preview
- `widthVariant` (int) — Width of the symbol

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.TrackMarkSymbol)

## QueryTypeID (class)

Implementation of the element type ID query

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/QueryTypeID/)

### Constructors
- `QueryTypeID()` — Initialize
- `QueryTypeID(typeID)` — Constructor
- `QueryTypeID(element)` — Copy constructor

### Methods
#### `GetQueryText()`

Get the query text

**Remarks:** Get the query text

**Returns:** `str` — Query text

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/QueryTypeID/#NemAll_Python_IFW_Input.QueryTypeID.GetQueryText)

#### `GetQueryTypeID()`

Get the ID of the type query

**Remarks:** Get the ID of the type query

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/QueryTypeID/#NemAll_Python_IFW_Input.QueryTypeID.GetQueryTypeID)

#### `__call__(ele)`

Overloaded operator ()

**Remarks:** Overloaded operator ()

**Parameters:**
- `ele` (BaseElementAdapter) — Element to check

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/QueryTypeID/#NemAll_Python_IFW_Input.QueryTypeID.__call__)

#### `__repr__()`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/QueryTypeID/#NemAll_Python_IFW_Input.QueryTypeID.__repr__)

## SelectElementsService (enum)

(No description provided in vendor docs for NemAll_Python_IFW_Input.SelectElementsService.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/SelectElementsService/)

### Methods
#### `SelectByPolygon(polygon, viewProjection, selCond, filter)`

Select an element by a point

**Remarks:** Select an element by a point

**Parameters:**
- `doc` (object) — Document
- `polygon` (Polygon2D) — surrounding search polygon (view coordinates)
- `viewProjection` (ViewWorldProjection) — Identification of view projection (window)
- `selCond` (eSelectCondition) — True when elements inside rectangle must be selected only
- `filter` (SelectionQuery) — Selection filter
- `activationData` (object) — Activation data

**Returns:** `BaseElementAdapterList` — Data of the selected elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/SelectElementsService/#NemAll_Python_IFW_Input.SelectElementsService.SelectByPolygon)

#### `SelectByRect(cursorLeftBottomPoint, cursorRightTopPoint, viewProjection, selCond, filter)`

Select an element by a point

**Remarks:** Select an element by a point

**Parameters:**
- `doc` (object) — Document
- `cursorLeftBottomPoint` (Point2D) — Coordinate of left bottom point where elements will be searching
- `cursorRightTopPoint` (Point2D) — Coordinate of right top point where elements will be searching
- `viewProjection` (ViewWorldProjection) — Identification of view projection (window)
- `selCond` (eSelectCondition) — True when elements inside rectangle must be selected only
- `filter` (SelectionQuery) — Selection filter
- `activationData` (object) — Activation data

**Returns:** `BaseElementAdapterList` — Data of the selected elements

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/SelectElementsService/#NemAll_Python_IFW_Input.SelectElementsService.SelectByRect)

## SelectionMode (enum)

Split type for 3D elements

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/SelectionMode/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `SelectionMode` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/SelectionMode/#NemAll_Python_IFW_Input.SelectionMode.__getitem__)

### Values
- `eSelectGeometry` = `0`
- `eSelectObject` = `1`
- `eSelectSubObject` = `3`

## SelectionQuery (class)

Implementation of the selection query

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/SelectionQuery/)

### Constructors
- `SelectionQuery()` — Initialize
- `SelectionQuery(query)` — Copy constructor
- `SelectionQuery(query)` — Constructor

### Methods
#### `Clear()`

Clear the query

**Remarks:** Clear the query

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/SelectionQuery/#NemAll_Python_IFW_Input.SelectionQuery.Clear)

#### `IsEmpty()`

Check for an empty query

**Remarks:** Check for an empty query

**Returns:** `bool` — Filter is empty: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/SelectionQuery/#NemAll_Python_IFW_Input.SelectionQuery.IsEmpty)

## SnoopElementGeometryFilter (class)

(No description provided in vendor docs for NemAll_Python_IFW_Input.SnoopElementGeometryFilter.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/SnoopElementGeometryFilter/)

### Constructors
- `SnoopElementGeometryFilter()` — Initialize
- `SnoopElementGeometryFilter(bFindBaseGeometry, bFindAreaGeometry, bPerpendicularOnElement, bFindNonPassiveOnly, bSplitAreaGeometries, bIdentifyEmbeddedElement, bFindCompleteFootprint, splitElement3D)` — Constructor Passive geometry is virtual geometry which is using for element snooping. Typical passive geometry is boundary area around text. This area has no edges like a Hatching, but if cursor is inside this area, then element must be snooped. Passive geometry has no points, perpendicular points, edges, etc. .

### Methods
#### `AddElements(arg2)`



[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/SnoopElementGeometryFilter/#NemAll_Python_IFW_Input.SnoopElementGeometryFilter.AddElements)

## UndoRedoService (class)

Implementation of the undo/redo service

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/UndoRedoService/)

### Constructors
- `UndoRedoService(doc, isPassivateAll=False, isLockPreviewDraw=False, collectEleForMultipleTransactions=False)` — Constructor
- `UndoRedoService(element)` — Copy constructor

### Methods
#### `ActivateCollectedElementsForFinishUpdate()`

activate collected elements from undo step, important for next finish update

**Remarks:** activate collected elements from undo step, important for next finish update

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/UndoRedoService/#NemAll_Python_IFW_Input.UndoRedoService.ActivateCollectedElementsForFinishUpdate)

#### `CollectElementsForMultipleTransactions()`

Collect the elements for adding multiple transactions to one undo step

**Remarks:** Collect the elements for adding multiple transactions to one undo step

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/UndoRedoService/#NemAll_Python_IFW_Input.UndoRedoService.CollectElementsForMultipleTransactions)

#### `CreateUndoStep()`

Create an undo step

**Remarks:** Create an undo step

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/UndoRedoService/#NemAll_Python_IFW_Input.UndoRedoService.CreateUndoStep)

#### `CreateUndoStep(eventID)`

Create an undo step

**Remarks:** Create an undo step

**Parameters:**
- `eventID` (int) — Event ID of the undo step

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/UndoRedoService/#NemAll_Python_IFW_Input.UndoRedoService.CreateUndoStep)

#### `IsInUndoService()`

Check if an active undo service is present

**Remarks:** Check if an active undo service is present

**Returns:** `bool` — Active undo service is present

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/UndoRedoService/#NemAll_Python_IFW_Input.UndoRedoService.IsInUndoService)

#### `SetUndoDescription(textID)`

Set the undo step description independent from menu event text

**Remarks:** Set the undo step description independent from menu event text

**Parameters:**
- `textID` (int) — Text ID of the undo step

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/UndoRedoService/#NemAll_Python_IFW_Input.UndoRedoService.SetUndoDescription)

#### `SetUndoStepEvent(eventID)`

Set the undo step event

**Remarks:** Set the undo step event

**Parameters:**
- `eventID` (int) — Event ID of the undo step

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/UndoRedoService/#NemAll_Python_IFW_Input.UndoRedoService.SetUndoStepEvent)

## ValueInputControlData (class)

(No description provided in vendor docs for NemAll_Python_IFW_Input.ValueInputControlData.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ValueInputControlData/)

### Constructors
- `ValueInputControlData()` — Initialize
- `ValueInputControlData(ctrlType, bSetFocus, bDisableCoord)` — Constructor to create a control with standard behavior defined by the control type
- `ValueInputControlData(ctrlType, initValue, bSetFocus, bDisableCoord)` — Constructor to create a control with an init value. The value range is defined by the control type

## ViewWorldProjection (class)

(No description provided in vendor docs for NemAll_Python_IFW_Input.ViewWorldProjection.)

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/)

### Constructors
- `ViewWorldProjection()` — Initialize

### Methods
#### `CreateForUnitTest()`



[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.CreateForUnitTest)

#### `GetDocumentID()`

Get the document ID

**Remarks:** Get the document ID

**Returns:** `int` — Document ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.GetDocumentID)

#### `GetEyePoint()`

Get eye point of current projection Eye point is point where camera is.

**Remarks:** Get eye point of current projection Eye point is point where camera is.

**Returns:** `Point3D` — Eye point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.GetEyePoint)

#### `GetIsoProjection()`

Get the isometric projection of the view

**Remarks:** Get the isometric projection of the view

**Returns:** `eProjectionType` — Isometric projection of the view

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.GetIsoProjection)

#### `GetMatrix()`



[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.GetMatrix)

#### `GetPixelFactor()`

Get factor of pixel to world coordinates Used when you need calculate how much millimeters are one pixel

**Remarks:** Get factor of pixel to world coordinates Used when you need calculate how much millimeters are one pixel

**Returns:** `tuple` — Pixel factor in x direction,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.GetPixelFactor)

#### `GetScreenScale()`

Get the screen scale

**Remarks:** Get the screen scale

**Returns:** `float` — Screen scale

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.GetScreenScale)

#### `GetSearchRadiusByPixel(pixel)`

Get the search radius by pixel

**Remarks:** Get the search radius by pixel

**Parameters:**
- `pixel` (int) — Pixel of the search radius

**Returns:** `float` — Search radius in view size

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.GetSearchRadiusByPixel)

#### `GetViewAngle()`

Get the rotation angle of the view

**Remarks:** Get the rotation angle of the view

**Returns:** `float` — Rotation angle of the view

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.GetViewAngle)

#### `GetViewPoint()`

Get view point of current projection View point is point where you are looking (focused). View point lie in plain of drawn data.

**Remarks:** Get view point of current projection View point is point where you are looking (focused). View point lie in plain of drawn data.

**Returns:** `Point3D` — Eye point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.GetViewPoint)

#### `GetViewSize()`

Get the size of the view

**Remarks:** Get the size of the view

**Returns:** `Vector2D` — Size of the view

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.GetViewSize)

#### `GetViewZAngle()`

Get the rotation angle of the view in z-direction

**Remarks:** Get the rotation angle of the view in z-direction

**Returns:** `float` — Rotation angle of the view in z-direction

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.GetViewZAngle)

#### `IsAssistWindow()`

Check for assist window

**Remarks:** Check for assist window

**Returns:** `bool` — The document is an assist window: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.IsAssistWindow)

#### `IsCentralProjection()`

Check, whether the projection is a central projection

**Remarks:** Check, whether the projection is a central projection

**Returns:** `bool` — Central projection: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.IsCentralProjection)

#### `IsFreeProjection()`

Check, whether the projection is a free projection

**Remarks:** Check, whether the projection is a free projection

**Returns:** `bool` — Projection is a free view: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.IsFreeProjection)

#### `IsGroundplanView()`

Check, whether the projection is ground plan view

**Remarks:** Check, whether the projection is ground plan view

**Returns:** `bool` — Projection is ground plan view: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.IsGroundplanView)

#### `IsInView(pnt)`

Check, whether the point is inside the view

**Remarks:** Check, whether the point is inside the view

**Parameters:**
- `pnt` (Point2D) — View point

**Returns:** `bool` — Point is inside the view: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.IsInView)

#### `IsIsometricProjection()`

Check, whether the projection is a isometric projection

**Remarks:** Check, whether the projection is a isometric projection

**Returns:** `bool` — Projection is a isometric projection: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.IsIsometricProjection)

#### `IsSideView()`

Check, whether the projection is a side view

**Remarks:** Check, whether the projection is a side view

**Returns:** `bool` — Projection is a side view: true/false

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.IsSideView)

#### `ProjectionToWorld(pnt, refPnt)`

Get the world 3D point from a projection 2D point and a reference point

**Remarks:** Get the world 3D point from a projection 2D point and a reference point

**Parameters:**
- `pnt` (Point2D) — View Point
- `refPnt` (Point3D) — Reference point with the additional coordinate

**Returns:** `Point3D` — World 3D point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.ProjectionToWorld)

#### `ViewPerpendicularToWorld(line3D, pnt)`

Transform a view perpendicular point to a world perpendicular point

**Remarks:** Transform a view perpendicular point to a world perpendicular point

**Parameters:**
- `line3D` (Line3D) — 3D perpendicular line
- `pnt` (Point3D) — Reference point for the perpendicular

**Returns:** `Point3D` — World perpendicular point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.ViewPerpendicularToWorld)

#### `ViewToPixel(pnt, bBottomTop, considerZoomwindow)`

Transform a view point to a pixel coordinate

**Remarks:** Transform a view point to a pixel coordinate

**Parameters:**
- `pnt` (Point2D) — View point
- `bBottomTop` (bool) — The y zero point is on the bottom of the screen: true/false
- `considerZoomwindow` (bool) — if true and the position is in a zoom window, then the zoom window will be used for the calculation

**Returns:** `Point2D` — Pixel view point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.ViewToPixel)

#### `ViewToWorld(pnt, z)`

Transform a view point to a world point

**Remarks:** Transform a view point to a world point

**Parameters:**
- `pnt` (Point2D) — View point
- `z` (float) — z-coordinate

**Returns:** `Point3D` — World point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.ViewToWorld)

#### `ViewToWorldBaseZ(pnt, zWorld)`

Transform a view point to a world point with resulting z-coordinate

**Remarks:** Transform a view point to a world point with resulting z-coordinate

**Parameters:**
- `pnt` (Point2D) — View point
- `zWorld` (float) — Z-coordinate

**Returns:** `Point3D` — World point with resulting z-coordinate

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.ViewToWorldBaseZ)

#### `ViewToWorldBaseZ0(pnt)`

Transform a view point to a world point with resulting z-world = 0

**Remarks:** Transform a view point to a world point with resulting z-world = 0

**Parameters:**
- `pnt` (Point2D) — View point

**Returns:** `Point3D` — World point with z=0

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.ViewToWorldBaseZ0)

#### `ViewToWorldPlane(pnt, plane)`

Transform the view point to a world plane

**Remarks:** Transform the view point to a world plane

**Parameters:**
- `pnt` (Point2D) — View point
- `plane` (Plane3D) — Plane

**Returns:** `Point3D` — World point at the plane

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.ViewToWorldPlane)

#### `ViewToWorldRay(pnt)`

Calculates a ray Calculates a ray (for non-vanishing-point-projections, rather a line) from the given view 2D point. Can be used for pick-point calculations. Note that we have a right-hand view coordinate system, so its Z axis points towards the eye.

**Remarks:** Calculates a ray Calculates a ray (for non-vanishing-point-projections, rather a line) from the given view 2D point. Can be used for pick-point calculations. Note that we have a right-hand view coordinate system, so its Z axis points towards the eye.

**Parameters:**
- `pnt` (Point2D) — View point

**Returns:** `tuple` — World point of ray,

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.ViewToWorldRay)

#### `WorldToPixel(pnt, bBottomTop)`

Transform the world point to a pixel point

**Remarks:** Transform the world point to a pixel point

**Parameters:**
- `pnt` (Point3D) — World point
- `bBottomTop` (bool) — The y zero point is on the bottom of the screen: true/false

**Returns:** `Point2D` — Pixel point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.WorldToPixel)

#### `WorldToProjection(line)`

Get the projection 2D line from a world 3D line

**Remarks:** Get the projection 2D line from a world 3D line

**Parameters:**
- `line` (Line3D) — World line

**Returns:** `Line2D` — 2D projection line

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.WorldToProjection)

#### `WorldToProjection(pnt)`

Get the projection 2D point from a world 3D point

**Remarks:** Get the projection 2D point from a world 3D point

**Parameters:**
- `pnt` (Point3D) — World point

**Returns:** `Point2D` — 2D projection point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.WorldToProjection)

#### `WorldToProjectionBase0(line)`

Get the world projection line with the base 0

**Remarks:** Get the world projection line with the base 0

**Parameters:**
- `line` (Line3D) — World coordinate line

**Returns:** `Line3D` — World projection point with the base 0

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.WorldToProjectionBase0)

#### `WorldToProjectionBase0(pnt)`

Get the world projection point with the base 0

**Remarks:** Get the world projection point with the base 0

**Parameters:**
- `pnt` (Point3D) — World point

**Returns:** `Point3D` — World projection point with the base 0

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.WorldToProjectionBase0)

#### `WorldToView(line)`

Transform a 3D world line to a 2D view line

**Remarks:** Transform a 3D world line to a 2D view line

**Parameters:**
- `line` (Line3D) — World line

**Returns:** `Line2D` — View line

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.WorldToView)

#### `WorldToView(line)`

Transform a 2D world line to a 2D view line

**Remarks:** Transform a 2D world line to a 2D view line

**Parameters:**
- `line` (Line2D) — World line

**Returns:** `Line2D` — View line

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.WorldToView)

#### `WorldToView(pnt)`

Transform a world point to a view point

**Remarks:** Transform a world point to a view point

**Parameters:**
- `pnt` (Point3D) — World point

**Returns:** `Point2D` — View point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.WorldToView)

#### `WorldToView(pnt)`

Transform a world point to a view point

**Remarks:** Transform a world point to a view point

**Parameters:**
- `pnt` (Point2D) — World point

**Returns:** `Point2D` — View point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.WorldToView)

#### `WorldToView(polyline3D)`

Transform a 3D world polyline to a 2D view polyline

**Remarks:** Transform a 3D world polyline to a 2D view polyline

**Parameters:**
- `polyline3D` (Polyline3D) — World polyline

**Returns:** `Polyline2D` — View polyline

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.WorldToView)

#### `WorldToView(x, y, z)`

Transform a world point to a view point

**Remarks:** Transform a world point to a view point

**Parameters:**
- `x` (float) — X-coordinate world
- `y` (float) — Y-coordinate world
- `z` (float) — Z-coordinate world

**Returns:** `Point2D` — View point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.WorldToView)

#### `WorldToView3D(pnt)`

Transform a world point to a view 3D point If Z coordinate of returned view point is positive, then world point is before eye (i.e. is visible).

**Remarks:** Transform a world point to a view 3D point If Z coordinate of returned view point is positive, then world point is before eye (i.e. is visible).

**Parameters:**
- `pnt` (Point3D) — World point

**Returns:** `Point3D` — View point

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.WorldToView3D)

#### `WorldToWorldPlane(line, plane)`

Transform a world line to a plane line

**Remarks:** Transform a world line to a plane line

**Parameters:**
- `line` (Line3D) — World line
- `plane` (Plane3D) — Plane

**Returns:** `Line3D` — Plane line in world coordinates

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.WorldToWorldPlane)

#### `WorldToWorldPlane(pnt, plane)`

Transform a world point to a plane point

**Remarks:** Transform a world point to a plane point

**Parameters:**
- `pnt` (Point3D) — World point
- `plane` (Plane3D) — Plane

**Returns:** `Point3D` — Plane point in world coordinates

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.WorldToWorldPlane)

## eDocumentSnoopType (enum)

Definition of the document snoop types

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/eDocumentSnoopType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `eDocumentSnoopType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/eDocumentSnoopType/#NemAll_Python_IFW_Input.eDocumentSnoopType.__getitem__)

### Values
- `eSnoopActiveDocuments` = `0`
- `eSnoopAllDocuments` = `2`
- `eSnoopPassiveDocsOrLayers` = `3`
- `eSnoopPassiveDocuments` = `1`

## eDrawElementIdentPointSymbols (enum)

Drawing state for the element identification point symbols

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/eDrawElementIdentPointSymbols/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `eDrawElementIdentPointSymbols` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/eDrawElementIdentPointSymbols/#NemAll_Python_IFW_Input.eDrawElementIdentPointSymbols.__getitem__)

### Values
- `eDRAW_IDENT_ELEMENT_POINT_SYMBOL_NO` = `0`
- `eDRAW_IDENT_ELEMENT_POINT_SYMBOL_YES` = `1`

## eIdentificationMode (enum)

Type of the identification mode

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/eIdentificationMode/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `eIdentificationMode` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/eIdentificationMode/#NemAll_Python_IFW_Input.eIdentificationMode.__getitem__)

### Values
- `eIDENT_ARCHPOINT` = `10`
- `eIDENT_ARCHPOINT_OFFSET` = `11`
- `eIDENT_ARCH_ELEMENTPOINT` = `12`
- `eIDENT_ELEMENTPOINT` = `1`
- `eIDENT_POINT` = `0`
- `eIDENT_POINT_ASSOC_VIEW_WORLD` = `2`
- `eIDENT_POINT_ELEMENT` = `3`
- `eIDENT_POINT_ELEMENT_ALWAYS` = `5`
- `eIDENT_POINT_ELEMENT_ALWAYS_CENTER` = `6`
- `eIDENT_POINT_ELEMENT_CENTER` = `4`
- `eIDENT_POINT_OFFSET` = `8`
- `eIDENT_POINT_PERPENDICULAR` = `7`
- `eIDENT_TEXTPOINT` = `9`

## eLayerSnoopType (enum)

Definition of the layer snoop types

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/eLayerSnoopType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `eLayerSnoopType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/eLayerSnoopType/#NemAll_Python_IFW_Input.eLayerSnoopType.__getitem__)

### Values
- `eSnoopActiveLayers` = `0`
- `eSnoopActiveLayersMsg` = `3`
- `eSnoopAllLayers` = `2`
- `eSnoopPassiveLayers` = `1`
- `eSnoopPassiveLayersOrDocs` = `4`

## eProjectionType (enum)

Projection type of the view

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/eProjectionType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `eProjectionType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/eProjectionType/#NemAll_Python_IFW_Input.eProjectionType.__getitem__)

### Values
- `EAST_VIEW` = `3`
- `FREE_ONLY_3D` = `9`
- `FREE_VIEW` = `0`
- `GROUND_PLAN` = `1`
- `NORTH_EAST_VIEW` = `5`
- `NORTH_VIEW` = `2`
- `NORTH_WEST_VIEW` = `6`
- `SOUTH_EAST_VIEW` = `8`
- `SOUTH_VIEW` = `-2`
- `SOUTH_WEST_VIEW` = `7`
- `WEST_VIEW` = `-3`
- `WORKING_PLANE_VIEW` = `4`

## eSplitElement3D (enum)

Split type for 3D elements

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/eSplitElement3D/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `eSplitElement3D` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/eSplitElement3D/#NemAll_Python_IFW_Input.eSplitElement3D.__getitem__)

### Values
- `ELEMENT3D_EDGES` = `2`
- `ELEMENT3D_FACES` = `1`
- `ELEMENT3D_NO_SPLIT` = `0`

## eTrackLineType (enum)

Definition of the track line types

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/eTrackLineType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `eTrackLineType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/eTrackLineType/#NemAll_Python_IFW_Input.eTrackLineType.__getitem__)

### Values
- `TRACKLINE_ENDLESS` = `1`
- `TRACKLINE_EXTENSION` = `2`
- `TRACKLINE_EXTENSION_END` = `3`
- `TRACKLINE_NO` = `0`

## eValueInputControlType (enum)

Type of the value input control

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/eValueInputControlType/)

### Methods
#### `__getitem__(key)`

get the item for a key

**Remarks:** get the item for a key

**Parameters:**
- `key` (str | int | float) — value key

**Returns:** `eValueInputControlType` — value for the key

[Docs](https://pythonparts.allplan.com/2024/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/eValueInputControlType/#NemAll_Python_IFW_Input.eValueInputControlType.__getitem__)

### Values
- `eANGLE_COMBOBOX` = `6`
- `eCONTROL_EXTERNAL` = `1`
- `eCONTROL_NONE` = `0`
- `eCOORDINATE_EDIT` = `101`
- `eCOORDINATE_EDIT_FIX` = `102`
- `eCOORDINATE_EDIT_GE0` = `103`
- `eCOORDINATE_EDIT_GT0` = `104`
- `eDIMENSION_EDIT` = `5`
- `eINT_COMBOBOX` = `4`
- `eINT_EDIT` = `3`
- `eNUMBER_EDIT_1` = `201`
- `eNUMBER_EDIT_1_GE0` = `202`
- `eROTATION_ANGLE_STEP` = `8`
- `eTEXT_EDIT` = `9`
- `eWALL_PLACEMENT` = `7`

