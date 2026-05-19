# Coverage review — rhino-8

**Verdict:** PASS
**Reviewer:** codex-rescue, run 2026-05-19T03:16:56Z
**IR source:** `cli-sidecar/Ingest/Output/rhino-8.0.ir.json` (sha256: `658b61b04fbacf0292f6ac62f57f861cd1682270dcafce3408e569a92d722b8d`)

## Step 1 — Type enumeration

- Vendor index: `https://raw.githubusercontent.com/mcneel/rhinocommon-api-docs/gh-pages/api/RhinoCommon/html/R_Project_RhinoCommon.htm`
- Namespaces walked: 38
- Vendor type count: 1399
- Catalog type count: 1399
- **missing_types_count:** 0 ✓

Sandcastle `T_*.htm` enumeration. Namespace anchors taken from the sidebar
TOC and the root page's `table#namespaceList`. For each namespace, parsed
`table.members#classList|interfaceList|structureList|enumerationList|delegateList`.
External members inherited from `System.Object` (href → `docs.microsoft.com`)
are not vendor types and were not expected in the catalog.

## Step 2 — Deep-check (50 random types)

- Seed: `1703633328` (= int32 of first 8 hex of IR sha256)
- Sampled (50): `Rhino.Render.PostEffects.PostEffectType, Rhino.Geometry.Dimension.ForceText, Rhino.Render.RenderContentStyles, Rhino.FileIO.File3dmShutLining, Rhino.Geometry.CurveProxy, Rhino.Runtime.RhinoAccounts.RhinoAccountsServerNotReachableException, Rhino.FileIO.FileDwg, Rhino.Runtime.InteropWrappers.StringHolder, Rhino.Display.ZBufferCapture, Rhino.Render.Fields.DoubleField, Rhino.FileIO.File3dmObject, Rhino.FileIO.FileReferenceStatus, Rhino.Geometry.CurveEvaluationSide, Rhino.DocObjects.RadialDimensionObject, Rhino.Display.DisplayPipelineAttributes.LinearWorkflowUsages, Rhino.Render.RenderMaterial.PreviewBackgroundType, Rhino.Geometry.Intersect.Intersection, Rhino.Runtime.RhinoAccounts.IRhinoAccountsManager, Rhino.Render.RenderContentSerializer, Rhino.UI.ToolbarGroup, Rhino.UI.Dialogs, Rhino.RhinoDoc.TextureMappingEventType, Rhino.Render.PostEffects.PostEffectJob, Rhino.Geometry.ComponentIndexType, Rhino.DocObjects.Linetype, Rhino.FileIO.FileDwgWriteOptions.AutocadVersion, Rhino.Render.PixelBuffer, Rhino.Geometry.SubDPatchStyle, Rhino.UI.Controls.ICollapsibleSection, Rhino.Render.TextureGraphInfo, Rhino.Geometry.SubD.NurbsSurfaceType, Rhino.Render.RenderWindow, Rhino.DocObjects.Tables.LightTableEventArgs, Rhino.DocObjects.ObjectMode, Rhino.Render.Fields.StringField, Rhino.Runtime.InteropWrappers.SimpleArrayGuid, Rhino.Render.RenderContentChangedEventArgs, Rhino.DocObjects.RhinoObject.ObjectFrameFlags, Rhino.Display.DisplayPipelineAttributes.WorldAxesIconColorUse, Rhino.Render.Utilities.PreviewQuality, Rhino.ApplicationSettings.ShortcutKeySettings, Rhino.Input.Custom.GetObjectGeometryFilter, Rhino.Render.RdkUndoRecord, Rhino.Runtime.InteropWrappers.SimpleArrayPlane, Rhino.DocumentEventArgs, Rhino.Geometry.Collections.BrepSurfaceList, Rhino.DocObjects.ObjectLinetypeSource, Rhino.Runtime.ViewCaptureWriter.PathPoint, Rhino.Input.Custom.GetString, Rhino.Display.InitFrameBufferEventArgs`
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
- 1399 catalog files validated against `cli-sidecar/Ingest/Schema/host-coverage-type.schema.json`
- Files validated: 1400
- **schema_violations:** 0 ✓

## Re-run command

`aware coverage review rhino-8`
