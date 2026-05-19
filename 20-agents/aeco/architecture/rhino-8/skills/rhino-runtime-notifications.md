---
name: rhino-rhino-runtime-notifications
description: This skill encodes the rhino 8.0 surface of the Rhino.Runtime.Notifications namespace — 7 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Notification, NotificationButtonClickedArgs, NotificationCenter, TrulyObservableOrderedSet<T>, IAssemblyRestrictedObject, ButtonType, Notification.Severity.
---

# Rhino.Runtime.Notifications

Auto-generated from vendor docs for rhino 8.0. 7 types in this namespace.

## ButtonType (enum)

The type of button in a notification.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_Notifications_ButtonType.htm)

### Values
- `CancelOrClose` = `0` — Denotes either the Cancel button as well as, on some platforms, the close button if present.
- `Confirm` = `1` — The Confirm buttton.
- `Alternate` = `2` — The Alternate button.

## IAssemblyRestrictedObject (interface)

A class that implements this interface signals its clients that its instances can only be modified by certain assemblies. This is useful in cases where only certain assemblies should be able to modify an object. The actual members of an instance that are restricted are left to the discretion of the instance's class, and should be documented.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_Notifications_IAssemblyRestrictedObject.htm)

### Methods
#### `bool Editable()`

Determines whether an assembly can modify the instance.

**Returns:** `Boolean` — true if the instance can be edited by the assembly, otherwise returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Notifications_IAssemblyRestrictedObject_Editable.htm)

## Notification (class)

A Notification instance can be used to inform the user about various events. For a Notification instance to be displayed in Rhino, it must be added to the NotificationCenter. When added, it will be displayed in the Notifications panel in Rhino. A Notification contains 1 to 3 buttons that are automatically wired to its ButtonClicked Action if it is not null. The buttons are displayed when the Notification is shown modally by either the user clicking on a particular notification in the Notifications panel, or by programatically showing it using ShowModal(). Currently, only process-wide notifications are supported; document specific notifications are not possible. Notification instances contain metadata that can be added, modified, or removed during its life. The metadata is important for LINQ queries and other patterns. For example, a particular action may require that multiple notifications be modified. Thus, a LINQ query can be performed on the NotificationCenter using metadata to retrieve related Notification objects and modify them as a batch. Notification objects implement IAssemblyRestrictedObject. By default, a Notification can be editedby any assembly, but explicitly specifing allowed assemblies in the constructor changes this behavior. Notification objects are not thread-safe and should only be manipulated in UI thread.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_Notifications_Notification.htm)

### Constructors
- `public Notification()` — Creates a new instance that can be edited by any assembly.
- `public Notification(IEnumerable<Assembly> allowedAssemblies)` — Creates a new instance that can be edited by the given assemblies.

### Methods
#### `public bool Editable()`

Determines whether an assembly can modify the instance. Any code that modifies an assembly protected notification must be wrapped in a ExecuteAssemblyProtectedCode(Action) method.

**Remarks:** Before modifying a notification you are not sure you created (such as a notification returned from a LINQ query), you should call this method first to ensure you can indeed edit the object.

**Returns:** `Boolean` — [Missing <returns> documentation for "M:Rhino.Runtime.Notifications.Notification.Editable"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Notifications_Notification_Editable.htm)

#### `public static void ExecuteAssemblyProtectedCode(Action action)`

If a Notification object is only allowed to be modified by certain assemblies, then any code that interacts with it must be wrapped around this method, or a InvalidOperationException will be thrown. For performance reasons, the code wrapped by this method should be kept as simple as possible.

**Remarks:** This method is not thread-safe and should only be manipulated in UI thread.

**Parameters:**
- `action` (System.Action) — The code to run that modifies one or more notification objects

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Notifications_Notification_ExecuteAssemblyProtectedCode.htm)

#### `public static TResult ExecuteAssemblyProtectedCode<TResult>(Func<TResult> func)`

If a Notification object is only allowed to be modified by certain assemblies, then any code that interacts with it must be wrapped around this method, or a InvalidOperationException will be thrown. For performance reasons, the code wrapped by this method should be kept as simple as possible.

**Remarks:** This method is not thread-safe and should only be manipulated in UI thread.

**Parameters:**
- `func` (System.Func<TResult>) — The code to run that modifies one or more notification objects

**Returns:** `TResult` — [Missing <returns> documentation for "M:Rhino.Runtime.Notifications.Notification.ExecuteAssemblyProtectedCode``1(System.Func{``0})"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Notifications_Notification_ExecuteAssemblyProtectedCode__1.htm)

#### `public void HideModal()`

Tells Rhino to hide the notification if it is being currently shown as a modal.

**Remarks:** If the notification is not being currently shown but is being queued by Rhino as a result of calling ShowModal(), then the notification will be dequeued. If the notification was never queued, then this method has no effect.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Notifications_Notification_HideModal.htm)

#### `public bool RemoveMetadata(string key)`

Removes metadata from this instance.

**Parameters:**
- `key` (System.String) — The key of the metadata to remove.

**Returns:** `Boolean` — true if the metada was removed; otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Notifications_Notification_RemoveMetadata.htm)

#### `public void ShowModal()`

Tells Rhino to display the notification modally.

**Remarks:** The notification will only be displayed if/once the instance is added to the NotificationCenter. Rhino keeps a queue of notifications that need to be shown modally, so calling this method does not mean that this particular notification will be immediately displayed.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Notifications_Notification_ShowModal.htm)

#### `public override string ToString()`

Returns a readable string representation of the instance.

**Returns:** `String` — [Missing <returns> documentation for "M:Rhino.Runtime.Notifications.Notification.ToString"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Notifications_Notification_ToString.htm)

### Properties
- `AllowedAssemblies` (ICollection<Assembly>, get) — The assemblies that can modify this instance.
- `AlternateButtonTitle` (String, get/set) — The localized title of the Alternate button.
- `ButtonClicked` (Action<ButtonType>, get/set) — An Action that will be invoked whenever a button for the notification is clicked or the notification is closed.
- `CancelButtonTitle` (String, get/set) — The localized title of the Cancel button.
- `ConfirmButtonTitle` (String, get/set) — The localized title of the Confirm button.
- `DateUpdated` (DateTime, get) — The date the notification was last modified.
- `Description` (String, get/set) — The description of the notification. The description is displayed in the Notifications panel in Rhino.
- `Item` (String, get/set) — Gets or sets metadata for this instance.
- `Message` (String, get/set) — The message of the notification. The message is shown only when the instance is displayed modally. It should contain details about the notification.
- `MetadataCopy` (IDictionary<String,String>, get) — A copy of all the metadata for this class.
- `SeverityLevel` (Notification.Severity, get/set) — The severity of the notification. Changing the severity of the notification may change the way Rhino chooses to display the Notifications panel.
- `ShowEventId` (Nullable<Guid>, get) — A field used by Rhino for displaying notifications. Not intended for public use.
- `Title` (String, get/set) — The title of the notification. The title is displayed when the notification is displayed modally in Rhino.

### Events
#### `PropertyChanged` (`System.ComponentModel.PropertyChangedEventHandler`)

**Signature:** `public event PropertyChangedEventHandler PropertyChanged`

Triggered whenever a visible property of the instance changes.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_Runtime_Notifications_Notification_PropertyChanged.htm)

## Notification.Severity (enum)

Determines the severity of a notification.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_Notifications_Notification_Severity.htm)

### Values
- `Debug` = `0` — Least serious.
- `Info` = `1` — Not serious.
- `Warning` = `2` — Important.
- `Serious` = `3` — Very important.
- `Critical` = `4` — Extremely important.

## NotificationButtonClickedArgs (class)

Used when a button is clicked for a notification.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_Notifications_NotificationButtonClickedArgs.htm)

### Constructors
- `public NotificationButtonClickedArgs(Notification notification, ButtonType buttonClicked)` — Creates a new instance.

### Properties
- `ButtonClicked` (ButtonType, get) — The button that was clicked.
- `Notification` (Notification, get) — The notification whose button was clicked.

## NotificationCenter (class)

The NotificationCenter holds all Notification objects that are displayed in the Notifications panel by Rhino. The NotificationCenter is not thread-safe and should only be used in the UI thread.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_Notifications_NotificationCenter.htm)

### Properties
- `Notifications` (TrulyObservableOrderedSet<Notification>, get) — A set containing all the Notification instances. Any Notification added will be displayed in the Notifications panel. Any Notification removed will be removed from the Notifications panel, and, if shown modally or queued to be shown modally, will be closed or dequeued from the modal queue.

## TrulyObservableOrderedSet<T> (class)

An ordered set that notifies its subscribers whenever one of its INotifyPropertyChanged elements raises its PropertyChanged event.

**Remarks:** This class prevents the removal of elements by assemblies they cannot be edited by. At the time of removal, each element's Editable() method will be invoked, and if false, an InvalidOperationException will be thrown.

[Vendor docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/T_Rhino_Runtime_Notifications_TrulyObservableOrderedSet_1.htm)

### Constructors
- `public TrulyObservableOrderedSet()` — Creates an empty instance.
- `public TrulyObservableOrderedSet(IEnumerable<T> items)` — Creates an instance with the given items.

### Methods
#### `public void Add(T item)`

Adds an object to the end of the ordered set if the set does not already contain the item.

**Parameters:**
- `item` (T) — The item to add.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Notifications_TrulyObservableOrderedSet_1_Add.htm)

#### `public void Clear()`

Clears the ordered set.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Notifications_TrulyObservableOrderedSet_1_Clear.htm)

#### `public bool Contains(T item)`

Determines whether an element is in the set.

**Parameters:**
- `item` (T) — The item to check for inclusion.

**Returns:** `Boolean` — True if the item is in the set; otherwise false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Notifications_TrulyObservableOrderedSet_1_Contains.htm)

#### `public void CopyTo(T[] array, int arrayIndex)`

Copies the values of the set to an array.

**Parameters:**
- `array` (T[]) — The array to copy the values to.
- `arrayIndex` (System.Int32) — The index of the array to start the copy.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Notifications_TrulyObservableOrderedSet_1_CopyTo.htm)

#### `public IEnumerator<T> GetEnumerator()`

Returns an enumerator that iterates through the set.

**Returns:** `IEnumerator<T>` — [Missing <returns> documentation for "M:Rhino.Runtime.Notifications.TrulyObservableOrderedSet`1.GetEnumerator"]

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Notifications_TrulyObservableOrderedSet_1_GetEnumerator.htm)

#### `public int IndexOf(T item)`

Searches for the specified object and returns the zero-based index of the first occurrence.

**Parameters:**
- `item` (T) — The item to locate.

**Returns:** `Int32` — The zero-based index of the first occurrence of item if found; otherwise -1.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Notifications_TrulyObservableOrderedSet_1_IndexOf.htm)

#### `public void Insert(int index, T item)`

Inserts an element at the specified index.

**Parameters:**
- `index` (System.Int32) — The index to insert the element at.
- `item` (T) — The item to insert.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Notifications_TrulyObservableOrderedSet_1_Insert.htm)

#### `public bool Remove(T item)`

Removes an element from the set.

**Parameters:**
- `item` (T) — The element to remove.

**Returns:** `Boolean` — Returns true if the element was removed; otherwise returns false.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Notifications_TrulyObservableOrderedSet_1_Remove.htm)

#### `public void RemoveAt(int index)`

Removes an element at the specified index from the set.

**Parameters:**
- `index` (System.Int32) — The index of the element to remove.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Notifications_TrulyObservableOrderedSet_1_RemoveAt.htm)

#### `public void Sort<TKey>(Func<T, TKey> keySelector, bool descending = false)`

Sorts the set.

**Parameters:**
- `keySelector` (System.Func<T,TKey>) — [Missing <param name="keySelector"/> documentation for "M:Rhino.Runtime.Notifications.TrulyObservableOrderedSet`1.Sort``1(System.Func{`0,``0},System.Boolean)"]
- `descending` (System.Boolean) — If true, the sort will happen in descending other; if false, it will happen in ascending order.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/M_Rhino_Runtime_Notifications_TrulyObservableOrderedSet_1_Sort__1.htm)

### Properties
- `Count` (Int32, get) — Returns the total number of items in the set.
- `IsReadOnly` (Boolean, get) — Always returns false.
- `Item` (T, get/set) — Gets or replaces an element in the ordered set at the specified index.

### Events
#### `CollectionChanged` (`System.Collections.Specialized.NotifyCollectionChangedEventHandler`)

**Signature:** `public event NotifyCollectionChangedEventHandler CollectionChanged`

Triggered whenever the set changes or when one of its elements has its PropertyChanged event triggered.

[Docs](https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/E_Rhino_Runtime_Notifications_TrulyObservableOrderedSet_1_CollectionChanged.htm)

