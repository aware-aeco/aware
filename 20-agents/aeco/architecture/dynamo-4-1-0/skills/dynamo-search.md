---
name: dynamo-dynamo-search
description: This skill encodes the dynamo 4.1.0 surface of the Dynamo.Search namespace — 10 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BrowserInternalElement, BrowserItem, EntryDictionary<V>, ISearchCategory<TEntry>, ISearchEntry, ISource<TItem>, LuceneSearch, NodeSearchModel, and 2 more types.
---

# Dynamo.Search

Auto-generated from vendor docs for dynamo 4.1.0. 10 types in this namespace.

## BrowserInternalElement (class)

This class represents internal elements of browser item

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/BrowserInternalElement.cs)

### Properties
- `FullCategoryName` (string, get/set) — Returns full category name consisting of root and all sub categories names
- `Items` (System.Collections.ObjectModel.ObservableCollection<Dynamo.Search.BrowserItem>, get/set) — The items inside of the browser item
- `Name` (string, get) — Returns name of the node
- `OldParent` (Dynamo.Search.BrowserItem, get/set) — Returns previous parent item
- `Parent` (Dynamo.Search.BrowserItem, get/set) — Returns browser item representing category which this element belongs to
- `Siblings` (System.Collections.ObjectModel.ObservableCollection<Dynamo.Search.BrowserItem>, get) — Returns items which are in the same category as the browser item
- `UIParent` (object, get/set) — The framework element hosting the extension

## BrowserItem (class)

Base class for all browser items in search area.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/BrowserItem.cs)

### Constructors
- `void BrowserItem()` — BrowserItem.BrowserItem

### Methods
#### `void OnExecuted()`

BrowserItem.OnExecuted

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/BrowserItem.cs)

### Properties
- `Height` (int, get/set) — The height of the element in search
- `IsExpanded` (bool, get/set) — Is the element expanded in the browser
- `IsSelected` (bool, get/set) — Whether the item is selected or not
- `Items` (System.Collections.ObjectModel.ObservableCollection<Dynamo.Search.BrowserItem>, get/set) — Returns items inside of the browser item
- `Name` (string, get) — A name (title) field for the BrowserItem
- `Visibility` (bool, get/set) — Whether the item is visible or not

### Events
#### `Executed` (`Dynamo.Search.BrowserItem.BrowserItemHandler`)

**Signature:** `public event Dynamo.Search.BrowserItem.BrowserItemHandler Executed`

Occurs when corresponding node is created

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/BrowserItem.cs)

## EntryDictionary<V> (class)

A dictionary of objects.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/EntryDictionary.cs)

### Methods
#### `void OnEntryAdded(V entry)`

EntryDictionary.OnEntryAdded

**Parameters:**
- `entry` (V)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/EntryDictionary.cs)

#### `void OnEntryRemoved(V entry)`

EntryDictionary.OnEntryRemoved

**Parameters:**
- `entry` (V)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/EntryDictionary.cs)

#### `void OnEntryUpdated(V entry)`

EntryDictionary.OnEntryUpdated

**Parameters:**
- `entry` (V)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/EntryDictionary.cs)

### Properties
- `Entries` (System.Collections.Generic.IEnumerable<V>, get) — All the current entries.
- `NumElements` (int, get) — The number of elements in the dictionary
- `NumTags` (int, get) — The number of tags in the dictionary

### Events
#### `EntryAdded` (`System.Action<V>`)

**Signature:** `public event System.Action<V> EntryAdded`

Event is fired when an entry is added.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/EntryDictionary.cs)

#### `EntryRemoved` (`System.Action<V>`)

**Signature:** `public event System.Action<V> EntryRemoved`

Event is fired when an entry is removed.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/EntryDictionary.cs)

#### `EntryUpdated` (`System.Action<V>`)

**Signature:** `public event System.Action<V> EntryUpdated`

Event is fired when an entry is updated.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/EntryDictionary.cs)

## ISearchCategory<TEntry> (interface)

A search category.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/ISearchCategory.cs)

### Properties
- `Entries` (System.Collections.Generic.IEnumerable<TEntry>, get) — Entries contained in this category.
- `Name` (string, get) — The name of this category.
- `SubCategories` (System.Collections.Generic.IEnumerable<Dynamo.Search.ISearchCategory<TEntry>>, get) — Sub-categories contained in this category

## ISearchEntry (interface)

Has a collection of strings that can be used to identifiy the instance in a search.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/ISearchEntry.cs)

### Properties
- `Description` (string, get) — Description of this search entry.
- `Name` (string, get) — Name of this search entry.
- `SearchTagWeights` (System.Collections.Generic.IEnumerable<double>, get) — Every search tag should have weight. This weight will be taken into account during search.
- `SearchTags` (System.Collections.Generic.ICollection<string>, get) — Tags which can be used to search for this entry.

## ISource<TItem> (interface)

Has an event that produces items.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/ISource.cs)

### Events
#### `ItemProduced` (`System.Action<TItem>`)

**Signature:** `public event System.Action<TItem> ItemProduced`

Produces items, potentially asynchronously.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/ISource.cs)

## LuceneSearch (class)

Singleton class that has access to all the lucene search utility classes.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/LuceneSearch.cs)

### Constructors
- `void LuceneSearch()` — LuceneSearch.LuceneSearch

### Properties
- `Instance` (Dynamo.Search.LuceneSearch, get) — The actual instance stored in the Singleton class

## NodeSearchModel (class)

Searchable library of NodeSearchElements that can produce NodeModels.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/NodeSearchModel.cs)

## SearchCategoryUtil (static-class)

Utility methods for categorizing search elements.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/SearchLibrary.cs)

## SearchLibrary<TEntry, TItem> (class)

Searchable library of item sources.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/SearchLibrary.cs)

### Methods
#### `void OnEntryAdded(TEntry entry)`

SearchLibrary.OnEntryAdded

**Parameters:**
- `entry` (TEntry)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/SearchLibrary.cs)

#### `void OnEntryRemoved(TEntry entry)`

SearchLibrary.OnEntryRemoved

**Parameters:**
- `entry` (TEntry)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/SearchLibrary.cs)

#### `void OnItemProduced(TItem item)`

SearchLibrary.OnItemProduced

**Parameters:**
- `item` (TItem)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/SearchLibrary.cs)

### Events
#### `ItemProduced` (`System.Action<TItem>`)

**Signature:** `public event System.Action<TItem> ItemProduced`

Produces an item whenever a search element produces an item.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/SearchLibrary.cs)

