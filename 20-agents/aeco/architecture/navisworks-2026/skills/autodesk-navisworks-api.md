---
name: navisworks-autodesk-navisworks-api
description: This skill encodes the navisworks 2026.0 surface of the Autodesk.Navisworks.Api namespace — 161 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AnimationBehavior, AppearanceOverrides, Application, Assignee, AxisAndAngleResult, BoundingBox2D, BoundingBox3D, CameraMode, and 153 more types.
---

# Autodesk.Navisworks.Api

Auto-generated from vendor docs for navisworks 2026.0. 161 types in this namespace.

## AnimationBehavior (enum)

Animation behavior as applied by the TimeLiner simulation engine.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.AnimationBehavior)

### Values
- `Scale` = `0` — The animation duration is scaled to be same as task duration.
- `Start` = `1` — The animation starts when the task starts. The animation may be clipped if the task finishes before animation does.
- `End` = `2` — The animation starts so that it ends when the task ends. The animation start is clipped (starts part way through) if the animation needs to start before task does.

## AppearanceOverrides (class)

Appearance overrides.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.AppearanceOverrides)

### Constructors
- `AppearanceOverrides(System.Collections.ObjectModel.Collection<Autodesk.Navisworks.Api.MaterialOverride> materialOverrides)` — Appearance overrides.

### Properties
- `MaterialOverrides` (System.Collections.ObjectModel.Collection<Autodesk.Navisworks.Api.MaterialOverride>, get) — Material overrides.

## Application (class)

The singleton object that represents the application.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Application)

### Methods
#### `static Autodesk.Navisworks.Api.Progress BeginProgress()`

Provides an instance of the Progress class which starts reporting of progress of an operation, typically displaying a progress bar or dialog to the end user.

**Returns:** `Autodesk.Navisworks.Api.Progress` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Application.BeginProgress)

#### `static Autodesk.Navisworks.Api.Progress BeginProgress(string title)`

Provides an instance of the Progress class which starts reporting of progress of an operation, typically displaying a progress bar or dialog to the end user.

**Parameters:**
- `title` (string) — Title for the overall progress operation

**Returns:** `Autodesk.Navisworks.Api.Progress` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Application.BeginProgress%28System.String%29)

#### `static Autodesk.Navisworks.Api.Progress BeginProgress(string title, string message)`

Provides an instance of the Progress class which starts reporting of progress of an operation, typically displaying a progress bar or dialog to the end user.

**Parameters:**
- `title` (string) — Title for the overall progress operation (title for Navisworks progress dialog)
- `message` (string) — Message describing the current operation (body of Navisworks progress dialog)

**Returns:** `Autodesk.Navisworks.Api.Progress` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Application.BeginProgress%28System.String%2CSystem.String%29)

#### `static void EndProgress()`

Finishes the operation and disposes of the Progress control allocated using BeginProgress()

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Application.EndProgress)

#### `static int[] GetCustomColors()`

Retrieve the custom colors for the current session

**Returns:** `int[]` — The custom colors for the current session

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Application.GetCustomColors)

#### `static Autodesk.Navisworks.Api.DocumentInfo LoadDocumentInfo(string fileName)`

Load DocumentInfo.

**Parameters:**
- `fileName` (string) — fileName to load DocumentInfo from

**Returns:** `Autodesk.Navisworks.Api.DocumentInfo` — Returns DocumentInfo if success

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Application.LoadDocumentInfo%28System.String%29)

#### `static void PostUserMessage(string internalId, string source, Autodesk.Navisworks.Api.MessageCenterCategory category, string message, string extendedMessage, int messageFlags)`

PostUserMessage method of Application.

**Parameters:**
- `internalId` (string)
- `source` (string)
- `category` (Autodesk.Navisworks.Api.MessageCenterCategory)
- `message` (string)
- `extendedMessage` (string)
- `messageFlags` (int)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Application.PostUserMessage%28System.String%2CSystem.String%2CAutodesk.Navisworks.Api.MessageCenterCategory%2CSystem.String%2CSystem.String%2CSystem.Int32%29)

#### `static int RegisterBitmap(System.Drawing.Bitmap bitmap)`

Register a bitmap for later use by the graphics system. The function returns an integer ID, or -1 if registration failed.

**Parameters:**
- `bitmap` (System.Drawing.Bitmap)

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Application.RegisterBitmap%28System.Drawing.Bitmap%29)

#### `static void SetCustomColors(int[] customColors)`

Set the custom colors for the current session The custom colors to set for the current session

**Parameters:**
- `customColors` (int[])

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Application.SetCustomColors%28System.Int32%5B%5D%29)

#### `static Autodesk.Navisworks.Api.DocumentInfo TryLoadDocumentInfo(string fileName)`

Try to Load DocumentInfo.

**Parameters:**
- `fileName` (string) — fileName to load DocumentInfo from

**Returns:** `Autodesk.Navisworks.Api.DocumentInfo` — Returns null if failed

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Application.TryLoadDocumentInfo%28System.String%29)

#### `static bool UnregisterBitmap(int bitmapIndex)`

Unregister a previously registered bitmap. The function returns true if successful, or false otherwise.

**Parameters:**
- `bitmapIndex` (int)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Application.UnregisterBitmap%28System.Int32%29)

### Properties
- `ActiveDocument` (Autodesk.Navisworks.Api.Document, get) — Gets a reference to the current Document being displayed in a View
- `Automation` (Autodesk.Navisworks.Api.ApplicationParts.ApplicationAutomation, get) — Gets a reference to the ApplicationAutomation singleton, which provides the same interface for automation as NavisWorksApplication in the Automation assembly
- `Bim360` (Autodesk.Navisworks.Api.ApplicationParts.IApplicationBim360, get) — Provides access to the BIM360 state of the application of which is hosting the API. Will be null if the application doesn't have BIM360 state.
- `Documents` (System.Collections.ObjectModel.ReadOnlyCollection<Autodesk.Navisworks.Api.Document>, get) — Gets a reference to the Read Only collection of documents currently open
- `DragDrop` (Autodesk.Navisworks.Api.ApplicationParts.ApplicationDragDrop, get) — Provides access to Drag Drop utilities.
- `Gui` (Autodesk.Navisworks.Api.ApplicationParts.IApplicationGui, get) — Provides information about the Gui of the application which is hosting the API. Will be null if the application doesn't have a Gui.
- `IsAutomated` (bool, get) — Gets whether this instance is running under Automation
- `IsUserDataCollectionEnabled` (bool, get) — Is user data collection enabled.
- `MainDocument` (Autodesk.Navisworks.Api.Document, get) — Gets a reference to the Primary Document being worked on. Once set the MainDocument does not change for the lifetime of the application.
- `Options` (Autodesk.Navisworks.Api.ApplicationParts.ApplicationOptions, get) — Provides access to Application Options.
- `Plugins` (Autodesk.Navisworks.Api.ApplicationParts.ApplicationPlugins, get) — Provides information about the Plugins available in the application runtime
- `Resources` (Autodesk.Navisworks.Api.ApplicationParts.ApplicationResources, get) — Provides access to Application Resources.
- `Title` (string, get) — Title for application. Suitable for display in window title bar, or similar.
- `Version` (Autodesk.Navisworks.Api.ApplicationParts.ApplicationVersion, get) — Provides information about the version of the currently running API and Runtime

### Events
#### `ActiveDocumentChanged` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> ActiveDocumentChanged`

Occurs when the ActiveDocument has changed

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Application.ActiveDocumentChanged)

#### `ActiveDocumentChanging` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> ActiveDocumentChanging`

Occurs when the ActiveDocument is about to change

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Application.ActiveDocumentChanging)

#### `Bim360Created` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> Bim360Created`

Occurs when the Application's BIM360 state been created. Bim360 will be non-null.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Application.Bim360Created)

#### `Bim360Destroying` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> Bim360Destroying`

Occurs when the Application's BIM360 state is about to be destroyed. Bim360 will be set to null after this event has been raised.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Application.Bim360Destroying)

#### `DocumentAdded` (`System.EventHandler<Autodesk.Navisworks.Api.DocumentEventArgs>`)

**Signature:** `event System.EventHandler<Autodesk.Navisworks.Api.DocumentEventArgs> DocumentAdded`

Occurs when a Document has been added to the Documents collection. Typically soon after the Document has been created.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Application.DocumentAdded)

#### `DocumentRemoved` (`System.EventHandler<Autodesk.Navisworks.Api.DocumentEventArgs>`)

**Signature:** `event System.EventHandler<Autodesk.Navisworks.Api.DocumentEventArgs> DocumentRemoved`

Occurs when a Document has been removed from the Documents collection. Typically shortly before the Document is destroyed.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Application.DocumentRemoved)

#### `FileInteractiveResolved` (`System.EventHandler<Autodesk.Navisworks.Api.FileInteractiveResolvedEventArgs>`)

**Signature:** `event System.EventHandler<Autodesk.Navisworks.Api.FileInteractiveResolvedEventArgs> FileInteractiveResolved`

Occurs when a file reference has been interactively resolved. Raised before FileResolved if automatic resolution has failed.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Application.FileInteractiveResolved)

#### `FileInteractiveResolving` (`System.EventHandler<Autodesk.Navisworks.Api.FileInteractiveResolvingEventArgs>`)

**Signature:** `event System.EventHandler<Autodesk.Navisworks.Api.FileInteractiveResolvingEventArgs> FileInteractiveResolving`

Occurs when a file reference needs to be interactively resolved. Raised after FileResolving if automatic resolution has failed.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Application.FileInteractiveResolving)

#### `FileResolved` (`System.EventHandler<Autodesk.Navisworks.Api.FileResolvedEventArgs>`)

**Signature:** `event System.EventHandler<Autodesk.Navisworks.Api.FileResolvedEventArgs> FileResolved`

Occurs when a file reference has been resolved. Typically when opening a file.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Application.FileResolved)

#### `FileResolving` (`System.EventHandler<Autodesk.Navisworks.Api.FileResolvingEventArgs>`)

**Signature:** `event System.EventHandler<Autodesk.Navisworks.Api.FileResolvingEventArgs> FileResolving`

Occurs when a file reference needs to be resolved. Typically when opening a file.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Application.FileResolving)

#### `GuiCreated` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> GuiCreated`

Occurs when the Application's GUI has been created. Gui will be non-null.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Application.GuiCreated)

#### `GuiDestroying` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> GuiDestroying`

Occurs when the Application's GUI is about to be destroyed. Gui will be set to null after this event has been raised.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Application.GuiDestroying)

#### `Idle` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> Idle`

Occurs when the application finishes processing and is about to enter the idle state.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Application.Idle)

#### `MainDocumentChanged` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> MainDocumentChanged`

Occurs when the MainDocument has changed

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Application.MainDocumentChanged)

#### `MainDocumentChanging` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> MainDocumentChanging`

Occurs when the MainDocument is about to change.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Application.MainDocumentChanging)

#### `ProgressBeginning` (`System.EventHandler<Autodesk.Navisworks.Api.ProgressBeginningEventArgs>`)

**Signature:** `event System.EventHandler<Autodesk.Navisworks.Api.ProgressBeginningEventArgs> ProgressBeginning`

Occurs when BeginProgress() is called. If any handler markes the event as handled, the standard progress dialog will not be displayed.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Application.ProgressBeginning)

#### `ProgressEnded` (`System.EventHandler<Autodesk.Navisworks.Api.ProgressEndedEventArgs>`)

**Signature:** `event System.EventHandler<Autodesk.Navisworks.Api.ProgressEndedEventArgs> ProgressEnded`

Occurs when EndProgress is called.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Application.ProgressEnded)

#### `ProgressErrorReporting` (`System.EventHandler<Autodesk.Navisworks.Api.ProgressErrorReportingEventArgs>`)

**Signature:** `event System.EventHandler<Autodesk.Navisworks.Api.ProgressErrorReportingEventArgs> ProgressErrorReporting`

Occurs when an error occurs during the lifetime of a Progress object. For example, if an error occurs whilst opening a large file the error can be displayed to the user who is offered the chance to cancel.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Application.ProgressErrorReporting)

#### `ProgressMessageReporting` (`System.EventHandler<Autodesk.Navisworks.Api.ProgressMessageReportingEventArgs>`)

**Signature:** `event System.EventHandler<Autodesk.Navisworks.Api.ProgressMessageReportingEventArgs> ProgressMessageReporting`

Occurs during Progress reporting when message describing current operation is updated.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Application.ProgressMessageReporting)

#### `ProgressSubOperationBegan` (`System.EventHandler<Autodesk.Navisworks.Api.ProgressSubOperationBeganEventArgs>`)

**Signature:** `event System.EventHandler<Autodesk.Navisworks.Api.ProgressSubOperationBeganEventArgs> ProgressSubOperationBegan`

Occurs when Progress.BeginSubOperation is called.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Application.ProgressSubOperationBegan)

#### `ProgressSubOperationEnded` (`System.EventHandler<Autodesk.Navisworks.Api.ProgressSubOperationEndedEventArgs>`)

**Signature:** `event System.EventHandler<Autodesk.Navisworks.Api.ProgressSubOperationEndedEventArgs> ProgressSubOperationEnded`

Occurs when Progress.EndSubOperation is called.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Application.ProgressSubOperationEnded)

#### `ProgressUpdating` (`System.EventHandler<Autodesk.Navisworks.Api.ProgressUpdatingEventArgs>`)

**Signature:** `event System.EventHandler<Autodesk.Navisworks.Api.ProgressUpdatingEventArgs> ProgressUpdating`

Occurs during Progress reporting when progress is updated.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Application.ProgressUpdating)

#### `ProgressVisibility` (`System.EventHandler<Autodesk.Navisworks.Api.ProgressVisibilityEventArgs>`)

**Signature:** `event System.EventHandler<Autodesk.Navisworks.Api.ProgressVisibilityEventArgs> ProgressVisibility`

Occurs when Progress.SetVisible is called.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Application.ProgressVisibility)

#### `UserMessagePosted` (`System.EventHandler<Autodesk.Navisworks.Api.UserMessageEventArgs>`)

**Signature:** `event System.EventHandler<Autodesk.Navisworks.Api.UserMessageEventArgs> UserMessagePosted`

UserMessagePosted event of Application.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Application.UserMessagePosted)

## Assignee (class)

Represents a person/company/role that an item can be assigned to.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Assignee)

### Constructors
- `Assignee()` — Constructs a new Assignee.
- `Assignee(Autodesk.Navisworks.Api.Assignee other)` — Constructs a new Assignee.
- `Assignee(string display_name)` — Constructs a new Assignee.
- `Assignee(string internal_id, string display_name)` — Constructs a new Assignee.

### Methods
#### `bool Equals(object obj)`

Determines whether the specified object and the current object refer to the same underlying native object. Returns false if either object has been disposed.

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Assignee.Equals%28System.Object%29)

#### `int GetHashCode()`

Serves as a hash function for a particular type. GetHashCode is suitable for use in hashing algorithms and data structures like a hash table.

**Returns:** `int` — A hash code for the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Assignee.GetHashCode)

#### `static Autodesk.Navisworks.Api.Assignee InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of Assignee.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Assignee` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Assignee.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of Assignee.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Assignee.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `string ToString()`

Returns a String that represents the current Object.

**Returns:** `string` — A String that represents the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Assignee.ToString)

#### `void Unassign()`

Unassign method of Assignee.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Assignee.Unassign)

### Properties
- `Assigned` (bool, get) — Is this assignee assigned.
- `DisplayName` (string, get/set) — Display name of the assignee.
- `InternalId` (string, get/set) — Internal ID of the assignee.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.

## AxisAndAngleResult (class)

Represents an axis and angle result

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.AxisAndAngleResult)

### Constructors
- `AxisAndAngleResult()` — Constructor for an angle/axis pair result.

### Properties
- `Angle` (double, get/set) — The angle in radians
- `Axis` (Autodesk.Navisworks.Api.UnitVector3D, get/set) — The axis

## BoundingBox2D (class)

Axis aligned 2D bounding box. Immutable.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.BoundingBox2D)

### Constructors
- `BoundingBox2D()` — Create an empty bounding box
- `BoundingBox2D(Autodesk.Navisworks.Api.Point2D minPoint, Autodesk.Navisworks.Api.Point2D maxPoint)` — Create a bounding box from a min and max point that define the opposite corners of the box

### Methods
#### `bool Contains(Autodesk.Navisworks.Api.BoundingBox2D box)`

True if this completely contains box

**Parameters:**
- `box` (Autodesk.Navisworks.Api.BoundingBox2D)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.BoundingBox2D.Contains%28Autodesk.Navisworks.Api.BoundingBox2D%29)

#### `bool Contains(Autodesk.Navisworks.Api.Point2D point)`

True if this contains point

**Parameters:**
- `point` (Autodesk.Navisworks.Api.Point2D)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.BoundingBox2D.Contains%28Autodesk.Navisworks.Api.Point2D%29)

#### `bool Equals(object obj)`

Determines whether the specified object and the current object refer to the same underlying native object. Returns false if either object has been disposed.

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.BoundingBox2D.Equals%28System.Object%29)

#### `Autodesk.Navisworks.Api.BoundingBox2D Extend(Autodesk.Navisworks.Api.BoundingBox2D box)`

Return box that represents union of this with another box

**Parameters:**
- `box` (Autodesk.Navisworks.Api.BoundingBox2D)

**Returns:** `Autodesk.Navisworks.Api.BoundingBox2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.BoundingBox2D.Extend%28Autodesk.Navisworks.Api.BoundingBox2D%29)

#### `int GetHashCode()`

Serves as a hash function for a particular type. GetHashCode is suitable for use in hashing algorithms and data structures like a hash table.

**Returns:** `int` — A hash code for the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.BoundingBox2D.GetHashCode)

#### `static Autodesk.Navisworks.Api.BoundingBox2D InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of BoundingBox2D.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.BoundingBox2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.BoundingBox2D.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of BoundingBox2D.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.BoundingBox2D.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `Autodesk.Navisworks.Api.BoundingBox2D Intersect(Autodesk.Navisworks.Api.BoundingBox2D box)`

Return box that represents intersection of this with another box

**Parameters:**
- `box` (Autodesk.Navisworks.Api.BoundingBox2D)

**Returns:** `Autodesk.Navisworks.Api.BoundingBox2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.BoundingBox2D.Intersect%28Autodesk.Navisworks.Api.BoundingBox2D%29)

#### `bool Intersects(Autodesk.Navisworks.Api.BoundingBox2D box)`

True if this intersects box

**Parameters:**
- `box` (Autodesk.Navisworks.Api.BoundingBox2D)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.BoundingBox2D.Intersects%28Autodesk.Navisworks.Api.BoundingBox2D%29)

#### `bool Intersects(Autodesk.Navisworks.Api.Point2D point)`

True if this intersects point (same as Contains() for a point)

**Parameters:**
- `point` (Autodesk.Navisworks.Api.Point2D)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.BoundingBox2D.Intersects%28Autodesk.Navisworks.Api.Point2D%29)

#### `string ToString()`

Returns a String that represents the current Object.

**Returns:** `string` — A String that represents the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.BoundingBox2D.ToString)

#### `Autodesk.Navisworks.Api.BoundingBox2D Translate(Autodesk.Navisworks.Api.Vector2D offset)`

Return this box translated by an offset

**Parameters:**
- `offset` (Autodesk.Navisworks.Api.Vector2D)

**Returns:** `Autodesk.Navisworks.Api.BoundingBox2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.BoundingBox2D.Translate%28Autodesk.Navisworks.Api.Vector2D%29)

### Properties
- `Area` (double, get) — Area of bounding box, 0 if empty
- `Center` (Autodesk.Navisworks.Api.Point2D, get) — Center of bounding box
- `HasArea` (bool, get) — Does bounding box have a non-zero area ?
- `IsEmpty` (bool, get) — Is bounding box empty ?
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Max` (Autodesk.Navisworks.Api.Point2D, get) — Return max point that defines corner of box with largest values for all components
- `Min` (Autodesk.Navisworks.Api.Point2D, get) — Return min point that defines corner of box with smallest values for all components
- `Size` (Autodesk.Navisworks.Api.Vector2D, get) — Extents of bounding box from min point to max point

## BoundingBox3D (class)

Axis aligned 3D bounding box. Immutable.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.BoundingBox3D)

### Constructors
- `BoundingBox3D()` — Create an empty bounding box
- `BoundingBox3D(Autodesk.Navisworks.Api.Point3D minPoint, Autodesk.Navisworks.Api.Point3D maxPoint)` — Create a bounding box from a min and max point that define the opposite corners of the box

### Methods
#### `Autodesk.Navisworks.Api.Point3D ClosestPoint(Autodesk.Navisworks.Api.Point3D point)`

Computes closest point in box to a given point

**Parameters:**
- `point` (Autodesk.Navisworks.Api.Point3D)

**Returns:** `Autodesk.Navisworks.Api.Point3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.BoundingBox3D.ClosestPoint%28Autodesk.Navisworks.Api.Point3D%29)

#### `bool Contains(Autodesk.Navisworks.Api.BoundingBox3D box)`

True if this completely contains box

**Parameters:**
- `box` (Autodesk.Navisworks.Api.BoundingBox3D)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.BoundingBox3D.Contains%28Autodesk.Navisworks.Api.BoundingBox3D%29)

#### `bool Contains(Autodesk.Navisworks.Api.Point3D point)`

True if this contains point

**Parameters:**
- `point` (Autodesk.Navisworks.Api.Point3D)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.BoundingBox3D.Contains%28Autodesk.Navisworks.Api.Point3D%29)

#### `bool Equals(object obj)`

Determines whether the specified object and the current object refer to the same underlying native object. Returns false if either object has been disposed.

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.BoundingBox3D.Equals%28System.Object%29)

#### `Autodesk.Navisworks.Api.BoundingBox3D Extend(Autodesk.Navisworks.Api.BoundingBox3D box)`

Return box that represents union of this with another box

**Parameters:**
- `box` (Autodesk.Navisworks.Api.BoundingBox3D)

**Returns:** `Autodesk.Navisworks.Api.BoundingBox3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.BoundingBox3D.Extend%28Autodesk.Navisworks.Api.BoundingBox3D%29)

#### `Autodesk.Navisworks.Api.BoundingBox3D Extend(Autodesk.Navisworks.Api.Point3D point)`

Return box that represents union of this with another point

**Parameters:**
- `point` (Autodesk.Navisworks.Api.Point3D)

**Returns:** `Autodesk.Navisworks.Api.BoundingBox3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.BoundingBox3D.Extend%28Autodesk.Navisworks.Api.Point3D%29)

#### `int GetHashCode()`

Serves as a hash function for a particular type. GetHashCode is suitable for use in hashing algorithms and data structures like a hash table.

**Returns:** `int` — A hash code for the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.BoundingBox3D.GetHashCode)

#### `static Autodesk.Navisworks.Api.BoundingBox3D InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of BoundingBox3D.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.BoundingBox3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.BoundingBox3D.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of BoundingBox3D.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.BoundingBox3D.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `Autodesk.Navisworks.Api.BoundingBox3D Intersect(Autodesk.Navisworks.Api.BoundingBox3D box)`

Return box that represents intersection of this with another box

**Parameters:**
- `box` (Autodesk.Navisworks.Api.BoundingBox3D)

**Returns:** `Autodesk.Navisworks.Api.BoundingBox3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.BoundingBox3D.Intersect%28Autodesk.Navisworks.Api.BoundingBox3D%29)

#### `bool Intersects(Autodesk.Navisworks.Api.BoundingBox3D box)`

True if this intersects box

**Parameters:**
- `box` (Autodesk.Navisworks.Api.BoundingBox3D)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.BoundingBox3D.Intersects%28Autodesk.Navisworks.Api.BoundingBox3D%29)

#### `bool Intersects(Autodesk.Navisworks.Api.Point3D point)`

True if this intersects point (same as Contains() for a point)

**Parameters:**
- `point` (Autodesk.Navisworks.Api.Point3D)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.BoundingBox3D.Intersects%28Autodesk.Navisworks.Api.Point3D%29)

#### `string ToString()`

Returns a String that represents the current Object.

**Returns:** `string` — A String that represents the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.BoundingBox3D.ToString)

#### `Autodesk.Navisworks.Api.BoundingBox3D Translate(Autodesk.Navisworks.Api.Vector3D offset)`

Return this box translated by an offset

**Parameters:**
- `offset` (Autodesk.Navisworks.Api.Vector3D)

**Returns:** `Autodesk.Navisworks.Api.BoundingBox3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.BoundingBox3D.Translate%28Autodesk.Navisworks.Api.Vector3D%29)

### Properties
- `Center` (Autodesk.Navisworks.Api.Point3D, get) — Center of bounding box
- `Empty` (Autodesk.Navisworks.Api.BoundingBox3D, get) — Returns an empty bounding box
- `HasSurfaceArea` (bool, get) — Does bounding box have a non-zero surface area ?
- `HasVolume` (bool, get) — Does bounding box have a non-zero volume ?
- `IsEmpty` (bool, get) — Is bounding box empty ?
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Max` (Autodesk.Navisworks.Api.Point3D, get) — Return max point that defines corner of box with largest values for all components
- `Min` (Autodesk.Navisworks.Api.Point3D, get) — Return min point that defines corner of box with smallest values for all components
- `Size` (Autodesk.Navisworks.Api.Vector3D, get) — Extents of bounding box from min point to max point
- `SurfaceArea` (double, get) — Sum of the areas of the faces of the bounding box, 0 if empty
- `Volume` (double, get) — Volume of bounding box, 0 if empty

## CameraMode (enum)

Describes the relationship of camera to viewer.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.CameraMode)

### Values
- `FirstPerson` = `0` — First person mode.
- `ThirdPerson` = `1` — Third person mode.

## CanceledOperationException (class)

Exception thrown when an operation could not complete because the end user has canceled it (usually through progress reporting).

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.CanceledOperationException)

### Constructors
- `CanceledOperationException()` — Creates a CanceledOperationException object
- `CanceledOperationException(string message)` — Creates a CanceledOperationException object
- `CanceledOperationException(string message, System.Exception innerException)` — Creates a CanceledOperationException object

## CivilAlignment (class)

CivilAlignment class in Autodesk.Navisworks.Api.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.CivilAlignment)

### Methods
#### `double GetDistanceForStation(double station)`

GetDistanceForStation method of CivilAlignment.

**Parameters:**
- `station` (double)

**Returns:** `double` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.CivilAlignment.GetDistanceForStation%28System.Double%29)

#### `int GetMajorStationDistance()`

GetMajorStationDistance method of CivilAlignment.

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.CivilAlignment.GetMajorStationDistance)

#### `double GetNextMajorStation(double station)`

GetNextMajorStation method of CivilAlignment.

**Parameters:**
- `station` (double)

**Returns:** `double` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.CivilAlignment.GetNextMajorStation%28System.Double%29)

#### `double GetNextStation(double station, double stationDelta)`

GetNextStation method of CivilAlignment.

**Parameters:**
- `station` (double)
- `stationDelta` (double)

**Returns:** `double` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.CivilAlignment.GetNextStation%28System.Double%2CSystem.Double%29)

#### `bool GetPointForDistance(double distance, out Autodesk.Navisworks.Api.Point3D position, out Autodesk.Navisworks.Api.UnitVector3D direction)`

GetPointForDistance method of CivilAlignment.

**Parameters:**
- `distance` (double)
- `position` (Autodesk.Navisworks.Api.Point3D)
- `direction` (Autodesk.Navisworks.Api.UnitVector3D)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.CivilAlignment.GetPointForDistance%28System.Double%2CAutodesk.Navisworks.Api.Point3D%40%2CAutodesk.Navisworks.Api.UnitVector3D%40%29)

#### `string GetStationLabel(double station)`

GetStationLabel method of CivilAlignment.

**Parameters:**
- `station` (double)

**Returns:** `string` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.CivilAlignment.GetStationLabel%28System.Double%29)

### Properties
- `EndStation` (double, get) — EndStation property of CivilAlignment.
- `Name` (string, get) — Name property of CivilAlignment.
- `StartStation` (double, get) — StartStation property of CivilAlignment.

## ClipPlane (class)

A representation of a clipping plane.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ClipPlane)

### Constructors
- `ClipPlane()` — Constructs a new ClipPlane.
- `ClipPlane(Autodesk.Navisworks.Api.ClipPlane other)` — Constructs a new ClipPlane.

### Methods
#### `void AlignToOrientation(Autodesk.Navisworks.Api.ClipPlaneAlignment align, Autodesk.Navisworks.Api.UnitVector3D up, Autodesk.Navisworks.Api.UnitVector3D front)`

AlignToOrientation method of ClipPlane.

**Parameters:**
- `align` (Autodesk.Navisworks.Api.ClipPlaneAlignment)
- `up` (Autodesk.Navisworks.Api.UnitVector3D)
- `front` (Autodesk.Navisworks.Api.UnitVector3D)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ClipPlane.AlignToOrientation%28Autodesk.Navisworks.Api.ClipPlaneAlignment%2CAutodesk.Navisworks.Api.UnitVector3D%2CAutodesk.Navisworks.Api.UnitVector3D%29)

#### `void AlignToPick(Autodesk.Navisworks.Api.ClipPlaneAlignment align, Autodesk.Navisworks.Api.Point3D origin, Autodesk.Navisworks.Api.UnitVector3D normal)`

AlignToPick method of ClipPlane.

**Parameters:**
- `align` (Autodesk.Navisworks.Api.ClipPlaneAlignment)
- `origin` (Autodesk.Navisworks.Api.Point3D)
- `normal` (Autodesk.Navisworks.Api.UnitVector3D)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ClipPlane.AlignToPick%28Autodesk.Navisworks.Api.ClipPlaneAlignment%2CAutodesk.Navisworks.Api.Point3D%2CAutodesk.Navisworks.Api.UnitVector3D%29)

#### `void AlignToPick(Autodesk.Navisworks.Api.ClipPlaneAlignment align, Autodesk.Navisworks.Api.Point3D origin, Autodesk.Navisworks.Api.UnitVector3D normal, Autodesk.Navisworks.Api.UnitVector3D xAxis)`

AlignToPick method of ClipPlane.

**Parameters:**
- `align` (Autodesk.Navisworks.Api.ClipPlaneAlignment)
- `origin` (Autodesk.Navisworks.Api.Point3D)
- `normal` (Autodesk.Navisworks.Api.UnitVector3D)
- `xAxis` (Autodesk.Navisworks.Api.UnitVector3D)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ClipPlane.AlignToPick%28Autodesk.Navisworks.Api.ClipPlaneAlignment%2CAutodesk.Navisworks.Api.Point3D%2CAutodesk.Navisworks.Api.UnitVector3D%2CAutodesk.Navisworks.Api.UnitVector3D%29)

#### `void ApplyTransform(Autodesk.Navisworks.Api.Transform3D trans)`

Transforms the clip plane.

**Parameters:**
- `trans` (Autodesk.Navisworks.Api.Transform3D)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ClipPlane.ApplyTransform%28Autodesk.Navisworks.Api.Transform3D%29)

#### `void CopyFrom(Autodesk.Navisworks.Api.ClipPlane from)`

CopyFrom method of ClipPlane.

**Parameters:**
- `from` (Autodesk.Navisworks.Api.ClipPlane)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ClipPlane.CopyFrom%28Autodesk.Navisworks.Api.ClipPlane%29)

#### `bool Initialize(Autodesk.Navisworks.Api.ClipPlaneAlignment align, Autodesk.Navisworks.Api.Point3D origin, Autodesk.Navisworks.Api.UnitVector3D up, Autodesk.Navisworks.Api.UnitVector3D front)`

Initialize method of ClipPlane.

**Parameters:**
- `align` (Autodesk.Navisworks.Api.ClipPlaneAlignment)
- `origin` (Autodesk.Navisworks.Api.Point3D)
- `up` (Autodesk.Navisworks.Api.UnitVector3D)
- `front` (Autodesk.Navisworks.Api.UnitVector3D)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ClipPlane.Initialize%28Autodesk.Navisworks.Api.ClipPlaneAlignment%2CAutodesk.Navisworks.Api.Point3D%2CAutodesk.Navisworks.Api.UnitVector3D%2CAutodesk.Navisworks.Api.UnitVector3D%29)

#### `bool Initialize(Autodesk.Navisworks.Api.ClipPlaneAlignment align, Autodesk.Navisworks.Api.Point3D origin, Autodesk.Navisworks.Api.UnitVector3D up, Autodesk.Navisworks.Api.UnitVector3D front, Autodesk.Navisworks.Api.UnitVector3D normal)`

Initialize method of ClipPlane.

**Parameters:**
- `align` (Autodesk.Navisworks.Api.ClipPlaneAlignment)
- `origin` (Autodesk.Navisworks.Api.Point3D)
- `up` (Autodesk.Navisworks.Api.UnitVector3D)
- `front` (Autodesk.Navisworks.Api.UnitVector3D)
- `normal` (Autodesk.Navisworks.Api.UnitVector3D)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ClipPlane.Initialize%28Autodesk.Navisworks.Api.ClipPlaneAlignment%2CAutodesk.Navisworks.Api.Point3D%2CAutodesk.Navisworks.Api.UnitVector3D%2CAutodesk.Navisworks.Api.UnitVector3D%2CAutodesk.Navisworks.Api.UnitVector3D%29)

#### `bool Initialize(Autodesk.Navisworks.Api.ClipPlaneAlignment align, Autodesk.Navisworks.Api.Point3D origin, Autodesk.Navisworks.Api.UnitVector3D up, Autodesk.Navisworks.Api.UnitVector3D front, Autodesk.Navisworks.Api.UnitVector3D normal, Autodesk.Navisworks.Api.UnitVector3D xAxis)`

Initialize method of ClipPlane.

**Parameters:**
- `align` (Autodesk.Navisworks.Api.ClipPlaneAlignment)
- `origin` (Autodesk.Navisworks.Api.Point3D)
- `up` (Autodesk.Navisworks.Api.UnitVector3D)
- `front` (Autodesk.Navisworks.Api.UnitVector3D)
- `normal` (Autodesk.Navisworks.Api.UnitVector3D)
- `xAxis` (Autodesk.Navisworks.Api.UnitVector3D)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ClipPlane.Initialize%28Autodesk.Navisworks.Api.ClipPlaneAlignment%2CAutodesk.Navisworks.Api.Point3D%2CAutodesk.Navisworks.Api.UnitVector3D%2CAutodesk.Navisworks.Api.UnitVector3D%2CAutodesk.Navisworks.Api.UnitVector3D%2CAutodesk.Navisworks.Api.UnitVector3D%29)

#### `bool Initialize(Autodesk.Navisworks.Api.ClipPlaneAlignment align, Autodesk.Navisworks.Api.UnitVector3D normal, double distance)`

Initialize method of ClipPlane.

**Parameters:**
- `align` (Autodesk.Navisworks.Api.ClipPlaneAlignment)
- `normal` (Autodesk.Navisworks.Api.UnitVector3D)
- `distance` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ClipPlane.Initialize%28Autodesk.Navisworks.Api.ClipPlaneAlignment%2CAutodesk.Navisworks.Api.UnitVector3D%2CSystem.Double%29)

#### `static Autodesk.Navisworks.Api.ClipPlane InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of ClipPlane.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.ClipPlane` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ClipPlane.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of ClipPlane.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ClipPlane.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `void Move(double dist)`

Move method of ClipPlane.

**Parameters:**
- `dist` (double)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ClipPlane.Move%28System.Double%29)

#### `void Reset()`

Reset method of ClipPlane.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ClipPlane.Reset)

#### `void Translate(Autodesk.Navisworks.Api.Vector3D offset)`

Translate method of ClipPlane.

**Parameters:**
- `offset` (Autodesk.Navisworks.Api.Vector3D)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ClipPlane.Translate%28Autodesk.Navisworks.Api.Vector3D%29)

### Properties
- `Alignment` (Autodesk.Navisworks.Api.ClipPlaneAlignment, get/set) — The clip plane alignment.
- `Default` (bool, get) — Is this clip plane in the default (uninitialized) state.
- `Distance` (double, get) — The distance of the plane from the world origin.
- `Enabled` (bool, get/set) — Enabled property of ClipPlane.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Normal` (Autodesk.Navisworks.Api.UnitVector3D, get/set) — The normal of the plane.
- `Origin` (Autodesk.Navisworks.Api.Point3D, get/set) — The origin of the plane.
- `Plane` (Autodesk.Navisworks.Api.Interop.LcLPlane3f, get) — Plane property of ClipPlane.
- `State` (Autodesk.Navisworks.Api.ClipPlaneState, get/set) — State property of ClipPlane.
- `Transform` (Autodesk.Navisworks.Api.Transform3D, get/set) — The transform of this plane.
- `XAxis` (Autodesk.Navisworks.Api.UnitVector3D, get) — Returns the X axis for the clip plane.

## ClipPlaneAlignment (enum)

Alignment of a clip plane.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ClipPlaneAlignment)

### Values
- `None` = `0` — Not aligned
- `View` = `1` — Aligned with the view
- `Back` = `2` — Aligned to the back
- `Front` = `3` — Aligned to the front
- `Left` = `4` — Aligned to the left
- `Right` = `5` — Aligned to the right
- `Top` = `6` — Aligned to the top
- `Bottom` = `7` — Aligned to the bottom
- `Surface` = `8` — Aligned with a surface
- `Line` = `9` — Aligned with a line
- `Custom` = `10` — Custom alignment

## ClipPlaneSet (class)

Represents the set of clip planes in a view.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ClipPlaneSet)

### Constructors
- `ClipPlaneSet()` — Constructs a new ClipPlaneSet.

### Methods
#### `bool Clips(Autodesk.Navisworks.Api.Point3D point)`

Clips method of ClipPlaneSet.

**Parameters:**
- `point` (Autodesk.Navisworks.Api.Point3D)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ClipPlaneSet.Clips%28Autodesk.Navisworks.Api.Point3D%29)

#### `void CopyFrom(Autodesk.Navisworks.Api.ClipPlaneSet other)`

CopyFrom method of ClipPlaneSet.

**Parameters:**
- `other` (Autodesk.Navisworks.Api.ClipPlaneSet)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ClipPlaneSet.CopyFrom%28Autodesk.Navisworks.Api.ClipPlaneSet%29)

#### `void FitToBox(Autodesk.Navisworks.Api.BoundingBox3D boundingBox)`

FitToBox method of ClipPlaneSet.

**Parameters:**
- `boundingBox` (Autodesk.Navisworks.Api.BoundingBox3D)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ClipPlaneSet.FitToBox%28Autodesk.Navisworks.Api.BoundingBox3D%29)

#### `Autodesk.Navisworks.Api.ClipPlane Get(int index)`

Get method of ClipPlaneSet.

**Parameters:**
- `index` (int)

**Returns:** `Autodesk.Navisworks.Api.ClipPlane` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ClipPlaneSet.Get%28System.Int32%29)

#### `void GetOrientedBox(Autodesk.Navisworks.Api.BoundingBox3D box, Autodesk.Navisworks.Api.Rotation3D rotation)`

GetOrientedBox method of ClipPlaneSet.

**Parameters:**
- `box` (Autodesk.Navisworks.Api.BoundingBox3D)
- `rotation` (Autodesk.Navisworks.Api.Rotation3D)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ClipPlaneSet.GetOrientedBox%28Autodesk.Navisworks.Api.BoundingBox3D%2CAutodesk.Navisworks.Api.Rotation3D%29)

#### `Autodesk.Navisworks.Api.Transform3D GetTransform(int index)`

GetTransform method of ClipPlaneSet.

**Parameters:**
- `index` (int)

**Returns:** `Autodesk.Navisworks.Api.Transform3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ClipPlaneSet.GetTransform%28System.Int32%29)

#### `static Autodesk.Navisworks.Api.ClipPlaneSet InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of ClipPlaneSet.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.ClipPlaneSet` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ClipPlaneSet.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of ClipPlaneSet.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ClipPlaneSet.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `bool IsOffsetPlanes(Autodesk.Navisworks.Api.ClipPlaneSet other)`

IsOffsetPlanes method of ClipPlaneSet.

**Parameters:**
- `other` (Autodesk.Navisworks.Api.ClipPlaneSet)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ClipPlaneSet.IsOffsetPlanes%28Autodesk.Navisworks.Api.ClipPlaneSet%29)

#### `void SetOrientedBox(Autodesk.Navisworks.Api.BoundingBox3D box, Autodesk.Navisworks.Api.Rotation3D rotation)`

SetOrientedBox method of ClipPlaneSet.

**Parameters:**
- `box` (Autodesk.Navisworks.Api.BoundingBox3D)
- `rotation` (Autodesk.Navisworks.Api.Rotation3D)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ClipPlaneSet.SetOrientedBox%28Autodesk.Navisworks.Api.BoundingBox3D%2CAutodesk.Navisworks.Api.Rotation3D%29)

#### `void SetTransform(int index, Autodesk.Navisworks.Api.Transform3D trans)`

SetTransform method of ClipPlaneSet.

**Parameters:**
- `index` (int)
- `trans` (Autodesk.Navisworks.Api.Transform3D)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ClipPlaneSet.SetTransform%28System.Int32%2CAutodesk.Navisworks.Api.Transform3D%29)

#### `int Size()`

Size method of ClipPlaneSet.

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ClipPlaneSet.Size)

#### `void Transform(Autodesk.Navisworks.Api.Transform3D trans)`

Transform method of ClipPlaneSet.

**Parameters:**
- `trans` (Autodesk.Navisworks.Api.Transform3D)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ClipPlaneSet.Transform%28Autodesk.Navisworks.Api.Transform3D%29)

#### `void Translate(int index, Autodesk.Navisworks.Api.Vector3D offset)`

Translate method of ClipPlaneSet.

**Parameters:**
- `index` (int)
- `offset` (Autodesk.Navisworks.Api.Vector3D)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ClipPlaneSet.Translate%28System.Int32%2CAutodesk.Navisworks.Api.Vector3D%29)

#### `bool ValueEquals(Autodesk.Navisworks.Api.ClipPlaneSet value)`

Performs comparison 'By Value'

**Parameters:**
- `value` (Autodesk.Navisworks.Api.ClipPlaneSet)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ClipPlaneSet.ValueEquals%28Autodesk.Navisworks.Api.ClipPlaneSet%29)

#### `static bool ValueEquals(Autodesk.Navisworks.Api.ClipPlaneSet value1, Autodesk.Navisworks.Api.ClipPlaneSet value2)`

Compares the two references 'By Value'

**Parameters:**
- `value1` (Autodesk.Navisworks.Api.ClipPlaneSet)
- `value2` (Autodesk.Navisworks.Api.ClipPlaneSet)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ClipPlaneSet.ValueEquals%28Autodesk.Navisworks.Api.ClipPlaneSet%2CAutodesk.Navisworks.Api.ClipPlaneSet%29)

### Properties
- `Box` (Autodesk.Navisworks.Api.BoundingBox3D, get/set) — The clipping box position.
- `BoxTransform` (Autodesk.Navisworks.Api.Transform3D, get) — The internal transform used for the section box.
- `CurrentPlane` (Autodesk.Navisworks.Api.ClipPlane, get) — The current clip plane.
- `CurrentPlaneIndex` (int, get/set) — The index of the current plane.
- `Enabled` (bool, get/set) — Is the clipping set enabled.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Linked` (bool, get/set) — Are the clip planes linked.
- `Mode` (Autodesk.Navisworks.Api.ClipPlaneSetMode, get/set) — The mode of the set.
- `Range` (Autodesk.Navisworks.Api.BoundingBox3D, get/set) — The range over which the clip planes operate.

## ClipPlaneSetMode (enum)

The mode of the clip plane set.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ClipPlaneSetMode)

### Values
- `Planes` = `0` — Planes
- `Box` = `1` — Box

## ClipPlaneState (enum)

Possible states a clip plane can be in.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ClipPlaneState)

### Values
- `Default` = `0` — Default (uninitialized)
- `Enabled` = `1` — Enabled
- `Disabled` = `2` — Disabled

## Color (class)

Represents a color as three floating-point components (r,g,b) with a normal range 0.0 to 1.0. Immutable.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Color)

### Constructors
- `Color(double red, double green, double blue)` — Create a color from float (r,g,b) components in the range 0 to 1

### Methods
#### `Autodesk.Navisworks.Api.Color Add(Autodesk.Navisworks.Api.Color color)`

Adds two colors to create another color. Same as operator+

**Parameters:**
- `color` (Autodesk.Navisworks.Api.Color)

**Returns:** `Autodesk.Navisworks.Api.Color` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Color.Add%28Autodesk.Navisworks.Api.Color%29)

#### `bool Equals(object obj)`

Determines whether the specified object and the current object refer to the same underlying native object. Returns false if either object has been disposed.

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Color.Equals%28System.Object%29)

#### `static Autodesk.Navisworks.Api.Color FromByteRGB(byte red, byte green, byte blue)`

Create a color from byte (r,g,b) components in the range 0 to 255

**Parameters:**
- `red` (byte)
- `green` (byte)
- `blue` (byte)

**Returns:** `Autodesk.Navisworks.Api.Color` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Color.FromByteRGB%28System.Byte%2CSystem.Byte%2CSystem.Byte%29)

#### `byte GetClampedByteValue(int index)`

Return color components as clamped bytes (in range 0 to 255)

**Parameters:**
- `index` (int)

**Returns:** `byte` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Color.GetClampedByteValue%28System.Int32%29)

#### `int GetHashCode()`

Serves as a hash function for a particular type. GetHashCode is suitable for use in hashing algorithms and data structures like a hash table.

**Returns:** `int` — A hash code for the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Color.GetHashCode)

#### `static Autodesk.Navisworks.Api.Color InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of Color.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Color` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Color.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of Color.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Color.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `Autodesk.Navisworks.Api.Color Multiply(double value)`

Scales intensity of color by value. Same as operator*

**Parameters:**
- `value` (double)

**Returns:** `Autodesk.Navisworks.Api.Color` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Color.Multiply%28System.Double%29)

#### `Autodesk.Navisworks.Api.Color Subtract(Autodesk.Navisworks.Api.Color color)`

Subtracts color to create another color. Same as operator-

**Parameters:**
- `color` (Autodesk.Navisworks.Api.Color)

**Returns:** `Autodesk.Navisworks.Api.Color` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Color.Subtract%28Autodesk.Navisworks.Api.Color%29)

#### `string ToString()`

Returns a String that represents the current Object.

**Returns:** `string` — A String that represents the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Color.ToString)

#### `Autodesk.Navisworks.Api.Vector3D ToVector3D()`

Convert color to a Vector3D

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Color.ToVector3D)

#### `static Autodesk.Navisworks.Api.Color op_Addition(Autodesk.Navisworks.Api.Color value1, Autodesk.Navisworks.Api.Color value2)`

Adds the R, G and B values of value1 and value2 and returns the resulting Color

**Parameters:**
- `value1` (Autodesk.Navisworks.Api.Color)
- `value2` (Autodesk.Navisworks.Api.Color)

**Returns:** `Autodesk.Navisworks.Api.Color` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Color.op_Addition%28Autodesk.Navisworks.Api.Color%2CAutodesk.Navisworks.Api.Color%29)

#### `static Autodesk.Navisworks.Api.Color op_Multiply(Autodesk.Navisworks.Api.Color color, double intensity)`

Multiplies the R, G and B values of color by intensity and returns the Resulting Color

**Parameters:**
- `color` (Autodesk.Navisworks.Api.Color)
- `intensity` (double)

**Returns:** `Autodesk.Navisworks.Api.Color` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Color.op_Multiply%28Autodesk.Navisworks.Api.Color%2CSystem.Double%29)

#### `static Autodesk.Navisworks.Api.Color op_Multiply(double intensity, Autodesk.Navisworks.Api.Color color)`

Multiplies the R, G and B values of color by intensity and returns the Resulting Color

**Parameters:**
- `intensity` (double)
- `color` (Autodesk.Navisworks.Api.Color)

**Returns:** `Autodesk.Navisworks.Api.Color` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Color.op_Multiply%28System.Double%2CAutodesk.Navisworks.Api.Color%29)

#### `static Autodesk.Navisworks.Api.Color op_Subtraction(Autodesk.Navisworks.Api.Color value1, Autodesk.Navisworks.Api.Color value2)`

Subtracts the R, G and B values of value2 from value1 and returns the resulting Color

**Parameters:**
- `value1` (Autodesk.Navisworks.Api.Color)
- `value2` (Autodesk.Navisworks.Api.Color)

**Returns:** `Autodesk.Navisworks.Api.Color` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Color.op_Subtraction%28Autodesk.Navisworks.Api.Color%2CAutodesk.Navisworks.Api.Color%29)

### Properties
- `B` (double, get) — Blue color component (in range 0 to 1)
- `Black` (Autodesk.Navisworks.Api.Color, get) — Returns a Color initialized to black
- `Blue` (Autodesk.Navisworks.Api.Color, get) — Returns a Color initialized to blue
- `G` (double, get) — Green color component (in range 0 to 1)
- `Green` (Autodesk.Navisworks.Api.Color, get) — Returns a Color initialized to green
- `IsBlack` (bool, get) — Is color black ?
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Item` (double, get) — Item property of Color.
- `R` (double, get) — Red color component (in range 0 to 1)
- `Red` (Autodesk.Navisworks.Api.Color, get) — Returns a Color initialized to red
- `White` (Autodesk.Navisworks.Api.Color, get) — Returns a Color initialized to white

## Comment (class)

A user comment. Immutable.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Comment)

### Constructors
- `Comment(Autodesk.Navisworks.Api.Comment from)` — Create a comment from a copy of another comment
- `Comment(string body, Autodesk.Navisworks.Api.CommentStatus status)` — Create a comment without a meaningfull id using current user as author. Use Document.CreateCommentWithUniqueId to create a comment with an unique id within a particular Document.
- `Comment(string body, Autodesk.Navisworks.Api.CommentStatus status, string author)` — Create a comment without a meaningfull id. Use Document.CreateCommentWithUniqueId to create a comment with an unique id within a particular Document.

### Methods
#### `bool Equals(object obj)`

Determines whether the specified object and the current object refer to the same underlying native object. Returns false if either object has been disposed.

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Comment.Equals%28System.Object%29)

#### `int GetHashCode()`

Serves as a hash function for a particular type. GetHashCode is suitable for use in hashing algorithms and data structures like a hash table.

**Returns:** `int` — A hash code for the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Comment.GetHashCode)

#### `static Autodesk.Navisworks.Api.Comment InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of Comment.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Comment` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Comment.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of Comment.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Comment.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `string ToString()`

Returns a String that represents the current Object.

**Returns:** `string` — A String that represents the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Comment.ToString)

### Properties
- `Author` (string, get) — Author (creator) of comment
- `Body` (string, get) — Body of comment
- `CreationDate` (System.DateTime, get) — Date and time at which comment was created
- `Id` (long, get) — Comment identifier. Unique within a Document. 0 if no meaningful id.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Status` (Autodesk.Navisworks.Api.CommentStatus, get) — Status of comment

## CommentCollection (class)

A collection of Comments

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.CommentCollection)

### Constructors
- `CommentCollection()` — Create an empty collection
- `CommentCollection(Autodesk.Navisworks.Api.CommentCollection from)` — Create a collection that is a copy of from

### Methods
#### `void Add(Autodesk.Navisworks.Api.Comment item)`

Adds item to the end of the collection.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.Comment)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.CommentCollection.Add%28Autodesk.Navisworks.Api.Comment%29)

#### `void AddRange(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.Comment> from)`

Add the items of the specified collection to the SavedItemCollection

**Parameters:**
- `from` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.Comment>) — The collection to add items from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.CommentCollection.AddRange%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.Comment%7D%29)

#### `void Clear()`

Removes all items from the collection

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.CommentCollection.Clear)

#### `bool Contains(Autodesk.Navisworks.Api.Comment item)`

Determines whether the SavedItemCollection contains a specific value.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.Comment)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.CommentCollection.Contains%28Autodesk.Navisworks.Api.Comment%29)

#### `void CopyFrom(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.Comment> from)`

Sets the contents of the collection to the items in from

**Parameters:**
- `from` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.Comment>) — The collection to copy contents from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.CommentCollection.CopyFrom%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.Comment%7D%29)

#### `void CopyTo(Autodesk.Navisworks.Api.Comment[] array, int arrayIndex)`

Copies the elements of the ICollection`1 to an Array, starting at a particular Array index.

**Parameters:**
- `array` (Autodesk.Navisworks.Api.Comment[]) — The one-dimensional Array that is the destination of the elements copied from ICollection`1. The Array must have zero-based indexing.
- `arrayIndex` (int) — The zero-based index in array at which copying begins.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.CommentCollection.CopyTo%28Autodesk.Navisworks.Api.Comment%5B%5D%2CSystem.Int32%29)

#### `void CopyTo(System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.Comment> to)`

Enumerates over the collection and copies the items into to

**Parameters:**
- `to` (System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.Comment>) — Collection to copy into

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.CommentCollection.CopyTo%28System.Collections.Generic.ICollection%7BAutodesk.Navisworks.Api.Comment%7D%29)

#### `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.Comment> GetEnumerator()`

Returns an enumerator that can iterate through the collection.

**Returns:** `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.Comment>` — An IEnumerator that can be used to iterate through the collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.CommentCollection.GetEnumerator)

#### `int IndexOf(Autodesk.Navisworks.Api.Comment item)`

Determines the index of a specific item in the Collection

**Parameters:**
- `item` (Autodesk.Navisworks.Api.Comment)

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.CommentCollection.IndexOf%28Autodesk.Navisworks.Api.Comment%29)

#### `void Insert(int index, Autodesk.Navisworks.Api.Comment item)`

Inserts item into the collection at the specified index

**Parameters:**
- `index` (int)
- `item` (Autodesk.Navisworks.Api.Comment)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.CommentCollection.Insert%28System.Int32%2CAutodesk.Navisworks.Api.Comment%29)

#### `static Autodesk.Navisworks.Api.CommentCollection InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of CommentCollection.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.CommentCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.CommentCollection.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of CommentCollection.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.CommentCollection.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `bool Remove(Autodesk.Navisworks.Api.Comment item)`

Removes the first occurrence of a specific item from the collection.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.Comment) — The object to remove from the collection.

**Returns:** `bool` — true if item was successfully removed from the collection; otherwise, false.
This method also returns false if item is not found in the original collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.CommentCollection.Remove%28Autodesk.Navisworks.Api.Comment%29)

#### `void RemoveAt(int index)`

Removes the IList`1 item at the specified index.

**Parameters:**
- `index` (int) — The zero-based index of the item to remove.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.CommentCollection.RemoveAt%28System.Int32%29)

### Properties
- `Count` (int, get) — Gets the number of elements contained in the collection.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Item` (Autodesk.Navisworks.Api.Comment, get/set) — Item property of CommentCollection.

## CommentStatus (enum)

Specifies status of comment.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.CommentStatus)

### Values
- `New` = `0` — Newly created comment.
- `Active` = `1` — Active issue.
- `Approved` = `2` — Issue has been approved.
- `Resolved` = `3` — Issue has been resolved.

## Cursor (enum)

Enumerates cursors used in input into Navisworks

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Cursor)

### Values
- `Unhandled` = `0` — Special case. Returned by GetCursor() if handled already.
- `Handled` = `1` — Handled Cursor.
- `Walk` = `2` — Walk Cursor.
- `Fly` = `3` — Fly Cursor.
- `Orbit` = `4` — Orbit Cursor.
- `Swivel` = `5` — Swivel Cursor.
- `Examine` = `6` — Examine Cursor.
- `Pan` = `7` — Pan Cursor.
- `Zoom` = `8` — Zoom Cursor.
- `Turntable` = `9` — Turntable Cursor.
- `Focus` = `10` — Focus Cursor.
- `Application` = `11` — Application Cursor.
- `ZoomBox` = `12` — ZoomBox Cursor.
- `Measure` = `13` — Measure Cursor.
- `HyperHand` = `14` — HyperHand Cursor.
- `PanWorld` = `15` — PanWorld Cursor.
- `Roll` = `16` — Roll Cursor.
- `Stop` = `17` — Stop Cursor.
- `MeasureEdge` = `18` — MeasureEdge Cursor.
- `MeasureVertex` = `19` — MeasureVertex Cursor.
- `Redline` = `20` — Redline Cursor.
- `Erase` = `21` — Erase Cursor.
- `Wheel` = `22` — Wheel Cursor.
- `MarkupSelection` = `23` — Markup Selection Cursor.
- `MarkupSnapping` = `24` — Markup Snapping Cursor.
- `MarkupEraser` = `25` — Markup Eraser Cursor.
- `MarkupQuickPick` = `26` — Markup QuickPick Cursor.
- `MarkupBucket` = `27` — Markup Bucket Cursor.
- `MarkupAutoPoly` = `28` — Markup AutoPoly Cursor.

## DataProperty (class)

Named property with data value. Immutable.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.DataProperty)

### Constructors
- `DataProperty(Autodesk.Navisworks.Api.NamedConstant combinedName, Autodesk.Navisworks.Api.VariantData data)` — Create a property from a combined name and display name
- `DataProperty(string name, string displayName, Autodesk.Navisworks.Api.VariantData value)` — Create a property from separate name and display name

### Methods
#### `bool Equals(object obj)`

Determines whether the specified object and the current object refer to the same underlying native object. Returns false if either object has been disposed.

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DataProperty.Equals%28System.Object%29)

#### `int GetHashCode()`

Serves as a hash function for a particular type. GetHashCode is suitable for use in hashing algorithms and data structures like a hash table.

**Returns:** `int` — A hash code for the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DataProperty.GetHashCode)

#### `static Autodesk.Navisworks.Api.DataProperty InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of DataProperty.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.DataProperty` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DataProperty.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of DataProperty.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DataProperty.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `Autodesk.Navisworks.Api.VariantData ParseCompatibleValue(string formattedValue)`

Parses formattedValue into a value compatible (same type and formatting) with the current Value.

**Parameters:**
- `formattedValue` (string)

**Returns:** `Autodesk.Navisworks.Api.VariantData` — Parsed value

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DataProperty.ParseCompatibleValue%28System.String%29)

#### `string ToString()`

Returns a String that represents the current Object.

**Returns:** `string` — A String that represents the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DataProperty.ToString)

#### `Autodesk.Navisworks.Api.VariantData TryParseCompatibleValue(string formattedValue)`

Try to parse formattedValue into a value compatible (same type and formatting) with the current Value. Returns a value with type of None on failure

**Parameters:**
- `formattedValue` (string)

**Returns:** `Autodesk.Navisworks.Api.VariantData` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DataProperty.TryParseCompatibleValue%28System.String%29)

### Properties
- `CombinedName` (Autodesk.Navisworks.Api.NamedConstant, get) — Combined name of property
- `DisplayName` (string, get) — Diplay name (localized, suitable for display to end user)
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Name` (string, get) — Name of property (suitable for programmatic use)
- `Value` (Autodesk.Navisworks.Api.VariantData, get) — Value of the property

## DataPropertyCollection (class)

A collection of DataProperty objects.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.DataPropertyCollection)

### Constructors
- `DataPropertyCollection()` — Create an empty collection
- `DataPropertyCollection(Autodesk.Navisworks.Api.DataPropertyCollection from)` — Create a collection that is a copy of from

### Methods
#### `void Add(Autodesk.Navisworks.Api.DataProperty item)`

Adds item to the end of the collection.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.DataProperty)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DataPropertyCollection.Add%28Autodesk.Navisworks.Api.DataProperty%29)

#### `void AddRange(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.DataProperty> from)`

Add the items of the specified collection to the SavedItemCollection

**Parameters:**
- `from` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.DataProperty>) — The collection to add items from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DataPropertyCollection.AddRange%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.DataProperty%7D%29)

#### `void Clear()`

Removes all items from the collection

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DataPropertyCollection.Clear)

#### `bool Contains(Autodesk.Navisworks.Api.DataProperty item)`

Determines whether the SavedItemCollection contains a specific value.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.DataProperty)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DataPropertyCollection.Contains%28Autodesk.Navisworks.Api.DataProperty%29)

#### `void CopyFrom(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.DataProperty> from)`

Sets the contents of the collection to the items in from

**Parameters:**
- `from` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.DataProperty>) — The collection to copy contents from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DataPropertyCollection.CopyFrom%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.DataProperty%7D%29)

#### `void CopyTo(Autodesk.Navisworks.Api.DataProperty[] array, int arrayIndex)`

Copies the elements of the ICollection`1 to an Array, starting at a particular Array index.

**Parameters:**
- `array` (Autodesk.Navisworks.Api.DataProperty[]) — The one-dimensional Array that is the destination of the elements copied from ICollection`1. The Array must have zero-based indexing.
- `arrayIndex` (int) — The zero-based index in array at which copying begins.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DataPropertyCollection.CopyTo%28Autodesk.Navisworks.Api.DataProperty%5B%5D%2CSystem.Int32%29)

#### `void CopyTo(System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.DataProperty> to)`

Enumerates over the collection and copies the items into to

**Parameters:**
- `to` (System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.DataProperty>) — Collection to copy into

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DataPropertyCollection.CopyTo%28System.Collections.Generic.ICollection%7BAutodesk.Navisworks.Api.DataProperty%7D%29)

#### `Autodesk.Navisworks.Api.DataProperty FindPropertyByCombinedName(Autodesk.Navisworks.Api.NamedConstant name)`

Find first property with specified CombinedName

**Parameters:**
- `name` (Autodesk.Navisworks.Api.NamedConstant)

**Returns:** `Autodesk.Navisworks.Api.DataProperty` — First matching property or null

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DataPropertyCollection.FindPropertyByCombinedName%28Autodesk.Navisworks.Api.NamedConstant%29)

#### `Autodesk.Navisworks.Api.DataProperty FindPropertyByDisplayName(string name)`

Find first property with specified DisplayName

**Parameters:**
- `name` (string)

**Returns:** `Autodesk.Navisworks.Api.DataProperty` — First matching property or null

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DataPropertyCollection.FindPropertyByDisplayName%28System.String%29)

#### `Autodesk.Navisworks.Api.DataProperty FindPropertyByName(string name)`

Find first property with specified Name

**Parameters:**
- `name` (string)

**Returns:** `Autodesk.Navisworks.Api.DataProperty` — First matching property or null

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DataPropertyCollection.FindPropertyByName%28System.String%29)

#### `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.DataProperty> GetEnumerator()`

Returns an enumerator that can iterate through the collection.

**Returns:** `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.DataProperty>` — An IEnumerator that can be used to iterate through the collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DataPropertyCollection.GetEnumerator)

#### `int IndexOf(Autodesk.Navisworks.Api.DataProperty item)`

Determines the index of a specific item in the Collection

**Parameters:**
- `item` (Autodesk.Navisworks.Api.DataProperty)

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DataPropertyCollection.IndexOf%28Autodesk.Navisworks.Api.DataProperty%29)

#### `void Insert(int index, Autodesk.Navisworks.Api.DataProperty item)`

Inserts item into the collection at the specified index

**Parameters:**
- `index` (int)
- `item` (Autodesk.Navisworks.Api.DataProperty)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DataPropertyCollection.Insert%28System.Int32%2CAutodesk.Navisworks.Api.DataProperty%29)

#### `static Autodesk.Navisworks.Api.DataPropertyCollection InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of DataPropertyCollection.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.DataPropertyCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DataPropertyCollection.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of DataPropertyCollection.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DataPropertyCollection.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `bool Remove(Autodesk.Navisworks.Api.DataProperty item)`

Removes the first occurrence of a specific item from the collection.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.DataProperty) — The object to remove from the collection.

**Returns:** `bool` — true if item was successfully removed from the collection; otherwise, false.
This method also returns false if item is not found in the original collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DataPropertyCollection.Remove%28Autodesk.Navisworks.Api.DataProperty%29)

#### `void RemoveAt(int index)`

Removes the IList`1 item at the specified index.

**Parameters:**
- `index` (int) — The zero-based index of the item to remove.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DataPropertyCollection.RemoveAt%28System.Int32%29)

### Properties
- `Count` (int, get) — Gets the number of elements contained in the collection.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Item` (Autodesk.Navisworks.Api.DataProperty, get/set) — Item property of DataPropertyCollection.

## DataPropertyNames (class)

Names (values of Name) of commonly found properties.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.DataPropertyNames)

### Properties
- `AutoCadEntityHandleValue` (string, get) — Name of AutoCad Entity Handle (AutoCadEntityHandle) "Value" property
- `ItemCreator` (string, get) — Name of Item "Creator" property (same as Creator)
- `ItemFileName` (string, get) — Name of Item "File Name" property (same as FileName)
- `ItemGuid` (string, get) — Name of Item "GUID" property (same as InstanceGuid)
- `ItemHidden` (string, get) — Name of Item "Hidden" property
- `ItemIcon` (string, get) — Name of Item "Icon" property
- `ItemInternalType` (string, get) — Name of Item "Internal Type" property (same as ClassName)
- `ItemLayer` (string, get) — Name of Item "Layer" property
- `ItemMaterial` (string, get) — Name of Item "Material" property
- `ItemName` (string, get) — Name of Item "Name" property (same as DisplayName)
- `ItemRequired` (string, get) — Name of Item "Required" property
- `ItemSourceFile` (string, get) — Name of Item "Source File" property
- `ItemSourceFileName` (string, get) — Name of Item "Source File Name" property (same as SourceFileName)
- `ItemType` (string, get) — Name of Item "Type" property (same as ClassDisplayName)
- `ItemUnit` (string, get) — Name of Item "Unit" property (same as Units)
- `MicrostationElementIdValue` (string, get) — Name of Microstation Element Id (MicrostationElementId) "Value" property
- `RevitElementIdValue` (string, get) — Name of Revit Element Id (RevitElementId) "Value" property

## Document (class)

Represents the contents of one or more loaded files.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Document)

### Methods
#### `void AppendFile(string fileName)`

Append file with files already in document, adding to current contents.

**Parameters:**
- `fileName` (string) — Path to try and append file from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.AppendFile%28System.String%29)

#### `void AppendFiles(System.Collections.Generic.IEnumerable<string> fileNames)`

Append files with files already in document, adding to current contents. More efficient than appending each file separately.

**Parameters:**
- `fileNames` (System.Collections.Generic.IEnumerable<string>) — Paths to append files from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.AppendFiles%28System.Collections.Generic.IEnumerable%7BSystem.String%7D%29)

#### `void AppendSheet(Autodesk.Navisworks.Api.SheetInfo sheet)`

Append contents of sheet to current active sheet.

**Parameters:**
- `sheet` (Autodesk.Navisworks.Api.SheetInfo) — SheetInfo to try and append sheet from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.AppendSheet%28Autodesk.Navisworks.Api.SheetInfo%29)

#### `Autodesk.Navisworks.Api.Transaction BeginTransaction(string displayName)`

Returns a new Transaction object. This is used to batch together edits. In the future it may support rollback.

**Parameters:**
- `displayName` (string) — String that may be displayed in the GUI for undoing this transaction.

**Returns:** `Autodesk.Navisworks.Api.Transaction` — New Transaction object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.BeginTransaction%28System.String%29)

#### `void Clear()`

Delete entire contents of document, returning to default (untitled, empty) state.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.Clear)

#### `void CloseFileHandles()`

Closes all file handles for currently loaded files, completing load for current sheet. File handles may be reopened in future (e.g. when changing the active sheet).

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.CloseFileHandles)

#### `void ConvertSheet(string sheetId)`

Convert sheet with sheetId.

**Parameters:**
- `sheetId` (string) — Id of sheet to convert

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.ConvertSheet%28System.String%29)

#### `Autodesk.Navisworks.Api.Comment CreateCommentWithUniqueId(string body, Autodesk.Navisworks.Api.CommentStatus status)`

Create a Comment with an Id that is unique within this Document

**Parameters:**
- `body` (string)
- `status` (Autodesk.Navisworks.Api.CommentStatus)

**Returns:** `Autodesk.Navisworks.Api.Comment` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.CreateCommentWithUniqueId%28System.String%2CAutodesk.Navisworks.Api.CommentStatus%29)

#### `Autodesk.Navisworks.Api.Comment CreateCommentWithUniqueId(string body, Autodesk.Navisworks.Api.CommentStatus status, string author)`

Create a Comment with an Id that is unique within this Document

**Parameters:**
- `body` (string)
- `status` (Autodesk.Navisworks.Api.CommentStatus)
- `author` (string)

**Returns:** `Autodesk.Navisworks.Api.Comment` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.CreateCommentWithUniqueId%28System.String%2CAutodesk.Navisworks.Api.CommentStatus%2CSystem.String%29)

#### `string DescribeAsAggregate()`

Describes current contents in terms of an JSON AggregateModelDefinition object.

**Returns:** `string` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.DescribeAsAggregate)

#### `void Dispose()`

Performs application-defined tasks associated with freeing, releasing, or resetting unmanaged resources.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.Dispose)

#### `void EndDisableUndo()`

Decrements an internal counter that determines if the undo mechanism is disabled. As the count goes from 1 to 0 Undo is re-enabled. Should be used with a matched call to StartDisableUndo.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.EndDisableUndo)

#### `void ExportAsDwf(string fileName)`

Exports the document as a 'DWF' file.

**Parameters:**
- `fileName` (string)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.ExportAsDwf%28System.String%29)

#### `void ExportToNwd(string fileName, Autodesk.Navisworks.Api.NwdExportOptions options)`

Exports contents of document to an NWD file..

**Parameters:**
- `fileName` (string) — Path to try and publish file to
- `options` (Autodesk.Navisworks.Api.NwdExportOptions) — Options to export file with

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.ExportToNwd%28System.String%2CAutodesk.Navisworks.Api.NwdExportOptions%29)

#### `System.Drawing.Bitmap GenerateImage(Autodesk.Navisworks.Api.ImageGenerationStyle style, int width, int height, bool enableSectioning)`

Generates an Image for this Document.

**Parameters:**
- `style` (Autodesk.Navisworks.Api.ImageGenerationStyle)
- `width` (int)
- `height` (int)
- `enableSectioning` (bool)

**Returns:** `System.Drawing.Bitmap` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.GenerateImage%28Autodesk.Navisworks.Api.ImageGenerationStyle%2CSystem.Int32%2CSystem.Int32%2CSystem.Boolean%29)

#### `System.Drawing.Bitmap GenerateImage(Autodesk.Navisworks.Api.ImageGenerationStyle style, int width, int height, double maxTimeHint, bool enableSectioning)`

Generates an Image for this Document.

**Parameters:**
- `style` (Autodesk.Navisworks.Api.ImageGenerationStyle)
- `width` (int)
- `height` (int)
- `maxTimeHint` (double)
- `enableSectioning` (bool)

**Returns:** `System.Drawing.Bitmap` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.GenerateImage%28Autodesk.Navisworks.Api.ImageGenerationStyle%2CSystem.Int32%2CSystem.Int32%2CSystem.Double%2CSystem.Boolean%29)

#### `Autodesk.Navisworks.Api.BoundingBox3D GetBoundingBox(bool ignoreHidden)`

Returns the bounding box for the entire document.

**Parameters:**
- `ignoreHidden` (bool) — If true, bounding box will not include hidden items, otherwise hidden items will be taken into account.

**Returns:** `Autodesk.Navisworks.Api.BoundingBox3D` — Bounding box for all items in the document.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.GetBoundingBox%28System.Boolean%29)

#### `System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.PointInfo> GetPointInfo(Autodesk.Navisworks.Api.Point3D worldPoint)`

Returns information relevant to the specified point.

**Parameters:**
- `worldPoint` (Autodesk.Navisworks.Api.Point3D) — A 3D point in the world

**Returns:** `System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.PointInfo>` — A list of PointInfo objects related to the world point.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.GetPointInfo%28Autodesk.Navisworks.Api.Point3D%29)

#### `void MergeFile(string fileName)`

Merge file into document, merging with current contents, ignoring duplicates.

**Parameters:**
- `fileName` (string) — Path to try and merge file from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.MergeFile%28System.String%29)

#### `void MergeFiles(System.Collections.Generic.IEnumerable<string> fileNames)`

Merge files with files already in document, merging with current contents. More efficient than merging each file separately.

**Parameters:**
- `fileNames` (System.Collections.Generic.IEnumerable<string>) — Paths to merge files from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.MergeFiles%28System.Collections.Generic.IEnumerable%7BSystem.String%7D%29)

#### `void MergeSheet(Autodesk.Navisworks.Api.SheetInfo sheet)`

Merge contents of sheet to current active sheet, ignoring duplicates.

**Parameters:**
- `sheet` (Autodesk.Navisworks.Api.SheetInfo) — SheetInfo to try and merge sheet from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.MergeSheet%28Autodesk.Navisworks.Api.SheetInfo%29)

#### `void OpenAggregate(string aggregateJson, string progressMedia)`

Loads aggregate into document, replacing current contents.

**Parameters:**
- `aggregateJson` (string) — Aggregate in the form of a JSON AggregateModelDefinition object.
- `progressMedia` (string)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.OpenAggregate%28System.String%2CSystem.String%29)

#### `void OpenFile(string fileName)`

Load file into document, replacing current contents.

**Parameters:**
- `fileName` (string) — Path to try and load file from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.OpenFile%28System.String%29)

#### `void PublishFile(string fileName, Autodesk.Navisworks.Api.NwdExportOptions options, Autodesk.Navisworks.Api.PublishProperties properties)`

Publish contents of document to a file.

**Parameters:**
- `fileName` (string) — Path to try and publish file to
- `options` (Autodesk.Navisworks.Api.NwdExportOptions)
- `properties` (Autodesk.Navisworks.Api.PublishProperties) — Publishing properties to publish file with

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.PublishFile%28System.String%2CAutodesk.Navisworks.Api.NwdExportOptions%2CAutodesk.Navisworks.Api.PublishProperties%29)

#### `void Redo()`

Redoes the last completed transaction and makes it available for Undo again.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.Redo)

#### `void RemoveFile(int index)`

Remove the file with the provided index from the document.

**Parameters:**
- `index` (int) — Index of the file to remove from the document

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.RemoveFile%28System.Int32%29)

#### `Autodesk.Navisworks.Api.SavedItem ResolveReference(Autodesk.Navisworks.Api.SavedItemReference reference)`

Finds an item specified by the reference within this document.

**Parameters:**
- `reference` (Autodesk.Navisworks.Api.SavedItemReference)

**Returns:** `Autodesk.Navisworks.Api.SavedItem` — Null if the item can't be found.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.ResolveReference%28Autodesk.Navisworks.Api.SavedItemReference%29)

#### `void Rollback()`

Rolls back (undoes) the last completed transaction. It will not be available for Redo.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.Rollback)

#### `void SaveFile(string fileName)`

Save contents of document to a file.

**Parameters:**
- `fileName` (string) — Path to try and save file to

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.SaveFile%28System.String%29)

#### `void SaveFile(string fileName, Autodesk.Navisworks.Api.DocumentFileVersion fileVersion)`

Save contents of document to a file using a specific file version.

**Parameters:**
- `fileName` (string) — Path to try and save file to
- `fileVersion` (Autodesk.Navisworks.Api.DocumentFileVersion)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.SaveFile%28System.String%2CAutodesk.Navisworks.Api.DocumentFileVersion%29)

#### `void SetActiveSheet(string sheetId)`

Switch to the sheet with Id sheetId.

**Parameters:**
- `sheetId` (string) — Id of sheet to make active

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.SetActiveSheet%28System.String%29)

#### `void SetGraduatedBackground(Autodesk.Navisworks.Api.Color topColor, Autodesk.Navisworks.Api.Color bottomColor)`

Sets a graduated background.

**Parameters:**
- `topColor` (Autodesk.Navisworks.Api.Color)
- `bottomColor` (Autodesk.Navisworks.Api.Color)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.SetGraduatedBackground%28Autodesk.Navisworks.Api.Color%2CAutodesk.Navisworks.Api.Color%29)

#### `void SetHorizonBackground(Autodesk.Navisworks.Api.Color skyTopColor, Autodesk.Navisworks.Api.Color skyBottomColor, Autodesk.Navisworks.Api.Color groundTopColor, Autodesk.Navisworks.Api.Color groundBottomColor)`

Sets a horizon background.

**Parameters:**
- `skyTopColor` (Autodesk.Navisworks.Api.Color)
- `skyBottomColor` (Autodesk.Navisworks.Api.Color)
- `groundTopColor` (Autodesk.Navisworks.Api.Color)
- `groundBottomColor` (Autodesk.Navisworks.Api.Color)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.SetHorizonBackground%28Autodesk.Navisworks.Api.Color%2CAutodesk.Navisworks.Api.Color%2CAutodesk.Navisworks.Api.Color%2CAutodesk.Navisworks.Api.Color%29)

#### `void SetPlainBackground(Autodesk.Navisworks.Api.Color color)`

Sets a plain background.

**Parameters:**
- `color` (Autodesk.Navisworks.Api.Color)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.SetPlainBackground%28Autodesk.Navisworks.Api.Color%29)

#### `void ShowMatchingItemInSheet(string sheetId, Autodesk.Navisworks.Api.ModelItem item)`

Search for items with same guid in item, if have, make the sheet with sheetId active, and select the matching model items.

**Parameters:**
- `sheetId` (string) — Id of sheet to search in
- `item` (Autodesk.Navisworks.Api.ModelItem) — ModelItem to search with same Guid

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.ShowMatchingItemInSheet%28System.String%2CAutodesk.Navisworks.Api.ModelItem%29)

#### `void StartDisableUndo()`

Increments an internal counter that determines if the undo mechanism is disabled. As the count goes from 0 to 1 Undo is disabled. Should be used with a matched call to EndDisableUndo.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.StartDisableUndo)

#### `string ToString()`

Returns a String that represents the current Object.

**Returns:** `string` — A String that represents the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.ToString)

#### `bool TryAppendFile(string fileName)`

Try to append file with files already in document, adding to current contents.

**Parameters:**
- `fileName` (string) — Path to try and append file from

**Returns:** `bool` — false if append fails

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.TryAppendFile%28System.String%29)

#### `bool TryAppendFiles(System.Collections.Generic.IEnumerable<string> fileNames)`

Try to append files with files already in document, adding to current contents. More efficient than appending each file separately.

**Parameters:**
- `fileNames` (System.Collections.Generic.IEnumerable<string>) — Paths to append files from

**Returns:** `bool` — false if append fails

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.TryAppendFiles%28System.Collections.Generic.IEnumerable%7BSystem.String%7D%29)

#### `bool TryAppendSheet(Autodesk.Navisworks.Api.SheetInfo sheet)`

Try to append contents of sheet to current active sheet.

**Parameters:**
- `sheet` (Autodesk.Navisworks.Api.SheetInfo) — SheetInfo to try and append sheet from

**Returns:** `bool` — false if append fails

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.TryAppendSheet%28Autodesk.Navisworks.Api.SheetInfo%29)

#### `bool TryCloseFileHandles()`

Try to close all file handles for currently loaded files, completing load for current sheet. File handles may be reopened in future (e.g. when changing the active sheet).

**Returns:** `bool` — false if load cannot be completed or file handles cannot be closed

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.TryCloseFileHandles)

#### `bool TryConvertSheet(string sheetId)`

Try to Convert sheet with sheetId.

**Parameters:**
- `sheetId` (string) — Id of sheet to try to convert

**Returns:** `bool` — false if sheet could not be converted

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.TryConvertSheet%28System.String%29)

#### `bool TryDescribeAsAggregate(out string aggregateJson)`

Describes current contents in terms of an JSON AggregateModelDefinition object.

**Parameters:**
- `aggregateJson` (string)

**Returns:** `bool` — false if current contents cannot be described as an aggregate.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.TryDescribeAsAggregate%28System.String%40%29)

#### `bool TryExportToNwd(string fileName, Autodesk.Navisworks.Api.NwdExportOptions options)`

Exports contents of document to an NWD file..

**Parameters:**
- `fileName` (string) — Path to try and publish file to
- `options` (Autodesk.Navisworks.Api.NwdExportOptions) — Options to export file with

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.TryExportToNwd%28System.String%2CAutodesk.Navisworks.Api.NwdExportOptions%29)

#### `bool TryMergeFile(string fileName)`

Try to merge file into document, merging with current contents, ignoring duplicates.

**Parameters:**
- `fileName` (string) — Path to try and merge file from

**Returns:** `bool` — false if merge fails

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.TryMergeFile%28System.String%29)

#### `bool TryMergeFiles(System.Collections.Generic.IEnumerable<string> fileNames)`

Try to merge files with files already in document, merging with current contents. More efficient than merging each file separately.

**Parameters:**
- `fileNames` (System.Collections.Generic.IEnumerable<string>) — Paths to merge files from

**Returns:** `bool` — false if merge fails

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.TryMergeFiles%28System.Collections.Generic.IEnumerable%7BSystem.String%7D%29)

#### `bool TryMergeSheet(Autodesk.Navisworks.Api.SheetInfo sheet)`

Try to merge contents of sheet to current active sheet, ignoring duplicates.

**Parameters:**
- `sheet` (Autodesk.Navisworks.Api.SheetInfo) — SheetInfo to try and merge sheet from

**Returns:** `bool` — false if merge fails

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.TryMergeSheet%28Autodesk.Navisworks.Api.SheetInfo%29)

#### `bool TryOpenAggregate(string aggregateJson, string progressMedia)`

Try to load aggregate into document, replacing current contents..

**Parameters:**
- `aggregateJson` (string) — Aggregate in the form of a JSON AggregateModelDefinition object.
- `progressMedia` (string)

**Returns:** `bool` — false if load fails

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.TryOpenAggregate%28System.String%2CSystem.String%29)

#### `bool TryOpenFile(string fileName)`

Try to load file into document, replacing current contents.

**Parameters:**
- `fileName` (string) — Path to try and load file from

**Returns:** `bool` — false if load fails

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.TryOpenFile%28System.String%29)

#### `bool TryPublishFile(string fileName, Autodesk.Navisworks.Api.NwdExportOptions options, Autodesk.Navisworks.Api.PublishProperties properties)`

Try to publish contents of document to a file.

**Parameters:**
- `fileName` (string) — Path to try and publish file to
- `options` (Autodesk.Navisworks.Api.NwdExportOptions)
- `properties` (Autodesk.Navisworks.Api.PublishProperties) — Publishing properties to publish file with

**Returns:** `bool` — false if publish fails

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.TryPublishFile%28System.String%2CAutodesk.Navisworks.Api.NwdExportOptions%2CAutodesk.Navisworks.Api.PublishProperties%29)

#### `bool TryRedo()`

Tries to redo the last completed transaction and make it available for Undo again.

**Returns:** `bool` — False if nothing can be redone or there is an active transaction in progress

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.TryRedo)

#### `bool TryRemoveFile(int index)`

Try to remove the file with the provided index from the document.

**Parameters:**
- `index` (int) — Index of the file to remove from the document

**Returns:** `bool` — false if the file could not be removed

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.TryRemoveFile%28System.Int32%29)

#### `bool TryRollback()`

Tries to roll back (undo) the last completed transaction and make it available for Redo.

**Returns:** `bool` — False if nothing can be undone or there is an active transaction in progress

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.TryRollback)

#### `bool TrySaveFile(string fileName)`

Try to save contents of document to a file.

**Parameters:**
- `fileName` (string) — Path to try and save file to

**Returns:** `bool` — false if save fails

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.TrySaveFile%28System.String%29)

#### `bool TrySaveFile(string fileName, Autodesk.Navisworks.Api.DocumentFileVersion fileVersion)`

Try to save contents of document to a file using a specific file version.

**Parameters:**
- `fileName` (string) — Path to try and save file to
- `fileVersion` (Autodesk.Navisworks.Api.DocumentFileVersion)

**Returns:** `bool` — false if save fails

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.TrySaveFile%28System.String%2CAutodesk.Navisworks.Api.DocumentFileVersion%29)

#### `bool TrySetActiveSheet(string sheetId)`

Try to switch to the sheet with Id sheetId.

**Parameters:**
- `sheetId` (string) — Id of sheet to make active

**Returns:** `bool` — false if sheet could not be made active

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.TrySetActiveSheet%28System.String%29)

#### `bool TryUndo()`

Tries to undo the last completed transaction and make it available for Redo.

**Returns:** `bool` — False if nothing can be undone or there is an active transaction in progress

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.TryUndo)

#### `void Undo()`

Undoes the last completed transaction and makes it available for Redo.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.Undo)

#### `bool UpdateFiles()`

Updates loaded files to match files on disk (i.e. if file has been changed since it was first loaded).

**Returns:** `bool` — true if anything in document was updated

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Document.UpdateFiles)

### Properties
- `ActiveSheet` (Autodesk.Navisworks.Api.SheetInfo, get) — Gets current active sheet
- `ActiveSheetType` (Autodesk.Navisworks.Api.SheetType, get) — Gets current active sheet type
- `ActiveView` (Autodesk.Navisworks.Api.View, get) — Gets a reference to the currently active View
- `Alignments` (Autodesk.Navisworks.Api.DocumentParts.DocumentAlignments, get) — Gets a DocumentAlignments object containing the Civil Alignment data.
- `CanDescribeAsAggregate` (bool, get) — Determines if current contents can described in terms of an JSON AggregateModelDefinition object.
- `Clash` (Autodesk.Navisworks.Api.DocumentParts.IDocumentClash, get) — Gets the associated DocumentClash part as discoverable IDocumentClash
- `CurrentComments` (Autodesk.Navisworks.Api.DocumentParts.DocumentCurrentComments, get) — Gets a DocumentCurrentComments object containing the current Comments data.
- `CurrentFileName` (string, get) — Current file name. Different from suggested file name in that SuggestedFileName will return a friendly name for a remote file on the basis that it expects you are trying to save locally and will return a "Untitled" name for files that aren't saved yet.
- `CurrentMeasurement` (Autodesk.Navisworks.Api.DocumentParts.DocumentCurrentMeasurement, get) — Gets a DocumentCurrentMeasurement object containing the current measurement data.
- `CurrentSearch` (Autodesk.Navisworks.Api.DocumentParts.DocumentCurrentSearch, get) — Gets a DocumentSearch object containing the current search.
- `CurrentSelection` (Autodesk.Navisworks.Api.DocumentParts.DocumentCurrentSelection, get) — Gets a DocumentCurrentSelection object containing the current selection.
- `CurrentViewpoint` (Autodesk.Navisworks.Api.DocumentParts.DocumentCurrentViewpoint, get) — Gets a DocumentCurrentViewpoint object containing the current Viewpoint data.
- `Database` (Autodesk.Navisworks.Api.DocumentParts.DocumentDatabase, get) — Gets a DocumentDatabase object containing embedded database.
- `DocumentInfo` (Autodesk.Navisworks.Api.DocumentParts.DocumentInfoPart, get) — Gets DocumentInfoPart
- `FileName` (string, get) — File name (or URL) for document. May be null if document is empty or has no associated file name.
- `FileVersion` (Autodesk.Navisworks.Api.DocumentFileVersion, get) — Current file version for document.
- `FrontRightTopViewUpVector` (Autodesk.Navisworks.Api.Vector3D, get) — Returns view direction up vector to look at model from front-right-top direction.
- `FrontRightTopViewVector` (Autodesk.Navisworks.Api.Vector3D, get) — Returns view direction to look at model from front-right-top direction.
- `FrontVector` (Autodesk.Navisworks.Api.Vector3D, get) — Natural front vector for this model. Will be Zero if no front vector defined.
- `Grids` (Autodesk.Navisworks.Api.DocumentParts.DocumentGrids, get) — Gets a DocumentGrids object containing the Grids data.
- `HomeView` (Autodesk.Navisworks.Api.Viewpoint, get) — The home view for the current document (if one is defined).
- `Hyperlinks` (Autodesk.Navisworks.Api.DocumentParts.DocumentHyperlinks, get) — Gets a DocumentHyperlinks object containing document hyperlinks.
- `IsActiveTransaction` (bool, get) — Is there a transaction in progress ?
- `IsClear` (bool, get) — Is document in its default (untitled, empty) state ?
- `IsDisposed` (bool, get) — Has this document been disposed ?
- `IsModified` (bool, get) — Returns true if modifications requiring a save have been performed
- `IsUndoDisabled` (bool, get) — Gets the disabled state of the Undo mechanism.
- `Models` (Autodesk.Navisworks.Api.DocumentParts.DocumentModels, get) — Gets a DocumentModels object containing the current list of Models.
- `NextRedo` (string, get) — Display name of transaction that will be redone next time Redo() is called.
- `NextUndo` (string, get) — Display name of transaction that will be undone next time Undo() is called.
- `RightVector` (Autodesk.Navisworks.Api.Vector3D, get) — Natural right vector for this model. Will be Zero if no right vector defined.
- `SavedViewpoints` (Autodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints, get) — Gets a DocumentCurrentViewpoint object containing the current Viewpoint data.
- `SelectionSets` (Autodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets, get) — Gets the SelectionSets stored in the document.
- `State` (Autodesk.Navisworks.Api.Interop.LcOwDocument, get) — State property of Document.
- `SuggestedFileName` (string, get) — Suggested file name which document could be saved to. Always a valid filename (absolute if file already exists, relative if suggested name for new file).
- `Takeoff` (Autodesk.Navisworks.Api.DocumentParts.IDocumentTakeoff, get) — Gets the associated DocumentTakeoff part as discoverable IDocumentTakeoff
- `Timeliner` (Autodesk.Navisworks.Api.DocumentParts.IDocumentTimeliner, get) — Gets the associated DocumentTimeliner part as discoverable IDocumentTimeliner
- `Title` (string, get) — Title for document. Suitable for display in window title bar, or similar.
- `Tool` (Autodesk.Navisworks.Api.DocumentParts.DocumentTool, get) — Gets a DocumentTool object containing the active Tool.
- `Units` (Autodesk.Navisworks.Api.Units, get) — Units for document. Apply to all geometry, properties, transforms, viewpoints, etc. in the document.
- `UpVector` (Autodesk.Navisworks.Api.Vector3D, get) — Natural up vector for this model. Will be Zero if no up vector defined.
- `Views` (System.Collections.ObjectModel.ReadOnlyCollection<Autodesk.Navisworks.Api.View>, get) — Gets a ReadOnlyCollection object containing the current list of Views of the Document.

### Events
#### `ActiveSheetChanged` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> ActiveSheetChanged`

Occurs when the ActiveSheet has changed.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Document.ActiveSheetChanged)

#### `ActiveSheetChanging` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> ActiveSheetChanging`

Occurs when the ActiveSheet is about to change.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Document.ActiveSheetChanging)

#### `ActiveViewChanged` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> ActiveViewChanged`

Occurs when the ActiveView has changed

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Document.ActiveViewChanged)

#### `ActiveViewChanging` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> ActiveViewChanging`

Occurs when the ActiveView is about to change

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Document.ActiveViewChanging)

#### `FileNameChanged` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> FileNameChanged`

Occurs when the file name of the Document is changed.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Document.FileNameChanged)

#### `FileSaved` (`System.EventHandler<Autodesk.Navisworks.Api.FileSaveEventArgs>`)

**Signature:** `event System.EventHandler<Autodesk.Navisworks.Api.FileSaveEventArgs> FileSaved`

Occurs when the document has been saved into a file

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Document.FileSaved)

#### `FileSaving` (`System.EventHandler<Autodesk.Navisworks.Api.FileSaveEventArgs>`)

**Signature:** `event System.EventHandler<Autodesk.Navisworks.Api.FileSaveEventArgs> FileSaving`

Occurs when the document is about to be saved into a file

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Document.FileSaving)

#### `FilesUpdated` (`System.EventHandler<Autodesk.Navisworks.Api.FileUpdateEventArgs>`)

**Signature:** `event System.EventHandler<Autodesk.Navisworks.Api.FileUpdateEventArgs> FilesUpdated`

Occurs when the document has updated files.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Document.FilesUpdated)

#### `FilesUpdating` (`System.EventHandler<Autodesk.Navisworks.Api.FileUpdateEventArgs>`)

**Signature:** `event System.EventHandler<Autodesk.Navisworks.Api.FileUpdateEventArgs> FilesUpdating`

Occurs when the document is about to update files.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Document.FilesUpdating)

#### `TransactionBeginning` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> TransactionBeginning`

Occurs when a Transaction (multiple batched edits) begins, before any of the edits in the transaction take place.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Document.TransactionBeginning)

#### `TransactionEnded` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> TransactionEnded`

Occurs when a Transaction (multiple batched edits) has ended and all the edits have been completed or rolled back.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Document.TransactionEnded)

#### `UnitsChanged` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> UnitsChanged`

Occurs when the units of the Document are changed.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Document.UnitsChanged)

#### `ViewAdded` (`System.EventHandler<Autodesk.Navisworks.Api.ViewEventArgs>`)

**Signature:** `event System.EventHandler<Autodesk.Navisworks.Api.ViewEventArgs> ViewAdded`

Occurs when a View has been added to the Views collection. Typically soon after the View has been created.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Document.ViewAdded)

#### `ViewRemoved` (`System.EventHandler<Autodesk.Navisworks.Api.ViewEventArgs>`)

**Signature:** `event System.EventHandler<Autodesk.Navisworks.Api.ViewEventArgs> ViewRemoved`

Occurs when a View has been removed from the Views collection. Typically shortly before the View is destroyed.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.Document.ViewRemoved)

## DocumentEventArgs (class)

Event arguments used for Document related events

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.DocumentEventArgs)

### Properties
- `Document` (Autodesk.Navisworks.Api.Document, get/set) — Gets the Document that the event is describing.

## DocumentFileException (class)

The exception that is thrown by file operation methods in Document

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.DocumentFileException)

### Constructors
- `DocumentFileException()` — Creates a DocumentFileException
- `DocumentFileException(string message)` — Creates a DocumentFileException
- `DocumentFileException(string message, System.Exception innerException)` — Creates a DocumentFileException

## DocumentFileVersion (enum)

Enumeration of Navisworks Document file format versions.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.DocumentFileVersion)

### Values
- `Current` = `0` — Use the most recent version of the file format supported by the runtime
- `Navisworks2015` = `441` — Version of file format used by Navisworks 2015
- `Navisworks2016` = `448` — Version of file format used by Navisworks 2016
- `Navisworks2017` = `448` — Version of file format used by Navisworks 2017
- `Navisworks2018` = `448` — Version of file format used by Navisworks 2018
- `Navisworks2019` = `448` — Version of file format used by Navisworks 2019
- `Navisworks2020` = `448` — Version of file format used by Navisworks 2020
- `Navisworks2021` = `448` — Version of file format used by Navisworks 2021
- `Navisworks2022` = `448` — Version of file format used by Navisworks 2022
- `Navisworks2023` = `448` — Version of file format used by Navisworks 2023
- `Navisworks2024` = `448` — Version of file format used by Navisworks 2024
- `Navisworks2025` = `448` — Version of file format used by Navisworks 2025
- `Navisworks2026` = `451` — Version of file format used by Navisworks 2026

## DocumentInfo (class)

Information about a Document that can be accessed without loading the entire document

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.DocumentInfo)

### Constructors
- `DocumentInfo()` — Create an empty DocumentInfoDocumentInfo

### Methods
#### `Autodesk.Navisworks.Api.SheetInfo FindSheet(string id)`

Find Sheet by sheet Id

**Parameters:**
- `id` (string)

**Returns:** `Autodesk.Navisworks.Api.SheetInfo` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentInfo.FindSheet%28System.String%29)

#### `int IndexOfSheetId(string id)`

Get Index by sheet Id

**Parameters:**
- `id` (string)

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentInfo.IndexOfSheetId%28System.String%29)

#### `static Autodesk.Navisworks.Api.DocumentInfo InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of DocumentInfo.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.DocumentInfo` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentInfo.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of DocumentInfo.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentInfo.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

### Properties
- `CurrentSheet` (Autodesk.Navisworks.Api.SheetInfo, get) — Get the currently active sheet
- `CurrentSheetId` (string, get) — Get the id of the currently active sheet
- `FileVersionGuid` (System.Guid, get) — Get Guid generated when current version of corresponding file was saved. Changes whenever document is saved.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `PropertyCategories` (Autodesk.Navisworks.Api.InfoPropertyCategoryCollection, get) — Get the property category collection
- `SourceGuid` (System.Guid, get) — Get original Guid when FileInfo was first created. Once set remains the same while Guid changes when saving to a file of a different type.

## DocumentInfoParts (enum)

Specifies parts of a DocumentInfo

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.DocumentInfoParts)

### Values
- `CurrentSheetId` = `256` — Current sheet Id

## EulerAngleResult (class)

Represents Euler angles returned from a method

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.EulerAngleResult)

### Constructors
- `EulerAngleResult()` — Constructor for a Euler angle result.

### Properties
- `X` (double, get/set) — 'X' Euler angle
- `Y` (double, get/set) — 'Y' Euler angle
- `Z` (double, get/set) — 'Z' Euler angle

## FileInteractiveResolvedEventArgs (class)

Event arguments used for the Application::FileInteractiveResolved event

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.FileInteractiveResolvedEventArgs)

### Properties
- `ResolvedFileReference` (string, get) — Get file reference that FileReference has been resolved to, or null if not resolved
- `Response` (Autodesk.Navisworks.Api.FileResolutionResponse, get) — Get user's response from interactive dialog.

## FileInteractiveResolvingEventArgs (class)

Event arguments used for the Application::FileInteractiveResolving event

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.FileInteractiveResolvingEventArgs)

### Properties
- `DisplayString` (string, get) — Get string to display to the end user explaining what sort of file is being resolved
- `Handled` (bool, get/set) — Set whether event handler has handled interactive file resolution. If set to true, default interactive file resolution will not occur.
- `ResolvedFileReference` (string, get/set) — Set file reference that FileReference has been resolved to by interactively asking user. Leave as null if file can’t be resolved.
- `Response` (Autodesk.Navisworks.Api.FileResolutionResponse, get/set) — Set user's response from interactive dialog.

## FileReferenceResolveResult (class)

Result of calling FileReferenceResolver::Resolve

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.FileReferenceResolveResult)

### Properties
- `FileNameToOpen` (string, get/set) — File name that can be opened to read contents of file referred to
- `ResolvedFileReference` (string, get/set) — Resolved (updated) reference
- `Response` (Autodesk.Navisworks.Api.FileResolutionResponse, get/set) — OK if reference was resolved successfully

## FileReferenceResolver (class)

Utility class used to resolve a file reference to a real file

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.FileReferenceResolver)

### Constructors
- `FileReferenceResolver()` — Default constructor

### Methods
#### `static void InternalClearGlobalCache()`

InternalClearGlobalCache method of FileReferenceResolver.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.FileReferenceResolver.InternalClearGlobalCache)

#### `static Autodesk.Navisworks.Api.FileReferenceResolver InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of FileReferenceResolver.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.FileReferenceResolver` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.FileReferenceResolver.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of FileReferenceResolver.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.FileReferenceResolver.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `Autodesk.Navisworks.Api.FileReferenceResolveResult Resolve(string fileReference)`

Resolve a file reference (absolute path, relative path, URL, ...)

**Parameters:**
- `fileReference` (string)

**Returns:** `Autodesk.Navisworks.Api.FileReferenceResolveResult` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.FileReferenceResolver.Resolve%28System.String%29)

#### `Autodesk.Navisworks.Api.FileReferenceResolveResult Resolve(string fileReference, bool resolveToOpen)`

Resolve a file reference (absolute path, relative path, URL, ...)

**Parameters:**
- `fileReference` (string)
- `resolveToOpen` (bool) — Specify whether result should include FileNameToOpen

**Returns:** `Autodesk.Navisworks.Api.FileReferenceResolveResult` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.FileReferenceResolver.Resolve%28System.String%2CSystem.Boolean%29)

### Properties
- `InteractiveDisplayString` (string, get/set) — String to display to end user if reference has to be resolved interactively
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `ReferringFileName` (string, get/set) — File name that is referring to file to be resolved, empty if not known
- `ReferringFileNameAsSaved` (string, get/set) — File name that is referring to file to be resolved as it was when saved, empty if not known

## FileReferenceType (enum)

Specifies type of FileReference

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.FileReferenceType)

### Values
- `None` = `0` — Null or empty file reference
- `Local` = `1` — Absolute path to file on disk
- `Remote` = `2` — URL for remote file
- `Embedded` = `3` — File embedded in referring file
- `Relative` = `4` — Path relative to referring file
- `Unknown` = `5` — Unknown type of file reference

## FileResolutionEventArgs (class)

Base class for file resolution related events

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.FileResolutionEventArgs)

### Properties
- `FileReference` (string, get) — Get file reference to be resolved
- `FileReferenceType` (Autodesk.Navisworks.Api.FileReferenceType, get) — Get type of file reference to be resolved
- `ReferringFileName` (string, get) — Get file name that is referring to file to be resolved, or null if not known
- `ReferringFileNameAsSaved` (string, get) — Get file name that is referring to file to be resolved as it was when saved, or null if not known
- `ResolveToOpen` (bool, get) — Get whether file is being resolved in order to open it.

## FileResolutionResponse (enum)

User response from request for file resolution

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.FileResolutionResponse)

### Values
- `OK` = `0` — User has supplied an updated file reference
- `Cancel` = `1` — User wants to cancel the entire operation
- `Ignore` = `2` — User wants to ignore this unresolved file
- `IgnoreAll` = `3` — User wants to ignore this and any other unresolved files

## FileResolvedEventArgs (class)

Event arguments used for the Application::FileResolved event

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.FileResolvedEventArgs)

### Properties
- `FileNameToOpen` (string, get) — Get file name to open to read contents of resolved reference. Same as ResolvedFileReference if file is local. Null if not resolved or file is remote and has not been downloaded.
- `ResolvedFileReference` (string, get) — Get file reference that FileReference has been resolved to, or null if not resolved
- `ResolvedFileReferenceType` (Autodesk.Navisworks.Api.FileReferenceType, get) — Get type of file reference to be resolved

## FileResolvingEventArgs (class)

Event arguments used for the Application::FileResolving event

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.FileResolvingEventArgs)

### Properties
- `FileNameToOpen` (string, get/set) — Set file name to open to read contents of resolved file. Leave as null if ResolveToOpen is false or file can’t be resolved.
- `Handled` (bool, get/set) — Set whether event handler has handled file resolution. If set to true, default file resolution will not occur.
- `ResolvedFileReference` (string, get/set) — Set file reference that FileReference has been resolved to. Leave as null if file can’t be resolved.

## FileSaveEventArgs (class)

Event arguments used for Document SaveFile related events

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.FileSaveEventArgs)

### Properties
- `FileName` (string, get) — Gets the name of the file that is being saved to
- `IsPublish` (bool, get) — Gets whether the save is a publish
- `SaveComplete` (bool, get) — Gets whether the save has completed successfully

## FileUpdateEventArgs (class)

Event arguments used for Document FileUpdate related events

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.FileUpdateEventArgs)

### Properties
- `AnyFilesUpdated` (bool, get) — Gets whether any files were updated.

## FolderItem (class)

A simple folder of saved items.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.FolderItem)

### Constructors
- `FolderItem()` — Create an empty folder

### Methods
#### `static Autodesk.Navisworks.Api.FolderItem InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of FolderItem.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.FolderItem` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.FolderItem.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of FolderItem.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.FolderItem.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

### Properties
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.

## Graphics (class)

Class encapsulating API for the rendering of client graphics.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Graphics)

### Methods
#### `void Arc(Autodesk.Navisworks.Api.Point2D centre, Autodesk.Navisworks.Api.Point2D start, Autodesk.Navisworks.Api.Point2D end, bool filled)`

Renders a 2D arc in the XY plane at the current Z depth.

**Parameters:**
- `centre` (Autodesk.Navisworks.Api.Point2D) — Centre point of the arc.
- `start` (Autodesk.Navisworks.Api.Point2D) — Start point of the arc.
- `end` (Autodesk.Navisworks.Api.Point2D) — End point of the arc.
- `filled` (bool) — Should the arc be filled in or an outline.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.Arc%28Autodesk.Navisworks.Api.Point2D%2CAutodesk.Navisworks.Api.Point2D%2CAutodesk.Navisworks.Api.Point2D%2CSystem.Boolean%29)

#### `void Arc(Autodesk.Navisworks.Api.Point3D center, Autodesk.Navisworks.Api.Point3D start, Autodesk.Navisworks.Api.Point3D end, Autodesk.Navisworks.Api.Vector3D normal, bool filled)`

Renders a 3D arc.

**Parameters:**
- `center` (Autodesk.Navisworks.Api.Point3D) — Center point of the arc.
- `start` (Autodesk.Navisworks.Api.Point3D) — Start point of the arc.
- `end` (Autodesk.Navisworks.Api.Point3D) — End point of the arc.
- `normal` (Autodesk.Navisworks.Api.Vector3D) — Normal for arc.
- `filled` (bool) — Should the arc be filled in or an outline.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.Arc%28Autodesk.Navisworks.Api.Point3D%2CAutodesk.Navisworks.Api.Point3D%2CAutodesk.Navisworks.Api.Point3D%2CAutodesk.Navisworks.Api.Vector3D%2CSystem.Boolean%29)

#### `void BeginComplexPolygon(Autodesk.Navisworks.Api.VertexProperties vertexProperties)`

Begin rendering a complex polygon.

**Parameters:**
- `vertexProperties` (Autodesk.Navisworks.Api.VertexProperties) — Vertex properties for the polygon.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.BeginComplexPolygon%28Autodesk.Navisworks.Api.VertexProperties%29)

#### `void BeginComplexPolygonContour()`

Begin complex polygon contour.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.BeginComplexPolygonContour)

#### `void BeginModelContext()`

Begin rendering in the Model Context.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.BeginModelContext)

#### `void BeginPolygon(Autodesk.Navisworks.Api.VertexProperties vertexProperties)`

Begins a convex polygon.

**Parameters:**
- `vertexProperties` (Autodesk.Navisworks.Api.VertexProperties) — Vertex properties for the polygon.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.BeginPolygon%28Autodesk.Navisworks.Api.VertexProperties%29)

#### `void BeginUserContext()`

Begin rendering in a user-defined context.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.BeginUserContext)

#### `void BeginWindowContext()`

Begin rendering in the Window Context.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.BeginWindowContext)

#### `void BezierSegment(Autodesk.Navisworks.Api.Point2D start, Autodesk.Navisworks.Api.Point2D control, Autodesk.Navisworks.Api.Point2D end)`

Renders a 2D segment of a 3rd order Bezier curve.

**Parameters:**
- `start` (Autodesk.Navisworks.Api.Point2D) — Start point of the segment.
- `control` (Autodesk.Navisworks.Api.Point2D) — Control point for segment.
- `end` (Autodesk.Navisworks.Api.Point2D)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.BezierSegment%28Autodesk.Navisworks.Api.Point2D%2CAutodesk.Navisworks.Api.Point2D%2CAutodesk.Navisworks.Api.Point2D%29)

#### `void BezierSegment(Autodesk.Navisworks.Api.Point2D start, Autodesk.Navisworks.Api.Point2D control, Autodesk.Navisworks.Api.Point2D end, int resolution)`

Renders a 2D segment of a 3rd order Bezier curve.

**Parameters:**
- `start` (Autodesk.Navisworks.Api.Point2D) — Start point of the segment.
- `control` (Autodesk.Navisworks.Api.Point2D) — Control point for segment.
- `end` (Autodesk.Navisworks.Api.Point2D)
- `resolution` (int) — Number of points to evaluate along the Bezier.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.BezierSegment%28Autodesk.Navisworks.Api.Point2D%2CAutodesk.Navisworks.Api.Point2D%2CAutodesk.Navisworks.Api.Point2D%2CSystem.Int32%29)

#### `void Bitmap(int index, Autodesk.Navisworks.Api.Point2D origin, double alpha)`

Renders a registered bitmap

**Parameters:**
- `index` (int) — Index of the registered bitmap to render.
- `origin` (Autodesk.Navisworks.Api.Point2D) — Origin point of the bitmap.
- `alpha` (double) — Overall alpha value for the bitmap.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.Bitmap%28System.Int32%2CAutodesk.Navisworks.Api.Point2D%2CSystem.Double%29)

#### `void Bitmap(int index, Autodesk.Navisworks.Api.Point2D origin, uint width, uint height, double alpha)`

Renders a registered bitmap

**Parameters:**
- `index` (int) — Index of the registered bitmap to render.
- `origin` (Autodesk.Navisworks.Api.Point2D) — Origin point of the bitmap.
- `width` (uint) — Width of the image you want to render.
- `height` (uint) — Height of the image you want to render.
- `alpha` (double) — Overall alpha value for the bitmap.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.Bitmap%28System.Int32%2CAutodesk.Navisworks.Api.Point2D%2CSystem.UInt32%2CSystem.UInt32%2CSystem.Double%29)

#### `void Blend(bool blendOn)`

Enables or disables blending.

**Parameters:**
- `blendOn` (bool) — Should blending be enabled.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.Blend%28System.Boolean%29)

#### `bool CanRenderText2D(Autodesk.Navisworks.Api.TextFontInfo fontInfo, string text)`

Check to see if you can render a given text string with given font info.

**Parameters:**
- `fontInfo` (Autodesk.Navisworks.Api.TextFontInfo)
- `text` (string)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.CanRenderText2D%28Autodesk.Navisworks.Api.TextFontInfo%2CSystem.String%29)

#### `void Circle(Autodesk.Navisworks.Api.Point2D centre, double radius, bool filled)`

Renders a 2D circle in the XY plane at the current Z depth.

**Parameters:**
- `centre` (Autodesk.Navisworks.Api.Point2D) — Centre point of the circle.
- `radius` (double) — Radius of circle.
- `filled` (bool) — Should the circle be filled in or an outline.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.Circle%28Autodesk.Navisworks.Api.Point2D%2CSystem.Double%2CSystem.Boolean%29)

#### `void Circle(Autodesk.Navisworks.Api.Point3D centre, Autodesk.Navisworks.Api.Vector3D normal, double radius, bool filled)`

Renders a 3D circle in the model space.

**Parameters:**
- `centre` (Autodesk.Navisworks.Api.Point3D) — Centre point of the circle.
- `normal` (Autodesk.Navisworks.Api.Vector3D) — normal for the circle.
- `radius` (double) — Radius of circle.
- `filled` (bool) — Should the circle be filled in or an outline.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.Circle%28Autodesk.Navisworks.Api.Point3D%2CAutodesk.Navisworks.Api.Vector3D%2CSystem.Double%2CSystem.Boolean%29)

#### `void Color(Autodesk.Navisworks.Api.Color colorValue, double alphaValue)`

Sets the current render color.

**Parameters:**
- `colorValue` (Autodesk.Navisworks.Api.Color) — Color value.
- `alphaValue` (double) — Alpha value. Range of 0.0 to 1.0. 0.0 is fully transparent, 1.0 is fully opaque.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.Color%28Autodesk.Navisworks.Api.Color%2CSystem.Double%29)

#### `void Cone(Autodesk.Navisworks.Api.Point3D start, Autodesk.Navisworks.Api.Point3D end, double startRadius, double endRadius)`

Renders a 3D cone.

**Parameters:**
- `start` (Autodesk.Navisworks.Api.Point3D) — Start point of centre axis.
- `end` (Autodesk.Navisworks.Api.Point3D) — End point of centre axis.
- `startRadius` (double) — Radius at start point of cone.
- `endRadius` (double) — Radius at end point of cone.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.Cone%28Autodesk.Navisworks.Api.Point3D%2CAutodesk.Navisworks.Api.Point3D%2CSystem.Double%2CSystem.Double%29)

#### `void Cuboid(Autodesk.Navisworks.Api.Point3D origin, Autodesk.Navisworks.Api.Vector3D xVector, Autodesk.Navisworks.Api.Vector3D yVector, Autodesk.Navisworks.Api.Vector3D zVector, bool filled)`

Renders a 3D cuboid.

**Parameters:**
- `origin` (Autodesk.Navisworks.Api.Point3D) — Origin point of rectangle.
- `xVector` (Autodesk.Navisworks.Api.Vector3D) — Vector representing 'x' side of cuboid.
- `yVector` (Autodesk.Navisworks.Api.Vector3D) — Vector representing 'y' side of cuboid.
- `zVector` (Autodesk.Navisworks.Api.Vector3D) — Vector representing 'y' side of cuboid.
- `filled` (bool) — Should the cuboid be filled in or an outline.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.Cuboid%28Autodesk.Navisworks.Api.Point3D%2CAutodesk.Navisworks.Api.Vector3D%2CAutodesk.Navisworks.Api.Vector3D%2CAutodesk.Navisworks.Api.Vector3D%2CSystem.Boolean%29)

#### `void Cylinder(Autodesk.Navisworks.Api.Point3D start, Autodesk.Navisworks.Api.Point3D end, double radius)`

Renders a 3D cylinder.

**Parameters:**
- `start` (Autodesk.Navisworks.Api.Point3D) — Start point of centre axis.
- `end` (Autodesk.Navisworks.Api.Point3D) — End point of centre axis.
- `radius` (double) — Radius of cylinder.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.Cylinder%28Autodesk.Navisworks.Api.Point3D%2CAutodesk.Navisworks.Api.Point3D%2CSystem.Double%29)

#### `void DepthMask(bool depthMaskOn)`

Enables or disables depth buffer writing.

**Parameters:**
- `depthMaskOn` (bool) — Should writing to the depth buffer be enabled.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.DepthMask%28System.Boolean%29)

#### `void DepthTest(bool depthTestOn)`

Enables or disables depth testing.

**Parameters:**
- `depthTestOn` (bool) — Should depth testing be enabled.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.DepthTest%28System.Boolean%29)

#### `void Ellipse(Autodesk.Navisworks.Api.Point2D centre, double xRadius, double yRadius, bool filled)`

Renders a 2D axis-aligned ellipse in the XY plane at the current Z depth.

**Parameters:**
- `centre` (Autodesk.Navisworks.Api.Point2D) — Centre point of the ellipse.
- `xRadius` (double) — Length of X radius.
- `yRadius` (double) — Length of Y radius.
- `filled` (bool) — Should the ellipse be filled in or an outline.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.Ellipse%28Autodesk.Navisworks.Api.Point2D%2CSystem.Double%2CSystem.Double%2CSystem.Boolean%29)

#### `void Ellipse(Autodesk.Navisworks.Api.Point2D centre, double xRadius, double yRadius, bool filled, int resolution)`

Renders a 2D axis-aligned ellipse in the XY plane at the current Z depth.

**Parameters:**
- `centre` (Autodesk.Navisworks.Api.Point2D) — Centre point of the ellipse.
- `xRadius` (double) — Length of X radius.
- `yRadius` (double) — Length of Y radius.
- `filled` (bool) — Should the ellipse be filled in or an outline.
- `resolution` (int) — Number of points used to draw ellipse.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.Ellipse%28Autodesk.Navisworks.Api.Point2D%2CSystem.Double%2CSystem.Double%2CSystem.Boolean%2CSystem.Int32%29)

#### `void EndComplexPolygon()`

Ends current complex polygon.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.EndComplexPolygon)

#### `void EndComplexPolygonContour()`

Ends current complex polygon contour.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.EndComplexPolygonContour)

#### `void EndModelContext()`

End rendering in the Model context.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.EndModelContext)

#### `void EndPolygon()`

End current convex polygon.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.EndPolygon)

#### `void EndUserContext()`

End rendering in the User context.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.EndUserContext)

#### `void EndWindowContext()`

End rendering in the Window Context.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.EndWindowContext)

#### `bool GetFontMetrics(Autodesk.Navisworks.Api.TextFontInfo fontInfo, Autodesk.Navisworks.Api.TextFontMetrics metrics)`

Returns font metrics.

**Parameters:**
- `fontInfo` (Autodesk.Navisworks.Api.TextFontInfo)
- `metrics` (Autodesk.Navisworks.Api.TextFontMetrics)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.GetFontMetrics%28Autodesk.Navisworks.Api.TextFontInfo%2CAutodesk.Navisworks.Api.TextFontMetrics%29)

#### `static Autodesk.Navisworks.Api.Graphics InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of Graphics.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Graphics` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of Graphics.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `void Lighting(bool lightingOn)`

Enables or disables lighting.

**Parameters:**
- `lightingOn` (bool) — Should lighting be enabled.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.Lighting%28System.Boolean%29)

#### `void Line(Autodesk.Navisworks.Api.Point2D point1, Autodesk.Navisworks.Api.Point2D point2)`

Renders a 2D line in the XY plane at the current depth.

**Parameters:**
- `point1` (Autodesk.Navisworks.Api.Point2D) — First point of line.
- `point2` (Autodesk.Navisworks.Api.Point2D) — Second point of line.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.Line%28Autodesk.Navisworks.Api.Point2D%2CAutodesk.Navisworks.Api.Point2D%29)

#### `void Line(Autodesk.Navisworks.Api.Point3D start, Autodesk.Navisworks.Api.Point3D end)`

Renders a 3D line.

**Parameters:**
- `start` (Autodesk.Navisworks.Api.Point3D) — Start point.
- `end` (Autodesk.Navisworks.Api.Point3D) — End point.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.Line%28Autodesk.Navisworks.Api.Point3D%2CAutodesk.Navisworks.Api.Point3D%29)

#### `void LineStipple(int factor, ushort pattern)`

Sets the current line stipple mode.

**Parameters:**
- `factor` (int) — Factor to stretch pattern by. May be clamped.
- `pattern` (ushort) — Bit pattern to stipple line with.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.LineStipple%28System.Int32%2CSystem.UInt16%29)

#### `void LineWidth(double width)`

Sets the current line width.

**Parameters:**
- `width` (double) — line width to render with.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.LineWidth%28System.Double%29)

#### `void Point(Autodesk.Navisworks.Api.Point2D point2D)`

Renders a 2D point in the XY plane at the current depth.

**Parameters:**
- `point2D` (Autodesk.Navisworks.Api.Point2D) — Position to render point.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.Point%28Autodesk.Navisworks.Api.Point2D%29)

#### `void Point(Autodesk.Navisworks.Api.Point3D point3D)`

Renders a 3D point.

**Parameters:**
- `point3D` (Autodesk.Navisworks.Api.Point3D) — Position to render point.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.Point%28Autodesk.Navisworks.Api.Point3D%29)

#### `void Polyline2D(Autodesk.Navisworks.Api.Point2DList points)`

Renders a 2D polyline in the XY plane at the current depth.

**Parameters:**
- `points` (Autodesk.Navisworks.Api.Point2DList) — Array of each point

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.Polyline2D%28Autodesk.Navisworks.Api.Point2DList%29)

#### `void Polyline3D(Autodesk.Navisworks.Api.Point3DList points)`

Renders a 3D polyline.

**Parameters:**
- `points` (Autodesk.Navisworks.Api.Point3DList) — Array of each point

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.Polyline3D%28Autodesk.Navisworks.Api.Point3DList%29)

#### `void Rectangle(Autodesk.Navisworks.Api.Point2D bottomLeft, Autodesk.Navisworks.Api.Point2D topRight, bool filled)`

Renders a 2D rectangle in the XY plane at the current Z depth.

**Parameters:**
- `bottomLeft` (Autodesk.Navisworks.Api.Point2D) — Bottom-left coordinate of rectangle.
- `topRight` (Autodesk.Navisworks.Api.Point2D) — Top-right coordinate of rectangle.
- `filled` (bool) — Should the rectangle be filled in or an outline.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.Rectangle%28Autodesk.Navisworks.Api.Point2D%2CAutodesk.Navisworks.Api.Point2D%2CSystem.Boolean%29)

#### `void Rectangle(Autodesk.Navisworks.Api.Point3D origin, Autodesk.Navisworks.Api.Vector3D xVector, Autodesk.Navisworks.Api.Vector3D yVector, bool filled)`

Renders a 3D rectangle.

**Parameters:**
- `origin` (Autodesk.Navisworks.Api.Point3D) — Origin point of rectangle.
- `xVector` (Autodesk.Navisworks.Api.Vector3D) — Vector representing 'x' side of rectangle.
- `yVector` (Autodesk.Navisworks.Api.Vector3D) — Vector representing 'y' side of rectangle.
- `filled` (bool) — Should the rectangle be filled in or an outline.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.Rectangle%28Autodesk.Navisworks.Api.Point3D%2CAutodesk.Navisworks.Api.Vector3D%2CAutodesk.Navisworks.Api.Vector3D%2CSystem.Boolean%29)

#### `void SetTexture(int imageId)`

Set texture image index.

**Parameters:**
- `imageId` (int) — image index registered by Application.RegisterBitmap API.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.SetTexture%28System.Int32%29)

#### `void Sphere(Autodesk.Navisworks.Api.Point3D center, double radius)`

Renders a 3D sphere.

**Parameters:**
- `center` (Autodesk.Navisworks.Api.Point3D) — Center of sphere.
- `radius` (double) — Radius of sphere.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.Sphere%28Autodesk.Navisworks.Api.Point3D%2CSystem.Double%29)

#### `Autodesk.Navisworks.Api.TextExtents2D Text2D(Autodesk.Navisworks.Api.TextFontInfo fontInfo, string text, Autodesk.Navisworks.Api.Point2D origin, int offsetX, int offsetY)`

Renders 2D text.Text is rendered in 2D as bitmaps. It always faces the viewer.

**Parameters:**
- `fontInfo` (Autodesk.Navisworks.Api.TextFontInfo)
- `text` (string)
- `origin` (Autodesk.Navisworks.Api.Point2D)
- `offsetX` (int)
- `offsetY` (int)

**Returns:** `Autodesk.Navisworks.Api.TextExtents2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.Text2D%28Autodesk.Navisworks.Api.TextFontInfo%2CSystem.String%2CAutodesk.Navisworks.Api.Point2D%2CSystem.Int32%2CSystem.Int32%29)

#### `Autodesk.Navisworks.Api.TextExtents2D Text2DExtents(Autodesk.Navisworks.Api.TextFontInfo fontInfo, string text)`

Returns the extents of 2D text.

**Parameters:**
- `fontInfo` (Autodesk.Navisworks.Api.TextFontInfo)
- `text` (string)

**Returns:** `Autodesk.Navisworks.Api.TextExtents2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.Text2DExtents%28Autodesk.Navisworks.Api.TextFontInfo%2CSystem.String%29)

#### `void TextureCoordinate(Autodesk.Navisworks.Api.Point2D coordinate)`

Specifies texture co-ordinate for next polygon vertex.

**Parameters:**
- `coordinate` (Autodesk.Navisworks.Api.Point2D) — Texture co-ordinate.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.TextureCoordinate%28Autodesk.Navisworks.Api.Point2D%29)

#### `void Triangle(Autodesk.Navisworks.Api.Point2D point1, Autodesk.Navisworks.Api.Point2D point2, Autodesk.Navisworks.Api.Point2D point3, bool filled)`

Renders a 2D triangle in the XY plane at the current Z depth.

**Parameters:**
- `point1` (Autodesk.Navisworks.Api.Point2D) — First point of triangle.
- `point2` (Autodesk.Navisworks.Api.Point2D) — Second point of triangle.
- `point3` (Autodesk.Navisworks.Api.Point2D) — Third point of triangle.
- `filled` (bool) — Should the triangle be filled in or an outline.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.Triangle%28Autodesk.Navisworks.Api.Point2D%2CAutodesk.Navisworks.Api.Point2D%2CAutodesk.Navisworks.Api.Point2D%2CSystem.Boolean%29)

#### `void Triangle(Autodesk.Navisworks.Api.Point3D point1, Autodesk.Navisworks.Api.Point3D point2, Autodesk.Navisworks.Api.Point3D point3, bool filled)`

Renders a 3D triangle.

**Parameters:**
- `point1` (Autodesk.Navisworks.Api.Point3D) — First point on triangle.
- `point2` (Autodesk.Navisworks.Api.Point3D) — Second point on triangle.
- `point3` (Autodesk.Navisworks.Api.Point3D) — Third point on triangle.
- `filled` (bool) — Should the triangle be filled in or an outline.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.Triangle%28Autodesk.Navisworks.Api.Point3D%2CAutodesk.Navisworks.Api.Point3D%2CAutodesk.Navisworks.Api.Point3D%2CSystem.Boolean%29)

#### `void Vertex(Autodesk.Navisworks.Api.Point3D coordinate)`

Specifies polygon vertex.

**Parameters:**
- `coordinate` (Autodesk.Navisworks.Api.Point3D) — Vertex co-ordinate.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.Vertex%28Autodesk.Navisworks.Api.Point3D%29)

#### `void VertexColor(Autodesk.Navisworks.Api.Color color, double alpha)`

Specifies color of next polygon vertex.

**Parameters:**
- `color` (Autodesk.Navisworks.Api.Color) — Vertex color.
- `alpha` (double) — Vertex alpha value.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.VertexColor%28Autodesk.Navisworks.Api.Color%2CSystem.Double%29)

#### `void VertexNormal(Autodesk.Navisworks.Api.Vector3D normal)`

Specifies normal of next polygon vertex.

**Parameters:**
- `normal` (Autodesk.Navisworks.Api.Vector3D) — Vertex normal.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Graphics.VertexNormal%28Autodesk.Navisworks.Api.Vector3D%29)

### Properties
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `PolygonTessellationWindingRule` (Autodesk.Navisworks.Api.TessellationWindingRule, get/set) — Tessellation winding rule
- `WindowHeight` (int, get) — Returns height of window.
- `WindowWidth` (int, get) — Returns width of window.

## GridIntersection (class)

Represents an Intersection of two Grid Lines

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.GridIntersection)

### Methods
#### `string FormatCombinedDisplayString(Autodesk.Navisworks.Api.Point3D point, double conversionFactor)`

FormatCombinedDisplayString method of GridIntersection.

**Parameters:**
- `point` (Autodesk.Navisworks.Api.Point3D)
- `conversionFactor` (double)

**Returns:** `string` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridIntersection.FormatCombinedDisplayString%28Autodesk.Navisworks.Api.Point3D%2CSystem.Double%29)

#### `string FormatIntersectionDisplayString(Autodesk.Navisworks.Api.Point3D point, double conversionFactor)`

FormatIntersectionDisplayString method of GridIntersection.

**Parameters:**
- `point` (Autodesk.Navisworks.Api.Point3D)
- `conversionFactor` (double)

**Returns:** `string` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridIntersection.FormatIntersectionDisplayString%28Autodesk.Navisworks.Api.Point3D%2CSystem.Double%29)

#### `string FormatLevelDisplayString(Autodesk.Navisworks.Api.Point3D point, double conversionFactor)`

FormatLevelDisplayString method of GridIntersection.

**Parameters:**
- `point` (Autodesk.Navisworks.Api.Point3D)
- `conversionFactor` (double)

**Returns:** `string` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridIntersection.FormatLevelDisplayString%28Autodesk.Navisworks.Api.Point3D%2CSystem.Double%29)

#### `static Autodesk.Navisworks.Api.GridIntersection InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of GridIntersection.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.GridIntersection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridIntersection.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of GridIntersection.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridIntersection.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

### Properties
- `DisplayName` (string, get) — The display name comprising the two grid line labels and the level label.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Level` (Autodesk.Navisworks.Api.GridLevel, get) — The Grid Level this Intersection is on.
- `Line1` (Autodesk.Navisworks.Api.GridLine, get) — The 1st Grid Line forming this Intersection.
- `Line1Direction` (Autodesk.Navisworks.Api.UnitVector3D, get) — The 3D direction of 1st grid line at the point of the intersection.
- `Line2` (Autodesk.Navisworks.Api.GridLine, get) — The 2nd Grid Line forming this Intersection.
- `Line2Direction` (Autodesk.Navisworks.Api.UnitVector3D, get) — The 3D direction of 2nd grid line at the point of the intersection.
- `Position` (Autodesk.Navisworks.Api.Point3D, get) — The 3D position of the intersection.

## GridLevel (class)

A grid level

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.GridLevel)

### Methods
#### `Autodesk.Navisworks.Api.GridIntersection ClosestIntersection(Autodesk.Navisworks.Api.Point3D point)`

Returns the intersection of the grid level closest to a given 3D point.

**Parameters:**
- `point` (Autodesk.Navisworks.Api.Point3D)

**Returns:** `Autodesk.Navisworks.Api.GridIntersection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLevel.ClosestIntersection%28Autodesk.Navisworks.Api.Point3D%29)

#### `static Autodesk.Navisworks.Api.GridLevel InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of GridLevel.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.GridLevel` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLevel.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of GridLevel.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLevel.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

### Properties
- `DisplayName` (string, get) — Name of the grid level as diplayed in the GUI.
- `Elevation` (double, get) — Evelation of the grid level.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.

## GridLevelCollection (class)

A collection of Grid Levels

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.GridLevelCollection)

### Methods
#### `void Add(Autodesk.Navisworks.Api.GridLevel item)`

Adds item to the end of the collection.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.GridLevel)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLevelCollection.Add%28Autodesk.Navisworks.Api.GridLevel%29)

#### `void AddRange(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.GridLevel> from)`

Add the items of the specified collection to the SavedItemCollection

**Parameters:**
- `from` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.GridLevel>) — The collection to add items from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLevelCollection.AddRange%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.GridLevel%7D%29)

#### `void Clear()`

Removes all items from the collection

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLevelCollection.Clear)

#### `bool Contains(Autodesk.Navisworks.Api.GridLevel item)`

Determines whether the SavedItemCollection contains a specific value.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.GridLevel)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLevelCollection.Contains%28Autodesk.Navisworks.Api.GridLevel%29)

#### `void CopyFrom(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.GridLevel> from)`

Sets the contents of the collection to the items in from

**Parameters:**
- `from` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.GridLevel>) — The collection to copy contents from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLevelCollection.CopyFrom%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.GridLevel%7D%29)

#### `void CopyTo(Autodesk.Navisworks.Api.GridLevel[] array, int arrayIndex)`

Copies the elements of the ICollection`1 to an Array, starting at a particular Array index.

**Parameters:**
- `array` (Autodesk.Navisworks.Api.GridLevel[]) — The one-dimensional Array that is the destination of the elements copied from ICollection`1. The Array must have zero-based indexing.
- `arrayIndex` (int) — The zero-based index in array at which copying begins.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLevelCollection.CopyTo%28Autodesk.Navisworks.Api.GridLevel%5B%5D%2CSystem.Int32%29)

#### `void CopyTo(System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.GridLevel> to)`

Enumerates over the collection and copies the items into to

**Parameters:**
- `to` (System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.GridLevel>) — Collection to copy into

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLevelCollection.CopyTo%28System.Collections.Generic.ICollection%7BAutodesk.Navisworks.Api.GridLevel%7D%29)

#### `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.GridLevel> GetEnumerator()`

Returns an enumerator that can iterate through the collection.

**Returns:** `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.GridLevel>` — An IEnumerator that can be used to iterate through the collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLevelCollection.GetEnumerator)

#### `int IndexOf(Autodesk.Navisworks.Api.GridLevel item)`

Determines the index of a specific item in the Collection

**Parameters:**
- `item` (Autodesk.Navisworks.Api.GridLevel)

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLevelCollection.IndexOf%28Autodesk.Navisworks.Api.GridLevel%29)

#### `void Insert(int index, Autodesk.Navisworks.Api.GridLevel item)`

Inserts item into the collection at the specified index

**Parameters:**
- `index` (int)
- `item` (Autodesk.Navisworks.Api.GridLevel)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLevelCollection.Insert%28System.Int32%2CAutodesk.Navisworks.Api.GridLevel%29)

#### `static Autodesk.Navisworks.Api.GridLevelCollection InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of GridLevelCollection.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.GridLevelCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLevelCollection.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of GridLevelCollection.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLevelCollection.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `bool Remove(Autodesk.Navisworks.Api.GridLevel item)`

Removes the first occurrence of a specific item from the collection.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.GridLevel) — The object to remove from the collection.

**Returns:** `bool` — true if item was successfully removed from the collection; otherwise, false.
This method also returns false if item is not found in the original collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLevelCollection.Remove%28Autodesk.Navisworks.Api.GridLevel%29)

#### `void RemoveAt(int index)`

Removes the IList`1 item at the specified index.

**Parameters:**
- `index` (int) — The zero-based index of the item to remove.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLevelCollection.RemoveAt%28System.Int32%29)

### Properties
- `Count` (int, get) — Gets the number of elements contained in the collection.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Item` (Autodesk.Navisworks.Api.GridLevel, get/set) — Item property of GridLevelCollection.

## GridLine (class)

A Grid Line

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.GridLine)

### Methods
#### `static Autodesk.Navisworks.Api.GridLine InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of GridLine.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.GridLine` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLine.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of GridLine.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLine.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

### Properties
- `DisplayName` (string, get) — Name of GridLine as diplayed in the GUI.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.

## GridLineCollection (class)

A collection of GridLines

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.GridLineCollection)

### Methods
#### `void Add(Autodesk.Navisworks.Api.GridLine item)`

Adds item to the end of the collection.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.GridLine)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLineCollection.Add%28Autodesk.Navisworks.Api.GridLine%29)

#### `void AddRange(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.GridLine> from)`

Add the items of the specified collection to the SavedItemCollection

**Parameters:**
- `from` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.GridLine>) — The collection to add items from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLineCollection.AddRange%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.GridLine%7D%29)

#### `void Clear()`

Removes all items from the collection

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLineCollection.Clear)

#### `bool Contains(Autodesk.Navisworks.Api.GridLine item)`

Determines whether the SavedItemCollection contains a specific value.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.GridLine)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLineCollection.Contains%28Autodesk.Navisworks.Api.GridLine%29)

#### `void CopyFrom(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.GridLine> from)`

Sets the contents of the collection to the items in from

**Parameters:**
- `from` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.GridLine>) — The collection to copy contents from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLineCollection.CopyFrom%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.GridLine%7D%29)

#### `void CopyTo(Autodesk.Navisworks.Api.GridLine[] array, int arrayIndex)`

Copies the elements of the ICollection`1 to an Array, starting at a particular Array index.

**Parameters:**
- `array` (Autodesk.Navisworks.Api.GridLine[]) — The one-dimensional Array that is the destination of the elements copied from ICollection`1. The Array must have zero-based indexing.
- `arrayIndex` (int) — The zero-based index in array at which copying begins.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLineCollection.CopyTo%28Autodesk.Navisworks.Api.GridLine%5B%5D%2CSystem.Int32%29)

#### `void CopyTo(System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.GridLine> to)`

Enumerates over the collection and copies the items into to

**Parameters:**
- `to` (System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.GridLine>) — Collection to copy into

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLineCollection.CopyTo%28System.Collections.Generic.ICollection%7BAutodesk.Navisworks.Api.GridLine%7D%29)

#### `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.GridLine> GetEnumerator()`

Returns an enumerator that can iterate through the collection.

**Returns:** `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.GridLine>` — An IEnumerator that can be used to iterate through the collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLineCollection.GetEnumerator)

#### `int IndexOf(Autodesk.Navisworks.Api.GridLine item)`

Determines the index of a specific item in the Collection

**Parameters:**
- `item` (Autodesk.Navisworks.Api.GridLine)

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLineCollection.IndexOf%28Autodesk.Navisworks.Api.GridLine%29)

#### `void Insert(int index, Autodesk.Navisworks.Api.GridLine item)`

Inserts item into the collection at the specified index

**Parameters:**
- `index` (int)
- `item` (Autodesk.Navisworks.Api.GridLine)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLineCollection.Insert%28System.Int32%2CAutodesk.Navisworks.Api.GridLine%29)

#### `static Autodesk.Navisworks.Api.GridLineCollection InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of GridLineCollection.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.GridLineCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLineCollection.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of GridLineCollection.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLineCollection.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `bool Remove(Autodesk.Navisworks.Api.GridLine item)`

Removes the first occurrence of a specific item from the collection.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.GridLine) — The object to remove from the collection.

**Returns:** `bool` — true if item was successfully removed from the collection; otherwise, false.
This method also returns false if item is not found in the original collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLineCollection.Remove%28Autodesk.Navisworks.Api.GridLine%29)

#### `void RemoveAt(int index)`

Removes the IList`1 item at the specified index.

**Parameters:**
- `index` (int) — The zero-based index of the item to remove.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridLineCollection.RemoveAt%28System.Int32%29)

### Properties
- `Count` (int, get) — Gets the number of elements contained in the collection.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Item` (Autodesk.Navisworks.Api.GridLine, get/set) — Item property of GridLineCollection.

## GridSystem (class)

A system of grid lines and levels

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.GridSystem)

### Methods
#### `Autodesk.Navisworks.Api.GridIntersection ClosestIntersection(Autodesk.Navisworks.Api.Point3D point)`

Returns the location closest to a given 3D point.

**Parameters:**
- `point` (Autodesk.Navisworks.Api.Point3D)

**Returns:** `Autodesk.Navisworks.Api.GridIntersection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridSystem.ClosestIntersection%28Autodesk.Navisworks.Api.Point3D%29)

#### `static Autodesk.Navisworks.Api.GridSystem InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of GridSystem.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.GridSystem` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridSystem.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of GridSystem.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridSystem.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

### Properties
- `DisplayName` (string, get) — Name of GridSystem as diplayed in the GUI.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Levels` (Autodesk.Navisworks.Api.GridLevelCollection, get) — Accesses Grid Levels contained in this System.
- `Lines` (Autodesk.Navisworks.Api.GridLineCollection, get) — Accesses Grid Lines contained in this System.
- `LockedLevel` (Autodesk.Navisworks.Api.GridLevel, get/set) — Determines if the GridSystem is locked to a particular level
- `Origin` (Autodesk.Navisworks.Api.Point3D, get) — Accesses origin point of this System.
- `UpDirection` (Autodesk.Navisworks.Api.UnitVector3D, get) — Accesses up direction of this System.

## GridSystemCollection (class)

A collection of GridSystems.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.GridSystemCollection)

### Methods
#### `void Add(Autodesk.Navisworks.Api.GridSystem item)`

Adds item to the end of the collection.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.GridSystem)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridSystemCollection.Add%28Autodesk.Navisworks.Api.GridSystem%29)

#### `void AddRange(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.GridSystem> from)`

Add the items of the specified collection to the SavedItemCollection

**Parameters:**
- `from` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.GridSystem>) — The collection to add items from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridSystemCollection.AddRange%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.GridSystem%7D%29)

#### `void Clear()`

Removes all items from the collection

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridSystemCollection.Clear)

#### `bool Contains(Autodesk.Navisworks.Api.GridSystem item)`

Determines whether the SavedItemCollection contains a specific value.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.GridSystem)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridSystemCollection.Contains%28Autodesk.Navisworks.Api.GridSystem%29)

#### `void CopyFrom(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.GridSystem> from)`

Sets the contents of the collection to the items in from

**Parameters:**
- `from` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.GridSystem>) — The collection to copy contents from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridSystemCollection.CopyFrom%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.GridSystem%7D%29)

#### `void CopyTo(Autodesk.Navisworks.Api.GridSystem[] array, int arrayIndex)`

Copies the elements of the ICollection`1 to an Array, starting at a particular Array index.

**Parameters:**
- `array` (Autodesk.Navisworks.Api.GridSystem[]) — The one-dimensional Array that is the destination of the elements copied from ICollection`1. The Array must have zero-based indexing.
- `arrayIndex` (int) — The zero-based index in array at which copying begins.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridSystemCollection.CopyTo%28Autodesk.Navisworks.Api.GridSystem%5B%5D%2CSystem.Int32%29)

#### `void CopyTo(System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.GridSystem> to)`

Enumerates over the collection and copies the items into to

**Parameters:**
- `to` (System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.GridSystem>) — Collection to copy into

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridSystemCollection.CopyTo%28System.Collections.Generic.ICollection%7BAutodesk.Navisworks.Api.GridSystem%7D%29)

#### `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.GridSystem> GetEnumerator()`

Returns an enumerator that can iterate through the collection.

**Returns:** `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.GridSystem>` — An IEnumerator that can be used to iterate through the collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridSystemCollection.GetEnumerator)

#### `int IndexOf(Autodesk.Navisworks.Api.GridSystem item)`

Determines the index of a specific item in the Collection

**Parameters:**
- `item` (Autodesk.Navisworks.Api.GridSystem)

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridSystemCollection.IndexOf%28Autodesk.Navisworks.Api.GridSystem%29)

#### `void Insert(int index, Autodesk.Navisworks.Api.GridSystem item)`

Inserts item into the collection at the specified index

**Parameters:**
- `index` (int)
- `item` (Autodesk.Navisworks.Api.GridSystem)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridSystemCollection.Insert%28System.Int32%2CAutodesk.Navisworks.Api.GridSystem%29)

#### `static Autodesk.Navisworks.Api.GridSystemCollection InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of GridSystemCollection.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.GridSystemCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridSystemCollection.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of GridSystemCollection.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridSystemCollection.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `bool Remove(Autodesk.Navisworks.Api.GridSystem item)`

Removes the first occurrence of a specific item from the collection.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.GridSystem) — The object to remove from the collection.

**Returns:** `bool` — true if item was successfully removed from the collection; otherwise, false.
This method also returns false if item is not found in the original collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridSystemCollection.Remove%28Autodesk.Navisworks.Api.GridSystem%29)

#### `void RemoveAt(int index)`

Removes the IList`1 item at the specified index.

**Parameters:**
- `index` (int) — The zero-based index of the item to remove.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridSystemCollection.RemoveAt%28System.Int32%29)

### Properties
- `Count` (int, get) — Gets the number of elements contained in the collection.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Item` (Autodesk.Navisworks.Api.GridSystem, get/set) — Item property of GridSystemCollection.

## GridsData (class)

The Grids data as returned by DocumentGrids.Value.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.GridsData)

### Constructors
- `GridsData(Autodesk.Navisworks.Api.GridsData from)` — Copy constructor

### Methods
#### `Autodesk.Navisworks.Api.GridsData CreateCopy()`

Create a new copy of this item

**Returns:** `Autodesk.Navisworks.Api.GridsData` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridsData.CreateCopy)

#### `static Autodesk.Navisworks.Api.GridsData InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of GridsData.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.GridsData` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridsData.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of GridsData.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GridsData.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

### Properties
- `ActiveSystem` (Autodesk.Navisworks.Api.GridSystem, get/set) — The Active Grid System
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `RenderMode` (Autodesk.Navisworks.Api.GridsRenderMode, get/set) — The render mode for grids and levels.
- `Systems` (System.Collections.ObjectModel.ReadOnlyCollection<Autodesk.Navisworks.Api.GridSystem>, get) — The collection of all GridSystems within the Document.

## GridsOptions (class)

Provides access to Grids Options

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.GridsOptions)

### Properties
- `AboveColor` (Autodesk.Navisworks.Api.Color, get/set) — The color of the Grid lines on the level directly above the camera.
- `BelowColor` (Autodesk.Navisworks.Api.Color, get/set) — The color of the Grid lines on the level directly below the camera.
- `Enabled` (bool, get/set) — Determines if Grids will rendered.
- `OtherColor` (Autodesk.Navisworks.Api.Color, get/set) — The color of the Grid lines on levels not directly above or below the camera.
- `XRayMode` (bool, get/set) — Renders occluded grid lines.

## GridsRenderMode (enum)

Determines which Levels are rendered.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.GridsRenderMode)

### Values
- `AboveAndBelow` = `0` — Renders Levels directly above and directly below the current Viewpoint.
- `Above` = `1` — Renders the Level directly above the current Viewpoint.
- `Below` = `2` — Renders the Level directly below the current Viewpoint.
- `All` = `3` — Renders all Levels.
- `Locked` = `4` — Renders just the level you are locked to.

## GroupItem (class)

Base class for classes which contain a collection of saved items.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.GroupItem)

### Methods
#### `Autodesk.Navisworks.Api.GroupItem CreateCopyWithoutChildren()`

Create a new copy of this item without its child-items

**Returns:** `Autodesk.Navisworks.Api.GroupItem` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GroupItem.CreateCopyWithoutChildren)

#### `static Autodesk.Navisworks.Api.GroupItem InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of GroupItem.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.GroupItem` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GroupItem.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of GroupItem.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.GroupItem.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

### Properties
- `Children` (Autodesk.Navisworks.Api.SavedItemCollection, get) — Children of the group
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.

## HelpIdResult (class)

Represents a Help Id returned from a call to a ToolPlugin or Input Plugin.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.HelpIdResult)

### Constructors
- `HelpIdResult()` — Constructor for a HelpIdResult.

### Properties
- `Handled` (bool, get/set) — The status of whether the plugin handled the call. If true then HelpId should containg the Help Id.
- `Id` (string, get/set) — Help Id being returned.

## Hyperlink (class)

Represents a hyperlink attached to a ModelItem.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Hyperlink)

### Constructors
- `Hyperlink(string name, System.Uri uri)` — Creates a new hyperlink from a Uri.
- `Hyperlink(string name, string url)` — Creates a new hyperlink.

### Methods
#### `bool Equals(object obj)`

Determines whether the specified object and the current object refer to the same underlying native object. Returns false if either object has been disposed.

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Hyperlink.Equals%28System.Object%29)

#### `int GetHashCode()`

Serves as a hash function for a particular type. GetHashCode is suitable for use in hashing algorithms and data structures like a hash table.

**Returns:** `int` — A hash code for the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Hyperlink.GetHashCode)

#### `static Autodesk.Navisworks.Api.Hyperlink InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of Hyperlink.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Hyperlink` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Hyperlink.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of Hyperlink.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Hyperlink.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `string ToString()`

Returns a String that represents the current Object.

**Returns:** `string` — A String that represents the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Hyperlink.ToString)

### Properties
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Name` (string, get) — Hyperlink name.
- `URL` (string, get) — Hyperlink URL.

## HyperlinkCollection (class)

A collection of Hyperlinks

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.HyperlinkCollection)

### Constructors
- `HyperlinkCollection()` — Create an empty collection
- `HyperlinkCollection(Autodesk.Navisworks.Api.HyperlinkCollection from)` — Create a collection that is a copy of from

### Methods
#### `void Add(Autodesk.Navisworks.Api.Hyperlink item)`

Adds item to the end of the collection.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.Hyperlink)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.HyperlinkCollection.Add%28Autodesk.Navisworks.Api.Hyperlink%29)

#### `void AddRange(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.Hyperlink> from)`

Add the items of the specified collection to the SavedItemCollection

**Parameters:**
- `from` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.Hyperlink>) — The collection to add items from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.HyperlinkCollection.AddRange%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.Hyperlink%7D%29)

#### `void Clear()`

Removes all items from the collection

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.HyperlinkCollection.Clear)

#### `bool Contains(Autodesk.Navisworks.Api.Hyperlink item)`

Determines whether the SavedItemCollection contains a specific value.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.Hyperlink)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.HyperlinkCollection.Contains%28Autodesk.Navisworks.Api.Hyperlink%29)

#### `void CopyFrom(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.Hyperlink> from)`

Sets the contents of the collection to the items in from

**Parameters:**
- `from` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.Hyperlink>) — The collection to copy contents from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.HyperlinkCollection.CopyFrom%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.Hyperlink%7D%29)

#### `void CopyTo(Autodesk.Navisworks.Api.Hyperlink[] array, int arrayIndex)`

Copies the elements of the ICollection`1 to an Array, starting at a particular Array index.

**Parameters:**
- `array` (Autodesk.Navisworks.Api.Hyperlink[]) — The one-dimensional Array that is the destination of the elements copied from ICollection`1. The Array must have zero-based indexing.
- `arrayIndex` (int) — The zero-based index in array at which copying begins.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.HyperlinkCollection.CopyTo%28Autodesk.Navisworks.Api.Hyperlink%5B%5D%2CSystem.Int32%29)

#### `void CopyTo(System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.Hyperlink> to)`

Enumerates over the collection and copies the items into to

**Parameters:**
- `to` (System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.Hyperlink>) — Collection to copy into

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.HyperlinkCollection.CopyTo%28System.Collections.Generic.ICollection%7BAutodesk.Navisworks.Api.Hyperlink%7D%29)

#### `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.Hyperlink> GetEnumerator()`

Returns an enumerator that can iterate through the collection.

**Returns:** `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.Hyperlink>` — An IEnumerator that can be used to iterate through the collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.HyperlinkCollection.GetEnumerator)

#### `int IndexOf(Autodesk.Navisworks.Api.Hyperlink item)`

Determines the index of a specific item in the Collection

**Parameters:**
- `item` (Autodesk.Navisworks.Api.Hyperlink)

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.HyperlinkCollection.IndexOf%28Autodesk.Navisworks.Api.Hyperlink%29)

#### `void Insert(int index, Autodesk.Navisworks.Api.Hyperlink item)`

Inserts item into the collection at the specified index

**Parameters:**
- `index` (int)
- `item` (Autodesk.Navisworks.Api.Hyperlink)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.HyperlinkCollection.Insert%28System.Int32%2CAutodesk.Navisworks.Api.Hyperlink%29)

#### `static Autodesk.Navisworks.Api.HyperlinkCollection InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of HyperlinkCollection.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.HyperlinkCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.HyperlinkCollection.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of HyperlinkCollection.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.HyperlinkCollection.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `bool Remove(Autodesk.Navisworks.Api.Hyperlink item)`

Removes the first occurrence of a specific item from the collection.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.Hyperlink) — The object to remove from the collection.

**Returns:** `bool` — true if item was successfully removed from the collection; otherwise, false.
This method also returns false if item is not found in the original collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.HyperlinkCollection.Remove%28Autodesk.Navisworks.Api.Hyperlink%29)

#### `void RemoveAt(int index)`

Removes the IList`1 item at the specified index.

**Parameters:**
- `index` (int) — The zero-based index of the item to remove.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.HyperlinkCollection.RemoveAt%28System.Int32%29)

### Properties
- `Count` (int, get) — Gets the number of elements contained in the collection.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Item` (Autodesk.Navisworks.Api.Hyperlink, get/set) — Item property of HyperlinkCollection.

## IHasDynamicProperties (interface)

Extends the NET interface IDynamicMetaObjectProvider to include provision of dynamic properties.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.IHasDynamicProperties)

### Methods
#### `object GetProperty(string name)`

Fetches the property value specified by the string argument.

**Parameters:**
- `name` (string) — Name of the dynamic property to obtain. As with real NET properties, this name is case-sensitive.

**Returns:** `object` — Value of the dynamic property retrieved.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.IHasDynamicProperties.GetProperty%28System.String%29)

#### `object SetProperty(string name, object value)`

Sets a value to the property specified by the string argument.

**Parameters:**
- `name` (string) — Name of the dynamic property to set. As with real NET properties, this name is case-sensitive.
- `value` (object) — Value to set. This should match the type of the dynamic property.

**Returns:** `object` — The same object as was passed to argument 'value'. This enables settor chaining.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.IHasDynamicProperties.SetProperty%28System.String%2CSystem.Object%29)

### Properties
- `DynamicMemberNames` (System.Collections.Generic.IEnumerable<string>, get) — Enumerates the dynamically added members of the object by name.

## ImageGenerationStyle (enum)

Defines the style of the image generation

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ImageGenerationStyle)

### Values
- `Scene` = `0` — Render scene
- `SceneUsingRayTrace` = `1` — Render scene using ray trace
- `ScenePlusOverlay` = `2` — Render scene plus any overlay such as Redlines

## InfoPropertyCategory (class)

Category of properties stored in DocumentInfo and SheetInfo objects.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.InfoPropertyCategory)

### Methods
#### `bool Equals(object obj)`

Determines whether the specified object and the current object refer to the same underlying native object. Returns false if either object has been disposed.

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.InfoPropertyCategory.Equals%28System.Object%29)

#### `int GetHashCode()`

Serves as a hash function for a particular type. GetHashCode is suitable for use in hashing algorithms and data structures like a hash table.

**Returns:** `int` — A hash code for the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.InfoPropertyCategory.GetHashCode)

#### `static Autodesk.Navisworks.Api.InfoPropertyCategory InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of InfoPropertyCategory.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.InfoPropertyCategory` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.InfoPropertyCategory.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of InfoPropertyCategory.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.InfoPropertyCategory.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

### Properties
- `DisplayName` (string, get) — Get the display name.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Properties` (Autodesk.Navisworks.Api.DataPropertyCollection, get) — Get the properties

## InfoPropertyCategoryCollection (class)

A collection of InfoPropertyCategory objects.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.InfoPropertyCategoryCollection)

### Constructors
- `InfoPropertyCategoryCollection()` — Create an empty collection

### Methods
#### `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.InfoPropertyCategory> GetEnumerator()`

Returns an enumerator that can iterate through the collection.

**Returns:** `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.InfoPropertyCategory>` — An IEnumerator that can be used to iterate through the collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.InfoPropertyCategoryCollection.GetEnumerator)

#### `static Autodesk.Navisworks.Api.InfoPropertyCategoryCollection InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of InfoPropertyCategoryCollection.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.InfoPropertyCategoryCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.InfoPropertyCategoryCollection.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of InfoPropertyCategoryCollection.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.InfoPropertyCategoryCollection.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `System.Collections.IEnumerator InternalGetEnumerator()`

InternalGetEnumerator method of InfoPropertyCategoryCollection.

**Returns:** `System.Collections.IEnumerator` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.InfoPropertyCategoryCollection.InternalGetEnumerator)

### Properties
- `Count` (int, get) — Gets the number of elements contained in the collection.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Item` (Autodesk.Navisworks.Api.InfoPropertyCategory, get) — Item property of InfoPropertyCategoryCollection.

## KeyModifiers (enum)

Enumerates key modifiers used in input into Navisworks

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.KeyModifiers)

### Values
- `None` = `0` — No modifier keys pressed
- `Shift` = `1` — SHIFT modifier key pressed
- `Alt` = `2` — ALT modifier keys pressed
- `Ctrl` = `4` — CTRL modifier key pressed
- `DoubleClick` = `8` — Double Click

## MaterialOverride (class)

Material Override

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.MaterialOverride)

### Constructors
- `MaterialOverride()` — Constructs a new MaterialOverride.

### Properties
- `Color` (Autodesk.Navisworks.Api.Color, get/set) — Material Override. May be null.
- `Item` (Autodesk.Navisworks.Api.ModelItem, get/set) — Material Override.
- `Transparency` (double?, get/set) — Material Override. May be null.

## Matrix3 (class)

A 3x3 matrix.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Matrix3)

### Constructors
- `Matrix3()` — Default constructor. Makes an identity matrix.
- `Matrix3(Autodesk.Navisworks.Api.Rotation3D rotation)` — Make from a rotation.
- `Matrix3(double e00, double e01, double e02, double e10, double e11, double e12, double e20, double e21, double e22)` — Make from elements specified in row major order.

### Methods
#### `double Determinant()`

Get determinant of matrix.

**Returns:** `double` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Matrix3.Determinant)

#### `bool Equals(Autodesk.Navisworks.Api.Matrix3 matrix)`

Equal within a tolerance, each element of matrix is equal to corresponding element within given tolerance.

**Parameters:**
- `matrix` (Autodesk.Navisworks.Api.Matrix3)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Matrix3.Equals%28Autodesk.Navisworks.Api.Matrix3%29)

#### `bool Equals(Autodesk.Navisworks.Api.Matrix3 matrix, double tolerance)`

Equal within a tolerance, each element of matrix is equal to corresponding element within given tolerance.

**Parameters:**
- `matrix` (Autodesk.Navisworks.Api.Matrix3)
- `tolerance` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Matrix3.Equals%28Autodesk.Navisworks.Api.Matrix3%2CSystem.Double%29)

#### `bool Equals(object obj)`

Determines whether the specified object and the current object refer to the same underlying native object. Returns false if either object has been disposed.

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Matrix3.Equals%28System.Object%29)

#### `double Get(int row, int column)`

"Get a single matrix element."

**Parameters:**
- `row` (int)
- `column` (int)

**Returns:** `double` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Matrix3.Get%28System.Int32%2CSystem.Int32%29)

#### `int GetHashCode()`

Serves as a hash function for a particular type. GetHashCode is suitable for use in hashing algorithms and data structures like a hash table.

**Returns:** `int` — A hash code for the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Matrix3.GetHashCode)

#### `static Autodesk.Navisworks.Api.Matrix3 InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of Matrix3.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Matrix3` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Matrix3.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of Matrix3.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Matrix3.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `bool IsIdentity()`

Is matrix the identity matrix ?

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Matrix3.IsIdentity)

#### `bool IsZero()`

Is matrix a zero matrix? A zero matrix is one where every component is 0.0.

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Matrix3.IsZero)

#### `string ToString()`

Returns a String that represents the current Object.

**Returns:** `string` — A String that represents the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Matrix3.ToString)

### Properties
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.

## MeasurementType (enum)

The type of measurement tool being used.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.MeasurementType)

### Values
- `None` = `0` — No measurement type
- `Distance` = `1` — Distance measurement type
- `Angle` = `2` — Angle measurement type
- `Area` = `3` — Area measurement type

## MessageCenterCategory (enum)

MessageCenterCategory enum in Autodesk.Navisworks.Api.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.MessageCenterCategory)

### Values
- `Information` = `0`
- `Warning` = `1`
- `Error` = `2`
- `Developer` = `3`

## Model (class)

Represents a loaded model within the model hierarchy

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Model)

### Methods
#### `bool Equals(object obj)`

Determines whether the specified object and the current object refer to the same underlying native object. Returns false if either object has been disposed.

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Model.Equals%28System.Object%29)

#### `int GetHashCode()`

Serves as a hash function for a particular type. GetHashCode is suitable for use in hashing algorithms and data structures like a hash table.

**Returns:** `int` — A hash code for the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Model.GetHashCode)

#### `static Autodesk.Navisworks.Api.Model InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of Model.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Model` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Model.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of Model.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Model.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

### Properties
- `Creator` (string, get) — Name of creating file loader or design application. Null if unknown.
- `FileName` (string, get) — Filename this model was loaded from
- `FrontVector` (Autodesk.Navisworks.Api.UnitVector3D, get) — Preferred front vector for model. Will be zero if no front vector defined.
- `Guid` (System.Guid, get) — Globally unique identifier for this model
- `HasFrontVector` (bool, get) — Is there a front vector defined for model ?
- `HasNorthVector` (bool, get) — Is there a north vector defined for model ?
- `HasRightVector` (bool, get) — Is there a right vector defined for model ?
- `HasUpVector` (bool, get) — Is there an up vector defined for model ?
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `IsTransformReflected` (bool, get) — Is the transform reflected.
- `NorthVector` (Autodesk.Navisworks.Api.UnitVector3D, get) — Preferred north vector for model. Will be zero if no north vector defined.
- `PublishProperties` (Autodesk.Navisworks.Api.PublishProperties, get) — Properties specified when model was last published. Will be null if model has not been published.
- `RightVector` (Autodesk.Navisworks.Api.UnitVector3D, get) — Preferred right vector for model. Will be zero if no right vector defined.
- `RootItem` (Autodesk.Navisworks.Api.ModelItem, get) — Root item of the model hierarchy corresponding to this model
- `SchemaDefinitions` (Autodesk.Navisworks.Api.Schema.SchemaDefinitionCollection, get) — Schema definitions in use by this model.
- `SourceFileName` (string, get) — Original source filename this model was first converted from. Null if unknown.
- `SourceGuid` (System.Guid, get) — Get original Guid that this model was created with. Guid may be modified to ensure all models within a sheet have unique Guids. If the same model is appended to a sheet twice, each model has the same SourceGuid but different Guids.
- `Transform` (Autodesk.Navisworks.Api.Transform3D, get) — Transform applied to the model.
- `Units` (Autodesk.Navisworks.Api.Units, get) — Units in which dimensions of this model are defined
- `UpVector` (Autodesk.Navisworks.Api.UnitVector3D, get) — Preferred up vector for model. Will be zero if no up vector defined.

## ModelCreatorNames (class)

Names (values of Creator) of commonly found model creators.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ModelCreatorNames)

### Properties
- `ArchicadExporter` (string, get) — Models created by export from Archicad
- `AsciiLaserReader` (string, get) — Models created by ASCII Laser reader
- `AutodeskRealDwgReader` (string, get) — Models created by Autodesk RealDWG reader
- `Cis2Reader` (string, get) — Models created by CIS/2 reader
- `DwfReader` (string, get) — Models created by DWF reader
- `FaroReader` (string, get) — Models created by Faro reader
- `IfcReader` (string, get) — Models created by IFC reader
- `IgesReader` (string, get) — Models created by IGES reader
- `InventorReader` (string, get) — Models created by Inventor reader
- `JTOpenReader` (string, get) — Models created by JTOpen reader
- `LeicaReader` (string, get) — Models created by Leica reader
- `MaxExporter` (string, get) — Models created by export from 3D Studio Max
- `MicrostationExporter` (string, get) — Models created by export from Microstation
- `MicrostationReader` (string, get) — Models created by Microstation design file (DGN) reader
- `ParasolidReader` (string, get) — Models created by Parasolid reader
- `RevitExporter` (string, get) — Models created by export from Revit
- `RieglReader` (string, get) — Models created by Riegl reader
- `RvmReader` (string, get) — Models created by RVM reader
- `SatReader` (string, get) — Models created by SAT reader
- `SketchUpReader` (string, get) — Models created by SketchUp reader
- `StepReader` (string, get) — Models created by STEP reader
- `StlReader` (string, get) — Models created by STL reader
- `The3DStudioReader` (string, get) — Models created by 3D Studio reader
- `VrmlReader` (string, get) — Models created by VRML reader
- `ZAndFReader` (string, get) — Models created by Z+F reader

## ModelGeometry (class)

Represents geometry within the model hierarchy

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ModelGeometry)

### Methods
#### `bool Equals(object obj)`

Determines whether the specified object and the current object refer to the same underlying native object. Returns false if either object has been disposed.

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelGeometry.Equals%28System.Object%29)

#### `int GetHashCode()`

Serves as a hash function for a particular type. GetHashCode is suitable for use in hashing algorithms and data structures like a hash table.

**Returns:** `int` — A hash code for the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelGeometry.GetHashCode)

#### `static Autodesk.Navisworks.Api.ModelGeometry InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of ModelGeometry.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.ModelGeometry` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelGeometry.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.ModelGeometry InternalCustomCreator(nint handleIn, Autodesk.Navisworks.Api.NativeHandle source)`

InternalCustomCreator method of ModelGeometry.

**Parameters:**
- `handleIn` (nint)
- `source` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.ModelGeometry` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelGeometry.InternalCustomCreator%28System.IntPtr%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of ModelGeometry.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelGeometry.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

### Properties
- `ActiveColor` (Autodesk.Navisworks.Api.Color, get) — Current (visible) color for this geometry
- `ActiveTransform` (Autodesk.Navisworks.Api.Transform3D, get) — Returns the currently active transform of the geometry.
- `ActiveTransparency` (double, get) — Current (visible) transparency for this geometry
- `BoundingBox` (Autodesk.Navisworks.Api.BoundingBox3D, get) — Bounding box of this geometry in world space
- `FragmentCount` (int, get) — Number of fragments this geometry is divided into
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `IsSolid` (bool, get) — Is this geometry solid ? (Includes triangle primitives which all form closed, manifold shells)
- `Item` (Autodesk.Navisworks.Api.ModelItem, get) — Item in the model hierarchy corresponding to this geometry
- `OriginalColor` (Autodesk.Navisworks.Api.Color, get) — Original color for this geometry (as specified by design file)
- `OriginalTransform` (Autodesk.Navisworks.Api.Transform3D, get) — Returns the original transform of the geometry when it was loaded.
- `OriginalTransparency` (double, get) — Original transparency for this geometry (as specified by design file)
- `PermanentColor` (Autodesk.Navisworks.Api.Color, get) — Permanent color for geometry. Either original color or color explicitly overriden by user.
- `PermanentOverrideTransform` (Autodesk.Navisworks.Api.Transform3D, get) — Transform applied to the original transform of the model geometry.
- `PermanentTransform` (Autodesk.Navisworks.Api.Transform3D, get) — Permanent transform for the model geometry. The transform formed as a result of combining the original transform with the override transform.
- `PermanentTransparency` (double, get) — Permanent transparency for geometry. Either original transparency or transparency explicitly overriden by user.
- `PrimitiveCount` (int, get) — Number of primitives (triangles,lines,points) that define this geometry
- `PrimitiveTypes` (Autodesk.Navisworks.Api.PrimitiveTypes, get) — Types of primitives used to define this geometry

## ModelItem (class)

Represents an instance within the model hierarchy, corresponding to an item in the Navisworks selection tree

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ModelItem)

### Methods
#### `Autodesk.Navisworks.Api.BoundingBox3D BoundingBox()`

Calculates bounding box of item and its children

**Returns:** `Autodesk.Navisworks.Api.BoundingBox3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItem.BoundingBox)

#### `Autodesk.Navisworks.Api.BoundingBox3D BoundingBox(bool ignoreHidden)`

Calculates bounding box of item and its children

**Parameters:**
- `ignoreHidden` (bool)

**Returns:** `Autodesk.Navisworks.Api.BoundingBox3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItem.BoundingBox%28System.Boolean%29)

#### `bool Equals(object obj)`

Determines whether the specified object and the current object refer to the same underlying native object. Returns false if either object has been disposed.

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItem.Equals%28System.Object%29)

#### `Autodesk.Navisworks.Api.ModelGeometry FindFirstGeometry()`

Depth first search for geometry below this item in hierarchy

**Returns:** `Autodesk.Navisworks.Api.ModelGeometry` — First geometry or null if none found

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItem.FindFirstGeometry)

#### `Autodesk.Navisworks.Api.ModelItem FindFirstObjectAncestor()`

Find ancestor that meets the requirements for First Object selection Behavior

**Returns:** `Autodesk.Navisworks.Api.ModelItem` — Return ancestor that meets the requirements for First Object selection Behavior or nulll if no ancestor does

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItem.FindFirstObjectAncestor)

#### `int GetHashCode()`

Serves as a hash function for a particular type. GetHashCode is suitable for use in hashing algorithms and data structures like a hash table.

**Returns:** `int` — A hash code for the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItem.GetHashCode)

#### `Autodesk.Navisworks.Api.PropertyCategoryCollection GetUserFilteredPropertyCategories()`

Properties of this item filtered so that only the categories that the end user would see in the GUI are shown (based on global options settings)

**Returns:** `Autodesk.Navisworks.Api.PropertyCategoryCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItem.GetUserFilteredPropertyCategories)

#### `static Autodesk.Navisworks.Api.ModelItem InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of ModelItem.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.ModelItem` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItem.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.ModelItem InternalCustomCreator(nint handleIn, Autodesk.Navisworks.Api.NativeHandle source)`

InternalCustomCreator method of ModelItem.

**Parameters:**
- `handleIn` (nint)
- `source` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.ModelItem` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItem.InternalCustomCreator%28System.IntPtr%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of ModelItem.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItem.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `bool IsSameInstance(Autodesk.Navisworks.Api.ModelItem item)`

Does this item refer to the same instance data as another item ?

**Parameters:**
- `item` (Autodesk.Navisworks.Api.ModelItem) — Other item to compare instance data with

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItem.IsSameInstance%28Autodesk.Navisworks.Api.ModelItem%29)

### Properties
- `Ancestors` (Autodesk.Navisworks.Api.ModelItemEnumerableCollection, get) — All ancestors of this item (excluding item itself) within the model hierarchy
- `AncestorsAndSelf` (Autodesk.Navisworks.Api.ModelItemEnumerableCollection, get) — All ancestors of this item (including item itself) within the model hierarchy
- `Children` (Autodesk.Navisworks.Api.ModelItemEnumerableCollection, get) — Children of this item within the model hierarchy
- `ClassDisplayName` (string, get) — Display name for type of item, derived from types in original design application.
- `ClassName` (string, get) — Name for type of item. Derived from types in original design application.
- `Descendants` (Autodesk.Navisworks.Api.ModelItemEnumerableCollection, get) — All descendants of this item (excluding item itself) within the model hierarchy
- `DescendantsAndSelf` (Autodesk.Navisworks.Api.ModelItemEnumerableCollection, get) — All descendants of this item (including item itself) within the model hierarchy
- `DisplayName` (string, get) — End user supplied name for item. May be null.
- `Geometry` (Autodesk.Navisworks.Api.ModelGeometry, get) — Geometry for this item, null if it does not have geometry
- `HasGeometry` (bool, get) — Does this item have geometry ?
- `HasModel` (bool, get) — Does this item refer to a model ?
- `InstanceGuid` (System.Guid, get) — Guid for the instance data refered to by this item
- `InstanceHashCode` (int, get) — HashCode for the instance data refered to by this item
- `Instances` (Autodesk.Navisworks.Api.ModelItemEnumerableCollection, get) — All items in the model hierarchy (including this item) that share instance data with this item
- `IsCollection` (bool, get) — Does this item represent a collection ?
- `IsComposite` (bool, get) — Does this item represent a composite object ?
- `IsFrozen` (bool, get) — Is this item frozen (frozen items cannot be picked graphically).
- `IsHidden` (bool, get) — Is this item hidden (won't be displayed in viewing window).
- `IsInsert` (bool, get) — Does this item represent an insert ?
- `IsLayer` (bool, get) — Does this item represent a layer ?
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `IsRequired` (bool, get) — Is this item required (will always display in viewing window, won't drop out).
- `Model` (Autodesk.Navisworks.Api.Model, get) — Model corresponding to item, null if it does not have a model
- `Parent` (Autodesk.Navisworks.Api.ModelItem, get) — Parent item within the model hierarchy, null if this is the root of the hierarchy
- `PropertyCategories` (Autodesk.Navisworks.Api.PropertyCategoryCollection, get) — Properties of this item divided into multiple categories
- `Self` (Autodesk.Navisworks.Api.ModelItemEnumerableCollection, get) — This item as an enumerable collection of one item
- `Transform` (Autodesk.Navisworks.Api.Transform3D, get) — Returns the Transform attached to this item in the original source design file

## ModelItemCollection (class)

A collection of ModelItems. Often used to represent an explicit selection

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ModelItemCollection)

### Constructors
- `ModelItemCollection()` — Create an empty collection
- `ModelItemCollection(Autodesk.Navisworks.Api.ModelItemCollection from)` — Create a copy of another collection

### Methods
#### `void Add(Autodesk.Navisworks.Api.ModelItem item)`

Adds an item to the ICollection`1.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.ModelItem) — The object to add to the ICollection`1.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemCollection.Add%28Autodesk.Navisworks.Api.ModelItem%29)

#### `void AddAllInstances()`

Add all instances of items included in the selection

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemCollection.AddAllInstances)

#### `void AddRange(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem> from)`

Add the items of the specified collection to the ModelItemCollection

**Parameters:**
- `from` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem>) — The collection to add items from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemCollection.AddRange%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.ModelItem%7D%29)

#### `bool AreAllInstancesSelected()`

Are all instances of items included in the selection also in the selection?

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemCollection.AreAllInstancesSelected)

#### `Autodesk.Navisworks.Api.BoundingBox3D BoundingBox()`

Calculates bounding box of all items contained in the selection

**Returns:** `Autodesk.Navisworks.Api.BoundingBox3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemCollection.BoundingBox)

#### `Autodesk.Navisworks.Api.BoundingBox3D BoundingBox(bool ignoreHidden)`

Calculates bounding box of all items contained in the selection

**Parameters:**
- `ignoreHidden` (bool)

**Returns:** `Autodesk.Navisworks.Api.BoundingBox3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemCollection.BoundingBox%28System.Boolean%29)

#### `void Clear()`

Removes all items from the ICollection`1.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemCollection.Clear)

#### `bool Contains(Autodesk.Navisworks.Api.ModelItem item)`

Determines whether the Selection contains a specific value.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.ModelItem) — The ModelItem to locate in the Selection.

**Returns:** `bool` — true if item is found in the Selection; otherwise, false.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemCollection.Contains%28Autodesk.Navisworks.Api.ModelItem%29)

#### `void CopyFrom(Autodesk.Navisworks.Api.ModelItemCollection from)`

Sets the contents of the collection to the items in from

**Parameters:**
- `from` (Autodesk.Navisworks.Api.ModelItemCollection)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemCollection.CopyFrom%28Autodesk.Navisworks.Api.ModelItemCollection%29)

#### `void CopyFrom(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem> from)`

Sets the contents of the collection to the items in from

**Parameters:**
- `from` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem>) — The collection to copy contents from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemCollection.CopyFrom%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.ModelItem%7D%29)

#### `void CopyTo(Autodesk.Navisworks.Api.ModelItem[] array, int arrayIndex)`

Copies the elements of the ICollection`1 to an Array, starting at a particular Array index.

**Parameters:**
- `array` (Autodesk.Navisworks.Api.ModelItem[]) — The one-dimensional Array that is the destination of the elements copied from ICollection`1. The Array must have zero-based indexing.
- `arrayIndex` (int) — The zero-based index in array at which copying begins.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemCollection.CopyTo%28Autodesk.Navisworks.Api.ModelItem%5B%5D%2CSystem.Int32%29)

#### `void CopyTo(System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.ModelItem> to)`

Copies the contents of the collection into to

**Parameters:**
- `to` (System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.ModelItem>) — The collection to copy contents into

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemCollection.CopyTo%28System.Collections.Generic.ICollection%7BAutodesk.Navisworks.Api.ModelItem%7D%29)

#### `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.ModelItem> GetEnumerator()`

Returns an enumerator that can iterate through the collection.

**Returns:** `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.ModelItem>` — An IEnumerator that can be used to iterate through the collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemCollection.GetEnumerator)

#### `static Autodesk.Navisworks.Api.ModelItemCollection InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of ModelItemCollection.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.ModelItemCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemCollection.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of ModelItemCollection.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemCollection.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `System.Collections.IEnumerator InternalGetEnumerator()`

InternalGetEnumerator method of ModelItemCollection.

**Returns:** `System.Collections.IEnumerator` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemCollection.InternalGetEnumerator)

#### `Autodesk.Navisworks.Api.Interop.LcOaPathLinkVector InternalGetPaths()`

InternalGetPaths method of ModelItemCollection.

**Returns:** `Autodesk.Navisworks.Api.Interop.LcOaPathLinkVector` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemCollection.InternalGetPaths)

#### `void Invert(Autodesk.Navisworks.Api.Document doc)`

Inverts the selection.

**Parameters:**
- `doc` (Autodesk.Navisworks.Api.Document) — The document to invert the selection relative to.
Should be the same document that you got the original collection from.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemCollection.Invert%28Autodesk.Navisworks.Api.Document%29)

#### `bool IsAnyContained(Autodesk.Navisworks.Api.ModelItemCollection value)`

Is any item in sel contained in selection? Are any items in value contained in any of the items in this selection?

**Parameters:**
- `value` (Autodesk.Navisworks.Api.ModelItemCollection)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemCollection.IsAnyContained%28Autodesk.Navisworks.Api.ModelItemCollection%29)

#### `bool IsContained(Autodesk.Navisworks.Api.ModelItem item)`

Is item contained in selection? Item must be contained in any of the items in this selection

**Parameters:**
- `item` (Autodesk.Navisworks.Api.ModelItem)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemCollection.IsContained%28Autodesk.Navisworks.Api.ModelItem%29)

#### `bool IsContained(Autodesk.Navisworks.Api.ModelItemCollection value)`

Is sel contained in selection? Are all items in value contained in any of the items in this selection?

**Parameters:**
- `value` (Autodesk.Navisworks.Api.ModelItemCollection)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemCollection.IsContained%28Autodesk.Navisworks.Api.ModelItemCollection%29)

#### `bool IsContentsIntersected(Autodesk.Navisworks.Api.ModelItemCollection value)`

Are any paths contained by this selection also contained by value?

**Parameters:**
- `value` (Autodesk.Navisworks.Api.ModelItemCollection)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemCollection.IsContentsIntersected%28Autodesk.Navisworks.Api.ModelItemCollection%29)

#### `bool IsDisjoint()`

Is no item contained by any other item?

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemCollection.IsDisjoint)

#### `bool IsSelected(Autodesk.Navisworks.Api.ModelItem item)`

Is item a member of this selection?

**Parameters:**
- `item` (Autodesk.Navisworks.Api.ModelItem)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemCollection.IsSelected%28Autodesk.Navisworks.Api.ModelItem%29)

#### `bool IsSelected(Autodesk.Navisworks.Api.ModelItemCollection value)`

Is value a subset of this selection? All paths in value must also be in this selection

**Parameters:**
- `value` (Autodesk.Navisworks.Api.ModelItemCollection)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemCollection.IsSelected%28Autodesk.Navisworks.Api.ModelItemCollection%29)

#### `void MakeDisjoint()`

Remove any selected items that are contained by other items

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemCollection.MakeDisjoint)

#### `void Minimize()`

Replaces this selection by one which contains the same items as this one does while including the minimum number of items.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemCollection.Minimize)

#### `bool Remove(Autodesk.Navisworks.Api.ModelItem item)`

Removes the first occurrence of a specific ModelItem from the Selection.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.ModelItem) — The object to remove from the Selection.

**Returns:** `bool` — true if item was successfully removed from the Selection; otherwise, false. This method also returns false if item is not found in the original Selection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemCollection.Remove%28Autodesk.Navisworks.Api.ModelItem%29)

#### `bool ValueEquals(Autodesk.Navisworks.Api.ModelItemCollection value)`

Performs comparison 'By Value'

**Parameters:**
- `value` (Autodesk.Navisworks.Api.ModelItemCollection)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemCollection.ValueEquals%28Autodesk.Navisworks.Api.ModelItemCollection%29)

#### `static bool ValueEquals(Autodesk.Navisworks.Api.ModelItemCollection value1, Autodesk.Navisworks.Api.ModelItemCollection value2)`

Compares the two references 'By Value'

**Parameters:**
- `value1` (Autodesk.Navisworks.Api.ModelItemCollection)
- `value2` (Autodesk.Navisworks.Api.ModelItemCollection)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemCollection.ValueEquals%28Autodesk.Navisworks.Api.ModelItemCollection%2CAutodesk.Navisworks.Api.ModelItemCollection%29)

#### `System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem> Where(Autodesk.Navisworks.Api.SearchCondition condition)`

Filters the items based on a search condition

**Parameters:**
- `condition` (Autodesk.Navisworks.Api.SearchCondition)

**Returns:** `System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem>` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemCollection.Where%28Autodesk.Navisworks.Api.SearchCondition%29)

#### `System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem> Where(Autodesk.Navisworks.Api.SearchConditionCollection conditions)`

Filters the items based on a set of search conditions

**Parameters:**
- `conditions` (Autodesk.Navisworks.Api.SearchConditionCollection)

**Returns:** `System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem>` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemCollection.Where%28Autodesk.Navisworks.Api.SearchConditionCollection%29)

### Properties
- `Count` (int, get) — Gets the number of elements contained in the collection.
- `Descendants` (Autodesk.Navisworks.Api.ModelItemEnumerableCollection, get) — All descendants of the items in the collection (excluding the items themselves)
- `DescendantsAndSelf` (Autodesk.Navisworks.Api.ModelItemEnumerableCollection, get) — All descendants of the items in the collection (including the items themselves)
- `First` (Autodesk.Navisworks.Api.ModelItem, get) — Return the first item in the collection or null if empty
- `InternalIsReadOnly` (bool, get) — InternalIsReadOnly property of ModelItemCollection.
- `IsEmpty` (bool, get) — Does the collection represent nothing selected?
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Item` (Autodesk.Navisworks.Api.ModelItem, get) — Item property of ModelItemCollection.

## ModelItemEnumerableCollection (class)

Collection of ModelItems that only supports Enumerable style interfaces

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ModelItemEnumerableCollection)

### Methods
#### `void CopyTo(System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.ModelItem> to)`

Enumerates over the collection and copies the items into to

**Parameters:**
- `to` (System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.ModelItem>) — Collection to copy into

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemEnumerableCollection.CopyTo%28System.Collections.Generic.ICollection%7BAutodesk.Navisworks.Api.ModelItem%7D%29)

#### `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.ModelItem> GetEnumerator()`

Returns an enumerator that can iterate through the collection.

**Returns:** `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.ModelItem>` — An IEnumerator that can be used to iterate through the collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemEnumerableCollection.GetEnumerator)

#### `System.Collections.IEnumerator InternalGetEnumerator()`

InternalGetEnumerator method of ModelItemEnumerableCollection.

**Returns:** `System.Collections.IEnumerator` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemEnumerableCollection.InternalGetEnumerator)

#### `string ToString()`

Returns a String that represents the current Object.

**Returns:** `string` — A String that represents the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemEnumerableCollection.ToString)

#### `System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem> Where(Autodesk.Navisworks.Api.SearchCondition condition)`

Filters the items based on a search condition

**Parameters:**
- `condition` (Autodesk.Navisworks.Api.SearchCondition)

**Returns:** `System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem>` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemEnumerableCollection.Where%28Autodesk.Navisworks.Api.SearchCondition%29)

#### `System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem> Where(Autodesk.Navisworks.Api.SearchConditionCollection conditions)`

Filters the instances based on a set of search conditions

**Parameters:**
- `conditions` (Autodesk.Navisworks.Api.SearchConditionCollection)

**Returns:** `System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem>` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemEnumerableCollection.Where%28Autodesk.Navisworks.Api.SearchConditionCollection%29)

#### `System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem> WhereInstanceGuid(System.Guid value)`

Filters the instances to those with given InstanceGuid

**Parameters:**
- `value` (System.Guid)

**Returns:** `System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem>` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.ModelItemEnumerableCollection.WhereInstanceGuid%28System.Guid%29)

### Properties
- `Empty` (Autodesk.Navisworks.Api.ModelItemEnumerableCollection, get) — Returns an empty ModelItemEnumerableCollection
- `First` (Autodesk.Navisworks.Api.ModelItem, get) — Returns the first item in the enumeration, returns null if the collection is empty

## NamedConstant (class)

A named integer constant with localized display name. Immutable.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.NamedConstant)

### Constructors
- `NamedConstant(int value, string baseName)` — Construct a named integer constant
- `NamedConstant(int value, string baseName, string displayName)` — Construct a named integer constant
- `NamedConstant(string name)` — Construct a named integer constant with value 0
- `NamedConstant(string name, string displayName)` — Construct a named integer constant with value 0

### Methods
#### `bool Equals(object obj)`

Determines whether the specified object and the current object refer to the same underlying native object. Returns false if either object has been disposed.

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.NamedConstant.Equals%28System.Object%29)

#### `int GetHashCode()`

Serves as a hash function for a particular type. GetHashCode is suitable for use in hashing algorithms and data structures like a hash table.

**Returns:** `int` — A hash code for the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.NamedConstant.GetHashCode)

#### `static Autodesk.Navisworks.Api.NamedConstant InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of NamedConstant.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.NamedConstant` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.NamedConstant.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of NamedConstant.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.NamedConstant.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `string ToString()`

Returns a String that represents the current Object.

**Returns:** `string` — A String that represents the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.NamedConstant.ToString)

#### `bool UserEquals(Autodesk.Navisworks.Api.NamedConstant other)`

Is this constant the same as another constant.

**Parameters:**
- `other` (Autodesk.Navisworks.Api.NamedConstant)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.NamedConstant.UserEquals%28Autodesk.Navisworks.Api.NamedConstant%29)

### Properties
- `BaseName` (string, get) — Base name for constant. Related constants (enumerations) have the same base name.
- `DisplayName` (string, get) — Display name of constant (localized)
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Name` (string, get) — Name of constant
- `Value` (int, get) — Value of constant

## NamedConstantCollection (class)

A collection of NamedConstant objects.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.NamedConstantCollection)

### Constructors
- `NamedConstantCollection()` — Create an empty collection
- `NamedConstantCollection(Autodesk.Navisworks.Api.NamedConstantCollection from)` — Create a collection that is a copy of from

### Methods
#### `bool Contains(Autodesk.Navisworks.Api.NamedConstant item)`

Determines whether the SavedItemCollection contains a specific value.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.NamedConstant)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.NamedConstantCollection.Contains%28Autodesk.Navisworks.Api.NamedConstant%29)

#### `void CopyTo(Autodesk.Navisworks.Api.NamedConstant[] array, int arrayIndex)`

Copies the elements of the ICollection`1 to an Array, starting at a particular Array index.

**Parameters:**
- `array` (Autodesk.Navisworks.Api.NamedConstant[]) — The one-dimensional Array that is the destination of the elements copied from ICollection`1. The Array must have zero-based indexing.
- `arrayIndex` (int) — The zero-based index in array at which copying begins.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.NamedConstantCollection.CopyTo%28Autodesk.Navisworks.Api.NamedConstant%5B%5D%2CSystem.Int32%29)

#### `void CopyTo(System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.NamedConstant> to)`

Enumerates over the collection and copies the items into to

**Parameters:**
- `to` (System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.NamedConstant>) — Collection to copy into

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.NamedConstantCollection.CopyTo%28System.Collections.Generic.ICollection%7BAutodesk.Navisworks.Api.NamedConstant%7D%29)

#### `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.NamedConstant> GetEnumerator()`

Returns an enumerator that can iterate through the collection.

**Returns:** `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.NamedConstant>` — An IEnumerator that can be used to iterate through the collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.NamedConstantCollection.GetEnumerator)

#### `int IndexOf(Autodesk.Navisworks.Api.NamedConstant item)`

Determines the index of a specific item in the Collection

**Parameters:**
- `item` (Autodesk.Navisworks.Api.NamedConstant)

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.NamedConstantCollection.IndexOf%28Autodesk.Navisworks.Api.NamedConstant%29)

#### `static Autodesk.Navisworks.Api.NamedConstantCollection InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of NamedConstantCollection.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.NamedConstantCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.NamedConstantCollection.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of NamedConstantCollection.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.NamedConstantCollection.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

### Properties
- `Count` (int, get) — Gets the number of elements contained in the collection.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Item` (Autodesk.Navisworks.Api.NamedConstant, get) — Item property of NamedConstantCollection.

## NativeHandle (class)

Common base class for classes which act as a managed handle to an implementation in native, unmanaged code

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.NativeHandle)

### Methods
#### `void Dispose()`

Performs application-defined tasks associated with freeing, releasing, or resetting unmanaged resources.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.NativeHandle.Dispose)

#### `bool Equals(object obj)`

Determines whether the specified object and the current object refer to the same underlying native object. Returns false if either object has been disposed.

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.NativeHandle.Equals%28System.Object%29)

#### `int GetHashCode()`

Serves as a hash function for a particular type. GetHashCode is suitable for use in hashing algorithms and data structures like a hash table.

**Returns:** `int` — A hash code for the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.NativeHandle.GetHashCode)

#### `static bool ReferenceEquals(Autodesk.Navisworks.Api.NativeHandle objA, Autodesk.Navisworks.Api.NativeHandle objB)`

Determines whether the two handles refer to the same underlying native object. Returns false if either handle has been disposed.

**Parameters:**
- `objA` (Autodesk.Navisworks.Api.NativeHandle) — First NativeHandle
- `objB` (Autodesk.Navisworks.Api.NativeHandle) — Second NativeHandle

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.NativeHandle.ReferenceEquals%28Autodesk.Navisworks.Api.NativeHandle%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `string ToString()`

Returns a String that represents the current Object.

**Returns:** `string` — A String that represents the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.NativeHandle.ToString)

#### `static bool op_Equality(Autodesk.Navisworks.Api.NativeHandle objA, Autodesk.Navisworks.Api.NativeHandle objB)`

Determines whether the two objects refer to the same underlying native object. Returns false if either object has been disposed.

**Parameters:**
- `objA` (Autodesk.Navisworks.Api.NativeHandle)
- `objB` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.NativeHandle.op_Equality%28Autodesk.Navisworks.Api.NativeHandle%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static bool op_Inequality(Autodesk.Navisworks.Api.NativeHandle objA, Autodesk.Navisworks.Api.NativeHandle objB)`

Determines whether the two objects refer to different underlying native objects. Returns true if either object has been disposed.

**Parameters:**
- `objA` (Autodesk.Navisworks.Api.NativeHandle)
- `objB` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.NativeHandle.op_Inequality%28Autodesk.Navisworks.Api.NativeHandle%2CAutodesk.Navisworks.Api.NativeHandle%29)

### Properties
- `IsDisposed` (bool, get) — Gets a value indicating whether the object has been disposed and is no longer available.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.

## NwdExportOptions (class)

NwdExportOptions class in Autodesk.Navisworks.Api.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.NwdExportOptions)

### Constructors
- `NwdExportOptions()` — Default constructor

### Methods
#### `static Autodesk.Navisworks.Api.NwdExportOptions InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of NwdExportOptions.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.NwdExportOptions` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.NwdExportOptions.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of NwdExportOptions.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.NwdExportOptions.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

### Properties
- `EmbedXrefs` (bool, get/set) — Should any externally referenced files including ReCap scans and textures be embedded in the exported file?
- `ExcludeHiddenItems` (bool, get/set) — Should hidden ModelItems be removed from the exported file?
- `FileVersion` (int, get/set) — File version to write out to.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `PreventObjectPropertyExport` (bool, get/set) — Should normal object properties be removed from the exported file?

## PickItemResult (class)

Represents the result of doing a HitTest in a view

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.PickItemResult)

### Constructors
- `PickItemResult()` — Constructor for PickItemResult.

### Properties
- `ModelItem` (Autodesk.Navisworks.Api.ModelItem, get/set) — The model item that was picked.
- `Normal` (Autodesk.Navisworks.Api.UnitVector3D, get/set) — The normal of the picked item
- `Point` (Autodesk.Navisworks.Api.Point3D, get/set) — The picked point on the surface of the picked model item (in world space).
- `ResultBits` (Autodesk.Navisworks.Api.PickResults, get/set) — The type of item that was picked

## PickPrimitives (enum)

Different types of primitive used to define geometry

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.PickPrimitives)

### Values
- `None` = `0` — Nothing extra to return or do
- `Edge0To1` = `1` — Snap to tri edge between vertex 0 and 1
- `Edge1To2` = `2` — Snap to tri edge between vertex 1 and 2
- `Edge2To0` = `4` — Snap to tri edge between vertex 2 and 0
- `Edge` = `7` — Snap to any tri edge
- `Vertex0` = `16` — Snap to tri vertex 0
- `Vertex1` = `32` — Snap to tri vertex 1
- `Vertex2` = `64` — Snap to tri vertex 2
- `Vertex` = `112` — Snap to any vertex
- `Point` = `256` — Point at pick
- `Distance` = `512` — Distance at pick
- `NearThreshold` = `4096` — Nearer than near_threshold
- `Center` = `8192` — Nearest to center, rather than to camera
- `CenterExactSurface` = `16384` — Ignore surface not under center
- `LineVertex0` = `65536` — Snap to line vertex 0
- `LineVertex1` = `131072` — Snap to line vertex 1
- `LineVertex` = `196608` — Snap to any line vertex
- `LineMiddle` = `262144` — Snap to middle of the line
- `ArcCenter` = `1048576` — Snap to the center of circular and elliptical arcs

## PickResults (enum)

The result of a pick action.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.PickResults)

### Values
- `None` = `0`
- `Edge0To1` = `1`
- `Edge1To2` = `2`
- `Edge2To0` = `4`
- `Edge` = `7`
- `Vertex0` = `16`
- `Vertex1` = `32`
- `Vertex2` = `64`
- `Vertex` = `112`
- `Contained` = `128`
- `LineVertex0` = `65536`
- `LineVertex1` = `131072`
- `LineVertex` = `196608`
- `LineMiddle` = `262144`
- `ArcCenter` = `1048576`

## Point2D (class)

Represents a position in 2D space.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Point2D)

### Constructors
- `Point2D()` — Create a zero vector
- `Point2D(Autodesk.Navisworks.Api.VariantData data)` — Create a point from a data object of type Point2D
- `Point2D(double x, double y)` — Create a point from X and Y components

### Methods
#### `Autodesk.Navisworks.Api.Point2D Add(Autodesk.Navisworks.Api.Vector2D vector)`

Translates a point by a vector to produce a new point

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector2D)

**Returns:** `Autodesk.Navisworks.Api.Point2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point2D.Add%28Autodesk.Navisworks.Api.Vector2D%29)

#### `double DistanceTo(Autodesk.Navisworks.Api.Point2D other)`

Distance from this point to another point

**Parameters:**
- `other` (Autodesk.Navisworks.Api.Point2D)

**Returns:** `double` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point2D.DistanceTo%28Autodesk.Navisworks.Api.Point2D%29)

#### `double DistanceToSquared(Autodesk.Navisworks.Api.Point2D other)`

Distance squared from this point to another point

**Parameters:**
- `other` (Autodesk.Navisworks.Api.Point2D)

**Returns:** `double` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point2D.DistanceToSquared%28Autodesk.Navisworks.Api.Point2D%29)

#### `bool Equals(object obj)`

Determines whether the specified object and the current object refer to the same underlying native object. Returns false if either object has been disposed.

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point2D.Equals%28System.Object%29)

#### `int GetHashCode()`

Serves as a hash function for a particular type. GetHashCode is suitable for use in hashing algorithms and data structures like a hash table.

**Returns:** `int` — A hash code for the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point2D.GetHashCode)

#### `static Autodesk.Navisworks.Api.Point2D InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of Point2D.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Point2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point2D.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of Point2D.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point2D.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `Autodesk.Navisworks.Api.Point2D Subtract(Autodesk.Navisworks.Api.Vector2D vector)`

Translates a point by a vector to produce a new point

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector2D)

**Returns:** `Autodesk.Navisworks.Api.Point2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point2D.Subtract%28Autodesk.Navisworks.Api.Vector2D%29)

#### `Autodesk.Navisworks.Api.Vector2D Subtract(Autodesk.Navisworks.Api.Point2D point)`

Creates a new vector from point to this

**Parameters:**
- `point` (Autodesk.Navisworks.Api.Point2D)

**Returns:** `Autodesk.Navisworks.Api.Vector2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point2D.Subtract%28Autodesk.Navisworks.Api.Point2D%29)

#### `Autodesk.Navisworks.Api.VariantData ToData()`

Converts point to a data object of type Point3D

**Returns:** `Autodesk.Navisworks.Api.VariantData` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point2D.ToData)

#### `string ToString()`

Returns a String that represents the current Object.

**Returns:** `string` — A String that represents the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point2D.ToString)

#### `bool TolerantEquals(Autodesk.Navisworks.Api.Point2D point)`

Compare points using a tolerance

**Parameters:**
- `point` (Autodesk.Navisworks.Api.Point2D)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point2D.TolerantEquals%28Autodesk.Navisworks.Api.Point2D%29)

#### `bool TolerantEquals(Autodesk.Navisworks.Api.Point2D point, double tolerance)`

Compare points using a tolerance

**Parameters:**
- `point` (Autodesk.Navisworks.Api.Point2D)
- `tolerance` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point2D.TolerantEquals%28Autodesk.Navisworks.Api.Point2D%2CSystem.Double%29)

#### `static bool TolerantEquals(Autodesk.Navisworks.Api.Point2D point0, Autodesk.Navisworks.Api.Point2D point1)`

Compare points using a tolerance

**Parameters:**
- `point0` (Autodesk.Navisworks.Api.Point2D)
- `point1` (Autodesk.Navisworks.Api.Point2D)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point2D.TolerantEquals%28Autodesk.Navisworks.Api.Point2D%2CAutodesk.Navisworks.Api.Point2D%29)

#### `static bool TolerantEquals(Autodesk.Navisworks.Api.Point2D point0, Autodesk.Navisworks.Api.Point2D point1, double tolerance)`

Compare points using a tolerance

**Parameters:**
- `point0` (Autodesk.Navisworks.Api.Point2D)
- `point1` (Autodesk.Navisworks.Api.Point2D)
- `tolerance` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point2D.TolerantEquals%28Autodesk.Navisworks.Api.Point2D%2CAutodesk.Navisworks.Api.Point2D%2CSystem.Double%29)

### Properties
- `Distance` (double, get) — Distance of point from origin
- `DistanceSquared` (double, get) — Distance of point from origin squared
- `IsOrigin` (bool, get) — Is this point at the origin (all components zero) ?
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Item` (double, get) — Item property of Point2D.
- `Origin` (Autodesk.Navisworks.Api.Point2D, get) — Return point at the origin (all components zero)
- `X` (double, get) — Returns the X component of the point
- `Y` (double, get) — Returns the Y component of the point

## Point2DList (class)

List of 2D points.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Point2DList)

### Constructors
- `Point2DList()` — Creates an empty list

### Methods
#### `void Add(Autodesk.Navisworks.Api.Point2D point)`

Adds a point to the back of the list

**Parameters:**
- `point` (Autodesk.Navisworks.Api.Point2D)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point2DList.Add%28Autodesk.Navisworks.Api.Point2D%29)

#### `static Autodesk.Navisworks.Api.Point2DList InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of Point2DList.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Point2DList` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point2DList.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of Point2DList.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point2DList.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

### Properties
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.

## Point3D (class)

Represents a position in 3D space. Immutable.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Point3D)

### Constructors
- `Point3D()` — Create a zero vector
- `Point3D(Autodesk.Navisworks.Api.VariantData data)` — Create a point from a data object of type Point3D
- `Point3D(double x, double y, double z)` — Create a point from X, Y and Z components

### Methods
#### `Autodesk.Navisworks.Api.Point3D Add(Autodesk.Navisworks.Api.Vector3D vector)`

Translates a point by a vector to produce a new point

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector3D)

**Returns:** `Autodesk.Navisworks.Api.Point3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point3D.Add%28Autodesk.Navisworks.Api.Vector3D%29)

#### `double DistanceTo(Autodesk.Navisworks.Api.Point3D other)`

Distance from this point to another point

**Parameters:**
- `other` (Autodesk.Navisworks.Api.Point3D)

**Returns:** `double` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point3D.DistanceTo%28Autodesk.Navisworks.Api.Point3D%29)

#### `double DistanceToSquared(Autodesk.Navisworks.Api.Point3D other)`

Distance squared from this point to another point

**Parameters:**
- `other` (Autodesk.Navisworks.Api.Point3D)

**Returns:** `double` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point3D.DistanceToSquared%28Autodesk.Navisworks.Api.Point3D%29)

#### `bool Equals(object obj)`

Determines whether the specified object and the current object refer to the same underlying native object. Returns false if either object has been disposed.

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point3D.Equals%28System.Object%29)

#### `int GetHashCode()`

Serves as a hash function for a particular type. GetHashCode is suitable for use in hashing algorithms and data structures like a hash table.

**Returns:** `int` — A hash code for the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point3D.GetHashCode)

#### `static Autodesk.Navisworks.Api.Point3D InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of Point3D.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Point3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point3D.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of Point3D.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point3D.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `Autodesk.Navisworks.Api.Point3D Subtract(Autodesk.Navisworks.Api.Vector3D vector)`

Translates a point by a vector to produce a new point

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector3D)

**Returns:** `Autodesk.Navisworks.Api.Point3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point3D.Subtract%28Autodesk.Navisworks.Api.Vector3D%29)

#### `Autodesk.Navisworks.Api.Vector3D Subtract(Autodesk.Navisworks.Api.Point3D point)`

Creates a new vector from point to this

**Parameters:**
- `point` (Autodesk.Navisworks.Api.Point3D)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point3D.Subtract%28Autodesk.Navisworks.Api.Point3D%29)

#### `Autodesk.Navisworks.Api.VariantData ToData()`

Converts point to a data object of type Point3D

**Returns:** `Autodesk.Navisworks.Api.VariantData` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point3D.ToData)

#### `string ToString()`

Returns a String that represents the current Object.

**Returns:** `string` — A String that represents the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point3D.ToString)

#### `Autodesk.Navisworks.Api.Vector3D ToVector3D()`

Convert point to a vector (equivalent to vector from origin to point)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point3D.ToVector3D)

#### `bool TolerantEquals(Autodesk.Navisworks.Api.Point3D point)`

Compare points using a tolerance

**Parameters:**
- `point` (Autodesk.Navisworks.Api.Point3D)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point3D.TolerantEquals%28Autodesk.Navisworks.Api.Point3D%29)

#### `bool TolerantEquals(Autodesk.Navisworks.Api.Point3D point, double tolerance)`

Compare points using a tolerance

**Parameters:**
- `point` (Autodesk.Navisworks.Api.Point3D)
- `tolerance` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point3D.TolerantEquals%28Autodesk.Navisworks.Api.Point3D%2CSystem.Double%29)

#### `static bool TolerantEquals(Autodesk.Navisworks.Api.Point3D point0, Autodesk.Navisworks.Api.Point3D point1)`

Compare points using a tolerance

**Parameters:**
- `point0` (Autodesk.Navisworks.Api.Point3D)
- `point1` (Autodesk.Navisworks.Api.Point3D)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point3D.TolerantEquals%28Autodesk.Navisworks.Api.Point3D%2CAutodesk.Navisworks.Api.Point3D%29)

#### `static bool TolerantEquals(Autodesk.Navisworks.Api.Point3D point0, Autodesk.Navisworks.Api.Point3D point1, double tolerance)`

Compare points using a tolerance

**Parameters:**
- `point0` (Autodesk.Navisworks.Api.Point3D)
- `point1` (Autodesk.Navisworks.Api.Point3D)
- `tolerance` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point3D.TolerantEquals%28Autodesk.Navisworks.Api.Point3D%2CAutodesk.Navisworks.Api.Point3D%2CSystem.Double%29)

#### `static Autodesk.Navisworks.Api.Point3D op_Addition(Autodesk.Navisworks.Api.Point3D value1, Autodesk.Navisworks.Api.Vector3D value2)`

Translates a point by a vector to produce a new point

**Parameters:**
- `value1` (Autodesk.Navisworks.Api.Point3D)
- `value2` (Autodesk.Navisworks.Api.Vector3D)

**Returns:** `Autodesk.Navisworks.Api.Point3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point3D.op_Addition%28Autodesk.Navisworks.Api.Point3D%2CAutodesk.Navisworks.Api.Vector3D%29)

#### `static Autodesk.Navisworks.Api.Point3D op_Addition(Autodesk.Navisworks.Api.Vector3D value1, Autodesk.Navisworks.Api.Point3D value2)`

Translates a vector by a point to produce a new point

**Parameters:**
- `value1` (Autodesk.Navisworks.Api.Vector3D)
- `value2` (Autodesk.Navisworks.Api.Point3D)

**Returns:** `Autodesk.Navisworks.Api.Point3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point3D.op_Addition%28Autodesk.Navisworks.Api.Vector3D%2CAutodesk.Navisworks.Api.Point3D%29)

#### `static Autodesk.Navisworks.Api.Point3D op_Subtraction(Autodesk.Navisworks.Api.Point3D value1, Autodesk.Navisworks.Api.Vector3D value2)`

Translates a point by a vector to produce a new point

**Parameters:**
- `value1` (Autodesk.Navisworks.Api.Point3D)
- `value2` (Autodesk.Navisworks.Api.Vector3D)

**Returns:** `Autodesk.Navisworks.Api.Point3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point3D.op_Subtraction%28Autodesk.Navisworks.Api.Point3D%2CAutodesk.Navisworks.Api.Vector3D%29)

#### `static Autodesk.Navisworks.Api.Vector3D op_Subtraction(Autodesk.Navisworks.Api.Point3D value1, Autodesk.Navisworks.Api.Point3D value2)`

Creates a new Vector from value2 to value1

**Parameters:**
- `value1` (Autodesk.Navisworks.Api.Point3D)
- `value2` (Autodesk.Navisworks.Api.Point3D)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point3D.op_Subtraction%28Autodesk.Navisworks.Api.Point3D%2CAutodesk.Navisworks.Api.Point3D%29)

### Properties
- `Distance` (double, get) — Distance of point from origin
- `DistanceSquared` (double, get) — Distance of point from origin squared
- `IsOrigin` (bool, get) — Is this point at the origin (all components zero) ?
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Item` (double, get) — Item property of Point3D.
- `Origin` (Autodesk.Navisworks.Api.Point3D, get) — Return point at the origin (all components zero)
- `X` (double, get) — Returns the X component of the point
- `Y` (double, get) — Returns the Y component of the point
- `Z` (double, get) — Returns the Z component of the point

## Point3DList (class)

List of 3D points.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Point3DList)

### Constructors
- `Point3DList()` — Creates an empty list

### Methods
#### `void Add(Autodesk.Navisworks.Api.Point3D point)`

Adds a point to the back of the list

**Parameters:**
- `point` (Autodesk.Navisworks.Api.Point3D)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point3DList.Add%28Autodesk.Navisworks.Api.Point3D%29)

#### `static Autodesk.Navisworks.Api.Point3DList InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of Point3DList.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Point3DList` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point3DList.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of Point3DList.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Point3DList.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

### Properties
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.

## PointInfo (class)

PointInfo class in Autodesk.Navisworks.Api.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.PointInfo)

### Constructors
- `PointInfo()` — Constructs a new PointInfo.

### Properties
- `DisplayName` (string, get/set) — DisplayName property of PointInfo.
- `InternalName` (string, get/set) — InternalName property of PointInfo.
- `LongInfo` (System.Collections.Generic.IEnumerable<string>, get/set) — LongInfo property of PointInfo.
- `ShortInfo` (string, get/set) — ShortInfo property of PointInfo.

## PrimitiveTypes (enum)

Different types of primitive used to define geometry

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.PrimitiveTypes)

### Values
- `None` = `0` — No primitives
- `Triangles` = `1` — Includes triangle primitives
- `Lines` = `2` — Includes line primitives
- `Points` = `4` — Includes point primitives
- `SnapPoints` = `8` — Includes snap point primitives
- `Text` = `16` — Includes text primitives

## Progress (class)

Used to report progress of lengthy operations

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Progress)

### Methods
#### `void BeginSubOperation(double fractionOfRemainingTime)`

Starts a sub operation. Progress in sub op between 0 and 1 maps to fraction of remaining time in main operation. Can be nested.

**Parameters:**
- `fractionOfRemainingTime` (double) — Fraction of remaining time this sub operation will take, between 0 and 1

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Progress.BeginSubOperation%28System.Double%29)

#### `void BeginSubOperation(double fractionOfRemainingTime, string message)`

Starts a sub operation. Progress in sub op between 0 and 1 maps to fraction of remaining time in main operation. Can be nested.

**Parameters:**
- `fractionOfRemainingTime` (double) — Fraction of remaining time this sub operation will take, between 0 and 1
- `message` (string) — User message to display for this sub operation.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Progress.BeginSubOperation%28System.Double%2CSystem.String%29)

#### `void Cancel()`

Forces operation to cancel next time Double) is called

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Progress.Cancel)

#### `void EndSubOperation()`

Ends the current sub operation. It may have either fully completed or ended early.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Progress.EndSubOperation)

#### `void EndSubOperation(bool completed)`

Ends the current sub operation. It may have either fully completed or ended early.

**Parameters:**
- `completed` (bool) — States whether the sub operation completed or not.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Progress.EndSubOperation%28System.Boolean%29)

#### `void Hide()`

Hide the progress if it is a dialog and it needs to get out of the way of another window that is about to open.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Progress.Hide)

#### `static Autodesk.Navisworks.Api.Progress InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of Progress.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Progress` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Progress.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of Progress.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Progress.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `bool ReportError(int errorCode)`

Report an error immediately to the user.

**Parameters:**
- `errorCode` (int) — Application/operation-defined error code.

**Returns:** `bool` — false if user wants to cancel operation, true if they want to ignore and continue.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Progress.ReportError%28System.Int32%29)

#### `bool ReportError(int errorCode, string message)`

Report an error immediately to the user.

**Parameters:**
- `errorCode` (int) — Application/operation-defined error code.
- `message` (string) — User error message.

**Returns:** `bool` — false if user wants to cancel operation, true if they want to ignore and continue.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Progress.ReportError%28System.Int32%2CSystem.String%29)

#### `bool ReportError(int errorCode, string message, bool isFileError)`

Report an error immediately to the user.

**Parameters:**
- `errorCode` (int) — Application/operation-defined error code.
- `message` (string) — User error message.
- `isFileError` (bool) — Is the error related to the current working filename ?

**Returns:** `bool` — false if user wants to cancel operation, true if they want to ignore and continue.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Progress.ReportError%28System.Int32%2CSystem.String%2CSystem.Boolean%29)

#### `void Show()`

Show the progress if it is a dialog and was previously hidden.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Progress.Show)

#### `bool Update(double fractionDone)`

Updates progress on the current operation or sub-operation

**Parameters:**
- `fractionDone` (double) — Fraction of operation or sub-operation completed, between 0 and 1

**Returns:** `bool` — false if user wants to cancel operation, true if they want to continue.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Progress.Update%28System.Double%29)

### Properties
- `IsCanceled` (bool, get) — Determines if the operation is cancelled.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `WorkingFileName` (string, get/set) — The working filename. Set if multiple files are being processed in this operation to give more detailed information.

## ProgressBeginningEventArgs (class)

Event arguments used for the Application::ProgressBeginning event

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ProgressBeginningEventArgs)

### Properties
- `Title` (string, get/set) — Gets the title of the overall operation that progress is being reported for. Displayed in the title bar of the Navisworks progress dialog.

## ProgressEndedEventArgs (class)

Event arguments used for the Application::ProgressEnded event

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ProgressEndedEventArgs)

## ProgressErrorReportingEventArgs (class)

Event arguments used for the Application::ProgressErrorReporting event

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ProgressErrorReportingEventArgs)

### Properties
- `Canceled` (bool, get/set) — Sets a value indicating whether the operation should be canceled.
- `ErrorCode` (int, get/set) — Gets the error code that describes the current Error.
- `IsFileError` (bool, get/set) — Gets whether the Error being reported was related to a file operation.
- `Message` (string, get/set) — Gets a message that describes the current error.
- `Progress` (Autodesk.Navisworks.Api.Progress, get/set) — Gets a reference to the Progress class instance relating to this Event.

## ProgressEventArgs (class)

Base class for progress related events where event handler can replace standard implementation

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ProgressEventArgs)

### Properties
- `Handled` (bool, get/set) — Sets a value that indicates whether the event handler has completely handled the event or whether the system should continue its own processing.

## ProgressMessageReportingEventArgs (class)

Event arguments used for the Application.ProgressMessageReporting event

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ProgressMessageReportingEventArgs)

### Properties
- `Message` (string, get/set) — Gets a message describing the current operation. Displayed in Navisworks progress dialog.
- `Progress` (Autodesk.Navisworks.Api.Progress, get/set) — Gets a reference to the Progress class instance relating to this Event.

## ProgressSubOperationBeganEventArgs (class)

Event arguments used for the Application::ProgressSubOperationBeganEventArgs event

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ProgressSubOperationBeganEventArgs)

### Properties
- `FractionOfRemainingTime` (double, get/set) — Gets the fraction of remaining time this sub operation will take
- `Message` (string, get/set) — Message property of ProgressSubOperationBeganEventArgs.
- `Progress` (Autodesk.Navisworks.Api.Progress, get/set) — Gets a reference to the Progress class instance relating to this Event.

## ProgressSubOperationEndedEventArgs (class)

Event arguments used for the Application::ProgressSubOperationEnded event

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ProgressSubOperationEndedEventArgs)

### Properties
- `Completed` (bool, get/set) — Gets a value indicating whether the sub-operation was fully completed
- `Progress` (Autodesk.Navisworks.Api.Progress, get/set) — Gets a reference to the Progress class instance relating to this Event.

## ProgressUpdatingEventArgs (class)

Event arguments used for the Application::ProgressUpdating event

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ProgressUpdatingEventArgs)

### Properties
- `Canceled` (bool, get/set) — Sets a value indicating whether the operation should be canceled.
- `OverallFractionDone` (double, get/set) — Gets the fraction of the overall progress operation that has been done. Displayed in title bar of Navisworks progress dialog.
- `Progress` (Autodesk.Navisworks.Api.Progress, get/set) — Gets a reference to the Progress class instance relating to this Event.
- `SubOperationFractionDone` (double, get/set) — Gets the fraction of the current progress sub-operation that has been done. Displayed in the progress bar of the Navisworks progress dialog.

## ProgressVisibilityEventArgs (class)

Event arguments used for the Application::ProgressVisibility event

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ProgressVisibilityEventArgs)

### Properties
- `Progress` (Autodesk.Navisworks.Api.Progress, get/set) — Gets a reference to the Progress class instance relating to this Event.
- `WantVisible` (bool, get/set) — Gets a value indicating whether the progress dialog should be visible or not

## ProjectionResult (class)

Represents result of projecting a point.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ProjectionResult)

### Constructors
- `ProjectionResult()` — Constructor for a projection result.

### Properties
- `Depth` (double, get/set) — Depth value (in clip space).
- `X` (int, get/set) — X screen co-ordinate.
- `Y` (int, get/set) — Y screen co-ordinate.

## PropertyCategory (class)

A named category of properties

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.PropertyCategory)

### Methods
#### `void Dispose()`

Dispose method of PropertyCategory.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.PropertyCategory.Dispose)

#### `byte[] GetBinaryData()`

Obtain any associated binary data if HasBinaryData is true

**Returns:** `byte[]` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.PropertyCategory.GetBinaryData)

#### `long GetInt64StableId()`

Returns value of the Int64 stable id contained in this category.

**Returns:** `long` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.PropertyCategory.GetInt64StableId)

#### `string GetStringStableId()`

Returns value of the String stable id contained in this category.

**Returns:** `string` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.PropertyCategory.GetStringStableId)

#### `string ToString()`

Returns a String that represents the current Object.

**Returns:** `string` — A String that represents the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.PropertyCategory.ToString)

### Properties
- `CombinedName` (Autodesk.Navisworks.Api.NamedConstant, get) — Combined name of category
- `DisplayName` (string, get) — Display name of category (localized)
- `HasBinaryData` (bool, get) — Does this category represent binary data?
- `HasInt64StableId` (bool, get) — Does this category contain an Int64 stable id ?
- `HasSchemaData` (bool, get) — Does this category represent any type of SchemaData?
- `HasStableId` (bool, get) — Does this category contain any type of stable id ? A stable id can be used to identify the same ModelItem across multiple versions of a Model. A category with a stable id contains only a single property with the value of the id.
- `HasStringStableId` (bool, get) — Does this category contain a String stable id ?
- `IsDisposed` (bool, get) — Has this object been disposed ?
- `IsEditable` (bool, get) — IsEditable property of PropertyCategory.
- `Name` (string, get) — Name of category (suitable for programmatic use)
- `Properties` (Autodesk.Navisworks.Api.DataPropertyCollection, get) — The properties in this category
- `SchemaData` (Autodesk.Navisworks.Api.Schema.SchemaData, get) — Obtain any associated SchemaData if HasSchemaData is true

## PropertyCategoryCollection (class)

A collection of PropertyCategory objects.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.PropertyCategoryCollection)

### Methods
#### `Autodesk.Navisworks.Api.PropertyCategory FindCategoryByCombinedName(Autodesk.Navisworks.Api.NamedConstant combinedName)`

Find first category with specified CombinedName

**Parameters:**
- `combinedName` (Autodesk.Navisworks.Api.NamedConstant)

**Returns:** `Autodesk.Navisworks.Api.PropertyCategory` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.PropertyCategoryCollection.FindCategoryByCombinedName%28Autodesk.Navisworks.Api.NamedConstant%29)

#### `Autodesk.Navisworks.Api.PropertyCategory FindCategoryByDisplayName(string displayName)`

Find first category with specified DisplayName

**Parameters:**
- `displayName` (string)

**Returns:** `Autodesk.Navisworks.Api.PropertyCategory` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.PropertyCategoryCollection.FindCategoryByDisplayName%28System.String%29)

#### `Autodesk.Navisworks.Api.PropertyCategory FindCategoryByName(string name)`

Find first category with specified Name

**Parameters:**
- `name` (string)

**Returns:** `Autodesk.Navisworks.Api.PropertyCategory` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.PropertyCategoryCollection.FindCategoryByName%28System.String%29)

#### `Autodesk.Navisworks.Api.PropertyCategory FindCategoryWithStableId()`

Find first category with a stable id

**Returns:** `Autodesk.Navisworks.Api.PropertyCategory` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.PropertyCategoryCollection.FindCategoryWithStableId)

#### `Autodesk.Navisworks.Api.DataProperty FindPropertyByCombinedName(Autodesk.Navisworks.Api.NamedConstant categoryCombinedName, Autodesk.Navisworks.Api.NamedConstant propertyCombinedName)`

Find first property with specified CombinedName and CombinedName

**Parameters:**
- `categoryCombinedName` (Autodesk.Navisworks.Api.NamedConstant)
- `propertyCombinedName` (Autodesk.Navisworks.Api.NamedConstant)

**Returns:** `Autodesk.Navisworks.Api.DataProperty` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.PropertyCategoryCollection.FindPropertyByCombinedName%28Autodesk.Navisworks.Api.NamedConstant%2CAutodesk.Navisworks.Api.NamedConstant%29)

#### `Autodesk.Navisworks.Api.DataProperty FindPropertyByDisplayName(string categoryDisplayName, string propertyDisplayName)`

Find first property with specified DisplayName and DisplayName

**Parameters:**
- `categoryDisplayName` (string)
- `propertyDisplayName` (string)

**Returns:** `Autodesk.Navisworks.Api.DataProperty` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.PropertyCategoryCollection.FindPropertyByDisplayName%28System.String%2CSystem.String%29)

#### `Autodesk.Navisworks.Api.DataProperty FindPropertyByName(string categoryName, string propertyName)`

Find first property with specified Name and Name

**Parameters:**
- `categoryName` (string)
- `propertyName` (string)

**Returns:** `Autodesk.Navisworks.Api.DataProperty` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.PropertyCategoryCollection.FindPropertyByName%28System.String%2CSystem.String%29)

#### `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.PropertyCategory> GetEnumerator()`

Returns an enumerator that can iterate through the collection.

**Returns:** `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.PropertyCategory>` — An IEnumerator that can be used to iterate through the collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.PropertyCategoryCollection.GetEnumerator)

#### `static Autodesk.Navisworks.Api.PropertyCategoryCollection InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of PropertyCategoryCollection.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.PropertyCategoryCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.PropertyCategoryCollection.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of PropertyCategoryCollection.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.PropertyCategoryCollection.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `System.Collections.IEnumerator InternalGetEnumerator()`

InternalGetEnumerator method of PropertyCategoryCollection.

**Returns:** `System.Collections.IEnumerator` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.PropertyCategoryCollection.InternalGetEnumerator)

#### `string InternalToJson()`

InternalToJson method of PropertyCategoryCollection.

**Returns:** `string` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.PropertyCategoryCollection.InternalToJson)

### Properties
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.

## PropertyCategoryNames (class)

Names (values of Name) of commonly found property categories.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.PropertyCategoryNames)

### Properties
- `AutoCad` (string, get) — Name of AutoCAD "AutoCad" property category
- `AutoCadEntityHandle` (string, get) — Name of AutoCAD "Entity Handle" property category
- `Geometry` (string, get) — Name of "Geometry" property category
- `Hyperlinks` (string, get) — Name of "Hyperlinks" property category
- `Item` (string, get) — Name of "Item" property category
- `Material` (string, get) — Name of "Material" property category
- `Microstation` (string, get) — Name of Microstation "MicroStation" property category
- `MicrostationElementId` (string, get) — Name of Microstation "Element Id" property category
- `RevitElementId` (string, get) — Name of Revit "Element Id" property category
- `Transform` (string, get) — Name of "Transform" property category
- `XRef` (string, get) — Name of "XRef" property category

## PublishProperties (class)

Set of properties used when publishing a file

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.PublishProperties)

### Constructors
- `PublishProperties()` — Default constructor
- `PublishProperties(Autodesk.Navisworks.Api.PublishProperties value)` — Copy constructor

### Methods
#### `static Autodesk.Navisworks.Api.PublishProperties InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of PublishProperties.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.PublishProperties` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.PublishProperties.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of PublishProperties.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.PublishProperties.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `void RemoveExpiryDate()`

Remove any expiry date that has been set

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.PublishProperties.RemoveExpiryDate)

#### `void RemovePassword()`

Remove any password that has been set

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.PublishProperties.RemovePassword)

#### `void SetPassword(string password)`

Set a password to protect the content with when published

**Parameters:**
- `password` (string)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.PublishProperties.SetPassword%28System.String%29)

### Properties
- `AllowResave` (bool, get/set) — Can the file be resaved after it has been published ?
- `Author` (string, get/set) — Who created data ?
- `Comments` (string, get/set) — Any comments
- `Copyright` (string, get/set) — Any copyright statement
- `DisplayAtPassword` (bool, get/set) — Should the publish properties be displayed when asking the user for a password ?
- `DisplayOnOpen` (bool, get/set) — Should the publish properties be displayed when the published file is opened ?
- `EmbedDatabaseProperties` (bool, get/set) — Should any properties from linked databases be embedded in the published file ?
- `ExpiryDate` (System.DateTime, get/set) — Date that content expires. Only valid if HasExpiryDate is true.
- `HasBeenResaved` (bool, get) — Has the published file been resaved (and possibly modified) since it was published ?
- `HasExpiryDate` (bool, get) — Is there a valid expiry date ?
- `HasPassword` (bool, get) — Is the content password protected ?
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Keywords` (string, get/set) — Any keywords for searching
- `PublishDate` (System.DateTime, get/set) — Date of publication
- `PublishedFor` (string, get/set) — Who's the data intended for ?
- `Publisher` (string, get/set) — Who published data ?
- `Subject` (string, get/set) — Subject
- `Title` (string, get/set) — Title of published content

## Rotation3D (class)

A rotation in 3D space defined as a quaternion.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Rotation3D)

### Constructors
- `Rotation3D()` — Default constructor, makes an Identity rotation
- `Rotation3D(Autodesk.Navisworks.Api.UnitVector3D axis, Autodesk.Navisworks.Api.UnitVector3D vector1, Autodesk.Navisworks.Api.UnitVector3D vector2)` — Creates rotation about axis by angle between v1 and v2 projected onto plane normal to axis.
- `Rotation3D(Autodesk.Navisworks.Api.UnitVector3D axis, double angle)` — Creates rotation about given axis by angle in radians
- `Rotation3D(Autodesk.Navisworks.Api.UnitVector3D vector1, Autodesk.Navisworks.Api.UnitVector3D vector2)` — Creates rotation that rotates vector1 to same direction as vector2
- `Rotation3D(double a, double b, double c, double d)` — Creates rotation from 4 components

### Methods
#### `static Autodesk.Navisworks.Api.Rotation3D CreateFromEulerAngles(double x, double y, double z)`

Creates a Euler angle rotation. Parameters are in radians. Rotation is created by combination of rotations about X, Y and Z axes.

**Parameters:**
- `x` (double)
- `y` (double)
- `z` (double)

**Returns:** `Autodesk.Navisworks.Api.Rotation3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Rotation3D.CreateFromEulerAngles%28System.Double%2CSystem.Double%2CSystem.Double%29)

#### `bool Equals(object obj)`

Determines whether the specified object and the current object refer to the same underlying native object. Returns false if either object has been disposed.

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Rotation3D.Equals%28System.Object%29)

#### `int GetHashCode()`

Serves as a hash function for a particular type. GetHashCode is suitable for use in hashing algorithms and data structures like a hash table.

**Returns:** `int` — A hash code for the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Rotation3D.GetHashCode)

#### `static Autodesk.Navisworks.Api.Rotation3D InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of Rotation3D.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Rotation3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Rotation3D.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of Rotation3D.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Rotation3D.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `Autodesk.Navisworks.Api.Rotation3D Invert()`

Returns an inverted version of this rotation (all components inverted)

**Returns:** `Autodesk.Navisworks.Api.Rotation3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Rotation3D.Invert)

#### `bool IsIdentity()`

Determines if the Rotation is an identity

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Rotation3D.IsIdentity)

#### `Autodesk.Navisworks.Api.Rotation3D Normalize()`

Returns a normalized version of this rotation (magnitude of 1)

**Returns:** `Autodesk.Navisworks.Api.Rotation3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Rotation3D.Normalize)

#### `Autodesk.Navisworks.Api.AxisAndAngleResult ToAxisAndAngle()`

Calculates an axis and angle representation of this rotation.

**Returns:** `Autodesk.Navisworks.Api.AxisAndAngleResult` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Rotation3D.ToAxisAndAngle)

#### `Autodesk.Navisworks.Api.EulerAngleResult ToEulerAngles()`

Calculates the Euler angles for this rotation.

**Returns:** `Autodesk.Navisworks.Api.EulerAngleResult` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Rotation3D.ToEulerAngles)

#### `string ToString()`

Returns a String that represents the current Object.

**Returns:** `string` — A String that represents the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Rotation3D.ToString)

#### `bool TolerantEquals(Autodesk.Navisworks.Api.Rotation3D rotation)`

Equal within a tolerance, treating quaternion as 4D position. Equality of representation, not of rotation.

**Parameters:**
- `rotation` (Autodesk.Navisworks.Api.Rotation3D)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Rotation3D.TolerantEquals%28Autodesk.Navisworks.Api.Rotation3D%29)

#### `bool TolerantEquals(Autodesk.Navisworks.Api.Rotation3D rotation, double tolerance)`

Equal within a tolerance, treating quaternion as 4D position. Equality of representation, not of rotation.

**Parameters:**
- `rotation` (Autodesk.Navisworks.Api.Rotation3D)
- `tolerance` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Rotation3D.TolerantEquals%28Autodesk.Navisworks.Api.Rotation3D%2CSystem.Double%29)

### Properties
- `A` (double, get) — The 'A' component of the quaternion
- `B` (double, get) — The 'B' component of the quaternion
- `C` (double, get) — The 'C' component of the quaternion
- `D` (double, get) — The 'D' component of the quaternion
- `Identity` (Autodesk.Navisworks.Api.Rotation3D, get) — Creates an Identity rotation
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Item` (double, get) — Item property of Rotation3D.

## Rule (class)

An instance of a rule that is used by plugins to apply behaviour.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Rule)

### Methods
#### `Autodesk.Navisworks.Api.Rule CreateCopy()`

Creates a copy of the rule.

**Returns:** `Autodesk.Navisworks.Api.Rule` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Rule.CreateCopy)

#### `static Autodesk.Navisworks.Api.Rule InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of Rule.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Rule` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Rule.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of Rule.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Rule.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `bool ValueEquals(Autodesk.Navisworks.Api.Rule value)`

Performs comparison 'By Value'

**Parameters:**
- `value` (Autodesk.Navisworks.Api.Rule)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Rule.ValueEquals%28Autodesk.Navisworks.Api.Rule%29)

#### `static bool ValueEquals(Autodesk.Navisworks.Api.Rule value1, Autodesk.Navisworks.Api.Rule value2)`

Compares the two references 'By Value'

**Parameters:**
- `value1` (Autodesk.Navisworks.Api.Rule)
- `value2` (Autodesk.Navisworks.Api.Rule)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Rule.ValueEquals%28Autodesk.Navisworks.Api.Rule%2CAutodesk.Navisworks.Api.Rule%29)

### Properties
- `AllParamsAreSet` (bool, get) — Determines if all of the required parameters for the rule have been set.
- `DisplayName` (string, get/set) — Name of the rule as diplayed in the GUI.
- `IsEnabled` (bool, get/set) — Determines if this rule is enabled.
- `IsImplemented` (bool, get) — Determines if the plugin that implements this rule has been found.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `SomeParamsAreSet` (bool, get) — Determines if any of the required parameters for the rule have been set.

## RuleCollection (class)

A collection of Rules

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.RuleCollection)

### Constructors
- `RuleCollection()` — Create an empty collection

### Methods
#### `void Add(Autodesk.Navisworks.Api.Rule item)`

Adds item to the end of the collection.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.Rule)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.RuleCollection.Add%28Autodesk.Navisworks.Api.Rule%29)

#### `void AddRange(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.Rule> from)`

Add the items of the specified collection to the SavedItemCollection

**Parameters:**
- `from` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.Rule>) — The collection to add items from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.RuleCollection.AddRange%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.Rule%7D%29)

#### `void Clear()`

Removes all items from the collection

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.RuleCollection.Clear)

#### `bool Contains(Autodesk.Navisworks.Api.Rule item)`

Determines whether the SavedItemCollection contains a specific value.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.Rule)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.RuleCollection.Contains%28Autodesk.Navisworks.Api.Rule%29)

#### `void CopyFrom(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.Rule> from)`

Sets the contents of the collection to the items in from

**Parameters:**
- `from` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.Rule>) — The collection to copy contents from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.RuleCollection.CopyFrom%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.Rule%7D%29)

#### `void CopyTo(Autodesk.Navisworks.Api.Rule[] array, int arrayIndex)`

Copies the elements of the ICollection`1 to an Array, starting at a particular Array index.

**Parameters:**
- `array` (Autodesk.Navisworks.Api.Rule[]) — The one-dimensional Array that is the destination of the elements copied from ICollection`1. The Array must have zero-based indexing.
- `arrayIndex` (int) — The zero-based index in array at which copying begins.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.RuleCollection.CopyTo%28Autodesk.Navisworks.Api.Rule%5B%5D%2CSystem.Int32%29)

#### `void CopyTo(System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.Rule> to)`

Enumerates over the collection and copies the items into to

**Parameters:**
- `to` (System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.Rule>) — Collection to copy into

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.RuleCollection.CopyTo%28System.Collections.Generic.ICollection%7BAutodesk.Navisworks.Api.Rule%7D%29)

#### `Autodesk.Navisworks.Api.RuleCollection CreateCopy()`

Creates a deep copy of the collection.

**Returns:** `Autodesk.Navisworks.Api.RuleCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.RuleCollection.CreateCopy)

#### `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.Rule> GetEnumerator()`

Returns an enumerator that can iterate through the collection.

**Returns:** `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.Rule>` — An IEnumerator that can be used to iterate through the collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.RuleCollection.GetEnumerator)

#### `int IndexOf(Autodesk.Navisworks.Api.Rule item)`

Determines the index of a specific item in the Collection

**Parameters:**
- `item` (Autodesk.Navisworks.Api.Rule)

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.RuleCollection.IndexOf%28Autodesk.Navisworks.Api.Rule%29)

#### `void Insert(int index, Autodesk.Navisworks.Api.Rule item)`

Inserts item into the collection at the specified index

**Parameters:**
- `index` (int)
- `item` (Autodesk.Navisworks.Api.Rule)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.RuleCollection.Insert%28System.Int32%2CAutodesk.Navisworks.Api.Rule%29)

#### `static Autodesk.Navisworks.Api.RuleCollection InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of RuleCollection.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.RuleCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.RuleCollection.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of RuleCollection.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.RuleCollection.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `void MergeInContents(Autodesk.Navisworks.Api.RuleCollection source)`

Maintains existing list as far as possible while making sure that same rules exist as in source, with rules not in source disabled.

**Parameters:**
- `source` (Autodesk.Navisworks.Api.RuleCollection)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.RuleCollection.MergeInContents%28Autodesk.Navisworks.Api.RuleCollection%29)

#### `bool Remove(Autodesk.Navisworks.Api.Rule item)`

Removes the first occurrence of a specific item from the collection.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.Rule) — The object to remove from the collection.

**Returns:** `bool` — true if item was successfully removed from the collection; otherwise, false.
This method also returns false if item is not found in the original collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.RuleCollection.Remove%28Autodesk.Navisworks.Api.Rule%29)

#### `void RemoveAt(int index)`

Removes the IList`1 item at the specified index.

**Parameters:**
- `index` (int) — The zero-based index of the item to remove.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.RuleCollection.RemoveAt%28System.Int32%29)

#### `bool ValueEquals(Autodesk.Navisworks.Api.RuleCollection value)`

Performs comparison 'By Value'

**Parameters:**
- `value` (Autodesk.Navisworks.Api.RuleCollection)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.RuleCollection.ValueEquals%28Autodesk.Navisworks.Api.RuleCollection%29)

#### `static bool ValueEquals(Autodesk.Navisworks.Api.RuleCollection value1, Autodesk.Navisworks.Api.RuleCollection value2)`

Compares the two references 'By Value'

**Parameters:**
- `value1` (Autodesk.Navisworks.Api.RuleCollection)
- `value2` (Autodesk.Navisworks.Api.RuleCollection)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.RuleCollection.ValueEquals%28Autodesk.Navisworks.Api.RuleCollection%2CAutodesk.Navisworks.Api.RuleCollection%29)

### Properties
- `Count` (int, get) — Gets the number of elements contained in the collection.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Item` (Autodesk.Navisworks.Api.Rule, get/set) — Item property of RuleCollection.

## RuntimeLoaderException (class)

The exception that is thrown when trying to use a Navisworks control but the control can't find a Navisworks runtime to use. For example if no Navisworks product is installed.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.RuntimeLoaderException)

### Constructors
- `RuntimeLoaderException()` — Creates a RuntimeLoaderException
- `RuntimeLoaderException(string message)` — Creates a RuntimeLoaderException
- `RuntimeLoaderException(string message, System.Exception innerException)` — Creates a RuntimeLoaderException

## SavedItem (class)

Base class for all saved items.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.SavedItem)

### Methods
#### `bool CanTransform()`

Indicates whether the implementation of Transform() will do anything

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedItem.CanTransform)

#### `Autodesk.Navisworks.Api.SavedItem CreateCopy()`

Create a new copy of this item

**Returns:** `Autodesk.Navisworks.Api.SavedItem` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedItem.CreateCopy)

#### `Autodesk.Navisworks.Api.SavedItem CreateNewInstance()`

Create a new item of the same type as this item

**Returns:** `Autodesk.Navisworks.Api.SavedItem` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedItem.CreateNewInstance)

#### `Autodesk.Navisworks.Api.SavedItem CreateUniqueCopy()`

Create an unique copy of item

**Returns:** `Autodesk.Navisworks.Api.SavedItem` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedItem.CreateUniqueCopy)

#### `static Autodesk.Navisworks.Api.SavedItem InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of SavedItem.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.SavedItem` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedItem.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of SavedItem.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedItem.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `void MakeDisplayNameUnique(Autodesk.Navisworks.Api.GroupItem group)`

Makes a name unique within the immediate children on the group

**Parameters:**
- `group` (Autodesk.Navisworks.Api.GroupItem) — group to make unique name

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedItem.MakeDisplayNameUnique%28Autodesk.Navisworks.Api.GroupItem%29)

#### `void MakeDisplayNameUnique(Autodesk.Navisworks.Api.GroupItem group, string salt)`

Makes a name unique within the immediate children on the group

**Parameters:**
- `group` (Autodesk.Navisworks.Api.GroupItem) — group to make unique name
- `salt` (string) — salt string to use in making a name unique

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedItem.MakeDisplayNameUnique%28Autodesk.Navisworks.Api.GroupItem%2CSystem.String%29)

#### `void Transform(Autodesk.Navisworks.Api.Transform3D transform)`

Apply the given transform to the item

**Parameters:**
- `transform` (Autodesk.Navisworks.Api.Transform3D)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedItem.Transform%28Autodesk.Navisworks.Api.Transform3D%29)

### Properties
- `Comments` (Autodesk.Navisworks.Api.CommentCollection, get) — Comments about the item
- `DisplayName` (string, get/set) — Name for item as displayed in the GUI
- `Guid` (System.Guid, get/set) — Globally Unique Id for item
- `IsGroup` (bool, get) — True if this item is a GroupItem
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Parent` (Autodesk.Navisworks.Api.GroupItem, get) — Parent of this item. Null if item has no parent

## SavedItemChangedAction (enum)

Describes the action that caused a changed event for a SavedItemCollection-based Document part.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.SavedItemChangedAction)

### Values
- `Add` = `0` — An item was added or inserted
- `Remove` = `1` — An item was removed
- `Replace` = `2` — An item was replaced
- `Move` = `3` — An item was moved within the SavedItem hierarchy
- `Edit` = `4` — An item was edited within the SavedItem hierarchy
- `Reset` = `5` — The contents of the Document part have changed dramatically

## SavedItemChangedEventArgs (class)

Event arguments when a SavedItem hierarchy changes

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.SavedItemChangedEventArgs)

### Properties
- `Action` (Autodesk.Navisworks.Api.SavedItemChangedAction, get) — Gets the action that caused the event
- `EditedParts` (Autodesk.Navisworks.Api.SavedItemParts, get) — Gets the parts of the item that were edited (None if Action is not equal to Edit)
- `NewIndex` (int, get) — Gets the index within NewParent at which a Add, Replace, Edit or Move action occurred.
- `NewItem` (Autodesk.Navisworks.Api.SavedItem, get) — Gets the item effected by a Add, Replace, Edit or Move action.
- `NewParent` (Autodesk.Navisworks.Api.GroupItem, get) — Gets the new Parent of the item effected by a Add, Replace, Edit or Move action.
- `OldIndex` (int, get) — Gets the index within OldParent at which a Remove, Replace, Edit or Move action occurred.
- `OldItem` (Autodesk.Navisworks.Api.SavedItem, get) — Gets the old item effected by a Remove, Replace, Edit or Move action. For an Edit action only the parts of the item specified by EditedParts are valid.
- `OldParent` (Autodesk.Navisworks.Api.GroupItem, get) — Gets the old Parent of the item effected by a Remove, Replace, Edit or Move action.

## SavedItemCollection (class)

Collection of SavedItems that stores the children of a GroupItem.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.SavedItemCollection)

### Methods
#### `void Add(Autodesk.Navisworks.Api.SavedItem item)`

Adds item to the end of the collection.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SavedItem)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedItemCollection.Add%28Autodesk.Navisworks.Api.SavedItem%29)

#### `void AddRange(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.SavedItem> from)`

Add the items of the specified collection to the SavedItemCollection

**Parameters:**
- `from` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.SavedItem>) — The collection to add items from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedItemCollection.AddRange%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.SavedItem%7D%29)

#### `bool CanAdd(Autodesk.Navisworks.Api.SavedItem item)`

Determines whether item can be added to the collection.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SavedItem)

**Returns:** `bool` — false if item is ReadOnly or of an incompatible type

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedItemCollection.CanAdd%28Autodesk.Navisworks.Api.SavedItem%29)

#### `void Clear()`

Removes all items from the collection

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedItemCollection.Clear)

#### `bool Contains(Autodesk.Navisworks.Api.SavedItem item)`

Determines whether the SavedItemCollection contains a specific value.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SavedItem)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedItemCollection.Contains%28Autodesk.Navisworks.Api.SavedItem%29)

#### `void CopyFrom(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.SavedItem> from)`

Sets the contents of the collection to the items in from

**Parameters:**
- `from` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.SavedItem>) — The collection to copy contents from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedItemCollection.CopyFrom%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.SavedItem%7D%29)

#### `void CopyTo(Autodesk.Navisworks.Api.SavedItem[] array, int arrayIndex)`

Copies the elements of the ICollection`1 to an Array, starting at a particular Array index.

**Parameters:**
- `array` (Autodesk.Navisworks.Api.SavedItem[]) — The one-dimensional Array that is the destination of the elements copied from ICollection`1. The Array must have zero-based indexing.
- `arrayIndex` (int) — The zero-based index in array at which copying begins.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedItemCollection.CopyTo%28Autodesk.Navisworks.Api.SavedItem%5B%5D%2CSystem.Int32%29)

#### `void CopyTo(System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.SavedItem> to)`

Enumerates over the collection and copies the items into to

**Parameters:**
- `to` (System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.SavedItem>) — Collection to copy into

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedItemCollection.CopyTo%28System.Collections.Generic.ICollection%7BAutodesk.Navisworks.Api.SavedItem%7D%29)

#### `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.SavedItem> GetEnumerator()`

Returns an enumerator that can iterate through the collection.

**Returns:** `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.SavedItem>` — An IEnumerator that
can be used to iterate through the collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedItemCollection.GetEnumerator)

#### `int IndexOf(Autodesk.Navisworks.Api.SavedItem item)`

Determines the index of a specific item in the SavedItemCollection

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SavedItem)

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedItemCollection.IndexOf%28Autodesk.Navisworks.Api.SavedItem%29)

#### `int IndexOfDisplayName(string displayName)`

Determines the index of the first item in the collection with the specified DisplayName

**Parameters:**
- `displayName` (string)

**Returns:** `int` — Index of first item or -1 if none found

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedItemCollection.IndexOfDisplayName%28System.String%29)

#### `int IndexOfGuid(System.Guid value)`

Determines the index of the first item in the collection with the specified guid

**Parameters:**
- `value` (System.Guid)

**Returns:** `int` — Index of first item or -1 if none found

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedItemCollection.IndexOfGuid%28System.Guid%29)

#### `void Insert(int index, Autodesk.Navisworks.Api.SavedItem item)`

Inserts item into the collection at the specified index

**Parameters:**
- `index` (int)
- `item` (Autodesk.Navisworks.Api.SavedItem)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedItemCollection.Insert%28System.Int32%2CAutodesk.Navisworks.Api.SavedItem%29)

#### `System.Collections.IEnumerator InternalGetEnumerator()`

InternalGetEnumerator method of SavedItemCollection.

**Returns:** `System.Collections.IEnumerator` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedItemCollection.InternalGetEnumerator)

#### `bool Remove(Autodesk.Navisworks.Api.SavedItem item)`

Removes the first occurrence of a specific item from the collection.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SavedItem) — The object to remove from the collection.

**Returns:** `bool` — true if item was successfully removed from the collection; otherwise, false.
This method also returns false if item is not found in the original collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedItemCollection.Remove%28Autodesk.Navisworks.Api.SavedItem%29)

#### `void RemoveAt(int index)`

Removes the IList`1 item at the specified index.

**Parameters:**
- `index` (int) — The zero-based index of the item to remove.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedItemCollection.RemoveAt%28System.Int32%29)

#### `void Sort()`

Performs a case-insensitive lexicographical sort of the items in the collection. Folders sort to top (like Windows Explorer).

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedItemCollection.Sort)

#### `void SortDescendantsAndSelf()`

Performs a case-insensitive lexicographical sort of the items in the collection. Folders sort to top (like Windows Explorer).

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedItemCollection.SortDescendantsAndSelf)

### Properties
- `Count` (int, get) — Gets the number of elements contained in the collection.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the collection is read-only.
- `Item` (Autodesk.Navisworks.Api.SavedItem, get/set) — Item property of SavedItemCollection.

## SavedItemParts (enum)

Specifies parts of a SavedItem

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.SavedItemParts)

### Values
- `None` = `0` — No parts
- `DisplayName` = `1` — DisplayName property
- `Comments` = `2` — Comments
- `XData` = `4` — Additional data attached to a SavedItem
- `SubclassPart1` = `256` — A part specific to a particular subclass of SavedItem
- `SubclassPart2` = `512` — A part specific to a particular subclass of SavedItem
- `SubclassPart3` = `1024` — A part specific to a particular subclass of SavedItem
- `SubclassPart4` = `2048` — A part specific to a particular subclass of SavedItem
- `SubclassPart5` = `4096` — A part specific to a particular subclass of SavedItem
- `SubclassPart6` = `8192` — A part specific to a particular subclass of SavedItem
- `SubclassPart7` = `16384` — A part specific to a particular subclass of SavedItem
- `SubclassPart8` = `32768` — A part specific to a particular subclass of SavedItem
- `SubclassPart9` = `65536` — A part specific to a particular subclass of SavedItem
- `SubclassPart10` = `131072` — A part specific to a particular subclass of SavedItem
- `SubclassPart11` = `262144` — A part specific to a particular subclass of SavedItem
- `SubclassPart12` = `524288` — A part specific to a particular subclass of SavedItem
- `SubclassPart13` = `1048576` — A part specific to a particular subclass of SavedItem
- `SubclassPart14` = `2097152` — A part specific to a particular subclass of SavedItem
- `SubclassPart15` = `4194304` — A part specific to a particular subclass of SavedItem
- `SubclassPart16` = `8388608` — A part specific to a particular subclass of SavedItem
- `SubclassPart17` = `16777216` — A part specific to a particular subclass of SavedItem
- `SubclassPart18` = `33554432` — A part specific to a particular subclass of SavedItem
- `SubclassPart19` = `67108864` — A part specific to a particular subclass of SavedItem
- `SubclassPart20` = `134217728` — A part specific to a particular subclass of SavedItem
- `SubclassParts` = `268435200` — All parts specific to a particular subclass of SavedItem
- `All` = `268435207` — All parts

## SavedItemReference (class)

Represents a reference to a saved item. Immutable.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.SavedItemReference)

### Constructors
- `SavedItemReference()` — Creates an empty reference.
- `SavedItemReference(Autodesk.Navisworks.Api.SavedItemReference value)` — Creates a copy of a saved item reference.
- `SavedItemReference(string documentPartId, string savedItemId)` — Creates a reference to a saved item in a document part.

### Methods
#### `bool Equals(object obj)`

Determines whether the specified object and the current object refer to the same underlying native object. Returns false if either object has been disposed.

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedItemReference.Equals%28System.Object%29)

#### `int GetHashCode()`

Serves as a hash function for a particular type. GetHashCode is suitable for use in hashing algorithms and data structures like a hash table.

**Returns:** `int` — A hash code for the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedItemReference.GetHashCode)

#### `static Autodesk.Navisworks.Api.SavedItemReference InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of SavedItemReference.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.SavedItemReference` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedItemReference.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of SavedItemReference.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedItemReference.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `string ToString()`

Returns a String that represents the current Object.

**Returns:** `string` — A String that represents the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedItemReference.ToString)

### Properties
- `DisplayString` (string, get) — Display string for the reference.
- `DocumentPartId` (string, get) — Identifies the document part containing the item.
- `IsNull` (bool, get) — True if this doesn't reference any saved item.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `SavedItemId` (string, get) — Identifies a saved item within a particular document part.

## SavedViewpoint (class)

A named saved viewpoint.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.SavedViewpoint)

### Constructors
- `SavedViewpoint()` — Default constructor
- `SavedViewpoint(Autodesk.Navisworks.Api.Viewpoint viewpoint)` — Initialize SavedViewpoint with a Viewpoint

### Methods
#### `Autodesk.Navisworks.Api.Interop.LcOpRedlineList EditRedlines()`

EditRedlines method of SavedViewpoint.

**Returns:** `Autodesk.Navisworks.Api.Interop.LcOpRedlineList` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedViewpoint.EditRedlines)

#### `Autodesk.Navisworks.Api.AppearanceOverrides GetAppearanceOverrides()`

Gets material overrides for this SavedViewpoint.

**Returns:** `Autodesk.Navisworks.Api.AppearanceOverrides` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedViewpoint.GetAppearanceOverrides)

#### `Autodesk.Navisworks.Api.VisibilityOverrides GetVisibilityOverrides()`

Gets visibility overrides for this SavedViewpoint.

**Returns:** `Autodesk.Navisworks.Api.VisibilityOverrides` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedViewpoint.GetVisibilityOverrides)

#### `static Autodesk.Navisworks.Api.SavedViewpoint InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of SavedViewpoint.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.SavedViewpoint` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedViewpoint.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of SavedViewpoint.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedViewpoint.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

### Properties
- `ContainsAppearanceOverrides` (bool, get) — Determines if this SavedViewpoint contains material overrides.
- `ContainsVisibilityOverrides` (bool, get) — Determines if this SavedViewpoint contains visibility overrides.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Redlines` (Autodesk.Navisworks.Api.Interop.LcOpRedlineList, get) — Redlines property of SavedViewpoint.
- `Viewpoint` (Autodesk.Navisworks.Api.Viewpoint, get) — Acesses the Viewpoint ascociated with this object.

## SavedViewpointAnimation (class)

An animation consisting of SavedViewpoints and SavedViewpointAnimationCuts.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.SavedViewpointAnimation)

### Constructors
- `SavedViewpointAnimation()` — Default constructor

### Methods
#### `static Autodesk.Navisworks.Api.SavedViewpointAnimation InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of SavedViewpointAnimation.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.SavedViewpointAnimation` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedViewpointAnimation.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of SavedViewpointAnimation.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedViewpointAnimation.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

### Properties
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Loops` (bool, get/set) — Does the animation run on a continuous loop
- `Smoothing` (Autodesk.Navisworks.Api.SavedViewpointAnimationSmoothing, get/set) — Control the interpolation between frames

## SavedViewpointAnimationCut (class)

Simple cut from one frame in an animation to another.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.SavedViewpointAnimationCut)

### Constructors
- `SavedViewpointAnimationCut()` — Default constructor

### Methods
#### `static Autodesk.Navisworks.Api.SavedViewpointAnimationCut InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of SavedViewpointAnimationCut.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.SavedViewpointAnimationCut` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedViewpointAnimationCut.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of SavedViewpointAnimationCut.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SavedViewpointAnimationCut.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

### Properties
- `Delay` (double, get/set) — Time in seconds of the delay
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.

## SavedViewpointAnimationSmoothing (enum)

SavedViewpointAnimationSmoothing enum in Autodesk.Navisworks.Api.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.SavedViewpointAnimationSmoothing)

### Values
- `None` = `0`
- `Sync` = `1`
- `Spline` = `2`

## Search (class)

Defines a search that can be run to find ModelItems

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Search)

### Constructors
- `Search()` — Create a clear search
- `Search(Autodesk.Navisworks.Api.Search from)` — Create a copy of another search

### Methods
#### `bool CanMerge(Autodesk.Navisworks.Api.Search spec)`

True if the search conditions and selection critera are compatible between this and another search

**Parameters:**
- `spec` (Autodesk.Navisworks.Api.Search)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Search.CanMerge%28Autodesk.Navisworks.Api.Search%29)

#### `void Clear()`

Clears search back to it's default state

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Search.Clear)

#### `Autodesk.Navisworks.Api.ModelItemCollection FindAll(Autodesk.Navisworks.Api.Document document, bool reportProgress)`

Returns a ModelItemCollection that represents the items specified by this Search in document.

**Parameters:**
- `document` (Autodesk.Navisworks.Api.Document) — Document to search
- `reportProgress` (bool) — true if progress should be reported via normal Application progress reporting

**Returns:** `Autodesk.Navisworks.Api.ModelItemCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Search.FindAll%28Autodesk.Navisworks.Api.Document%2CSystem.Boolean%29)

#### `Autodesk.Navisworks.Api.ModelItemCollection FindAll(bool reportProgress)`

Returns a ModelItemCollection that represents the items specified by this Search in the document which it is associated with.

**Parameters:**
- `reportProgress` (bool) — true if progress should be reported via normal Application progress reporting

**Returns:** `Autodesk.Navisworks.Api.ModelItemCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Search.FindAll%28System.Boolean%29)

#### `Autodesk.Navisworks.Api.ModelItem FindFirst(Autodesk.Navisworks.Api.Document document, bool reportProgress)`

Returns the first ModelItem specified by this Search in document. Returns null if none found.

**Parameters:**
- `document` (Autodesk.Navisworks.Api.Document) — Document to search
- `reportProgress` (bool) — true if progress should be reported via normal Application progress reporting

**Returns:** `Autodesk.Navisworks.Api.ModelItem` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Search.FindFirst%28Autodesk.Navisworks.Api.Document%2CSystem.Boolean%29)

#### `Autodesk.Navisworks.Api.ModelItem FindFirst(bool reportProgress)`

Returns the first ModelItem specified by this Search in the document which it is associated with. Returns null if none found.

**Parameters:**
- `reportProgress` (bool) — true if progress should be reported via normal Application progress reporting

**Returns:** `Autodesk.Navisworks.Api.ModelItem` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Search.FindFirst%28System.Boolean%29)

#### `System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem> FindIncremental(Autodesk.Navisworks.Api.Document document, bool reportProgress)`

Returns an enumerable that steps through the items specified by this Search in document.

**Parameters:**
- `document` (Autodesk.Navisworks.Api.Document) — Document to search
- `reportProgress` (bool) — true if progress should be reported via normal Application progress reporting

**Returns:** `System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem>` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Search.FindIncremental%28Autodesk.Navisworks.Api.Document%2CSystem.Boolean%29)

#### `System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem> FindIncremental(bool reportProgress)`

Returns a an enumerable that steps through the items specified by this Search in the document which it is associated with.

**Parameters:**
- `reportProgress` (bool) — true if progress should be reported via normal Application progress reporting

**Returns:** `System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem>` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Search.FindIncremental%28System.Boolean%29)

#### `static Autodesk.Navisworks.Api.Search InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of Search.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Search` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Search.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of Search.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Search.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `void Merge(Autodesk.Navisworks.Api.Search search)`

Merges another Search into this Search, appending its conditions as a new group of conditions.

**Parameters:**
- `search` (Autodesk.Navisworks.Api.Search) — Search to merge.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Search.Merge%28Autodesk.Navisworks.Api.Search%29)

#### `bool TryMerge(Autodesk.Navisworks.Api.Search search)`

Attempts to merge another Search into this Search, combining conditions of search as a new group of conditions. Returns true if the merge is successful, and false if the Searches are incompatible e.g. different model selections searched.

**Parameters:**
- `search` (Autodesk.Navisworks.Api.Search) — Search to merge.

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Search.TryMerge%28Autodesk.Navisworks.Api.Search%29)

#### `bool ValueEquals(Autodesk.Navisworks.Api.Search value)`

Performs comparison 'By Value'

**Parameters:**
- `value` (Autodesk.Navisworks.Api.Search)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Search.ValueEquals%28Autodesk.Navisworks.Api.Search%29)

#### `static bool ValueEquals(Autodesk.Navisworks.Api.Search value1, Autodesk.Navisworks.Api.Search value2)`

Compares the two references 'By Value'

**Parameters:**
- `value1` (Autodesk.Navisworks.Api.Search)
- `value2` (Autodesk.Navisworks.Api.Search)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Search.ValueEquals%28Autodesk.Navisworks.Api.Search%2CAutodesk.Navisworks.Api.Search%29)

### Properties
- `IsClear` (bool, get) — True if selection is clear, and there are no search conditions
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Locations` (Autodesk.Navisworks.Api.SearchLocations, get/set) — Locations within Selection to search. Default is DescendantsAndSelf.
- `PruneBelowMatch` (bool, get/set) — When value is true, search ignores descendants of any matching model items. Default is true.
- `SearchConditions` (Autodesk.Navisworks.Api.SearchConditionCollection, get) — Search conditions that will be applied during search
- `Selection` (Autodesk.Navisworks.Api.Selection, get) — Selection that will be searched

## SearchCondition (class)

Defines a PropertyCategories based condition for use in searches. Immutable.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.SearchCondition)

### Constructors
- `SearchCondition(Autodesk.Navisworks.Api.NamedConstant categoryCombinedName, Autodesk.Navisworks.Api.NamedConstant propertyCombinedName, Autodesk.Navisworks.Api.SearchConditionOptions options, Autodesk.Navisworks.Api.SearchConditionComparison comparison, Autodesk.Navisworks.Api.VariantData value)` — Create a search condition

### Methods
#### `Autodesk.Navisworks.Api.SearchCondition CompareWith(Autodesk.Navisworks.Api.SearchConditionComparison comparison, Autodesk.Navisworks.Api.VariantData value)`

Creates condition that matches if item has property specified by this condition and compares successfully with value

**Parameters:**
- `comparison` (Autodesk.Navisworks.Api.SearchConditionComparison)
- `value` (Autodesk.Navisworks.Api.VariantData)

**Returns:** `Autodesk.Navisworks.Api.SearchCondition` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchCondition.CompareWith%28Autodesk.Navisworks.Api.SearchConditionComparison%2CAutodesk.Navisworks.Api.VariantData%29)

#### `Autodesk.Navisworks.Api.SearchCondition DisplayStringContains(string value)`

Creates condition that matches if item has property specified by this condition and has a DisplayString value that contains specified string

**Parameters:**
- `value` (string)

**Returns:** `Autodesk.Navisworks.Api.SearchCondition` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchCondition.DisplayStringContains%28System.String%29)

#### `Autodesk.Navisworks.Api.SearchCondition DisplayStringWildcard(string value)`

Creates condition that matches if item has property specified by this condition and has a DisplayString value that matches specified wildcard string

**Parameters:**
- `value` (string)

**Returns:** `Autodesk.Navisworks.Api.SearchCondition` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchCondition.DisplayStringWildcard%28System.String%29)

#### `Autodesk.Navisworks.Api.SearchCondition EqualValue(Autodesk.Navisworks.Api.VariantData value)`

Creates condition that matches if item has property specified by this condition and equal value

**Parameters:**
- `value` (Autodesk.Navisworks.Api.VariantData)

**Returns:** `Autodesk.Navisworks.Api.SearchCondition` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchCondition.EqualValue%28Autodesk.Navisworks.Api.VariantData%29)

#### `bool Equals(object obj)`

Determines whether the specified object and the current object refer to the same underlying native object. Returns false if either object has been disposed.

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchCondition.Equals%28System.Object%29)

#### `int GetHashCode()`

Serves as a hash function for a particular type. GetHashCode is suitable for use in hashing algorithms and data structures like a hash table.

**Returns:** `int` — A hash code for the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchCondition.GetHashCode)

#### `static Autodesk.Navisworks.Api.SearchCondition HasCategoryByCombinedName(Autodesk.Navisworks.Api.NamedConstant combinedName)`

Creates condition that matches if item has category with specified CombinedName

**Parameters:**
- `combinedName` (Autodesk.Navisworks.Api.NamedConstant)

**Returns:** `Autodesk.Navisworks.Api.SearchCondition` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchCondition.HasCategoryByCombinedName%28Autodesk.Navisworks.Api.NamedConstant%29)

#### `static Autodesk.Navisworks.Api.SearchCondition HasCategoryByDisplayName(string displayName)`

Creates condition that matches if item has category with specified DisplayName

**Parameters:**
- `displayName` (string)

**Returns:** `Autodesk.Navisworks.Api.SearchCondition` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchCondition.HasCategoryByDisplayName%28System.String%29)

#### `static Autodesk.Navisworks.Api.SearchCondition HasCategoryByName(string name)`

Creates condition that matches if item has category with specified Name

**Parameters:**
- `name` (string)

**Returns:** `Autodesk.Navisworks.Api.SearchCondition` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchCondition.HasCategoryByName%28System.String%29)

#### `static Autodesk.Navisworks.Api.SearchCondition HasPropertyByCombinedName(Autodesk.Navisworks.Api.NamedConstant categoryCombinedName, Autodesk.Navisworks.Api.NamedConstant propertyCombinedName)`

Creates condition that matches if item has property in category with specified CombinedNames

**Parameters:**
- `categoryCombinedName` (Autodesk.Navisworks.Api.NamedConstant)
- `propertyCombinedName` (Autodesk.Navisworks.Api.NamedConstant)

**Returns:** `Autodesk.Navisworks.Api.SearchCondition` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchCondition.HasPropertyByCombinedName%28Autodesk.Navisworks.Api.NamedConstant%2CAutodesk.Navisworks.Api.NamedConstant%29)

#### `static Autodesk.Navisworks.Api.SearchCondition HasPropertyByDisplayName(string categoryDisplayName, string propertyDisplayName)`

Creates condition that matches if item has property in category with specified DisplayNames

**Parameters:**
- `categoryDisplayName` (string)
- `propertyDisplayName` (string)

**Returns:** `Autodesk.Navisworks.Api.SearchCondition` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchCondition.HasPropertyByDisplayName%28System.String%2CSystem.String%29)

#### `static Autodesk.Navisworks.Api.SearchCondition HasPropertyByName(string categoryName, string propertyName)`

Creates condition that matches if item has property in category with specified Names

**Parameters:**
- `categoryName` (string)
- `propertyName` (string)

**Returns:** `Autodesk.Navisworks.Api.SearchCondition` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchCondition.HasPropertyByName%28System.String%2CSystem.String%29)

#### `Autodesk.Navisworks.Api.SearchCondition IgnoreStringValueAccents()`

Creates condition that matches if item matches this condition while ignoring accents during string value comparisons

**Returns:** `Autodesk.Navisworks.Api.SearchCondition` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchCondition.IgnoreStringValueAccents)

#### `Autodesk.Navisworks.Api.SearchCondition IgnoreStringValueCase()`

Creates condition that matches if item matches this condition while ignoring case during string value comparisons

**Returns:** `Autodesk.Navisworks.Api.SearchCondition` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchCondition.IgnoreStringValueCase)

#### `Autodesk.Navisworks.Api.SearchCondition IgnoreStringValueCharWidths()`

Creates condition that matches if item matches this condition while ignoring character widths during string value comparisons

**Returns:** `Autodesk.Navisworks.Api.SearchCondition` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchCondition.IgnoreStringValueCharWidths)

#### `static Autodesk.Navisworks.Api.SearchCondition InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of SearchCondition.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.SearchCondition` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchCondition.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of SearchCondition.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchCondition.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `Autodesk.Navisworks.Api.SearchCondition Negate()`

Creates condition that matches if item does not match this condition

**Returns:** `Autodesk.Navisworks.Api.SearchCondition` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchCondition.Negate)

#### `Autodesk.Navisworks.Api.SearchCondition SameType(Autodesk.Navisworks.Api.VariantDataType type)`

Creates condition that matches if item has property specified by this condition and value of specified type

**Parameters:**
- `type` (Autodesk.Navisworks.Api.VariantDataType)

**Returns:** `Autodesk.Navisworks.Api.SearchCondition` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchCondition.SameType%28Autodesk.Navisworks.Api.VariantDataType%29)

#### `Autodesk.Navisworks.Api.SearchCondition StartGroup()`

Creates condition that is same as this condition with StartGroup option set

**Returns:** `Autodesk.Navisworks.Api.SearchCondition` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchCondition.StartGroup)

#### `string ToString()`

Returns a String that represents the current Object.

**Returns:** `string` — A String that represents the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchCondition.ToString)

### Properties
- `CategoryCombinedName` (Autodesk.Navisworks.Api.NamedConstant, get) — Combined name for category to compare against
- `Comparison` (Autodesk.Navisworks.Api.SearchConditionComparison, get) — Comparison operator to use when evaluating condition
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Options` (Autodesk.Navisworks.Api.SearchConditionOptions, get) — Options to use when evaluating condition
- `PropertyCombinedName` (Autodesk.Navisworks.Api.NamedConstant, get) — Combined name for property to compare against
- `Value` (Autodesk.Navisworks.Api.VariantData, get) — Value to compare against when evaluating condition

## SearchConditionCollection (class)

A collection of SearchConditions

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.SearchConditionCollection)

### Constructors
- `SearchConditionCollection()` — Create an empty collection
- `SearchConditionCollection(Autodesk.Navisworks.Api.SearchConditionCollection from)` — Create a collection that is a copy of from

### Methods
#### `void Add(Autodesk.Navisworks.Api.SearchCondition item)`

Adds item to the end of the collection.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SearchCondition)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchConditionCollection.Add%28Autodesk.Navisworks.Api.SearchCondition%29)

#### `void AddGroup(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.SearchCondition> from)`

Add the items of the specified collection to the SearchConditionCollection as a new group of conditions. The combined set of SearchConditions will match if any group within the set matches.

**Parameters:**
- `from` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.SearchCondition>) — The collection to add the items from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchConditionCollection.AddGroup%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.SearchCondition%7D%29)

#### `void AddRange(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.SearchCondition> from)`

Add the items of the specified collection to the SavedItemCollection

**Parameters:**
- `from` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.SearchCondition>) — The collection to add items from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchConditionCollection.AddRange%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.SearchCondition%7D%29)

#### `void Clear()`

Removes all items from the collection

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchConditionCollection.Clear)

#### `bool Contains(Autodesk.Navisworks.Api.SearchCondition item)`

Determines whether the SavedItemCollection contains a specific value.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SearchCondition)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchConditionCollection.Contains%28Autodesk.Navisworks.Api.SearchCondition%29)

#### `void CopyFrom(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.SearchCondition> from)`

Sets the contents of the collection to the items in from

**Parameters:**
- `from` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.SearchCondition>) — The collection to copy contents from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchConditionCollection.CopyFrom%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.SearchCondition%7D%29)

#### `void CopyTo(Autodesk.Navisworks.Api.SearchCondition[] array, int arrayIndex)`

Copies the elements of the ICollection`1 to an Array, starting at a particular Array index.

**Parameters:**
- `array` (Autodesk.Navisworks.Api.SearchCondition[]) — The one-dimensional Array that is the destination of the elements copied from ICollection`1. The Array must have zero-based indexing.
- `arrayIndex` (int) — The zero-based index in array at which copying begins.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchConditionCollection.CopyTo%28Autodesk.Navisworks.Api.SearchCondition%5B%5D%2CSystem.Int32%29)

#### `void CopyTo(System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.SearchCondition> to)`

Enumerates over the collection and copies the items into to

**Parameters:**
- `to` (System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.SearchCondition>) — Collection to copy into

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchConditionCollection.CopyTo%28System.Collections.Generic.ICollection%7BAutodesk.Navisworks.Api.SearchCondition%7D%29)

#### `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.SearchCondition> GetEnumerator()`

Returns an enumerator that can iterate through the collection.

**Returns:** `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.SearchCondition>` — An IEnumerator that can be used to iterate through the collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchConditionCollection.GetEnumerator)

#### `int IndexOf(Autodesk.Navisworks.Api.SearchCondition item)`

Determines the index of a specific item in the Collection

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SearchCondition)

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchConditionCollection.IndexOf%28Autodesk.Navisworks.Api.SearchCondition%29)

#### `void Insert(int index, Autodesk.Navisworks.Api.SearchCondition item)`

Inserts item into the collection at the specified index

**Parameters:**
- `index` (int)
- `item` (Autodesk.Navisworks.Api.SearchCondition)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchConditionCollection.Insert%28System.Int32%2CAutodesk.Navisworks.Api.SearchCondition%29)

#### `static Autodesk.Navisworks.Api.SearchConditionCollection InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of SearchConditionCollection.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.SearchConditionCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchConditionCollection.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of SearchConditionCollection.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchConditionCollection.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `bool Remove(Autodesk.Navisworks.Api.SearchCondition item)`

Removes the first occurrence of a specific item from the collection.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SearchCondition) — The object to remove from the collection.

**Returns:** `bool` — true if item was successfully removed from the collection; otherwise, false.
This method also returns false if item is not found in the original collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchConditionCollection.Remove%28Autodesk.Navisworks.Api.SearchCondition%29)

#### `void RemoveAt(int index)`

Removes the IList`1 item at the specified index.

**Parameters:**
- `index` (int) — The zero-based index of the item to remove.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SearchConditionCollection.RemoveAt%28System.Int32%29)

### Properties
- `Count` (int, get) — Gets the number of elements contained in the collection.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Item` (Autodesk.Navisworks.Api.SearchCondition, get/set) — Item property of SearchConditionCollection.

## SearchConditionComparison (enum)

Specifies comparison operator used to see whether a SearchCondition will match

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.SearchConditionComparison)

### Values
- `None` = `0` — Never matches
- `HasCategory` = `1` — Matches if item has specified category
- `NotHasCategory` = `2` — Matches if item does not have specified category
- `HasProperty` = `3` — Matches if item has the specified property in the specified category
- `NotHasProperty` = `4` — Matches if item does not have the specified property in the specified category
- `SameType` = `5` — Matches if specified property exists and value has the same VariantDataType
- `Equal` = `6` — Matches if specified property exists and has the same value
- `NotEqual` = `7` — Matches if specified property exists and does not have the same value
- `NumericLessThan` = `8` — Matches if specified property exists, has the same numeric VariantDataType (Int32, AnyDouble, DateTime) and property value less than condition value
- `NumericLessThanOrEqual` = `9` — Matches if specified property exists, has the same numeric VariantDataType (Int32, AnyDouble, DateTime) and property value less than or equal to condition value
- `NumericGreaterThanOrEqual` = `10` — Matches if specified property exists, has the same numeric VariantDataType (Int32, AnyDouble, DateTime) and property value greater than or equal to condition value
- `NumericGreaterThan` = `11` — Matches if specified property exists, has the same numeric VariantDataType (Int32, AnyDouble, DateTime) and property value greater than condition value
- `DisplayStringContains` = `12` — Matches if specified property exists, is of DisplayString type and contains the display string in condition value
- `DisplayStringWildcard` = `13` — Matches if specified property exists, is of DisplayString type and matches the wildcard display string in condition value
- `DateTimeWithinDay` = `14` — Matches if specified property exists, is of DateTime type and is within one day prior to the DateTime in condition value
- `DateTimeWithinWeek` = `15` — Matches if specified property exists, is of DateTime type and is within one week prior to the DateTime in condition value

## SearchConditionOptions (enum)

Options that modify SearchCondition matching behavior

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.SearchConditionOptions)

### Values
- `None` = `0` — No additional options
- `IgnoreCategoryDisplayName` = `1` — Do not compare DisplayName when looking for matching category
- `IgnoreCategoryName` = `2` — Do not compare Name when looking for matching category
- `IgnorePropertyDisplayName` = `4` — Do not compare DisplayName when looking for matching property
- `IgnoreDisplayNames` = `5` — Do not compare DisplayName when looking for matching category or property
- `IgnorePropertyName` = `8` — Do not compare Name when looking for matching property
- `IgnoreNames` = `10` — Do not compare Name when looking for matching category or property
- `IgnoreDisplayStringValueCase` = `16` — Ignore case when comparing DisplayString values
- `NegateCondition` = `32` — Negate the sense of the condition
- `StartGroup` = `64` — When used in a SearchConditionCollection, marks the start of a group of conditions. Collection matches if any group of conditions matches.
- `IgnoreDisplayStringValueAccents` = `128` — Ignore accents when comparing DisplayString values
- `IgnoreDisplayStringCharWidths` = `256` — Ignore character width when comparing DisplayString values

## SearchLocations (enum)

Which items within a Search Selection should be searched

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.SearchLocations)

### Values
- `None` = `0` — No items
- `Self` = `1` — Direct contents of the selection
- `Descendants` = `2` — Descendants of the items in the selection
- `DescendantsAndSelf` = `3` — Direct contents of the selection and their descendants

## Selection (class)

Represents a selection as either an explicit collection of ModelItems or as a collection of SelectionSources that will generate a selection on demand.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Selection)

### Constructors
- `Selection()` — Create a clear selection
- `Selection(Autodesk.Navisworks.Api.ModelItemCollection from)` — Create a selection as a explicit collection of ModelItems
- `Selection(Autodesk.Navisworks.Api.Selection from)` — Create a copy of another selection
- `Selection(Autodesk.Navisworks.Api.SelectionSourceCollection from)` — Create a selection as a collection of SelectionSources

### Methods
#### `void Clear()`

Reset selection to the default state, explicit and nothing selected

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Selection.Clear)

#### `void CopyFrom(Autodesk.Navisworks.Api.ModelItemCollection from)`

Initialise as explicit selection with a copy of items

**Parameters:**
- `from` (Autodesk.Navisworks.Api.ModelItemCollection)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Selection.CopyFrom%28Autodesk.Navisworks.Api.ModelItemCollection%29)

#### `void CopyFrom(Autodesk.Navisworks.Api.Selection from)`

Initialise selection with a copy of another selection

**Parameters:**
- `from` (Autodesk.Navisworks.Api.Selection)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Selection.CopyFrom%28Autodesk.Navisworks.Api.Selection%29)

#### `void CopyFrom(Autodesk.Navisworks.Api.SelectionSourceCollection from)`

Initialise as collection of SelectionSources

**Parameters:**
- `from` (Autodesk.Navisworks.Api.SelectionSourceCollection)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Selection.CopyFrom%28Autodesk.Navisworks.Api.SelectionSourceCollection%29)

#### `void CopyFrom(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem> from)`

Initialise as explicit selection with items in from

**Parameters:**
- `from` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem>) — The collection to copy contents from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Selection.CopyFrom%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.ModelItem%7D%29)

#### `Autodesk.Navisworks.Api.ModelItemCollection GetSelectedItems()`

Returns a ModelItemCollection that represents the items specified by this Selection in the document which it is associated with.

**Returns:** `Autodesk.Navisworks.Api.ModelItemCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Selection.GetSelectedItems)

#### `Autodesk.Navisworks.Api.ModelItemCollection GetSelectedItems(Autodesk.Navisworks.Api.Document document)`

Returns a ModelItemCollection that represents the items specified by this Selection in document.

**Parameters:**
- `document` (Autodesk.Navisworks.Api.Document)

**Returns:** `Autodesk.Navisworks.Api.ModelItemCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Selection.GetSelectedItems%28Autodesk.Navisworks.Api.Document%29)

#### `static Autodesk.Navisworks.Api.Selection InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of Selection.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Selection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Selection.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of Selection.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Selection.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `void SelectAll()`

Initialises selection with a selection source that selects everything in document when resolved

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Selection.SelectAll)

#### `bool ValueEquals(Autodesk.Navisworks.Api.Selection value)`

Performs comparison 'By Value'

**Parameters:**
- `value` (Autodesk.Navisworks.Api.Selection)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Selection.ValueEquals%28Autodesk.Navisworks.Api.Selection%29)

#### `static bool ValueEquals(Autodesk.Navisworks.Api.Selection value1, Autodesk.Navisworks.Api.Selection value2)`

Compares the two references 'By Value'

**Parameters:**
- `value1` (Autodesk.Navisworks.Api.Selection)
- `value2` (Autodesk.Navisworks.Api.Selection)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Selection.ValueEquals%28Autodesk.Navisworks.Api.Selection%2CAutodesk.Navisworks.Api.Selection%29)

### Properties
- `ExplicitSelection` (Autodesk.Navisworks.Api.ModelItemCollection, get) — Selection as explicit collection of ModelItems
- `HasExplicitSelection` (bool, get) — Does this selection contain an explicit collection of ModelItems ?
- `HasSelectionSources` (bool, get) — Does this selection contain a collection of SelectionSources ?
- `IsClear` (bool, get) — Is this the default selection (explicit and nothing selected) ?
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `SelectionSources` (Autodesk.Navisworks.Api.SelectionSourceCollection, get) — Selection as collection of SelectionSources

## SelectionBehavior (enum)

Determines which ModelItem is selected when using the Select tool to pick an object

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.SelectionBehavior)

### Values
- `Model` = `0` — First item containing object picked that HasModel (File)
- `Layer` = `1` — First item containing object picked that IsLayer
- `FirstObject` = `2` — First item containing object picked that is child of IsLayer item
- `LastUnique` = `3` — Last item containing object picked that has Instances count of 1
- `Geometry` = `4` — Last item containing object picked that HasGeometry
- `UniqueGeometryParent` = `5` — First item that only contains object picked
- `NoSelectionChange` = `6` — Picking has no effect (selection does not change)
- `LastComposite` = `7` — Last item containing object picked that IsComposite
- `Custom` = `8` — Custom selection behaviour, defined by the application

## SelectionSet (class)

SavedItem containing a saved ModelItemCollection or Search

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.SelectionSet)

### Constructors
- `SelectionSet()` — Create a SelectionSet with empty ExplicitModelItems
- `SelectionSet(Autodesk.Navisworks.Api.ModelItemCollection items)` — Create a SelectionSet with ExplicitModelItems
- `SelectionSet(Autodesk.Navisworks.Api.Search search)` — Create a SelectionSet with a Search
- `SelectionSet(Autodesk.Navisworks.Api.SelectionSet value)` — Create a copy of another SelectionSet

### Methods
#### `void CopyFrom(Autodesk.Navisworks.Api.ModelItemCollection items)`

Initialize SelectionSet with copy of ExplicitModelItems

**Parameters:**
- `items` (Autodesk.Navisworks.Api.ModelItemCollection)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSet.CopyFrom%28Autodesk.Navisworks.Api.ModelItemCollection%29)

#### `void CopyFrom(Autodesk.Navisworks.Api.Search search)`

Initialize SelectionSet with copy of a Search

**Parameters:**
- `search` (Autodesk.Navisworks.Api.Search)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSet.CopyFrom%28Autodesk.Navisworks.Api.Search%29)

#### `void CopyFrom(Autodesk.Navisworks.Api.SelectionSet value)`

Initialize SelectionSet with copy of another SelectionSet

**Parameters:**
- `value` (Autodesk.Navisworks.Api.SelectionSet)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSet.CopyFrom%28Autodesk.Navisworks.Api.SelectionSet%29)

#### `void CopyFrom(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem> from)`

Initialise as explicit ModelItems with items in from

**Parameters:**
- `from` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem>) — The collection to copy contents from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSet.CopyFrom%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.ModelItem%7D%29)

#### `Autodesk.Navisworks.Api.ModelItemCollection GetSelectedItems()`

Returns a ModelItemCollection that represents the items specified by this SelectionSet in the document which it is associated with.

**Returns:** `Autodesk.Navisworks.Api.ModelItemCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSet.GetSelectedItems)

#### `Autodesk.Navisworks.Api.ModelItemCollection GetSelectedItems(Autodesk.Navisworks.Api.Document document)`

Returns a ModelItemCollection that represents the items specified by this SelectionSet in document.

**Parameters:**
- `document` (Autodesk.Navisworks.Api.Document) — Document to search (if required)

**Returns:** `Autodesk.Navisworks.Api.ModelItemCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSet.GetSelectedItems%28Autodesk.Navisworks.Api.Document%29)

#### `static Autodesk.Navisworks.Api.SelectionSet InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of SelectionSet.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.SelectionSet` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSet.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of SelectionSet.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSet.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `bool ValueEquals(Autodesk.Navisworks.Api.SelectionSet value)`

Performs comparison 'By Value'

**Parameters:**
- `value` (Autodesk.Navisworks.Api.SelectionSet)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSet.ValueEquals%28Autodesk.Navisworks.Api.SelectionSet%29)

#### `static bool ValueEquals(Autodesk.Navisworks.Api.SelectionSet value1, Autodesk.Navisworks.Api.SelectionSet value2)`

Compares the two references 'By Value'

**Parameters:**
- `value1` (Autodesk.Navisworks.Api.SelectionSet)
- `value2` (Autodesk.Navisworks.Api.SelectionSet)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSet.ValueEquals%28Autodesk.Navisworks.Api.SelectionSet%2CAutodesk.Navisworks.Api.SelectionSet%29)

### Properties
- `ExplicitModelItems` (Autodesk.Navisworks.Api.ModelItemCollection, get) — The explicit ModelItems stored in the selection set
- `HasExplicitModelItems` (bool, get) — Does the SelectionSet contain an explicit set of ModelItems ?
- `HasSearch` (bool, get) — Does the SelectionSet contain a Search ?
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Search` (Autodesk.Navisworks.Api.Search, get) — The Search stored in the selection set

## SelectionSource (class)

Represents a reference to a source that can generate a selection on demand. Immutable.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.SelectionSource)

### Methods
#### `bool Equals(object obj)`

Determines whether the specified object and the current object refer to the same underlying native object. Returns false if either object has been disposed.

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSource.Equals%28System.Object%29)

#### `int GetHashCode()`

Serves as a hash function for a particular type. GetHashCode is suitable for use in hashing algorithms and data structures like a hash table.

**Returns:** `int` — A hash code for the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSource.GetHashCode)

#### `string GetRoot()`

GetRoot method of SelectionSource.

**Returns:** `string` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSource.GetRoot)

#### `static Autodesk.Navisworks.Api.SelectionSource InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of SelectionSource.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.SelectionSource` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSource.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of SelectionSource.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSource.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `string ToString()`

Returns a String that represents the current Object.

**Returns:** `string` — A String that represents the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSource.ToString)

#### `Autodesk.Navisworks.Api.Search TryGetSearch(Autodesk.Navisworks.Api.Document document)`

Returns a Search that will generate the selected items specified by the selection source

**Parameters:**
- `document` (Autodesk.Navisworks.Api.Document)

**Returns:** `Autodesk.Navisworks.Api.Search` — null if source could not be resolved or source does not represent selection as a Search

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSource.TryGetSearch%28Autodesk.Navisworks.Api.Document%29)

#### `Autodesk.Navisworks.Api.ModelItemCollection TryGetSelectedItems(Autodesk.Navisworks.Api.Document document)`

Returns a ModelItemCollection that represents the selected items specified by the selection source

**Parameters:**
- `document` (Autodesk.Navisworks.Api.Document)

**Returns:** `Autodesk.Navisworks.Api.ModelItemCollection` — null if source could not be resolved

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSource.TryGetSelectedItems%28Autodesk.Navisworks.Api.Document%29)

### Properties
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.

## SelectionSourceCollection (class)

A collection of SelectionSources

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.SelectionSourceCollection)

### Constructors
- `SelectionSourceCollection()` — Create an empty collection
- `SelectionSourceCollection(Autodesk.Navisworks.Api.SelectionSourceCollection from)` — Create a collection that is a copy of from

### Methods
#### `void Add(Autodesk.Navisworks.Api.SelectionSource item)`

Adds an item to the ICollection`1.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SelectionSource) — The object to add to the ICollection`1.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSourceCollection.Add%28Autodesk.Navisworks.Api.SelectionSource%29)

#### `void AddRange(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.SelectionSource> from)`

Add the items of the specified collection to the SelectionSourceCollection

**Parameters:**
- `from` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.SelectionSource>) — The collection to add items from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSourceCollection.AddRange%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.SelectionSource%7D%29)

#### `void Clear()`

Removes all items from the ICollection`1.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSourceCollection.Clear)

#### `bool Contains(Autodesk.Navisworks.Api.SelectionSource item)`

Determines whether the SelectionSourceCollection contains a specific value.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SelectionSource)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSourceCollection.Contains%28Autodesk.Navisworks.Api.SelectionSource%29)

#### `void CopyFrom(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.SelectionSource> from)`

Sets the contents of the collection to the items in from

**Parameters:**
- `from` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.SelectionSource>) — The collection to copy contents from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSourceCollection.CopyFrom%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.SelectionSource%7D%29)

#### `void CopyTo(Autodesk.Navisworks.Api.SelectionSource[] array, int arrayIndex)`

Copies the elements of the ICollection`1 to an Array, starting at a particular Array index.

**Parameters:**
- `array` (Autodesk.Navisworks.Api.SelectionSource[]) — The one-dimensional Array that is the destination of the elements copied from ICollection`1. The Array must have zero-based indexing.
- `arrayIndex` (int) — The zero-based index in array at which copying begins.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSourceCollection.CopyTo%28Autodesk.Navisworks.Api.SelectionSource%5B%5D%2CSystem.Int32%29)

#### `void CopyTo(System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.SelectionSource> to)`

Copies the contents of the collection into to

**Parameters:**
- `to` (System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.SelectionSource>) — The collection to copy contents into

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSourceCollection.CopyTo%28System.Collections.Generic.ICollection%7BAutodesk.Navisworks.Api.SelectionSource%7D%29)

#### `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.SelectionSource> GetEnumerator()`

Returns an enumerator that can iterate through the collection.

**Returns:** `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.SelectionSource>` — An IEnumerator that can be used to iterate through the collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSourceCollection.GetEnumerator)

#### `int IndexOf(Autodesk.Navisworks.Api.SelectionSource item)`

Determines the index of a specific item in the SelectionSourceCollection

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SelectionSource)

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSourceCollection.IndexOf%28Autodesk.Navisworks.Api.SelectionSource%29)

#### `void Insert(int index, Autodesk.Navisworks.Api.SelectionSource item)`

Inserts an item to the IList`1 at the specified index.

**Parameters:**
- `index` (int) — The zero-based index at which item should be inserted.
- `item` (Autodesk.Navisworks.Api.SelectionSource) — The object to insert into the IList`1.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSourceCollection.Insert%28System.Int32%2CAutodesk.Navisworks.Api.SelectionSource%29)

#### `static Autodesk.Navisworks.Api.SelectionSourceCollection InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of SelectionSourceCollection.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.SelectionSourceCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSourceCollection.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of SelectionSourceCollection.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSourceCollection.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `System.Collections.IEnumerator InternalGetEnumerator()`

InternalGetEnumerator method of SelectionSourceCollection.

**Returns:** `System.Collections.IEnumerator` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSourceCollection.InternalGetEnumerator)

#### `bool Remove(Autodesk.Navisworks.Api.SelectionSource item)`

Removes the first occurrence of a specific item from the collection.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SelectionSource) — The object to remove from the collection.

**Returns:** `bool` — true if item was successfully removed from the collection; otherwise, false. This method also returns false if item is not found in the original collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSourceCollection.Remove%28Autodesk.Navisworks.Api.SelectionSource%29)

#### `void RemoveAt(int index)`

Removes the IList`1 item at the specified index.

**Parameters:**
- `index` (int) — The zero-based index of the item to remove.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSourceCollection.RemoveAt%28System.Int32%29)

#### `bool ValueEquals(Autodesk.Navisworks.Api.SelectionSourceCollection value)`

Performs comparison 'By Value'

**Parameters:**
- `value` (Autodesk.Navisworks.Api.SelectionSourceCollection)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSourceCollection.ValueEquals%28Autodesk.Navisworks.Api.SelectionSourceCollection%29)

#### `static bool ValueEquals(Autodesk.Navisworks.Api.SelectionSourceCollection value1, Autodesk.Navisworks.Api.SelectionSourceCollection value2)`

Compares the two references 'By Value'

**Parameters:**
- `value1` (Autodesk.Navisworks.Api.SelectionSourceCollection)
- `value2` (Autodesk.Navisworks.Api.SelectionSourceCollection)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SelectionSourceCollection.ValueEquals%28Autodesk.Navisworks.Api.SelectionSourceCollection%2CAutodesk.Navisworks.Api.SelectionSourceCollection%29)

### Properties
- `Count` (int, get) — Gets the number of elements contained in the collection.
- `InternalIsReadOnly` (bool, get) — InternalIsReadOnly property of SelectionSourceCollection.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Item` (Autodesk.Navisworks.Api.SelectionSource, get/set) — Item property of SelectionSourceCollection.

## ServerType (enum)

Sign In Server Type.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ServerType)

### Values
- `Production` = `0` — Production.
- `Staging` = `1` — Staging.
- `Development` = `2` — Development.

## SheetInfo (class)

Sheet information. Often used to represent a sheet in a multi-sheet file

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.SheetInfo)

### Methods
#### `bool CheckIsConvertedForLoad()`

Checks whether this sheet has been previously converted and will load quickly if required

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SheetInfo.CheckIsConvertedForLoad)

#### `bool CheckIsConvertedForSearch()`

Checks whether this sheet has been previously converted and can be searched without loading

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SheetInfo.CheckIsConvertedForSearch)

#### `bool ContainsMatchingItem(Autodesk.Navisworks.Api.ModelItem item)`

Checks whether this sheet contains matching modelItem.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.ModelItem) — ModelItem to check

**Returns:** `bool` — Returns false if this sheet does not contain matching ModelItem or this sheet hasn't been previously converted

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SheetInfo.ContainsMatchingItem%28Autodesk.Navisworks.Api.ModelItem%29)

#### `Autodesk.Navisworks.Api.SheetInfo CreateUniqueCopy()`

Return a copy with unique sheet id

**Returns:** `Autodesk.Navisworks.Api.SheetInfo` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SheetInfo.CreateUniqueCopy)

#### `static Autodesk.Navisworks.Api.SheetInfo InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of SheetInfo.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.SheetInfo` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SheetInfo.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of SheetInfo.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.SheetInfo.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

### Properties
- `Id` (string, get) — Uniquely identifies a specific sheet within a file
- `IsLoaded` (bool, get) — Get whether the sheet has been loaded during the current Navisworks session
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `PropertyCategories` (Autodesk.Navisworks.Api.InfoPropertyCategoryCollection, get) — Get the property category collection
- `SheetType` (Autodesk.Navisworks.Api.SheetType, get) — Get type of sheet
- `SourceGuid` (System.Guid, get) — Get original Guid that this sheet was created with. Guid may be modified to ensure all sheets have unique Guids. If the same sheet is added to a document twice, each sheet has the same SourceGuid but different Guids.
- `Thumbnail` (string, get) — Get the sheet thumbnail file name

## SheetType (enum)

Determines which type the sheet is

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.SheetType)

### Values
- `Sheet3D` = `0` — Sheet contains a 3D model
- `Sheet2D` = `1` — Sheet contains a 2D model (plot)

## SimulationType (enum)

Type of simulation being run.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.SimulationType)

### Values
- `None` = `0` — No simulation
- `Timeliner` = `1` — Timeliner simulation
- `Animator` = `2` — Animator simulation

## StereoBehaviorMode (enum)

The stereo mode in use in stereo viewing options.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.StereoBehaviorMode)

### Values
- `BehindScreen` = `0` — Objects look as if they're behind the screen
- `AroundFocalPoint` = `1` — Focal point is on screen, objects can be in front and behind

## TessellationWindingRule (enum)

Tessellation winding rule

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.TessellationWindingRule)

### Values
- `Odd` = `100130` — Winding rule odd
- `Nonzero` = `100131` — Winding rule non-zero
- `Positive` = `100132` — Winding rule positive
- `Negative` = `100133` — Winding rule negative
- `AbsGEqTwo` = `100134` — Winding rule Abs_GEq_Two

## TextExtents2D (class)

Extents of a 2D text object. Immutable.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.TextExtents2D)

### Constructors
- `TextExtents2D()` — Creates an empty extents object.
- `TextExtents2D(int left, int right, int bottom, int top, int originX, int originY)` — Creates a new extents object.

### Methods
#### `bool Equals(object obj)`

Determines whether the specified object and the current object refer to the same underlying native object. Returns false if either object has been disposed.

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.TextExtents2D.Equals%28System.Object%29)

#### `int GetHashCode()`

Serves as a hash function for a particular type. GetHashCode is suitable for use in hashing algorithms and data structures like a hash table.

**Returns:** `int` — A hash code for the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.TextExtents2D.GetHashCode)

#### `static Autodesk.Navisworks.Api.TextExtents2D InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of TextExtents2D.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.TextExtents2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.TextExtents2D.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of TextExtents2D.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.TextExtents2D.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `string ToString()`

Returns a String that represents the current Object.

**Returns:** `string` — A String that represents the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.TextExtents2D.ToString)

### Properties
- `Bounds` (Autodesk.Navisworks.Api.BoundingBox2D, get) — Bounding box
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Origin` (Autodesk.Navisworks.Api.Point2D, get) — Origin

## TextFontInfo (class)

Information about a text font. Immutable.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.TextFontInfo)

### Constructors
- `TextFontInfo()` — Creates an empty font info object.
- `TextFontInfo(string typeface, int pointSize, int weight, bool italic, bool strikeout)` — Creates a new font info object.

### Methods
#### `bool Equals(object obj)`

Determines whether the specified object and the current object refer to the same underlying native object. Returns false if either object has been disposed.

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.TextFontInfo.Equals%28System.Object%29)

#### `int GetHashCode()`

Serves as a hash function for a particular type. GetHashCode is suitable for use in hashing algorithms and data structures like a hash table.

**Returns:** `int` — A hash code for the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.TextFontInfo.GetHashCode)

#### `static Autodesk.Navisworks.Api.TextFontInfo InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of TextFontInfo.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.TextFontInfo` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.TextFontInfo.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of TextFontInfo.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.TextFontInfo.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `string ToString()`

Returns a String that represents the current Object.

**Returns:** `string` — A String that represents the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.TextFontInfo.ToString)

### Properties
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Italic` (bool, get) — Italics
- `PointSize` (int, get) — Font size in points
- `Strikeout` (bool, get) — Strikeout
- `Typeface` (string, get) — Font typeface
- `Weight` (int, get) — Font weight

## TextFontMetrics (class)

Global metrics for a typeface.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.TextFontMetrics)

### Methods
#### `static Autodesk.Navisworks.Api.TextFontMetrics InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of TextFontMetrics.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.TextFontMetrics` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.TextFontMetrics.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of TextFontMetrics.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.TextFontMetrics.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

### Properties
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.

## Tool (enum)

View tool enumeration.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Tool)

### Values
- `None` = `0` — No active tool
- `Select` = `1` — Select
- `SelectBox` = `100` — Select Box
- `RedlineFreehand` = `200` — Redline Freehand
- `RedlineLine` = `201` — Redline Line
- `RedlineEllipse` = `202` — Redline Ellipse
- `RedlineCloud` = `203` — Redline Cloud
- `RedlineLineString` = `204` — Redline Line String
- `RedlineTag` = `205` — Redline Tag
- `RedlineText` = `206` — Redline Text
- `RedlineErase` = `207` — Redline Erase
- `RedlineArrow` = `208` — Redline Arrow
- `MeasurePointToPoint` = `300` — Measure Point To Point
- `MeasurePointToMultiplePoints` = `301` — Measure Point To Multiple Points
- `MeasurePointLine` = `302` — Measure Point Line
- `MeasureAccumulate` = `303` — Measure Accumulate
- `MeasureAngle` = `304` — Measure Angle
- `MeasureArea` = `305` — Measure Area
- `MeasureSingle` = `306`
- `BasicViewObjectWheel` = `400` — Basic View Object Wheel
- `BasicTourBuildingWheel` = `401` — Basic Tour Building Wheel
- `FullNavigationWheel` = `402` — Full Navigation Wheel
- `MiniViewObjectWheel` = `403` — Mini View Object Wheel
- `MiniTourBuildingWheel` = `404` — Mini Tour Building Wheel
- `MiniFullNavigationWheel` = `405` — Mini Full Navigation Wheel
- `Full2DNavigationWheel` = `406` — Full 2D Navigation Wheel
- `CommonPan` = `500` — Pan common across all Autodesk products
- `CommonZoom` = `501` — Zoom common across all Autodesk products
- `CommonZoomWindow` = `502` — Zoom Window common across all Autodesk products
- `CommonOrbit` = `503` — Orbit common across all Autodesk products
- `CommonFreeOrbit` = `504` — Free Orbit common across all Autodesk products
- `CommonConstrainedOrbit` = `505` — Constrained Orbit common across all Autodesk products
- `CommonLookAt` = `506` — Look At common across all Autodesk products
- `CommonLookAround` = `507` — Look Around common across all Autodesk products
- `CommonWalk` = `508` — Walk common across all Autodesk products
- `CommonCenter` = `509` — Center common across all Autodesk products
- `NavigateFixed` = `600` — Camera fixed in place
- `NavigateFreeLookAround` = `601` — Classic Navisworks Free Look Around (Swivel)
- `NavigateFreeOrbit` = `602` — Classic Navisworks Free Orbit (Examine)
- `NavigateWalk` = `603` — Classic Navisworks Walk
- `NavigateFly` = `604` — Classic Navisworks Fly
- `NavigateConstrainedOrbit` = `605` — Classic Navisworks Constrained Orbit (Turntable)
- `NavigateZoom` = `607` — Classic Navisworks Zoom
- `NavigatePan` = `608` — Classic Navisworks Pan
- `NavigateConstrainedPan` = `609` — Classic Navisworks Constrained Pan
- `NavigateLookAround` = `610` — Clasic Navisworks Look Around (Swivel)
- `NavigateOrbit` = `611` — Classic Navisworks Orbit
- `NavigateZoomWindow` = `612` — Classic Navisworks Zoom Window (Zoom Box)
- `CustomToolPlugin` = `700` — Functionality is provided by a ToolPlugin

## TooltipResult (class)

Represents a Tooltip and an identifier, that is returned from a call to a ToolPlugin or InputPlugin.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.TooltipResult)

### Constructors
- `TooltipResult()` — Constructor for a TooltipResult.

### Properties
- `DisplayName` (string, get/set) — Tooltip display name.
- `Id` (ulong, get/set) — Tooltip Id.

## Transaction (class)

This is used to batch together edits. Currently it is a requirement that you must call Commit, after doing your edits. In the future rollback may be supported.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Transaction)

### Constructors
- `Transaction(Autodesk.Navisworks.Api.Document document, string displayName)` — This is used to batch together edits. In C# it would typically be used within a 'using' block.In the future it may support rollback.

### Methods
#### `void Commit()`

Call to commit the transaction.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Transaction.Commit)

#### `void Dispose()`

Dispose method of Transaction.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Transaction.Dispose)

### Properties
- `DisplayName` (string, get) — String that may be displayed in the GUI for undoing this transaction.
- `Document` (Autodesk.Navisworks.Api.Document, get) — The Document that this transaction applies to.
- `IsCommitted` (bool, get) — Has this transaction been committed ?
- `IsDisposed` (bool, get) — Has this transaction been disposed ?

## Transform3D (class)

A generic transformation in 3D space.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Transform3D)

### Constructors
- `Transform3D()` — Default constructor. Makes an Identity transform.
- `Transform3D(Autodesk.Navisworks.Api.Matrix3 linear, Autodesk.Navisworks.Api.Vector3D trans)` — Make a transform from a matrix and a vector.
- `Transform3D(Autodesk.Navisworks.Api.Rotation3D rotation)` — Make a transform from a rotation (3D homogenous).
- `Transform3D(Autodesk.Navisworks.Api.Rotation3D rotation, Autodesk.Navisworks.Api.Vector3D translation)` — Make a transform from a rotation and a translation.
- `Transform3D(Autodesk.Navisworks.Api.Transform3D transform)` — Make a transform from another transform.

### Methods
#### `static Autodesk.Navisworks.Api.Transform3D CreateIdentity()`

Create an identity transform.

**Returns:** `Autodesk.Navisworks.Api.Transform3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Transform3D.CreateIdentity)

#### `static Autodesk.Navisworks.Api.Transform3D CreateTranslation(Autodesk.Navisworks.Api.Vector3D translation)`

Set matrix to be 3D homogenous translation matrix.

**Parameters:**
- `translation` (Autodesk.Navisworks.Api.Vector3D)

**Returns:** `Autodesk.Navisworks.Api.Transform3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Transform3D.CreateTranslation%28Autodesk.Navisworks.Api.Vector3D%29)

#### `bool Equals(object obj)`

Determines whether the specified object and the current object refer to the same underlying native object. Returns false if either object has been disposed.

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Transform3D.Equals%28System.Object%29)

#### `Autodesk.Navisworks.Api.Transform3DComponents Factor()`

Treat as homogenous 3D matrix and factor into scale orientation, scale, rotation and translation components.

**Returns:** `Autodesk.Navisworks.Api.Transform3DComponents` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Transform3D.Factor)

#### `Autodesk.Navisworks.Api.Transform3DComponents Factor(Autodesk.Navisworks.Api.Vector3D worldCenter)`

Treat as homogenous 3D matrix and factor into scale orientation, scale, rotation and translation components.

**Parameters:**
- `worldCenter` (Autodesk.Navisworks.Api.Vector3D)

**Returns:** `Autodesk.Navisworks.Api.Transform3DComponents` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Transform3D.Factor%28Autodesk.Navisworks.Api.Vector3D%29)

#### `int GetHashCode()`

Serves as a hash function for a particular type. GetHashCode is suitable for use in hashing algorithms and data structures like a hash table.

**Returns:** `int` — A hash code for the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Transform3D.GetHashCode)

#### `static Autodesk.Navisworks.Api.Transform3D InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of Transform3D.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Transform3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Transform3D.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of Transform3D.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Transform3D.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `Autodesk.Navisworks.Api.Transform3D Inverse()`

Return inverse of matrix. Matrix must be non-singular (non-zero determinant).

**Returns:** `Autodesk.Navisworks.Api.Transform3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Transform3D.Inverse)

#### `bool IsAffine()`

Determines if the transform is affine.

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Transform3D.IsAffine)

#### `bool IsIdentity()`

Determines if the transform is an identity.

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Transform3D.IsIdentity)

#### `bool IsLinear()`

Determines if the transform is linear.

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Transform3D.IsLinear)

#### `bool IsRotation()`

Determines if the transform is a rotation.

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Transform3D.IsRotation)

#### `bool IsTranslation()`

Determines if the transform is a translation.

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Transform3D.IsTranslation)

#### `bool IsUniformScale()`

Determines if the transform is a uniform scaling.

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Transform3D.IsUniformScale)

#### `static Autodesk.Navisworks.Api.Transform3D Multiply(Autodesk.Navisworks.Api.Transform3D leftTransform, Autodesk.Navisworks.Api.Transform3D rightTransform)`

Multiply two transforms in the order that the arguments are given, and return the result.

**Parameters:**
- `leftTransform` (Autodesk.Navisworks.Api.Transform3D)
- `rightTransform` (Autodesk.Navisworks.Api.Transform3D)

**Returns:** `Autodesk.Navisworks.Api.Transform3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Transform3D.Multiply%28Autodesk.Navisworks.Api.Transform3D%2CAutodesk.Navisworks.Api.Transform3D%29)

#### `string ToString()`

Returns a String that represents the current Object.

**Returns:** `string` — A String that represents the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Transform3D.ToString)

#### `bool TolerantEquals(Autodesk.Navisworks.Api.Transform3D transform)`

Equal within a tolerance. Each element of matrix is equal to corresponding element, within the given tolerance.

**Parameters:**
- `transform` (Autodesk.Navisworks.Api.Transform3D)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Transform3D.TolerantEquals%28Autodesk.Navisworks.Api.Transform3D%29)

#### `bool TolerantEquals(Autodesk.Navisworks.Api.Transform3D transform, double tolerance)`

Equal within a tolerance. Each element of matrix is equal to corresponding element, within the given tolerance.

**Parameters:**
- `transform` (Autodesk.Navisworks.Api.Transform3D)
- `tolerance` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Transform3D.TolerantEquals%28Autodesk.Navisworks.Api.Transform3D%2CSystem.Double%29)

#### `static Autodesk.Navisworks.Api.Transform3D TranslateRight(Autodesk.Navisworks.Api.Vector3D translation, Autodesk.Navisworks.Api.Transform3D transform)`

Translate a transform by a vector.

**Parameters:**
- `translation` (Autodesk.Navisworks.Api.Vector3D)
- `transform` (Autodesk.Navisworks.Api.Transform3D)

**Returns:** `Autodesk.Navisworks.Api.Transform3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Transform3D.TranslateRight%28Autodesk.Navisworks.Api.Vector3D%2CAutodesk.Navisworks.Api.Transform3D%29)

### Properties
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Linear` (Autodesk.Navisworks.Api.Matrix3, get) — Get parts of matrix which represent orthogonal transforms.
- `Translation` (Autodesk.Navisworks.Api.Vector3D, get) — Get parts of matrix which represent the translation.

## Transform3DComponents (class)

Affine transform represented as individual components that combine to form complete transform. Immutable.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Transform3DComponents)

### Methods
#### `Autodesk.Navisworks.Api.Transform3D Combine()`

Combine components together into a composite transform.

**Returns:** `Autodesk.Navisworks.Api.Transform3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Transform3DComponents.Combine)

#### `int GetHashCode()`

Serves as a hash function for a particular type. GetHashCode is suitable for use in hashing algorithms and data structures like a hash table.

**Returns:** `int` — A hash code for the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Transform3DComponents.GetHashCode)

#### `static Autodesk.Navisworks.Api.Transform3DComponents InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of Transform3DComponents.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Transform3DComponents` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Transform3DComponents.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of Transform3DComponents.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Transform3DComponents.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `string ToString()`

Returns a String that represents the current Object.

**Returns:** `string` — A String that represents the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Transform3DComponents.ToString)

### Properties
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Rotation` (Autodesk.Navisworks.Api.Rotation3D, get/set) — The rotation component of the transform.
- `Scale` (Autodesk.Navisworks.Api.Vector3D, get/set) — The scale component of the transform.
- `ScaleOrientation` (Autodesk.Navisworks.Api.Rotation3D, get/set) — The scale orientation component of the transform.
- `Translation` (Autodesk.Navisworks.Api.Vector3D, get/set) — The translation component of the transform.

## UnitConversion (class)

Unit conversion package class.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.UnitConversion)

### Methods
#### `static Autodesk.Navisworks.Api.UnitConversion InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of UnitConversion.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.UnitConversion` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitConversion.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of UnitConversion.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitConversion.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `static double ScaleFactor(Autodesk.Navisworks.Api.Units from, Autodesk.Navisworks.Api.Units to)`

Return the scale factor between two linear units.

**Parameters:**
- `from` (Autodesk.Navisworks.Api.Units)
- `to` (Autodesk.Navisworks.Api.Units)

**Returns:** `double` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitConversion.ScaleFactor%28Autodesk.Navisworks.Api.Units%2CAutodesk.Navisworks.Api.Units%29)

### Properties
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.

## UnitVector3D (class)

A vector with unit length. Represents a direction in 3D space. Immutable.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.UnitVector3D)

### Constructors
- `UnitVector3D(Autodesk.Navisworks.Api.Vector3D vector)` — Create a unit vector from a vector, normalizing if required
- `UnitVector3D(double x, double y, double z)` — Create a unit vector from X, Y and Z components, normalizing if required

### Methods
#### `Autodesk.Navisworks.Api.Vector3D Add(Autodesk.Navisworks.Api.Vector3D vector)`

Adds a vector to this to produce a new vector

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector3D)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.Add%28Autodesk.Navisworks.Api.Vector3D%29)

#### `Autodesk.Navisworks.Api.Vector3D Add(double value)`

Adds a value to every X, Y and Z coordinate in the Vector and returns a new Vector with the result

**Parameters:**
- `value` (double)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.Add%28System.Double%29)

#### `double Angle(Autodesk.Navisworks.Api.UnitVector3D vector)`

Returns the angle between this vector and another vector in the range [0, PI]

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.UnitVector3D) — vector to calculate angle with

**Returns:** `double` — angle in radians between this and vector

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.Angle%28Autodesk.Navisworks.Api.UnitVector3D%29)

#### `double AngleWithReference(Autodesk.Navisworks.Api.UnitVector3D vector, Autodesk.Navisworks.Api.UnitVector3D reference)`

Returns the angle between this vector and another vector in the range [-PI, PI], using a reference vector to determine the sign of the angle

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.UnitVector3D) — vector to calculate angle with
- `reference` (Autodesk.Navisworks.Api.UnitVector3D) — reference vector pointing away from the plane containing this and vector

**Returns:** `double` — angle in radians between this and vector

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.AngleWithReference%28Autodesk.Navisworks.Api.UnitVector3D%2CAutodesk.Navisworks.Api.UnitVector3D%29)

#### `Autodesk.Navisworks.Api.Vector3D Cross(Autodesk.Navisworks.Api.Vector3D vector)`

Returns the cross product between this vector and another

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector3D)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.Cross%28Autodesk.Navisworks.Api.Vector3D%29)

#### `bool DirectionEquals(Autodesk.Navisworks.Api.Vector3D vector)`

Compare with direction of vector using a tolerance

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector3D)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.DirectionEquals%28Autodesk.Navisworks.Api.Vector3D%29)

#### `bool DirectionEquals(Autodesk.Navisworks.Api.Vector3D vector, double tolerance)`

Compare with direction of vector using a tolerance

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector3D)
- `tolerance` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.DirectionEquals%28Autodesk.Navisworks.Api.Vector3D%2CSystem.Double%29)

#### `Autodesk.Navisworks.Api.Vector3D Divide(Autodesk.Navisworks.Api.Vector3D vector)`

Divides this vector by another to produce a new vector

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector3D)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.Divide%28Autodesk.Navisworks.Api.Vector3D%29)

#### `Autodesk.Navisworks.Api.Vector3D Divide(double value)`

Divides every X, Y and Z coordinate in the Vector by a value and returns a new Vector with the result

**Parameters:**
- `value` (double)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.Divide%28System.Double%29)

#### `double Dot(Autodesk.Navisworks.Api.Vector3D vector)`

Returns the dot product between this vector and another

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector3D)

**Returns:** `double` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.Dot%28Autodesk.Navisworks.Api.Vector3D%29)

#### `bool Equals(object obj)`

Determines whether the specified object and the current object refer to the same underlying native object. Returns false if either object has been disposed.

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.Equals%28System.Object%29)

#### `int GetHashCode()`

Serves as a hash function for a particular type. GetHashCode is suitable for use in hashing algorithms and data structures like a hash table.

**Returns:** `int` — A hash code for the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.GetHashCode)

#### `static Autodesk.Navisworks.Api.UnitVector3D InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of UnitVector3D.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.UnitVector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of UnitVector3D.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `Autodesk.Navisworks.Api.Vector3D Multiply(Autodesk.Navisworks.Api.Vector3D vector)`

Multiplies a vector by this to produce a new vector

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector3D)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.Multiply%28Autodesk.Navisworks.Api.Vector3D%29)

#### `Autodesk.Navisworks.Api.Vector3D Multiply(double value)`

Multiplies every X, Y and Z coordinate in the Vector by a value and returns a new Vector with the result

**Parameters:**
- `value` (double)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.Multiply%28System.Double%29)

#### `Autodesk.Navisworks.Api.UnitVector3D Negate()`

Returns a negated version of this vector (all components negated)

**Returns:** `Autodesk.Navisworks.Api.UnitVector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.Negate)

#### `Autodesk.Navisworks.Api.Vector3D Subtract(Autodesk.Navisworks.Api.Vector3D vector)`

Subtracts a vector from this to produce a new vector

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector3D)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.Subtract%28Autodesk.Navisworks.Api.Vector3D%29)

#### `Autodesk.Navisworks.Api.Vector3D Subtract(double value)`

Subtracts a value from every X, Y and Z coordinate in the Vector and returns a new Vector with the result

**Parameters:**
- `value` (double)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.Subtract%28System.Double%29)

#### `string ToString()`

Returns a String that represents the current Object.

**Returns:** `string` — A String that represents the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.ToString)

#### `Autodesk.Navisworks.Api.Vector3D ToVector3D()`

Convert unit vector to a vector

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.ToVector3D)

#### `bool TolerantEquals(Autodesk.Navisworks.Api.UnitVector3D vector)`

Compare unit vectors using a tolerance

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.UnitVector3D)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.TolerantEquals%28Autodesk.Navisworks.Api.UnitVector3D%29)

#### `bool TolerantEquals(Autodesk.Navisworks.Api.UnitVector3D vector, double tolerance)`

Compare unit vectors using a tolerance

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.UnitVector3D)
- `tolerance` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.TolerantEquals%28Autodesk.Navisworks.Api.UnitVector3D%2CSystem.Double%29)

#### `static bool TolerantEquals(Autodesk.Navisworks.Api.UnitVector3D vector0, Autodesk.Navisworks.Api.UnitVector3D vector1)`

Compare vectors using a tolerance

**Parameters:**
- `vector0` (Autodesk.Navisworks.Api.UnitVector3D)
- `vector1` (Autodesk.Navisworks.Api.UnitVector3D)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.TolerantEquals%28Autodesk.Navisworks.Api.UnitVector3D%2CAutodesk.Navisworks.Api.UnitVector3D%29)

#### `static bool TolerantEquals(Autodesk.Navisworks.Api.UnitVector3D vector0, Autodesk.Navisworks.Api.UnitVector3D vector1, double tolerance)`

Compare vectors using a tolerance

**Parameters:**
- `vector0` (Autodesk.Navisworks.Api.UnitVector3D)
- `vector1` (Autodesk.Navisworks.Api.UnitVector3D)
- `tolerance` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.TolerantEquals%28Autodesk.Navisworks.Api.UnitVector3D%2CAutodesk.Navisworks.Api.UnitVector3D%2CSystem.Double%29)

#### `static Autodesk.Navisworks.Api.Vector3D op_Addition(Autodesk.Navisworks.Api.UnitVector3D value1, Autodesk.Navisworks.Api.UnitVector3D value2)`

Adds two vectors to produce a new vector

**Parameters:**
- `value1` (Autodesk.Navisworks.Api.UnitVector3D)
- `value2` (Autodesk.Navisworks.Api.UnitVector3D)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.op_Addition%28Autodesk.Navisworks.Api.UnitVector3D%2CAutodesk.Navisworks.Api.UnitVector3D%29)

#### `static Autodesk.Navisworks.Api.Vector3D op_Addition(Autodesk.Navisworks.Api.UnitVector3D value1, Autodesk.Navisworks.Api.Vector3D value2)`

Adds two vectors to produce a new vector

**Parameters:**
- `value1` (Autodesk.Navisworks.Api.UnitVector3D)
- `value2` (Autodesk.Navisworks.Api.Vector3D)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.op_Addition%28Autodesk.Navisworks.Api.UnitVector3D%2CAutodesk.Navisworks.Api.Vector3D%29)

#### `static Autodesk.Navisworks.Api.Vector3D op_Addition(Autodesk.Navisworks.Api.UnitVector3D vector, double value)`

Adds a value to every X, Y and Z coordinate in the Vector and returns a new Vector with the result

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.UnitVector3D)
- `value` (double)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.op_Addition%28Autodesk.Navisworks.Api.UnitVector3D%2CSystem.Double%29)

#### `static Autodesk.Navisworks.Api.Vector3D op_Addition(Autodesk.Navisworks.Api.Vector3D value1, Autodesk.Navisworks.Api.UnitVector3D value2)`

Adds two vectors to produce a new vector

**Parameters:**
- `value1` (Autodesk.Navisworks.Api.Vector3D)
- `value2` (Autodesk.Navisworks.Api.UnitVector3D)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.op_Addition%28Autodesk.Navisworks.Api.Vector3D%2CAutodesk.Navisworks.Api.UnitVector3D%29)

#### `static Autodesk.Navisworks.Api.Vector3D op_Addition(double value, Autodesk.Navisworks.Api.UnitVector3D vector)`

Adds a value to every X, Y and Z coordinate in the Vector and returns a new Vector with the result

**Parameters:**
- `value` (double)
- `vector` (Autodesk.Navisworks.Api.UnitVector3D)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.op_Addition%28System.Double%2CAutodesk.Navisworks.Api.UnitVector3D%29)

#### `static Autodesk.Navisworks.Api.Vector3D op_Division(Autodesk.Navisworks.Api.UnitVector3D vector, double value)`

Divides every X, Y and Z coordinate in the Vector by a value and returns a new Vector with the result

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.UnitVector3D)
- `value` (double)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.op_Division%28Autodesk.Navisworks.Api.UnitVector3D%2CSystem.Double%29)

#### `static Autodesk.Navisworks.Api.Vector3D op_Implicit(Autodesk.Navisworks.Api.UnitVector3D vector)`

Converts a UnitVector3D to a Vector3D

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.UnitVector3D)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.op_Implicit%28Autodesk.Navisworks.Api.UnitVector3D%29~Autodesk.Navisworks.Api.Vector3D)

#### `static Autodesk.Navisworks.Api.Vector3D op_Multiply(Autodesk.Navisworks.Api.UnitVector3D vector, double value)`

Multiplies every X, Y and Z coordinate in the Vector by a value and returns a new Vector with the result

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.UnitVector3D)
- `value` (double)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.op_Multiply%28Autodesk.Navisworks.Api.UnitVector3D%2CSystem.Double%29)

#### `static Autodesk.Navisworks.Api.Vector3D op_Multiply(double value, Autodesk.Navisworks.Api.UnitVector3D vector)`

Multiplies every X, Y and Z coordinate in the Vector by a value and returns a new Vector with the result

**Parameters:**
- `value` (double)
- `vector` (Autodesk.Navisworks.Api.UnitVector3D)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.op_Multiply%28System.Double%2CAutodesk.Navisworks.Api.UnitVector3D%29)

#### `static Autodesk.Navisworks.Api.Vector3D op_Subtraction(Autodesk.Navisworks.Api.UnitVector3D value1, Autodesk.Navisworks.Api.UnitVector3D value2)`

Subtracts two vectors to produce a new vector

**Parameters:**
- `value1` (Autodesk.Navisworks.Api.UnitVector3D)
- `value2` (Autodesk.Navisworks.Api.UnitVector3D)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.op_Subtraction%28Autodesk.Navisworks.Api.UnitVector3D%2CAutodesk.Navisworks.Api.UnitVector3D%29)

#### `static Autodesk.Navisworks.Api.Vector3D op_Subtraction(Autodesk.Navisworks.Api.UnitVector3D value1, Autodesk.Navisworks.Api.Vector3D value2)`

Subtracts two vectors to produce a new vector

**Parameters:**
- `value1` (Autodesk.Navisworks.Api.UnitVector3D)
- `value2` (Autodesk.Navisworks.Api.Vector3D)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.op_Subtraction%28Autodesk.Navisworks.Api.UnitVector3D%2CAutodesk.Navisworks.Api.Vector3D%29)

#### `static Autodesk.Navisworks.Api.Vector3D op_Subtraction(Autodesk.Navisworks.Api.UnitVector3D vector, double value)`

Subtracts a value from every X, Y and Z coordinate in the Vector and returns a new Vector with the result

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.UnitVector3D)
- `value` (double)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.op_Subtraction%28Autodesk.Navisworks.Api.UnitVector3D%2CSystem.Double%29)

#### `static Autodesk.Navisworks.Api.Vector3D op_Subtraction(Autodesk.Navisworks.Api.Vector3D value1, Autodesk.Navisworks.Api.UnitVector3D value2)`

Subtracts two vectors to produce a new vector

**Parameters:**
- `value1` (Autodesk.Navisworks.Api.Vector3D)
- `value2` (Autodesk.Navisworks.Api.UnitVector3D)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.UnitVector3D.op_Subtraction%28Autodesk.Navisworks.Api.Vector3D%2CAutodesk.Navisworks.Api.UnitVector3D%29)

### Properties
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `IsZero` (bool, get) — Is this a zero vector ? Will occur if all inputs to constructor are zero.
- `Item` (double, get) — Item property of UnitVector3D.
- `UnitX` (Autodesk.Navisworks.Api.UnitVector3D, get) — Returns a unit vector in the X direction (1,0,0)
- `UnitY` (Autodesk.Navisworks.Api.UnitVector3D, get) — Returns a unit vector in the Y direction (0,1,0)
- `UnitZ` (Autodesk.Navisworks.Api.UnitVector3D, get) — Returns a unit vector in the Z direction (0,0,1)
- `X` (double, get) — Returns the X component of the vector
- `Y` (double, get) — Returns the Y component of the vector
- `Z` (double, get) — Returns the Z component of the vector

## Units (enum)

Units of linear dimension.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Units)

### Values
- `Meters` = `0` — SI meter. Distance light travels, in a vacuum, in 1 299,792,458th of a second
- `Centimeters` = `1` — 1 100th of an SI meter
- `Millimeters` = `2` — 1 1000th of an SI meter
- `Feet` = `3` — 1 3rd of a yard
- `Inches` = `4` — 1 12th of a foot, 2.54 centimeters exactly
- `Yards` = `5` — Imperial yard, 0.9144 SI meters exactly
- `Kilometers` = `6` — 1000 SI meters
- `Miles` = `7` — 1760 yards
- `Micrometers` = `8` — 1 1,000,000th of an SI meter, a micron
- `Mils` = `9` — 1 1,000th of an inch
- `Microinches` = `10` — 1 1,000,000th of an inch

## UserMessageEventArgs (class)

UserMessageEventArgs class in Autodesk.Navisworks.Api.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.UserMessageEventArgs)

### Properties
- `Category` (Autodesk.Navisworks.Api.MessageCenterCategory, get/set) — Category property of UserMessageEventArgs.
- `ExtendedMessage` (string, get/set) — ExtendedMessage property of UserMessageEventArgs.
- `InternalId` (string, get/set) — InternalId property of UserMessageEventArgs.
- `Message` (string, get/set) — Message property of UserMessageEventArgs.
- `MessageFlags` (int, get/set) — MessageFlags property of UserMessageEventArgs.
- `Source` (string, get/set) — Source property of UserMessageEventArgs.

## VariantData (class)

A value type that can store data of one of several different types.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.VariantData)

### Constructors
- `VariantData()` — Creates Data with no value stored
- `VariantData(Autodesk.Navisworks.Api.NamedConstant value)` — Creates a named constant value
- `VariantData(bool value)` — Creates a boolean value
- `VariantData(double value)` — Creates a unitless double value
- `VariantData(double value, Autodesk.Navisworks.Api.VariantDataType type)` — Creates a specific type of double based value
- `VariantData(double x, double y)` — Creates a 2D point value
- `VariantData(double x, double y, double z)` — Creates a 3D point value
- `VariantData(int value)` — Creates an integer value
- `VariantData(string value)` — Creates a display string value
- `VariantData(uint value)` — Creates an LtNat32 value
- `VariantData(ulong value)` — Creates an LtNat64 value

### Methods
#### `static string ElementName()`

The XML element name used for XML serialization

**Returns:** `string` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.ElementName)

#### `bool Equals(object obj)`

Determines whether the specified object and the current object refer to the same underlying native object. Returns false if either object has been disposed.

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.Equals%28System.Object%29)

#### `static Autodesk.Navisworks.Api.VariantData FromBoolean(bool value)`

Creates a boolean value

**Parameters:**
- `value` (bool)

**Returns:** `Autodesk.Navisworks.Api.VariantData` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.FromBoolean%28System.Boolean%29)

#### `static Autodesk.Navisworks.Api.VariantData FromDateTime(System.DateTime value)`

Creates a DateTime value

**Parameters:**
- `value` (System.DateTime)

**Returns:** `Autodesk.Navisworks.Api.VariantData` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.FromDateTime%28System.DateTime%29)

#### `static Autodesk.Navisworks.Api.VariantData FromDisplayString(string value)`

Creates a display string value

**Parameters:**
- `value` (string)

**Returns:** `Autodesk.Navisworks.Api.VariantData` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.FromDisplayString%28System.String%29)

#### `static Autodesk.Navisworks.Api.VariantData FromDouble(double value)`

Creates a unitless double value

**Parameters:**
- `value` (double)

**Returns:** `Autodesk.Navisworks.Api.VariantData` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.FromDouble%28System.Double%29)

#### `static Autodesk.Navisworks.Api.VariantData FromDoubleAngle(double value)`

Creates a double angle value

**Parameters:**
- `value` (double)

**Returns:** `Autodesk.Navisworks.Api.VariantData` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.FromDoubleAngle%28System.Double%29)

#### `static Autodesk.Navisworks.Api.VariantData FromDoubleArea(double value)`

Creates a double area value

**Parameters:**
- `value` (double)

**Returns:** `Autodesk.Navisworks.Api.VariantData` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.FromDoubleArea%28System.Double%29)

#### `static Autodesk.Navisworks.Api.VariantData FromDoubleLength(double value)`

Creates a double length value

**Parameters:**
- `value` (double)

**Returns:** `Autodesk.Navisworks.Api.VariantData` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.FromDoubleLength%28System.Double%29)

#### `static Autodesk.Navisworks.Api.VariantData FromDoubleVolume(double value)`

Creates a double volume value

**Parameters:**
- `value` (double)

**Returns:** `Autodesk.Navisworks.Api.VariantData` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.FromDoubleVolume%28System.Double%29)

#### `static Autodesk.Navisworks.Api.VariantData FromIdentifierString(string value)`

Creates an identifier string value

**Parameters:**
- `value` (string)

**Returns:** `Autodesk.Navisworks.Api.VariantData` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.FromIdentifierString%28System.String%29)

#### `static Autodesk.Navisworks.Api.VariantData FromInt32(int value)`

Creates a 32 bit integer value

**Parameters:**
- `value` (int)

**Returns:** `Autodesk.Navisworks.Api.VariantData` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.FromInt32%28System.Int32%29)

#### `static Autodesk.Navisworks.Api.VariantData FromInt64(long value)`

Creates a 64 bit integer value

**Parameters:**
- `value` (long)

**Returns:** `Autodesk.Navisworks.Api.VariantData` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.FromInt64%28System.Int64%29)

#### `static Autodesk.Navisworks.Api.VariantData FromNamedConstant(Autodesk.Navisworks.Api.NamedConstant value)`

Creates a named constant value

**Parameters:**
- `value` (Autodesk.Navisworks.Api.NamedConstant)

**Returns:** `Autodesk.Navisworks.Api.VariantData` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.FromNamedConstant%28Autodesk.Navisworks.Api.NamedConstant%29)

#### `static Autodesk.Navisworks.Api.VariantData FromNat32(uint value)`

Creates a Nat32 value

**Parameters:**
- `value` (uint)

**Returns:** `Autodesk.Navisworks.Api.VariantData` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.FromNat32%28System.UInt32%29)

#### `static Autodesk.Navisworks.Api.VariantData FromNat64(ulong value)`

Creates a Nat64 value

**Parameters:**
- `value` (ulong)

**Returns:** `Autodesk.Navisworks.Api.VariantData` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.FromNat64%28System.UInt64%29)

#### `static Autodesk.Navisworks.Api.VariantData FromNone()`

Creates Data with no value stored

**Returns:** `Autodesk.Navisworks.Api.VariantData` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.FromNone)

#### `static Autodesk.Navisworks.Api.VariantData FromPoint2D(Autodesk.Navisworks.Api.Point2D point)`

Creates a Point2D value

**Parameters:**
- `point` (Autodesk.Navisworks.Api.Point2D)

**Returns:** `Autodesk.Navisworks.Api.VariantData` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.FromPoint2D%28Autodesk.Navisworks.Api.Point2D%29)

#### `static Autodesk.Navisworks.Api.VariantData FromPoint2D(double x, double y)`

Creates a 2D point value

**Parameters:**
- `x` (double)
- `y` (double)

**Returns:** `Autodesk.Navisworks.Api.VariantData` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.FromPoint2D%28System.Double%2CSystem.Double%29)

#### `static Autodesk.Navisworks.Api.VariantData FromPoint3D(Autodesk.Navisworks.Api.Point3D point)`

Creates a Point3D value

**Parameters:**
- `point` (Autodesk.Navisworks.Api.Point3D)

**Returns:** `Autodesk.Navisworks.Api.VariantData` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.FromPoint3D%28Autodesk.Navisworks.Api.Point3D%29)

#### `static Autodesk.Navisworks.Api.VariantData FromPoint3D(double x, double y, double z)`

Creates a 3D point value

**Parameters:**
- `x` (double)
- `y` (double)
- `z` (double)

**Returns:** `Autodesk.Navisworks.Api.VariantData` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.FromPoint3D%28System.Double%2CSystem.Double%2CSystem.Double%29)

#### `int GetHashCode()`

Serves as a hash function for a particular type. GetHashCode is suitable for use in hashing algorithms and data structures like a hash table.

**Returns:** `int` — A hash code for the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.GetHashCode)

#### `static Autodesk.Navisworks.Api.VariantData InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of VariantData.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.VariantData` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of VariantData.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `static Autodesk.Navisworks.Api.VariantData ReadFromXml(string type_attrib_value, string element_content_value, bool include_names)`

Create a VariantData object from serialised XML values

**Parameters:**
- `type_attrib_value` (string)
- `element_content_value` (string)
- `include_names` (bool)

**Returns:** `Autodesk.Navisworks.Api.VariantData` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.ReadFromXml%28System.String%2CSystem.String%2CSystem.Boolean%29)

#### `double ToAnyDouble()`

Access any type of double value

**Returns:** `double` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.ToAnyDouble)

#### `bool ToBoolean()`

Access boolean value

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.ToBoolean)

#### `System.DateTime ToDateTime()`

Access DateTime value

**Returns:** `System.DateTime` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.ToDateTime)

#### `string ToDisplayString()`

Access display string value

**Returns:** `string` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.ToDisplayString)

#### `double ToDouble()`

Access unitless double value

**Returns:** `double` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.ToDouble)

#### `double ToDoubleAngle()`

Access double angle value

**Returns:** `double` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.ToDoubleAngle)

#### `double ToDoubleArea()`

Access double area value

**Returns:** `double` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.ToDoubleArea)

#### `double ToDoubleLength()`

Access double length value

**Returns:** `double` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.ToDoubleLength)

#### `double ToDoubleVolume()`

Access double volume value

**Returns:** `double` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.ToDoubleVolume)

#### `string ToIdentifierString()`

Access identifier string value

**Returns:** `string` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.ToIdentifierString)

#### `int ToInt32()`

Access 32 bit integer value

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.ToInt32)

#### `long ToInt64()`

Access Int64 value

**Returns:** `long` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.ToInt64)

#### `Autodesk.Navisworks.Api.NamedConstant ToNamedConstant()`

Access named constant value

**Returns:** `Autodesk.Navisworks.Api.NamedConstant` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.ToNamedConstant)

#### `uint ToNat32()`

Access LtNat32 value

**Returns:** `uint` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.ToNat32)

#### `ulong ToNat64()`

Access LtNat64 value

**Returns:** `ulong` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.ToNat64)

#### `Autodesk.Navisworks.Api.Point2D ToPoint2D()`

Access Point2D value

**Returns:** `Autodesk.Navisworks.Api.Point2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.ToPoint2D)

#### `Autodesk.Navisworks.Api.Point3D ToPoint3D()`

Access Point3D value

**Returns:** `Autodesk.Navisworks.Api.Point3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.ToPoint3D)

#### `void ToSerializableString(out string str)`

Write serializable string representation

**Parameters:**
- `str` (string)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.ToSerializableString%28System.String%40%29)

#### `string ToString()`

Returns a String that represents the current Object.

**Returns:** `string` — A String that represents the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.ToString)

#### `static string XmlAttribName()`

The XML attribute name used for XML serialization

**Returns:** `string` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.XmlAttribName)

#### `bool XmlAttribValue(out string value)`

The XML attribute value used for XML serialization

**Parameters:**
- `value` (string)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.XmlAttribValue%28System.String%40%29)

#### `bool XmlElementContent(out string content, bool include_names)`

The XML element content used for XML serialization

**Parameters:**
- `content` (string)
- `include_names` (bool)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantData.XmlElementContent%28System.String%40%2CSystem.Boolean%29)

### Properties
- `DataType` (Autodesk.Navisworks.Api.VariantDataType, get) — Type of value stored
- `IsAnyDouble` (bool, get) — Is data any type of double based value ?
- `IsBoolean` (bool, get) — Is data a boolean value ?
- `IsDateTime` (bool, get) — Is data a DateTime value ?
- `IsDisplayString` (bool, get) — Is data a display string value ?
- `IsDouble` (bool, get) — Is data a unitless double value ?
- `IsDoubleAngle` (bool, get) — Is data a double angle value ?
- `IsDoubleArea` (bool, get) — Is data a double area value ?
- `IsDoubleLength` (bool, get) — Is data a double length value ?
- `IsDoubleVolume` (bool, get) — Is data a double volume value ?
- `IsIdentifierString` (bool, get) — Is data an identifier string value ?
- `IsInt32` (bool, get) — Is data a unitless 32 bit integer value ?
- `IsInt64` (bool, get) — Is data an int64?
- `IsNamedConstant` (bool, get) — Is data a named constant value ?
- `IsNat32` (bool, get) — Is data an LtNat32?
- `IsNat64` (bool, get) — Is data an LtNat64?
- `IsNone` (bool, get) — Does data have no value ?
- `IsPoint2D` (bool, get) — Is data a 2D point?
- `IsPoint3D` (bool, get) — Is data a 3D point?
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.

## VariantDataCollection (class)

VariantDataCollection class in Autodesk.Navisworks.Api.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.VariantDataCollection)

### Constructors
- `VariantDataCollection()` — Create an empty collection
- `VariantDataCollection(Autodesk.Navisworks.Api.VariantDataCollection from)` — Create a collection that is a copy of from

### Methods
#### `bool Contains(Autodesk.Navisworks.Api.VariantData item)`

Determines whether the SavedItemCollection contains a specific value.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.VariantData)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantDataCollection.Contains%28Autodesk.Navisworks.Api.VariantData%29)

#### `void CopyTo(Autodesk.Navisworks.Api.VariantData[] array, int arrayIndex)`

Copies the elements of the ICollection`1 to an Array, starting at a particular Array index.

**Parameters:**
- `array` (Autodesk.Navisworks.Api.VariantData[]) — The one-dimensional Array that is the destination of the elements copied from ICollection`1. The Array must have zero-based indexing.
- `arrayIndex` (int) — The zero-based index in array at which copying begins.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantDataCollection.CopyTo%28Autodesk.Navisworks.Api.VariantData%5B%5D%2CSystem.Int32%29)

#### `void CopyTo(System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.VariantData> to)`

Enumerates over the collection and copies the items into to

**Parameters:**
- `to` (System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.VariantData>) — Collection to copy into

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantDataCollection.CopyTo%28System.Collections.Generic.ICollection%7BAutodesk.Navisworks.Api.VariantData%7D%29)

#### `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.VariantData> GetEnumerator()`

Returns an enumerator that can iterate through the collection.

**Returns:** `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.VariantData>` — An IEnumerator that can be used to iterate through the collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantDataCollection.GetEnumerator)

#### `int IndexOf(Autodesk.Navisworks.Api.VariantData item)`

Determines the index of a specific item in the Collection

**Parameters:**
- `item` (Autodesk.Navisworks.Api.VariantData)

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantDataCollection.IndexOf%28Autodesk.Navisworks.Api.VariantData%29)

#### `static Autodesk.Navisworks.Api.VariantDataCollection InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of VariantDataCollection.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.VariantDataCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantDataCollection.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of VariantDataCollection.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.VariantDataCollection.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

### Properties
- `Count` (int, get) — Gets the number of elements contained in the collection.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Item` (Autodesk.Navisworks.Api.VariantData, get) — Item property of VariantDataCollection.

## VariantDataType (enum)

Type of data stored in a VariantData class.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.VariantDataType)

### Values
- `None` = `0` — Empty. No data stored.
- `Double` = `1` — Unitless double value
- `Int32` = `2` — Unitless 32 bit integer value
- `Boolean` = `3` — Boolean (true/false) value
- `DisplayString` = `4` — String intended for display to the end user (normally localized)
- `DateTime` = `5` — A specific date and time (usually UTC)
- `DoubleLength` = `6` — A double that represents a length (specific units depend on context)
- `DoubleAngle` = `7` — A double that represents an angle in radians
- `NamedConstant` = `8` — A named constant
- `IdentifierString` = `9` — String intended to be used as a programmatic identifier. 7-bit ASCII characters only.
- `DoubleArea` = `10` — A double that species an area (specific units depend on context)
- `DoubleVolume` = `11` — A double that species a volume (specific units depend on context)
- `Point3D` = `12` — A 3D point value
- `Point2D` = `13` — A 2D point value
- `Nat32` = `14` — Unsigned 32 bit integer value
- `Int64` = `15` — 64 bit integer value
- `Nat64` = `16` — Unsigned 64 bit integer value

## Vector2D (class)

Represents a direction with magnitude in 2D space. Immutable.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Vector2D)

### Constructors
- `Vector2D()` — Create a zero vector
- `Vector2D(double x, double y)` — Create a vector from X and Y components

### Methods
#### `Autodesk.Navisworks.Api.Point2D Add(Autodesk.Navisworks.Api.Point2D point)`

Translates a point by this vector and returns the resulting point

**Parameters:**
- `point` (Autodesk.Navisworks.Api.Point2D)

**Returns:** `Autodesk.Navisworks.Api.Point2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.Add%28Autodesk.Navisworks.Api.Point2D%29)

#### `Autodesk.Navisworks.Api.Vector2D Add(Autodesk.Navisworks.Api.Vector2D vector)`

Adds another vector to this to produce a new vector

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector2D)

**Returns:** `Autodesk.Navisworks.Api.Vector2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.Add%28Autodesk.Navisworks.Api.Vector2D%29)

#### `Autodesk.Navisworks.Api.Vector2D Add(double value)`

Adds a value to every X, Y and Z coordinate in the Vector and returns a new Vector with the result

**Parameters:**
- `value` (double)

**Returns:** `Autodesk.Navisworks.Api.Vector2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.Add%28System.Double%29)

#### `double Angle(Autodesk.Navisworks.Api.Vector2D vector)`

Returns the angle between this vector and another vector in the range [0, PI]

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector2D) — Vector to calculate angle with.

**Returns:** `double` — Angle in radians between this and vector.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.Angle%28Autodesk.Navisworks.Api.Vector2D%29)

#### `double AngleWithReference(Autodesk.Navisworks.Api.Vector2D vector, Autodesk.Navisworks.Api.Vector2D reference)`

Returns the angle between this vector and another vector in the range [-PI, PI], using a reference vector to determine the sign of the angle

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector2D) — Vector to calculate angle with.
- `reference` (Autodesk.Navisworks.Api.Vector2D) — Reference vector pointing away from the plane containing this and vector.

**Returns:** `double` — Angle in radians between this and vector.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.AngleWithReference%28Autodesk.Navisworks.Api.Vector2D%2CAutodesk.Navisworks.Api.Vector2D%29)

#### `double Cross(Autodesk.Navisworks.Api.Vector2D vector)`

Returns the cross product between this vector and another

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector2D)

**Returns:** `double` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.Cross%28Autodesk.Navisworks.Api.Vector2D%29)

#### `bool DirectionEquals(Autodesk.Navisworks.Api.Vector2D vector)`

Compare direction of vectors using a tolerance

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector2D)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.DirectionEquals%28Autodesk.Navisworks.Api.Vector2D%29)

#### `bool DirectionEquals(Autodesk.Navisworks.Api.Vector2D vector, double tolerance)`

Compare direction of vectors using a tolerance

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector2D)
- `tolerance` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.DirectionEquals%28Autodesk.Navisworks.Api.Vector2D%2CSystem.Double%29)

#### `Autodesk.Navisworks.Api.Vector2D Divide(Autodesk.Navisworks.Api.Vector2D vector)`

Divides this vector by another to produce a new vector

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector2D)

**Returns:** `Autodesk.Navisworks.Api.Vector2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.Divide%28Autodesk.Navisworks.Api.Vector2D%29)

#### `Autodesk.Navisworks.Api.Vector2D Divide(double value)`

Divides every X, Y and Z coordinate in the Vector by a value and returns a new Vector with the result

**Parameters:**
- `value` (double)

**Returns:** `Autodesk.Navisworks.Api.Vector2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.Divide%28System.Double%29)

#### `double Dot(Autodesk.Navisworks.Api.Vector2D vector)`

Returns the dot product between this vector and another

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector2D)

**Returns:** `double` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.Dot%28Autodesk.Navisworks.Api.Vector2D%29)

#### `bool Equals(object obj)`

Determines whether the specified object and the current object refer to the same underlying native object. Returns false if either object has been disposed.

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.Equals%28System.Object%29)

#### `int GetHashCode()`

Serves as a hash function for a particular type. GetHashCode is suitable for use in hashing algorithms and data structures like a hash table.

**Returns:** `int` — A hash code for the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.GetHashCode)

#### `static Autodesk.Navisworks.Api.Vector2D InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of Vector2D.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Vector2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of Vector2D.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `Autodesk.Navisworks.Api.Vector2D Invert()`

Returns an inverted version of this vector (all components inverted)

**Returns:** `Autodesk.Navisworks.Api.Vector2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.Invert)

#### `Autodesk.Navisworks.Api.Vector2D Multiply(Autodesk.Navisworks.Api.Vector2D vector)`

Multiplies a vector by this to produce a new vector

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector2D)

**Returns:** `Autodesk.Navisworks.Api.Vector2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.Multiply%28Autodesk.Navisworks.Api.Vector2D%29)

#### `Autodesk.Navisworks.Api.Vector2D Multiply(double value)`

Multiplies every X, Y and Z coordinate in the Vector by a value and returns a new Vector with the result

**Parameters:**
- `value` (double)

**Returns:** `Autodesk.Navisworks.Api.Vector2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.Multiply%28System.Double%29)

#### `Autodesk.Navisworks.Api.Vector2D Negate()`

Returns a negated version of this vector (all components negated)

**Returns:** `Autodesk.Navisworks.Api.Vector2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.Negate)

#### `Autodesk.Navisworks.Api.Vector2D Normalize()`

Returns a normalized version of this vector (magnitude of 1)

**Returns:** `Autodesk.Navisworks.Api.Vector2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.Normalize)

#### `Autodesk.Navisworks.Api.Vector2D Subtract(Autodesk.Navisworks.Api.Vector2D vector)`

Subtracts a vector from this to produce a new vector

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector2D)

**Returns:** `Autodesk.Navisworks.Api.Vector2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.Subtract%28Autodesk.Navisworks.Api.Vector2D%29)

#### `Autodesk.Navisworks.Api.Vector2D Subtract(double value)`

Subtracts a value from every X, Y and Z coordinate in the Vector and returns a new Vector with the result

**Parameters:**
- `value` (double)

**Returns:** `Autodesk.Navisworks.Api.Vector2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.Subtract%28System.Double%29)

#### `Autodesk.Navisworks.Api.Point2D ToPoint2D()`

Convert vector to a point (equivalent to point from adding vector to origin)

**Returns:** `Autodesk.Navisworks.Api.Point2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.ToPoint2D)

#### `string ToString()`

Returns a String that represents the current Object.

**Returns:** `string` — A String that represents the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.ToString)

#### `bool TolerantEquals(Autodesk.Navisworks.Api.Vector2D vector)`

Compare vectors using a tolerance

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector2D)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.TolerantEquals%28Autodesk.Navisworks.Api.Vector2D%29)

#### `bool TolerantEquals(Autodesk.Navisworks.Api.Vector2D vector, double tolerance)`

Compare vectors using a tolerance

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector2D)
- `tolerance` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.TolerantEquals%28Autodesk.Navisworks.Api.Vector2D%2CSystem.Double%29)

#### `static bool TolerantEquals(Autodesk.Navisworks.Api.Vector2D vector0, Autodesk.Navisworks.Api.Vector2D vector1)`

Compare vectors using a tolerance

**Parameters:**
- `vector0` (Autodesk.Navisworks.Api.Vector2D)
- `vector1` (Autodesk.Navisworks.Api.Vector2D)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.TolerantEquals%28Autodesk.Navisworks.Api.Vector2D%2CAutodesk.Navisworks.Api.Vector2D%29)

#### `static bool TolerantEquals(Autodesk.Navisworks.Api.Vector2D vector0, Autodesk.Navisworks.Api.Vector2D vector1, double tolerance)`

Compare vectors using a tolerance

**Parameters:**
- `vector0` (Autodesk.Navisworks.Api.Vector2D)
- `vector1` (Autodesk.Navisworks.Api.Vector2D)
- `tolerance` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.TolerantEquals%28Autodesk.Navisworks.Api.Vector2D%2CAutodesk.Navisworks.Api.Vector2D%2CSystem.Double%29)

#### `static Autodesk.Navisworks.Api.Vector2D op_Addition(Autodesk.Navisworks.Api.Vector2D value1, Autodesk.Navisworks.Api.Vector2D value2)`

Adds two vectors to produce a new vector

**Parameters:**
- `value1` (Autodesk.Navisworks.Api.Vector2D)
- `value2` (Autodesk.Navisworks.Api.Vector2D)

**Returns:** `Autodesk.Navisworks.Api.Vector2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.op_Addition%28Autodesk.Navisworks.Api.Vector2D%2CAutodesk.Navisworks.Api.Vector2D%29)

#### `static Autodesk.Navisworks.Api.Vector2D op_Addition(Autodesk.Navisworks.Api.Vector2D vector, double value)`

Adds a value to every X, Y and Z coordinate in the Vector and returns a new Vector with the result

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector2D)
- `value` (double)

**Returns:** `Autodesk.Navisworks.Api.Vector2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.op_Addition%28Autodesk.Navisworks.Api.Vector2D%2CSystem.Double%29)

#### `static Autodesk.Navisworks.Api.Vector2D op_Addition(double value, Autodesk.Navisworks.Api.Vector2D vector)`

Adds a value to every X, Y and Z coordinate in the Vector and returns a new Vector with the result

**Parameters:**
- `value` (double)
- `vector` (Autodesk.Navisworks.Api.Vector2D)

**Returns:** `Autodesk.Navisworks.Api.Vector2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.op_Addition%28System.Double%2CAutodesk.Navisworks.Api.Vector2D%29)

#### `static Autodesk.Navisworks.Api.Vector2D op_Division(Autodesk.Navisworks.Api.Vector2D value1, Autodesk.Navisworks.Api.Vector2D value2)`

Divides two vectors to produce a new vector

**Parameters:**
- `value1` (Autodesk.Navisworks.Api.Vector2D)
- `value2` (Autodesk.Navisworks.Api.Vector2D)

**Returns:** `Autodesk.Navisworks.Api.Vector2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.op_Division%28Autodesk.Navisworks.Api.Vector2D%2CAutodesk.Navisworks.Api.Vector2D%29)

#### `static Autodesk.Navisworks.Api.Vector2D op_Division(Autodesk.Navisworks.Api.Vector2D vector, double value)`

Divides every X, Y and Z coordinate in the Vector by a value and returns a new Vector with the result

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector2D)
- `value` (double)

**Returns:** `Autodesk.Navisworks.Api.Vector2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.op_Division%28Autodesk.Navisworks.Api.Vector2D%2CSystem.Double%29)

#### `static Autodesk.Navisworks.Api.Vector2D op_Multiply(Autodesk.Navisworks.Api.Vector2D value1, Autodesk.Navisworks.Api.Vector2D value2)`

Multiplies two vectors to produce a new vector

**Parameters:**
- `value1` (Autodesk.Navisworks.Api.Vector2D)
- `value2` (Autodesk.Navisworks.Api.Vector2D)

**Returns:** `Autodesk.Navisworks.Api.Vector2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.op_Multiply%28Autodesk.Navisworks.Api.Vector2D%2CAutodesk.Navisworks.Api.Vector2D%29)

#### `static Autodesk.Navisworks.Api.Vector2D op_Multiply(Autodesk.Navisworks.Api.Vector2D vector, double value)`

Multiplies every X, Y and Z coordinate in the Vector by a value and returns a new Vector with the result

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector2D)
- `value` (double)

**Returns:** `Autodesk.Navisworks.Api.Vector2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.op_Multiply%28Autodesk.Navisworks.Api.Vector2D%2CSystem.Double%29)

#### `static Autodesk.Navisworks.Api.Vector2D op_Multiply(double value, Autodesk.Navisworks.Api.Vector2D vector)`

Multiplies every X, Y and Z coordinate in the Vector by a value and returns a new Vector with the result

**Parameters:**
- `value` (double)
- `vector` (Autodesk.Navisworks.Api.Vector2D)

**Returns:** `Autodesk.Navisworks.Api.Vector2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.op_Multiply%28System.Double%2CAutodesk.Navisworks.Api.Vector2D%29)

#### `static Autodesk.Navisworks.Api.Vector2D op_Subtraction(Autodesk.Navisworks.Api.Vector2D value1, Autodesk.Navisworks.Api.Vector2D value2)`

Subtracts two vectors to produce a new vector

**Parameters:**
- `value1` (Autodesk.Navisworks.Api.Vector2D)
- `value2` (Autodesk.Navisworks.Api.Vector2D)

**Returns:** `Autodesk.Navisworks.Api.Vector2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.op_Subtraction%28Autodesk.Navisworks.Api.Vector2D%2CAutodesk.Navisworks.Api.Vector2D%29)

#### `static Autodesk.Navisworks.Api.Vector2D op_Subtraction(Autodesk.Navisworks.Api.Vector2D vector, double value)`

Subtracts a value from every X, Y and Z coordinate in the Vector and returns a new Vector with the result

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector2D)
- `value` (double)

**Returns:** `Autodesk.Navisworks.Api.Vector2D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector2D.op_Subtraction%28Autodesk.Navisworks.Api.Vector2D%2CSystem.Double%29)

### Properties
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `IsZero` (bool, get) — Is this a zero vector (all components are zero) ?
- `Item` (double, get) — Item property of Vector2D.
- `Length` (double, get) — Magnitude of vector
- `LengthSquared` (double, get) — Magnitude of vector squared
- `UnitX` (Autodesk.Navisworks.Api.Vector2D, get) — Returns a unit vector in the X direction (1,0)
- `UnitY` (Autodesk.Navisworks.Api.Vector2D, get) — Returns a unit vector in the Y direction (0,1)
- `X` (double, get) — Returns the X component of the vector
- `Y` (double, get) — Returns the Y component of the vector
- `Zero` (Autodesk.Navisworks.Api.Vector2D, get) — Returns a zero vector (0,0)

## Vector3D (class)

Represents a direction with magnitude in 3D space. Immutable.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Vector3D)

### Constructors
- `Vector3D()` — Create a zero vector
- `Vector3D(double x, double y, double z)` — Create a vector from X, Y and Z components

### Methods
#### `Autodesk.Navisworks.Api.Point3D Add(Autodesk.Navisworks.Api.Point3D point)`

Translates a point by this vector and returns the resulting point

**Parameters:**
- `point` (Autodesk.Navisworks.Api.Point3D)

**Returns:** `Autodesk.Navisworks.Api.Point3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.Add%28Autodesk.Navisworks.Api.Point3D%29)

#### `Autodesk.Navisworks.Api.Vector3D Add(Autodesk.Navisworks.Api.Vector3D vector)`

Adds another vector to this to produce a new vector

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector3D)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.Add%28Autodesk.Navisworks.Api.Vector3D%29)

#### `Autodesk.Navisworks.Api.Vector3D Add(double value)`

Adds a value to every X, Y and Z coordinate in the Vector and returns a new Vector with the result

**Parameters:**
- `value` (double)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.Add%28System.Double%29)

#### `double Angle(Autodesk.Navisworks.Api.Vector3D vector)`

Returns the angle between this vector and another vector in the range [0, PI]

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector3D) — vector to calculate angle with

**Returns:** `double` — angle in radians between this and vector

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.Angle%28Autodesk.Navisworks.Api.Vector3D%29)

#### `double AngleWithReference(Autodesk.Navisworks.Api.Vector3D vector, Autodesk.Navisworks.Api.Vector3D reference)`

Returns the angle between this vector and another vector in the range [-PI, PI], using a reference vector to determine the sign of the angle

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector3D) — vector to calculate angle with
- `reference` (Autodesk.Navisworks.Api.Vector3D) — reference vector pointing away from the plane containing this and vector

**Returns:** `double` — angle in radians between this and vector

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.AngleWithReference%28Autodesk.Navisworks.Api.Vector3D%2CAutodesk.Navisworks.Api.Vector3D%29)

#### `Autodesk.Navisworks.Api.Vector3D Cross(Autodesk.Navisworks.Api.Vector3D vector)`

Returns the cross product between this vector and another

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector3D)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.Cross%28Autodesk.Navisworks.Api.Vector3D%29)

#### `bool DirectionEquals(Autodesk.Navisworks.Api.UnitVector3D vector)`

Compare direction of vectors using a tolerance

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.UnitVector3D)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.DirectionEquals%28Autodesk.Navisworks.Api.UnitVector3D%29)

#### `bool DirectionEquals(Autodesk.Navisworks.Api.UnitVector3D vector, double tolerance)`

Compare direction of vectors using a tolerance

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.UnitVector3D)
- `tolerance` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.DirectionEquals%28Autodesk.Navisworks.Api.UnitVector3D%2CSystem.Double%29)

#### `bool DirectionEquals(Autodesk.Navisworks.Api.Vector3D vector)`

Compare direction of vectors using a tolerance

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector3D)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.DirectionEquals%28Autodesk.Navisworks.Api.Vector3D%29)

#### `bool DirectionEquals(Autodesk.Navisworks.Api.Vector3D vector, double tolerance)`

Compare direction of vectors using a tolerance

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector3D)
- `tolerance` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.DirectionEquals%28Autodesk.Navisworks.Api.Vector3D%2CSystem.Double%29)

#### `Autodesk.Navisworks.Api.Vector3D Divide(Autodesk.Navisworks.Api.Vector3D vector)`

Divides this vector by another to produce a new vector

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector3D)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.Divide%28Autodesk.Navisworks.Api.Vector3D%29)

#### `Autodesk.Navisworks.Api.Vector3D Divide(double value)`

Divides every X, Y and Z coordinate in the Vector by a value and returns a new Vector with the result

**Parameters:**
- `value` (double)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.Divide%28System.Double%29)

#### `double Dot(Autodesk.Navisworks.Api.Vector3D vector)`

Returns the dot product between this vector and another

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector3D)

**Returns:** `double` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.Dot%28Autodesk.Navisworks.Api.Vector3D%29)

#### `bool Equals(object obj)`

Determines whether the specified object and the current object refer to the same underlying native object. Returns false if either object has been disposed.

**Parameters:**
- `obj` (object)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.Equals%28System.Object%29)

#### `int GetHashCode()`

Serves as a hash function for a particular type. GetHashCode is suitable for use in hashing algorithms and data structures like a hash table.

**Returns:** `int` — A hash code for the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.GetHashCode)

#### `static Autodesk.Navisworks.Api.Vector3D InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of Vector3D.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of Vector3D.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `Autodesk.Navisworks.Api.Vector3D Invert()`

Returns an inverted version of this vector (all components inverted)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.Invert)

#### `Autodesk.Navisworks.Api.Vector3D Multiply(Autodesk.Navisworks.Api.Matrix3 matrix)`

Multiply method of Vector3D.

**Parameters:**
- `matrix` (Autodesk.Navisworks.Api.Matrix3)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.Multiply%28Autodesk.Navisworks.Api.Matrix3%29)

#### `Autodesk.Navisworks.Api.Vector3D Multiply(Autodesk.Navisworks.Api.Vector3D vector)`

Multiply method of Vector3D.

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector3D)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.Multiply%28Autodesk.Navisworks.Api.Vector3D%29)

#### `Autodesk.Navisworks.Api.Vector3D Multiply(double value)`

Multiply method of Vector3D.

**Parameters:**
- `value` (double)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.Multiply%28System.Double%29)

#### `Autodesk.Navisworks.Api.Vector3D Negate()`

Returns a negated version of this vector (all components negated)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.Negate)

#### `Autodesk.Navisworks.Api.Vector3D Normalize()`

Returns a normalized version of this vector (magnitude of 1)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.Normalize)

#### `Autodesk.Navisworks.Api.Vector3D Subtract(Autodesk.Navisworks.Api.Vector3D vector)`

Subtracts a vector from this to produce a new vector

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector3D)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.Subtract%28Autodesk.Navisworks.Api.Vector3D%29)

#### `Autodesk.Navisworks.Api.Vector3D Subtract(double value)`

Subtracts a value from every X, Y and Z coordinate in the Vector and returns a new Vector with the result

**Parameters:**
- `value` (double)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.Subtract%28System.Double%29)

#### `Autodesk.Navisworks.Api.Color ToColor()`

Convert vector to a color

**Returns:** `Autodesk.Navisworks.Api.Color` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.ToColor)

#### `Autodesk.Navisworks.Api.Point3D ToPoint3D()`

Convert vector to a point (equivalent to point from adding vector to origin)

**Returns:** `Autodesk.Navisworks.Api.Point3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.ToPoint3D)

#### `string ToString()`

Returns a String that represents the current Object.

**Returns:** `string` — A String that represents the current Object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.ToString)

#### `bool TolerantEquals(Autodesk.Navisworks.Api.Vector3D vector)`

Compare vectors using a tolerance

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector3D)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.TolerantEquals%28Autodesk.Navisworks.Api.Vector3D%29)

#### `bool TolerantEquals(Autodesk.Navisworks.Api.Vector3D vector, double tolerance)`

Compare vectors using a tolerance

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector3D)
- `tolerance` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.TolerantEquals%28Autodesk.Navisworks.Api.Vector3D%2CSystem.Double%29)

#### `static bool TolerantEquals(Autodesk.Navisworks.Api.Vector3D vector0, Autodesk.Navisworks.Api.Vector3D vector1)`

Compare vectors using a tolerance

**Parameters:**
- `vector0` (Autodesk.Navisworks.Api.Vector3D)
- `vector1` (Autodesk.Navisworks.Api.Vector3D)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.TolerantEquals%28Autodesk.Navisworks.Api.Vector3D%2CAutodesk.Navisworks.Api.Vector3D%29)

#### `static bool TolerantEquals(Autodesk.Navisworks.Api.Vector3D vector0, Autodesk.Navisworks.Api.Vector3D vector1, double tolerance)`

Compare vectors using a tolerance

**Parameters:**
- `vector0` (Autodesk.Navisworks.Api.Vector3D)
- `vector1` (Autodesk.Navisworks.Api.Vector3D)
- `tolerance` (double)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.TolerantEquals%28Autodesk.Navisworks.Api.Vector3D%2CAutodesk.Navisworks.Api.Vector3D%2CSystem.Double%29)

#### `static Autodesk.Navisworks.Api.Vector3D op_Addition(Autodesk.Navisworks.Api.Vector3D value1, Autodesk.Navisworks.Api.Vector3D value2)`

Adds two vectors to produce a new vector

**Parameters:**
- `value1` (Autodesk.Navisworks.Api.Vector3D)
- `value2` (Autodesk.Navisworks.Api.Vector3D)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.op_Addition%28Autodesk.Navisworks.Api.Vector3D%2CAutodesk.Navisworks.Api.Vector3D%29)

#### `static Autodesk.Navisworks.Api.Vector3D op_Addition(Autodesk.Navisworks.Api.Vector3D vector, double value)`

Adds a value to every X, Y and Z coordinate in the Vector and returns a new Vector with the result

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector3D)
- `value` (double)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.op_Addition%28Autodesk.Navisworks.Api.Vector3D%2CSystem.Double%29)

#### `static Autodesk.Navisworks.Api.Vector3D op_Addition(double value, Autodesk.Navisworks.Api.Vector3D vector)`

Adds a value to every X, Y and Z coordinate in the Vector and returns a new Vector with the result

**Parameters:**
- `value` (double)
- `vector` (Autodesk.Navisworks.Api.Vector3D)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.op_Addition%28System.Double%2CAutodesk.Navisworks.Api.Vector3D%29)

#### `static Autodesk.Navisworks.Api.Vector3D op_Division(Autodesk.Navisworks.Api.Vector3D value1, Autodesk.Navisworks.Api.Vector3D value2)`

Divides two vectors to produce a new vector

**Parameters:**
- `value1` (Autodesk.Navisworks.Api.Vector3D)
- `value2` (Autodesk.Navisworks.Api.Vector3D)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.op_Division%28Autodesk.Navisworks.Api.Vector3D%2CAutodesk.Navisworks.Api.Vector3D%29)

#### `static Autodesk.Navisworks.Api.Vector3D op_Division(Autodesk.Navisworks.Api.Vector3D vector, double value)`

Divides every X, Y and Z coordinate in the Vector by a value and returns a new Vector with the result

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector3D)
- `value` (double)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.op_Division%28Autodesk.Navisworks.Api.Vector3D%2CSystem.Double%29)

#### `static Autodesk.Navisworks.Api.Vector3D op_Multiply(Autodesk.Navisworks.Api.Vector3D value1, Autodesk.Navisworks.Api.Vector3D value2)`

Multiplies two vectors to produce a new vector

**Parameters:**
- `value1` (Autodesk.Navisworks.Api.Vector3D)
- `value2` (Autodesk.Navisworks.Api.Vector3D)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.op_Multiply%28Autodesk.Navisworks.Api.Vector3D%2CAutodesk.Navisworks.Api.Vector3D%29)

#### `static Autodesk.Navisworks.Api.Vector3D op_Multiply(Autodesk.Navisworks.Api.Vector3D vector, double value)`

Multiplies every X, Y and Z coordinate in the Vector by a value and returns a new Vector with the result

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector3D)
- `value` (double)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.op_Multiply%28Autodesk.Navisworks.Api.Vector3D%2CSystem.Double%29)

#### `static Autodesk.Navisworks.Api.Vector3D op_Multiply(double value, Autodesk.Navisworks.Api.Vector3D vector)`

Multiplies every X, Y and Z coordinate in the Vector by a value and returns a new Vector with the result

**Parameters:**
- `value` (double)
- `vector` (Autodesk.Navisworks.Api.Vector3D)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.op_Multiply%28System.Double%2CAutodesk.Navisworks.Api.Vector3D%29)

#### `static Autodesk.Navisworks.Api.Vector3D op_Subtraction(Autodesk.Navisworks.Api.Vector3D value1, Autodesk.Navisworks.Api.Vector3D value2)`

Subtracts two vectors to produce a new vector

**Parameters:**
- `value1` (Autodesk.Navisworks.Api.Vector3D)
- `value2` (Autodesk.Navisworks.Api.Vector3D)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.op_Subtraction%28Autodesk.Navisworks.Api.Vector3D%2CAutodesk.Navisworks.Api.Vector3D%29)

#### `static Autodesk.Navisworks.Api.Vector3D op_Subtraction(Autodesk.Navisworks.Api.Vector3D vector, double value)`

Subtracts a value from every X, Y and Z coordinate in the Vector and returns a new Vector with the result

**Parameters:**
- `vector` (Autodesk.Navisworks.Api.Vector3D)
- `value` (double)

**Returns:** `Autodesk.Navisworks.Api.Vector3D` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Vector3D.op_Subtraction%28Autodesk.Navisworks.Api.Vector3D%2CSystem.Double%29)

### Properties
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `IsZero` (bool, get) — Is this a zero vector (all components are zero) ?
- `Item` (double, get) — Item property of Vector3D.
- `Length` (double, get) — Magnitude of vector
- `LengthSquared` (double, get) — Magnitude of vector squared
- `UnitX` (Autodesk.Navisworks.Api.Vector3D, get) — Returns a unit vector in the X direction (1,0,0)
- `UnitY` (Autodesk.Navisworks.Api.Vector3D, get) — Returns a unit vector in the Y direction (0,1,0)
- `UnitZ` (Autodesk.Navisworks.Api.Vector3D, get) — Returns a unit vector in the Z direction (0,0,1)
- `X` (double, get) — Returns the X component of the vector
- `Y` (double, get) — Returns the Y component of the vector
- `Z` (double, get) — Returns the Z component of the vector
- `Zero` (Autodesk.Navisworks.Api.Vector3D, get) — Returns a zero vector (0,0,0)

## VertexProperties (enum)

Vertex properties enumeration.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.VertexProperties)

### Values
- `None` = `0` — None
- `HasNormals` = `1` — Normals
- `HasColors` = `2` — Colours
- `HasTextureCoordinates` = `4` — Texture co-ordinates

## View (class)

Represents a view of a document.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.View)

### Methods
#### `void CopyViewpointFrom(Autodesk.Navisworks.Api.Viewpoint viewpoint, Autodesk.Navisworks.Api.ViewChange change)`

Updates the viewpoint in this view based on change required.

**Parameters:**
- `viewpoint` (Autodesk.Navisworks.Api.Viewpoint)
- `change` (Autodesk.Navisworks.Api.ViewChange)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.View.CopyViewpointFrom%28Autodesk.Navisworks.Api.Viewpoint%2CAutodesk.Navisworks.Api.ViewChange%29)

#### `Autodesk.Navisworks.Api.Viewpoint CreateViewpointCopy()`

Returns a viewpoint representing this view.

**Returns:** `Autodesk.Navisworks.Api.Viewpoint` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.View.CreateViewpointCopy)

#### `void Dispose()`

Dispose method of View.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.View.Dispose)

#### `void EndInteractive()`

Decrements an internal counter that determines if the view is in interactive mode. As the count goes from 1 to 0 the View is set out of interactive mode. Should be used with a matched call to StartInteractive.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.View.EndInteractive)

#### `void FocusOnCurrentSelection()`

Focuses the view on the currently selected objects.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.View.FocusOnCurrentSelection)

#### `System.Drawing.Bitmap GenerateImage(Autodesk.Navisworks.Api.ImageGenerationStyle style, int width, int height, bool enableSectioning)`

Generates an Image for this View.

**Parameters:**
- `style` (Autodesk.Navisworks.Api.ImageGenerationStyle)
- `width` (int)
- `height` (int)
- `enableSectioning` (bool)

**Returns:** `System.Drawing.Bitmap` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.View.GenerateImage%28Autodesk.Navisworks.Api.ImageGenerationStyle%2CSystem.Int32%2CSystem.Int32%2CSystem.Boolean%29)

#### `System.Drawing.Bitmap GenerateImage(Autodesk.Navisworks.Api.ImageGenerationStyle style, int width, int height, double maxTimeHint, bool enableSectioning)`

Generates an Image for this View.

**Parameters:**
- `style` (Autodesk.Navisworks.Api.ImageGenerationStyle)
- `width` (int)
- `height` (int)
- `maxTimeHint` (double)
- `enableSectioning` (bool)

**Returns:** `System.Drawing.Bitmap` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.View.GenerateImage%28Autodesk.Navisworks.Api.ImageGenerationStyle%2CSystem.Int32%2CSystem.Int32%2CSystem.Double%2CSystem.Boolean%29)

#### `string GetClippingPlanes()`

Gets clipping planes in the form of a JSON ClipPlaneSet object.

**Returns:** `string` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.View.GetClippingPlanes)

#### `string GetRedlines()`

Gets redlines in the form of a JSON RedlineCollection object.

**Returns:** `string` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.View.GetRedlines)

#### `void LookFromFrontRightTop()`

Move's the Camera to look from FrontRightTop.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.View.LookFromFrontRightTop)

#### `Autodesk.Navisworks.Api.PickItemResult PickItemFromPoint(int x, int y)`

Picks a single item from the given window coordinate point.

**Parameters:**
- `x` (int)
- `y` (int)

**Returns:** `Autodesk.Navisworks.Api.PickItemResult` — A PickItemResult object that contains the picked item.
Returns null if nothing is picked or coordinates are invalid.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.View.PickItemFromPoint%28System.Int32%2CSystem.Int32%29)

#### `Autodesk.Navisworks.Api.PickItemResult PickItemFromPoint(int x, int y, int pickingWidth, bool asRendered, Autodesk.Navisworks.Api.PickPrimitives pickPrimitives)`

Picks a single item from the given window coordinate point.

**Parameters:**
- `x` (int)
- `y` (int)
- `pickingWidth` (int) — Width of picking region in pixels. Normally 1.
- `asRendered` (bool) — Whether picking should operate only on items that were rendered during the last frame. Normally true.
- `pickPrimitives` (Autodesk.Navisworks.Api.PickPrimitives)

**Returns:** `Autodesk.Navisworks.Api.PickItemResult` — A PickItemResult object that contains the picked item.
Returns null if nothing is picked or coordinates are invalid.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.View.PickItemFromPoint%28System.Int32%2CSystem.Int32%2CSystem.Int32%2CSystem.Boolean%2CAutodesk.Navisworks.Api.PickPrimitives%29)

#### `Autodesk.Navisworks.Api.PickItemResult PickItemFromRectangle(int x, int y, int width, int height)`

Picks a single item from the given window coordinate rectangle.

**Parameters:**
- `x` (int)
- `y` (int)
- `width` (int)
- `height` (int)

**Returns:** `Autodesk.Navisworks.Api.PickItemResult` — A PickItemResult object that contains the picked item
Returns null if nothing is picked or coordinates are invalid.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.View.PickItemFromRectangle%28System.Int32%2CSystem.Int32%2CSystem.Int32%2CSystem.Int32%29)

#### `Autodesk.Navisworks.Api.PickItemResult PickItemFromRectangle(int x, int y, int width, int height, bool asRendered, Autodesk.Navisworks.Api.PickPrimitives pickPrimitives)`

Picks a single item from the given window coordinate rectangle.

**Parameters:**
- `x` (int)
- `y` (int)
- `width` (int)
- `height` (int)
- `asRendered` (bool) — Whether picking should operate only on items that were rendered during the last frame. Normally true.
- `pickPrimitives` (Autodesk.Navisworks.Api.PickPrimitives)

**Returns:** `Autodesk.Navisworks.Api.PickItemResult` — A PickItemResult object that contains the picked item
Returns null if nothing is picked or coordinates are invalid.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.View.PickItemFromRectangle%28System.Int32%2CSystem.Int32%2CSystem.Int32%2CSystem.Int32%2CSystem.Boolean%2CAutodesk.Navisworks.Api.PickPrimitives%29)

#### `Autodesk.Navisworks.Api.ModelItemCollection PickItemsFromRectangle(int x, int y, int width, int height, bool containedOnly)`

Picks multiple items from the given window coordinate rectangle.

**Parameters:**
- `x` (int)
- `y` (int)
- `width` (int)
- `height` (int)
- `containedOnly` (bool) — Whether picking should operate only on items that are contained in the rectangle

**Returns:** `Autodesk.Navisworks.Api.ModelItemCollection` — A ModelItemCollection containing the picked items.
Returns null if nothing is picked or coordinates are invalid.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.View.PickItemsFromRectangle%28System.Int32%2CSystem.Int32%2CSystem.Int32%2CSystem.Int32%2CSystem.Boolean%29)

#### `Autodesk.Navisworks.Api.ModelItemCollection PickItemsFromRectangle(int x, int y, int width, int height, bool containedOnly, bool asRendered)`

Picks multiple items from the given window coordinate rectangle.

**Parameters:**
- `x` (int)
- `y` (int)
- `width` (int)
- `height` (int)
- `containedOnly` (bool) — Whether picking should operate only on items that are contained in the rectangle.
- `asRendered` (bool) — Whether picking should operate only on items that were rendered during the last frame. Normally true.

**Returns:** `Autodesk.Navisworks.Api.ModelItemCollection` — A ModelItemCollection containing the picked items.
Returns null if nothing is picked or coordinates are invalid.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.View.PickItemsFromRectangle%28System.Int32%2CSystem.Int32%2CSystem.Int32%2CSystem.Int32%2CSystem.Boolean%2CSystem.Boolean%29)

#### `Autodesk.Navisworks.Api.ProjectionResult ProjectPoint(Autodesk.Navisworks.Api.Point3D point, bool sectionClip, bool frustumClip)`

Converts a point in world space into view coordinates.

**Parameters:**
- `point` (Autodesk.Navisworks.Api.Point3D)
- `sectionClip` (bool)
- `frustumClip` (bool)

**Returns:** `Autodesk.Navisworks.Api.ProjectionResult` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.View.ProjectPoint%28Autodesk.Navisworks.Api.Point3D%2CSystem.Boolean%2CSystem.Boolean%29)

#### `void RequestDelayedRedraw(Autodesk.Navisworks.Api.ViewRedrawRequests redrawRequests)`

Asks for Render after all other events have been processed.

**Parameters:**
- `redrawRequests` (Autodesk.Navisworks.Api.ViewRedrawRequests)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.View.RequestDelayedRedraw%28Autodesk.Navisworks.Api.ViewRedrawRequests%29)

#### `void SetClippingPlanes(string jsonClipPlaneSet)`

Sets clipping planes.

**Parameters:**
- `jsonClipPlaneSet` (string) — Clipping planes in the form of a JSON ClipPlaneSet object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.View.SetClippingPlanes%28System.String%29)

#### `void SetRedlines(string jsonRedlineCollection)`

Sets redlines.

**Parameters:**
- `jsonRedlineCollection` (string) — Redlines in the form of a JSON RedlineCollection object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.View.SetRedlines%28System.String%29)

#### `void StartInteractive()`

Increments an internal counter that determines if the view is in interactive mode. As the count goes from 0 to 1 the View is set into interactive mode. Should be used with a matched call to EndInteractive.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.View.StartInteractive)

#### `bool TrySetClippingPlanes(string jsonClipPlaneSet)`

Sets clipping planes.

**Parameters:**
- `jsonClipPlaneSet` (string) — Clipping planes in the form of a JSON ClipPlaneSet object.

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.View.TrySetClippingPlanes%28System.String%29)

#### `bool TrySetRedlines(string jsonRedlineCollection)`

Sets redlines.

**Parameters:**
- `jsonRedlineCollection` (string) — Redlines in the form of a JSON RedlineCollection object.

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.View.TrySetRedlines%28System.String%29)

### Properties
- `Document` (Autodesk.Navisworks.Api.Document, get) — The document associated with this View.
- `Height` (int, get) — The renderable height of the view in Windows screen pixels
- `IsDisposed` (bool, get) — Has this view been disposed ?
- `IsInteractive` (bool, get) — Determines if the view is in interactive mode.
- `Viewer` (Autodesk.Navisworks.Api.Interop.LcOpKernelExViewer, get) — Viewer property of View.
- `Width` (int, get) — The renderable width of the view in Windows screen pixels

## ViewChange (enum)

Specifies type of change being made to a View.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ViewChange)

### Values
- `JumpCut` = `0` — The current viewpoint jumps straight to the new viewpoint. Collision detection and gravity have no effect.
- `Navigation` = `1` — User is interactively navigating from current viewpoint to new viewpoint. Actual viewpoint set may be modified due to collision detection and gravity.

## ViewEventArgs (class)

Event arguments used for View related events

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ViewEventArgs)

### Properties
- `View` (Autodesk.Navisworks.Api.View, get/set) — Gets the View that the event is describing.

## ViewRedrawRequests (enum)

Flags specifying which sorts of redraw are requested.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ViewRedrawRequests)

### Values
- `None` = `0` — No redraw requested unless other flags specified.
- `Render` = `1` — Main Render requested.
- `OverlayRender` = `2` — Overlay Render requested.
- `All` = `3` — Main Render and Overlay Render requested.

## Viewpoint (class)

Represents a viewpoint

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.Viewpoint)

### Constructors
- `Viewpoint()` — Default constructor

### Methods
#### `void AlignDirection(Autodesk.Navisworks.Api.Vector3D direction)`

Rotate camera so that it points in given direction using shortest rotation path.

**Parameters:**
- `direction` (Autodesk.Navisworks.Api.Vector3D)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Viewpoint.AlignDirection%28Autodesk.Navisworks.Api.Vector3D%29)

#### `void AlignUp(Autodesk.Navisworks.Api.Vector3D up)`

Rotate camera about view direction so that camera up vector is aligned with given up vector (in same plane as view direction and given up vector).

**Parameters:**
- `up` (Autodesk.Navisworks.Api.Vector3D)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Viewpoint.AlignUp%28Autodesk.Navisworks.Api.Vector3D%29)

#### `Autodesk.Navisworks.Api.Viewpoint CreateCopy()`

Creates a copy of the Viewpoint.

**Returns:** `Autodesk.Navisworks.Api.Viewpoint` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Viewpoint.CreateCopy)

#### `string GetCamera()`

Describes viewpoint in terms of an JSON AutodeskCommonCamera object.

**Returns:** `string` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Viewpoint.GetCamera)

#### `static Autodesk.Navisworks.Api.Viewpoint InternalCreator(nint handleIn, Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership ownership, Autodesk.Navisworks.Api.NativeHandle parent)`

InternalCreator method of Viewpoint.

**Parameters:**
- `handleIn` (nint)
- `ownership` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership)
- `parent` (Autodesk.Navisworks.Api.NativeHandle)

**Returns:** `Autodesk.Navisworks.Api.Viewpoint` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Viewpoint.InternalCreator%28System.IntPtr%2CAutodesk.Navisworks.Internal.ApiImplementation.NativeHandleOwnership%2CAutodesk.Navisworks.Api.NativeHandle%29)

#### `static Autodesk.Navisworks.Api.NativeHandle InternalFactory(ref Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit reserved)`

InternalFactory method of Viewpoint.

**Parameters:**
- `reserved` (Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit)

**Returns:** `Autodesk.Navisworks.Api.NativeHandle` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Viewpoint.InternalFactory%28Autodesk.Navisworks.Internal.ApiImplementation.NativeHandleInit%40%29)

#### `void PointAt(Autodesk.Navisworks.Api.Point3D to)`

Rotate camera so that it points at given location using shortest rotation path.

**Parameters:**
- `to` (Autodesk.Navisworks.Api.Point3D)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Viewpoint.PointAt%28Autodesk.Navisworks.Api.Point3D%29)

#### `void SetCamera(string cameraJson)`

Set viewpoint from JSON AutodeskCommonCamera.

**Parameters:**
- `cameraJson` (string) — Camera in the form of a JSON AutodeskCommonCamera object.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Viewpoint.SetCamera%28System.String%29)

#### `void SetExtentsAtFocalDistance(double horizontal, double vertical)`

Set the distances between the left and right planes, and the top and bottom planes of the view frustum at the focal distance along the view direction from the camera position. Need to call SetFocalDistance before calling this method.

**Parameters:**
- `horizontal` (double)
- `vertical` (double)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Viewpoint.SetExtentsAtFocalDistance%28System.Double%2CSystem.Double%29)

#### `bool TrySetCamera(string cameraJson)`

Try to set viewpoint from JSON AutodeskCommonCamera.

**Parameters:**
- `cameraJson` (string) — Camera in the form of a JSON AutodeskCommonCamera.

**Returns:** `bool` — false if operation fails

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Viewpoint.TrySetCamera%28System.String%29)

#### `void ZoomBox(Autodesk.Navisworks.Api.BoundingBox3D box)`

Zooms the viewpoint to view a given bounding box.

**Parameters:**
- `box` (Autodesk.Navisworks.Api.BoundingBox3D) — The bounding box to zoom to view.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.Viewpoint.ZoomBox%28Autodesk.Navisworks.Api.BoundingBox3D%29)

### Properties
- `AngularSpeed` (double, get/set) — Angular speed of the camera.
- `ApertureDiameter` (double, get/set) — The diameter of the physical camera aperture in camera units. Used for advanced rendering effects.
- `AspectRatio` (double, get/set) — Aspect ratio of camera (width/height). Default 1.0 (square).
- `ClipPlanes` (Autodesk.Navisworks.Api.ClipPlaneSet, get/set) — ClipPlanes property of Viewpoint.
- `FarDistance` (double, get/set) — Distance from eye point to far plane along view direction. Must be greater than near distance.
- `FarDistanceType` (Autodesk.Navisworks.Api.ViewpointDistanceType, get/set) — Describes the users intentions if the product has to modify the far distance (typically because of graphics system limitations).
- `FarPlaneDistance` (double, get/set) — Far plane distance.
- `FocalDistance` (double, get/set) — The focal distance of the camera.
- `HasAngularSpeed` (bool, get) — Determines if AngularSpeed has been set
- `HasFocalDistance` (bool, get) — Determines if FocalDistance has been set
- `HasLighting` (bool, get) — Determines if Lighting has been set
- `HasLinearSpeed` (bool, get) — Determines if LinearSpeed has been set
- `HasRenderStyle` (bool, get) — Determines if RenderStyle has been set
- `HasTool` (bool, get) — Determines if Tool has been set
- `HasViewer` (bool, get) — Determines if Viewer has been set.
- `HasWorldUpVector` (bool, get) — Determines if WorldUpVector has been set
- `HeightField` (double, get/set) — Defines camera field of view in combination with aspect ratio. For a perspective camera, this is the vertical field of view (the angle between top and bottom planes of the camera view frustum). For an orthographic camera, this is the distance between top and bottom planes of the camera view frustum. Default 0.785398 (45 degrees)
- `HorizontalExtentAtFocalDistance` (double, get) — The distance between the left and right planes of the view frustum at the focal distance along the view direction from the camera position.
- `HorizontalScale` (double, get/set) — Required horizontal scaling of the image when adapting the camera view to the display window.
- `ImageFit` (Autodesk.Navisworks.Api.ViewpointImageFit, get/set) — Describes how to adapt the camera view so that it's aspect ratio matches that of the display window.
- `InternalViewpoint` (Autodesk.Navisworks.Api.Interop.LcNvViewPoint, get) — InternalViewpoint property of Viewpoint.
- `IsReadOnly` (bool, get) — Gets a value indicating whether the object is read-only.
- `Lighting` (Autodesk.Navisworks.Api.ViewpointLighting, get/set) — Lighting used for this viewpoint
- `LinearSpeed` (double, get/set) — Linear speed of the camera.
- `NearDistance` (double, get/set) — Distance from eye point to near plane along view direction. Must be greater than zero.
- `NearDistanceType` (Autodesk.Navisworks.Api.ViewpointDistanceType, get/set) — Describes the users intentions if the product has to modify the near distance (typically because of graphics system limitations).
- `NearPlaneDistance` (double, get/set) — Near plane distance.
- `Position` (Autodesk.Navisworks.Api.Point3D, get/set) — Position of the camera.
- `Projection` (Autodesk.Navisworks.Api.ViewpointProjection, get/set) — The projection type of the current viewpoint.
- `RenderStyle` (Autodesk.Navisworks.Api.ViewpointRenderStyle, get/set) — The style used for rendering.
- `RightOffsetAtFocalDistance` (double, get/set) — Distance that the plane normal to the view direction, at the focal distance along the view direction from the camera position, is offset to the right. Need to call SetFocalDistance before using this property.
- `RightOffsetFactor` (double, get/set) — The ratio between the right offset (the distance that the target plane is offset to the right) and the horizontal extent (the distance between the left and right planes of the view frustum at the target distance).
- `Rotation` (Autodesk.Navisworks.Api.Rotation3D, get/set) — Rotation of camera from base orientation. The base orientation is looking down the negative Z-axis with +X to the right and +Y up. Default 0 0 0 1 (no rotation).
- `ShutterSpeed` (double, get/set) — The time measured in seconds that the physical aperture is open. Used for advanced rendering effects.
- `Tool` (Autodesk.Navisworks.Api.Tool, get/set) — The Tool in use.
- `UpOffsetAtFocalDistance` (double, get/set) — Distance that the plane normal to the view direction, at the focal distance along the view direction from the camera position, is offset upwards. Need to call SetFocalDistance before using this property.
- `UpOffsetFactor` (double, get/set) — The ratio between the up offset (the distance that the target plane is offset upwards) and the vertical extent (the distance between the top and bottom planes of the view frustum at the target distance).
- `VerticalExtentAtFocalDistance` (double, get) — The distance between the top and bottom planes of the view frustum at the focal distance along the view direction from the camera position.
- `ViewerAvatar` (string, get/set) — Avatar used for this viewpoint.
- `ViewerCameraMode` (Autodesk.Navisworks.Api.CameraMode, get/set) — Avatar used for this viewpoint.
- `WorldUpVector` (Autodesk.Navisworks.Api.UnitVector3D, get/set) — Preferred up vector. Used by "walk" paradigm navigation methods to move on a plane.

## ViewpointDistanceType (enum)

Describes the user's intentions if the product has to modify near and far distances (typically because of graphics system limitations).

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ViewpointDistanceType)

### Values
- `Fixed` = `0` — User has specified exact distance.
- `Constrained` = `1` — For near distance, user has specified maximum distance and the product can use a smaller value to increase render quality. For far distance, user has specified minimum distance and the product can use a larger value to increase render quality.
- `Auto` = `2` — Distance was chosen automatically by the product.

## ViewpointImageFit (enum)

Describes how to adapt the camera view so that its aspect ratio matches that of the display window.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ViewpointImageFit)

### Values
- `Mask` = `0` — The frustum is used as is within the largest subset of the window that has the correct aspect ratio. The rest of the window is masked out (e.g. black bars).
- `Fill` = `1` — The frustum extents are enlarged so that all of the original image is visible.
- `Vertical` = `2` — Only HorizontalExtent is modified so that vertical extent is preserved.
- `Horizontal` = `3` — Only VerticalExtent is modified so that horizontal extent is preserved.
- `Overscan` = `4` — The frustum extents are reduced so that a subset of the original image is visible.

## ViewpointLighting (enum)

The lighting of the viewpoint

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ViewpointLighting)

### Values
- `None` = `0` — Turns off lighting completely
- `SceneLights` = `1` — Lights provided by user in model
- `Headlight` = `2` — An automatic headlight provided by viewer
- `FullLights` = `3` — Advanced level of lighting, implemented by application

## ViewpointProjection (enum)

The projection type of the viewpoint

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ViewpointProjection)

### Values
- `Perspective` = `0` — Perspective viewpoint
- `Orthographic` = `1` — Orthographic viewpoint

## ViewpointRenderStyle (enum)

The render style of the viewpoint

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ViewpointRenderStyle)

### Values
- `FullRender` = `0` — Highest quality, as specified in model
- `Preview` = `1` — Lower quality, same appearance
- `Shaded` = `2` — Smooth shaded (simple shade model, no textures)
- `Wireframe` = `3` — Wire frame
- `HiddenLine` = `4` — Hidden line wire frame

## ViewpointStereoProjection (enum)

The stereo projection type of the viewpoint.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.ViewpointStereoProjection)

### Values
- `Mono` = `0` — Standard mono camera
- `Left` = `1` — Left eye for stereo
- `Right` = `2` — Right eye for stereo

## VisibilityOverrides (class)

Visibility Overrides.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.VisibilityOverrides)

### Constructors
- `VisibilityOverrides(Autodesk.Navisworks.Api.ModelItemCollection hidden)` — Visibility Overrides.

### Properties
- `Hidden` (Autodesk.Navisworks.Api.ModelItemCollection, get) — Visibility Overrides.

