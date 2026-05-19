---
name: dynamo-dynamo-graph
description: This skill encodes the dynamo 4.1.0 surface of the Dynamo.Graph namespace — 6 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ConnectorPinModel, SaveContext, UpdateValueParams, ModelBase, ILocatable, NodeGraph.
---

# Dynamo.Graph

Auto-generated from vendor docs for dynamo 4.1.0. 6 types in this namespace.

## ConnectorPinModel (class)

Connector Pin Model

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/ConnectorPinModel.cs)

### Constructors
- `void ConnectorPinModel(double x, double y, System.Guid id, System.Guid connectorId)` — Constructor

### Methods
#### `void DeserializeCore(System.Xml.XmlElement nodeElement, Dynamo.Graph.SaveContext context)`

ConnectorPinModel.DeserializeCore

**Parameters:**
- `nodeElement` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/ConnectorPinModel.cs)

#### `void SerializeCore(System.Xml.XmlElement element, Dynamo.Graph.SaveContext context)`

ConnectorPinModel.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/ConnectorPinModel.cs)

#### `bool UpdateValueCore(Dynamo.Graph.UpdateValueParams updateValueParams)`

ConnectorPinModel.UpdateValueCore

**Parameters:**
- `updateValueParams` (Dynamo.Graph.UpdateValueParams)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/ConnectorPinModel.cs)

### Properties
- `ConnectorId` (System.Guid, get/set) — Connector Id
- `Height` (double, get) — Static height of pin
- `StaticWidth` (double, get) — Required when needing to pull a reliable value for objects of this type when there is no instances of this object type.
- `Width` (double, get) — Static width of pin

## ILocatable (interface)

Interface contains definitions for locatable objects. Objects such as nodes, connectors, workspaces etc.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/ModelBase.cs)

### Methods
#### `void ReportPosition()`

Notify listeners that the position of the object has changed, then all dependant objects will also redraw themselves.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/ModelBase.cs)

### Properties
- `CenterX` (double, get/set) — X coordinate of center point.
- `CenterY` (double, get/set) — Y coordinate of center point.
- `Height` (double, get/set) — Height of locatable object.
- `Rect` (Dynamo.Utilities.Rect2D, get) — Bounds of locatable object.
- `Width` (double, get/set) — Width of locatable object.
- `X` (double, get/set) — X coordinate of locatable object.
- `Y` (double, get/set) — Y coordinate of locatable object.

## ModelBase (class)

Base class for all objects with which user can interact in Dynamo.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/ModelBase.cs)

### Constructors
- `void ModelBase()` — Protected constructor.

### Methods
#### `System.Xml.XmlElement CreateElement(System.Xml.XmlDocument xmlDocument, Dynamo.Graph.SaveContext context)`

ModelBase.CreateElement

**Parameters:**
- `xmlDocument` (System.Xml.XmlDocument)
- `context` (Dynamo.Graph.SaveContext)

**Returns:** `System.Xml.XmlElement` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/ModelBase.cs)

#### `void Deselect()`

This method sets the isSelected property on the object to false. E.g. when user clicks on canvas.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/ModelBase.cs)

#### `void Deserialize(System.Xml.XmlElement element, Dynamo.Graph.SaveContext context)`

Deserialize model from xml node.

**Parameters:**
- `element` (System.Xml.XmlElement) — Xml node
- `context` (Dynamo.Graph.SaveContext) — Save context. E.g. save in file, copy node etc.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/ModelBase.cs)

#### `void DeserializeCore(System.Xml.XmlElement nodeElement, Dynamo.Graph.SaveContext context)`

ModelBase.DeserializeCore

**Parameters:**
- `nodeElement` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/ModelBase.cs)

#### `void Dispose()`

Invokes Dispose on the object.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/ModelBase.cs)

#### `void Log(Dynamo.Logging.ILogMessage obj)`

ModelBase.Log

**Parameters:**
- `obj` (Dynamo.Logging.ILogMessage)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/ModelBase.cs)

#### `void Log(string msg)`

ModelBase.Log

**Parameters:**
- `msg` (string)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/ModelBase.cs)

#### `void Log(string msg, Dynamo.Logging.WarningLevel severity)`

ModelBase.Log

**Parameters:**
- `msg` (string)
- `severity` (Dynamo.Logging.WarningLevel)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/ModelBase.cs)

#### `void OnDeletionStarted(System.ComponentModel.CancelEventArgs e)`

Invokes the DeletionStarted event on this object.

**Parameters:**
- `e` (System.ComponentModel.CancelEventArgs)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/ModelBase.cs)

#### `void ReportPosition()`

Notifies listeners that the position of the object has changed, then all dependant objects will also redraw themselves.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/ModelBase.cs)

#### `void Select()`

This method sets the isSelected property on the object to true. E.g. when user clicks on node.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/ModelBase.cs)

#### `System.Xml.XmlElement Serialize(System.Xml.XmlDocument xmlDocument, Dynamo.Graph.SaveContext context)`

Serialize model into xml node.

**Parameters:**
- `xmlDocument` (System.Xml.XmlDocument) — Xml document
- `context` (Dynamo.Graph.SaveContext) — Context in which object is saved

**Returns:** `System.Xml.XmlElement` — xml node

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/ModelBase.cs)

#### `void SerializeCore(System.Xml.XmlElement element, Dynamo.Graph.SaveContext context)`

ModelBase.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/ModelBase.cs)

#### `void SetSize(double w, double h)`

Set the width and the height of the node model and report once.

**Parameters:**
- `w` (double) — Width
- `h` (double) — Height

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/ModelBase.cs)

#### `bool ShouldSerializeX()`

Override in derived classes to specify whether to serialize the X property.

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/ModelBase.cs)

#### `bool ShouldSerializeY()`

Override in derived classes to specify whether to serialize the Y property.

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/ModelBase.cs)

#### `bool UpdateValue(Dynamo.Graph.UpdateValueParams updateValueParams)`

Updates object properties. UpdateValueCore is overridden in derived classes.

**Parameters:**
- `updateValueParams` (Dynamo.Graph.UpdateValueParams) — Please see UpdateValueParams for details.

**Returns:** `bool` — Returns true if the call has been handled, or false otherwise.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/ModelBase.cs)

#### `bool UpdateValueCore(Dynamo.Graph.UpdateValueParams updateValueParams)`

This method is supplied as a generic way for command framework to update a given named-value in a ModelBase (which has to work under both user and playback scenarios). During playback, the command framework issues pre-recorded UpdateModelValueCommand that targets a model. Since there is no data-binding at play here, there will not be IValueConverter. This method takes only string input (the way they appear in DynamoTextBox), which overridden method can use for value conversion.

**Parameters:**
- `updateValueParams` (Dynamo.Graph.UpdateValueParams) — Please see UpdateValueParams for details.

**Returns:** `bool` — Returns true if the call has been handled, or false otherwise.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/ModelBase.cs)

### Properties
- `CenterX` (double, get/set) — X coordinate of center point.
- `CenterY` (double, get/set) — Y coordinate of center point.
- `GUID` (System.Guid, get/set) — Unique ID.
- `Height` (double, get/set) — The height of the object.
- `IsSelected` (bool, get/set) — Returns true if the object is selected otherwise false.
- `Position` (Dynamo.Utilities.Point2D, get) — A position defined by the x and y components. Used for notification in situations where you don't want to have property notifications for X and Y
- `Rect` (Dynamo.Utilities.Rect2D, get) — The bounds of the object.
- `Width` (double, get/set) — The width of the object.
- `X` (double, get/set) — The X coordinate of the node in canvas space.
- `Y` (double, get/set) — The Y coordinate of the node in canvas space.

### Events
#### `DeletionStarted` (`System.EventHandler<System.ComponentModel.CancelEventArgs>`)

**Signature:** `public event System.EventHandler<System.ComponentModel.CancelEventArgs> DeletionStarted`

Fired when this Model is about to be deleted. Operation can be cancelled by setting the Cancel flag on the args object to true.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/ModelBase.cs)

#### `Disposed` (`System.Action<Dynamo.Graph.ModelBase>`)

**Signature:** `public event System.Action<Dynamo.Graph.ModelBase> Disposed`

Fired when this Model is disposed.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/ModelBase.cs)

#### `MessageLogged` (`System.Action<Dynamo.Logging.ILogMessage>`)

**Signature:** `public event System.Action<Dynamo.Logging.ILogMessage> MessageLogged`

This event is fired when a message is logged.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/ModelBase.cs)

## NodeGraph (class)

This class is used to load workspace from xml file.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/NodeGraph.cs)

### Methods
#### `Dynamo.Graph.Connectors.ConnectorModel LoadConnectorFromXml(System.Xml.XmlElement connEl, System.Collections.Generic.IDictionary<System.Guid, Dynamo.Graph.Nodes.NodeModel> nodes)`

Creates and initializes a ConnectorModel from its Xml representation.

**Parameters:**
- `connEl` (System.Xml.XmlElement) — XmlElement for a ConnectorModel.
- `nodes` (System.Collections.Generic.IDictionary<System.Guid, Dynamo.Graph.Nodes.NodeModel>) — Dictionary to be used for looking up a NodeModel by it's Guid.

**Returns:** `Dynamo.Graph.Connectors.ConnectorModel` — Returns the new instance of ConnectorModel loaded from XmlElement.

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/NodeGraph.cs)

#### `Dynamo.Graph.NodeGraph LoadGraphFromXml(System.Xml.XmlDocument xmlDoc, Dynamo.Graph.Nodes.NodeLoaders.NodeFactory nodeFactory)`

Loads NodeModels, ConnectorModels, and NoteModels from an XmlDocument.

**Parameters:**
- `xmlDoc` (System.Xml.XmlDocument) — An XmlDocument representing a serialized Dynamo workspace.
- `nodeFactory` (Dynamo.Graph.Nodes.NodeLoaders.NodeFactory) — A NodeFactory, used to load and instantiate nodes.

**Returns:** `Dynamo.Graph.NodeGraph` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/NodeGraph.cs)

#### `Dynamo.Graph.Nodes.NodeModel LoadNodeFromXml(System.Xml.XmlElement elNode, Dynamo.Graph.SaveContext context, Dynamo.Graph.Nodes.NodeLoaders.NodeFactory nodeFactory, ProtoCore.Namespace.ElementResolver resolver)`

Creates and initializes a NodeModel from its Xml representation.

**Parameters:**
- `elNode` (System.Xml.XmlElement) — XmlElement for a NodeModel.
- `context` (Dynamo.Graph.SaveContext) — The serialization context for initialization.
- `nodeFactory` (Dynamo.Graph.Nodes.NodeLoaders.NodeFactory) — A NodeFactory, to be used to create the node.
- `resolver` (ProtoCore.Namespace.ElementResolver) — 

**Returns:** `Dynamo.Graph.Nodes.NodeModel` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/NodeGraph.cs)

#### `Dynamo.Graph.Notes.NoteModel LoadNoteFromXml(System.Xml.XmlNode note)`

Creates and initializes a NoteModel from its Xml representation.

**Parameters:**
- `note` (System.Xml.XmlNode) — 

**Returns:** `Dynamo.Graph.Notes.NoteModel` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/NodeGraph.cs)

#### `System.Collections.Generic.IEnumerable<Dynamo.Graph.Presets.PresetModel> LoadPresetsFromXml(System.Xml.XmlDocument xmlDoc, System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.NodeModel> nodesInNodeGraph)`

Loads presets from xml file.

**Parameters:**
- `xmlDoc` (System.Xml.XmlDocument) — xml file
- `nodesInNodeGraph` (System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.NodeModel>) — node models

**Returns:** `System.Collections.Generic.IEnumerable<Dynamo.Graph.Presets.PresetModel>` — list of presets

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/NodeGraph.cs)

### Properties
- `Annotations` (System.Collections.Generic.List<Dynamo.Graph.Annotations.AnnotationModel>, get) — Dynamo.Graph.Annotations.AnnotationModel loaded from xml.
- `Connectors` (System.Collections.Generic.List<Dynamo.Graph.Connectors.ConnectorModel>, get) — Dynamo.Graph.Connectors.ConnectorModel loaded from xml.
- `ElementResolver` (ProtoCore.Namespace.ElementResolver, get) — Partial class name nodes loaded from xml. E.g. Range turns into DSCoreNodesUI.Range.
- `Nodes` (System.Collections.Generic.List<Dynamo.Graph.Nodes.NodeModel>, get) — Dynamo.Graph.Nodes.NodeModel loaded from xml.
- `Notes` (System.Collections.Generic.List<Dynamo.Graph.Notes.NoteModel>, get) — Dynamo.Graph.Notes.NoteModel loaded from xml.
- `Presets` (System.Collections.Generic.List<Dynamo.Graph.Presets.PresetModel>, get) — Dynamo.Graph.Presets.PresetModel loaded from xml.

## SaveContext (enum)

SaveContext represents several contexts, in which node can be serialized/deserialized.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/ModelBase.cs)

### Values
- `Copy` = `1`
- `File` = `0`
- `None` = `4`
- `Preset` = `3`
- `Save` = `5`
- `SaveAs` = `6`
- `Undo` = `2`

## UpdateValueParams (class)

This class encapsulates the input parameters that need to be passed into nodes when they are updated in the UI.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/ModelBase.cs)

### Constructors
- `void UpdateValueParams(string propertyName, string propertyValue, ProtoCore.Namespace.ElementResolver elementResolver)` — 

### Properties
- `ElementResolver` (ProtoCore.Namespace.ElementResolver, get) — Returns Dynamo.Graph.UpdateValueParams.ElementResolver object responsible for resolving class namespaces
- `PropertyName` (string, get) — Returns name of the property whose value needs to be updated.
- `PropertyValue` (string, get) — Returns string representation of value to update specified node property

