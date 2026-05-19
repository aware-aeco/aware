---
name: allplan-nemall_python_ifw_input
description: This skill encodes the allplan 2025.0 surface of the NemAll_Python_IFW_Input namespace — 38 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Functions, AddMsgInfo, BuildingElementInputControls, CNOI_DocumentWrapper, CoordinateInputMode, CoordinateInputResult, CoordinateInput, ElementHandleType, and 30 more types.
---

# NemAll_Python_IFW_Input

Auto-generated from vendor docs for allplan 2025.0. 38 types in this namespace.

## AddMsgInfo (class)

(No description provided in vendor docs for NemAll_Python_IFW_Input.AddMsgInfo.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/AddMsgInfo/)

### Constructors
- `AddMsgInfo()` — Initialize

## BuildingElementInputControls (class)

Implementation of the building element input controls

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/BuildingElementInputControls/)

### Constructors
- `BuildingElementInputControls() | BuildingElementInputControls(element: BuildingElementInputControls)` — Initialize

### Methods
#### `CloseControls()`

Close the input controls

**Remarks:** Close the input controls

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/BuildingElementInputControls/#NemAll_Python_IFW_Input.BuildingElementInputControls.CloseControls)

#### `CreateControls( handlePropList: object, insertionMat: Matrix3D, viewProj: ViewWorldProjection, bUpdateControls: bool, assoRefObj: object, )`

Create the controls

**Remarks:** Create the controls

**Parameters:**
- `handlePropList` (object) — List with the handle properties
- `insertionMat` (Matrix3D) — Transformation matrix
- `viewProj` (ViewWorldProjection) — View world projection
- `bUpdateControls` (bool) — Update the controls: true/false
- `assoRefObj` (object) — Reference element for the drawing inside the associative views

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/BuildingElementInputControls/#NemAll_Python_IFW_Input.BuildingElementInputControls.CreateControls)

## CNOI_DocumentWrapper (class)

(No description provided in vendor docs for NemAll_Python_IFW_Input.CNOI_DocumentWrapper.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CNOI_DocumentWrapper/)

### Constructors
- `CNOI_DocumentWrapper()` — Initialize

## CoordinateInput (class)

Base class implementation of the coordinate input

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/)

### Constructors
- `CoordinateInput(bZCoord: bool = True)` — Constructor

### Methods
#### `AddGeometryFromPreviewElements(elements: list)`

Add the geometry elements from the preview elements for the point and element search

**Remarks:** Add the geometry elements from the preview elements for the point and element search

**Parameters:**
- `elements` (list) — Geometry elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.AddGeometryFromPreviewElements)

#### `AllowSelectGeometryElement(bSelectGeoElement: bool)`

Allow to select an element

**Remarks:** Allow to select an element

**Parameters:**
- `bSelectGeoElement` (bool) — Select geometry element allowed: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.AllowSelectGeometryElement)

#### `CancelHighlightGeoElement()`

Cancel geometry element highlight

**Remarks:** Cancel geometry element highlight

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.CancelHighlightGeoElement)

#### `CancelInput()`

Explicit cancel of the input function

**Remarks:** Explicit cancel of the input function

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.CancelInput)

#### `EnableChangeXYFocus(bChangeXYFocus: bool)`

Enable the change of the x/y-focus after each input

**Remarks:** Enable the change of the x/y-focus after each input

**Parameters:**
- `bChangeXYFocus` (bool) — if true, the xy focus will be changed

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.EnableChangeXYFocus)

#### `EnableCoordinateInput(bEnable: bool)`

Enable / disable the coordinate input This function can be used to enable or disable the coordinate input.

**Remarks:** Enable / disable the coordinate input This function can be used to enable or disable the coordinate input.

**Parameters:**
- `bEnable` (bool) — Enable the coordinate input: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.EnableCoordinateInput)

#### `EnableUndoStep(bEnableUndoStep: bool)`

Enable disable the undo step

**Remarks:** Enable disable the undo step

**Parameters:**
- `bEnableUndoStep` (bool) — Enable the undo step: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.EnableUndoStep)

#### `EnableZCoord(bEnable: bool)`

Enable the z-coordinate inside the coordinate dialog

**Remarks:** Enable the z-coordinate inside the coordinate dialog

**Parameters:**
- `bEnable` (bool) — Z coordinate inside the coordinate dialog is enabled: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.EnableZCoord)

#### `GetAssocViewFromPoint( mouseMsg: int, pnt: Point2D, pMsgInfo: AddMsgInfo ) -> AssocViewElementAdapter`

Get the associative view from the point

**Remarks:** Get the associative view from the point

**Parameters:**
- `mouseMsg` (int) — Mouse message WM_xxx
- `pnt` (Point2D) — Cursor point (view coordinate)
- `pMsgInfo` (AddMsgInfo) — Additional message info

**Returns:** `AssocViewElementAdapter` — Standard return

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.GetAssocViewFromPoint)

#### `GetCurrentPoint() -> CoordinateInputResult | GetCurrentPoint(startPnt: Point3D) -> CoordinateInputResult | GetCurrentPoint(startPnt: Point3D, bStartPnt: bool) -> CoordinateInputResult | GetCurrentPoint(bStartPnt: bool) -> CoordinateInputResult`

Get and mark the current input point

**Remarks:** Get and mark the current input point

**Returns:** `CoordinateInputResult` — Current input point

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.GetCurrentPoint)

#### `GetInputAssocView() -> AssocViewElementAdapter`

Get the associative view of the input point / element The function can be used after a call to GetInputPoint, SelectGeometryElement, SelectElement, ...

**Remarks:** Get the associative view of the input point / element The function can be used after a call to GetInputPoint, SelectGeometryElement, SelectElement, ...

**Returns:** `AssocViewElementAdapter` — Associative view of the input point / element, NULL if point is outside

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.GetInputAssocView)

#### `GetInputControlIntValue() -> int`

Get the integer value from the value input control

**Remarks:** Get the integer value from the value input control

**Returns:** `int` — Integer value from the value input control

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.GetInputControlIntValue)

#### `GetInputControlText() -> object`

Get the text from the text input control

**Remarks:** Get the text from the text input control

**Returns:** `object` — Text from the text input control

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.GetInputControlText)

#### `GetInputControlValue() -> float`

Get the double value from the value input control

**Remarks:** Get the double value from the value input control

**Returns:** `float` — Double value from the value input control

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.GetInputControlValue)

#### `GetInputPoint( mouseMsg: int, pnt: Point2D, pMsgInfo: AddMsgInfo ) -> CoordinateInputResult | GetInputPoint( mouseMsg: int, pnt: Point2D, pMsgInfo: AddMsgInfo, bStartPnt: bool ) -> CoordinateInputResult | GetInputPoint( mouseMsg: int, pnt: Point2D, pMsgInfo: AddMsgInfo, startPnt: Point3D, bStartPnt: bool, ) -> CoordinateInputResult`

Perform a point identification inside the viewport

**Remarks:** Perform a point identification inside the viewport

**Parameters:**
- `mouseMsg` (int) — Mouse message
- `pnt` (Point2D) — Cursor point (view coordinate)
- `pMsgInfo` (AddMsgInfo) — Additional message info

**Returns:** `CoordinateInputResult` — Result of the point identification

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.GetInputPoint)

#### `GetReferenceLine() -> Line2D`

Get the reference line, on which the identified point is located. This method works when eIdentificationMode is set to eIDENT_ARCH... and only with on architectural elements

**Remarks:** Get the reference line, on which the identified point is located. This method works when eIdentificationMode is set to eIDENT_ARCH... and only with on architectural elements

**Returns:** `Line2D` — Reference line

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.GetReferenceLine)

#### `GetSelectedElement() -> BaseElementAdapter`

Get the selected element The function can be used in case of eIdentMode = MODE_TEXTPOINT, SelectGeometryElement, SelectElement, ...

**Remarks:** Get the selected element The function can be used in case of eIdentMode = MODE_TEXTPOINT, SelectGeometryElement, SelectElement, ...

**Returns:** `BaseElementAdapter` — Selected element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.GetSelectedElement)

#### `GetSelectedElementAssocView() -> AssocViewElementAdapter`

Get the related associative view of the selected element

**Remarks:** Get the related associative view of the selected element

**Returns:** `AssocViewElementAdapter` — Related associative view of the selected element

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.GetSelectedElementAssocView)

#### `GetSelectedElements() -> BaseElementAdapterList`

Get the selected elements The function can be used in case of eIdentMode = MODE_TEXTPOINT, SelectGeometryElement, SelectElement, ...

**Remarks:** Get the selected elements The function can be used in case of eIdentMode = MODE_TEXTPOINT, SelectGeometryElement, SelectElement, ...

**Returns:** `BaseElementAdapterList` — Vector of elements with the same distance to the cursor

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.GetSelectedElements)

#### `GetSelectedGeometryElement() -> Any`

Get the selected geometry element

**Remarks:** Get the selected geometry element

**Returns:** `Any` — IGeometry element of the selected geometry element (0 = no selection)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.GetSelectedGeometryElement)

#### `GetSelectedGeometryElements() -> list[Any]`

Get the selected geometry element

**Remarks:** Get the selected geometry element

**Returns:** `list[Any]` — IGeometry elements of the selected geometry elements (with the same distance)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.GetSelectedGeometryElements)

#### `InitFirstElementInput( text: InputStringConvert, identMode: CoordinateInputMode = CoordinateInputMode(), )`

Initialize the input for an element selection (point, line, ...) as free point input:

**Remarks:** Initialize the input for an element selection (point, line, ...) as free point input:

**Parameters:**
- `text` (InputStringConvert) — Prompt shown in the dialog line
- `identMode` (CoordinateInputMode) — Identification mode

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.InitFirstElementInput)

#### `InitFirstElementValueInput( text: InputStringConvert, ctrlData: ValueInputControlData, identMode: CoordinateInputMode = CoordinateInputMode(), )`

Initialize the input for an element selection (point, line, ...) as free point input:

**Remarks:** Initialize the input for an element selection (point, line, ...) as free point input:

**Parameters:**
- `text` (InputStringConvert) — Prompt shown in the dialog line
- `ctrlData` (ValueInputControlData) — Settings of the input control
- `identMode` (CoordinateInputMode) — Identification mode

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.InitFirstElementValueInput)

#### `InitFirstPointInput( text: InputStringConvert, identMode: CoordinateInputMode = CoordinateInputMode(), inputViewData: InputViewData = InputViewData(), )`

Initialize the input of a point:

**Remarks:** Initialize the input of a point:

**Parameters:**
- `text` (InputStringConvert) — Prompt shown in the dialog line
- `identMode` (CoordinateInputMode) — Identification mode
- `inputViewData` (InputViewData) — Input view data

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.InitFirstPointInput)

#### `InitFirstPointValueInput( text: InputStringConvert, ctrlData: ValueInputControlData, identMode: CoordinateInputMode = CoordinateInputMode(), inputViewData: InputViewData = InputViewData(), )`

Initialize the input of a point:

**Remarks:** Initialize the input of a point:

**Parameters:**
- `text` (InputStringConvert) — Prompt shown in the dialog line
- `ctrlData` (ValueInputControlData) — Settings of the additional input control
- `identMode` (CoordinateInputMode) — Identification mode

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.InitFirstPointValueInput)

#### `InitNextElementInput( text: InputStringConvert, identMode: CoordinateInputMode = CoordinateInputMode(), )`

Initialize the input for an element selection (point, line, ...) as free point input.

**Remarks:** Initialize the input for an element selection (point, line, ...) as free point input.

**Parameters:**
- `text` (InputStringConvert) — Prompt shown in the dialog line
- `identMode` (CoordinateInputMode) — Identification mode

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.InitNextElementInput)

#### `InitNextElementValueInput( text: InputStringConvert, ctrlData: ValueInputControlData, identMode: CoordinateInputMode = CoordinateInputMode(), )`

Initialize the input for an element selection (point, line, ...) as free point input:

**Remarks:** Initialize the input for an element selection (point, line, ...) as free point input:

**Parameters:**
- `text` (InputStringConvert) — Prompt shown in the dialog line
- `ctrlData` (ValueInputControlData) — Settings of the input control
- `identMode` (CoordinateInputMode) — Identification mode

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.InitNextElementValueInput)

#### `InitNextPointInput( text: InputStringConvert, identMode: CoordinateInputMode = CoordinateInputMode(), )`

Initialize the input of a point:

**Remarks:** Initialize the input of a point:

**Parameters:**
- `text` (InputStringConvert) — Prompt shown in the dialog line
- `identMode` (CoordinateInputMode) — Identification mode

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.InitNextPointInput)

#### `InitNextPointValueInput( text: InputStringConvert, ctrlData: ValueInputControlData, identMode: CoordinateInputMode = CoordinateInputMode(), )`

Initialize the input of a point:

**Remarks:** Initialize the input of a point:

**Parameters:**
- `text` (InputStringConvert) — Prompt shown in the dialog line
- `ctrlData` (ValueInputControlData) — Settings of the additional input control
- `identMode` (CoordinateInputMode) — Identification mode

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.InitNextPointValueInput)

#### `InitValueInput(text: InputStringConvert, ctrlData: ValueInputControlData)`

Initialize the value input

**Remarks:** Initialize the value input

**Parameters:**
- `text` (InputStringConvert) — Prompt shown in the dialog line
- `ctrlData` (ValueInputControlData) — Settings of the additional input control

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.InitValueInput)

#### `IsCoordinateInputEnabled() -> bool`

Check, whether the input controls are active (enabled)

**Remarks:** Check, whether the input controls are active (enabled)

**Returns:** `bool` — Coordinate input is active: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.IsCoordinateInputEnabled)

#### `IsEmptyValueInputControl() -> bool`

Check, whether there is no input inside the input control

**Remarks:** Check, whether there is no input inside the input control

**Returns:** `bool` — Input control is empty: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.IsEmptyValueInputControl)

#### `IsIdentElementAllowed(bAllowCenter: bool) -> bool`

Check, whether the result of the point identification allows an element identification

**Remarks:** Check, whether the result of the point identification allows an element identification

**Parameters:**
- `bAllowCenter` (bool) — Allow element identification by center point

**Returns:** `bool` — Element identification allowed: true false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.IsIdentElementAllowed)

#### `IsIdentModeOriginal() -> bool`

Check, whether the current identification mode is the original mode

**Remarks:** Check, whether the current identification mode is the original mode

**Returns:** `bool` — Current identification mode is the original mode

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.IsIdentModeOriginal)

#### `IsIdentModePointWallPoint() -> bool`

Check, whether the identification mode is point and wall point

**Remarks:** Check, whether the identification mode is point and wall point

**Returns:** `bool` — Identification mode is point and wall point: true false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.IsIdentModePointWallPoint)

#### `IsIdentModeWallPoint() -> bool`

Check, whether the identification mode is wall point

**Remarks:** Check, whether the identification mode is wall point

**Returns:** `bool` — Identification mode is wall point: true false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.IsIdentModeWallPoint)

#### `IsIdentPoint() -> bool`

Get the identification state of the current point

**Remarks:** Get the identification state of the current point

**Returns:** `bool` — Identification point: true false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.IsIdentPoint)

#### `IsMouseLeave() -> bool`

Check, whether the mouse is outside the view

**Remarks:** Check, whether the mouse is outside the view

**Returns:** `bool` — Mouse is outside the view: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.IsMouseLeave)

#### `IsMouseMove(mouseMsg: int) -> bool`

Check on mouse move

**Remarks:** Check on mouse move

**Parameters:**
- `mouseMsg` (int) — Mouse message WM_xxx

**Returns:** `bool` — Mouse move: true false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.IsMouseMove)

#### `IsSelectedGeometryElement() -> bool`

Get the state of the selected geometry element

**Remarks:** Get the state of the selected geometry element

**Returns:** `bool` — Geometry element is selected: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.IsSelectedGeometryElement)

#### `IsValueInputControl(id: int) -> bool | IsValueInputControl() -> bool`

Check, whether the ID belongs to the value input control (from the coordinate input dialog)

**Remarks:** Check, whether the ID belongs to the value input control (from the coordinate input dialog)

**Parameters:**
- `id` (int) — ID to check

**Returns:** `bool` — ID belongs to the value input control (from the coordinate input dialog): true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.IsValueInputControl)

#### `IsValueInputControlInput(bIdentPoint: bool) -> bool`

Check, whether an input inside the value input control is done and the value should be used

**Remarks:** Check, whether an input inside the value input control is done and the value should be used

**Parameters:**
- `bIdentPoint` (bool) — Identification point has higher priority: true/false

**Returns:** `bool` — Check, whether the input value inside the added control

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.IsValueInputControlInput)

#### `ProcessMouseMsg(mouseMsg: int, pnt: Point2D, pMsgInfo: AddMsgInfo) -> object`

Process a mouse message

**Remarks:** Process a mouse message

**Parameters:**
- `mouseMsg` (int) — Mouse message WM_xxx
- `pnt` (Point2D) — Cursor point (view coordinate)
- `pMsgInfo` (AddMsgInfo) — Additional message info

**Returns:** `object` — Standard return

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.ProcessMouseMsg)

#### `SelectElement( mouseMsg: int, pnt: Point2D, pMsgInfo: AddMsgInfo, bHighlight: bool, bSelAlways: bool, bAllowCenter: bool, ) -> bool | SelectElement( mouseMsg: int, pnt: Point2D, pMsgInfo: AddMsgInfo, bHighlight: bool, bSelAlways: bool, bAllowCenter: bool, selectSetting: ElementSelectFilterSetting, ) -> bool`

Perform element search Uses the filter set by the SetElementFilter

**Remarks:** Perform element search Uses the filter set by the SetElementFilter

**Parameters:**
- `mouseMsg` (int) — Mouse message
- `pnt` (Point2D) — Cursor point (view coordinate)
- `pMsgInfo` (AddMsgInfo) — Additional message info
- `bHighlight` (bool) — Highlight the selected element
- `bSelAlways` (bool) — Relevant when used in combination with GetInputPoint method. When set to False, the element search is terminated after the method GetInputPoint found a start/end/mid point of an element. When set to True, the element search will be performed anyway.
- `bAllowCenter` (bool) — Relevant when used in combination with GetInputPoint method. When bSelAlways is set to False and this option to True, the element search is performed after the method GetInputPoint found a mid point. When bSelAlways is set to True, this option becomes irrelevant.

**Returns:** `bool` — True, when element is selected. False otherwise

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.SelectElement)

#### `SelectGeometryElement( mouseMsg: int, pnt: Point2D, pMsgInfo: AddMsgInfo, bHighlightCompleteElement: bool = False, ) -> bool`

Select a base geometry element. Use the filter set by SetGeometryElementFilter and SetGeometryFilter

**Remarks:** Select a base geometry element. Use the filter set by SetGeometryElementFilter and SetGeometryFilter

**Parameters:**
- `mouseMsg` (int) — Mouse message WM_xxx
- `pnt` (Point2D) — Cursor point (view coordinate)
- `pMsgInfo` (AddMsgInfo) — Additional message info
- `bHighlightCompleteElement` (bool) — true = highlight the complete element, false = highlight only the selected geometry part

**Returns:** `bool` — Element was found: true false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.SelectGeometryElement)

#### `SelectPolyhedronFace( arg2: BaseElementAdapter, arg3: Point2D, arg4: bool ) -> tuple`

.. deprecated:: since Allplan 2023-1-0 use FaceSelectService::SelectPolyhedronFace

**Remarks:** .. deprecated:: since Allplan 2023-1-0 use FaceSelectService::SelectPolyhedronFace

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.SelectPolyhedronFace)

#### `SelectWallFace(arg2: BaseElementAdapter, arg3: Point2D, arg4: bool) -> tuple`

.. deprecated:: since Allplan 2023-1-0 use FaceSelectService::SelectWallFace

**Remarks:** .. deprecated:: since Allplan 2023-1-0 use FaceSelectService::SelectWallFace

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.SelectWallFace)

#### `SelectWallFaceInUVS( arg2: BaseElementAdapter, arg3: Point2D, arg4: bool ) -> tuple`

.. deprecated:: since Allplan 2023-1-0 use FaceSelectService::SelectWallFaceInUVS

**Remarks:** .. deprecated:: since Allplan 2023-1-0 use FaceSelectService::SelectWallFaceInUVS

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.SelectWallFaceInUVS)

#### `SetAbscissaElement(abscissa: object, abscissaAssoMat: Matrix3D)`

Set the abscissa element

**Remarks:** Set the abscissa element

**Parameters:**
- `abscissa` (object) — Abscissa
- `abscissaAssoMat` (Matrix3D) — Abscissa matrix of the associative view

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.SetAbscissaElement)

#### `SetElementFilter(selectSetting: ElementSelectFilterSetting)`

Set the element selection filter

**Remarks:** Set the element selection filter

**Parameters:**
- `selectSetting` (ElementSelectFilterSetting) — Element selection filter

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.SetElementFilter)

#### `SetGeometryElementFilter(selectSetting: ElementSelectFilterSetting)`

Set the geometry element selection filter

**Remarks:** Set the geometry element selection filter

**Parameters:**
- `selectSetting` (ElementSelectFilterSetting) — Geometry element selection filter

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.SetGeometryElementFilter)

#### `SetGeometryFilter(geoFilter: SnoopElementGeometryFilter)`

Set the geometry filter used for geometry search during point input (initialized with InitFirstPointInput or InitNextPointInput). This filter is used, when the identification mode (eIdentificationMode) is set to identify points and geometry elements, i.e. to eIDENT_POINT_ELEMENT...

**Remarks:** Set the geometry filter used for geometry search during point input (initialized with InitFirstPointInput or InitNextPointInput). This filter is used, when the identification mode (eIdentificationMode) is set to identify points and geometry elements, i.e. to eIDENT_POINT_ELEMENT...

**Parameters:**
- `selectSetting` (object) — Geometry element selection filter

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.SetGeometryFilter)

#### `SetInputPlane(plane: Plane3D)`

Set the input plane. The input point will be projected onto this plane along the plane's normal vector.

**Remarks:** Set the input plane. The input point will be projected onto this plane along the plane's normal vector.

**Parameters:**
- `plane` (Plane3D) — Input plane

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.SetInputPlane)

#### `SetInputText(text: InputStringConvert)`

Set the input text

**Remarks:** Set the input text

**Parameters:**
- `text` (InputStringConvert) — Request string as resource ID, str

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.SetInputText)

#### `SetInputTextPrefix(iDPrefix: int)`

Set the input text prefix

**Remarks:** Set the input text prefix

**Parameters:**
- `iDPrefix` (int) — ID of the prefix text

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.SetInputTextPrefix)

#### `SetProjectionBase0(setProjectionBase0: bool)`

Set the projection base to 0 during point input in floor plan view or side view When this option is set to True, then if a point is found and snapped during the input inside a floor plan view or a side view (front, rear, left or right), the resulting point will be projected on: When this option is set to False, then the resulting point is not projected onto those planes. Instead it is being snapped to the first visible point from the observer's perspective. Default value is: False

**Remarks:** Set the projection base to 0 during point input in floor plan view or side view When this option is set to True, then if a point is found and snapped during the input inside a floor plan view or a side view (front, rear, left or right), the resulting point will be projected on: When this option is set to False, then the resulting point is not projected onto those planes. Instead it is being snapped to the first visible point from the observer's perspective. Default value is: False

**Parameters:**
- `setProjectionBase0` (bool) — True, to set the projection base to 0

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInput/#NemAll_Python_IFW_Input.CoordinateInput.SetProjectionBase0)

## CoordinateInputMode (class)

Settings regarding mode, in which elements and/or points should be identified during point input

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInputMode/)

### Constructors
- `CoordinateInputMode() | CoordinateInputMode( identMode: eIdentificationMode, drawPointSymbol: eDrawElementIdentPointSymbols = eDRAW_IDENT_ELEMENT_POINT_SYMBOL_YES, ) | CoordinateInputMode(identMode: eIdentificationMode, bLocalCoordInput: bool) | CoordinateInputMode(element: CoordinateInputMode)` — Initialize

### Methods
#### `GetIdentMode() -> eIdentificationMode`

Get the identification mode

**Remarks:** Get the identification mode

**Returns:** `eIdentificationMode` — Identification mode

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInputMode/#NemAll_Python_IFW_Input.CoordinateInputMode.GetIdentMode)

#### `IsLocalCoordInput() -> bool`

Get the local coordinate input state

**Remarks:** Get the local coordinate input state

**Returns:** `bool` — Local coordinate input state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInputMode/#NemAll_Python_IFW_Input.CoordinateInputMode.IsLocalCoordInput)

#### `SetLocalCoordInput(bLocalCoordInput: bool)`

Set the local coordinate input state

**Remarks:** Set the local coordinate input state

**Parameters:**
- `bLocalCoordInput` (bool) — Only local coordinate input: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInputMode/#NemAll_Python_IFW_Input.CoordinateInputMode.SetLocalCoordInput)

## CoordinateInputResult (class)

Data class containing result of a successful point input

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInputResult/)

### Constructors
- `CoordinateInputResult()` — Initialize

### Methods
#### `GetPoint() -> Point3D`

Get the 3D point identified during a point input in world coordinates

**Remarks:** Get the 3D point identified during a point input in world coordinates

**Returns:** `Point3D` — Identified point

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/CoordinateInputResult/#NemAll_Python_IFW_Input.CoordinateInputResult.GetPoint)

## ElementHandleType (class)

Element handle type

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementHandleType/)

### Constructors
- `ElementHandleType()` — Initialize

### Methods
#### `__repr__() -> str`

Convert the list to a string

**Remarks:** Convert the list to a string

**Returns:** `str` — List values as string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementHandleType/#NemAll_Python_IFW_Input.ElementHandleType.__repr__)

### Properties
- `HANDLE_ARROW` (ElementHandleType, get) — Arrow handle
- `HANDLE_CIRCLE` (ElementHandleType, get) — Circle handle
- `HANDLE_SQUARE_BLUE` (ElementHandleType, get) — Filled square handle with blue color
- `HANDLE_SQUARE_EMPTY` (ElementHandleType, get) — Empty square handle
- `HANDLE_SQUARE_RED` (ElementHandleType, get) — Filled square handle with red color
- `HANDLE_SQUARE_RIGHT` (ElementHandleType, get) — Halve-filled square handle

## ElementSelect (class)

Implementation of the ElementSelect wrapper

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelect/)

### Constructors
- `ElementSelect()` — Initialize

### Methods
#### `InitSelection(text: InputStringConvert)`

Initialize the selection

**Remarks:** Initialize the selection

**Parameters:**
- `text` (InputStringConvert) — Input text

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelect/#NemAll_Python_IFW_Input.ElementSelect.InitSelection)

#### `IsMouseMove(mouseMsg: int) -> bool`

Check for mouse move

**Remarks:** Check for mouse move

**Parameters:**
- `mouseMsg` (int) — Mouse message

**Returns:** `bool` — Message is a mouse move: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelect/#NemAll_Python_IFW_Input.ElementSelect.IsMouseMove)

## ElementSelectFilterSetting (class)

Class containing settings for filtering elements during element selection, such as:

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelectFilterSetting/)

### Constructors
- `ElementSelectFilterSetting() | ElementSelectFilterSetting(bSnoopAllElements: bool) | ElementSelectFilterSetting(filter: SelectionQuery, bSnoopAllElements: bool) | ElementSelectFilterSetting( filter: SelectionQuery, documentSnoopType: eDocumentSnoopType = eSnoopActiveDocuments, layerSnoopType: eLayerSnoopType = eSnoopActiveLayers, ) | ElementSelectFilterSetting(selectSetting: ElementSelectFilterSetting)` — Construct the filter with default settings:

### Methods
#### `Clear()`

Reset the settings

**Remarks:** Reset the settings

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelectFilterSetting/#NemAll_Python_IFW_Input.ElementSelectFilterSetting.Clear)

#### `GetLayerSelectType() -> eLayerSnoopType`

Get the layer selection type

**Remarks:** Get the layer selection type

**Returns:** `eLayerSnoopType` — Return the layer selection type

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelectFilterSetting/#NemAll_Python_IFW_Input.ElementSelectFilterSetting.GetLayerSelectType)

#### `IsBaseClassType(typeID: GUID) -> bool`

Check, whether the element type is a base class type

**Remarks:** Check, whether the element type is a base class type

**Parameters:**
- `typeID` (GUID) — Element type ID

**Returns:** `bool` — Element type is a base class type: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelectFilterSetting/#NemAll_Python_IFW_Input.ElementSelectFilterSetting.IsBaseClassType)

#### `IsClear() -> bool`

Get the clear state

**Remarks:** Get the clear state

**Returns:** `bool` — true, if the members contain default values

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelectFilterSetting/#NemAll_Python_IFW_Input.ElementSelectFilterSetting.IsClear)

#### `IsPointSelect() -> bool`

Get the point select state

**Remarks:** Get the point select state

**Returns:** `bool` — Point selection is active: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelectFilterSetting/#NemAll_Python_IFW_Input.ElementSelectFilterSetting.IsPointSelect)

#### `IsSelectPassiveInfoElement() -> bool`

Get the selection state of a passive info element

**Remarks:** Get the selection state of a passive info element

**Returns:** `bool` — Allow to select passive info element if no active element was found: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelectFilterSetting/#NemAll_Python_IFW_Input.ElementSelectFilterSetting.IsSelectPassiveInfoElement)

#### `SelectPassiveInfoElement()`

Allow to select passive info element if no active element was found

**Remarks:** Allow to select passive info element if no active element was found

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelectFilterSetting/#NemAll_Python_IFW_Input.ElementSelectFilterSetting.SelectPassiveInfoElement)

#### `SetArchitectureFilterQuery(positive: bool = True)`

Set a prefabricated filter for all architecture elements

**Remarks:** Set a prefabricated filter for all architecture elements

**Parameters:**
- `positive` (bool) — if the filter will true for architecture elements otherwise the filter will be false for architecture elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelectFilterSetting/#NemAll_Python_IFW_Input.ElementSelectFilterSetting.SetArchitectureFilterQuery)

#### `SetAssoFilterQuery(positive: bool = True)`

Set a prefabricated filter for all associative view elements

**Remarks:** Set a prefabricated filter for all associative view elements

**Parameters:**
- `positive` (bool) — if the filter will true for associative view elements otherwise the filter will be false for associative view elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelectFilterSetting/#NemAll_Python_IFW_Input.ElementSelectFilterSetting.SetAssoFilterQuery)

#### `SetDocumentLayerFilter(bSnoopAllElements: bool)`

Set the document and layer filter

**Remarks:** Set the document and layer filter

**Parameters:**
- `bSnoopAllElements` (bool) — Snoop all elements: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelectFilterSetting/#NemAll_Python_IFW_Input.ElementSelectFilterSetting.SetDocumentLayerFilter)

#### `SetDocumentSelectType(documentSnoopType: eDocumentSnoopType)`

Set the selection mode for the document (active, passive or all documents)

**Remarks:** Set the selection mode for the document (active, passive or all documents)

**Parameters:**
- `documentSnoopType` (eDocumentSnoopType) — Selection mode for the document (active, passive or all documents)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelectFilterSetting/#NemAll_Python_IFW_Input.ElementSelectFilterSetting.SetDocumentSelectType)

#### `SetLayerSelectType(layerSnoopType: eLayerSnoopType)`

Set the layer selection type

**Remarks:** Set the layer selection type

**Parameters:**
- `layerSnoopType` (eLayerSnoopType) — Type of the allowed layers for the selection

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelectFilterSetting/#NemAll_Python_IFW_Input.ElementSelectFilterSetting.SetLayerSelectType)

#### `SetPointSelect()`

Set the point select state

**Remarks:** Set the point select state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ElementSelectFilterSetting/#NemAll_Python_IFW_Input.ElementSelectFilterSetting.SetPointSelect)

### Properties
- `LayerSelectType` (eLayerSnoopType, get/set) — Get the layer selection type

## Functions (static-class)

Module-level functions of NemAll_Python_IFW_Input

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/)

## HandleService (class)

Implementation of the handle service

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/HandleService/)

### Constructors
- `HandleService()` — Initialize

### Methods
#### `AddHandles( doc: DocumentAdapter, handlePropList: object, insertionMat: Matrix3D, assoRefObj: object, )`

Add the handles

**Remarks:** Add the handles

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `handlePropList` (object) — Handle properties list
- `insertionMat` (Matrix3D) — Transformation matrix
- `assoRefObj` (object) — Reference element for the drawing inside the associative views

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/HandleService/#NemAll_Python_IFW_Input.HandleService.AddHandles)

#### `DeleteToolTipText()`

Delete the tool tip text

**Remarks:** Delete the tool tip text

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/HandleService/#NemAll_Python_IFW_Input.HandleService.DeleteToolTipText)

#### `DrawHandles()`

Draw the handles

**Remarks:** Draw the handles

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/HandleService/#NemAll_Python_IFW_Input.HandleService.DrawHandles)

#### `RemoveHandles()`

Remove the handles

**Remarks:** Remove the handles

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/HandleService/#NemAll_Python_IFW_Input.HandleService.RemoveHandles)

#### `SelectHandle( pnt: Point2D, viewProj: ViewWorldProjection ) -> tuple[int, Matrix3D]`

Select a handle

**Remarks:** Select a handle

**Parameters:**
- `pnt` (Point2D) — Cursor point
- `viewProj` (ViewWorldProjection) — View world projection

**Returns:** `tuple[int, Matrix3D]` — Handle index (-1 = no selection) , associative view to world matrix

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/HandleService/#NemAll_Python_IFW_Input.HandleService.SelectHandle)

#### `ShowToolTipText(text: str)`

Show the tool tip text

**Remarks:** Show the tool tip text

**Parameters:**
- `text` (str) — Text

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/HandleService/#NemAll_Python_IFW_Input.HandleService.ShowToolTipText)

## HighlightService (class)

(No description provided in vendor docs for NemAll_Python_IFW_Input.HighlightService.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/HighlightService/)

### Methods
#### `CancelAllHighlightedElements(documentID: int)`

Cancel the highlight of all elements

**Remarks:** Cancel the highlight of all elements

**Parameters:**
- `documentID` (int) — document ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/HighlightService/#NemAll_Python_IFW_Input.HighlightService.CancelAllHighlightedElements)

#### `HighlightElements(eleList: BaseElementAdapterList)`

Highlight the elements

**Remarks:** Highlight the elements

**Parameters:**
- `eleList` (BaseElementAdapterList) — Element list as BaseElementAdapterList

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/HighlightService/#NemAll_Python_IFW_Input.HighlightService.HighlightElements)

## InputFunctionStarter (class)

Utility with methods to start and terminate multiple element selection (selection with a rectangle)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/InputFunctionStarter/)

### Methods
#### `RemoveFunction()`

Remove the current input function from the input function stack. This method can be called only, if there is a selection function in the input function stack (started by calling e.g. StartElementSelect)

**Remarks:** Remove the current input function from the input function stack. This method can be called only, if there is a selection function in the input function stack (started by calling e.g. StartElementSelect)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/InputFunctionStarter/#NemAll_Python_IFW_Input.InputFunctionStarter.RemoveFunction)

#### `StartElementSelect( text: str, selectSetting: ElementSelectFilterSetting, postSel: PostElementSelection, markSelectedElements: bool, selectionMode: SelectionMode = eSelectGeometry, )`

Start the multiple element selection. In this mode elements can be selected in the viewport by a selection rectangle or polygonal fence. The user can also point out individual elements by activating a bracket (right-click before and after selection) The element selection overloads the process_mouse_msg, which means that this event is not called until the selection is completed. This is the case, when at least one of the following applies: After that, the result is saved in the PostElementSelection object and the function is removed from the function stack. IMPORTANT: There can only be one selection function in the input function stack! If the running element selection must be restarted, call RemoveFunction first!

**Remarks:** Start the multiple element selection. In this mode elements can be selected in the viewport by a selection rectangle or polygonal fence. The user can also point out individual elements by activating a bracket (right-click before and after selection) The element selection overloads the process_mouse_msg, which means that this event is not called until the selection is completed. This is the case, when at least one of the following applies: After that, the result is saved in the PostElementSelection object and the function is removed from the function stack. IMPORTANT: There can only be one selection function in the input function stack! If the running element selection must be restarted, call RemoveFunction first!

**Parameters:**
- `text` (str) — Prompt message shown to the user in the Allplan dialog line
- `selectSetting` (ElementSelectFilterSetting) — Filter setting
- `postSel` (PostElementSelection) — Object to store the result of the selection. The result will be saved after a successful selection (see cases above)
- `markSelectedElements` (bool) — Whether to mark the selected elements
- `selectionMode` (SelectionMode) — Selection mode

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/InputFunctionStarter/#NemAll_Python_IFW_Input.InputFunctionStarter.StartElementSelect)

## InputStringConvert (class)

Representation of the prompt shown in Allplan UI in the so called dialog line during an input (selection or point input)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/InputStringConvert/)

### Constructors
- `InputStringConvert(id: int) | InputStringConvert(text: str)` — Construct the prompt from an ID pointing out to a built-in string

### Methods
#### `GetString() -> str`

Get the input string

**Remarks:** Get the input string

**Returns:** `str` — Input string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/InputStringConvert/#NemAll_Python_IFW_Input.InputStringConvert.GetString)

## InputViewData (class)

(No description provided in vendor docs for NemAll_Python_IFW_Input.InputViewData.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/InputViewData/)

### Constructors
- `InputViewData()` — Initialize

## InputViewDocumentData (class)

Implementation of the functions for the input view and document data

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/InputViewDocumentData/)

### Methods
#### `EnableAssistWndClick(bEnable: bool)`

Enable/disable a click inside the assist window

**Remarks:** Enable/disable a click inside the assist window

**Parameters:**
- `bEnable` (bool) — Enable a click inside the assist window: 1/0

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/InputViewDocumentData/#NemAll_Python_IFW_Input.InputViewDocumentData.EnableAssistWndClick)

#### `GetActiveViewDocument() -> DocumentAdapter`

Get the active view document

**Remarks:** Get the active view document

**Returns:** `DocumentAdapter` — Active view document

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/InputViewDocumentData/#NemAll_Python_IFW_Input.InputViewDocumentData.GetActiveViewDocument)

#### `GetInputViewDocument() -> DocumentAdapter`

Get the input view document

**Remarks:** Get the input view document

**Returns:** `DocumentAdapter` — Input view document

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/InputViewDocumentData/#NemAll_Python_IFW_Input.InputViewDocumentData.GetInputViewDocument)

#### `GetInputViewDocumentID() -> int`

Get the document ID of the current input view.

**Remarks:** Get the document ID of the current input view.

**Returns:** `int` — Document ID of the current input view

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/InputViewDocumentData/#NemAll_Python_IFW_Input.InputViewDocumentData.GetInputViewDocumentID)

#### `GetViewWorldProjection() -> ViewWorldProjection`

Get the view-world projection object

**Remarks:** Get the view-world projection object

**Returns:** `ViewWorldProjection` — View-world projection object

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/InputViewDocumentData/#NemAll_Python_IFW_Input.InputViewDocumentData.GetViewWorldProjection)

## LCS_Flags (enum)

eStandard: standard coordinate system symbol eSmall : small coordinate system symbol eHoverX : the x arrow will be black eHoverY : the y arrow will be black eHoverZ : the z arrow will be black

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/LCS_Flags/)

### Values
- `eHoverX` = `2`
- `eHoverY` = `4`
- `eHoverZ` = `8`
- `eSmall` = `1`
- `eStandard` = `0`

## PolygonInput (class)

Implementation of the polygon input

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PolygonInput/)

### Constructors
- `PolygonInput(coordInput: CoordinateInput, bZCoord: bool, multiPolygon: bool)` — Default constructor After constructing the object, the polygon input toolbar is shown in the Allplan UI

### Methods
#### `ExecuteInput(mouseMsg: int, pnt: Point2D, pMsgInfo: AddMsgInfo) -> int`

Execute the input

**Remarks:** Execute the input

**Parameters:**
- `mouseMsg` (int) — Mouse message
- `pnt` (Point2D) — View input point
- `pMsgInfo` (AddMsgInfo) — Additional message info

**Returns:** `int` — execution state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PolygonInput/#NemAll_Python_IFW_Input.PolygonInput.ExecuteInput)

#### `GetPolygon() -> Polygon3D`

get the final polygon

**Remarks:** get the final polygon

**Returns:** `Polygon3D` — final polygon

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PolygonInput/#NemAll_Python_IFW_Input.PolygonInput.GetPolygon)

#### `GetPreviewPolygon() -> Polygon3D`

get the preview polygon

**Remarks:** get the preview polygon

**Returns:** `Polygon3D` — preview polygon

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PolygonInput/#NemAll_Python_IFW_Input.PolygonInput.GetPreviewPolygon)

#### `StartNewInput()`

Start new input

**Remarks:** Start new input

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PolygonInput/#NemAll_Python_IFW_Input.PolygonInput.StartNewInput)

## PolylineInput (class)

Implementation of the polyline input

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PolylineInput/)

### Constructors
- `PolylineInput(coordInput: CoordinateInput, bZCoord: bool)` — Default constructor After constructing the object, the line input toolbar is shown in the Allplan UI

### Methods
#### `ExecuteInput(mouseMsg: int, pnt: Point2D, pMsgInfo: AddMsgInfo) -> int`

Execute the input

**Remarks:** Execute the input

**Parameters:**
- `mouseMsg` (int) — Mouse message
- `pnt` (Point2D) — View input point
- `pMsgInfo` (AddMsgInfo) — Additional message info

**Returns:** `int` — execution state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PolylineInput/#NemAll_Python_IFW_Input.PolylineInput.ExecuteInput)

#### `GetPolyline() -> Polyline3D`

get the final polyline

**Remarks:** get the final polyline

**Returns:** `Polyline3D` — final polyline

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PolylineInput/#NemAll_Python_IFW_Input.PolylineInput.GetPolyline)

#### `GetPreviewPolyline() -> Polyline3D`

get the preview polyline

**Remarks:** get the preview polyline

**Returns:** `Polyline3D` — preview polyline

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PolylineInput/#NemAll_Python_IFW_Input.PolylineInput.GetPreviewPolyline)

#### `StartNewInput()`

Start new input

**Remarks:** Start new input

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PolylineInput/#NemAll_Python_IFW_Input.PolylineInput.StartNewInput)

## PostElementSelection (class)

Data class containing the result of an element selection started with the InputFunctionStarter

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PostElementSelection/)

### Constructors
- `PostElementSelection()` — Initialize

### Methods
#### `GetSelectedElements(doc: DocumentAdapter) -> BaseElementAdapterList`

Pop the elements out of the object and return them as a list with element adapters

**Remarks:** Pop the elements out of the object and return them as a list with element adapters

**Parameters:**
- `doc` (DocumentAdapter) — Document

**Returns:** `BaseElementAdapterList` — Selected elements as element adapters

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PostElementSelection/#NemAll_Python_IFW_Input.PostElementSelection.GetSelectedElements)

## PreviewSymbolBuilder (class)

Implementation of the preview symbol builder

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/)

### Methods
#### `ArrowSymbol( pnt: Point3D, bDrawIso: bool, viewProj: ViewWorldProjection, colorVariant: ARGB, widthVariant: int, rotationAngle: float, allWindows: bool = True, ) | ArrowSymbol( pnt: Point3D, bDrawIso: bool, viewProj: ViewWorldProjection, colorVariant: ARGB, rotationAngle: float, allWindows: bool = True, )`

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.ArrowSymbol)

#### `Circle3DSymbol( refPnt: Point3D, circle: Arc3D, viewProj: ViewWorldProjection, colorVariant: ARGB, linePattern: int, lineWidth: float, )`

Create a 3D circle symbol preview

**Remarks:** Create a 3D circle symbol preview

**Parameters:**
- `refPnt` (Point3D) — Reference point
- `circle` (Arc3D) — 3D circle
- `viewProj` (ViewWorldProjection) — View world projection data
- `colorVariant` (ARGB) — Color of the preview
- `linePattern` (int) — Line pattern
- `lineWidth` (float) — Width of the line

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.Circle3DSymbol)

#### `CircleSymbol( pnt: Point3D, bDrawIso: bool, viewProj: ViewWorldProjection, colorVariant: ARGB, radius: int, )`

Create a circle symbol

**Remarks:** Create a circle symbol

**Parameters:**
- `pnt` (Point3D) — Center point
- `bDrawIso` (bool) — Draw the circle symbol inside the isometric view: true/false
- `viewProj` (ViewWorldProjection) — View world projection data
- `colorVariant` (ARGB) — Color of the preview
- `radius` (int) — Radius of the circle in pixel

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.CircleSymbol)

#### `CoordCrossSymbol(plane: Plane3D, armLength: int, viewProj: ViewWorldProjection) | CoordCrossSymbol( axisPlacement: AxisPlacement3D, armLength: int, viewProj: ViewWorldProjection, )`

Draw the coordinate cross symbol

**Remarks:** Draw the coordinate cross symbol

**Parameters:**
- `plane` (Plane3D) — Plane
- `armLength` (int) — Length of the symbol arms
- `viewProj` (ViewWorldProjection) — View world projection

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.CoordCrossSymbol)

#### `CrossSymbol( pnt: Point3D, bDrawIso: bool, viewProj: ViewWorldProjection, colorVariant: ARGB, widthVariant: int, )`

Create a cross symbol

**Remarks:** Create a cross symbol

**Parameters:**
- `pnt` (Point3D) — Cross center point
- `bDrawIso` (bool) — Draw the cross symbol inside the isometric view: true/false
- `viewProj` (ViewWorldProjection) — View world projection data
- `colorVariant` (ARGB) — Color of the preview
- `widthVariant` (int) — Width of the symbol

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.CrossSymbol)

#### `FilledRectangleSymbol( pnt: Point3D, bDrawIso: bool, viewProj: ViewWorldProjection, colorVariant: ARGB, widthVariant: int, rotationAngle: float, )`

Create a filled rectangle symbol

**Remarks:** Create a filled rectangle symbol

**Parameters:**
- `pnt` (Point3D) — Rectangle center point
- `bDrawIso` (bool) — Draw the rectangle symbol inside the isometric view: true/false
- `viewProj` (ViewWorldProjection) — View world projection data
- `colorVariant` (ARGB) — Color of the preview
- `widthVariant` (int) — Width of the symbol
- `rotationAngle` (float) — Rotation angle of the rectangle

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.FilledRectangleSymbol)

#### `Line3DSymbol( refPnt: Point3D, line: Line3D, viewProj: ViewWorldProjection, colorVariant: ARGB, linePattern: int, lineWidth: int, )`

Create a 3D line symbol preview

**Remarks:** Create a 3D line symbol preview

**Parameters:**
- `refPnt` (Point3D) — Reference point
- `line` (Line3D) — 3D line
- `viewProj` (ViewWorldProjection) — View world projection data
- `colorVariant` (ARGB) — Color of the preview
- `linePattern` (int) — Line pattern
- `lineWidth` (int) — Line width

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.Line3DSymbol)

#### `LocalCoordinateSystem( coordSystemMatrix: Matrix3D, flags: LCS_Flags, maxSize: float )`

Create a symbol for a local coordinate system

**Remarks:** Create a symbol for a local coordinate system

**Parameters:**
- `coordSystemMatrix` (Matrix3D) — Matrix of the coordinate system
- `flags` (LCS_Flags) — Coordinate system flags
- `maxSize` (float) — Max size of the coordinate system

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.LocalCoordinateSystem)

#### `MarkSymbol( pnt: Point3D, bDrawIso: bool, viewProj: ViewWorldProjection, colorVariant: ARGB, widthVariant: int, )`

Create a mark symbol (drawn an x)

**Remarks:** Create a mark symbol (drawn an x)

**Parameters:**
- `pnt` (Point3D) — Mark center point
- `bDrawIso` (bool) — Draw the mark symbol inside the isometric view: true/false
- `viewProj` (ViewWorldProjection) — View world projection data
- `colorVariant` (ARGB) — Color of the preview
- `widthVariant` (int) — Width of the symbol

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.MarkSymbol)

#### `OffsetPointSymbols( refPnt: Point3D, offPnt: Point3D, bDrawIso: bool, viewProj: ViewWorldProjection, colorVariant: ARGB, widthVariant: int, refPntAngle: float, offPntAngle: float, )`

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

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.OffsetPointSymbols)

#### `OrthogonalSymbol( pnt: Point3D, bDrawIso: bool, viewProj: ViewWorldProjection, colorVariant: ARGB, widthVariant: int, )`

Create an orthogonal symbol

**Remarks:** Create an orthogonal symbol

**Parameters:**
- `pnt` (Point3D) — Center point
- `bDrawIso` (bool) — Draw the circle symbol inside the isometric view: true/false
- `viewProj` (ViewWorldProjection) — View world projection data
- `colorVariant` (ARGB) — Color of the preview
- `widthVariant` (int) — Width of the symbol

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.OrthogonalSymbol)

#### `ParallelSymbol( pnt: Point3D, bDrawIso: bool, viewProj: ViewWorldProjection, colorVariant: ARGB, widthVariant: int, )`

Create a parallel symbol

**Remarks:** Create a parallel symbol

**Parameters:**
- `pnt` (Point3D) — Center point
- `bDrawIso` (bool) — Draw the circle symbol inside the isometric view: true/false
- `viewProj` (ViewWorldProjection) — View world projection data
- `colorVariant` (ARGB) — Color of the preview
- `widthVariant` (int) — Width of the symbol

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.ParallelSymbol)

#### `Polyline2DSymbol( refPnt: Point3D, polyline: Polyline2D, viewProj: ViewWorldProjection, colorVariant: ARGB, linePattern: int, )`

Create a 2D polyline symbol preview

**Remarks:** Create a 2D polyline symbol preview

**Parameters:**
- `refPnt` (Point3D) — Reference point
- `polyline` (Polyline2D) — Polyline
- `viewProj` (ViewWorldProjection) — View world projection
- `colorVariant` (ARGB) — Color of the preview
- `linePattern` (int) — Line pattern

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.Polyline2DSymbol)

#### `Polyline3DSymbol( polyline: Polyline3D, viewProj: ViewWorldProjection, colorVariant: ARGB, linePattern: int, ) | Polyline3DSymbol( refPnt: Point3D, polyline: Polyline3D, viewProj: ViewWorldProjection, colorVariant: ARGB, linePattern: int, )`

Create a 3D polyline symbol preview

**Remarks:** Create a 3D polyline symbol preview

**Parameters:**
- `polyline` (Polyline3D) — 3D Polyline
- `viewProj` (ViewWorldProjection) — View world projection data
- `colorVariant` (ARGB) — Color of the preview
- `linePattern` (int) — Line pattern

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.Polyline3DSymbol)

#### `TrackLine( line: Line3D, bDrawIso: bool, viewProj: ViewWorldProjection, colorVariant: ARGB, trackLineType: eTrackLineType, )`

Create a track line

**Remarks:** Create a track line

**Parameters:**
- `line` (Line3D) — Track line
- `bDrawIso` (bool) — Draw the circle symbol inside the isometric view: true/false
- `viewProj` (ViewWorldProjection) — View world projection data
- `colorVariant` (ARGB) — Color of the preview
- `trackLineType` (eTrackLineType) — Type of the track line

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.TrackLine)

#### `TrackMarkSymbol( pnt: Point3D, bDrawIso: bool, viewProj: ViewWorldProjection, colorVariant: ARGB, widthVariant: int, )`

Create a track mark symbol

**Remarks:** Create a track mark symbol

**Parameters:**
- `pnt` (Point3D) — Center point
- `bDrawIso` (bool) — Draw the circle symbol inside the isometric view: true/false
- `viewProj` (ViewWorldProjection) — View world projection data
- `colorVariant` (ARGB) — Color of the preview
- `widthVariant` (int) — Width of the symbol

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/PreviewSymbolBuilder/#NemAll_Python_IFW_Input.PreviewSymbolBuilder.TrackMarkSymbol)

## QueryTypeID (class)

Implementation of the element type ID query

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/QueryTypeID/)

### Constructors
- `QueryTypeID() | QueryTypeID(typeID: GUID) | QueryTypeID(element: QueryTypeID)` — Initialize

### Methods
#### `GetQueryText() -> str`

Get the query text

**Remarks:** Get the query text

**Returns:** `str` — Query text

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/QueryTypeID/#NemAll_Python_IFW_Input.QueryTypeID.GetQueryText)

#### `GetQueryTypeID() -> GUID`

Get the ID of the type query

**Remarks:** Get the ID of the type query

**Returns:** `GUID` — ID of the type query

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/QueryTypeID/#NemAll_Python_IFW_Input.QueryTypeID.GetQueryTypeID)

#### `__call__(ele: BaseElementAdapter) -> bool`

Overloaded operator ()

**Remarks:** Overloaded operator ()

**Parameters:**
- `ele` (BaseElementAdapter) — Element to check

**Returns:** `bool` — Type of the element is same as the filter type: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/QueryTypeID/#NemAll_Python_IFW_Input.QueryTypeID.__call__)

#### `__repr__() -> str`

Convert to string

**Remarks:** Convert to string

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/QueryTypeID/#NemAll_Python_IFW_Input.QueryTypeID.__repr__)

## SelectElementsService (class)

(No description provided in vendor docs for NemAll_Python_IFW_Input.SelectElementsService.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/SelectElementsService/)

### Methods
#### `SelectByPolygon( doc: DocumentAdapter, polygon: Polygon2D, viewProjection: ViewWorldProjection, selCond: eSelectCondition, filter: SelectionQuery, isWorldPolygon: bool = False, ) -> BaseElementAdapterList`

Select an element by a point

**Remarks:** Select an element by a point

**Parameters:**
- `doc` (DocumentAdapter) — Document
- `polygon` (Polygon2D) — surrounding search polygon (view coordinates)
- `viewProjection` (ViewWorldProjection) — Identification of view projection (window)
- `selCond` (eSelectCondition) — True when elements inside rectangle must be selected only
- `filter` (SelectionQuery) — Selection filter
- `isWorldPolygon` (bool) — true=world polygon, false=view polygon

**Returns:** `BaseElementAdapterList` — Data of the selected elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/SelectElementsService/#NemAll_Python_IFW_Input.SelectElementsService.SelectByPolygon)

#### `SelectByRect( cursorLeftBottomPoint: Point2D, cursorRightTopPoint: Point2D, viewProjection: ViewWorldProjection, selCond: eSelectCondition, filter: SelectionQuery, ) -> BaseElementAdapterList`

Select an element by a point

**Remarks:** Select an element by a point

**Parameters:**
- `cursorLeftBottomPoint` (Point2D) — Coordinate of left bottom point where elements will be searching
- `cursorRightTopPoint` (Point2D) — Coordinate of right top point where elements will be searching
- `viewProjection` (ViewWorldProjection) — Identification of view projection (window)
- `selCond` (eSelectCondition) — True when elements inside rectangle must be selected only
- `filter` (SelectionQuery) — Selection filter

**Returns:** `BaseElementAdapterList` — Data of the selected elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/SelectElementsService/#NemAll_Python_IFW_Input.SelectElementsService.SelectByRect)

## SelectionMode (enum)

Possible selection modes for multiple selection using InputFunctionStarter

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/SelectionMode/)

### Values
- `eSelectGeometry` = `0` — Result of the selection is only the selected element
- `eSelectObject` = `1` — Result of the selection is the selected element with all its related elements (e.g. wall tier together with axis, labels, attribute container, etc...) and all child elements (if existing)
- `eSelectSubObject` = `3` — Result of the selection is the selected element with some related elements (e.g. wall tier with labels and attribute container)

## SelectionQuery (class)

Class containing checking procedures to obtain, whether an element is valid for being selected during element selection.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/SelectionQuery/)

### Constructors
- `SelectionQuery() | SelectionQuery(query: SelectionQuery) | SelectionQuery(query: list[QueryTypeID | SelectionQuery() | SelectionQuery() | SelectionQuery()` — Construct empty query. All elements are considered valid for selection

### Methods
#### `Clear()`

Clear the query

**Remarks:** Clear the query

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/SelectionQuery/#NemAll_Python_IFW_Input.SelectionQuery.Clear)

#### `IsEmpty() -> bool`

Check for an empty query

**Remarks:** Check for an empty query

**Returns:** `bool` — Filter is empty: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/SelectionQuery/#NemAll_Python_IFW_Input.SelectionQuery.IsEmpty)

#### `__call__(arg2: BaseElementAdapter) -> bool`



[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/SelectionQuery/#NemAll_Python_IFW_Input.SelectionQuery.__call__)

## SnoopElementGeometryFilter (class)

(No description provided in vendor docs for NemAll_Python_IFW_Input.SnoopElementGeometryFilter.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/SnoopElementGeometryFilter/)

### Constructors
- `SnoopElementGeometryFilter() | SnoopElementGeometryFilter( bFindBaseGeometry: bool, bFindAreaGeometry: bool, bPerpendicularOnElement: bool, bFindNonPassiveOnly: bool, bSplitAreaGeometries: bool, bIdentifyEmbeddedElement: bool, bFindCompleteFootprint: bool, splitElement3D: eSplitElement3D, )` — Initialize

### Methods
#### `AddElements(elementNames: str)`

Add elements to the filter

**Remarks:** Add elements to the filter

**Parameters:**
- `elementNames` (str) — Element names, separated by ,

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/SnoopElementGeometryFilter/#NemAll_Python_IFW_Input.SnoopElementGeometryFilter.AddElements)

## UndoRedoService (class)

Implementation of the undo/redo service

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/UndoRedoService/)

### Constructors
- `UndoRedoService( doc: DocumentAdapter, isPassivateAll: bool = False, isLockPreviewDraw: bool = False, collectEleForMultipleTransactions: bool = False, ) | UndoRedoService(element: UndoRedoService)` — Constructor

### Methods
#### `ActivateCollectedElementsForFinishUpdate()`

activate collected elements from undo step, important for next finish update

**Remarks:** activate collected elements from undo step, important for next finish update

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/UndoRedoService/#NemAll_Python_IFW_Input.UndoRedoService.ActivateCollectedElementsForFinishUpdate)

#### `CollectElementsForMultipleTransactions()`

Collect the elements for adding multiple transactions to one undo step

**Remarks:** Collect the elements for adding multiple transactions to one undo step

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/UndoRedoService/#NemAll_Python_IFW_Input.UndoRedoService.CollectElementsForMultipleTransactions)

#### `CreateUndoStep(eventID: int) | CreateUndoStep()`

Create an undo step

**Remarks:** Create an undo step

**Parameters:**
- `eventID` (int) — Event ID of the undo step

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/UndoRedoService/#NemAll_Python_IFW_Input.UndoRedoService.CreateUndoStep)

#### `IsInUndoService() -> bool`

Check if an active undo service is present

**Remarks:** Check if an active undo service is present

**Returns:** `bool` — Active undo service is present

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/UndoRedoService/#NemAll_Python_IFW_Input.UndoRedoService.IsInUndoService)

#### `ResetTransactionError()`

Reset the transaction error

**Remarks:** Reset the transaction error

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/UndoRedoService/#NemAll_Python_IFW_Input.UndoRedoService.ResetTransactionError)

#### `SetUndoDescription(textID: int)`

Set the undo step description independent from menu event text

**Remarks:** Set the undo step description independent from menu event text

**Parameters:**
- `textID` (int) — Text ID of the undo step

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/UndoRedoService/#NemAll_Python_IFW_Input.UndoRedoService.SetUndoDescription)

#### `SetUndoStepEvent(eventID: int)`

Set the undo step event

**Remarks:** Set the undo step event

**Parameters:**
- `eventID` (int) — Event ID of the undo step

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/UndoRedoService/#NemAll_Python_IFW_Input.UndoRedoService.SetUndoStepEvent)

## ValueInputControlData (class)

Data class with settings of the input control, that can be optionally displayed in the dialog line e.g. along with the XYZ coordinate input fields.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ValueInputControlData/)

### Constructors
- `ValueInputControlData() | ValueInputControlData( ctrlType: eValueInputControlType, bSetFocus: bool = True, bDisableCoord: bool = True, ) | ValueInputControlData( ctrlType: eValueInputControlType, minValue: float, maxValue: float, bSetFocus: bool, bDisableCoord: bool, ) | ValueInputControlData( ctrlType: eValueInputControlType, minValue: int, maxValue: int, bSetFocus: bool, bDisableCoord: bool, ) | ValueInputControlData( ctrlType: eValueInputControlType, initValue: float, minValue: float, maxValue: float, bSetFocus: bool, bDisableCoord: bool, ) | ValueInputControlData( ctrlType: eValueInputControlType, initValue: int, minValue: int, maxValue: int, bSetFocus: bool, bDisableCoord: bool, ) | ValueInputControlData( ctrlType: eValueInputControlType, initValue: float, bSetFocus: bool, bDisableCoord: bool, ) | ValueInputControlData( ctrlType: eValueInputControlType, initText: object, minTextLen: int, maxTextLen: int, bSetFocus: bool = True, bDisableCoord: bool = False, ) | ValueInputControlData(element: ValueInputControlData)` — Initialize

### Methods
#### `AddShortcutControl(ctrlType: eValueInputControlType)`

Add a shortcut control to the coordinate input dialog

**Remarks:** Add a shortcut control to the coordinate input dialog

**Parameters:**
- `ctrlType` (eValueInputControlType) — Type of the shortcut control

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ValueInputControlData/#NemAll_Python_IFW_Input.ValueInputControlData.AddShortcutControl)

#### `GetControlType() -> eValueInputControlType`

Get the type of the input control

**Remarks:** Get the type of the input control

**Returns:** `eValueInputControlType` — Type of the input control

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ValueInputControlData/#NemAll_Python_IFW_Input.ValueInputControlData.GetControlType)

#### `GetInitText() -> object`

Get the init input text

**Remarks:** Get the init input text

**Returns:** `object` — Init input text

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ValueInputControlData/#NemAll_Python_IFW_Input.ValueInputControlData.GetInitText)

#### `GetInitValue() -> float`

Get the init input value

**Remarks:** Get the init input value

**Returns:** `float` — Init input value

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ValueInputControlData/#NemAll_Python_IFW_Input.ValueInputControlData.GetInitValue)

#### `GetMaxTextLen() -> int`

Get the maximal length of the input text

**Remarks:** Get the maximal length of the input text

**Returns:** `int` — Maximal length of the input text

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ValueInputControlData/#NemAll_Python_IFW_Input.ValueInputControlData.GetMaxTextLen)

#### `GetMaxValue() -> float`

Get the maximal input value

**Remarks:** Get the maximal input value

**Returns:** `float` — Maximal input value

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ValueInputControlData/#NemAll_Python_IFW_Input.ValueInputControlData.GetMaxValue)

#### `GetMinTextLen() -> int`

Get the minimal length of the input text

**Remarks:** Get the minimal length of the input text

**Returns:** `int` — Minimal length of the input text

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ValueInputControlData/#NemAll_Python_IFW_Input.ValueInputControlData.GetMinTextLen)

#### `GetMinValue() -> float`

Get the minimal input value

**Remarks:** Get the minimal input value

**Returns:** `float` — Minimal input value

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ValueInputControlData/#NemAll_Python_IFW_Input.ValueInputControlData.GetMinValue)

#### `GetShortcutControlType() -> eValueInputControlType`

Get the type of the shortcut input control

**Remarks:** Get the type of the shortcut input control

**Returns:** `eValueInputControlType` — Type of the shortcut input control

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ValueInputControlData/#NemAll_Python_IFW_Input.ValueInputControlData.GetShortcutControlType)

#### `IsDisableCoord() -> bool`

Get the disable coordinate input state

**Remarks:** Get the disable coordinate input state

**Returns:** `bool` — Disable the coordinate input in case of value input: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ValueInputControlData/#NemAll_Python_IFW_Input.ValueInputControlData.IsDisableCoord)

#### `IsEmptyControl() -> bool`

Get the empty input control state

**Remarks:** Get the empty input control state

**Returns:** `bool` — Set an empty value to the input control: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ValueInputControlData/#NemAll_Python_IFW_Input.ValueInputControlData.IsEmptyControl)

#### `IsSetFocus() -> bool`

Get the input focus state

**Remarks:** Get the input focus state

**Returns:** `bool` — Set the input focus to the control: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ValueInputControlData/#NemAll_Python_IFW_Input.ValueInputControlData.IsSetFocus)

## ViewWorldProjection (class)

Implementation of the view - world projection

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/)

### Constructors
- `ViewWorldProjection() | ViewWorldProjection(element: ViewWorldProjection)` — Initialize

### Methods
#### `CreateForUnitTest(doc: DocumentAdapter) -> ViewWorldProjection`

Create the object for a unit test

**Remarks:** Create the object for a unit test

**Parameters:**
- `doc` (DocumentAdapter) — Document

**Returns:** `ViewWorldProjection` — created object

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.CreateForUnitTest)

#### `GetDocumentID() -> int`

Get the document ID

**Remarks:** Get the document ID

**Returns:** `int` — Document ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.GetDocumentID)

#### `GetEyePoint() -> Point3D`

Get eye point of current projection Eye point is point where camera is.

**Remarks:** Get eye point of current projection Eye point is point where camera is.

**Returns:** `Point3D` — Eye point

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.GetEyePoint)

#### `GetIsoProjection() -> eProjectionType`

Get the isometric projection of the view

**Remarks:** Get the isometric projection of the view

**Returns:** `eProjectionType` — Isometric projection of the view

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.GetIsoProjection)

#### `GetMatrix() -> Matrix3D`

Get the view matrix

**Remarks:** Get the view matrix

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.GetMatrix)

#### `GetPixelFactor() -> tuple[float, float]`

Get factor of pixel to world coordinates Used when you need calculate how much millimeters are one pixel

**Remarks:** Get factor of pixel to world coordinates Used when you need calculate how much millimeters are one pixel

**Returns:** `tuple[float, float]` — tuple(Pixel factor in x direction, Pixel factor in y direction)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.GetPixelFactor)

#### `GetScreenScale() -> float`

Get the screen scale

**Remarks:** Get the screen scale

**Returns:** `float` — Screen scale

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.GetScreenScale)

#### `GetSearchRadiusByPixel(pixel: int) -> float`

Get the search radius by pixel

**Remarks:** Get the search radius by pixel

**Parameters:**
- `pixel` (int) — Pixel of the search radius

**Returns:** `float` — Search radius in view size

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.GetSearchRadiusByPixel)

#### `GetViewAngle() -> float`

Get the rotation angle of the view

**Remarks:** Get the rotation angle of the view

**Returns:** `float` — Rotation angle of the view

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.GetViewAngle)

#### `GetViewPoint() -> Point3D`

Get view point of current projection View point is point where you are looking (focused). View point lie in plain of drawn data.

**Remarks:** Get view point of current projection View point is point where you are looking (focused). View point lie in plain of drawn data.

**Returns:** `Point3D` — Eye point

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.GetViewPoint)

#### `GetViewSize() -> Vector2D`

Get the size of the view

**Remarks:** Get the size of the view

**Returns:** `Vector2D` — Size of the view

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.GetViewSize)

#### `GetViewZAngle() -> float`

Get the rotation angle of the view in z-direction

**Remarks:** Get the rotation angle of the view in z-direction

**Returns:** `float` — Rotation angle of the view in z-direction

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.GetViewZAngle)

#### `GetWindowNumber() -> int`

Get the window number

**Remarks:** Get the window number

**Returns:** `int` — Window number

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.GetWindowNumber)

#### `GetZoomWindowNumber() -> int`

Get the zoom window number. Numbers 1 and more for active zoom window.

**Remarks:** Get the zoom window number. Numbers 1 and more for active zoom window.

**Returns:** `int` — Window number

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.GetZoomWindowNumber)

#### `GetZoomWndTextFac() -> float`

Get the text-factor of the zoom window

**Remarks:** Get the text-factor of the zoom window

**Returns:** `float` — Text-factor of the zoom window

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.GetZoomWndTextFac)

#### `GetZoomWndXFac() -> float`

Get the x-factor of the zoom window

**Remarks:** Get the x-factor of the zoom window

**Returns:** `float` — X-factor of the zoom window

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.GetZoomWndXFac)

#### `GetZoomWndYFac() -> float`

Get the y-factor of the zoom window

**Remarks:** Get the y-factor of the zoom window

**Returns:** `float` — Y-factor of the zoom window

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.GetZoomWndYFac)

#### `IsAssistWindow() -> bool`

Check for assist window

**Remarks:** Check for assist window

**Returns:** `bool` — The document is an assist window: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.IsAssistWindow)

#### `IsCentralProjection() -> bool`

Check, whether the projection is a central projection

**Remarks:** Check, whether the projection is a central projection

**Returns:** `bool` — Central projection: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.IsCentralProjection)

#### `IsFreeProjection() -> bool`

Check, whether the projection is a free projection

**Remarks:** Check, whether the projection is a free projection

**Returns:** `bool` — Projection is a free view: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.IsFreeProjection)

#### `IsGroundplanView() -> bool`

Check, whether the projection is ground plan view

**Remarks:** Check, whether the projection is ground plan view

**Returns:** `bool` — Projection is ground plan view: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.IsGroundplanView)

#### `IsInView(pnt: Point2D) -> bool`

Check, whether the point is inside the view

**Remarks:** Check, whether the point is inside the view

**Parameters:**
- `pnt` (Point2D) — View point

**Returns:** `bool` — Point is inside the view: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.IsInView)

#### `IsIsometricProjection() -> bool`

Check, whether the projection is a isometric projection

**Remarks:** Check, whether the projection is a isometric projection

**Returns:** `bool` — Projection is a isometric projection: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.IsIsometricProjection)

#### `IsSideView() -> bool`

Check, whether the projection is a side view

**Remarks:** Check, whether the projection is a side view

**Returns:** `bool` — Projection is a side view: true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.IsSideView)

#### `IsZoomWindow() -> bool`

Check if the input is inside the zoom window

**Remarks:** Check if the input is inside the zoom window

**Returns:** `bool` — Window number

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.IsZoomWindow)

#### `ProjectionToWorld(pnt: Point2D, refPnt: Point3D) -> Point3D`

Get the world 3D point from a projection 2D point and a reference point

**Remarks:** Get the world 3D point from a projection 2D point and a reference point

**Parameters:**
- `pnt` (Point2D) — View Point
- `refPnt` (Point3D) — Reference point with the additional coordinate

**Returns:** `Point3D` — World 3D point

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.ProjectionToWorld)

#### `SetPointDepth(pointWithDepth: Point3D) -> Point3D`

Set point depth by current view projection type and pointWithDepth

**Remarks:** Set point depth by current view projection type and pointWithDepth

**Parameters:**
- `pointWithDepth` (Point3D) — Point with depth

**Returns:** `Point3D` — Point to set

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.SetPointDepth)

#### `ViewPerpendicularToWorld(line3D: Line3D, pnt: Point3D) -> Point3D`

Transform a view perpendicular point to a world perpendicular point

**Remarks:** Transform a view perpendicular point to a world perpendicular point

**Parameters:**
- `line3D` (Line3D) — 3D perpendicular line
- `pnt` (Point3D) — Reference point for the perpendicular

**Returns:** `Point3D` — World perpendicular point

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.ViewPerpendicularToWorld)

#### `ViewToPixel( pnt: Point2D, bBottomTop: bool, considerZoomwindow: bool = True ) -> Point2D`

Transform a view point to a pixel coordinate

**Remarks:** Transform a view point to a pixel coordinate

**Parameters:**
- `pnt` (Point2D) — View point
- `bBottomTop` (bool) — The y zero point is on the bottom of the screen: true/false
- `considerZoomwindow` (bool) — if true and the position is in a zoom window, then the zoom window will be used for the calculation

**Returns:** `Point2D` — Pixel view point

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.ViewToPixel)

#### `ViewToWorld(pnt: Point2D, z: float = 0) -> Point3D`

Transform a view point to a world point

**Remarks:** Transform a view point to a world point

**Parameters:**
- `pnt` (Point2D) — View point
- `z` (float) — z-coordinate

**Returns:** `Point3D` — World point

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.ViewToWorld)

#### `ViewToWorldBaseZ(pnt: Point2D, zWorld: float) -> Point3D`

Transform a view point to a world point with resulting z-coordinate

**Remarks:** Transform a view point to a world point with resulting z-coordinate

**Parameters:**
- `pnt` (Point2D) — View point
- `zWorld` (float) — Z-coordinate

**Returns:** `Point3D` — World point with resulting z-coordinate

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.ViewToWorldBaseZ)

#### `ViewToWorldBaseZ0(pnt: Point2D) -> Point3D`

Transform a view point to a world point with resulting z-world = 0

**Remarks:** Transform a view point to a world point with resulting z-world = 0

**Parameters:**
- `pnt` (Point2D) — View point

**Returns:** `Point3D` — World point with z=0

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.ViewToWorldBaseZ0)

#### `ViewToWorldPlane(pnt: Point2D, plane: Plane3D) -> Point3D`

Transform the view point to a world plane

**Remarks:** Transform the view point to a world plane

**Parameters:**
- `pnt` (Point2D) — View point
- `plane` (Plane3D) — Plane

**Returns:** `Point3D` — World point at the plane

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.ViewToWorldPlane)

#### `ViewToWorldRay(pnt: Point2D) -> tuple[Point3D, Vector3D]`

Calculates a ray Calculates a ray (for non-vanishing-point-projections, rather a line) from the given view 2D point. Can be used for pick-point calculations. Note that we have a right-hand view coordinate system, so its Z axis points towards the eye.

**Remarks:** Calculates a ray Calculates a ray (for non-vanishing-point-projections, rather a line) from the given view 2D point. Can be used for pick-point calculations. Note that we have a right-hand view coordinate system, so its Z axis points towards the eye.

**Parameters:**
- `pnt` (Point2D) — View point

**Returns:** `tuple[Point3D, Vector3D]` — tuple(World point of ray, Vector of calculated ray point)

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.ViewToWorldRay)

#### `ViewToWorldViewZ(pnt: Point2D, viewZ: float) -> Point3D`

Transform a view 2D point with view Z coordinate to a world point

**Remarks:** Transform a view 2D point with view Z coordinate to a world point

**Parameters:**
- `pnt` (Point2D) — View point
- `viewZ` (float) — z-coordinate in view transformation

**Returns:** `Point3D` — World point

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.ViewToWorldViewZ)

#### `WorldToPixel(pnt: Point3D, bBottomTop: bool) -> Point2D`

Transform the world point to a pixel point

**Remarks:** Transform the world point to a pixel point

**Parameters:**
- `pnt` (Point3D) — World point
- `bBottomTop` (bool) — The y zero point is on the bottom of the screen: true/false

**Returns:** `Point2D` — Pixel point

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.WorldToPixel)

#### `WorldToProjection(pnt: Point3D) -> Point2D | WorldToProjection(line: Line3D) -> Line2D`

Get the projection 2D point from a world 3D point

**Remarks:** Get the projection 2D point from a world 3D point

**Parameters:**
- `pnt` (Point3D) — World point

**Returns:** `Point2D` — 2D projection point

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.WorldToProjection)

#### `WorldToProjectionBase0(pnt: Point3D) -> Point3D | WorldToProjectionBase0(line: Line3D) -> Line3D`

Get the world projection point with the base 0

**Remarks:** Get the world projection point with the base 0

**Parameters:**
- `pnt` (Point3D) — World point

**Returns:** `Point3D` — World projection point with the base 0

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.WorldToProjectionBase0)

#### `WorldToView(pnt: Point3D) -> Point2D | WorldToView(pnt: Point2D) -> Point2D | WorldToView(x: float, y: float, z: float) -> Point2D | WorldToView(line: Line3D) -> Line2D | WorldToView(line: Line2D) -> Line2D | WorldToView(polyline3D: Polyline3D) -> Polyline2D`

Transform a world point to a view point

**Remarks:** Transform a world point to a view point

**Parameters:**
- `pnt` (Point3D) — World point

**Returns:** `Point2D` — View point

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.WorldToView)

#### `WorldToView3D(pnt: Point3D) -> Point3D`

Transform a world point to a view 3D point If Z coordinate of returned view point is positive, then world point is before eye (i.e. is visible).

**Remarks:** Transform a world point to a view 3D point If Z coordinate of returned view point is positive, then world point is before eye (i.e. is visible).

**Parameters:**
- `pnt` (Point3D) — World point

**Returns:** `Point3D` — View point

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.WorldToView3D)

#### `WorldToWorldPlane(pnt: Point3D, plane: Plane3D) -> Point3D | WorldToWorldPlane(line: Line3D, plane: Plane3D) -> Line3D`

Transform a world point to a plane point

**Remarks:** Transform a world point to a plane point

**Parameters:**
- `pnt` (Point3D) — World point
- `plane` (Plane3D) — Plane

**Returns:** `Point3D` — Plane point in world coordinates

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/ViewWorldProjection/#NemAll_Python_IFW_Input.ViewWorldProjection.WorldToWorldPlane)

## VisibleService (class)

(No description provided in vendor docs for NemAll_Python_IFW_Input.VisibleService.)

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/VisibleService/)

### Methods
#### `ShowAllElements()`

Show all elements

**Remarks:** Show all elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/VisibleService/#NemAll_Python_IFW_Input.VisibleService.ShowAllElements)

#### `ShowElement(eleList: BaseElementAdapter, bShow: bool, bForceDraw: bool)`

Show/hide the element

**Remarks:** Show/hide the element

**Parameters:**
- `eleList` (BaseElementAdapter) — Element list as BaseElementAdapterList
- `bShow` (bool) — Show / Hide = true/false
- `bForceDraw` (bool) — Force the draw state

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/VisibleService/#NemAll_Python_IFW_Input.VisibleService.ShowElement)

#### `ShowElements(eleList: BaseElementAdapterList, bShow: bool)`

Show/hide the elements

**Remarks:** Show/hide the elements

**Parameters:**
- `eleList` (BaseElementAdapterList) — Element list as BaseElementAdapterList
- `bShow` (bool) — Show / Hide = true/false

[Docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/VisibleService/#NemAll_Python_IFW_Input.VisibleService.ShowElements)

## eDocumentSnoopType (enum)

Definition of the document snoop types

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/eDocumentSnoopType/)

### Values
- `eSnoopActiveDocuments` = `0`
- `eSnoopAllDocuments` = `2`
- `eSnoopPassiveDocsOrLayers` = `3`
- `eSnoopPassiveDocuments` = `1`

## eDrawElementIdentPointSymbols (enum)

If the identification mode allows to select an element (IDENT_...ELEMENT...), it's possible to draw also the identification point symbol of the identification point (element point, center point, ...) at the element

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/eDrawElementIdentPointSymbols/)

### Values
- `eDRAW_IDENT_ELEMENT_POINT_SYMBOL_NO` = `0`
- `eDRAW_IDENT_ELEMENT_POINT_SYMBOL_YES` = `1`

## eIdentificationMode (enum)

Type of the identification mode during point input.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/eIdentificationMode/)

### Values
- `eIDENT_ARCHPOINT` = `10` — Identify the nearest point at the cursor position. Similar to eIDENT_POINT, but additionally when point was found on an architectural element, a reference 2D line, on which this point is located, is identified. The line can be get with GetReferenceLine.
- `eIDENT_ARCHPOINT_OFFSET` = `11` — Identify the nearest point at the cursor position. As eIDENT_ARCHPOINT, but in case a line is identified, a second input step is activated similar to the in eIDENT_POINT_OFFSET When the point was found in an UVS, the point is returned in the world coordinate system
- `eIDENT_ARCH_ELEMENTPOINT` = `12` — Identify the nearest element point at an architecture element. Similar to eIDENT_ELEMENTPOINT, but additionally a reference line of an architectural element is identified and can be get with GetReferenceLine
- `eIDENT_DOCKINGPOINT` = `13` — Identify the nearest docking point at the cursor position
- `eIDENT_ELEMENTPOINT` = `1` — Identify the nearest element point at the cursor position. Similar to eIDENT_POINT, but input controls for XYZ coordinates are disabled and track tracing is not performed. This implies, that the input point lies on an existing element.
- `eIDENT_POINT` = `0` — Identify the nearest point at the cursor position. Identified are start, end, center or intersection points of all kind of elements. Snapping options set by the user in Allplan settings are considered. This is the default way of point identification.
- `eIDENT_POINT_ASSOC_VIEW_WORLD` = `2` — Identify the nearest point at the cursor position. Similar to eIDENT_POINT, but when the point was found in an UVS, the point is returned in the world coordinate system
- `eIDENT_POINT_ELEMENT` = `3` — Identify the nearest point or element at the cursor position As eIDENT_POINT, but additionally a geometry element (like edge/face of a polyhedron, polygon segment, etc...) is searched during the point input. The found geometry element can be get with GetSelectedGeometryElement. Element is not identified, when a point (start, end, center, intersection, etc...) is found. Filter for the element identification can be set with SetGeometryFilter and SetGeometryElementFilter
- `eIDENT_POINT_ELEMENT_ALWAYS` = `5` — Similar to eIDENT_POINT_ELEMENT, but the element is always identified, even when a point was found
- `eIDENT_POINT_ELEMENT_ALWAYS_CENTER` = `6` — As eIDENT_POINT_ELEMENT_ALWAYS
- `eIDENT_POINT_ELEMENT_CENTER` = `4` — Similar to eIDENT_POINT_ELEMENT, but the element is identified also, when a center point (but not start/end point) is found.
- `eIDENT_POINT_OFFSET` = `8` — Identify the nearest point or element at the cursor position Similar to POINT_ELEMENT, but in case a line is identified (2D or 3D), a second input step is introduced, where the user can provide the offset distance. The resulting point will be located on the line at the given distance from the start or end point (depending, where the line was clicked)
- `eIDENT_POINT_PERPENDICULAR` = `7` — Identify the nearest point or point perpendicular to a line Similar to eIDENT_POINT, but additionally if a 2D line is identified, the result will be a point calculated by projecting the previously input point onto this line. Works only in ground view!
- `eIDENT_TEXTPOINT` = `9` — Snapping only to the anchor points of text or label elements
- `eIDENT_TEXTPOINTER_POINT` = `14`

## eLayerSnoopType (enum)

Definition of the layer snoop types

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/eLayerSnoopType/)

### Values
- `eSnoopActiveLayers` = `0`
- `eSnoopAllLayers` = `2`
- `eSnoopPassiveLayers` = `1`

## eProjectionType (enum)

Projection type of the view

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/eProjectionType/)

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

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/eSplitElement3D/)

### Values
- `ELEMENT3D_EDGES` = `2`
- `ELEMENT3D_FACES` = `1`
- `ELEMENT3D_NO_SPLIT` = `0`

## eTrackLineType (enum)

Definition of the track line types

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/eTrackLineType/)

### Values
- `TRACKLINE_ENDLESS` = `1`
- `TRACKLINE_EXTENSION` = `2`
- `TRACKLINE_EXTENSION_END` = `3`
- `TRACKLINE_NO` = `0`

## eValueInputControlType (enum)

Types of the input controls, that can be displayed in the dialog line in Allplan

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_Input/eValueInputControlType/)

### Values
- `eANGLE_COMBOBOX` = `6` — Angle input as a combo box with suggested angle values. Free input is possible.
- `eANGLE_EDIT` = `11` — Angle input as a regular edit field. Value is entered in the current Allplan angle units, but returned in radians.
- `eCONTROL_EXTERNAL` = `1`
- `eCONTROL_NONE` = `0` — No input control
- `eCOORDINATE_EDIT` = `101` — Input control for a single coordinate. Value is entered in the current Allplan length units, but returned in millimeters. Allowed values between -1e10 and 1-e10
- `eCOORDINATE_EDIT_FIX` = `102` — Similar to eCOORDINATE_EDIT
- `eCOORDINATE_EDIT_GE0` = `103` — Similar to eCOORDINATE_EDIT, but only values >= 0 allowed
- `eCOORDINATE_EDIT_GT0` = `104` — Similar to eCOORDINATE_EDIT, but only values > 0 allowed
- `eDIMENSION_EDIT` = `5` — Length input with current length units. Returned value is in millimeters. Values between 0.0 and 1e10 allowed
- `eDIMENSION_EDIT_MM` = `10` — Length input in millimeters. Values between 0.0 and 1e10 allowed
- `eDOUBLE_EDIT` = `2` — Input control for a double value. Values between 0.0 and 1e10 allowed
- `eINT_COMBOBOX` = `4` — Similar to eINT_EDIT, but with a combo box with suggested values.
- `eINT_EDIT` = `3` — Input control for a 32-bit signed integer, with allowed value between 0 and -2147483684
- `eNUMBER_EDIT_1` = `201` — Edit control for an integer value between 1 and 1e6
- `eNUMBER_EDIT_1_GE0` = `202` — Edit control for an integer value between 0 and 1e6
- `eROTATION_ANGLE_STEP` = `8` — Input control for a shortcut input (+ or -). Input triggers the on_shortcut_control_input event.
- `eTEXT_EDIT` = `9` — String input control
- `eWALL_PLACEMENT` = `7` — Input control for a shortcut input (+ or -). Input triggers the on_shortcut_control_input event.

