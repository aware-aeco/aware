---
name: rhino-meshes-vs-breps
description: This skill should be used when composing snippets that need to choose between Rhino's two solid representations — `Brep` (boundary representation, NURBS surfaces) versus `Mesh` (polygonal). Encodes when each is the right primitive, the conversion paths (Brep -> render-mesh, Mesh -> Brep via MeshToNurb), the cost / quality trade-offs, the role of mesh-repair (close naked edges, weld, cull) before STL/3MF/glTF export, and the AWARE curated `mesh.repair` verb's invariants. Critical for any export pipeline (3D-print, viz, GIS).
---

# Rhino meshes vs Breps

Rhino's geometry kernel maintains TWO solid representations: **NURBS-based BREPs** (boundary representations) and **polygonal Meshes**. Choose the wrong one and downstream tools (rendering, fabrication, export) either choke or produce visibly bad results.

## The two primitives

| Aspect | Brep | Mesh |
|---|---|---|
| Surface type | NURBS (smooth, parametric) | Triangular / quad facets |
| Memory | Compact; one Brep can describe a whole building | Heavy; scales linearly with facet count |
| Edit operations | Trim, split, offset, fillet are exact | Editing means re-meshing |
| Booleans | `Brep.CreateBooleanUnion / Difference / Intersection` — exact; can fail at tolerance | `Mesh.CreateBooleanUnion / Difference / Intersection` — robust but lossy |
| Export | STEP, IGES, native Rhino | STL, OBJ, PLY, 3MF, glTF |
| Rendering | Auto-generates a "render mesh" on demand | Already a mesh — renders directly |
| Volume / area | Exact (`AreaMassProperties` works precisely) | Approximate (depends on facet density) |

## When to use which

**Use Brep when:** Modeling architecture / mechanical with smooth surfaces; doing precise unions/differences; computing exact volumes; exchanging to other CAD tools via STEP/IGES.

**Use Mesh when:** Exporting to STL / 3MF for 3D print; exporting to OBJ / glTF for viz / web; doing dense scatter-plot or particle-system geometry; importing scanned point-cloud meshes.

**Both:** Most AECO work models in Breps and converts to Meshes only at export time. Keep the Brep as source-of-truth.

## Conversion paths

### Brep → Mesh (the common direction)

Two main APIs:

```csharp
// 1. The default render mesh (Rhino auto-generates one per object).
var renderMesh = brep.GetMesh(Rhino.Geometry.MeshType.Render);

// 2. Explicit meshing with parameters.
var meshParams = Rhino.Geometry.MeshingParameters.QualityRenderMesh;
// or: .FastRenderMesh, .DefaultAnalysisMesh, .CreateFromMeshDensity(0..1)
var meshes = Rhino.Geometry.Mesh.CreateFromBrep(brep, meshParams);
// Multiple meshes may be returned, one per Brep face.
```

For STL export, `MeshingParameters.QualityRenderMesh` is almost always what you want. Defaults to fairly dense; tweak `MaximumEdgeLength` if the result is too coarse / too heavy.

### Mesh → Brep (rare and lossy)

```csharp
// Best-effort NURBS reconstruction. Slow on large meshes.
var brep = Rhino.Geometry.Mesh.MeshToBrep(...); // method depends on Rhino version
```

This is approximation: each mesh face becomes a planar Brep face. Curved areas need a re-NURBS pass via Rhino's `_-MeshToNURB` command (no clean API equivalent in RhinoCommon).

If you find yourself converting Mesh → Brep in production code, you're usually solving the wrong problem. Try keeping the Brep source longer.

## Pre-export sanity (the `mesh.repair` story)

Meshes for fabrication MUST be watertight. The AWARE curated `mesh.repair` verb closes naked edges, welds vertices within a tolerance, and culls degenerate faces. The order matters:

1. **Cull degenerate faces** first — removes zero-area faces that confuse subsequent ops
2. **Weld vertices** — merges coincident vertices within the angle tolerance
3. **Fill holes / heal naked edges** — closes gaps left by cull+weld
4. **Re-check NakedEdges** — count should be 0 for a fully closed mesh

Naked edges are the failure mode for STL/3MF export — slicers reject them or produce broken parts. Always re-check after repair.

## Render mesh vs explicit mesh

`brep.GetMesh(MeshType.Render)` retrieves the AUTO-GENERATED render mesh that Rhino computed for display. It may be coarse (depends on the user's display settings). For export, regenerate explicitly via `Mesh.CreateFromBrep` with quality parameters — don't trust the render mesh blindly.

## Common patterns

```csharp
// "Add every BREP to a render queue, exporting cached meshes for speed"
foreach (var obj in doc.Objects)
{
    var brep = obj.Geometry as Rhino.Geometry.Brep;
    if (brep == null) continue;
    var m = brep.GetMesh(Rhino.Geometry.MeshType.Render);
    if (m != null) { /* enqueue m */ }
}

// "Repair selected meshes before STL export"
foreach (var obj in doc.Objects.GetSelectedObjects(false, false))
{
    var mesh = obj.Geometry as Rhino.Geometry.Mesh;
    if (mesh == null) continue;
    mesh.Faces.CullDegenerateFaces();
    mesh.Vertices.CullUnused();
    mesh.Weld(Math.PI / 6.0);   // 30 degrees
    mesh.FillHoles();
    doc.Objects.Replace(obj.Id, mesh);
}
```

## Gotchas

- `Brep.IsValid` returns false for a Brep that has any micro-tolerance edge issue. Run `Brep.Repair(tol)` before trusting a Brep returned from booleans.
- `Mesh.IsClosed` returns false on any naked edge. Don't STL-export until this is true.
- Casting `obj.Geometry as Mesh` returns null for Breps with render meshes — fetch via `obj.GetMeshes(MeshType.Render)` instead if you want the auto-generated polygonal representation of a Brep object.
- Render-mesh density inherits from the **object**, not the doc. Override per-object via `obj.Attributes.MeshingParameters`.

## See also

- `_-MeshRepair` (Rhino command) — interactive equivalent with visual diagnostics
- The `mesh.repair` curated verb in `commands/`
