---
name: grasshopper-grasshopper-kernel-geometry-spatialtrees
description: This skill encodes the grasshopper 7.0 surface of the Grasshopper.Kernel.Geometry.SpatialTrees namespace — 5 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Index3d<T>, Node3d<T>, TreeDelegates, Coordinates3d<T>, Validation3d<T>.
---

# Grasshopper.Kernel.Geometry.SpatialTrees

Auto-generated from vendor docs for grasshopper 7.0. 5 types in this namespace.

## Coordinates3d<T> (delegate)

Delegate for coordinate retrieval.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Geometry_SpatialTrees_Coordinates3d_1.htm)

## Index3d<T> (class)

Represents an element index within a Tree3d instance.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Geometry_SpatialTrees_Index3d_1.htm)

### Constructors
- `public Index3d(Node3d<T> node, int nodeIndex)` — Initializes a new instance of the Index3d<T> class

### Methods
#### `public int CompareTo(Index3d<T> other)`



**Parameters:**
- `other` (Grasshopper.Kernel.Geometry.SpatialTrees.Index3d<T>)

**Returns:** `Int32` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_Index3d_1_CompareTo.htm)

### Properties
- `GlobalIndex` (Int32, get) — Gets the item index within the tree global list.
- `Item` (T, get) — Gets the actual item.
- `LocalIndex` (Int32, get) — Gets the index within the node index list.
- `Node` (Node3d<T>, get) — Gets the node of the tree in which this element is stored.

## Node3d<T> (class)

Basic node in a Tree3d structure. Nodes in tree structures maintain a local region and either a list of content indices or a list of up to 8 child nodes.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Geometry_SpatialTrees_Node3d_1.htm)

### Constructors
- `public Node3d(Coordinates3d<T> converter, BoundingBox region)` — Create a new spatial tree root.
- `public Node3d(Coordinates3d<T> converter, BoundingBox region, int limit)` — Create a new spatial tree root.

### Methods
#### `public void Add(T item)`

Insert another item into the tree. The item should be within the region of this node.

**Parameters:**
- `item` (T) — Item to insert.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_Node3d_1_Add.htm)

#### `public void AddRange(IEnumerable<T> items)`

Insert a collection of items into the tree. The items should all be within the region of this node.

**Parameters:**
- `items` (System.Collections.Generic.IEnumerable<T>) — Items to add.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_Node3d_1_AddRange.htm)

#### `public void AddToRhinoDocument(RhinoDoc doc)`



**Parameters:**
- `doc` (RhinoDoc)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_Node3d_1_AddToRhinoDocument.htm)

#### `public Node3d<T> CollapseNodes()`

Collapse the (sub)tree rooted at this node. Collapsing happens when a node only has a single child, in which case the child usurps the position previously held by the parent. Do not collapse a (sub)tree if you still plan to add items later.

**Returns:** `Node3d<T>` — The new root for this (sub)tree. May well be an instance of this node if it cannot be collapsed.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_Node3d_1_CollapseNodes.htm)

#### `public Index3d<T> FurthestItem(double x, double y, double z)`

Find the furtest item.

**Parameters:**
- `x` (System.Double) — x coordinate to search from.
- `y` (System.Double) — y coordinate to search from.
- `z` (System.Double) — z coordinate to search from.

**Returns:** `Index3d<T>` — The index of the furtest item or null if no furtest item could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_Node3d_1_FurthestItem_2.htm)

#### `public Index3d<T> FurthestItem(double x, double y, double z, double minimumDistance, double maximumDistance)`

Find the furtest item.

**Parameters:**
- `x` (System.Double) — x coordinate to search from.
- `y` (System.Double) — y coordinate to search from.
- `z` (System.Double) — z coordinate to search from.
- `minimumDistance` (System.Double) — Minimum distance for furtest item. Items closer than minimumDistance will be ignored.
- `maximumDistance` (System.Double) — Maximum distance for furtest item. Items further than maximumDistance will be ignored.

**Returns:** `Index3d<T>` — The index of the furtest item or null if no furtest item could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_Node3d_1_FurthestItem_3.htm)

#### `public Index3d<T> FurthestItem(Point3d locus)`

Find the furtest item.

**Parameters:**
- `locus` (Point3d) — Location to search from.

**Returns:** `Index3d<T>` — The index of the furtest item or null if no furtest item could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_Node3d_1_FurthestItem.htm)

#### `public Index3d<T> FurthestItem(Point3d locus, double minimumDistance, double maximumDistance)`

Find the furtest item.

**Parameters:**
- `locus` (Point3d) — Location to search from.
- `minimumDistance` (System.Double) — Minimum distance for furtest item. Items closer than minimumDistance will be ignored.
- `maximumDistance` (System.Double) — Maximum distance for furtest item. Items further than maximumDistance will be ignored.

**Returns:** `Index3d<T>` — The index of the furtest item or null if no furtest item could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_Node3d_1_FurthestItem_1.htm)

#### `public Index3d<T> FurthestItem(T locus)`

Find the furtest item.

**Parameters:**
- `locus` (T) — Location to search from.

**Returns:** `Index3d<T>` — The index of the furtest item or null if no furtest item could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_Node3d_1_FurthestItem_4.htm)

#### `public Index3d<T> FurthestItem(T locus, double minimumDistance, double maximumDistance)`

Find the furtest item.

**Parameters:**
- `locus` (T) — Location to search from.
- `minimumDistance` (System.Double) — Minimum distance for furtest item. Items closer than minimumDistance will be ignored.
- `maximumDistance` (System.Double) — Maximum distance for furtest item. Items further than maximumDistance will be ignored.

**Returns:** `Index3d<T>` — The index of the furtest item or null if no furtest item could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_Node3d_1_FurthestItem_5.htm)

#### `public Index3d<T> NearestItem(double x, double y, double z)`

Find the nearest item.

**Parameters:**
- `x` (System.Double) — x coordinate to search from.
- `y` (System.Double) — y coordinate to search from.
- `z` (System.Double) — z coordinate to search from.

**Returns:** `Index3d<T>` — The index of the nearest item or null if no nearest item could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_Node3d_1_NearestItem_2.htm)

#### `public Index3d<T> NearestItem(double x, double y, double z, double minimumDistance, double maximumDistance)`

Find the nearest item.

**Parameters:**
- `x` (System.Double) — x coordinate to search from.
- `y` (System.Double) — y coordinate to search from.
- `z` (System.Double) — z coordinate to search from.
- `minimumDistance` (System.Double) — Minimum distance for nearest item. Items closer than minimumDistance will be ignored.
- `maximumDistance` (System.Double) — Maximum distance for nearest item. Items further than maximumDistance will be ignored.

**Returns:** `Index3d<T>` — The index of the nearest item or null if no nearest item could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_Node3d_1_NearestItem_4.htm)

#### `public Index3d<T> NearestItem(double x, double y, double z, Validation3d<T> validationDelegate)`

Find the nearest item using custom validation criteria.

**Parameters:**
- `x` (System.Double) — x coordinate to search from.
- `y` (System.Double) — y coordinate to search from.
- `z` (System.Double) — z coordinate to search from.
- `validationDelegate` (Grasshopper.Kernel.Geometry.SpatialTrees.Validation3d<T>) — Item validation delegate.

**Returns:** `Index3d<T>` — The index of the nearest item or null if no nearest item could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_Node3d_1_NearestItem_3.htm)

#### `public Index3d<T> NearestItem(Point3d locus)`

Find the nearest item.

**Parameters:**
- `locus` (Point3d) — Location to search from.

**Returns:** `Index3d<T>` — The index of the nearest item or null if no nearest item could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_Node3d_1_NearestItem.htm)

#### `public Index3d<T> NearestItem(Point3d locus, double minimumDistance, double maximumDistance)`

Find the nearest item.

**Parameters:**
- `locus` (Point3d) — Location to search from.
- `minimumDistance` (System.Double) — Minimum distance for nearest item. Items closer than minimumDistance will be ignored.
- `maximumDistance` (System.Double) — Maximum distance for nearest item. Items further than maximumDistance will be ignored.

**Returns:** `Index3d<T>` — The index of the nearest item or null if no nearest item could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_Node3d_1_NearestItem_1.htm)

#### `public Index3d<T> NearestItem(T locus)`

Find the nearest item.

**Parameters:**
- `locus` (T) — Location to search from.

**Returns:** `Index3d<T>` — The index of the nearest item or null if no nearest item could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_Node3d_1_NearestItem_5.htm)

#### `public Index3d<T> NearestItem(T locus, double minimumDistance, double maximumDistance)`

Find the nearest item.

**Parameters:**
- `locus` (T) — Location to search from.
- `minimumDistance` (System.Double) — Minimum distance for nearest item. Items closer than minimumDistance will be ignored.
- `maximumDistance` (System.Double) — Maximum distance for nearest item. Items further than maximumDistance will be ignored.

**Returns:** `Index3d<T>` — The index of the nearest item or null if no nearest item could be found.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_Node3d_1_NearestItem_6.htm)

#### `public List<Index3d<T>> NearestItems(double x, double y, double z, int groupSize)`

Find the N nearest items.

**Parameters:**
- `x` (System.Double) — x coordinate to search from.
- `y` (System.Double) — y coordinate to search from.
- `z` (System.Double) — z coordinate to search from.
- `groupSize` (System.Int32) — Number of nearest items to return.

**Returns:** `List<Index3d<T>>` — A list of nearest items, sorted from nearest to furthest.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_Node3d_1_NearestItems.htm)

#### `public List<Index3d<T>> NearestItems(double x, double y, double z, int groupSize, double minimumDistance, double maximumDistance)`

Find the N nearest items.

**Parameters:**
- `x` (System.Double) — x coordinate to search from.
- `y` (System.Double) — y coordinate to search from.
- `z` (System.Double) — z coordinate to search from.
- `groupSize` (System.Int32) — Number of nearest items to return.
- `minimumDistance` (System.Double) — Minimum distance for nearest items. Items closer than minimumDistance will be ignored.
- `maximumDistance` (System.Double) — Maximum distance for nearest items. Items further than maximumDistance will be ignored.

**Returns:** `List<Index3d<T>>` — A list of nearest items, sorted from nearest to furthest.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_Node3d_1_NearestItems_1.htm)

#### `public List<Index3d<T>> NearestItems(T locus, int groupSize)`

Find the N nearest items.

**Parameters:**
- `locus` (T) — Search locus.
- `groupSize` (System.Int32) — Number of nearest items to return.

**Returns:** `List<Index3d<T>>` — A list of nearest items, sorted from nearest to furthest.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_Node3d_1_NearestItems_2.htm)

#### `public List<Index3d<T>> NearestItems(T locus, int groupSize, double minimumDistance, double maximumDistance)`

Find the N nearest items.

**Parameters:**
- `locus` (T) — Search locus.
- `groupSize` (System.Int32) — Number of nearest items to return.
- `minimumDistance` (System.Double) — Minimum distance for nearest items. Items closer than minimumDistance will be ignored.
- `maximumDistance` (System.Double) — Maximum distance for nearest items. Items further than maximumDistance will be ignored.

**Returns:** `List<Index3d<T>>` — A list of nearest items, sorted from nearest to furthest.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_Node3d_1_NearestItems_3.htm)

#### `public Node3d<T> OptimizeTree()`

Optimize this tree for fast searches. Do not call this method if you still plan to add items in the future. You can no longer modify this tree once it has been optimized.

**Returns:** `Node3d<T>` — The new root of the optimized tree. If the root could not be optimized an instance of this node will be returned.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_Node3d_1_OptimizeTree.htm)

#### `public void Remove(Index3d<T> index)`

Remove the item with the given index from the entire tree. It doesn't matter on which node you call this function, it is a tree-wide operation.

**Parameters:**
- `index` (Grasshopper.Kernel.Geometry.SpatialTrees.Index3d<T>) — Index of item to remove.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_Node3d_1_Remove.htm)

#### `public void Remove(int index)`

Remove the item with the given index from the entire tree. It doesn't matter on which node you call this function, it is a tree-wide operation.

**Parameters:**
- `index` (System.Int32) — Index of item to remove.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_Node3d_1_Remove_1.htm)

#### `public void ShrinkRegions()`

Shrink the region for this node and all child nodes. Do not use this method if you intend to add more items later as it creates spatial gaps in the tree structure. You can shrink nodes if you're done adding items and want to start searching the tree.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_Node3d_1_ShrinkRegions.htm)

#### `public IEnumerator<Node3d<T>> SubTree(bool omitStructuralNodes)`

Gets an iterator for all nodes in this (sub)tree.

**Parameters:**
- `omitStructuralNodes` (System.Boolean) — If true, nodes that contain other nodes will be skipped. Only nodes containing items will be included.

**Returns:** `IEnumerator<Node3d<T>>` — 

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_Node3d_1_SubTree.htm)

#### `public void TrimExcess()`

Trim the excess space on all index lists. You can call this method to reduce memory usage. It will not modify the tree in any functional way.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_Node3d_1_TrimExcess.htm)

### Properties
- `Center` (Point3d, get) — Gets the center of the spatial region of this node. If the node contains no children the center is always in the middle of the Region. If the node does contain child-nodes, the center may be anywhere within the region.
- `ChildCount` (Int32, get) — Gets the number of defined child nodes. Leaf nodes have no children. Root and twig nodes can have anywhere between 1 and 8 children.
- `ChildNode` (Node3d<T>, get) — Gets the child node at the given index.
- `ContentAverage` (Point3d, get) — Returns the average coordinate of all items in this node. If this node does not contain any items, Point3d.Unset is returned.
- `ContentBoundingBox` (BoundingBox, get) — Returns the boundingbox of all items in this node. If this node does not contain any items, BoundingBox.Empty is returned.
- `IndicesLocal` (List<Int32>, get) — Gets the list of item indices that are contained within this node.
- `IndicesRecursive` (List<Int32>, get) — Gets the list of item indices that are contained within this node and any child nodes.
- `IsLeaf` (Boolean, get) — Gets whether this node is a leaf node. Leaf nodes have no child nodes.
- `IsMutable` (Boolean, get) — Gets whether this tree is mutable. You can only add items to mutable trees. We don't recommend removing items from unmutable trees, though that shouldn't necessarily lead to problems. Trees become unmutable after a call to ShrinkRegions(), CollapseNodes() or OptimizeTree().
- `IsRoot` (Boolean, get) — Gets whether this node is a root node. Root nodes have no parent node and depth zero.
- `IsTwig` (Boolean, get) — Gets whether this node is a twig node. Twig nodes have both parents and at least one child.
- `ItemCount` (Int32, get) — Gets the total number of items stored directly in this node.
- `ItemsGlobal` (List<T>, get) — Gets the list of all items stored inside this entire tree. Do not modify this collection unless you know what you are doing.
- `ItemsLocal` (List<T>, get) — Gets a list of all the items stored directly in this node. This list is constructed every time you access this property, so keep it down to a minimum.
- `Limit` (Int32, get) — Gets the subdivision limit for this tree. This limit can only be set once when you create a new tree. It is fixed forever after.
- `MemoryConsumption` (Int64, get) — Gets the estimated memory consumption of the (sub)tree structure. Items inside the global list are not included in this estimate.
- `NextNode` (Node3d<T>, get) — Gets the logical neighbour to the right of this node. There is no spatial relationships between logical neighbours, this is purely an iteration aid.
- `NodeDepth` (Int32, get) — Gets the recursive depth of this node. The tree root is at depth zero. The first subdivision is at depth one, and so on and so forth.
- `ParentNode` (Node3d<T>, get) — Gets the immediate parent of this node. Root nodes have no parent.
- `Region` (BoundingBox, get) — Gets the spatial region of this node.
- `RootNode` (Node3d<T>, get) — Gets the ultimate root node for this tree.
- `WeightedSubdivision` (Boolean, get/set) — Gets or sets whether subdivision is weighted based on content averages. Setting this value will only affect future subdivisions, not existing ones.

## TreeDelegates (class)

Provides a bunch of standard coordinate extractor methods.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Geometry_SpatialTrees_TreeDelegates.htm)

### Methods
#### `public static void Node2Coordinates(Node2 pt, ref double x, ref double y, ref double z)`



**Parameters:**
- `pt` (Node2)
- `x` (System.Double)
- `y` (System.Double)
- `z` (System.Double)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_TreeDelegates_Node2Coordinates.htm)

#### `public static void Node3Coordinates(Node3 pt, ref double x, ref double y, ref double z)`



**Parameters:**
- `pt` (Node3)
- `x` (System.Double)
- `y` (System.Double)
- `z` (System.Double)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_TreeDelegates_Node3Coordinates.htm)

#### `public static void Point2dCoordinates(Point2d pt, ref double x, ref double y, ref double z)`



**Parameters:**
- `pt` (Point2d)
- `x` (System.Double)
- `y` (System.Double)
- `z` (System.Double)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_TreeDelegates_Point2dCoordinates.htm)

#### `public static void Point2fCoordinates(Point2d pt, ref double x, ref double y, ref double z)`



**Parameters:**
- `pt` (Point2d)
- `x` (System.Double)
- `y` (System.Double)
- `z` (System.Double)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_TreeDelegates_Point2fCoordinates.htm)

#### `public static void Point3dCoordinates(Point3d pt, ref double x, ref double y, ref double z)`



**Parameters:**
- `pt` (Point3d)
- `x` (System.Double)
- `y` (System.Double)
- `z` (System.Double)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_TreeDelegates_Point3dCoordinates.htm)

#### `public static void Point3fCoordinates(Point3d pt, ref double x, ref double y, ref double z)`



**Parameters:**
- `pt` (Point3d)
- `x` (System.Double)
- `y` (System.Double)
- `z` (System.Double)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_TreeDelegates_Point3fCoordinates.htm)

#### `public static void PointCoordinates(Point pt, ref double x, ref double y, ref double z)`



**Parameters:**
- `pt` (Point)
- `x` (System.Double)
- `y` (System.Double)
- `z` (System.Double)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/M_Grasshopper_Kernel_Geometry_SpatialTrees_TreeDelegates_PointCoordinates.htm)

## Validation3d<T> (delegate)

Delegate to be used during advanced customized searches.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/T_Grasshopper_Kernel_Geometry_SpatialTrees_Validation3d_1.htm)

