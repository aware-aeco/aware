---
name: dynamo-dynamo-graph-nodes-nodeloaders
description: This skill encodes the dynamo 4.1.1 surface of the Dynamo.Graph.Nodes.NodeLoaders namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: INodeLoader<T>, INodeFactory<T>, NodeFactory.
---

# Dynamo.Graph.Nodes.NodeLoaders

Auto-generated from vendor docs for dynamo 4.1.1. 3 types in this namespace.

## INodeFactory<T> (interface)

An object which can create a new NodeModel.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeLoaders/NodeFactory.cs)

### Methods
#### `T CreateNode()`

Creates a new NodeModel instance.

**Returns:** `T` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeLoaders/NodeFactory.cs)

## INodeLoader<T> (interface)

An object which can load a NodeModel from Xml.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeLoaders/NodeFactory.cs)

### Methods
#### `T CreateNodeFromXml(System.Xml.XmlElement elNode, Dynamo.Graph.SaveContext context, ProtoCore.Namespace.ElementResolver resolver)`

Create a new NodeModel from its serialized form.

**Parameters:**
- `elNode` (System.Xml.XmlElement) — Serialized NodeModel
- `context` (Dynamo.Graph.SaveContext) — Serialization context
- `resolver` (ProtoCore.Namespace.ElementResolver) — Element resolver for resolve namespace conflict

**Returns:** `T` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeLoaders/NodeFactory.cs)

## NodeFactory (class)

Manages factories and loaders for NodeModels. Can use registered factories and loaders to instantiate and load new NodeModels.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Nodes/NodeLoaders/NodeFactory.cs)

### Constructors
- `void NodeFactory()` — NodeFactory.NodeFactory

