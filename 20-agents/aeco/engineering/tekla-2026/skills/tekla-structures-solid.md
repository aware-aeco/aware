---
name: tekla-tekla-structures-solid
description: This skill encodes the tekla 2026.0 surface of the Tekla.Structures.Solid namespace — 11 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: EdgeEnumerator, Edge, Face, FaceEnumerator, LoopEnumerator, Loop, Shell, ShellEnumerator, and 3 more types.
---

# Tekla.Structures.Solid

Auto-generated from vendor docs for tekla 2026.0. 11 types in this namespace.

## Edge (class)

The Edge class represents a single global edge in a solid.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/4d366e24-93b4-cd9e-139b-06c8be0434f1)

### Methods
#### `Clone(...)`

Method for cloning the current object.

[Docs](https://developer.tekla.com/topic/en/18/47/edc70204-4a28-e97c-bdc8-3fb27d1f9f50)

### Properties
- `EndPoint` (object, get/set) — Gets the end point of the edge.
- `StartPoint` (object, get/set) — Gets the start point of the edge.
- `Type` (object, get/set) — Gets the type of the edge.

## Edge.EdgeTypeEnum (enum)

The type of the edge.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/3fa7b5f2-2f4b-34cb-2986-25ed0babf382)

### Values
- `EDGE_TYPE_NORMAL` = `0` — The normal edge type.
- `EDGE_TYPE_CURVED_SURFACE` = `1` — The curved surface edge type.
- `EDGE_TYPE_HIDDEN` = `2` — The hidden edge type.

## EdgeEnumerator (class)

The EdgeEnumerator class is used to enumerate the edges of a solid.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/7edcc94f-9b44-d752-6310-c162d76eda59)

### Methods
#### `MoveNext(...)`

Moves to the next item in the enumerator.

[Docs](https://developer.tekla.com/topic/en/18/47/a423ca93-c65a-f109-3172-3afe103a2d16)

#### `Reset(...)`

Resets the enumerator to the beginning.

[Docs](https://developer.tekla.com/topic/en/18/47/1a21b63a-93bf-480d-e258-981e7de33612)

### Properties
- `Current` (object, get/set) — Gets the current edge.

## Face (class)

The Face class represents a single face in a solid.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/5831d571-d8ee-b975-19be-799d8dcb1439)

### Methods
#### `GetLoopEnumerator(...)`

Gets a new loop enumerator for enumerating through the face's loops.

[Docs](https://developer.tekla.com/topic/en/18/47/0d4e4992-db2c-3dad-c8a8-87ed34a7f270)

### Properties
- `Normal` (object, get/set) — The normal vector of the face.
- `OriginPartId` (object, get/set) — The part id of the part that this face originated from. For example, a face create by a part cut has the id of the cutting part.

## FaceEnumerator (class)

The FaceEnumerator class is used to enumerate the faces of a solid.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/ddfdfd4e-696e-a234-32b3-9cc3c7041cb3)

### Methods
#### `MoveNext(...)`

Moves to the next item in the enumerator.

[Docs](https://developer.tekla.com/topic/en/18/47/7cb685be-03b2-c801-d86b-76a8de3c24ec)

#### `Reset(...)`

Resets the enumerator to the beginning.

[Docs](https://developer.tekla.com/topic/en/18/47/870623c5-5457-156a-56fd-4eb684ca26d5)

### Properties
- `Current` (object, get/set) — Returns the current face.

## ISolid (interface)

The ISolid interface represents a physical object in the model or a drawing created by a part instance. The solid instance can be used to query the actual geometry of the part.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/3c08ad50-3fdd-a6f7-05b9-fb07c1e25ea9)

### Methods
#### `GetEdgeEnumerator(...)`

Returns a new edge enumerator in the current plane.

[Docs](https://developer.tekla.com/topic/en/18/47/b46e8163-fd7e-cc0a-0f5b-c8d33432af00)

#### `GetFaceEnumerator(...)`

Returns a new face enumerator in the current plane.

[Docs](https://developer.tekla.com/topic/en/18/47/385e2055-7a80-6f26-e2ad-a8627797c834)

### Properties
- `MaximumPoint` (object, get/set) — The maximum axis-aligned point of the solid in the current plane.
- `MinimumPoint` (object, get/set) — The minimum axis-aligned point of the solid in the current plane.

## Loop (class)

The Loop class represents a single loop in a face.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/2fa3c91f-d68b-9348-077b-f1fdde502ef6)

### Methods
#### `GetVertexEnumerator(...)`

Gets a new vertex enumerator.

[Docs](https://developer.tekla.com/topic/en/18/47/ff661a24-9b92-f3e4-7db9-4c062f71f19a)

## LoopEnumerator (class)

The LoopEnumerator class is used to enumerate the loops of a face instance.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/12ea17c8-9b81-5906-dc7f-d14bc934f7c0)

### Methods
#### `MoveNext(...)`

Moves to the next item in the enumerator.

[Docs](https://developer.tekla.com/topic/en/18/47/eda4e568-ca17-a8e5-84d6-fa64d29e8470)

#### `Reset(...)`

Resets the enumerator to the beginning.

[Docs](https://developer.tekla.com/topic/en/18/47/d99626c8-ee72-cbfb-0edf-39323caa4db1)

### Properties
- `Current` (object, get/set) — Returns the current loop.

## Shell (class)

The Shell class represents a single shell in a solid.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/60711cf3-1397-9ec4-0740-4ca81fd8befd)

### Methods
#### `GetEdgeEnumerator(...)`

Returns a new edge enumerator in the current plane.

[Docs](https://developer.tekla.com/topic/en/18/47/98de58ec-a62d-393a-9816-7a8398b2e1cb)

#### `GetFaceEnumerator(...)`

Returns a new face enumerator in the current plane.

[Docs](https://developer.tekla.com/topic/en/18/47/f9e3233e-bebd-896f-b713-4755424f71e1)

## ShellEnumerator (class)

The ShellEnumerator class is used to enumerate the shells of a solid.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/129adcea-7427-5204-6df8-622483d21f39)

### Methods
#### `MoveNext(...)`

Moves to the next item in the enumerator.

[Docs](https://developer.tekla.com/topic/en/18/47/c34845cf-8306-297b-69f0-996247973225)

#### `Reset(...)`

Resets the enumerator to the beginning.

[Docs](https://developer.tekla.com/topic/en/18/47/ecc6e966-8065-dd88-6e15-ab58ed76821a)

### Properties
- `Current` (object, get/set) — Gets the current shell.

## VertexEnumerator (class)

The VertexEnumerator class is used to enumerate the vertexes of a loop instance.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/6cff93f0-7fe6-7e86-61f9-287418f3f72b)

### Methods
#### `MoveNext(...)`

Moves to the next item in the enumerator.

[Docs](https://developer.tekla.com/topic/en/18/47/82296b31-303c-4ca2-940d-5ae78820e294)

#### `Reset(...)`

Resets the enumerator to the beginning.

[Docs](https://developer.tekla.com/topic/en/18/47/17307965-cc44-9be8-dba1-c5d96f9e9039)

### Properties
- `Current` (object, get/set) — Returns the current vertex.

