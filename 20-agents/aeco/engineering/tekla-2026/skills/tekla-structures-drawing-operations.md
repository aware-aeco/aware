---
name: tekla-tekla-structures-drawing-operations
description: This skill encodes the tekla 2026.0 surface of the Tekla.Structures.Drawing.Operations namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Operation.
---

# Tekla.Structures.Drawing.Operations

Auto-generated from vendor docs for tekla 2026.0. 1 types in this namespace.

## Operation (class)

Operations for drawing objects.

[Vendor docs](https://developer.tekla.com/topic/en/18/47/2172371d-8e05-8a9c-a7cf-e27af89483f0)

### Constructors
- `public Operation()` — Initializes a new instance of the Operation class

### Methods
#### `public static bool MergeMarks(IList<MarkBase> marks, out List<MarkBase> mergedMarks)`

Merges given marks objects. NOTE!! Merge requires the original mark content to be similar and of same type. Operation does not support associative notes, mark sets or template content.

**Parameters:**
- `marks` (System.Collections.Generic.IList<MarkBase>) — Contains the of marks to be merged
- `mergedMarks` (System.Collections.Generic.List<MarkBase>.) — Merged marks

**Returns:** `Boolean` — True in case of success, false otherwise

[Docs](https://developer.tekla.com/topic/en/18/47/33045019-b167-2f31-9f39-7fd8fbf633c4)

#### `public static bool SortObjectsByPresentationOrder(IList<DrawingObject> objects, out List<DrawingObject> objectsByPresentationOrder)`

Sorts objects relatively by presentation order from active drawing. Result is used to define relative drawing order of objects which are on top of each other. Last object from the list is drawn on top of all objects.

**Parameters:**
- `objects` (System.Collections.Generic.IList<DrawingObject>) — Contains the objects layers are asked from
- `objectsByPresentationOrder` (System.Collections.Generic.List<DrawingObject>.) — Contains objects sorted by presentation order

**Returns:** `Boolean` — True in case of success, false otherwise

[Docs](https://developer.tekla.com/topic/en/18/47/4762a6c1-1637-130d-c937-1efedfc8045a)

#### `public static bool SplitMarks(IList<MarkBase> marks)`

Splits given marks to original marks.

**Parameters:**
- `marks` (System.Collections.Generic.IList<MarkBase>) — Contains the list of marks to be splitted

**Returns:** `Boolean` — True in case of success, false otherwise

[Docs](https://developer.tekla.com/topic/en/18/47/4a588896-d9e1-e726-94a1-9b5c3e099f37)

