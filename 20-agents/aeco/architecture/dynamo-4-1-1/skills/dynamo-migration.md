---
name: dynamo-dynamo-migration
description: This skill encodes the dynamo 4.1.1 surface of the Dynamo.Migration namespace — 6 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: MigrationManager, PortId, NodeMigrationData, NodeMigrationAttribute, WorkspaceMigrationAttribute, WorkspaceMigrations.
---

# Dynamo.Migration

Auto-generated from vendor docs for dynamo 4.1.1. 6 types in this namespace.

## MigrationManager (class)

This class contains methods and properties used for migration of Dynamo workspaces.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

### Constructors
- `void MigrationManager(Dynamo.Migration.MigrationManager.FutureFileCallback displayFutureFileMessage, Dynamo.Migration.MigrationManager.ObsoleteFileCallback displayObsoleteFileMessage)` — Initializes a new instance of Dynamo.Migration.MigrationManager class

### Methods
#### `void AddMigrationType(Dynamo.Graph.Nodes.TypeLoadData t)`

Adds a new type containing Migration methods into this manager.

**Parameters:**
- `t` (Dynamo.Graph.Nodes.TypeLoadData) — Type data to add

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

#### `System.Xml.XmlElement CloneAndChangeName(System.Xml.XmlElement element, string type, string name, bool cloneInnerXml)`

Call this method to create a clone of the original XmlElement and change its type at one go. This method preserves all the attributes while updating only the type name.

**Parameters:**
- `element` (System.Xml.XmlElement) — The XmlElement to be cloned and the type name updated.
- `type` (string) — The fully qualified name of the new type.
- `name` (string) — The new name, by which this node is known.
- `cloneInnerXml` (bool) — Parameter indicating whether the inner xml of the original node should be cloned.

**Returns:** `System.Xml.XmlElement` — Returns the cloned and updated XmlElement.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

#### `System.Xml.XmlElement CreateCodeBlockNodeFrom(System.Xml.XmlElement srcElement)`

Call this method to create an empty Code Block node, with all attributes carried over from an existing src XmlElement.

**Parameters:**
- `srcElement` (System.Xml.XmlElement) — The source element from which the Code Block node XmlElement is constructed. All attributes of the source XmlElement will be copied over, and Code Block node specific attributes will be added.

**Returns:** `System.Xml.XmlElement` — Returns an XmlElement that represents the resulting Code Block node.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

#### `System.Xml.XmlElement CreateCodeBlockNodeModelNode(System.Xml.XmlDocument document, System.Xml.XmlElement oldNode, int nodeIndex, string codeText)`

This method creates an empty CodeBlockNodeModel node that contains basic code block node information.

**Parameters:**
- `document` (System.Xml.XmlDocument) — The XmlDocument to create the node in.
- `oldNode` (System.Xml.XmlElement) — Base node to create a new one
- `nodeIndex` (int) — Index of the node
- `codeText` (string) — Code block content

**Returns:** `System.Xml.XmlElement` — Returns the XmlElement that represents a CodeBlockNodeModel node with its basic function information with default attributes.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

#### `System.Xml.XmlElement CreateCustomNodeFrom(System.Xml.XmlDocument document, System.Xml.XmlElement srcElement, string id, string name, string description, System.Collections.Generic.List<string> inputs, System.Collections.Generic.List<string> outputs)`

Create a custom node as a replacement for an existing node.

**Parameters:**
- `document` (System.Xml.XmlDocument) — The XmlDocument to create the node in.
- `srcElement` (System.Xml.XmlElement) — The source XmlElement object.
- `id` (string) — The custom node id.
- `name` (string) — The custom node name.
- `description` (string) — The custom node's description.
- `inputs` (System.Collections.Generic.List<string>) — A list of input names.
- `outputs` (System.Collections.Generic.List<string>) — A list of output names.

**Returns:** `System.Xml.XmlElement` — Returns the XmlElement that represents a custom node with its basic function information with default attributes.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

#### `System.Xml.XmlElement CreateDummyNode(System.Xml.XmlElement element, int inportCount, int outportCount)`

Call this method to create a dummy node, should a node failed to be migrated. This results in a dummy node with a description of what the original node type was, and also retain the number of input and output ports.

**Parameters:**
- `element` (System.Xml.XmlElement) — XmlElement representing the original node which has failed migration.
- `inportCount` (int) — The number of input ports required on the new dummy node. This number must be a positive number greater or equal to zero.
- `outportCount` (int) — The number of output ports required on the new dummy node. This number must be a positive number greater or equal to zero.

**Returns:** `System.Xml.XmlElement` — Returns a new XmlElement representing the dummy node.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

#### `System.Xml.XmlElement CreateFunctionNode(System.Xml.XmlDocument document, System.Xml.XmlElement oldNode, int nodeIndex, string assembly, string name, string signature)`

Call this method to create an empty DSFunction node that contains basic function node information.

**Parameters:**
- `document` (System.Xml.XmlDocument) — The XmlDocument to create the node in.
- `oldNode` (System.Xml.XmlElement) — Base node to create a new one
- `nodeIndex` (int) — Index of the node
- `assembly` (string) — Name of the assembly that implements this function.
- `name` (string) — The name to display on the node.
- `signature` (string) — The signature of the function.

**Returns:** `System.Xml.XmlElement` — Returns the XmlElement that represents a DSFunction node with its basic function information with default attributes.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

#### `System.Xml.XmlElement CreateFunctionNodeFrom(System.Xml.XmlElement srcElement)`

Call this method to create a duplicated XmlElement with all the attributes found from the source XmlElement.

**Parameters:**
- `srcElement` (System.Xml.XmlElement) — The source XmlElement to duplicate.

**Returns:** `System.Xml.XmlElement` — Returns the duplicated XmlElement with all attributes found in the source XmlElement. The resulting XmlElement will also have a mandatory "type" attribute with value "Dynamo.Graph.Nodes.ZeroTouch.DSFunction".

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

#### `System.Xml.XmlElement CreateFunctionNodeFrom(System.Xml.XmlElement srcElement, string[] attribNames)`

Call this method to create a XmlElement with a set of attributes carried over from the source XmlElement. The new XmlElement will have a name of "Dynamo.Graph.Nodes.ZeroTouch.DSFunction".

**Parameters:**
- `srcElement` (System.Xml.XmlElement) — The source XmlElement object.
- `attribNames` (string[]) — The list of attribute names whose values are to be carried over to the resulting XmlElement. This list is mandatory and it cannot be empty. If a specified attribute cannot be found in srcElement, an empty attribute with the same name will be created in the resulting XmlElement.

**Returns:** `System.Xml.XmlElement` — Returns the resulting XmlElement with specified attributes duplicated from srcElement. The resulting XmlElement will also have a mandatory "type" attribute with value "Dynamo.Graph.Nodes.ZeroTouch.DSFunction".

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

#### `System.Xml.XmlElement CreateMissingNode(System.Xml.XmlElement element, int inportCount, int outportCount)`

Call this method to create a dummy node, should a node failed to be migrated. This results in a dummy node with a description of what the original node type was, and also retain the number of input and output ports.

**Parameters:**
- `element` (System.Xml.XmlElement) — XmlElement representing the original node which has failed migration.
- `inportCount` (int) — The number of input ports required on the new dummy node. This number must be a positive number greater or equal to zero.
- `outportCount` (int) — The number of output ports required on the new dummy node. This number must be a positive number greater or equal to zero.

**Returns:** `System.Xml.XmlElement` — Returns a new XmlElement representing the dummy node.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

#### `System.Xml.XmlElement CreateNode(System.Xml.XmlDocument document, System.Xml.XmlElement oldNode, int nodeIndex, string type, string name)`

This method creates an empty NodeModel node that contains basic code block node information.

**Parameters:**
- `document` (System.Xml.XmlDocument) — The XmlDocument to create the node in.
- `oldNode` (System.Xml.XmlElement) — Base node to create a new one
- `nodeIndex` (int) — Index of the node
- `type` (string) — Node type name
- `name` (string) — Name to display on the node

**Returns:** `System.Xml.XmlElement` — Returns the XmlElement that represents a NodeModel node with its basic function information with default attributes.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

#### `System.Xml.XmlElement CreateVarArgFunctionNode(System.Xml.XmlDocument document, System.Xml.XmlElement oldNode, int nodeIndex, string assembly, string name, string signature, string inputcount)`

Call this method to create an empty DSVarArgFunction node that contains basic function node information.

**Parameters:**
- `document` (System.Xml.XmlDocument) — The XmlDocument to create the node in.
- `oldNode` (System.Xml.XmlElement) — Base node to create a new one
- `nodeIndex` (int) — Index of the node
- `assembly` (string) — Name of the assembly that implements this function.
- `name` (string) — The name to display on the node.
- `signature` (string) — The signature of the function.
- `inputcount` (string) — 

**Returns:** `System.Xml.XmlElement` — Returns the XmlElement that represents a DSVarArgFunction node with its basic function information with default attributes.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

#### `System.Xml.XmlElement CreateVarArgFunctionNodeFrom(System.Xml.XmlElement srcElement)`

This method creates a duplicated XmlElement representing DSVarArgFunction with all the attributes found from the source XmlElement.

**Parameters:**
- `srcElement` (System.Xml.XmlElement) — The source XmlElement to duplicate.

**Returns:** `System.Xml.XmlElement` — Returns the duplicated XmlElement representing DSVarArgFunction with all attributes found in the source XmlElement. The resulting XmlElement will also have a mandatory "type" attribute with value "Dynamo.Graph.Nodes.ZeroTouch.DSVarArgFunction".

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

#### `string GetGuidFromXmlElement(System.Xml.XmlElement element)`

Returns node guid specified in given System.Xml.XmlElement

**Parameters:**
- `element` (System.Xml.XmlElement) — System.Xml.XmlElement containing node guid

**Returns:** `string` — Guid of the node

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

#### `Dynamo.Migration.NodeMigrationData MigrateXmlNode(System.Version currentVersion, System.Xml.XmlNode elNode, System.Type type, System.Version workspaceVersion)`

Attempts to migrate a node to the current version of Dynamo by given xml data

**Parameters:**
- `currentVersion` (System.Version) — Current Dynamo version
- `elNode` (System.Xml.XmlNode) — Xml data about node to migrate
- `type` (System.Type) — Type of node
- `workspaceVersion` (System.Version) — Version of workspace which the node belongs to

**Returns:** `Dynamo.Migration.NodeMigrationData` — Data about migration of the node

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

#### `void ProcessNodesInWorkspace(System.Xml.XmlDocument xmlDoc, System.Version workspaceVersion, System.Version currentVersion, Dynamo.Graph.Nodes.NodeLoaders.NodeFactory nodeFactory)`

Attempts to migrate nodes in a workspace to the current version of Dynamo.

**Parameters:**
- `xmlDoc` (System.Xml.XmlDocument) — Xml data about workspace with nodes to migrate
- `workspaceVersion` (System.Version) — Version of workspace to migrate
- `currentVersion` (System.Version) — Current Dynamo version
- `nodeFactory` (Dynamo.Graph.Nodes.NodeLoaders.NodeFactory) — Factory to create nodes

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

#### `bool ProcessWorkspace(Dynamo.Graph.Workspaces.WorkspaceInfo workspaceInfo, System.Xml.XmlDocument xmlDoc, bool isTestMode, Dynamo.Graph.Nodes.NodeLoaders.NodeFactory factory)`

Attempts to migrate a workspace to the current version of Dynamo.

**Parameters:**
- `workspaceInfo` (Dynamo.Graph.Workspaces.WorkspaceInfo) — Information about workspace to migrate
- `xmlDoc` (System.Xml.XmlDocument) — Xml data about workspace to migrate
- `isTestMode` (bool) — Indicates if current code is running in tests
- `factory` (Dynamo.Graph.Nodes.NodeLoaders.NodeFactory) — Factory to create nodes

**Returns:** `bool` — True if the workspace is migrated successfully

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

#### `void ProcessWorkspaceMigrations(System.Version currentVersion, System.Xml.XmlDocument xmlDoc, System.Version workspaceVersion)`

Runs all migration methods found on the listed migration target types.

**Parameters:**
- `currentVersion` (System.Version) — Current Dynamo version
- `xmlDoc` (System.Xml.XmlDocument) — Xml data about workspace to migrate
- `workspaceVersion` (System.Version) — Version of workspace to migrate

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

#### `void ReplaceAttributeValue(System.Xml.XmlElement element, string attribute, string from, string to)`

Call this method to replace the value of attribute.

**Parameters:**
- `element` (System.Xml.XmlElement) — Xml element where to replace attribute value
- `attribute` (string) — Attribute name
- `from` (string) — Original value
- `to` (string) — New value

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

#### `void SetFunctionSignature(System.Xml.XmlElement element, string assemblyName, string methodName, string signature)`

Sets function data such as assembly, display name and function signature

**Parameters:**
- `element` (System.Xml.XmlElement) — Xml element where to set data
- `assemblyName` (string) — Assembly name of the function
- `methodName` (string) — Name of the function
- `signature` (string) — FullName of the function

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

### Properties
- `MigrationTargets` (System.Collections.Generic.List<System.Type>, get/set) — A collection of types which contain migration methods.

## NodeMigrationAttribute (class)

Marks methods on a NodeModel to be used for version migration.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

### Constructors
- `void NodeMigrationAttribute(string version)` — NodeMigrationAttribute.NodeMigrationAttribute

### Properties
- `Version` (System.Version, get) — Version specifies the latest workspace version to which this migration will be applied.

## NodeMigrationData (class)

This class contains the resulting nodes as a result of node migration. Note that this class may contain other information (e.g. connectors) in the future in the event a migration process results in other elements.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

### Constructors
- `void NodeMigrationData(System.Xml.XmlDocument document)` — NodeMigrationData.NodeMigrationData

### Methods
#### `void AppendNode(System.Xml.XmlElement node)`

NodeMigrationData.AppendNode

**Parameters:**
- `node` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

#### `void CreateConnector(System.Xml.XmlElement connector)`

NodeMigrationData.CreateConnector

**Parameters:**
- `connector` (System.Xml.XmlElement)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

#### `void CreateConnector(System.Xml.XmlElement startNode, int startIndex, System.Xml.XmlElement endNode, int endIndex)`

NodeMigrationData.CreateConnector

**Parameters:**
- `startNode` (System.Xml.XmlElement)
- `startIndex` (int)
- `endNode` (System.Xml.XmlElement)
- `endIndex` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

#### `void CreateConnectorFromId(string startNodeId, int startIndex, string endNodeId, int endIndex)`

NodeMigrationData.CreateConnectorFromId

**Parameters:**
- `startNodeId` (string)
- `startIndex` (int)
- `endNodeId` (string)
- `endIndex` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

#### `System.Xml.XmlElement FindConnector(Dynamo.Migration.PortId startPort, Dynamo.Migration.PortId endPort)`

Call this method to find the connector in the associate XmlDocument, given its start and end port information.

**Parameters:**
- `startPort` (Dynamo.Migration.PortId) — The identity of the start port.
- `endPort` (Dynamo.Migration.PortId) — The identity of the end port.

**Returns:** `System.Xml.XmlElement` — Returns the notmatching connector if one is found, or null otherwise.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

#### `System.Collections.Generic.IEnumerable<System.Xml.XmlElement> FindConnectors(Dynamo.Migration.PortId portId)`

Given a port, get all connectors that connect to it.

**Parameters:**
- `portId` (Dynamo.Migration.PortId) — The identity of the port for which connectors are to be retrieved.

**Returns:** `System.Collections.Generic.IEnumerable<System.Xml.XmlElement>` — Returns the list of connectors connecting to the given port, or null if no connection is found connecting to it.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

#### `System.Xml.XmlElement FindFirstConnector(Dynamo.Migration.PortId portId)`

Call this method to retrieve the first connector given a port. This method is a near equivalent of FindConnectors, but only return the first connector found. This way the caller codes can be simplified in a way that it does not have the validate the returned list for item count before accessing its element.

**Parameters:**
- `portId` (Dynamo.Migration.PortId) — The identity of the port for which the first connector is to be retrieved.

**Returns:** `System.Xml.XmlElement` — Returns the first connector found to connect to the given port, or null otherwise.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

#### `void ReconnectToPort(System.Xml.XmlElement connector, Dynamo.Migration.PortId port)`

Reconnect a given connector to another port identified by "port".

**Parameters:**
- `connector` (System.Xml.XmlElement) — The connector to update. Note that this parameter can be null, in which case there won't be any movement performed. This simplifies the caller so that it does not have to do a null-check before every call to this method (connectors may not present).
- `port` (Dynamo.Migration.PortId) — The new port to connect to.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

#### `void RemoveFirstConnector(Dynamo.Migration.PortId portId)`

NodeMigrationData.RemoveFirstConnector

**Parameters:**
- `portId` (Dynamo.Migration.PortId)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

### Properties
- `Document` (System.Xml.XmlDocument, get) — NodeMigrationData.Document
- `MigratedNodes` (System.Collections.Generic.IEnumerable<System.Xml.XmlElement>, get) — NodeMigrationData.MigratedNodes

## PortId (struct)

This structure uniquely identifies a given port in the graph.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

### Constructors
- `void PortId(string owningNode, int portIndex, Dynamo.Graph.Nodes.PortType type)` — Creates PortId.

### Properties
- `OwningNode` (string, get) — Node GUID.
- `PortIndex` (int, get) — Index of the port.
- `PortType` (Dynamo.Graph.Nodes.PortType, get) — PortType. The port can be incoming or outcoming.

## WorkspaceMigrationAttribute (class)

Specifies versions which workspace should be migrated between.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/Migration.cs)

### Constructors
- `void WorkspaceMigrationAttribute(string from, string to)` — Initializes a new instance of the Dynamo.Migration.WorkspaceMigrationAttribute class with the versions which workspace should be migrated between.

### Properties
- `From` (System.Version, get) — Returns version which workspace should be migrated from.
- `To` (System.Version, get) — Returns version which workspace should be migrated to.

## WorkspaceMigrations (class)

Contains methods to migrate a workspace from one version to another

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/WorkspaceMigrations.cs)

### Constructors
- `void WorkspaceMigrations(Dynamo.Models.DynamoModel dynamoModel)` — Initializes a new instance of the Dynamo.Migration.WorkspaceMigrations class with current Dynamo.Models.DynamoModel object.

### Methods
#### `void Migrate_0_5_3_to_0_6_0(System.Xml.XmlDocument doc)`

Performs migration of a workspace from 0.5.3.0 to 0.6.1.0.

**Parameters:**
- `doc` (System.Xml.XmlDocument) — The System.Xml.XmlDocument object.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Migration/WorkspaceMigrations.cs)

