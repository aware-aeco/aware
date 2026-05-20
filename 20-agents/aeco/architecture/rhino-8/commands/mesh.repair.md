# mesh.repair

Close naked edges, weld vertices, and remove duplicate faces on the selected meshes. The pre-export sanity pass before STL/3MF.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `weld-angle-degrees` | number | no | 0 | 0 disables welding. Common value: 30. |
| `close-naked-edges` | boolean | no | true | Close gaps where mesh edges have no neighbour face. |
| `cull-degenerate-faces` | boolean | no | true | Remove zero-area faces. |

`mode: write` — declares safety in the composing app.

Operates on the currently-selected mesh objects. Returns one result row per processed mesh.

## Outputs

```json
{
  "processed-count": 3,
  "results": [
    { "id": "abc...", "naked-edges-closed": 12, "faces-removed": 0, "vertices-welded": 4 },
    ...
  ]
}
```

## Implementation (shipped through `aware-rhino exec`)

```csharp
var doc = Rhino.RhinoDoc.ActiveDoc;
double weldDeg = Convert.ToDouble(args.TryGetValue("weld-angle-degrees", out var w) ? w : 0);
bool closeNaked = !args.TryGetValue("close-naked-edges", out var c) || Convert.ToBoolean(c);
bool cullDegen  = !args.TryGetValue("cull-degenerate-faces", out var d) || Convert.ToBoolean(d);

var rows = new List<object>();
foreach (var obj in doc.Objects.GetSelectedObjects(false, false))
{
    var mesh = obj.Geometry as Rhino.Geometry.Mesh;
    if (mesh == null) continue;

    int nakedBefore = mesh.GetNakedEdges()?.Length ?? 0;
    int facesBefore = mesh.Faces.Count;
    int vertsBefore = mesh.Vertices.Count;

    // Cull bad geometry first so subsequent ops have less to work on.
    if (cullDegen)
    {
        mesh.Faces.CullDegenerateFaces();
        mesh.Vertices.CullUnused();
    }
    // Weld within angle tolerance.
    if (weldDeg > 0)
    {
        mesh.Weld(weldDeg * Math.PI / 180.0);
    }
    // Close any gaps left after weld+cull.
    if (closeNaked)
    {
        var tol = doc.ModelAbsoluteTolerance > 0 ? doc.ModelAbsoluteTolerance : 0.001;
        mesh.FillHoles();
        mesh.HealNakedEdges(tol);
    }
    // Persist back to the document.
    doc.Objects.Replace(obj.Id, mesh);

    int nakedAfter = mesh.GetNakedEdges()?.Length ?? 0;
    int facesAfter = mesh.Faces.Count;
    int vertsAfter = mesh.Vertices.Count;

    rows.Add(new {
        id                  = obj.Id.ToString(),
        naked_edges_closed  = Math.Max(0, nakedBefore - nakedAfter),
        faces_removed       = Math.Max(0, facesBefore - facesAfter),
        vertices_welded     = Math.Max(0, vertsBefore - vertsAfter),
    });
}
doc.Views.Redraw();
return new { ok = true, processed_count = rows.Count, results = rows };
```

## Gotchas

- `HealNakedEdges` uses model tolerance; if your model tolerance is too loose, repair will collapse intentional detail.
- `Weld` is destructive — sharp creases vanish if the weld angle exceeds the crease angle.
- Operates only on top-level selected meshes. Meshes inside blocks are not touched.
- `Mesh.FillHoles()` may create overlapping faces on complex non-planar holes. Re-run with `cull-degenerate-faces: true` to clean up.

## See also

- `_-MeshRepair` (Rhino command) — interactive equivalent with diagnostic panel
- `_-Export` STL — typical downstream step after this verb
