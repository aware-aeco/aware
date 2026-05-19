# Coverage review — rhino-7

**Verdict:** PASS
**Reviewer:** codex-rescue, run 2026-05-19T03:16:56Z
**IR source:** `cli-sidecar/Ingest/Output/rhino-7.0.ir.json` (sha256: `1edadfdbc781887698a34740b756ec850fe84aea296ede1023f161caa0eef68d`)

## Step 1 — Type enumeration

- Vendor index: `https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/7/html/R_Project_RhinoCommon.htm`
- Namespaces walked: 37
- Vendor type count: 1154
- Catalog type count: 1154
- **missing_types_count:** 0 ✓

Sandcastle `T_*.htm` enumeration. Namespace anchors taken from the sidebar
TOC and the root page's `table#namespaceList`. For each namespace, parsed
`table.members#classList|interfaceList|structureList|enumerationList|delegateList`.
External members inherited from `System.Object` (href → `docs.microsoft.com`)
are not vendor types and were not expected in the catalog.

## Step 2 — Deep-check (50 random types)

- Seed: `517660635` (= int32 of first 8 hex of IR sha256)
- Sampled (50): `Rhino.Input.Custom.TaskCompleteEventArgs, Rhino.Geometry.HermiteSurface, Rhino.Render.CustomRenderContentAttribute, Rhino.Input.Custom.ConeConstraint, Rhino.Geometry.Light, Rhino.Geometry.Mesh, Rhino.Geometry.Intersect.LineCircleIntersection, Rhino.PlugIns.RenderPlugIn, Rhino.PlugIns.ValidateResult, Rhino.UI.Controls.ThumbnailUI.IRhRdkThumbnailList_Shapes, Rhino.UI.ShowMessageMode, Rhino.Runtime.InteropWrappers.SimpleArrayClippingPlaneObjectPointer, Rhino.Render.RenderContent.ChangeContexts, Rhino.FileIO.File3dmDimStyleTable, Rhino.Render.ChangeQueue.Material, Rhino.Runtime.RhinoAccounts.RhinoAccountsCannotListenException, Rhino.UI.RhinoPlotWidthValue, Rhino.Commands.CommandEventArgs, Rhino.DocObjects.Custom.CustomObjectGrips, Rhino.Geometry.LinearDimension, Rhino.Runtime.InteropWrappers.SimpleArrayInterval, Rhino.FileIO.FilePlyWriteOptions, Rhino.Geometry.Point4d, Rhino.Render.RenderContentStyles, Rhino.Collections.CurveList, Rhino.Geometry.MeshExtruder, Rhino.DocObjects.ViewInfo, Rhino.Geometry.ComponentIndexType, Rhino.Runtime.InteropWrappers.SimpleArrayInt, Rhino.Render.TextureEnvironmentMappingMode, Rhino.Geometry.Light.Attenuation, Rhino.UI.Gumball.GumballPickResult, Rhino.Geometry.Intersect.CircleCircleIntersection, Rhino.Render.DataSources.ContentFactory, Rhino.Render.ContentUndoHelper, Rhino.DocObjects.CurveObject, Rhino.Render.RenderMaterial.StandardChildSlots, Rhino.DocObjects.Tables.ObjectTable, Rhino.Render.RenderContent, Rhino.Geometry.Collections.MeshFaceNormalList, Rhino.Geometry.SubD, Rhino.DocObjects.ModelComponent, Rhino.Render.TextureGraphInfo, Rhino.Display.Text3d, Rhino.Geometry.Arrowhead, Rhino.PlugIns.LicenseStatus, Rhino.DocObjects.Tables.GroupTable, Rhino.UI.ModifierKey, Rhino.Render.SimulatedTexture, Rhino.Render.RenderPipeline`
- **deep_check_pass_rate:** 50/50 ✓

Each sampled type's vendor page parsed for constructors / methods (+ operators) /
properties (+ fields) / events / enum members. Catalog row counts and names
match vendor 100% across all 50 samples. Operators folded into methods and
fields folded into properties per `EXTRACTION-NOTES.md`. The LST C# variant
substitution (`[?|]cs=([^|"]*)`) is honored.

## Step 3 — Behavioral spot-check (20 random methods)

- Continued seeded RNG (no re-seed) — 20 methods drawn across the 50 sampled
  types from the catalog's `methods[]` arrays.
- **behavior_present_rate:** 20/20 ✓

Each sampled method's vendor `doc_url` was fetched. Catalog `summary` /
`remarks` non-empty whenever vendor page had real prose. Sandcastle
`[Missing <summary>…]` / `[Missing <remarks>…]` placeholders count as
"vendor has no prose" (not a defect).

## Step 4 — Schema validation

- IR validated against `cli-sidecar/Ingest/Schema/host-coverage.schema.json`
- 1154 catalog files validated against `cli-sidecar/Ingest/Schema/host-coverage-type.schema.json`
- Files validated: 1155
- **schema_violations:** 0 ✓

## Re-run command

`aware coverage review rhino-7`
