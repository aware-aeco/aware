---
name: tekla-structures-solid
description: "Tekla Open API - Solid namespace. Iterate solid B-Rep geometry of model parts: shells, faces, loops, vertices, and edges. This skill should be used when working with part solid geometry, face normals, edge extraction, or vertex enumeration in Tekla."
---

# Tekla.Structures.Solid API Reference (v2025)

## Overview

Types for iterating over the solid (B-Rep) geometry of model parts. The iteration pattern is: Solid -> Shells -> Faces -> Loops -> Vertices, and separately Solid -> Edges.

---

## ISolid (interface)
> Represents a physical object's solid geometry. Obtained from a Part via `GetSolid()`.

**Properties:**
- `Point MaximumPoint { get; }` - maximum axis-aligned point of the solid in the current plane
- `Point MinimumPoint { get; }` - minimum axis-aligned point of the solid in the current plane

**Methods:**
- `EdgeEnumerator GetEdgeEnumerator()` - enumerate all edges
- `FaceEnumerator GetFaceEnumerator()` - enumerate all faces

---

## Shell
> Represents a single shell in a solid.

**Methods:**
- `EdgeEnumerator GetEdgeEnumerator()` - enumerate edges of this shell
- `FaceEnumerator GetFaceEnumerator()` - enumerate faces of this shell

---

## ShellEnumerator
> Enumerates the shells of a solid.

**Properties:**
- `object Current { get; }` - current shell (cast to Shell)

**Methods:**
- `bool MoveNext()`
- `void Reset()`

---

## Face
> Represents a single face in a solid.

**Properties:**
- `Vector Normal { get; }` - normal vector of the face
- `Identifier OriginPartId { get; }` - ID of the part that created this face (e.g., a cutting part)

**Methods:**
- `LoopEnumerator GetLoopEnumerator()` - enumerate the loops of this face

---

## FaceEnumerator
> Enumerates the faces of a solid or shell.

**Properties:**
- `Face Current { get; }` - current face

**Methods:**
- `bool MoveNext()`
- `void Reset()`

---

## Loop
> Represents a single loop (boundary) in a face. The first loop is the outer boundary; subsequent loops are holes.

**Methods:**
- `VertexEnumerator GetVertexEnumerator()` - enumerate vertices of this loop

---

## LoopEnumerator
> Enumerates the loops of a face.

**Properties:**
- `Loop Current { get; }` - current loop

**Methods:**
- `bool MoveNext()`
- `void Reset()`

---

## Edge
> Represents a single global edge in a solid.

**Properties:**
- `Point EndPoint { get; }` - end point of the edge
- `Point StartPoint { get; }` - start point of the edge
- `EdgeTypeEnum Type { get; }` - type of the edge

**Methods:**
- `object Clone()`

### Edge.EdgeTypeEnum
| Value | Int | Description |
|-------|-----|-------------|
| EDGE_TYPE_NORMAL | 0 | Normal edge |
| EDGE_TYPE_CURVED_SURFACE | 1 | Curved surface edge |
| EDGE_TYPE_HIDDEN | 2 | Hidden edge |

---

## EdgeEnumerator
> Enumerates the edges of a solid or shell.

**Properties:**
- `object Current { get; }` - current edge (cast to Edge)

**Methods:**
- `bool MoveNext()`
- `void Reset()`

---

## VertexEnumerator
> Enumerates the vertices of a loop.

**Properties:**
- `Point Current { get; }` - current vertex as a Point

**Methods:**
- `bool MoveNext()`
- `void Reset()`

---

## Common Usage Patterns

### Iterating solid faces and vertices
```csharp
Solid solid = part.GetSolid();
FaceEnumerator faceEnum = solid.GetFaceEnumerator();
while (faceEnum.MoveNext())
{
    Face face = faceEnum.Current;
    Vector normal = face.Normal;
    LoopEnumerator loopEnum = face.GetLoopEnumerator();
    while (loopEnum.MoveNext())
    {
        Loop loop = loopEnum.Current;
        VertexEnumerator vertexEnum = loop.GetVertexEnumerator();
        while (vertexEnum.MoveNext())
        {
            Point vertex = vertexEnum.Current;
            // process vertex
        }
    }
}
```
