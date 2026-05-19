---
name: dynamo-dynamo-graph-annotations
description: This skill encodes the dynamo 4.1.1 surface of the Dynamo.Graph.Annotations namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: AnnotationModel.
---

# Dynamo.Graph.Annotations

Auto-generated from vendor docs for dynamo 4.1.1. 1 types in this namespace.

## AnnotationModel (class)

This class contains methods and properties used for creating groups in Dynamo.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Annotations/AnnotationModel.cs)

### Constructors
- `void AnnotationModel(System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.NodeModel> nodes, System.Collections.Generic.IEnumerable<Dynamo.Graph.Notes.NoteModel> notes)` — Initializes a new instance of the Dynamo.Graph.Annotations.AnnotationModel class.
- `void AnnotationModel(System.Collections.Generic.IEnumerable<Dynamo.Graph.Nodes.NodeModel> nodes, System.Collections.Generic.IEnumerable<Dynamo.Graph.Notes.NoteModel> notes, System.Collections.Generic.IEnumerable<Dynamo.Graph.Annotations.AnnotationModel> groups)` — Initializes a new instance of the Dynamo.Graph.Annotations.AnnotationModel class.

### Methods
#### `bool ContainsModel(Dynamo.Graph.ModelBase modelBase)`

Checks if the provided modelbase belongs to this group.

**Parameters:**
- `modelBase` (Dynamo.Graph.ModelBase) — modelbase to check if belongs to this group

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Annotations/AnnotationModel.cs)

#### `void Deselect()`

Overriding the Deselect behavior because deselecting the group should deselect the models within that group

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Annotations/AnnotationModel.cs)

#### `void DeserializeCore(System.Xml.XmlElement element, Dynamo.Graph.SaveContext context)`

AnnotationModel.DeserializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Annotations/AnnotationModel.cs)

#### `void Dispose()`

Implementation of Dispose method

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Annotations/AnnotationModel.cs)

#### `void Select()`

Overriding the Select behavior because selecting the group should select the models within that group

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Annotations/AnnotationModel.cs)

#### `void SerializeCore(System.Xml.XmlElement element, Dynamo.Graph.SaveContext context)`

AnnotationModel.SerializeCore

**Parameters:**
- `element` (System.Xml.XmlElement)
- `context` (Dynamo.Graph.SaveContext)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Annotations/AnnotationModel.cs)

#### `bool UpdateValueCore(Dynamo.Graph.UpdateValueParams updateValueParams)`

Group the Models based on Height and Width

**Parameters:**
- `updateValueParams` (Dynamo.Graph.UpdateValueParams)

**Returns:** `bool` — the width and height of the last model

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Annotations/AnnotationModel.cs)

### Properties
- `AnnotationDescriptionText` (string, get/set) — Group description text
- `AnnotationText` (string, get/set) — Returns title of the group
- `Background` (string, get/set) — Returns background of the group
- `DeletedModelBases` (System.Collections.Generic.List<Dynamo.Graph.ModelBase>, get/set) — DeletedModelBases is used to keep track of deleted / ungrouped models. During Undo operations this is used to get those models that are deleted from the group
- `FontSize` (double, get/set) — Returns font size of the text of the group
- `GUID` (System.Guid, get/set) — ID of the AnnotationModel, which is unique within the graph.
- `GroupState` (Dynamo.Graph.Nodes.ElementState, get) — Indicates whether the group contains nodes that are in an info/warning/error state. This includes the state of any nodes that are in nested groups.
- `GroupStyleId` (System.Guid, get/set) — Returns the Groupstyle applied
- `HasNestedGroups` (bool, get) — Checks if this group contains any nested groups.
- `HasToggledOptionalInPorts` (bool, get/set) — Indicates whether the user manually toggled the visibility of optional input ports. If true, this overrides the global preference setting.
- `HasToggledUnconnectedOutPorts` (bool, get/set) — Indicates whether the user manually toggled the visibility of unconnected output ports. If true, this overrides the global preference setting.
- `Height` (double, get/set) — Returns the full height of the group. That means ModelAreaHeight + TextBlockHeight
- `HeightAdjustment` (double, get/set) — Adjustment margin to be added to the height of the group. When set the height of the group will always be set to Height + heightAdjustment
- `InitialHeight` (double, get/set) — Required to calculate the HEIGHT of a group
- `InitialTop` (double, get/set) — Required to calculate the TOP position in a group
- `IsCollapsedToMinSize` (bool, get/set) — Gets or sets a value indicating whether the group was manually resized while collapsed
- `IsExpanded` (bool, get/set) — Returns whether or not the group is expanded.
- `IsFrozen` (bool, get) — Returns whether or not all nodes in the group are frozen.
- `IsOptionalInPortsCollapsed` (bool, get/set) — Indicates whether optional input ports were manually expanded or collapsed when the graph was last saved. Used only for serialization.
- `IsUnconnectedOutPortsCollapsed` (bool, get/set) — Indicates whether unconnected output ports were manually expanded or collapsed when the graph was last saved. Used only for serialization.
- `IsVisible` (bool, get) — Preview visibility of the nodes in a group
- `MinWidthOnCollapsed` (double, get/set) — Gets or sets the minimum width of the group when it is collapsed. This value equals the combined width of the group's proxy input and output ports.
- `ModelAreaHeight` (double, get/set) — Returns the height of the area that all model belonging to this group is encapsulated in.
- `Nodes` (System.Collections.Generic.IEnumerable<Dynamo.Graph.ModelBase>, get/set) — Returns collection of models (nodes and notes) which the group contains
- `PinnedNode` (Dynamo.Graph.Nodes.NodeModel, get/set) — Optional reference to pinned node This reference will be used in note serialization as note is deserialized from an annotation model
- `Rect` (Dynamo.Utilities.Rect2D, get) — Overriding the Rect from Modelbase This queries the actual RECT of the group. This is required to make the group as ILocatable.
- `Text` (string, get/set) — Returns text of the group
- `TextBlockHeight` (double, get/set) — Returns height of the text area of the group
- `TextMaxWidth` (double, get/set) — Returns the maxWidth of text area of the group
- `UserSetHeight` (double, get/set) — Indicates the height the user manually set using the resize thumb.
- `UserSetWidth` (double, get/set) — Indicates the height the user manually set using the resize thumb. Indicates the width the user manually set using the resize thumb. Not necessarily equal to the actual rendered width.
- `Width` (double, get/set) — Returns width of the group
- `WidthAdjustment` (double, get/set) — Adjustment margin to be added to the width of the group. When set the width of the group will always be set to Width + widthAdjustment
- `loadFromXML` (bool, get/set) — Indicates if group properties should be read from xml data

### Events
#### `ModelBaseRequested` (`System.Func<System.Guid, Dynamo.Graph.ModelBase>`)

**Signature:** `public event System.Func<System.Guid, Dynamo.Graph.ModelBase> ModelBaseRequested`

Triggers when it needs to get the model to add from Workspace

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Graph/Annotations/AnnotationModel.cs)

