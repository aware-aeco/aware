---
name: tekla-tekla-structures-drawing-ui
description: This skill encodes the tekla 2025.0 surface of the Tekla.Structures.Drawing.UI namespace — 14 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: DrawingSelector, DrawingObjectSelector, Picker, Events, Events.DocumentManagerClosedDelegate, IUIEvents, Events.DrawingEditorClosedDelegate, Events.DrawingEditorOpenedDelegate, and 6 more types.
---

# Tekla.Structures.Drawing.UI

Auto-generated from vendor docs for tekla 2025.0. 14 types in this namespace.

## DrawingObjectSelector (class)

The DrawingObjectSelector class is used to select drawing objects in the drawing. The class contains methods for selecting/unselecting single objects or a list of objects. Currently, these selections both select the objects from the database and highlight them visually.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/7180d038-5676-9fc4-bd6c-bd5f4ec53535)

### Methods
#### `public DrawingObjectEnumerator GetSelected()`

Gets the selected objects in the drawing.

**Returns:** `DrawingObjectEnumerator` — A drawing object enumerator of the selected objects in the drawing.

[Docs](https://developer.tekla.com/topic/en/18/43/23520711-1212-6069-bd1e-684691554671)

#### `public bool SelectObject(DrawingObject DrawingObject)`

Selects a single object in the drawing.

**Parameters:**
- `DrawingObject` (Tekla.Structures.Drawing.DrawingObject) — The object to select.

**Returns:** `Boolean` — True if succeeded.

[Docs](https://developer.tekla.com/topic/en/18/43/ce422e54-d551-c67b-ee2c-2e85907b91c7)

#### `public bool SelectObjects(ArrayList DrawingObjects, bool ExtendSelection)`

Selects the specified drawing objects in the drawing.

**Parameters:**
- `DrawingObjects` (System.Collections.ArrayList) — The drawing objects to be selected.
- `ExtendSelection` (System.Boolean) — Defines whether the selector keeps the currently selected objects selected or not.

**Returns:** `Boolean` — True if succeeded.

[Docs](https://developer.tekla.com/topic/en/18/43/16229e0b-b7ed-5b1c-3cc6-c64911d01ef4)

#### `public bool UnselectAllObjects()`

Unselects all the selected objects.

**Returns:** `Boolean` — True if succeeded.

[Docs](https://developer.tekla.com/topic/en/18/43/bf12b799-6c49-70a7-dca9-d09c62c64cd9)

#### `public bool UnselectObject(DrawingObject DrawingObject)`

Unselects a drawing object.

**Parameters:**
- `DrawingObject` (Tekla.Structures.Drawing.DrawingObject) — The drawing object to be unselected.

**Returns:** `Boolean` — True if succeeded.

[Docs](https://developer.tekla.com/topic/en/18/43/7dfd42cd-254d-ef27-6ab0-201a6d94b50c)

#### `public bool UnselectObjects(ArrayList DrawingObjects)`

Unselects the specified drawing objects keeping the other objects still selected.

**Parameters:**
- `DrawingObjects` (System.Collections.ArrayList) — The drawing objects to be unselected.

**Returns:** `Boolean` — True if succeeded.

[Docs](https://developer.tekla.com/topic/en/18/43/16da69e3-8e6d-8cbc-6ecb-c10645b03487)

## DrawingSelector (class)

The DrawingSelector class provides functionality for accessing the drawing list dialog.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/1bd3874e-2a25-28a2-62ee-6db4ffdeef5a)

### Methods
#### `public DrawingEnumerator GetSelected()`

Gets the drawings that are currently selected in the drawing list dialog.

**Returns:** `DrawingEnumerator` — The drawings that are currently selected in the drawing list dialog.

[Docs](https://developer.tekla.com/topic/en/18/43/ef028482-5c99-2d37-0d57-8998b46346de)

## Events (class)

The Events class allows the user to register event listeners for drawing user interface events.

**Remarks:** Asynchronous event handling Registered event handlers are called asynchronously so that many handlers may be running simultaneously. Event handlers are not guaranteed to be run in the same thread where they were registered. The asynchronous nature of the events requires the use of synchronization constructs on objects and data structures that are not defined to be thread safe.Microsoft's documentation about the lock statement: http://msdn.microsoft.com/en-us/library/c5kehkcz.aspxMicrosoft's documentation about delegates: http://msdn.microsoft.com/en-us/library/900fyy8e.aspxMicrosoft's documentation about managed threading: http://msdn.microsoft.com/en-us/library/1c9txz50.aspx

[Vendor docs](https://developer.tekla.com/topic/en/18/43/5e9a7269-86c0-e674-6b7d-775dd6d0ffc1)

### Constructors
- `public Events()` — Constructs a new instance of Events.

### Methods
#### `public void Dispose()`

Disposes of the Events instance.

[Docs](https://developer.tekla.com/topic/en/18/43/ad8753f7-5a3e-b9e5-07fc-cc5bf3bdcbcc)

#### `public override Object InitializeLifetimeService()`

Initializes the lifetime service.

**Returns:** `Object` — Always null.

[Docs](https://developer.tekla.com/topic/en/18/43/2d43bede-eff0-ea69-10ca-39097da266e4)

#### `public void OnInterrupted()`

Called when uesr interrupts.

[Docs](https://developer.tekla.com/topic/en/18/43/993d9fd4-eff0-c073-fa67-ea0f56113a5b)

#### `public void Register()`

Registers the instance to listen to the specified events.

[Docs](https://developer.tekla.com/topic/en/18/43/40f9a059-e3b6-7441-9fe5-52a122ef9834)

#### `public void UnRegister()`

Unregisters the instance from listening to events.

[Docs](https://developer.tekla.com/topic/en/18/43/c5d65f46-f114-4387-fc57-4ce16baec662)

### Events
#### `DocumentManagerClosed` (`Tekla.Structures.Drawing.UI.Events.DocumentManagerClosedDelegate`)

**Signature:** `public event Events.DocumentManagerClosedDelegate DocumentManagerClosed`

The DocumentManagerClosed eventis raised when the document manager is closed.

[Docs](https://developer.tekla.com/topic/en/18/43/5f7146c5-b5c4-4fdd-2082-98882bc08712)

#### `DrawingEditorClosed` (`Tekla.Structures.Drawing.UI.Events.DrawingEditorClosedDelegate`)

**Signature:** `public event Events.DrawingEditorClosedDelegate DrawingEditorClosed`

The DrawingEditorClosed event is raised when the drawing editor is closed.

[Docs](https://developer.tekla.com/topic/en/18/43/c74b3846-83ff-ae56-e133-7528a0b6773a)

#### `DrawingEditorOpened` (`Tekla.Structures.Drawing.UI.Events.DrawingEditorOpenedDelegate`)

**Signature:** `public event Events.DrawingEditorOpenedDelegate DrawingEditorOpened`

The DrawingEditorOpened event is raised when the drawing editor is opened.

[Docs](https://developer.tekla.com/topic/en/18/43/41ddb49f-e0cd-a44d-a408-5ca172bed4ab)

#### `DrawingListSelectionChanged` (`Tekla.Structures.Drawing.UI.Events.DrawingListSelectionChangedDelegate`)

**Signature:** `public event Events.DrawingListSelectionChangedDelegate DrawingListSelectionChanged`

The DrawingListSelectionChanged event is raised when selection on the drawing list has changed.

[Docs](https://developer.tekla.com/topic/en/18/43/43e29a01-2a71-56de-70b5-feab70afa5d6)

#### `DrawingLoaded` (`Tekla.Structures.Drawing.UI.Events.DrawingLoadedDelegate`)

**Signature:** `public event Events.DrawingLoadedDelegate DrawingLoaded`

The DrawingLoaded event is raised when a new drawing is opened in the drawing editor.

[Docs](https://developer.tekla.com/topic/en/18/43/d4a747ac-8038-2da1-1f65-519cd8de16e0)

#### `Interrupted` (`Tekla.Structures.Drawing.UI.Events.InterruptedDelegate`)

**Signature:** `public event Events.InterruptedDelegate Interrupted`

Occurs when user interrupts.

[Docs](https://developer.tekla.com/topic/en/18/43/78b51d31-526d-0f15-e398-151534863260)

#### `LayoutEditingModeEntered` (`Tekla.Structures.Drawing.UI.Events.LayoutEditingModeEnteredDelegate`)

**Signature:** `public event Events.LayoutEditingModeEnteredDelegate LayoutEditingModeEntered`

The LayoutEditingModeEntered event is raised when layout editing mode is started.

[Docs](https://developer.tekla.com/topic/en/18/43/7050d35e-9db2-6988-15f3-b5805eee4100)

#### `LayoutEditingModeExited` (`Tekla.Structures.Drawing.UI.Events.LayoutEditingModeExitedDelegate`)

**Signature:** `public event Events.LayoutEditingModeExitedDelegate LayoutEditingModeExited`

The LayoutEditingModeEntered event is raised when layout editing mode is finished.

[Docs](https://developer.tekla.com/topic/en/18/43/265cc7d5-ee9d-23a0-f7e9-7c32880975e4)

#### `LayoutOptionModified` (`LayoutOptionModifiedDelegate`)

**Signature:** `public event LayoutOptionModifiedDelegate LayoutOptionModified`

The event is raised when layout option is modified

[Docs](https://developer.tekla.com/topic/en/18/43/397e75fb-55aa-f902-0da0-9eec16361ad1)

#### `LayoutTableDeleted` (`LayoutTableDeletedDelegate`)

**Signature:** `public event LayoutTableDeletedDelegate LayoutTableDeleted`

The event is raised when table is deleted

[Docs](https://developer.tekla.com/topic/en/18/43/1337f393-15f0-a718-b14a-af77be23a741)

#### `LayoutTableInserted` (`LayoutTableInsertedDelegate`)

**Signature:** `public event LayoutTableInsertedDelegate LayoutTableInserted`

The event is raised when table is inserted

[Docs](https://developer.tekla.com/topic/en/18/43/f80d9c64-1523-52b1-dc61-f5fedcf5549e)

#### `LayoutTableLayoutModified` (`LayoutTableLayoutModifiedDelegate`)

**Signature:** `public event LayoutTableLayoutModifiedDelegate LayoutTableLayoutModified`

The event is raised when tablelayout is modified

[Docs](https://developer.tekla.com/topic/en/18/43/b188cd2a-87d6-6b76-4873-bcad2dc8808f)

#### `LayoutTableModified` (`LayoutTableModifiedDelegate`)

**Signature:** `public event LayoutTableModifiedDelegate LayoutTableModified`

The event is never raised

[Docs](https://developer.tekla.com/topic/en/18/43/539f548b-e6ad-b40f-2396-2644ac220015)

#### `SelectionChange` (`Tekla.Structures.Drawing.UI.Events.SelectionChangeDelegate`)

**Signature:** `public event Events.SelectionChangeDelegate SelectionChange`

The SelectionChange event is raised when the selection is changed in a Tekla Structures drawing.

[Docs](https://developer.tekla.com/topic/en/18/43/c4f6d216-43ae-7d5e-3246-c7ef3f91b5db)

## Events.DocumentManagerClosedDelegate (delegate)

The delegate to use for the document manager closing.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/b911c9af-e21a-db2c-3040-a7111731da91)

## Events.DrawingEditorClosedDelegate (delegate)

The delegate to use for drawing editor closing.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/b7c732ea-318f-1fdf-e56f-01029f1e8552)

## Events.DrawingEditorOpenedDelegate (delegate)

The delegate to use for drawing editor opening.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/f462b10f-c16f-42e4-d1c4-1289a5d1bf74)

## Events.DrawingListSelectionChangedDelegate (delegate)

The delegate to use for drawing list selection change.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/9c2c5433-f5a1-3979-731f-5144f29d749d)

## Events.DrawingLoadedDelegate (delegate)

The delegate to use for drawing loading.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/bb9072ed-f484-80fe-d748-42956da051f8)

## Events.InterruptedDelegate (delegate)

The delegate to use for interrupted event

[Vendor docs](https://developer.tekla.com/topic/en/18/43/58edfe76-6281-7d73-380a-7c204b18b7e3)

## Events.LayoutEditingModeEnteredDelegate (delegate)

The delegate to use for entering layout edit mode.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/7f3005ca-2bb9-2728-458f-741c7a9a1e07)

## Events.LayoutEditingModeExitedDelegate (delegate)

The delegate to use for exiting layout edit mode.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/d31a011b-2c67-07b7-824b-50be25abcbdb)

## Events.SelectionChangeDelegate (delegate)

The delegate to use for selection change.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/10c8f8a4-513d-aa67-a39d-d0dcb7e86846)

## IUIEvents (interface)

Interface for the Events class allows the user to register event listeners for DrawingUI events. This interface allows the Marshaling of the Events class with Trimble.Remoting

[Vendor docs](https://developer.tekla.com/topic/en/18/43/fcca08a4-5bcd-8d4d-1796-e4ea942d3b91)

### Methods
#### `void OnInterrupted()`

Called when uesr interrupts.

[Docs](https://developer.tekla.com/topic/en/18/43/79cef3d6-e028-dde1-dda0-68fab9990ead)

#### `void Register()`

Registers the instance to listen to the specified events.

[Docs](https://developer.tekla.com/topic/en/18/43/c70f206a-d96f-1194-313d-a1fd3ddd51cd)

#### `void UnRegister()`

Unregisters the instance from listening to events.

[Docs](https://developer.tekla.com/topic/en/18/43/dd0db162-4261-8eaf-ac23-56353216b51b)

### Events
#### `DocumentManagerClosed` (`Tekla.Structures.Drawing.UI.Events.DocumentManagerClosedDelegate`)

**Signature:** `event Events.DocumentManagerClosedDelegate DocumentManagerClosed`

The DocumentManagerClosed eventis raised when the document manager is closed.

[Docs](https://developer.tekla.com/topic/en/18/43/f85b670d-2fa4-c973-eea8-8811cc01ea31)

#### `DrawingEditorClosed` (`Tekla.Structures.Drawing.UI.Events.DrawingEditorClosedDelegate`)

**Signature:** `event Events.DrawingEditorClosedDelegate DrawingEditorClosed`

The DrawingEditorClosed event is raised when the drawing editor is closed.

[Docs](https://developer.tekla.com/topic/en/18/43/62ee9224-ff44-6811-7bec-0735ee04ae2c)

#### `DrawingEditorOpened` (`Tekla.Structures.Drawing.UI.Events.DrawingEditorOpenedDelegate`)

**Signature:** `event Events.DrawingEditorOpenedDelegate DrawingEditorOpened`

The DrawingEditorOpened event is raised when the drawing editor is opened.

[Docs](https://developer.tekla.com/topic/en/18/43/1c303c30-c9b8-f492-a79a-dfd83f77a68d)

#### `DrawingListSelectionChanged` (`Tekla.Structures.Drawing.UI.Events.DrawingListSelectionChangedDelegate`)

**Signature:** `event Events.DrawingListSelectionChangedDelegate DrawingListSelectionChanged`

The DrawingListSelectionChanged event is raised when selection on the drawing list has changed.

[Docs](https://developer.tekla.com/topic/en/18/43/24ccc4b9-2c62-b936-33a5-34085f067bfa)

#### `DrawingLoaded` (`Tekla.Structures.Drawing.UI.Events.DrawingLoadedDelegate`)

**Signature:** `event Events.DrawingLoadedDelegate DrawingLoaded`

The DrawingLoaded event is raised when a new drawing is opened in the drawing editor.

[Docs](https://developer.tekla.com/topic/en/18/43/fe91aaa8-6824-f3db-4aa7-51e26a8e288e)

#### `Interrupted` (`Tekla.Structures.Drawing.UI.Events.InterruptedDelegate`)

**Signature:** `event Events.InterruptedDelegate Interrupted`

Occurs when user interrupts.

[Docs](https://developer.tekla.com/topic/en/18/43/249e5e09-4c01-a334-0d4e-61b3a54921b1)

#### `SelectionChange` (`Tekla.Structures.Drawing.UI.Events.SelectionChangeDelegate`)

**Signature:** `event Events.SelectionChangeDelegate SelectionChange`

The SelectionChange event is raised when the selection is changed in a Tekla Structures drawing.

[Docs](https://developer.tekla.com/topic/en/18/43/6b58856b-f085-7d08-e263-8041c5d46fa3)

## Picker (class)

The Picker class is used to pick points and objects in the drawing.

**Remarks:** Prompts Tekla Structures prefixes the given prompt with "prompt_" and looks for a translation in the prompts.ail file. If the translation (e.g. "prompt_Pick_first_position") is not found in the prompts.ail file, the prompt string is displayed as such. This feature can be used to give already translated strings to the picker.

[Vendor docs](https://developer.tekla.com/topic/en/18/43/276155dd-edd3-807b-23e1-91517eaf2f13)

### Methods
#### `public bool IsInteractive()`

Indicates whether the picker is interactive and currently allowed to access the UI or not. If not, then the plug-in should avoid making dialog calls or message boxes.

**Returns:** `Boolean` — True if the picker is interactive.

[Docs](https://developer.tekla.com/topic/en/18/43/da5bbacc-48fe-9a67-3ea6-bceba9bb9170)

#### `public Tuple<DrawingObject, ViewBase> PickObject(string prompt)`

Requests the user to pick one object. Returns the picked drawing object and the view in which the pick occurred.

**Parameters:**
- `prompt` (System.String) — The prompt shown in the status bar before the pick. See the class remarks for more information about prompts.

**Returns:** `Tuple<DrawingObject,ViewBase>` — 

[Docs](https://developer.tekla.com/topic/en/18/43/8e5e5ea6-402d-1136-59b4-b38982a719f5)

#### `public void PickObject(string prompt, Type[] typeFilter, out DrawingObject pickedObject, out ViewBase pickedView, out Point pickedPoint)`

Requests the user to pick one object of the given types. Returns the picked point in addition to the picked object.

**Remarks:** The list of types to pick may contain any object derived from DrawingObject or DrawingObject itself. For example, typeof(ModelObject) will allow picking of part, bolts, reinforcements, etc.

**Parameters:**
- `prompt` (System.String) — The prompt shown in the status bar before the picks. See the class remarks for more information about prompts.
- `typeFilter` (.System.Type.) — The types to pick. If null, any type is allowed.
- `pickedObject` (Tekla.Structures.Drawing.DrawingObject.) — The picked drawing object.
- `pickedView` (Tekla.Structures.Drawing.ViewBase.) — The view in which the pick occurred.
- `pickedPoint` (Tekla.Structures.Geometry3d.Point.) — The picked point in the view coordinates.

[Docs](https://developer.tekla.com/topic/en/18/43/a0fcee9f-cac6-a2bd-43fe-ff98962669ed)

#### `public void PickObject(string prompt, out DrawingObject pickedObject, out ViewBase pickedView)`

Requests the user to pick one object.

**Parameters:**
- `prompt` (System.String) — The prompt shown in the status bar before the pick. See the class remarks for more information about prompts.
- `pickedObject` (Tekla.Structures.Drawing.DrawingObject.) — The picked drawing object.
- `pickedView` (Tekla.Structures.Drawing.ViewBase.) — The view in which the pick occurred.

[Docs](https://developer.tekla.com/topic/en/18/43/8a365ec4-234c-1bcb-58c4-03510172175f)

#### `public void PickObject(string prompt, out DrawingObject pickedObject, out ViewBase pickedView, out Point pickedPoint)`

Requests the user to pick one object. Returns the picked point in addition to the picked object.

**Parameters:**
- `prompt` (System.String) — The prompt shown in the status bar before the picks. See the class remarks for more information about prompts.
- `pickedObject` (Tekla.Structures.Drawing.DrawingObject.) — The picked drawing object.
- `pickedView` (Tekla.Structures.Drawing.ViewBase.) — The view in which the pick occurred.
- `pickedPoint` (Tekla.Structures.Geometry3d.Point.) — The picked point in the view coordinates.

[Docs](https://developer.tekla.com/topic/en/18/43/ea9439d0-680a-6c28-38eb-43576a70d5fc)

#### `public Tuple<DrawingObject, ViewBase, Point> PickObjectAndPoint(string prompt)`

Requests the user to pick one object. Returns the picked drawing object, the view in which the pick occurred and picked point in the view coordinates.

**Parameters:**
- `prompt` (System.String) — The prompt shown in the status bar before the pick. See the class remarks for more information about prompts.

**Returns:** `Tuple<DrawingObject,ViewBase,Point>` — 

[Docs](https://developer.tekla.com/topic/en/18/43/a87593a7-ff9c-9236-5a63-8257be35c60d)

#### `public Tuple<Point, ViewBase> PickPoint(string prompt)`

Requests the user to pick one point. Returns the picked point in the view coordinates and the view in which the pick occurred.

**Parameters:**
- `prompt` (System.String) — The prompt shown in the status bar. See the class remarks for more information about prompts.

**Returns:** `Tuple<Point,ViewBase>` — 

[Docs](https://developer.tekla.com/topic/en/18/43/aed79bed-3da4-c92a-9905-b014452a3646)

#### `public void PickPoint(string prompt, out Point pickedPoint, out ViewBase pickedView)`

Requests the user to pick one point.

**Parameters:**
- `prompt` (System.String) — The prompt shown in the status bar. See the class remarks for more information about prompts.
- `pickedPoint` (Tekla.Structures.Geometry3d.Point.) — The picked point in the view coordinates.
- `pickedView` (Tekla.Structures.Drawing.ViewBase.) — The view in which the pick occurred.

[Docs](https://developer.tekla.com/topic/en/18/43/7421218b-def4-b597-3aad-6808da76b797)

#### `public Tuple<PointList, ViewBase> PickPoints(int numberOfPicks, StringList prompts)`

Requests N picks of points from the user. Returns the picked points in the view coordinates and the view in which the pick occurred.

**Parameters:**
- `numberOfPicks` (System.Int32) — The number of requested picks.
- `prompts` (Tekla.Structures.Drawing.StringList) — The prompts shown in the status bar before the picks. See the class remarks for more information about prompts.

**Returns:** `Tuple<PointList,ViewBase>` — 

[Docs](https://developer.tekla.com/topic/en/18/43/0b689aad-2a96-5dfb-b137-06fe03d555c7)

#### `public void PickPoints(int numberOfPicks, StringList prompts, out PointList pickedPoints, out ViewBase pickedView)`

Requests N picks of points from the user.

**Parameters:**
- `numberOfPicks` (System.Int32) — The number of requested picks.
- `prompts` (Tekla.Structures.Drawing.StringList) — The prompts shown in the status bar before the picks. See the class remarks for more information about prompts.
- `pickedPoints` (Tekla.Structures.Drawing.PointList.) — The picked points in the view coordinates.
- `pickedView` (Tekla.Structures.Drawing.ViewBase.) — The view in which the pick occurred.

[Docs](https://developer.tekla.com/topic/en/18/43/fc10f2d5-3830-a740-b695-e7e7303d292c)

#### `public Tuple<PointList, ViewBase> PickPoints(StringList prompts)`

Requests picks of points from the user. The sequence is terminated by the middle button pick. Returns the picked points in the view coordinates and the view in which the pick occurred.

**Parameters:**
- `prompts` (Tekla.Structures.Drawing.StringList) — The prompts shown in the status bar before the picks. See the class remarks for more information about prompts.

**Returns:** `Tuple<PointList,ViewBase>` — 

[Docs](https://developer.tekla.com/topic/en/18/43/44081ef2-4ff6-8b9d-55d1-ba460ff95bd7)

#### `public void PickPoints(StringList prompts, out PointList pickedPoints, out ViewBase pickedView)`

Requests picks of points from the user. The sequence is terminated by the middle button pick.

**Parameters:**
- `prompts` (Tekla.Structures.Drawing.StringList) — The prompts shown in the status bar before the picks. See the class remarks for more information about prompts.
- `pickedPoints` (Tekla.Structures.Drawing.PointList.) — The picked points in the view coordinates.
- `pickedView` (Tekla.Structures.Drawing.ViewBase.) — The view in which the pick occurred.

[Docs](https://developer.tekla.com/topic/en/18/43/7626f277-a74f-3952-5010-2cd624915b04)

#### `public void PickThreePoints(string firstPrompt, string secondPrompt, string thirdPrompt, out Point firstPickedPoint, out Point secondPickedPoint, out Point thirdPickedPoint, out ViewBase pickedView)`

Requests a pick of three points from the user.

**Parameters:**
- `firstPrompt` (System.String) — The prompt shown in the status bar before the first pick. See the class remarks for more information about prompts.
- `secondPrompt` (System.String) — The prompt shown in the status bar before the second pick. See the class remarks for more information about prompts.
- `thirdPrompt` (System.String) — The prompt shown in the status bar before the third pick. See the class remarks for more information about prompts.
- `firstPickedPoint` (Tekla.Structures.Geometry3d.Point.) — The first picked point in the view coordinates.
- `secondPickedPoint` (Tekla.Structures.Geometry3d.Point.) — The second picked point in the view coordinates.
- `thirdPickedPoint` (Tekla.Structures.Geometry3d.Point.) — The third picked point in the view coordinates.
- `pickedView` (Tekla.Structures.Drawing.ViewBase.) — The view in which the pick occurred.

[Docs](https://developer.tekla.com/topic/en/18/43/2303df0f-78d7-97b0-bb03-78968762536c)

#### `public void PickTwoPoints(string firstPrompt, string secondPrompt, out Point firstPickedPoint, out Point secondPickedPoint, out ViewBase pickedView)`

Requests a pick of two points from the user.

**Parameters:**
- `firstPrompt` (System.String) — The prompt shown in the status bar before the first pick. See the class remarks for more information about prompts.
- `secondPrompt` (System.String) — The prompt shown in the status bar before the second pick. See the class remarks for more information about prompts.
- `firstPickedPoint` (Tekla.Structures.Geometry3d.Point.) — The first picked point in the view coordinates.
- `secondPickedPoint` (Tekla.Structures.Geometry3d.Point.) — The second picked point in the view coordinates.
- `pickedView` (Tekla.Structures.Drawing.ViewBase.) — The view in which the pick occurred.

[Docs](https://developer.tekla.com/topic/en/18/43/158b8403-e2db-f604-2eb9-f1d2a1a80978)

