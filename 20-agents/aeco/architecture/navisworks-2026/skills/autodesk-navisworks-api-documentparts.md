---
name: navisworks-autodesk-navisworks-api-documentparts
description: This skill encodes the navisworks 2026.0 surface of the Autodesk.Navisworks.Api.DocumentParts namespace — 18 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: DocumentAlignments, DocumentCurrentComments, DocumentCurrentMeasurement, DocumentCurrentSearch, DocumentCurrentSelection, DocumentCurrentViewpoint, DocumentDatabase, DocumentGrids, and 10 more types.
---

# Autodesk.Navisworks.Api.DocumentParts

Auto-generated from vendor docs for navisworks 2026.0. 18 types in this namespace.

## DocumentAlignments (class)

Stores the Civil Alignment information contained in a Document.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.DocumentParts.DocumentAlignments)

### Methods
#### `bool Any()`

Are there any alignments in the scene

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentAlignments.Any)

#### `System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.CivilAlignment> GetAlignments()`

Get a list of all of the alignments in the scene

**Returns:** `System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.CivilAlignment>` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentAlignments.GetAlignments)

### Properties
- `ActiveAlignmentIndex` (int, get/set) — The index of the alignment, from the provided list, that is the current alignment used for rendering and everything else.
- `Enabled` (bool, get/set) — Is rendering of the current alignment enabled

## DocumentCurrentComments (class)

Stores the Current Comments information contained in a Document.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.DocumentParts.DocumentCurrentComments)

### Methods
#### `void ChangeSource(string sourceOwner, Autodesk.Navisworks.Api.CommentCollection sourceComments)`

Changes the Current Comments. Typically called when the focus changes from one Comment Source to an other.

**Parameters:**
- `sourceOwner` (string) — Internal string representing the owner of the comments.
- `sourceComments` (Autodesk.Navisworks.Api.CommentCollection) — The Comments.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentComments.ChangeSource%28System.String%2CAutodesk.Navisworks.Api.CommentCollection%29)

#### `void ChangeSource(string sourceOwner, Autodesk.Navisworks.Api.CommentCollection sourceComments, int activeCommentIndex)`

Changes the Current Comments. Typically called when the focus changes from one Comment Source to an other.

**Parameters:**
- `sourceOwner` (string) — Internal string representing the owner of the comments.
- `sourceComments` (Autodesk.Navisworks.Api.CommentCollection) — The Comments.
- `activeCommentIndex` (int) — The Comment to select.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentComments.ChangeSource%28System.String%2CAutodesk.Navisworks.Api.CommentCollection%2CSystem.Int32%29)

#### `void ClearSource()`

Sets situation that there is no owner, comments are cleared and editing prevented.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentComments.ClearSource)

#### `void CopyFrom(Autodesk.Navisworks.Api.CommentCollection comments)`

Copies the CommentCollection from comments.

**Parameters:**
- `comments` (Autodesk.Navisworks.Api.CommentCollection)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentComments.CopyFrom%28Autodesk.Navisworks.Api.CommentCollection%29)

#### `Autodesk.Navisworks.Api.CommentCollection CreateCopy()`

Creates a copy of the CommentCollection associated with this object.

**Returns:** `Autodesk.Navisworks.Api.CommentCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentComments.CreateCopy)

#### `Autodesk.Navisworks.Api.CommentCollection ToCommentCollection()`

Explicit form of conversion operator returning Value.

**Returns:** `Autodesk.Navisworks.Api.CommentCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentComments.ToCommentCollection)

#### `static Autodesk.Navisworks.Api.CommentCollection op_Implicit(Autodesk.Navisworks.Api.DocumentParts.DocumentCurrentComments that)`

Conversion operator returning Value.

**Parameters:**
- `that` (Autodesk.Navisworks.Api.DocumentParts.DocumentCurrentComments)

**Returns:** `Autodesk.Navisworks.Api.CommentCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentComments.op_Implicit%28Autodesk.Navisworks.Api.DocumentParts.DocumentCurrentComments%29~Autodesk.Navisworks.Api.CommentCollection)

### Properties
- `Owner` (string, get) — Internal string representing the owner of the comments.
- `Value` (Autodesk.Navisworks.Api.CommentCollection, get) — The value of the current CommentsCollection within the Document.

### Events
#### `Changed` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> Changed`

Occurs when the Value of this object has changed.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentComments.Changed)

#### `Changing` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> Changing`

Occurs when when the Value of this object is about to change.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentComments.Changing)

## DocumentCurrentMeasurement (class)

The current measurement state in a Document

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.DocumentParts.DocumentCurrentMeasurement)

### Methods
#### `void AddRedlinesToCurrentSavedViewpoint()`

Converts the measurement to redlines. Then adds the redlines to the current saved viewpoint. If there is no current saved viewpoint, then one is created.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentMeasurement.AddRedlinesToCurrentSavedViewpoint)

### Properties
- `CanConvertToRedlines` (bool, get) — Determines if the measurement can be converted to redlines
- `Difference` (Autodesk.Navisworks.Api.Vector3D, get) — Returns the difference
- `EndPoint` (Autodesk.Navisworks.Api.Point3D, get) — Returns the end point
- `FirstPoint` (Autodesk.Navisworks.Api.Point3D, get) — Returns the first point
- `HasDifference` (bool, get) — Determines if the difference is available
- `HasEndPoint` (bool, get) — Determines if the end point is available
- `HasFirstPoint` (bool, get) — Determines if the first point is available
- `HasMeasurement` (bool, get) — Determines if the MeasurementValue and MeasurementType are available
- `MeasurementType` (Autodesk.Navisworks.Api.MeasurementType, get) — Returns the measurement type
- `MeasurementValue` (double, get) — Returns the measurement value
- `Tool` (Autodesk.Navisworks.Api.Tool, get) — Returns the measurement tool in use
- `ToolMeasurementType` (Autodesk.Navisworks.Api.MeasurementType, get) — Returns the tool measurement type

## DocumentCurrentSearch (class)

The current search in a Document

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.DocumentParts.DocumentCurrentSearch)

### Methods
#### `void Clear()`

Clears the current search

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentSearch.Clear)

#### `void CopyFrom(Autodesk.Navisworks.Api.Search search)`

Copies the Search from search

**Parameters:**
- `search` (Autodesk.Navisworks.Api.Search)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentSearch.CopyFrom%28Autodesk.Navisworks.Api.Search%29)

#### `Autodesk.Navisworks.Api.Search CreateCopy()`

Creates a copy of the Search associated with this object

**Returns:** `Autodesk.Navisworks.Api.Search` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentSearch.CreateCopy)

#### `Autodesk.Navisworks.Api.Search ToSearch()`

Explicit form of conversion operator returning Value

**Returns:** `Autodesk.Navisworks.Api.Search` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentSearch.ToSearch)

#### `static Autodesk.Navisworks.Api.Search op_Implicit(Autodesk.Navisworks.Api.DocumentParts.DocumentCurrentSearch that)`

Conversion operator returning Value

**Parameters:**
- `that` (Autodesk.Navisworks.Api.DocumentParts.DocumentCurrentSearch)

**Returns:** `Autodesk.Navisworks.Api.Search` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentSearch.op_Implicit%28Autodesk.Navisworks.Api.DocumentParts.DocumentCurrentSearch%29~Autodesk.Navisworks.Api.Search)

### Properties
- `IsClear` (bool, get) — Returns true when there is no selection or search conditions in the search
- `Value` (Autodesk.Navisworks.Api.Search, get) — The value of the current search within the document

### Events
#### `Changed` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> Changed`

Occurs when the Search contained in this object has changed

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentSearch.Changed)

#### `Changing` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> Changing`

Occurs when the Search contained in this object is about to change

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentSearch.Changing)

## DocumentCurrentSelection (class)

The current selection in a Document

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.DocumentParts.DocumentCurrentSelection)

### Methods
#### `void Add(Autodesk.Navisworks.Api.ModelItem item)`

Adds a ModelItem to the Selection

**Parameters:**
- `item` (Autodesk.Navisworks.Api.ModelItem) — The ModelItem we wish to add to the selection

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentSelection.Add%28Autodesk.Navisworks.Api.ModelItem%29)

#### `void AddRange(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem> range)`

Adds a range of ModelItem to the Selection

**Parameters:**
- `range` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem>) — The ModelItems we wish to add to the selection

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentSelection.AddRange%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.ModelItem%7D%29)

#### `void Clear()`

Removes all entries from

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentSelection.Clear)

#### `void CopyFrom(Autodesk.Navisworks.Api.ModelItemCollection from)`

Copies the ModelItems from a ModelItemCollection and replaces the list of entries held in SelectedItems

**Parameters:**
- `from` (Autodesk.Navisworks.Api.ModelItemCollection) — The ModelItems to reference for this operation

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentSelection.CopyFrom%28Autodesk.Navisworks.Api.ModelItemCollection%29)

#### `void CopyFrom(Autodesk.Navisworks.Api.Selection selection)`

Copies the Selection from selection and replaces the list of entries held in SelectedItems

**Parameters:**
- `selection` (Autodesk.Navisworks.Api.Selection)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentSelection.CopyFrom%28Autodesk.Navisworks.Api.Selection%29)

#### `void CopyFrom(Autodesk.Navisworks.Api.SelectionSourceCollection selectionSources)`

Updates the SelectionSources in Selection from selectionSources and replaces the list of entries held in SelectedItems

**Parameters:**
- `selectionSources` (Autodesk.Navisworks.Api.SelectionSourceCollection)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentSelection.CopyFrom%28Autodesk.Navisworks.Api.SelectionSourceCollection%29)

#### `void CopyFrom(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem> from)`

Copies the ModelItems from a IEnumerable>ModelItem< and replaces the list of entries held in SelectedItems

**Parameters:**
- `from` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem>) — The ModelItems to reference for this operation

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentSelection.CopyFrom%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.ModelItem%7D%29)

#### `Autodesk.Navisworks.Api.Selection CreateCopy()`

Creates a copy of the Selection associated with this object

**Returns:** `Autodesk.Navisworks.Api.Selection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentSelection.CreateCopy)

#### `void Remove(Autodesk.Navisworks.Api.ModelItem item)`

Removes a ModelItem from the Selection

**Parameters:**
- `item` (Autodesk.Navisworks.Api.ModelItem) — The ModelItems we wish to remove from the selection

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentSelection.Remove%28Autodesk.Navisworks.Api.ModelItem%29)

#### `void SelectAll()`

Selects all the ModelItems in the current document

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentSelection.SelectAll)

#### `Autodesk.Navisworks.Api.Selection ToSelection()`

Explicit form of conversion operator returning Value

**Returns:** `Autodesk.Navisworks.Api.Selection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentSelection.ToSelection)

#### `static Autodesk.Navisworks.Api.Selection op_Implicit(Autodesk.Navisworks.Api.DocumentParts.DocumentCurrentSelection that)`

Conversion operator returning Value

**Parameters:**
- `that` (Autodesk.Navisworks.Api.DocumentParts.DocumentCurrentSelection)

**Returns:** `Autodesk.Navisworks.Api.Selection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentSelection.op_Implicit%28Autodesk.Navisworks.Api.DocumentParts.DocumentCurrentSelection%29~Autodesk.Navisworks.Api.Selection)

### Properties
- `IsEmpty` (bool, get) — Returns true when there are no ModelItems in the selection
- `SelectedItems` (Autodesk.Navisworks.Api.ModelItemCollection, get) — Collection of instances of ModelItem in this Selection
- `Value` (Autodesk.Navisworks.Api.Selection, get) — The value of the current selection within the document

### Events
#### `Changed` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> Changed`

Occurs when the Selection contained in this object has changed

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentSelection.Changed)

#### `Changing` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> Changing`

Occurs when the Selection contained in this object is about to change

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentSelection.Changing)

## DocumentCurrentViewpoint (class)

The Current Viewpoint in a Document

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.DocumentParts.DocumentCurrentViewpoint)

### Methods
#### `void CopyFrom(Autodesk.Navisworks.Api.Viewpoint viewpoint)`

Copies the Viewpoint from viewpoint int the one in the Document

**Parameters:**
- `viewpoint` (Autodesk.Navisworks.Api.Viewpoint)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentViewpoint.CopyFrom%28Autodesk.Navisworks.Api.Viewpoint%29)

#### `Autodesk.Navisworks.Api.Viewpoint CreateCopy()`

Creates a copy of the Viewpoint associated with this object

**Returns:** `Autodesk.Navisworks.Api.Viewpoint` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentViewpoint.CreateCopy)

#### `Autodesk.Navisworks.Api.Viewpoint ToViewpoint()`

Explicit form of conversion operator returning Value

**Returns:** `Autodesk.Navisworks.Api.Viewpoint` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentViewpoint.ToViewpoint)

#### `static Autodesk.Navisworks.Api.Viewpoint op_Implicit(Autodesk.Navisworks.Api.DocumentParts.DocumentCurrentViewpoint that)`

Conversion operator returning Value

**Parameters:**
- `that` (Autodesk.Navisworks.Api.DocumentParts.DocumentCurrentViewpoint)

**Returns:** `Autodesk.Navisworks.Api.Viewpoint` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentViewpoint.op_Implicit%28Autodesk.Navisworks.Api.DocumentParts.DocumentCurrentViewpoint%29~Autodesk.Navisworks.Api.Viewpoint)

### Properties
- `Value` (Autodesk.Navisworks.Api.Viewpoint, get) — The value of the current Viewpoint within the document

### Events
#### `Changed` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> Changed`

Occurs when the Viewpoint contained in this object has changed.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentViewpoint.Changed)

#### `Changing` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> Changing`

Occurs when the Viewpoint contained in this object is about to change.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentCurrentViewpoint.Changing)

## DocumentDatabase (class)

The embedded database in a Document

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.DocumentParts.DocumentDatabase)

### Methods
#### `Autodesk.Navisworks.Api.Data.NavisworksTransaction BeginTransaction(Autodesk.Navisworks.Api.Data.DatabaseChangedAction action)`

Begin a transaction.

**Parameters:**
- `action` (Autodesk.Navisworks.Api.Data.DatabaseChangedAction) — Determines which action should do after commit transaction, DatabaseChangedAction

**Returns:** `Autodesk.Navisworks.Api.Data.NavisworksTransaction` — If success return NavisworksTransaction, if failed, return null

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentDatabase.BeginTransaction%28Autodesk.Navisworks.Api.Data.DatabaseChangedAction%29)

#### `void Dispose()`

Dispose method of DocumentDatabase.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentDatabase.Dispose)

#### `void EnableTableUndoable(string tableName)`

Enable table to undoable or redoable.

**Parameters:**
- `tableName` (string)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentDatabase.EnableTableUndoable%28System.String%29)

#### `Autodesk.Navisworks.Api.Data.NavisworksConnection ToNavisworksConnection()`

Explicit form of conversion operator returning Value

**Returns:** `Autodesk.Navisworks.Api.Data.NavisworksConnection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentDatabase.ToNavisworksConnection)

#### `static Autodesk.Navisworks.Api.Data.NavisworksConnection op_Implicit(Autodesk.Navisworks.Api.DocumentParts.DocumentDatabase that)`

Conversion operator returning Value

**Parameters:**
- `that` (Autodesk.Navisworks.Api.DocumentParts.DocumentDatabase)

**Returns:** `Autodesk.Navisworks.Api.Data.NavisworksConnection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentDatabase.op_Implicit%28Autodesk.Navisworks.Api.DocumentParts.DocumentDatabase%29~Autodesk.Navisworks.Api.Data.NavisworksConnection)

### Properties
- `ExistLiveTransaction` (bool, get) — If is in a transaction.
- `Value` (Autodesk.Navisworks.Api.Data.NavisworksConnection, get) — The value of the NavisworksConnection of embedded database in the document

### Events
#### `Changed` (`System.EventHandler<Autodesk.Navisworks.Api.Data.DatabaseChangedEventArgs>`)

**Signature:** `event System.EventHandler<Autodesk.Navisworks.Api.Data.DatabaseChangedEventArgs> Changed`

Occurs when the DocumentDatabase has changed

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentDatabase.Changed)

#### `Changing` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> Changing`

Occurs when the DocumentDatabase is about to change

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentDatabase.Changing)

#### `Loaded` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> Loaded`

Occurs when entirely new database loaded from file.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentDatabase.Loaded)

#### `TransactionBeginning` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> TransactionBeginning`

Occurs when a database transaction is about to begins, before any of the edits in the transaction take place.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentDatabase.TransactionBeginning)

#### `TransactionBegun` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> TransactionBegun`

Occurs when a database transaction begins, before any of the edits in the transaction take place.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentDatabase.TransactionBegun)

#### `TransactionCommitting` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> TransactionCommitting`

Occurs when a database transaction has completed all edits and will commit.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentDatabase.TransactionCommitting)

#### `TransactionRollingBack` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> TransactionRollingBack`

Occurs when a database transaction has completed all edits and happen error, but will rollback.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentDatabase.TransactionRollingBack)

#### `Unloading` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> Unloading`

Current database about to be unloaded (about to change file or exit).

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentDatabase.Unloading)

## DocumentGrids (class)

Stores the Grids information contained in a Document.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.DocumentParts.DocumentGrids)

### Methods
#### `void SetSystemLockedLevel(Autodesk.Navisworks.Api.GridSystem system, Autodesk.Navisworks.Api.GridLevel level)`

Sets the LockedLevel for a particular System.

**Parameters:**
- `system` (Autodesk.Navisworks.Api.GridSystem)
- `level` (Autodesk.Navisworks.Api.GridLevel)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentGrids.SetSystemLockedLevel%28Autodesk.Navisworks.Api.GridSystem%2CAutodesk.Navisworks.Api.GridLevel%29)

### Properties
- `ActiveSystem` (Autodesk.Navisworks.Api.GridSystem, get/set) — The GridSystem that will be rendered.
- `RenderMode` (Autodesk.Navisworks.Api.GridsRenderMode, get/set) — The GridSystem that will be rendered.
- `Systems` (Autodesk.Navisworks.Api.GridSystemCollection, get) — The collection of all GridSystems within the Document.

### Events
#### `Changed` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> Changed`

Occurs when the GridsData contained in this object has changed.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentGrids.Changed)

#### `Changing` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> Changing`

Occurs when the GridsData contained in about to change.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentGrids.Changing)

## DocumentHyperlinks (class)

Stores the Hyperlinks information contained in a Document.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.DocumentParts.DocumentHyperlinks)

### Methods
#### `void DeleteHyperlinks(Autodesk.Navisworks.Api.ModelItem modelItem)`

Deletes all hyperlinks from a ModelItem.

**Parameters:**
- `modelItem` (Autodesk.Navisworks.Api.ModelItem)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentHyperlinks.DeleteHyperlinks%28Autodesk.Navisworks.Api.ModelItem%29)

#### `Autodesk.Navisworks.Api.HyperlinkCollection GetHyperlinks(Autodesk.Navisworks.Api.ModelItem modelItem)`

Gets hyperlinks for a ModelItem.

**Parameters:**
- `modelItem` (Autodesk.Navisworks.Api.ModelItem)

**Returns:** `Autodesk.Navisworks.Api.HyperlinkCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentHyperlinks.GetHyperlinks%28Autodesk.Navisworks.Api.ModelItem%29)

#### `void ResetAllHyperlinks()`

Resets hyperlinks on all ModelItems.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentHyperlinks.ResetAllHyperlinks)

#### `void ResetHyperlinks(Autodesk.Navisworks.Api.ModelItem modelItem)`

Resets hyperlinks on a ModelItem back to their original state.

**Parameters:**
- `modelItem` (Autodesk.Navisworks.Api.ModelItem)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentHyperlinks.ResetHyperlinks%28Autodesk.Navisworks.Api.ModelItem%29)

#### `void SetHyperlinks(Autodesk.Navisworks.Api.ModelItem modelItem, Autodesk.Navisworks.Api.HyperlinkCollection links)`

Sets hyperlinks for a ModelItem from a list of hyperlinks.

**Parameters:**
- `modelItem` (Autodesk.Navisworks.Api.ModelItem)
- `links` (Autodesk.Navisworks.Api.HyperlinkCollection)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentHyperlinks.SetHyperlinks%28Autodesk.Navisworks.Api.ModelItem%2CAutodesk.Navisworks.Api.HyperlinkCollection%29)

### Events
#### `Changed` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> Changed`

Occurs when the document hyperlinks have changed.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentHyperlinks.Changed)

#### `Changing` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> Changing`

Occurs when the document hyperlinks are about to change.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentHyperlinks.Changing)

## DocumentInfoPart (class)

The DocumentInfo stored in a Document

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.DocumentParts.DocumentInfoPart)

### Methods
#### `void AddCopy(Autodesk.Navisworks.Api.GroupItem parent, Autodesk.Navisworks.Api.SavedItem item)`

Adds a copy of item to the end of the specified parent group contained by DocumentInfo.

**Parameters:**
- `parent` (Autodesk.Navisworks.Api.GroupItem) — The parent group to add to. RootItem or null to add directly to Value
- `item` (Autodesk.Navisworks.Api.SavedItem)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentInfoPart.AddCopy%28Autodesk.Navisworks.Api.GroupItem%2CAutodesk.Navisworks.Api.SavedItem%29)

#### `void AddCopy(Autodesk.Navisworks.Api.SavedItem item)`

Adds a copy of item to the end of Value.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SavedItem)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentInfoPart.AddCopy%28Autodesk.Navisworks.Api.SavedItem%29)

#### `void AddUniqueCopy(Autodesk.Navisworks.Api.SavedItem item, string sourceFileName)`

Adds an unique copy of item to the end of Value

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SavedItem)
- `sourceFileName` (string) — If the parent group have the same name, will add it to item name

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentInfoPart.AddUniqueCopy%28Autodesk.Navisworks.Api.SavedItem%2CSystem.String%29)

#### `Autodesk.Navisworks.Api.DocumentInfo CreateCopy()`

Creates a copy of Value

**Returns:** `Autodesk.Navisworks.Api.DocumentInfo` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentInfoPart.CreateCopy)

#### `void EditDisplayName(Autodesk.Navisworks.Api.SavedItem item, string newDisplayName)`

Edit item and update it's DisplayName

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SavedItem)
- `newDisplayName` (string)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentInfoPart.EditDisplayName%28Autodesk.Navisworks.Api.SavedItem%2CSystem.String%29)

#### `void InsertCopy(Autodesk.Navisworks.Api.GroupItem parent, int index, Autodesk.Navisworks.Api.SavedItem item)`

Inserts a copy of item into the specified parent group contained by DocumentInfo.

**Parameters:**
- `parent` (Autodesk.Navisworks.Api.GroupItem) — The parent group to insert into. RootItem or null to insert directly into Value
- `index` (int)
- `item` (Autodesk.Navisworks.Api.SavedItem)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentInfoPart.InsertCopy%28Autodesk.Navisworks.Api.GroupItem%2CSystem.Int32%2CAutodesk.Navisworks.Api.SavedItem%29)

#### `void InsertCopy(int index, Autodesk.Navisworks.Api.SavedItem item)`

Inserts a copy of item into Value.

**Parameters:**
- `index` (int)
- `item` (Autodesk.Navisworks.Api.SavedItem)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentInfoPart.InsertCopy%28System.Int32%2CAutodesk.Navisworks.Api.SavedItem%29)

#### `void InsertUniqueCopy(Autodesk.Navisworks.Api.GroupItem parent, int index, Autodesk.Navisworks.Api.SavedItem item, string sourceFileName)`

Inserts a copy of item into the specified parent group contained by DocumentInfo. If have same sheet, will reset id and rename

**Parameters:**
- `parent` (Autodesk.Navisworks.Api.GroupItem) — The parent group to insert into. RootItem or null to insert directly into Value
- `index` (int)
- `item` (Autodesk.Navisworks.Api.SavedItem)
- `sourceFileName` (string) — If the parent group have the same name, will add it to item name

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentInfoPart.InsertUniqueCopy%28Autodesk.Navisworks.Api.GroupItem%2CSystem.Int32%2CAutodesk.Navisworks.Api.SavedItem%2CSystem.String%29)

#### `void Move(Autodesk.Navisworks.Api.GroupItem oldParent, int oldIndex, Autodesk.Navisworks.Api.GroupItem newParent, int newIndex)`

Moves item at oldIndex in OldParent to newIndex in newParent

**Parameters:**
- `oldParent` (Autodesk.Navisworks.Api.GroupItem) — The parent group to move the item from.
- `oldIndex` (int)
- `newParent` (Autodesk.Navisworks.Api.GroupItem) — The parent group to move to.
If newParent is equal to oldParent, the item will be moved within the same SavedItemCollection.
- `newIndex` (int)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentInfoPart.Move%28Autodesk.Navisworks.Api.GroupItem%2CSystem.Int32%2CAutodesk.Navisworks.Api.GroupItem%2CSystem.Int32%29)

#### `void Move(int oldIndex, int newIndex)`

Move sheet from oldIndex to new position in collection.

**Parameters:**
- `oldIndex` (int) — The sheet in this position to move
- `newIndex` (int) — The moved sheet place in this position

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentInfoPart.Move%28System.Int32%2CSystem.Int32%29)

#### `bool Remove(Autodesk.Navisworks.Api.GroupItem parent, Autodesk.Navisworks.Api.SavedItem item)`

Removes specified item from the specified parent group contained by DocumentInfo.

**Parameters:**
- `parent` (Autodesk.Navisworks.Api.GroupItem) — The parent group to remove from. null to remove directly from DocumentInfo
- `item` (Autodesk.Navisworks.Api.SavedItem)

**Returns:** `bool` — false if item not found in DocumentInfo

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentInfoPart.Remove%28Autodesk.Navisworks.Api.GroupItem%2CAutodesk.Navisworks.Api.SavedItem%29)

#### `bool Remove(Autodesk.Navisworks.Api.SavedItem item)`

Removes specified item from DocumentInfo.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SavedItem)

**Returns:** `bool` — false if item not found in DocumentInfo

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentInfoPart.Remove%28Autodesk.Navisworks.Api.SavedItem%29)

#### `void RemoveAt(Autodesk.Navisworks.Api.GroupItem parent, int index)`

Removes item from the specified parent group contained by DocumentInfo.

**Parameters:**
- `parent` (Autodesk.Navisworks.Api.GroupItem) — The parent group to remove from. RootItem or null to remove directly from DocumentInfo
- `index` (int)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentInfoPart.RemoveAt%28Autodesk.Navisworks.Api.GroupItem%2CSystem.Int32%29)

#### `void RemoveAt(int index)`

Removes item at the specified index from DocumentInfo.

**Parameters:**
- `index` (int)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentInfoPart.RemoveAt%28System.Int32%29)

#### `void ReplaceWithCopy(Autodesk.Navisworks.Api.GroupItem parent, int index, Autodesk.Navisworks.Api.SavedItem item)`

Replaces item from the specified parent group with a copy of item

**Parameters:**
- `parent` (Autodesk.Navisworks.Api.GroupItem) — The parent group to insert into. RootItem or null to insert directly into Value
- `index` (int)
- `item` (Autodesk.Navisworks.Api.SavedItem)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentInfoPart.ReplaceWithCopy%28Autodesk.Navisworks.Api.GroupItem%2CSystem.Int32%2CAutodesk.Navisworks.Api.SavedItem%29)

#### `void ReplaceWithCopy(int index, Autodesk.Navisworks.Api.SavedItem item)`

Replaces item at given index within DocumentInfo with a copy of item

**Parameters:**
- `index` (int)
- `item` (Autodesk.Navisworks.Api.SavedItem)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentInfoPart.ReplaceWithCopy%28System.Int32%2CAutodesk.Navisworks.Api.SavedItem%29)

#### `Autodesk.Navisworks.Api.DocumentInfo ToDocumentInfo()`

Explicit form of conversion operator returning Value

**Returns:** `Autodesk.Navisworks.Api.DocumentInfo` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentInfoPart.ToDocumentInfo)

#### `static Autodesk.Navisworks.Api.DocumentInfo op_Implicit(Autodesk.Navisworks.Api.DocumentParts.DocumentInfoPart that)`

Conversion operator returning Value

**Parameters:**
- `that` (Autodesk.Navisworks.Api.DocumentParts.DocumentInfoPart)

**Returns:** `Autodesk.Navisworks.Api.DocumentInfo` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentInfoPart.op_Implicit%28Autodesk.Navisworks.Api.DocumentParts.DocumentInfoPart%29~Autodesk.Navisworks.Api.DocumentInfo)

### Properties
- `Value` (Autodesk.Navisworks.Api.DocumentInfo, get) — The value of the current DocumentInfo within the document

### Events
#### `Changed` (`System.EventHandler<Autodesk.Navisworks.Api.SavedItemChangedEventArgs>`)

**Signature:** `event System.EventHandler<Autodesk.Navisworks.Api.SavedItemChangedEventArgs> Changed`

Occurs when the DocumentInfo contained in this object has changed

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentInfoPart.Changed)

#### `Changing` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> Changing`

Occurs when the DocumentInfo contained in this object is about to change

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentInfoPart.Changing)

## DocumentModels (class)

Stores the collection of Models contained in a Document

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.DocumentParts.DocumentModels)

### Methods
#### `bool Contains(Autodesk.Navisworks.Api.Model item)`

Determines whether the collection contains a specific value.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.Model) — The Model to locate in the collection.

**Returns:** `bool` — true if item is found in the collection; otherwise, false.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.Contains%28Autodesk.Navisworks.Api.Model%29)

#### `void CopyTo(Autodesk.Navisworks.Api.Model[] array, int arrayIndex)`

Copies the elements of the ICollection`1 to an Array, starting at a particular Array index.

**Parameters:**
- `array` (Autodesk.Navisworks.Api.Model[]) — The one-dimensional Array that is the destination of the elements copied from ICollection`1. The Array must have zero-based indexing.
- `arrayIndex` (int) — The zero-based index in array at which copying begins.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.CopyTo%28Autodesk.Navisworks.Api.Model%5B%5D%2CSystem.Int32%29)

#### `void CopyTo(System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.Model> to)`

Copies the contents of the collection into to

**Parameters:**
- `to` (System.Collections.Generic.ICollection<Autodesk.Navisworks.Api.Model>) — The target collection

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.CopyTo%28System.Collections.Generic.ICollection%7BAutodesk.Navisworks.Api.Model%7D%29)

#### `Autodesk.Navisworks.Api.ModelItemCollection CreateCollection()`

Returns an empty collection associated with this Document

**Returns:** `Autodesk.Navisworks.Api.ModelItemCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.CreateCollection)

#### `Autodesk.Navisworks.Api.ModelItemCollection CreateCollectionFromRootItems()`

Returns a snapshot collection containing the RootItem from each Model

**Returns:** `Autodesk.Navisworks.Api.ModelItemCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.CreateCollectionFromRootItems)

#### `System.Collections.ObjectModel.Collection<int> CreateIndexPath(Autodesk.Navisworks.Api.ModelItem item)`

Returns a collection of indices that defines a path from the root item through its descendants to the specified item.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.ModelItem)

**Returns:** `System.Collections.ObjectModel.Collection<int>` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.CreateIndexPath%28Autodesk.Navisworks.Api.ModelItem%29)

#### `Autodesk.Navisworks.Api.DocumentParts.ModelItemPathId CreatePathId(Autodesk.Navisworks.Api.ModelItem item)`

Return the path identifier that defines a path from the root item through its descendants to the specified item, and the model index this item belongs to.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.ModelItem)

**Returns:** `Autodesk.Navisworks.Api.DocumentParts.ModelItemPathId` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.CreatePathId%28Autodesk.Navisworks.Api.ModelItem%29)

#### `ulong FindIdForModelItem(Autodesk.Navisworks.Api.ModelItem item)`

Returns numeric ID for a given model item.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.ModelItem)

**Returns:** `ulong` — ID for model item, or 0 if no ID.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.FindIdForModelItem%28Autodesk.Navisworks.Api.ModelItem%29)

#### `Autodesk.Navisworks.Api.ModelItemCollection FindModelItemsById(int modelIndex, ulong id)`

Returns collection of model items that have a given numeric ID.

**Parameters:**
- `modelIndex` (int)
- `id` (ulong)

**Returns:** `Autodesk.Navisworks.Api.ModelItemCollection` — Model item collection containing requested items (if any).

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.FindModelItemsById%28System.Int32%2CSystem.UInt64%29)

#### `Autodesk.Navisworks.Api.ModelItemCollection GetAllHiddenAtModelState()`

Gets the hidden status as defined by the constituent models.

**Returns:** `Autodesk.Navisworks.Api.ModelItemCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.GetAllHiddenAtModelState)

#### `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.Model> GetEnumerator()`

Returns an enumerator that can iterate through the collection.

**Returns:** `System.Collections.Generic.IEnumerator<Autodesk.Navisworks.Api.Model>` — An IEnumerator that can be used to iterate through the collection.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.GetEnumerator)

#### `int IndexOf(Autodesk.Navisworks.Api.Model item)`

Determines the index of a specific item in the DocumentModels.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.Model)

**Returns:** `int` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.IndexOf%28Autodesk.Navisworks.Api.Model%29)

#### `void InternalAdd(Autodesk.Navisworks.Api.Model model)`

InternalAdd method of DocumentModels.

**Parameters:**
- `model` (Autodesk.Navisworks.Api.Model)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.InternalAdd%28Autodesk.Navisworks.Api.Model%29)

#### `void InternalClear()`

InternalClear method of DocumentModels.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.InternalClear)

#### `System.Collections.IEnumerator InternalGetEnumerator()`

InternalGetEnumerator method of DocumentModels.

**Returns:** `System.Collections.IEnumerator` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.InternalGetEnumerator)

#### `void InternalInsert(int index, Autodesk.Navisworks.Api.Model item)`

InternalInsert method of DocumentModels.

**Parameters:**
- `index` (int)
- `item` (Autodesk.Navisworks.Api.Model)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.InternalInsert%28System.Int32%2CAutodesk.Navisworks.Api.Model%29)

#### `bool InternalRemove(Autodesk.Navisworks.Api.Model item)`

InternalRemove method of DocumentModels.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.Model)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.InternalRemove%28Autodesk.Navisworks.Api.Model%29)

#### `void InternalRemoveAt(int index)`

InternalRemoveAt method of DocumentModels.

**Parameters:**
- `index` (int)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.InternalRemoveAt%28System.Int32%29)

#### `bool IsFrozen(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem> items)`

Returns true if all items have their IsFrozen property set.

**Parameters:**
- `items` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem>)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.IsFrozen%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.ModelItem%7D%29)

#### `bool IsHidden(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem> items)`

Returns true if all items have their Hidden property set.

**Parameters:**
- `items` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem>)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.IsHidden%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.ModelItem%7D%29)

#### `bool IsRequired(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem> items)`

Returns true if all items have their IsRequired property set.

**Parameters:**
- `items` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem>)

**Returns:** `bool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.IsRequired%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.ModelItem%7D%29)

#### `void MakeVisible(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem> items)`

Ensures the model items given and all of their children are visible.

**Parameters:**
- `items` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem>)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.MakeVisible%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.ModelItem%7D%29)

#### `void OverridePermanentColor(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem> items, Autodesk.Navisworks.Api.Color color)`

Overrides the permanent color of all ModelGeometry descendants of items.

**Parameters:**
- `items` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem>)
- `color` (Autodesk.Navisworks.Api.Color)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.OverridePermanentColor%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.ModelItem%7D%2CAutodesk.Navisworks.Api.Color%29)

#### `void OverridePermanentTransform(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem> items, Autodesk.Navisworks.Api.Transform3D transform, bool updateModelTransform)`

Apply an incremental transform to a selection.

**Parameters:**
- `items` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem>)
- `transform` (Autodesk.Navisworks.Api.Transform3D)
- `updateModelTransform` (bool)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.OverridePermanentTransform%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.ModelItem%7D%2CAutodesk.Navisworks.Api.Transform3D%2CSystem.Boolean%29)

#### `void OverridePermanentTransparency(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem> items, double transparency)`

Overrides the permanent transparency of all ModelGeometry descendants of items.

**Parameters:**
- `items` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem>)
- `transparency` (double)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.OverridePermanentTransparency%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.ModelItem%7D%2CSystem.Double%29)

#### `void OverrideTemporaryColor(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem> items, Autodesk.Navisworks.Api.Color color)`

Overrides the temporary color of all ModelGeometry descendants of items.

**Parameters:**
- `items` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem>)
- `color` (Autodesk.Navisworks.Api.Color)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.OverrideTemporaryColor%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.ModelItem%7D%2CAutodesk.Navisworks.Api.Color%29)

#### `void OverrideTemporaryTransparency(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem> items, double transparency)`

Overrides the temporary transparency of all ModelGeometry descendants of items.

**Parameters:**
- `items` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem>)
- `transparency` (double)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.OverrideTemporaryTransparency%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.ModelItem%7D%2CSystem.Double%29)

#### `void ResetAllFrozen()`

Clears the IsFrozen property on all items in the models.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.ResetAllFrozen)

#### `void ResetAllHidden()`

Clears the IsHidden property on all items in the models.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.ResetAllHidden)

#### `void ResetAllHiddenToModelState()`

Resets the hidden status to that defined in the constituent models.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.ResetAllHiddenToModelState)

#### `void ResetAllPermanentMaterials()`

Resets permanent material properties (color and transparency) of all ModelGeometry in the models.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.ResetAllPermanentMaterials)

#### `void ResetAllPermanentTransforms()`

Reset incremental transforms for all model items contained in the entire model.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.ResetAllPermanentTransforms)

#### `void ResetAllRequired()`

Clears the IsRequired property on all items in the models.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.ResetAllRequired)

#### `void ResetAllTemporaryMaterials()`

Resets temporary material properties (color and transparency) of all ModelGeometry in the models.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.ResetAllTemporaryMaterials)

#### `void ResetPermanentMaterials(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem> items)`

Resets permanent material properties (color and transparency) of all ModelGeometry descendants of items.

**Parameters:**
- `items` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem>)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.ResetPermanentMaterials%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.ModelItem%7D%29)

#### `void ResetPermanentTransform(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem> items)`

Reset incremental transforms for all model items contained in the selection.

**Parameters:**
- `items` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem>)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.ResetPermanentTransform%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.ModelItem%7D%29)

#### `void ResetTemporaryMaterials(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem> items)`

Resets temporary material properties (color and transparency) of all ModelGeometry descendants of items.

**Parameters:**
- `items` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem>)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.ResetTemporaryMaterials%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.ModelItem%7D%29)

#### `Autodesk.Navisworks.Api.ModelItem ResolveIndexPath(System.Collections.Generic.IEnumerable<int> path)`

Returns the saved item referenced by a collection of indices that define a path from the root item through its descendants to the specified item.

**Parameters:**
- `path` (System.Collections.Generic.IEnumerable<int>)

**Returns:** `Autodesk.Navisworks.Api.ModelItem` — null if source cannot be resolved

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.ResolveIndexPath%28System.Collections.Generic.IEnumerable%7BSystem.Int32%7D%29)

#### `Autodesk.Navisworks.Api.ModelItem ResolvePathId(Autodesk.Navisworks.Api.DocumentParts.ModelItemPathId pathId)`

Returns the saved item referenced by a model index and a path identifier that define a path from the root item through its descendants to the specified item.

**Parameters:**
- `pathId` (Autodesk.Navisworks.Api.DocumentParts.ModelItemPathId)

**Returns:** `Autodesk.Navisworks.Api.ModelItem` — null if source cannot be resolved

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.ResolvePathId%28Autodesk.Navisworks.Api.DocumentParts.ModelItemPathId%29)

#### `void SetFrozen(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem> items, bool value)`

Sets the IsFrozen property on all instances of specified items.

**Parameters:**
- `items` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem>)
- `value` (bool)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.SetFrozen%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.ModelItem%7D%2CSystem.Boolean%29)

#### `void SetHidden(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem> items, bool value)`

Sets the IsHidden property on all instances of specified items.

**Parameters:**
- `items` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem>)
- `value` (bool)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.SetHidden%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.ModelItem%7D%2CSystem.Boolean%29)

#### `void SetModelUnitsAndTransform(Autodesk.Navisworks.Api.Model model, Autodesk.Navisworks.Api.Units units, Autodesk.Navisworks.Api.Transform3D transform, bool transformReflected)`

Sets the model units and transform.

**Parameters:**
- `model` (Autodesk.Navisworks.Api.Model)
- `units` (Autodesk.Navisworks.Api.Units)
- `transform` (Autodesk.Navisworks.Api.Transform3D)
- `transformReflected` (bool)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.SetModelUnitsAndTransform%28Autodesk.Navisworks.Api.Model%2CAutodesk.Navisworks.Api.Units%2CAutodesk.Navisworks.Api.Transform3D%2CSystem.Boolean%29)

#### `void SetRequired(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem> items, bool value)`

Sets the IsRequired property on all instances of specified items.

**Parameters:**
- `items` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.ModelItem>)
- `value` (bool)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.SetRequired%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.ModelItem%7D%2CSystem.Boolean%29)

### Properties
- `Count` (int, get) — Gets the number of elements contained in the collection.
- `First` (Autodesk.Navisworks.Api.Model, get) — Gets the first Model in the collection, returns null if the collection is empty
- `IsReadOnly` (bool, get) — Gets a value indicating whether the collection is read-only.
- `Item` (Autodesk.Navisworks.Api.Model, get/set) — Item property of DocumentModels.
- `RootItemDescendants` (Autodesk.Navisworks.Api.ModelItemEnumerableCollection, get) — All descendants of RootItems (excluding the RootItems themselves).
- `RootItemDescendantsAndSelf` (Autodesk.Navisworks.Api.ModelItemEnumerableCollection, get) — All descendants of RootItems (including the RootItems themselves).
- `RootItems` (Autodesk.Navisworks.Api.ModelItemEnumerableCollection, get) — The RootItem from each model

### Events
#### `CollectionChanged` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> CollectionChanged`

Occurs when the collection of Models has changed

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.CollectionChanged)

#### `CollectionChanging` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> CollectionChanging`

Occurs when the collection of Models is about to change

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.CollectionChanging)

#### `ModelGeometryMaterialChanged` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> ModelGeometryMaterialChanged`

Occurs when the material (color or transparency) of any ModelGeometry in the models has changed.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.ModelGeometryMaterialChanged)

#### `ModelItemPropertiesChanged` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> ModelItemPropertiesChanged`

Occurs when the editable properties (Hidden or Required) of any ModelItem in the models has changed.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.ModelItemPropertiesChanged)

#### `ModelTransformChanged` (`System.EventHandler<Autodesk.Navisworks.Api.Data.ModelTransformEventArgs>`)

**Signature:** `event System.EventHandler<Autodesk.Navisworks.Api.Data.ModelTransformEventArgs> ModelTransformChanged`

Occurs when the units or transform of a Model has changed.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.ModelTransformChanged)

#### `ModelTransformChanging` (`System.EventHandler<Autodesk.Navisworks.Api.Data.ModelTransformEventArgs>`)

**Signature:** `event System.EventHandler<Autodesk.Navisworks.Api.Data.ModelTransformEventArgs> ModelTransformChanging`

Occurs when the units or transform of a Model are about to change.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.ModelTransformChanging)

#### `SceneLoaded` (`System.EventHandler<Autodesk.Navisworks.Api.Interop.SceneLoadedEventArgs>`)

**Signature:** `event System.EventHandler<Autodesk.Navisworks.Api.Interop.SceneLoadedEventArgs> SceneLoaded`

Occurs whenever a new scene is loaded (or reloaded). This is meant only for internal Navisworks usage.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentModels.SceneLoaded)

## DocumentSavedViewpoints (class)

The saved viewpoints stored in a Document.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints)

### Methods
#### `void AddComment(Autodesk.Navisworks.Api.SavedItem item, Autodesk.Navisworks.Api.Comment comment)`

Edit item and add Comment to item's Comments

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SavedItem)
- `comment` (Autodesk.Navisworks.Api.Comment)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints.AddComment%28Autodesk.Navisworks.Api.SavedItem%2CAutodesk.Navisworks.Api.Comment%29)

#### `void AddCopy(Autodesk.Navisworks.Api.GroupItem parent, Autodesk.Navisworks.Api.SavedItem item)`

Adds a copy of item to the end of the specified parent group contained by SavedViewpoints.

**Parameters:**
- `parent` (Autodesk.Navisworks.Api.GroupItem) — The parent group to add to. RootItem or null to add directly to Value
- `item` (Autodesk.Navisworks.Api.SavedItem)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints.AddCopy%28Autodesk.Navisworks.Api.GroupItem%2CAutodesk.Navisworks.Api.SavedItem%29)

#### `void AddCopy(Autodesk.Navisworks.Api.SavedItem item)`

Adds a copy of item to the end of Value.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SavedItem)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints.AddCopy%28Autodesk.Navisworks.Api.SavedItem%29)

#### `Autodesk.Navisworks.Api.SavedViewpoint CaptureRuntimeOverrides()`

Creates a view that captures current runtime overrides.

**Returns:** `Autodesk.Navisworks.Api.SavedViewpoint` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints.CaptureRuntimeOverrides)

#### `void Clear()`

Removes all items in Value

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints.Clear)

#### `void CopyFrom(Autodesk.Navisworks.Api.SavedItemCollection value)`

Copies the SavedViewpoints from value and replaces the list of entries held in Value.

**Parameters:**
- `value` (Autodesk.Navisworks.Api.SavedItemCollection)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints.CopyFrom%28Autodesk.Navisworks.Api.SavedItemCollection%29)

#### `void CopyFrom(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.SavedItem> value)`

Copies the SavedViewpoints from value and replaces the list of entries held in Value.

**Parameters:**
- `value` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.SavedItem>)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints.CopyFrom%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.SavedItem%7D%29)

#### `System.Collections.ObjectModel.Collection<Autodesk.Navisworks.Api.SavedItem> CreateCopy()`

Creates a copy of the SavedViewpoints associated with this object

**Returns:** `System.Collections.ObjectModel.Collection<Autodesk.Navisworks.Api.SavedItem>` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints.CreateCopy)

#### `System.Collections.ObjectModel.Collection<int> CreateIndexPath(Autodesk.Navisworks.Api.SavedItem item)`

Returns a collection of indices that defines a path from the root item through its descendents to the specified item.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SavedItem)

**Returns:** `System.Collections.ObjectModel.Collection<int>` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints.CreateIndexPath%28Autodesk.Navisworks.Api.SavedItem%29)

#### `Autodesk.Navisworks.Api.SavedItemReference CreateReference(Autodesk.Navisworks.Api.SavedItem item)`

Returns a reference to a saved item.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SavedItem)

**Returns:** `Autodesk.Navisworks.Api.SavedItemReference` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints.CreateReference%28Autodesk.Navisworks.Api.SavedItem%29)

#### `void EditComments(Autodesk.Navisworks.Api.SavedItem item, Autodesk.Navisworks.Api.CommentCollection newComments)`

Edit item and update it's Comments

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SavedItem)
- `newComments` (Autodesk.Navisworks.Api.CommentCollection)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints.EditComments%28Autodesk.Navisworks.Api.SavedItem%2CAutodesk.Navisworks.Api.CommentCollection%29)

#### `void EditDisplayName(Autodesk.Navisworks.Api.SavedItem item, string newDisplayName)`

Edit item and update it's DisplayName

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SavedItem)
- `newDisplayName` (string)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints.EditDisplayName%28Autodesk.Navisworks.Api.SavedItem%2CSystem.String%29)

#### `void InsertCopy(Autodesk.Navisworks.Api.GroupItem parent, int index, Autodesk.Navisworks.Api.SavedItem item)`

Inserts a copy of item into the specified parent group contained by SavedViewpoints.

**Parameters:**
- `parent` (Autodesk.Navisworks.Api.GroupItem) — The parent group to insert into. RootItem or null to insert directly into Value
- `index` (int)
- `item` (Autodesk.Navisworks.Api.SavedItem)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints.InsertCopy%28Autodesk.Navisworks.Api.GroupItem%2CSystem.Int32%2CAutodesk.Navisworks.Api.SavedItem%29)

#### `void InsertCopy(int index, Autodesk.Navisworks.Api.SavedItem item)`

Inserts a copy of item into Value.

**Parameters:**
- `index` (int)
- `item` (Autodesk.Navisworks.Api.SavedItem)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints.InsertCopy%28System.Int32%2CAutodesk.Navisworks.Api.SavedItem%29)

#### `void Move(Autodesk.Navisworks.Api.GroupItem oldParent, int oldIndex, Autodesk.Navisworks.Api.GroupItem newParent, int newIndex)`

Moves item at oldIndex in OldParent to newIndex in newParent

**Parameters:**
- `oldParent` (Autodesk.Navisworks.Api.GroupItem) — The parent group to move the item from. RootItem or null to remove directly from Value
- `oldIndex` (int)
- `newParent` (Autodesk.Navisworks.Api.GroupItem) — The parent group to move to. RootItem or null to move directly into Value.
If newParent is equal to oldParent, the item will be moved within the same SavedItemCollection.
- `newIndex` (int)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints.Move%28Autodesk.Navisworks.Api.GroupItem%2CSystem.Int32%2CAutodesk.Navisworks.Api.GroupItem%2CSystem.Int32%29)

#### `void Move(int oldIndex, int newIndex)`

Moves item from oldIndex to newIndex within Value

**Parameters:**
- `oldIndex` (int)
- `newIndex` (int)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints.Move%28System.Int32%2CSystem.Int32%29)

#### `bool Remove(Autodesk.Navisworks.Api.GroupItem parent, Autodesk.Navisworks.Api.SavedItem item)`

Removes specified item from the specified parent group contained by SavedViewpoints.

**Parameters:**
- `parent` (Autodesk.Navisworks.Api.GroupItem) — The parent group to remove from. RootItem or null to remove directly from Value
- `item` (Autodesk.Navisworks.Api.SavedItem)

**Returns:** `bool` — false if item not found in Value

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints.Remove%28Autodesk.Navisworks.Api.GroupItem%2CAutodesk.Navisworks.Api.SavedItem%29)

#### `bool Remove(Autodesk.Navisworks.Api.SavedItem item)`

Removes specified item from Value.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SavedItem)

**Returns:** `bool` — false if item not found in Value

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints.Remove%28Autodesk.Navisworks.Api.SavedItem%29)

#### `void RemoveAt(Autodesk.Navisworks.Api.GroupItem parent, int index)`

Removes item from the specified parent group contained by SavedViewpoints.

**Parameters:**
- `parent` (Autodesk.Navisworks.Api.GroupItem) — The parent group to remove from. RootItem or null to remove directly from Value
- `index` (int)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints.RemoveAt%28Autodesk.Navisworks.Api.GroupItem%2CSystem.Int32%29)

#### `void RemoveAt(int index)`

Removes item at the specified index from Value.

**Parameters:**
- `index` (int)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints.RemoveAt%28System.Int32%29)

#### `void ReplaceFromCurrentView(Autodesk.Navisworks.Api.SavedViewpoint savedViewpoint)`

Replaces the SavedViewpoint within Value with an updated copy. Viewpoint, Redlines and visibility are updated to those in the current View.

**Parameters:**
- `savedViewpoint` (Autodesk.Navisworks.Api.SavedViewpoint)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints.ReplaceFromCurrentView%28Autodesk.Navisworks.Api.SavedViewpoint%29)

#### `void ReplaceWithCopy(Autodesk.Navisworks.Api.GroupItem parent, int index, Autodesk.Navisworks.Api.SavedItem item)`

Replaces item from the specified parent group with a copy of item

**Parameters:**
- `parent` (Autodesk.Navisworks.Api.GroupItem) — The parent group to insert into. RootItem or null to insert directly into Value
- `index` (int)
- `item` (Autodesk.Navisworks.Api.SavedItem)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints.ReplaceWithCopy%28Autodesk.Navisworks.Api.GroupItem%2CSystem.Int32%2CAutodesk.Navisworks.Api.SavedItem%29)

#### `void ReplaceWithCopy(int index, Autodesk.Navisworks.Api.SavedItem item)`

Replaces item at given index within Value with a copy of item

**Parameters:**
- `index` (int)
- `item` (Autodesk.Navisworks.Api.SavedItem)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints.ReplaceWithCopy%28System.Int32%2CAutodesk.Navisworks.Api.SavedItem%29)

#### `Autodesk.Navisworks.Api.SavedItem ResolveGuid(System.Guid value)`

Returns the saved item referenced by a guid.

**Parameters:**
- `value` (System.Guid)

**Returns:** `Autodesk.Navisworks.Api.SavedItem` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints.ResolveGuid%28System.Guid%29)

#### `Autodesk.Navisworks.Api.SavedItem ResolveIndexPath(System.Collections.Generic.IEnumerable<int> path)`

Returns the saved item referenced by a collection of indices that define a path from the root item through its descendents to the specified item.

**Parameters:**
- `path` (System.Collections.Generic.IEnumerable<int>)

**Returns:** `Autodesk.Navisworks.Api.SavedItem` — null if source cannot be resolved

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints.ResolveIndexPath%28System.Collections.Generic.IEnumerable%7BSystem.Int32%7D%29)

#### `Autodesk.Navisworks.Api.SavedItem ResolveReference(Autodesk.Navisworks.Api.SavedItemReference reference)`

Returns the saved item referenced by a SavedItemReference.

**Parameters:**
- `reference` (Autodesk.Navisworks.Api.SavedItemReference)

**Returns:** `Autodesk.Navisworks.Api.SavedItem` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints.ResolveReference%28Autodesk.Navisworks.Api.SavedItemReference%29)

#### `Autodesk.Navisworks.Api.SavedItemCollection ToSavedItemCollection()`

Explicit form of conversion operator returning Value

**Returns:** `Autodesk.Navisworks.Api.SavedItemCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints.ToSavedItemCollection)

#### `static Autodesk.Navisworks.Api.SavedItemCollection op_Implicit(Autodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints that)`

Conversion operator returning Value

**Parameters:**
- `that` (Autodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints)

**Returns:** `Autodesk.Navisworks.Api.SavedItemCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints.op_Implicit%28Autodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints%29~Autodesk.Navisworks.Api.SavedItemCollection)

### Properties
- `CurrentSavedViewpoint` (Autodesk.Navisworks.Api.SavedItem, get/set) — The current item as displayed in the GUI. Must exist in Value or be null.
- `Id` (string, get) — The internal identifier for this type of document part.
- `RootItem` (Autodesk.Navisworks.Api.FolderItem, get) — Value as the children of a FolderItem
- `Value` (Autodesk.Navisworks.Api.SavedItemCollection, get) — The collection of SavedViewpoints stored in the Document

### Events
#### `Changed` (`System.EventHandler<Autodesk.Navisworks.Api.SavedItemChangedEventArgs>`)

**Signature:** `event System.EventHandler<Autodesk.Navisworks.Api.SavedItemChangedEventArgs> Changed`

Occurs when the SavedViewpoints contained in this object has changed

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints.Changed)

#### `Changing` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> Changing`

Occurs when the SavedViewpoint contained in this object is about to change

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints.Changing)

#### `CurrentSavedViewpointChanged` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> CurrentSavedViewpointChanged`

Occurs when CurrentSavedViewpoint has changed.

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSavedViewpoints.CurrentSavedViewpointChanged)

## DocumentSelectionSets (class)

The SelectionSets stored in a Document

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets)

### Methods
#### `void AddComment(Autodesk.Navisworks.Api.SavedItem item, Autodesk.Navisworks.Api.Comment comment)`

Edit item and add Comment to item's Comments

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SavedItem)
- `comment` (Autodesk.Navisworks.Api.Comment)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets.AddComment%28Autodesk.Navisworks.Api.SavedItem%2CAutodesk.Navisworks.Api.Comment%29)

#### `void AddCopy(Autodesk.Navisworks.Api.GroupItem parent, Autodesk.Navisworks.Api.SavedItem item)`

Adds a copy of item to the end of the specified parent group contained by SelectionSets.

**Parameters:**
- `parent` (Autodesk.Navisworks.Api.GroupItem) — The parent group to add to. RootItem or null to add directly to Value
- `item` (Autodesk.Navisworks.Api.SavedItem)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets.AddCopy%28Autodesk.Navisworks.Api.GroupItem%2CAutodesk.Navisworks.Api.SavedItem%29)

#### `void AddCopy(Autodesk.Navisworks.Api.SavedItem item)`

Adds a copy of item to the end of Value.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SavedItem)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets.AddCopy%28Autodesk.Navisworks.Api.SavedItem%29)

#### `void Clear()`

Removes all items in Value

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets.Clear)

#### `void CopyFrom(Autodesk.Navisworks.Api.SavedItemCollection value)`

Copies the SelectionSets from value and replaces the list of entries held in Value.

**Parameters:**
- `value` (Autodesk.Navisworks.Api.SavedItemCollection)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets.CopyFrom%28Autodesk.Navisworks.Api.SavedItemCollection%29)

#### `void CopyFrom(System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.SavedItem> value)`

Copies the SelectionSets from value and replaces the list of entries held in Value.

**Parameters:**
- `value` (System.Collections.Generic.IEnumerable<Autodesk.Navisworks.Api.SavedItem>)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets.CopyFrom%28System.Collections.Generic.IEnumerable%7BAutodesk.Navisworks.Api.SavedItem%7D%29)

#### `System.Collections.ObjectModel.Collection<Autodesk.Navisworks.Api.SavedItem> CreateCopy()`

Creates a copy of the SelectionSets associated with this object

**Returns:** `System.Collections.ObjectModel.Collection<Autodesk.Navisworks.Api.SavedItem>` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets.CreateCopy)

#### `System.Collections.ObjectModel.Collection<int> CreateIndexPath(Autodesk.Navisworks.Api.SavedItem item)`

Returns a collection of indices that defines a path from the root item through its descendents to the specified item.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SavedItem)

**Returns:** `System.Collections.ObjectModel.Collection<int>` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets.CreateIndexPath%28Autodesk.Navisworks.Api.SavedItem%29)

#### `Autodesk.Navisworks.Api.SavedItemReference CreateReference(Autodesk.Navisworks.Api.SavedItem item)`

Returns a reference to a saved item.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SavedItem)

**Returns:** `Autodesk.Navisworks.Api.SavedItemReference` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets.CreateReference%28Autodesk.Navisworks.Api.SavedItem%29)

#### `Autodesk.Navisworks.Api.SelectionSource CreateSelectionSource(Autodesk.Navisworks.Api.SavedItem item)`

Returns a selection source that refers to the selection defined by a Selection Set or folder of Selection Sets.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SavedItem)

**Returns:** `Autodesk.Navisworks.Api.SelectionSource` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets.CreateSelectionSource%28Autodesk.Navisworks.Api.SavedItem%29)

#### `void EditComments(Autodesk.Navisworks.Api.SavedItem item, Autodesk.Navisworks.Api.CommentCollection newComments)`

Edit item and update it's Comments

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SavedItem)
- `newComments` (Autodesk.Navisworks.Api.CommentCollection)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets.EditComments%28Autodesk.Navisworks.Api.SavedItem%2CAutodesk.Navisworks.Api.CommentCollection%29)

#### `void EditDisplayName(Autodesk.Navisworks.Api.SavedItem item, string newDisplayName)`

Edit item and update it's DisplayName

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SavedItem)
- `newDisplayName` (string)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets.EditDisplayName%28Autodesk.Navisworks.Api.SavedItem%2CSystem.String%29)

#### `void InsertCopy(Autodesk.Navisworks.Api.GroupItem parent, int index, Autodesk.Navisworks.Api.SavedItem item)`

Inserts a copy of item into the specified parent group contained by SelectionSets.

**Parameters:**
- `parent` (Autodesk.Navisworks.Api.GroupItem) — The parent group to insert into. RootItem or null to insert directly into Value
- `index` (int)
- `item` (Autodesk.Navisworks.Api.SavedItem)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets.InsertCopy%28Autodesk.Navisworks.Api.GroupItem%2CSystem.Int32%2CAutodesk.Navisworks.Api.SavedItem%29)

#### `void InsertCopy(int index, Autodesk.Navisworks.Api.SavedItem item)`

Inserts a copy of item into Value.

**Parameters:**
- `index` (int)
- `item` (Autodesk.Navisworks.Api.SavedItem)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets.InsertCopy%28System.Int32%2CAutodesk.Navisworks.Api.SavedItem%29)

#### `void Move(Autodesk.Navisworks.Api.GroupItem oldParent, int oldIndex, Autodesk.Navisworks.Api.GroupItem newParent, int newIndex)`

Moves item at oldIndex in OldParent to newIndex in newParent

**Parameters:**
- `oldParent` (Autodesk.Navisworks.Api.GroupItem) — The parent group to move the item from. RootItem or null to remove directly from Value
- `oldIndex` (int)
- `newParent` (Autodesk.Navisworks.Api.GroupItem) — The parent group to move to. RootItem or null to move directly into Value.
If newParent is equal to oldParent, the item will be moved within the same SavedItemCollection.
- `newIndex` (int)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets.Move%28Autodesk.Navisworks.Api.GroupItem%2CSystem.Int32%2CAutodesk.Navisworks.Api.GroupItem%2CSystem.Int32%29)

#### `void Move(int oldIndex, int newIndex)`

Moves item from oldIndex to newIndex within Value

**Parameters:**
- `oldIndex` (int)
- `newIndex` (int)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets.Move%28System.Int32%2CSystem.Int32%29)

#### `bool Remove(Autodesk.Navisworks.Api.GroupItem parent, Autodesk.Navisworks.Api.SavedItem item)`

Removes specified item from the specified parent group contained by SelectionSets.

**Parameters:**
- `parent` (Autodesk.Navisworks.Api.GroupItem) — The parent group to remove from. RootItem or null to remove directly from Value
- `item` (Autodesk.Navisworks.Api.SavedItem)

**Returns:** `bool` — false if item not found in Value

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets.Remove%28Autodesk.Navisworks.Api.GroupItem%2CAutodesk.Navisworks.Api.SavedItem%29)

#### `bool Remove(Autodesk.Navisworks.Api.SavedItem item)`

Removes specified item from Value.

**Parameters:**
- `item` (Autodesk.Navisworks.Api.SavedItem)

**Returns:** `bool` — false if item not found in Value

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets.Remove%28Autodesk.Navisworks.Api.SavedItem%29)

#### `void RemoveAt(Autodesk.Navisworks.Api.GroupItem parent, int index)`

Removes item from the specified parent group contained by SelectionSets.

**Parameters:**
- `parent` (Autodesk.Navisworks.Api.GroupItem) — The parent group to remove from. RootItem or null to remove directly from Value
- `index` (int)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets.RemoveAt%28Autodesk.Navisworks.Api.GroupItem%2CSystem.Int32%29)

#### `void RemoveAt(int index)`

Removes item at the specified index from Value.

**Parameters:**
- `index` (int)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets.RemoveAt%28System.Int32%29)

#### `void ReplaceWithCopy(Autodesk.Navisworks.Api.GroupItem parent, int index, Autodesk.Navisworks.Api.SavedItem item)`

Replaces item from the specified parent group with a copy of item

**Parameters:**
- `parent` (Autodesk.Navisworks.Api.GroupItem) — The parent group to insert into. RootItem or null to insert directly into Value
- `index` (int)
- `item` (Autodesk.Navisworks.Api.SavedItem)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets.ReplaceWithCopy%28Autodesk.Navisworks.Api.GroupItem%2CSystem.Int32%2CAutodesk.Navisworks.Api.SavedItem%29)

#### `void ReplaceWithCopy(int index, Autodesk.Navisworks.Api.SavedItem item)`

Replaces item at given index within Value with a copy of item

**Parameters:**
- `index` (int)
- `item` (Autodesk.Navisworks.Api.SavedItem)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets.ReplaceWithCopy%28System.Int32%2CAutodesk.Navisworks.Api.SavedItem%29)

#### `Autodesk.Navisworks.Api.SavedItem ResolveGuid(System.Guid value)`

Returns the saved item referenced by a guid.

**Parameters:**
- `value` (System.Guid)

**Returns:** `Autodesk.Navisworks.Api.SavedItem` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets.ResolveGuid%28System.Guid%29)

#### `Autodesk.Navisworks.Api.SavedItem ResolveIndexPath(System.Collections.Generic.IEnumerable<int> path)`

Returns the saved item referenced by a collection of indices that define a path from the root item through its descendents to the specified item.

**Parameters:**
- `path` (System.Collections.Generic.IEnumerable<int>)

**Returns:** `Autodesk.Navisworks.Api.SavedItem` — null if source cannot be resolved

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets.ResolveIndexPath%28System.Collections.Generic.IEnumerable%7BSystem.Int32%7D%29)

#### `Autodesk.Navisworks.Api.SavedItem ResolveReference(Autodesk.Navisworks.Api.SavedItemReference reference)`

Returns the saved item referenced by a SavedItemReference.

**Parameters:**
- `reference` (Autodesk.Navisworks.Api.SavedItemReference)

**Returns:** `Autodesk.Navisworks.Api.SavedItem` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets.ResolveReference%28Autodesk.Navisworks.Api.SavedItemReference%29)

#### `Autodesk.Navisworks.Api.SavedItem ResolveSelectionSource(Autodesk.Navisworks.Api.SelectionSource source)`

Returns the saved item referenced by a SelectionSource

**Parameters:**
- `source` (Autodesk.Navisworks.Api.SelectionSource)

**Returns:** `Autodesk.Navisworks.Api.SavedItem` — null if source cannot be resolved

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets.ResolveSelectionSource%28Autodesk.Navisworks.Api.SelectionSource%29)

#### `Autodesk.Navisworks.Api.SavedItemCollection ToSavedItemCollection()`

Explicit form of conversion operator returning Value

**Returns:** `Autodesk.Navisworks.Api.SavedItemCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets.ToSavedItemCollection)

#### `static Autodesk.Navisworks.Api.SavedItemCollection op_Implicit(Autodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets that)`

Conversion operator returning Value

**Parameters:**
- `that` (Autodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets)

**Returns:** `Autodesk.Navisworks.Api.SavedItemCollection` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets.op_Implicit%28Autodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets%29~Autodesk.Navisworks.Api.SavedItemCollection)

### Properties
- `Id` (string, get) — The internal identifier for this type of document part.
- `RootItem` (Autodesk.Navisworks.Api.FolderItem, get) — Value as the children of a FolderItem
- `Value` (Autodesk.Navisworks.Api.SavedItemCollection, get) — The collection of SelectionSets stored in the Document

### Events
#### `Changed` (`System.EventHandler<Autodesk.Navisworks.Api.SavedItemChangedEventArgs>`)

**Signature:** `event System.EventHandler<Autodesk.Navisworks.Api.SavedItemChangedEventArgs> Changed`

Occurs when the SelectionSets contained in this object has changed

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets.Changed)

#### `Changing` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> Changing`

Occurs when the Selection contained in this object is about to change

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentSelectionSets.Changing)

## DocumentTool (class)

The active tool in a Document

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.DocumentParts.DocumentTool)

### Methods
#### `void Set(Autodesk.Navisworks.Api.Tool tool)`

Explicit method that sets Value

**Parameters:**
- `tool` (Autodesk.Navisworks.Api.Tool)

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentTool.Set%28Autodesk.Navisworks.Api.Tool%29)

#### `void SetCustomToolPlugin(Autodesk.Navisworks.Api.Plugins.ToolPlugin toolPlugin)`

Sets the current Tool to a custom ToolPlugin.

**Parameters:**
- `toolPlugin` (Autodesk.Navisworks.Api.Plugins.ToolPlugin) — Loaded ToolPlugin

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentTool.SetCustomToolPlugin%28Autodesk.Navisworks.Api.Plugins.ToolPlugin%29)

#### `Autodesk.Navisworks.Api.Tool ToTool()`

Explicit form of conversion operator returning Value

**Returns:** `Autodesk.Navisworks.Api.Tool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentTool.ToTool)

#### `static Autodesk.Navisworks.Api.Tool op_Implicit(Autodesk.Navisworks.Api.DocumentParts.DocumentTool that)`

Conversion operator returning Value

**Parameters:**
- `that` (Autodesk.Navisworks.Api.DocumentParts.DocumentTool)

**Returns:** `Autodesk.Navisworks.Api.Tool` — 

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#M%3AAutodesk.Navisworks.Api.DocumentParts.DocumentTool.op_Implicit%28Autodesk.Navisworks.Api.DocumentParts.DocumentTool%29~Autodesk.Navisworks.Api.Tool)

### Properties
- `CustomToolPluginId` (string, get) — For the case where Value is CustomToolPlugin, this will return the the passed in ToolPlugin identifier.
- `Value` (Autodesk.Navisworks.Api.Tool, get/set) — The value of the active Tool within the document Attempts to set Value to CustomToolPlugin will be ignored.

### Events
#### `Changed` (`System.EventHandler<System.EventArgs>`)

**Signature:** `event System.EventHandler<System.EventArgs> Changed`

Occurs when the current Tool has changed

[Docs](https://aps.autodesk.com/developer/overview/navisworks-api#E%3AAutodesk.Navisworks.Api.DocumentParts.DocumentTool.Changed)

## IDocumentClash (interface)

Discoverable Document Part for Clash Detective

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.DocumentParts.IDocumentClash)

## IDocumentTakeoff (interface)

Discoverable Document Part for Takeoff

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.DocumentParts.IDocumentTakeoff)

## IDocumentTimeliner (interface)

Discoverable Document Part for Timeliner

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.DocumentParts.IDocumentTimeliner)

## ModelItemPathId (class)

Represent a combined id with model index and the path identifier.

[Vendor docs](https://aps.autodesk.com/developer/overview/navisworks-api#T:Autodesk.Navisworks.Api.DocumentParts.ModelItemPathId)

### Constructors
- `ModelItemPathId()` — Constructor for a ModelItemPathId.

### Properties
- `ModelIndex` (int, get/set) — The model index
- `PathId` (string, get/set) — The path identifier

