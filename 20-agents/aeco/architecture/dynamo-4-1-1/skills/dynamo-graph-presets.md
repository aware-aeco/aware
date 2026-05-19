---
name: dynamo-dynamo-graph-presets
description: This skill encodes the dynamo 4.1.1 surface of the Dynamo.Graph.Presets namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: PresetModel.
---

# Dynamo.Graph.Presets

Auto-generated from vendor docs for dynamo 4.1.1. 1 types in this namespace.

## PresetModel (class)

This class references a set of nodemodels, and a set of serialized versions of those nodemodels a client can use this class to store the state of a set of nodes from a graph

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Presets/PresetModel.cs)

### Constructors
- `void PresetModel(string name, string description, System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.NodeModel> inputsToSave)` — Create a new presetsState, this will serialize all the referenced nodes by calling their serialize method, the resulting XML elements will be used to save this state when the presetModel is saved on workspace save

### Methods
#### `void DeserializeCore(System.Xml.XmlElement nodeElement, Dynamo.Graph.SaveContext context)`

PresetModel.DeserializeCore

**Parameters:**
- `nodeElement` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Presets/PresetModel.cs)

#### `void SerializeCore(System.Xml.XmlElement element, Dynamo.Graph.SaveContext context)`

PresetModel.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Presets/PresetModel.cs)

### Properties
- `Description` (string, get) — Returns description of the preset
- `Name` (string, get) — Returns name of the preset
- `Nodes` (System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.NodeModel>, get) — Returns list of nodemodels that this state serializes
- `SerializedNodes` (System.Collections.Generic.IEnumerable<System.Xml.XmlElement>, get) — Returns list of serialized nodes

