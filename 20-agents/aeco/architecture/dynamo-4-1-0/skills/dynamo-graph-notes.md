---
name: dynamo-dynamo-graph-notes
description: This skill encodes the dynamo 4.1.0 surface of the Dynamo.Graph.Notes namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: NoteModel.
---

# Dynamo.Graph.Notes

Auto-generated from vendor docs for dynamo 4.1.0. 1 types in this namespace.

## NoteModel (class)

NoteModel represents notes in Dynamo.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Notes/NoteModel.cs)

### Constructors
- `void NoteModel(double x, double y, string text, System.Guid guid)` — Creates NoteModel.
- `void NoteModel(double x, double y, string text, System.Guid guid, Dynamo.Graph.Nodes.NodeModel pinnedNode)` — Creates NoteModel with a reference to a pinned node.

### Methods
#### `void DeserializeCore(System.Xml.XmlElement nodeElement, Dynamo.Graph.SaveContext context)`

NoteModel.DeserializeCore

**Parameters:**
- `nodeElement` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Notes/NoteModel.cs)

#### `void SerializeCore(System.Xml.XmlElement element, Dynamo.Graph.SaveContext context)`

NoteModel.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Notes/NoteModel.cs)

#### `bool UpdateValueCore(Dynamo.Graph.UpdateValueParams updateValueParams)`

NoteModel.UpdateValueCore

**Parameters:**
- `updateValueParams` (Dynamo.Graph.UpdateValueParams)

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/v4.1.0/src/DynamoCore/Graph/Notes/NoteModel.cs)

### Properties
- `PinnedNode` (Dynamo.Graph.Nodes.NodeModel, get/set) — NodeModel which this Note is pinned to When using the pin to node command note and node become entangled so that if you select and move one the other one moves as well.
- `PinnedNodeGuid` (System.Guid, get/set) — NoteModel.PinnedNodeGuid
- `Text` (string, get/set) — Returns the text inside the note.
- `UndoRedoAction` (Dynamo.Graph.Notes.NoteModel.UndoAction, get/set) — NoteModel.UndoRedoAction

