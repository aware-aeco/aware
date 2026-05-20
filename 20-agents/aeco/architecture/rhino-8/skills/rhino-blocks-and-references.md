---
name: rhino-blocks-and-references
description: This skill should be used when composing snippets that read or write Rhino's `InstanceObject` / `InstanceDefinition` graph, attach external `.3dm` files as block-references or worksession-references, audit imported geometry for unintentional re-use, or perform the SketchUp-style "find every instance of this part" operation. Covers definition-vs-instance ownership, the `InstanceXform` matrix (origin/scale/rotation extraction), the difference between linked-blocks and worksessions, the gotcha where deleting an instance does NOT delete the definition (and vice versa), and the AWARE curated `block.list-instances` verb's relationship to the underlying API.
---

# Rhino blocks and references

Blocks (called "block definitions" in the UI, `InstanceDefinition` in the API) are the cornerstone of every Rhino model larger than a single object. Get the lifecycle wrong and you either ship a bloated model or accidentally delete other people's geometry.

## Definitions vs instances

A **definition** is the geometry template — a named bundle of objects with their own local axes. It lives in `doc.InstanceDefinitions` and has a GUID + a name.

An **instance** is a placement of a definition at a transform. It's an `InstanceObject` that lives in `doc.Objects` like any other object. It has an `InstanceDefinition` reference (pointing at the definition) and an `InstanceXform` (4x4 placement matrix).

**Multiple instances share one definition.** Edit the definition's geometry → every instance updates. Move one instance → only that instance moves.

## Reading the placement transform

`InstanceObject.InstanceXform` is a `Transform` (4x4 matrix). Extract the three useful pieces:

```csharp
var xform = inst.InstanceXform;

// Origin (where the instance sits in world coordinates)
var origin = new Rhino.Geometry.Point3d(xform.M03, xform.M13, xform.M23);

// Per-axis scale (norm of each column of the upper-left 3x3)
double sx = Math.Sqrt(xform.M00 * xform.M00 + xform.M10 * xform.M10 + xform.M20 * xform.M20);
double sy = Math.Sqrt(xform.M01 * xform.M01 + xform.M11 * xform.M11 + xform.M21 * xform.M21);
double sz = Math.Sqrt(xform.M02 * xform.M02 + xform.M12 * xform.M12 + xform.M22 * xform.M22);

// Rotation about Z (assumes orthogonal axes; degrees)
double rotZ = Math.Atan2(xform.M10 / sx, xform.M00 / sx) * 180.0 / Math.PI;
```

For non-orthogonal (sheared) instances the scale formula above produces nonsense — sheared placements are rare in AECO work but worth knowing about.

## Deleting safely

`doc.Objects.Delete(inst.Id, true)` deletes the INSTANCE but leaves the definition in place. The definition stays in the model — taking memory, available for re-use, browsable in `_-BlockManager`.

To delete the DEFINITION (and ALL its instances), use `doc.InstanceDefinitions.Delete(idef.Index, true, true)`. Pass `true, true` to delete instances AND purge nested empty defs. Without that, you'll have orphan instances pointing at a stale def → CrashRhino on next operation.

The AWARE curated `block.list-instances` verb is a read-only audit primitive that returns position/scale/rotation per instance — it never deletes. Pair it with `_-BlockManager` for visual cleanup.

## Worksessions vs linked blocks

Rhino has TWO different "attach an external .3dm" mechanisms:

| Mechanism | What it is | Lifecycle |
|---|---|---|
| **Linked block** | The external file is loaded as an `InstanceDefinition` with `UpdateType.Linked`. Re-imports on file change. | Per-instance. The link is metadata on the def. |
| **Worksession** | Multiple .3dm files coordinated as one "session" via a .rws file. Each external file is a top-level reference, not a block def. | Per-doc. Loaded via `doc.Worksession`. |

If you see `InstanceDefinition.UpdateType == UpdateType.Linked`, you're looking at a linked block. If you see entries in `doc.Worksession`, you're looking at a worksession reference. They DON'T overlap.

The AWARE curated `worksession.reload` verb refreshes worksession refs only. Linked blocks reload via `doc.InstanceDefinitions[i].UpdateLinked()`.

## Common gotchas

- **Instances inherit the definition's units.** Place a meter-units linked block in a millimeter doc and you'll get a wall 1000x bigger than expected. Scale the InstanceXform to compensate, OR open the linked file's units to match.
- **Nested instances (blocks-in-blocks) require recursion.** `idef.GetObjects()` returns the direct children only. To unroll a deep hierarchy, recurse.
- **In-place definition edit invalidates downstream caches.** If your snippet edits a definition mid-loop, refresh any cached references.
- **Definition GUIDs are stable across saves; instance entity IDs are NOT.** Use the definition GUID for cross-session lookups; never the instance entity ID.

## When to use blocks vs groups

Use a **block** when geometry repeats: doors, windows, columns, equipment, families. Edit-once-update-everywhere is the win.

Use a **group** when geometry is unique but you want to drag it as one entity. Groups have no definition — they're purely UI-level bundling.

The AWARE curated `block.list-instances` verb has no group analog because there's no "definition" to audit instances of — groups ARE the entity.

## See also

- `_-BlockManager` (Rhino command) — interactive UI for the same operations
- The `block.list-instances` and `worksession.reload` curated verbs in `commands/`
