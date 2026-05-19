---
name: dynamo-dynamo-utilities
description: This skill encodes the dynamo 4.1.1 surface of the Dynamo.Utilities namespace — 20 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: LuceneCustomAnalyzer, AssemblyConfiguration, AssemblyHelper, DataMarshaler, IRecursiveGrouping<T>, ITree<TNodeTag, TLeaf>, ExtensionMethods, GuidUtility, and 12 more types.
---

# Dynamo.Utilities

Auto-generated from vendor docs for dynamo 4.1.1. 20 types in this namespace.

## AssemblyConfiguration (class)

A tool for obtaining configuration values from the current Library.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/AssemblyConfiguration.cs)

### Methods
#### `string GetAppSetting(string key)`

AssemblyConfiguration.GetAppSetting

**Parameters:**
- `key` (string)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/AssemblyConfiguration.cs)

### Properties
- `Instance` (Dynamo.Utilities.AssemblyConfiguration, get) — AssemblyConfiguration.Instance

## AssemblyHelper (class)

Type AssemblyHelper

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/AssemblyHelper.cs)

### Constructors
- `void AssemblyHelper(string moduleRootFolder, System.Collections.Generic.IEnumerable<string> additionalResolutionPaths, bool testMode)` — AssemblyHelper.AssemblyHelper

### Methods
#### `System.Version GetDynamoVersion(bool includeRevisionNumber)`

AssemblyHelper.GetDynamoVersion

**Parameters:**
- `includeRevisionNumber` (bool)

**Returns:** `System.Version` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/AssemblyHelper.cs)

#### `System.Reflection.Assembly ResolveAssembly(object sender, System.ResolveEventArgs args)`

Handler to the ApplicationDomain's AssemblyResolve event. If an assembly's location cannot be resolved, an exception is thrown. Failure to resolve an assembly will leave Dynamo in a bad state, so we should throw an exception here which gets caught by our unhandled exception handler and presents the crash dialogue.

**Parameters:**
- `sender` (object) — 
- `args` (System.ResolveEventArgs) — 

**Returns:** `System.Reflection.Assembly` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/AssemblyHelper.cs)

## DataMarshaler (class)

Provides ability to register data marshalers which can then be used to marshal arbitrary data.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/DataMarshaler.cs)

### Constructors
- `void DataMarshaler()` — DataMarshaler.DataMarshaler

### Methods
#### `object Marshal(object obj)`

Marshals data using the registered marshalers. If no marshaler exists, data is returned unmodified.

**Parameters:**
- `obj` (object) — Data to marshal.

**Returns:** `object` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/DataMarshaler.cs)

#### `void RegisterMarshaler<T>(System.Converter<T, object> marshaler)`

Registers a new data marshaler for a given type.

**Parameters:**
- `marshaler` (System.Converter<T, object>) — Converter to be used to marshaling.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/DataMarshaler.cs)

#### `void RegisterMarshaler(System.Type t, System.Converter<object, object> marshaler)`

Registers a new data marshaler for a given type.

**Parameters:**
- `t` (System.Type) — Type to marshal.
- `marshaler` (System.Converter<object, object>) — Converter to be used to marshaling.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/DataMarshaler.cs)

#### `void UnregisterMarshalerOfType<T>()`

Unregisters a data marshaler for a given type.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/DataMarshaler.cs)

#### `void UnregisterMarshalerOfType(System.Type t)`

Unregisters a data marshaler for a given type.

**Parameters:**
- `t` (System.Type) — Type of data to unregister marshaling for.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/DataMarshaler.cs)

## Either (static-class)

Utility methods for working with Either instances.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Option.cs)

### Methods
#### `Dynamo.Utilities.IEither<TLeft, TRight> Left<TLeft, TRight>(TLeft value)`

Creates a new IEither(TLeft, TRight) instance containing a Left value.

**Parameters:**
- `value` (TLeft) — Left value to be stored in the new IEither(TLeft, TRight) instance.

**Returns:** `Dynamo.Utilities.IEither<TLeft, TRight>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Option.cs)

#### `void Match<TLeft, TRight>(Dynamo.Utilities.IEither<TLeft, TRight> either, System.Action<TLeft> leftAction, System.Action<TRight> rightAction)`

Performs an given Action based on whether the IEither(TLeft, TRight) contains a Left or Right value.

**Parameters:**
- `either` (Dynamo.Utilities.IEither<TLeft, TRight>) — An IEither(TLeft, TRight) instance.
- `leftAction` (System.Action<TLeft>) — Action used for a Left value.
- `rightAction` (System.Action<TRight>) — Action used for a Right value.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Option.cs)

#### `Dynamo.Utilities.IEither<TLeft, TRight> Right<TLeft, TRight>(TRight value)`

Creates a new IEither(TLeft, TRight) instance containing a Right value.

**Parameters:**
- `value` (TRight) — Right value to be stored in the new IEither(TLeft, TRight) instance.

**Returns:** `Dynamo.Utilities.IEither<TLeft, TRight>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Option.cs)

#### `Dynamo.Utilities.IEither<TNewLeft, TRight> SelectLeft<TOldLeft, TNewLeft, TRight>(Dynamo.Utilities.IEither<TOldLeft, TRight> either, System.Func<TOldLeft, TNewLeft> selector)`

Return an IEither(TNewLeft, TRight) instance by either passing the contained Left value to a conversion function, or propagating the Right value.

**Parameters:**
- `either` (Dynamo.Utilities.IEither<TOldLeft, TRight>) — An IEither(TOldLeft, TRight) instance.
- `selector` (System.Func<TOldLeft, TNewLeft>) — Function used to convert a Left value.

**Returns:** `Dynamo.Utilities.IEither<TNewLeft, TRight>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Option.cs)

#### `Dynamo.Utilities.IEither<TLeft, TNewRight> SelectRight<TLeft, TOldRight, TNewRight>(Dynamo.Utilities.IEither<TLeft, TOldRight> either, System.Func<TOldRight, TNewRight> selector)`

Return an IEither(TLeft, TNewRight) instance by either passing the contained Right value to a conversion function, or propagating the Left value.

**Parameters:**
- `either` (Dynamo.Utilities.IEither<TLeft, TOldRight>) — An IEither(TLeft, TOldRight) instance.
- `selector` (System.Func<TOldRight, TNewRight>) — Function used to convert a Right value.

**Returns:** `Dynamo.Utilities.IEither<TLeft, TNewRight>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Option.cs)

## ExtensionMethods (static-class)

Type ExtensionMethods

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/EnumerableExtensions.cs)

### Methods
#### `System.Collections.ObjectModel.Collection<T> AddRange<T>(System.Collections.ObjectModel.Collection<T> collection, System.Collections.Generic.IEnumerable<T> items)`

ExtensionMethods.AddRange

**Parameters:**
- `collection` (System.Collections.ObjectModel.Collection<T>)
- `items` (System.Collections.Generic.IEnumerable<T>)

**Returns:** `System.Collections.ObjectModel.Collection<T>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/EnumerableExtensions.cs)

#### `System.Collections.Generic.IEnumerable<T> AsSingleton<T>(T o)`

ExtensionMethods.AsSingleton

**Parameters:**
- `o` (T)

**Returns:** `System.Collections.Generic.IEnumerable<T>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/EnumerableExtensions.cs)

#### `System.Collections.Generic.IEnumerable<System.Collections.Generic.IEnumerable<T>> CartesianProduct<T>(System.Collections.Generic.IEnumerable<System.Collections.Generic.IEnumerable<T>> sequences)`

ExtensionMethods.CartesianProduct

**Parameters:**
- `sequences` (System.Collections.Generic.IEnumerable<System.Collections.Generic.IEnumerable<T>>)

**Returns:** `System.Collections.Generic.IEnumerable<System.Collections.Generic.IEnumerable<T>>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/EnumerableExtensions.cs)

#### `System.Collections.Generic.IEnumerable<Dynamo.Utilities.ExtensionMethods.IIndexed<T>> Enumerate<T>(System.Collections.Generic.IEnumerable<T> seq)`

ExtensionMethods.Enumerate

**Parameters:**
- `seq` (System.Collections.Generic.IEnumerable<T>)

**Returns:** `System.Collections.Generic.IEnumerable<Dynamo.Utilities.ExtensionMethods.IIndexed<T>>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/EnumerableExtensions.cs)

#### `System.Collections.Generic.IEnumerable<TNodeTag> GetAllTags<TNodeTag, TLeaf>(Dynamo.Utilities.ITree<TNodeTag, TLeaf> tree)`

Returns all the tags of an ITree.

**Parameters:**
- `tree` (Dynamo.Utilities.ITree<TNodeTag, TLeaf>) — 

**Returns:** `System.Collections.Generic.IEnumerable<TNodeTag>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/EnumerableExtensions.cs)

#### `string GetChildNodeDoubleValue(System.Xml.XmlNode nodeElement)`

ExtensionMethods.GetChildNodeDoubleValue

**Parameters:**
- `nodeElement` (System.Xml.XmlNode)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/EnumerableExtensions.cs)

#### `string GetChildNodeStringValue(System.Xml.XmlNode nodeElement)`

ExtensionMethods.GetChildNodeStringValue

**Parameters:**
- `nodeElement` (System.Xml.XmlNode)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/EnumerableExtensions.cs)

#### `string GetFullName(System.Delegate del)`

ExtensionMethods.GetFullName

**Parameters:**
- `del` (System.Delegate)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/EnumerableExtensions.cs)

#### `System.Collections.Generic.IEnumerable<Dynamo.Utilities.IRecursiveGrouping<Dynamo.Utilities.IEither<TNodeKey, System.Collections.Generic.IEnumerable<TLeaf>>>> GroupByRecursive<TLeaf, TNodeKey>(System.Collections.Generic.IEnumerable<TLeaf> allLeaves, System.Func<TLeaf, System.Collections.Generic.ICollection<TNodeKey>> keySelector)`

Contructs a tree by recursively grouping elements from a sequence. Essentially, performs a GroupBy operation, and then for each Grouping, performs another GroupBy, assuming there is a GroupBy key available for the sub-group.

**Parameters:**
- `allLeaves` (System.Collections.Generic.IEnumerable<TLeaf>) — 
- `keySelector` (System.Func<TLeaf, System.Collections.Generic.ICollection<TNodeKey>>) — 

**Returns:** `System.Collections.Generic.IEnumerable<Dynamo.Utilities.IRecursiveGrouping<Dynamo.Utilities.IEither<TNodeKey, System.Collections.Generic.IEnumerable<TLeaf>>>>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/EnumerableExtensions.cs)

#### `TTree GroupByRecursive<TLeaf, TNodeKey, TTree>(System.Collections.Generic.IEnumerable<TLeaf> allLeaves, System.Func<TLeaf, System.Collections.Generic.ICollection<TNodeKey>> keySelector, System.Func<TNodeKey, System.Collections.Generic.IEnumerable<TTree>, System.Collections.Generic.IEnumerable<TLeaf>, TTree> treeCreator, TNodeKey rootKey)`

Contructs a tree by recursively grouping elements from a sequence. Essentially, performs a GroupBy operation, and then for each Grouping, performs another GroupBy, assuming there is a GroupBy key available for the sub-group.

**Parameters:**
- `allLeaves` (System.Collections.Generic.IEnumerable<TLeaf>) — 
- `keySelector` (System.Func<TLeaf, System.Collections.Generic.ICollection<TNodeKey>>) — 
- `treeCreator` (System.Func<TNodeKey, System.Collections.Generic.IEnumerable<TTree>, System.Collections.Generic.IEnumerable<TLeaf>, TTree>)
- `rootKey` (TNodeKey)

**Returns:** `TTree` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/EnumerableExtensions.cs)

#### `int IndexOf<T>(System.Collections.Generic.IEnumerable<T> source, T value)`

Returns the index of an element in an IEnumerable

**Parameters:**
- `source` (System.Collections.Generic.IEnumerable<T>) — The IEnumerable
- `value` (T) — The value for which the index is sought

**Returns:** `int` — Zero or greater if in the IEnumerable, otherwise -1

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/EnumerableExtensions.cs)

#### `System.Collections.Generic.IEnumerable<System.Collections.Generic.IEnumerable<T>> LongestSet<T>(System.Collections.Generic.IEnumerable<System.Collections.Generic.IEnumerable<T>> sequences)`

Returns the longest list of arguments. For a set List of Lists like {a} {b1,b2,b3} {c1,c2} This will return a List of Lists of objects like: {a,b1,c1} {a,b2,c2} {a,b3,c2}

**Parameters:**
- `sequences` (System.Collections.Generic.IEnumerable<System.Collections.Generic.IEnumerable<T>>) — 

**Returns:** `System.Collections.Generic.IEnumerable<System.Collections.Generic.IEnumerable<T>>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/EnumerableExtensions.cs)

#### `System.Collections.ObjectModel.ObservableCollection<T> RemoveAll<T>(System.Collections.ObjectModel.ObservableCollection<T> coll, System.Predicate<T> predicate)`

An extension to the ObservableCollection class which allows you to remove all objects which don't pass a predicate method.

**Parameters:**
- `coll` (System.Collections.ObjectModel.ObservableCollection<T>) — The observable collection.
- `predicate` (System.Predicate<T>) — The predicate method.

**Returns:** `System.Collections.ObjectModel.ObservableCollection<T>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/EnumerableExtensions.cs)

#### `System.Collections.ObjectModel.ObservableCollection<T> RemoveRange<T>(System.Collections.ObjectModel.ObservableCollection<T> coll, int index, int count)`

ExtensionMethods.RemoveRange

**Parameters:**
- `coll` (System.Collections.ObjectModel.ObservableCollection<T>)
- `index` (int)
- `count` (int)

**Returns:** `System.Collections.ObjectModel.ObservableCollection<T>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/EnumerableExtensions.cs)

#### `System.Collections.Generic.IEnumerable<System.Collections.Generic.IEnumerable<T>> ShortestSet<T>(System.Collections.Generic.IEnumerable<System.Collections.Generic.IEnumerable<T>> sequences)`

ExtensionMethods.ShortestSet

**Parameters:**
- `sequences` (System.Collections.Generic.IEnumerable<System.Collections.Generic.IEnumerable<T>>)

**Returns:** `System.Collections.Generic.IEnumerable<System.Collections.Generic.IEnumerable<T>>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/EnumerableExtensions.cs)

#### `System.Collections.Generic.IEnumerable<System.Collections.Generic.IEnumerable<T>> SingleSet<T>(System.Collections.Generic.IEnumerable<System.Collections.Generic.IEnumerable<T>> sequences)`

ExtensionMethods.SingleSet

**Parameters:**
- `sequences` (System.Collections.Generic.IEnumerable<System.Collections.Generic.IEnumerable<T>>)

**Returns:** `System.Collections.Generic.IEnumerable<System.Collections.Generic.IEnumerable<T>>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/EnumerableExtensions.cs)

#### `System.Collections.ObjectModel.ObservableCollection<T> ToObservableCollection<T>(System.Collections.Generic.IEnumerable<T> coll)`

ExtensionMethods.ToObservableCollection

**Parameters:**
- `coll` (System.Collections.Generic.IEnumerable<T>)

**Returns:** `System.Collections.ObjectModel.ObservableCollection<T>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/EnumerableExtensions.cs)

#### `Dynamo.Utilities.ITree<TNodeTag, TLeaf> ToTree<TNodeTag, TLeaf>(System.Collections.Generic.IEnumerable<TLeaf> entries, System.Func<TLeaf, System.Collections.Generic.ICollection<TNodeTag>> categorySelector, TNodeTag rootTag)`

Contructs a tree by recursively grouping elements from a sequence. Essentially, performs a GroupBy operation, and then for each Grouping, performs another GroupBy, assuming there is a GroupBy key available for the sub-group.

**Parameters:**
- `entries` (System.Collections.Generic.IEnumerable<TLeaf>) — 
- `categorySelector` (System.Func<TLeaf, System.Collections.Generic.ICollection<TNodeTag>>) — 
- `rootTag` (TNodeTag) — 

**Returns:** `Dynamo.Utilities.ITree<TNodeTag, TLeaf>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/EnumerableExtensions.cs)

## GuidUtility (static-class)

Helper methods for working with System.Guid.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Guid.cs)

### Methods
#### `System.Guid Create(System.Guid namespaceId, string name)`

Creates a name-based UUID using the algorithm from RFC 4122 §4.3.

**Remarks:** See Generating a deterministic GUID.

**Parameters:**
- `namespaceId` (System.Guid) — The ID of the namespace.
- `name` (string) — The name (within that namespace).

**Returns:** `System.Guid` — A UUID derived from the namespace and name.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Guid.cs)

#### `System.Guid Create(System.Guid namespaceId, string name, int version)`

Creates a name-based UUID using the algorithm from RFC 4122 §4.3.

**Remarks:** See Generating a deterministic GUID.

**Parameters:**
- `namespaceId` (System.Guid) — The ID of the namespace.
- `name` (string) — The name (within that namespace).
- `version` (int) — The version number of the UUID to create; this value must be either 3 (for MD5 hashing) or 5 (for SHA-1 hashing).

**Returns:** `System.Guid` — A UUID derived from the namespace and name.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Guid.cs)

#### `System.Guid tryParseOrCreateGuid(string idstring)`

attempts to parse a string into a guid - if this fails, uses the create method to create a deterministic UUID.

**Parameters:**
- `idstring` (string) — 

**Returns:** `System.Guid` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Guid.cs)

## IEither<TLeft, TRight> (interface)

Object containing an instance of one of two potential types, labeled "Left" or "Right".

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Option.cs)

### Methods
#### `Dynamo.Utilities.IEither<TNewLeft, TRight> BindLeft<TNewLeft>(System.Func<TLeft, Dynamo.Utilities.IEither<TNewLeft, TRight>> selector)`

If the IEither(TLeft, TRight) instance contains a Left value, project the Left value into a new IEither(TNewLeft, TRight). If the instance contains a Right value, just return the instance.

**Parameters:**
- `selector` (System.Func<TLeft, Dynamo.Utilities.IEither<TNewLeft, TRight>>) — Function used to project a Left value.

**Returns:** `Dynamo.Utilities.IEither<TNewLeft, TRight>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Option.cs)

#### `Dynamo.Utilities.IEither<TLeft, TNewRight> BindRight<TNewRight>(System.Func<TRight, Dynamo.Utilities.IEither<TLeft, TNewRight>> selector)`

If the IEither(TLeft, TRight) instance contains a Right value, project the Right value into a new IEither(TLeft, TNewRight). If the instance contains a Left value, just return the instance.

**Parameters:**
- `selector` (System.Func<TRight, Dynamo.Utilities.IEither<TLeft, TNewRight>>) — Function used to project a Right value.

**Returns:** `Dynamo.Utilities.IEither<TLeft, TNewRight>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Option.cs)

#### `T Match<T>(System.Func<TLeft, T> leftCase, System.Func<TRight, T> rightCase)`

Produces a new value of type T using one of the given functions, based on whether the IEither(TLeft, TRight) contains a Left or Right value.

**Parameters:**
- `leftCase` (System.Func<TLeft, T>) — Function used to create a result from a Left value.
- `rightCase` (System.Func<TRight, T>) — Function used to create a result from a Right value.

**Returns:** `T` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Option.cs)

#### `Dynamo.Utilities.IEither<TNewLeft, TNewRight> Select<TNewLeft, TNewRight>(System.Func<TLeft, TNewLeft> leftSelector, System.Func<TRight, TNewRight> rightSelector)`

Create a new IEither(TNewLeft, TNewRight) instance by passing the value the a conversion function. Which function is selected depends on whether it is a Right or Left value.

**Parameters:**
- `leftSelector` (System.Func<TLeft, TNewLeft>) — Function used to convert a Left value.
- `rightSelector` (System.Func<TRight, TNewRight>) — Function used to convert a Right value.

**Returns:** `Dynamo.Utilities.IEither<TNewLeft, TNewRight>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Option.cs)

### Properties
- `IsLeft` (bool, get) — Specifies if this instance contains a Left value. If true, it does. If false, it contains a Right value.
- `LeftValue` (TLeft, get) — Attempts to get a Left value.
- `RightValue` (TRight, get) — Attempts to get a Right value.

## IOption<T> (interface)

Object that either contains a single value, or is empty.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Option.cs)

### Methods
#### `Dynamo.Utilities.IOption<U> Bind<U>(System.Func<T, Dynamo.Utilities.IOption<U>> selector)`

If the IOption(T) instance contains a value, project the value into a new IOption(U) instance using the given function. Otherwise, propagate the empty option.

**Parameters:**
- `selector` (System.Func<T, Dynamo.Utilities.IOption<U>>) — Function used to project a value.

**Returns:** `Dynamo.Utilities.IOption<U>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Option.cs)

#### `U Match<U>(System.Func<T, U> someCase, System.Func<U> noneCase)`

Produces a new value of type U using one of the given functions, based on whether the IOption(T) contains a value or not.

**Parameters:**
- `someCase` (System.Func<T, U>) — Function used to create a result from a value.
- `noneCase` (System.Func<U>) — Function used to create a result from no value.

**Returns:** `U` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Option.cs)

### Properties
- `Value` (T, get) — Attempts to get a contained value.

## IRecursiveGrouping<T> (interface)

A collection of recursive groupings that have a common key.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/EnumerableExtensions.cs)

## ITree<TNodeTag, TLeaf> (interface)

A tree that has a tag, leaves, and subtrees.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/EnumerableExtensions.cs)

### Properties
- `Leaves` (System.Collections.Generic.IEnumerable<TLeaf>, get) — ITree.Leaves
- `SubTrees` (System.Collections.Generic.IEnumerable<Dynamo.Utilities.ITree<TNodeTag, TLeaf>>, get) — ITree.SubTrees
- `Tag` (TNodeTag, get) — ITree.Tag

## LuceneCustomAnalyzer (class)

Due that the Lucene StandardAnalyzer removes special characters/words like "+", "*", "And" then we had to implement a Custom Analyzer that support that that kind of search terms

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Utilities/LuceneSearchUtility.cs)

### Constructors
- `void LuceneCustomAnalyzer(Lucene.Net.Util.LuceneVersion matchVersion, string language)` — LuceneCustomAnalyzer.LuceneCustomAnalyzer

### Methods
#### `Lucene.Net.Analysis.TokenStreamComponents CreateComponents(string fieldName, System.IO.TextReader reader)`

LuceneCustomAnalyzer.CreateComponents

**Parameters:**
- `fieldName` (string)
- `reader` (System.IO.TextReader)

**Returns:** `Lucene.Net.Analysis.TokenStreamComponents` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Utilities/LuceneSearchUtility.cs)

## ModifierKeys (enum)

Type ModifierKeys

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/ModifierKeys.cs)

### Values
- `Alt` = `1`
- `Control` = `2`
- `None` = `0`
- `Shift` = `4`
- `Windows` = `8`

## Option (static-class)

Utility method for working with Option instances.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Option.cs)

### Methods
#### `bool HasValue<T>(Dynamo.Utilities.IOption<T> option)`

Determines if the given Option contains a value or not.

**Parameters:**
- `option` (Dynamo.Utilities.IOption<T>) — Option instance to check for a value.

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Option.cs)

#### `Dynamo.Utilities.IOption<T> None<T>()`

Creates an empty Option.

**Returns:** `Dynamo.Utilities.IOption<T>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Option.cs)

#### `Dynamo.Utilities.IOption<U> Select<T, U>(Dynamo.Utilities.IOption<T> option, System.Func<T, U> selector)`

Creates a new IOption(U) from an IOption(T) by converting the potentially contained value, using a given conversion function.

**Parameters:**
- `option` (Dynamo.Utilities.IOption<T>) — An option to Select over.
- `selector` (System.Func<T, U>) — A function used to convert the potential value.

**Returns:** `Dynamo.Utilities.IOption<U>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Option.cs)

#### `Dynamo.Utilities.IOption<T> Some<T>(T value)`

Creates a new IOption(T) instance containing a value.

**Parameters:**
- `value` (T) — Value to be stored in the new IOption(T) instance.

**Returns:** `Dynamo.Utilities.IOption<T>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Option.cs)

## OrderedSet<T> (class)

A set of unique elements, with insertion order preserved.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/OrderedSet.cs)

### Constructors
- `void OrderedSet()` — OrderedSet.OrderedSet

### Methods
#### `bool Add(T item)`

Adds an element to the current set and returns a value to indicate if the element was successfully added.

**Parameters:**
- `item` (T) — The element to add to the set.

**Returns:** `bool` — true if the element is added to the set; false if the element is already in the set.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/OrderedSet.cs)

#### `void Clear()`

Removes all items from the System.Collections.Generic.ICollection`1.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/OrderedSet.cs)

#### `bool Contains(T item)`

Determines whether the System.Collections.Generic.ICollection`1 contains a specific value.

**Parameters:**
- `item` (T) — The object to locate in the System.Collections.Generic.ICollection`1.

**Returns:** `bool` — true if item is found in the System.Collections.Generic.ICollection`1; otherwise, false.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/OrderedSet.cs)

#### `void CopyTo(T[] array, int arrayIndex)`

Copies the elements of the System.Collections.Generic.ICollection`1 to an System.Array, starting at a particular System.Array index.

**Parameters:**
- `array` (T[]) — The one-dimensional System.Array that is the destination of the elements copied from System.Collections.Generic.ICollection`1. The System.Array must have zero-based indexing.
- `arrayIndex` (int) — The zero-based index in array at which copying begins.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/OrderedSet.cs)

#### `void ExceptWith(System.Collections.Generic.IEnumerable<T> other)`

Removes all elements in the specified collection from the current set.

**Parameters:**
- `other` (System.Collections.Generic.IEnumerable<T>) — The collection of items to remove from the set.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/OrderedSet.cs)

#### `System.Collections.Generic.IEnumerator<T> GetEnumerator()`

Returns an enumerator that iterates through the collection.

**Returns:** `System.Collections.Generic.IEnumerator<T>` — A System.Collections.Generic.IEnumerator`1 that can be used to iterate through the collection.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/OrderedSet.cs)

#### `void IntersectWith(System.Collections.Generic.IEnumerable<T> other)`

Modifies the current set so that it contains only elements that are also in a specified collection.

**Parameters:**
- `other` (System.Collections.Generic.IEnumerable<T>) — The collection to compare to the current set.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/OrderedSet.cs)

#### `bool IsProperSubsetOf(System.Collections.Generic.IEnumerable<T> other)`

Determines whether the current set is a proper (strict) subset of a specified collection.

**Parameters:**
- `other` (System.Collections.Generic.IEnumerable<T>) — The collection to compare to the current set.

**Returns:** `bool` — true if the current set is a proper subset of other; otherwise, false.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/OrderedSet.cs)

#### `bool IsProperSupersetOf(System.Collections.Generic.IEnumerable<T> other)`

Determines whether the current set is a proper (strict) superset of a specified collection.

**Parameters:**
- `other` (System.Collections.Generic.IEnumerable<T>) — The collection to compare to the current set.

**Returns:** `bool` — true if the current set is a proper superset of other; otherwise, false.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/OrderedSet.cs)

#### `bool IsSubsetOf(System.Collections.Generic.IEnumerable<T> other)`

Determines whether a set is a subset of a specified collection.

**Parameters:**
- `other` (System.Collections.Generic.IEnumerable<T>) — The collection to compare to the current set.

**Returns:** `bool` — true if the current set is a subset of other; otherwise, false.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/OrderedSet.cs)

#### `bool IsSupersetOf(System.Collections.Generic.IEnumerable<T> other)`

Determines whether the current set is a superset of a specified collection.

**Parameters:**
- `other` (System.Collections.Generic.IEnumerable<T>) — The collection to compare to the current set.

**Returns:** `bool` — true if the current set is a superset of other; otherwise, false.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/OrderedSet.cs)

#### `bool Overlaps(System.Collections.Generic.IEnumerable<T> other)`

Determines whether the current set overlaps with the specified collection.

**Parameters:**
- `other` (System.Collections.Generic.IEnumerable<T>) — The collection to compare to the current set.

**Returns:** `bool` — true if the current set and other share at least one common element; otherwise, false.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/OrderedSet.cs)

#### `bool Remove(T item)`

Removes the first occurrence of a specific object from the System.Collections.Generic.ICollection`1.

**Parameters:**
- `item` (T) — The object to remove from the System.Collections.Generic.ICollection`1.

**Returns:** `bool` — true if item was successfully removed from the System.Collections.Generic.ICollection`1; otherwise, false. This method also returns false if item is not found in the original System.Collections.Generic.ICollection`1.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/OrderedSet.cs)

#### `bool SetEquals(System.Collections.Generic.IEnumerable<T> other)`

Determines whether the current set and the specified collection contain the same elements.

**Parameters:**
- `other` (System.Collections.Generic.IEnumerable<T>) — The collection to compare to the current set.

**Returns:** `bool` — true if the current set is equal to other; otherwise, false.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/OrderedSet.cs)

#### `void SymmetricExceptWith(System.Collections.Generic.IEnumerable<T> other)`

Modifies the current set so that it contains only elements that are present either in the current set or in the specified collection, but not both.

**Parameters:**
- `other` (System.Collections.Generic.IEnumerable<T>) — The collection to compare to the current set.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/OrderedSet.cs)

#### `void UnionWith(System.Collections.Generic.IEnumerable<T> other)`

Modifies the current set so that it contains all elements that are present in both the current set and in the specified collection.

**Parameters:**
- `other` (System.Collections.Generic.IEnumerable<T>) — The collection to compare to the current set.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/OrderedSet.cs)

### Properties
- `Count` (int, get) — Returns the number of elements contained in the System.Collections.Generic.ICollection`1.
- `IsReadOnly` (bool, get) — Returns a value indicating whether the System.Collections.Generic.ICollection`1 is read-only.

## Point2D (struct)

Type Point2D

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Point2D.cs)

### Constructors
- `void Point2D(double x, double y)` — Point2D.Point2D

### Methods
#### `bool Equals(Dynamo.Utilities.Point2D value)`

Point2D.Equals

**Parameters:**
- `value` (Dynamo.Utilities.Point2D)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Point2D.cs)

#### `bool Equals(object o)`

Point2D.Equals

**Parameters:**
- `o` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Point2D.cs)

#### `int GetHashCode()`

Point2D.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Point2D.cs)

#### `string ToString()`

Point2D.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Point2D.cs)

#### `string ToString(System.IFormatProvider provider)`

Point2D.ToString

**Parameters:**
- `provider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Point2D.cs)

### Properties
- `X` (double, get/set) — Point2D.X
- `Y` (double, get/set) — Point2D.Y

## Rect2D (struct)

Type Rect2D

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Rect2D.cs)

### Constructors
- `void Rect2D(Dynamo.Utilities.Point2D point1, Dynamo.Utilities.Point2D point2)` — Rect2D.Rect2D
- `void Rect2D(double x, double y, double width, double height)` — Rect2D.Rect2D

### Methods
#### `bool Contains(Dynamo.Utilities.Point2D point)`

Rect2D.Contains

**Parameters:**
- `point` (Dynamo.Utilities.Point2D)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Rect2D.cs)

#### `bool Contains(Dynamo.Utilities.Rect2D rect)`

Rect2D.Contains

**Parameters:**
- `rect` (Dynamo.Utilities.Rect2D)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Rect2D.cs)

#### `bool Contains(double x, double y)`

Rect2D.Contains

**Parameters:**
- `x` (double)
- `y` (double)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Rect2D.cs)

#### `bool Equals(Dynamo.Utilities.Rect2D value)`

Rect2D.Equals

**Parameters:**
- `value` (Dynamo.Utilities.Rect2D)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Rect2D.cs)

#### `bool Equals(Dynamo.Utilities.Rect2D rect1, Dynamo.Utilities.Rect2D rect2)`

Rect2D.Equals

**Parameters:**
- `rect1` (Dynamo.Utilities.Rect2D)
- `rect2` (Dynamo.Utilities.Rect2D)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Rect2D.cs)

#### `bool Equals(object o)`

Rect2D.Equals

**Parameters:**
- `o` (object)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Rect2D.cs)

#### `int GetHashCode()`

Rect2D.GetHashCode

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Rect2D.cs)

#### `void Intersect(Dynamo.Utilities.Rect2D rect)`

Rect2D.Intersect

**Parameters:**
- `rect` (Dynamo.Utilities.Rect2D)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Rect2D.cs)

#### `Dynamo.Utilities.Rect2D Intersect(Dynamo.Utilities.Rect2D rect1, Dynamo.Utilities.Rect2D rect2)`

Rect2D.Intersect

**Parameters:**
- `rect1` (Dynamo.Utilities.Rect2D)
- `rect2` (Dynamo.Utilities.Rect2D)

**Returns:** `Dynamo.Utilities.Rect2D` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Rect2D.cs)

#### `bool IntersectsWith(Dynamo.Utilities.Rect2D rect)`

Rect2D.IntersectsWith

**Parameters:**
- `rect` (Dynamo.Utilities.Rect2D)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Rect2D.cs)

#### `void Scale(double scaleX, double scaleY)`

Rect2D.Scale

**Parameters:**
- `scaleX` (double)
- `scaleY` (double)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Rect2D.cs)

#### `string ToString()`

Rect2D.ToString

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Rect2D.cs)

#### `string ToString(System.IFormatProvider provider)`

Rect2D.ToString

**Parameters:**
- `provider` (System.IFormatProvider)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Rect2D.cs)

### Properties
- `Bottom` (double, get) — Rect2D.Bottom
- `BottomLeft` (Dynamo.Utilities.Point2D, get) — Rect2D.BottomLeft
- `BottomRight` (Dynamo.Utilities.Point2D, get) — Rect2D.BottomRight
- `Empty` (Dynamo.Utilities.Rect2D, get) — Rect2D.Empty
- `Height` (double, get/set) — Rect2D.Height
- `IsEmpty` (bool, get) — Rect2D.IsEmpty
- `Left` (double, get) — Rect2D.Left
- `Location` (Dynamo.Utilities.Point2D, get/set) — Rect2D.Location
- `Right` (double, get) — Rect2D.Right
- `Top` (double, get) — Rect2D.Top
- `TopLeft` (Dynamo.Utilities.Point2D, get) — Rect2D.TopLeft
- `TopRight` (Dynamo.Utilities.Point2D, get) — Rect2D.TopRight
- `Width` (double, get/set) — Rect2D.Width
- `X` (double, get/set) — Rect2D.X
- `Y` (double, get/set) — Rect2D.Y

## Thickness (struct)

Type Thickness

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/Thickness.cs)

### Constructors
- `void Thickness(double thickness)` — Thickness.Thickness
- `void Thickness(double left, double top, double right, double bottom)` — Thickness.Thickness

### Properties
- `Bottom` (double, get/set) — Thickness.Bottom
- `Left` (double, get/set) — Thickness.Left
- `Right` (double, get/set) — Thickness.Right
- `Top` (double, get/set) — Thickness.Top

## TypeExtensions (static-class)

Type TypeExtensions

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/TypeExtensions.cs)

### Methods
#### `System.Func<T> GetDefaultConstructor<T>()`

Creates a function that constructs an instance of an object of the given type.

**Returns:** `System.Func<T>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/TypeExtensions.cs)

#### `System.Func<T> GetDefaultConstructor<T>(System.Type type)`

Creates a function that constructs an instance of an object of the given type.

**Parameters:**
- `type` (System.Type) — The type to create a constructor for.

**Returns:** `System.Func<T>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/TypeExtensions.cs)

#### `object GetInstance(System.Type type)`

Returns an instance of the type on which the method is invoked.

**Parameters:**
- `type` (System.Type) — The type on which the method was invoked.

**Returns:** `object` — An instance of the type.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/TypeExtensions.cs)

#### `object GetInstance<TArg>(System.Type type, TArg argument)`

Returns an instance of the type on which the method is invoked.

**Parameters:**
- `type` (System.Type) — The type on which the method was invoked.
- `argument` (TArg)

**Returns:** `object` — An instance of the type.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/TypeExtensions.cs)

#### `object GetInstance<TArg1, TArg2>(System.Type type, TArg1 argument1, TArg2 argument2)`

Returns an instance of the type on which the method is invoked.

**Parameters:**
- `type` (System.Type) — The type on which the method was invoked.
- `argument1` (TArg1)
- `argument2` (TArg2)

**Returns:** `object` — An instance of the type.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/TypeExtensions.cs)

#### `object GetInstance<TArg1, TArg2, TArg3>(System.Type type, TArg1 argument1, TArg2 argument2, TArg3 argument3)`

Returns an instance of the type on which the method is invoked.

**Parameters:**
- `type` (System.Type) — The type on which the method was invoked.
- `argument1` (TArg1)
- `argument2` (TArg2)
- `argument3` (TArg3)

**Returns:** `object` — An instance of the type.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/TypeExtensions.cs)

#### `bool ImplementsGeneric(System.Type generic, System.Type toCheck)`

TypeExtensions.ImplementsGeneric

**Parameters:**
- `generic` (System.Type)
- `toCheck` (System.Type)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/TypeExtensions.cs)

## VersionUtilities (static-class)

Type VersionUtilities

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/VersionUtilities.cs)

### Methods
#### `System.Version Parse(string version)`

A utility method accepting any string input and trying to return a valid Version from it If the initial input is not valid, returns null

**Parameters:**
- `version` (string) — The string representation of the version to be parsed.

**Returns:** `System.Version` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/VersionUtilities.cs)

#### `System.Version PartialParse(string versionString, int numberOfFields)`

Parse the first n fields of a version string. Delegates to Version.Parse.

**Parameters:**
- `versionString` (string)
- `numberOfFields` (int)

**Returns:** `System.Version` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/VersionUtilities.cs)

#### `System.Version WildCardParse(string version)`

Parse the first n fields of a version string. Delegates to Version.Parse.

**Parameters:**
- `version` (string)

**Returns:** `System.Version` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/VersionUtilities.cs)

## XmlElementHelper (class)

Type XmlElementHelper

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/XmlElementHelper.cs)

### Constructors
- `void XmlElementHelper(System.Xml.XmlElement xmlElement)` — XmlElementHelper.XmlElementHelper

### Methods
#### `bool HasAttribute(string attribName)`

XmlElementHelper.HasAttribute

**Parameters:**
- `attribName` (string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/XmlElementHelper.cs)

#### `bool ReadBoolean(string attribName)`

XmlElementHelper.ReadBoolean

**Parameters:**
- `attribName` (string)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/XmlElementHelper.cs)

#### `bool ReadBoolean(string attribName, bool defaultValue)`

XmlElementHelper.ReadBoolean

**Parameters:**
- `attribName` (string)
- `defaultValue` (bool)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/XmlElementHelper.cs)

#### `double ReadDouble(string attribName)`

XmlElementHelper.ReadDouble

**Parameters:**
- `attribName` (string)

**Returns:** `double` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/XmlElementHelper.cs)

#### `double ReadDouble(string attribName, double defaultValue)`

XmlElementHelper.ReadDouble

**Parameters:**
- `attribName` (string)
- `defaultValue` (double)

**Returns:** `double` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/XmlElementHelper.cs)

#### `TEnum ReadEnum<TEnum>(string attribName, TEnum defaultValue)`

XmlElementHelper.ReadEnum

**Parameters:**
- `attribName` (string)
- `defaultValue` (TEnum)

**Returns:** `TEnum` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/XmlElementHelper.cs)

#### `System.Guid ReadGuid(string attribName)`

XmlElementHelper.ReadGuid

**Parameters:**
- `attribName` (string)

**Returns:** `System.Guid` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/XmlElementHelper.cs)

#### `System.Guid ReadGuid(string attribName, System.Guid defaultValue)`

XmlElementHelper.ReadGuid

**Parameters:**
- `attribName` (string)
- `defaultValue` (System.Guid)

**Returns:** `System.Guid` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/XmlElementHelper.cs)

#### `int ReadInteger(string attribName)`

XmlElementHelper.ReadInteger

**Parameters:**
- `attribName` (string)

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/XmlElementHelper.cs)

#### `int ReadInteger(string attribName, int defaultValue)`

XmlElementHelper.ReadInteger

**Parameters:**
- `attribName` (string)
- `defaultValue` (int)

**Returns:** `int` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/XmlElementHelper.cs)

#### `string ReadString(string attribName)`

XmlElementHelper.ReadString

**Parameters:**
- `attribName` (string)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/XmlElementHelper.cs)

#### `string ReadString(string attribName, string defaultValue)`

XmlElementHelper.ReadString

**Parameters:**
- `attribName` (string)
- `defaultValue` (string)

**Returns:** `string` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/XmlElementHelper.cs)

#### `void SetAttribute(string name, System.Guid value)`

XmlElementHelper.SetAttribute

**Parameters:**
- `name` (string)
- `value` (System.Guid)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/XmlElementHelper.cs)

#### `void SetAttribute(string name, System.Nullable<bool> value)`

XmlElementHelper.SetAttribute

**Parameters:**
- `name` (string)
- `value` (System.Nullable<bool>)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/XmlElementHelper.cs)

#### `void SetAttribute(string name, System.Type value)`

XmlElementHelper.SetAttribute

**Parameters:**
- `name` (string)
- `value` (System.Type)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/XmlElementHelper.cs)

#### `void SetAttribute(string name, bool value)`

XmlElementHelper.SetAttribute

**Parameters:**
- `name` (string)
- `value` (bool)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/XmlElementHelper.cs)

#### `void SetAttribute(string name, double value)`

XmlElementHelper.SetAttribute

**Parameters:**
- `name` (string)
- `value` (double)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/XmlElementHelper.cs)

#### `void SetAttribute(string name, int value)`

XmlElementHelper.SetAttribute

**Parameters:**
- `name` (string)
- `value` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/XmlElementHelper.cs)

#### `void SetAttribute(string name, string value)`

XmlElementHelper.SetAttribute

**Parameters:**
- `name` (string)
- `value` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoUtilities/XmlElementHelper.cs)

