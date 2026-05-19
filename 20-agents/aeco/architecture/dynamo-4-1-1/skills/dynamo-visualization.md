---
name: dynamo-dynamo-visualization
description: This skill encodes the dynamo 4.1.1 surface of the Dynamo.Visualization namespace — 5 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: DefaultRenderPackageFactory, DefaultRenderPackage, IRenderPackageFactory, IRenderPackageSource<T>, RenderPackageCache.
---

# Dynamo.Visualization

Auto-generated from vendor docs for dynamo 4.1.1. 5 types in this namespace.

## DefaultRenderPackage (class)

Example implementation of IRenderPackage. DefaultRenderPackage can be used as package for your visualization.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/DefaultRenderPackageFactory.cs)

### Constructors
- `void DefaultRenderPackage()` — DefaultRenderPackage.DefaultRenderPackage

### Methods
#### `void AddLineStripVertex(double x, double y, double z)`

Add a line vertex to the render package.

**Parameters:**
- `x` (double)
- `y` (double)
- `z` (double)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/DefaultRenderPackageFactory.cs)

#### `void AddLineStripVertexColor(byte red, byte green, byte blue, byte alpha)`

Add a line strip vertex color to the render package.

**Parameters:**
- `red` (byte)
- `green` (byte)
- `blue` (byte)
- `alpha` (byte)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/DefaultRenderPackageFactory.cs)

#### `void AddLineStripVertexCount(int n)`

Add a line strip vertex count to the render package.

**Parameters:**
- `n` (int)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/DefaultRenderPackageFactory.cs)

#### `void AddPointVertex(double x, double y, double z)`

Add a point vertex to the render package.

**Parameters:**
- `x` (double)
- `y` (double)
- `z` (double)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/DefaultRenderPackageFactory.cs)

#### `void AddPointVertexColor(byte red, byte green, byte blue, byte alpha)`

Add a point color to the render package.

**Parameters:**
- `red` (byte)
- `green` (byte)
- `blue` (byte)
- `alpha` (byte)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/DefaultRenderPackageFactory.cs)

#### `void AddTextureMapForMeshVerticesRange(int startIndex, int endIndex, byte[] textureMap, int stride)`

Set a color map for a range of mesh vertices

**Parameters:**
- `startIndex` (int) — The index associated with the first vertex in MeshVertices we want to associate with the texture map
- `endIndex` (int) — The index associated with the last vertex in MeshVertices we want to associate with the texture map
- `textureMap` (byte[]) — An array of bytes representing RGBA colors to be used as a color texture map
- `stride` (int) — The size of one dimension of the colors array

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/DefaultRenderPackageFactory.cs)

#### `void AddTriangleVertex(double x, double y, double z)`

Add a triangle vertex location to the render package.

**Parameters:**
- `x` (double)
- `y` (double)
- `z` (double)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/DefaultRenderPackageFactory.cs)

#### `void AddTriangleVertexColor(byte red, byte green, byte blue, byte alpha)`

Add a triangle vertex color to the render package.

**Parameters:**
- `red` (byte)
- `green` (byte)
- `blue` (byte)
- `alpha` (byte)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/DefaultRenderPackageFactory.cs)

#### `void AddTriangleVertexNormal(double x, double y, double z)`

Add a triangle vertex normal to the render package.

**Parameters:**
- `x` (double)
- `y` (double)
- `z` (double)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/DefaultRenderPackageFactory.cs)

#### `void AddTriangleVertexUV(double u, double v)`

Add a triangle texture coordinate to the render package.

**Parameters:**
- `u` (double)
- `v` (double)

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/DefaultRenderPackageFactory.cs)

#### `void AppendLineVertexColorRange(byte[] colors)`

Append a color range for line vertices.

**Parameters:**
- `colors` (byte[]) — A buffer of R,G,B,A values corresponding to each vertex.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/DefaultRenderPackageFactory.cs)

#### `void AppendMeshVertexColorRange(byte[] colors)`

Append a color range for mesh vertex.

**Parameters:**
- `colors` (byte[]) — A buffer of R,G,B,A values corresponding to each vertex.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/DefaultRenderPackageFactory.cs)

#### `void AppendPointVertexColorRange(byte[] colors)`

Append a color range for point vertices.

**Parameters:**
- `colors` (byte[]) — A buffer of R,G,B,A values corresponding to each vertex.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/DefaultRenderPackageFactory.cs)

#### `void ApplyLineVertexColors(byte[] colors)`

Apply a color to a sequence of line vertices.

**Parameters:**
- `colors` (byte[])

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/DefaultRenderPackageFactory.cs)

#### `void ApplyMeshVertexColors(byte[] colors)`

Apply a color to each mesh vertex.

**Parameters:**
- `colors` (byte[]) — A buffer of R,G,B,A values corresponding to each vertex.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/DefaultRenderPackageFactory.cs)

#### `void ApplyPointVertexColors(byte[] colors)`

Apply a color to each point vertex.

**Parameters:**
- `colors` (byte[]) — A buffer of R,G,B,A values corresponding to each vertex.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/DefaultRenderPackageFactory.cs)

#### `void Clear()`

Clear all render data from the render package.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/DefaultRenderPackageFactory.cs)

#### `void SetColors(byte[] colors)`

Sets an array of bytes that is used as a color map.

**Parameters:**
- `colors` (byte[]) — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/DefaultRenderPackageFactory.cs)

#### `void UpdateLineVertexColorForRange(int startIndex, int endIndex, byte red, byte green, byte blue, byte alpha)`

Update a color to a range of line vertices.

**Parameters:**
- `startIndex` (int) — The index associated with the first vertex in LineVertices we want to associate with a color
- `endIndex` (int) — The index associated with the last vertex in LineVertices we want to associate with a color
- `red` (byte) — byte for the red RGB value
- `green` (byte) — byte for the green RGB value
- `blue` (byte) — byte for the blue RGB value
- `alpha` (byte) — byte for the alpha RGB value

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/DefaultRenderPackageFactory.cs)

#### `void UpdateMeshVertexColorForRange(int startIndex, int endIndex, byte red, byte green, byte blue, byte alpha)`

Update a color to a range of of mesh vertices.

**Parameters:**
- `startIndex` (int) — The index associated with the first vertex in MeshVertices we want to associate with a color
- `endIndex` (int) — The index associated with the last vertex in MeshVertices we want to associate with a color
- `red` (byte) — byte for the red RGB value
- `green` (byte) — byte for the green RGB value
- `blue` (byte) — byte for the blue RGB value
- `alpha` (byte) — byte for the alpha RGB value

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/DefaultRenderPackageFactory.cs)

#### `void UpdatePointVertexColorForRange(int startIndex, int endIndex, byte red, byte green, byte blue, byte alpha)`

Update a color to a range of point vertices.

**Parameters:**
- `startIndex` (int) — The index associated with the first vertex in PointVertices we want to associate with a color
- `endIndex` (int) — The index associated with the last vertex in PointVertices we want to associate with a color
- `red` (byte) — byte for the red RGB value
- `green` (byte) — byte for the green RGB value
- `blue` (byte) — byte for the blue RGB value
- `alpha` (byte) — byte for the alpha RGB value

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/DefaultRenderPackageFactory.cs)

### Properties
- `AllowLegacyColorOperations` (bool, get/set) — Allow legacy usage of the color methods in IRenderPackage This flag is used by the UpdateRenderPackageAsyncTask implementation to flag any third party usage of deprecated color methods in IRenderPackage API
- `Colors` (System.Collections.Generic.IEnumerable<byte>, get) — Returns the collection of bytes representing RGBA colors. This field can be used to populate textures for mapping onto surfaces. Use the ColorsStride property to define the size of one dimension of the collection.
- `ColorsStride` (int, get/set) — The size of one dimension of the Colors collection.
- `Description` (string, get/set) — A tag used to store information about the render package.
- `DisplayLabels` (bool, get/set) — Returns true if the render package is displaying labels.
- `HasRenderingData` (bool, get) — Returns true if the render package has data.
- `IsSelected` (bool, get/set) — Returns true if the render package is selected.
- `LineStripIndices` (System.Collections.Generic.IEnumerable<int>, get) — Returns the collection containing all line strip indices.
- `LineStripVertexColors` (System.Collections.Generic.IEnumerable<byte>, get) — Returns the collection containing all line strip colors as r1,g1,b1,a1,r2,g2,b2,a2...
- `LineStripVertexCounts` (System.Collections.Generic.IEnumerable<int>, get) — Returns the collection of int values representing how many vertices comprise each line segment in the package.
- `LineStripVertices` (System.Collections.Generic.IEnumerable<double>, get) — Returns the collection containing all line strip vertices as x1,y1,z1,x2,y2,z2...
- `LineVertexColorCount` (int, get) — The number of line vertices colors in the package (Optimized for speed).
- `LineVertexCount` (int, get) — Returns the number of line vertices in the package divided by 3.
- `MeshIndices` (System.Collections.Generic.IEnumerable<int>, get) — Returns the collection containing all mesh vertex indices.
- `MeshNormals` (System.Collections.Generic.IEnumerable<double>, get) — Returns the collection containing all mesh normals as x1,y1,z1,x2,y2,z2...
- `MeshTextureCoordinates` (System.Collections.Generic.IEnumerable<double>, get) — Returns the collection containing all mesh texture coordinates as u1,v1,u2,v2...
- `MeshVertexColorCount` (int, get) — The number of mesh vertices colors in the package (Optimized for speed).
- `MeshVertexColors` (System.Collections.Generic.IEnumerable<byte>, get) — Returns the collection containing all mesh vertex colors as r1,g1,b1,a1,r2,g2,b2,a2...
- `MeshVertexCount` (int, get) — Returns the number of mesh vertices in the package divided by 3.
- `MeshVertices` (System.Collections.Generic.IEnumerable<double>, get) — Returns the collection containing all mesh vertices as x1,y1,z1,x2,y2,z2...
- `MeshVerticesRangesAssociatedWithTextureMaps` (System.Collections.Generic.List<System.Tuple<int, int>>, get) — A list of mesh vertices ranges that have associated texture maps
- `PointIndices` (System.Collections.Generic.IEnumerable<int>, get) — Returns the collection containing all mesh vertex indices.
- `PointVertexColorCount` (int, get) — The number of point vertices colors in the package (Optimized for speed).
- `PointVertexColors` (System.Collections.Generic.IEnumerable<byte>, get) — Returns the collection containing all mesh vertex colors as r1,g1,b1,a1,r2,g2,b2,a2...
- `PointVertexCount` (int, get) — Returns the number of point vertices in the package divided by 3.
- `PointVertices` (System.Collections.Generic.IEnumerable<double>, get) — Returns the collection containing all point vertices as x1,y1,z1,x2,y2,z2...
- `RequiresPerVertexColoration` (bool, get/set) — Returns true if the render package requires per vertex coloration
- `TextureMapsList` (System.Collections.Generic.List<byte[]>, get) — A List containing arrays of bytes representing RGBA colors. These arrays can be used to populate textures for mapping onto specific meshes
- `TextureMapsStrideList` (System.Collections.Generic.List<int>, get) — A list containing the size of one dimension of the associated texture map array in TextureMapsList.

## DefaultRenderPackageFactory (class)

Example implementation of IRenderPackageFactory. DefaultRenderPackageFactory produces DefaultRenderPackages.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/DefaultRenderPackageFactory.cs)

### Constructors
- `void DefaultRenderPackageFactory()` — DefaultRenderPackageFactory.DefaultRenderPackageFactory

### Methods
#### `Autodesk.DesignScript.Interfaces.IRenderPackage CreateRenderPackage()`

Creates DefaultRenderPackage.

**Returns:** `Autodesk.DesignScript.Interfaces.IRenderPackage` — DefaultRenderPackage

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/DefaultRenderPackageFactory.cs)

### Properties
- `TessellationParameters` (Autodesk.DesignScript.Interfaces.TessellationParameters, get/set) — Sets or Returns Tessellation parameters. MaxTessellationDivisions equals 32.

## IRenderPackageFactory (interface)

IRenderPackageFactory is used to create IRenderPackage objects suitable for a specific rendering pipeline. IRenderPackages generated from IRenderPackageFactory classes contain tessellated geometry for rendering, which may be stored in different forms depending on the rendering pipeline being used.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/IRenderPackageFactory.cs)

### Methods
#### `Autodesk.DesignScript.Interfaces.IRenderPackage CreateRenderPackage()`

Create an IRenderPackage object of the type manufactured by this factory.

**Returns:** `Autodesk.DesignScript.Interfaces.IRenderPackage` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/IRenderPackageFactory.cs)

### Properties
- `TessellationParameters` (Autodesk.DesignScript.Interfaces.TessellationParameters, get/set) — Returns or sets the Tessellation parameters.

## IRenderPackageSource<T> (interface)

This interface has events that is fired when the render packages are changed.

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/IRenderPackageSource.cs)

### Events
#### `RenderPackagesUpdated` (`System.Action<T, Dynamo.Visualization.RenderPackageCache>`)

**Signature:** `public event System.Action<T, Dynamo.Visualization.RenderPackageCache> RenderPackagesUpdated`

An event raised then the source has updated IRenderPackages.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/IRenderPackageSource.cs)

## RenderPackageCache (class)

This class controls the collection and distribution of render packages

[Vendor docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/RenderPackageCache.cs)

### Constructors
- `void RenderPackageCache()` — Create an empty RenderPackageCache object
- `void RenderPackageCache(System.Collections.Generic.IEnumerable<Autodesk.DesignScript.Interfaces.IRenderPackage> otherPackages)` — Create a RenderPackageCache object containing the given render packages

### Methods
#### `void Add(Autodesk.DesignScript.Interfaces.IRenderPackage package)`

Adds a render package to this cache

**Parameters:**
- `package` (Autodesk.DesignScript.Interfaces.IRenderPackage) — The package to add to this cache.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/RenderPackageCache.cs)

#### `void Add(Autodesk.DesignScript.Interfaces.IRenderPackage package, System.Guid outputPortId)`

Adds a render package to this cache, including a reference to the output port that the package originated from

**Parameters:**
- `package` (Autodesk.DesignScript.Interfaces.IRenderPackage) — The package to add to this cache.
- `outputPortId` (System.Guid) — The output port to associate the package with.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/RenderPackageCache.cs)

#### `void Add(Dynamo.Visualization.RenderPackageCache other)`

Concatenates the other cache into this cache

**Parameters:**
- `other` (Dynamo.Visualization.RenderPackageCache) — The cache to add to this cache.

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/RenderPackageCache.cs)

#### `Dynamo.Visualization.RenderPackageCache GetPortPackages(System.Guid portId)`

Get the RenderPackageCache object for the given port ID

**Parameters:**
- `portId` (System.Guid) — The port ID used to get the sub-cache.

**Returns:** `Dynamo.Visualization.RenderPackageCache` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/RenderPackageCache.cs)

#### `bool IsEmpty()`

Returns true if the cache is empty

**Returns:** `bool` — 

[Docs](https://github.com/DynamoDS/Dynamo/blob/RC4.1.1_master/src/DynamoCore/Visualization/RenderPackageCache.cs)

### Properties
- `Packages` (System.Collections.Generic.IEnumerable<Autodesk.DesignScript.Interfaces.IRenderPackage>, get) — Get the render packages in this cache

