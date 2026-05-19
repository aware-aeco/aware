---
name: dynamo-dynamo-graph-connectors
description: This skill encodes the dynamo 4.1.0 surface of the Dynamo.Graph.Connectors namespace — 2 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ConnectorType, ConnectorModel.
---

# Dynamo.Graph.Connectors

Auto-generated from vendor docs for dynamo 4.1.0. 2 types in this namespace.

## ConnectorModel (class)

Represents a connector between nodes. Connector can be a bezier or polyline Dynamo.Graph.Connectors.ConnectorType.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Connectors/ConnectorModel.cs)

### Constructors
- `void ConnectorModel(Dynamo.Graph.Nodes.PortModel start, Dynamo.Graph.Nodes.PortModel end, System.Guid guid)` — Constructor used when only the start and end Dynamo.Graph.Nodes.PortModel are known.

### Methods
#### `void DeserializeCore(System.Xml.XmlElement nodeElement, Dynamo.Graph.SaveContext context)`

ConnectorModel.DeserializeCore

**Parameters:**
- `nodeElement` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Connectors/ConnectorModel.cs)

#### `void OnDeleted()`

ConnectorModel.OnDeleted

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Connectors/ConnectorModel.cs)

#### `void SerializeCore(System.Xml.XmlElement element, Dynamo.Graph.SaveContext context)`

ConnectorModel.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Connectors/ConnectorModel.cs)

#### `bool UpdateValueCore(Dynamo.Graph.UpdateValueParams updateValueParams)`

ConnectorModel.UpdateValueCore

**Parameters:**
- `updateValueParams` (Dynamo.Graph.UpdateValueParams)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Connectors/ConnectorModel.cs)

#### `bool op_Equality(Dynamo.Graph.Connectors.ConnectorModel lhs, Dynamo.Graph.Connectors.ConnectorModel rhs)`

Overload for EQUAL operator.

**Parameters:**
- `lhs` (Dynamo.Graph.Connectors.ConnectorModel)
- `rhs` (Dynamo.Graph.Connectors.ConnectorModel)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Connectors/ConnectorModel.cs)

#### `bool op_Inequality(Dynamo.Graph.Connectors.ConnectorModel lhs, Dynamo.Graph.Connectors.ConnectorModel rhs)`

Overload for NOT EQUAL operator.

**Parameters:**
- `lhs` (Dynamo.Graph.Connectors.ConnectorModel)
- `rhs` (Dynamo.Graph.Connectors.ConnectorModel)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Connectors/ConnectorModel.cs)

### Properties
- `ConnectorPinModels` (System.Collections.ObjectModel.ObservableCollection<Dynamo.Graph.ConnectorPinModel>, get) — Collection of ConnectorPinModels associated with this Connector.
- `End` (Dynamo.Graph.Nodes.PortModel, get) — Returns end port model.
- `GUID` (System.Guid, get/set) — ID of the Connector, which is unique within the graph.
- `IsHidden` (bool, get/set) — IsHidden flag controlling the visibility of a connector
- `Start` (Dynamo.Graph.Nodes.PortModel, get) — Returns start port model.

### Events
#### `Deleted` (`System.Action`)

**Signature:** `public event System.Action Deleted`

Occurs when deleting connector.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Connectors/ConnectorModel.cs)

## ConnectorType (enum)

Returns the Type of connector.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Connectors/ConnectorModel.cs)

### Values
- `BEZIER` = `0` — Displays connectors as bezier curves
- `POLYLINE` = `1` — Displays connectors as set of connected straight lines

