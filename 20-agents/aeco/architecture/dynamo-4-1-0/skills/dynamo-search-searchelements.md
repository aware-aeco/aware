---
name: dynamo-dynamo-search-searchelements
description: This skill encodes the dynamo 4.1.0 surface of the Dynamo.Search.SearchElements namespace — 11 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: CodeBlockNodeSearchElement, CustomNodeSearchElement, NodeModelSearchElement, NodeModelSearchElementBase, NodeSearchElement, AutoCompletionNodeMachineLearningInfo, DragDropNodeSearchElementInfo, SearchElementBase, and 3 more types.
---

# Dynamo.Search.SearchElements

Auto-generated from vendor docs for dynamo 4.1.0. 11 types in this namespace.

## AutoCompletionNodeMachineLearningInfo (class)

This class will support the Machine Learning info related to a node element of Auto-completion feature.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/SearchElements/NodeSearchElement.cs)

### Constructors
- `void AutoCompletionNodeMachineLearningInfo()` — default constructor
- `void AutoCompletionNodeMachineLearningInfo(bool displayIcon, bool isByRecommendation, double confidenceScore)` — Creates ML info object using DisplayIcon, IsByRecommendation and ConfidenceScore params

### Properties
- `ConfidenceScore` (double, get/set) — Confidence score in percentage
- `DisplayIcon` (bool, get/set) — Indicates if the icon is displayed to show the condifent score or by-usage
- `IsByRecommendation` (bool, get/set) — Indicates if the Node is part of the ML result per recommendation
- `IsByUse` (bool, get/set) — Indicates if the Node is part of ML result per use

## CodeBlockNodeSearchElement (class)

Search element for Code Block nodes.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/SearchElements/CodeBlockNodeSearchElement.cs)

### Constructors
- `void CodeBlockNodeSearchElement(Dynamo.Graph.Nodes.TypeLoadData data, Dynamo.Engine.LibraryServices manager)` — CodeBlockNodeSearchElement.CodeBlockNodeSearchElement

### Methods
#### `Dynamo.Graph.Nodes.NodeModel ConstructNewNodeModel()`

CodeBlockNodeSearchElement.ConstructNewNodeModel

**Returns:** `Dynamo.Graph.Nodes.NodeModel` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/SearchElements/CodeBlockNodeSearchElement.cs)

## CustomNodeSearchElement (class)

Search element for custom nodes.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/SearchElements/CustomNodeSearchElement.cs)

### Constructors
- `void CustomNodeSearchElement(Dynamo.Graph.Nodes.CustomNodes.ICustomNodeSource customNodeManager, Dynamo.CustomNodeInfo info)` — Initializes a new instance of the Dynamo.Search.SearchElements.CustomNodeSearchElement class.

### Methods
#### `Dynamo.Graph.Nodes.NodeModel ConstructNewNodeModel()`

CustomNodeSearchElement.ConstructNewNodeModel

**Returns:** `Dynamo.Graph.Nodes.NodeModel` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/SearchElements/CustomNodeSearchElement.cs)

#### `System.Collections.Generic.IEnumerable<System.Tuple<string, string>> GenerateInputParameters()`

CustomNodeSearchElement.GenerateInputParameters

**Returns:** `System.Collections.Generic.IEnumerable<System.Tuple<string, string>>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/SearchElements/CustomNodeSearchElement.cs)

#### `System.Collections.Generic.IEnumerable<string> GenerateOutputParameters()`

CustomNodeSearchElement.GenerateOutputParameters

**Returns:** `System.Collections.Generic.IEnumerable<string>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/SearchElements/CustomNodeSearchElement.cs)

#### `void SyncWithCustomNodeInfo(Dynamo.CustomNodeInfo info)`

Updates the properties of this search element.

**Parameters:**
- `info` (Dynamo.CustomNodeInfo) — Actual data of custom node

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/SearchElements/CustomNodeSearchElement.cs)

### Properties
- `CreationName` (string, get) — Returns a name using which this node can be created
- `ID` (System.Guid, get) — Returns identifier of the custom node
- `Path` (string, get) — Path to this custom node in disk, used in the Edit context menu.

## DragDropNodeSearchElementInfo (class)

This class returns Dynamo.Search.SearchElements.NodeSearchElement which is used for creating a node, when the element is drag and dropped.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/SearchElements/NodeSearchElement.cs)

### Constructors
- `void DragDropNodeSearchElementInfo(Dynamo.Search.SearchElements.NodeSearchElement searchElement)` — Initializes a new instance of the Dynamo.Search.SearchElements.DragDropNodeSearchElementInfo class.

### Properties
- `SearchElement` (Dynamo.Search.SearchElements.NodeSearchElement, get) — Returns Dynamo.Search.SearchElements.NodeSearchElement to drag and drop.

## ElementTypes (enum)

Enumeration of external elements' types

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/SearchElements/SearchElementGroup.cs)

### Values
- `BuiltIn` = `65536` — Type for things that Dynamo ships with (operator, geometries...)
- `CustomNode` = `2` — Type for all the DYFs (both loose DYF files and packaged)
- `None` = `0` — Represents no type
- `Packaged` = `131072` — Type for a packaged element (either zero-touch DLLs or DYFs)
- `ZeroTouch` = `1` — Type for all the DLLs (can be built-in, third-party, and packaged)

## NodeModelSearchElement (class)

Search element for basic NodeModels.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/SearchElements/NodeModelSearchElement.cs)

### Methods
#### `Dynamo.Graph.Nodes.NodeModel ConstructNewNodeModel()`

NodeModelSearchElement.ConstructNewNodeModel

**Returns:** `Dynamo.Graph.Nodes.NodeModel` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/SearchElements/NodeModelSearchElement.cs)

### Properties
- `CreationName` (string, get) — NodeModelSearchElement.CreationName

## NodeModelSearchElementBase (class)

Base class for node search elements that can be initialized from TypeLoadData.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/SearchElements/NodeModelSearchElementBase.cs)

### Constructors
- `void NodeModelSearchElementBase(Dynamo.Graph.Nodes.TypeLoadData typeLoadData)` — Initializes a new instance of the Dynamo.Search.SearchElements.NodeModelSearchElementBase class.

## NodeSearchElement (class)

Base class for all Dynamo Node search elements.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/SearchElements/NodeSearchElement.cs)

### Constructors
- `void NodeSearchElement()` — NodeSearchElement.NodeSearchElement

### Methods
#### `object Clone()`

Clones the NodeSearchElement

**Returns:** `object` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/SearchElements/NodeSearchElement.cs)

#### `Dynamo.Graph.Nodes.NodeModel ConstructNewNodeModel()`

Creates a new NodeModel to be inserted into the current Dynamo workspace.

**Returns:** `Dynamo.Graph.Nodes.NodeModel` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/SearchElements/NodeSearchElement.cs)

#### `System.Collections.Generic.IEnumerable<System.Tuple<string, string>> GenerateInputParameters()`

NodeSearchElement.GenerateInputParameters

**Returns:** `System.Collections.Generic.IEnumerable<System.Tuple<string, string>>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/SearchElements/NodeSearchElement.cs)

#### `System.Collections.Generic.IEnumerable<string> GenerateOutputParameters()`

NodeSearchElement.GenerateOutputParameters

**Returns:** `System.Collections.Generic.IEnumerable<string>` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/SearchElements/NodeSearchElement.cs)

#### `void OnItemProduced(Dynamo.Graph.Nodes.NodeModel obj)`

NodeSearchElement.OnItemProduced

**Parameters:**
- `obj` (Dynamo.Graph.Nodes.NodeModel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/SearchElements/NodeSearchElement.cs)

### Properties
- `Assembly` (string, get) — Assembly to which Node belongs to
- `Categories` (System.Collections.Generic.ICollection<string>, get) — List of nested categories this search element is contained in.
- `CreationName` (string, get) — The name that is used during node creation
- `Description` (string, get/set) — Description of the node.
- `ElementType` (Dynamo.Search.SearchElements.ElementTypes, get) — Indicates whether it is custom node or zero-touch element. And whether this element comes from package or not.
- `FullCategoryName` (string, get/set) — The category name of this node.
- `FullName` (string, get) — The full name of entry which consists of category name and entry name.
- `Group` (Dynamo.Search.SearchElements.SearchElementGroup, get/set) — Group to which Node belongs to
- `IconName` (string, get) — Returns the name of the node icon.
- `InputParameters` (System.Collections.Generic.IEnumerable<System.Tuple<string, string>>, get) — NodeSearchElement.InputParameters
- `IsVisibleInSearch` (bool, get/set) — Specifies whether or not this entry should appear in search.
- `Name` (string, get) — The name of this entry as it appears in the library.
- `OutputParameters` (System.Collections.Generic.IEnumerable<string>, get) — NodeSearchElement.OutputParameters
- `Parameters` (string, get) — The parameters of this entry, used for overloaded nodes.
- `SearchKeywords` (System.Collections.Generic.ICollection<string>, get) — Collection of keywords which can be used to search for this element.

### Events
#### `ItemProduced` (`System.Action<Dynamo.Graph.Nodes.NodeModel>`)

**Signature:** `public event System.Action<Dynamo.Graph.Nodes.NodeModel> ItemProduced`

Event fired when this search element produces a new NodeModel. This typically happens when it is selected in the library by the user.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/SearchElements/NodeSearchElement.cs)

## SearchElementBase (class)

A base class for elements found in search

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/SearchElements/SearchElementBase.cs)

### Constructors
- `void SearchElementBase()` — SearchElementBase.SearchElementBase

### Methods
#### `void OnExecuted()`

SearchElementBase.OnExecuted

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/SearchElements/SearchElementBase.cs)

### Properties
- `CreationName` (string, get) — The name that is used during node creation
- `Description` (string, get) — Description property
- `Keywords` (string, get/set) — Keywords property
- `Searchable` (bool, get) — Searchable property
- `Type` (string, get) — Type property
- `Weight` (double, get/set) — Weight property

## SearchElementGroup (enum)

Returns one of the possible values: None, Create, Action, Query. E.g. Point.ByCoordinates is member of create group.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/SearchElements/SearchElementGroup.cs)

### Values
- `Action` = `2`
- `Create` = `1`
- `None` = `0`
- `Query` = `3`

## ZeroTouchSearchElement (class)

Search element for a Zero Touch node (DSFunction / DSVarArgFunction)

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/SearchElements/ZeroTouchSearchElement.cs)

### Constructors
- `void ZeroTouchSearchElement(Dynamo.Engine.FunctionDescriptor functionDescriptor)` — Initializes a new instance of the Dynamo.Search.SearchElements.ZeroTouchSearchElement class with the DesignScript function description

### Methods
#### `Dynamo.Graph.Nodes.NodeModel ConstructNewNodeModel()`

ZeroTouchSearchElement.ConstructNewNodeModel

**Returns:** `Dynamo.Graph.Nodes.NodeModel` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Search/SearchElements/ZeroTouchSearchElement.cs)

### Properties
- `CreationName` (string, get) — The name that is used during node creation
- `FullName` (string, get) — The full name of entry which consists of assembly name and qualified name for function descriptor.

