# Coverage review — grasshopper-8

**Verdict:** PASS
**Reviewer:** codex-rescue, run 2026-05-19T03:16:56Z
**IR source:** `cli-sidecar/Ingest/Output/grasshopper-8.0.ir.json` (sha256: `90b7be2ebec49c6b8033bbc4002afe9ec605d8fa77707b6ac94716aa3d96fdb8`)

## Step 1 — Type enumeration

- Vendor index: `https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/723c01da-9986-4db2-8f53-6f3a7494df75.htm`
- Namespaces walked: 36
- Vendor type count: 825
- Catalog type count: 825
- **missing_types_count:** 0 ✓

Sandcastle `T_*.htm` enumeration. The Grasshopper API root page is a
"Welcome / Development / Examples" landing — it does not carry a
`table#namespaceList`. Namespace anchors were taken from the sidebar TOC
links matching `N_*.htm`. For each namespace, parsed
`table.members#classList|interfaceList|structureList|enumerationList|delegateList`.

## Step 2 — Deep-check (50 random types)

- Seed: `280477230` (= int32 of first 8 hex of IR sha256)
- Sampled (50): `Grasshopper.CentralSettings.CreateSolutionUndoRecordChangedEventHandler, Grasshopper.Kernel.Types.GH_SubD.GH_SubDProxy, Grasshopper.Kernel.GH_BakeUtility, Grasshopper.Kernel.Types.GH_TextDot.GH_TextDotProxy, Grasshopper.Kernel.Geometry.SpatialTrees.Index3d<T>, Grasshopper.Kernel.GH_MemoryBitmap, Grasshopper.Rhinoceros.ModelUnitSystem.Value, Grasshopper.Kernel.GH_ComponentParamServer, Grasshopper.Rhinoceros.Drafting.ModelHatchLine, Grasshopper.Kernel.IGH_StateTag, Grasshopper.Kernel.GH_GHALoadingEventArgs, GH_IO.Serialization.GH_Chunk, Grasshopper.GUI.Canvas.GH_Canvas.CanvasPostPaintOverlayEventHandler, Grasshopper.Rhinoceros.Render.ModelRenderSkylight.Attributes, Grasshopper.Kernel.Types.GH_GooProxy<T>, Grasshopper.GUI.GH_TooltipDisplayEventArgs, Grasshopper.Kernel.Types.GH_String, Grasshopper.Kernel.Types.GH_Material.GH_Material_Proxy, Grasshopper.GUI.Gradient.GH_Gradient.GradientChangedEventHandler, Grasshopper.GUI.GH_MouseTracker, Grasshopper.Kernel.Types.Transforms.Generic, Grasshopper.Kernel.GH_DocSettingsEventArgs, Grasshopper.CentralSettings.PruderyLevelChangedEventHandler, Grasshopper.Kernel.IGH_DocumentObject, Grasshopper.Kernel.Types.GH_PointCloud.PointCloudProxy, Grasshopper.Kernel.IGH_Author, Grasshopper.Kernel.Undo.Actions.GH_HiddenAction, Grasshopper.Kernel.Attributes.GH_ResizableAttributes<T>, Grasshopper.Rhinoceros.Render.ModelRenderTexture.Attributes, Grasshopper.Kernel.Types.GH_Hatch, GH_IO.VersionNumber.Branch, Grasshopper.Rhinoceros.Display.DisplayColorGradient, Grasshopper.Kernel.Types.IGH_GeometricGoo, Grasshopper.Kernel.Parameters.Param_InstanceReference, Grasshopper.Kernel.Types.GH_Time, Grasshopper.Rhinoceros.Drafting.ModelHatchLine.Attributes, Grasshopper.Kernel.GH_RuntimeMessage, Grasshopper.Documentation.GH_GlossaryItem, Grasshopper.GUI.Canvas.GH_WireDirection, Grasshopper.Kernel.Types.GH_TextEntity, Grasshopper.Kernel.IGH_Component, Grasshopper.Kernel.GH_SettingsServer, Grasshopper.Kernel.Types.GH_Arc, Grasshopper.Kernel.Data.GH_LexerCombo, Grasshopper.GUI.Canvas.GH_CanvasNavigation, Grasshopper.Kernel.IGH_ValueProxy, Grasshopper.Kernel.IGH_ContextualParameter2, Grasshopper.Plugin.GH_PluginUtil, Grasshopper.Kernel.GH_DocumentSettings, Grasshopper.CentralSettings.AuthorCompanyChangedEventHandler`
- **deep_check_pass_rate:** 50/50 ✓

Each sampled type's vendor page parsed for constructors / methods (+ operators) /
properties (+ fields) / events / enum members. Catalog row counts and names
match vendor 100% across all 50 samples. Sandcastle structure is identical
to RhinoCommon's; the same parser logic applies.

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
- 825 catalog files validated against `cli-sidecar/Ingest/Schema/host-coverage-type.schema.json`
- Files validated: 826
- **schema_violations:** 0 ✓

## Re-run command

`aware coverage review grasshopper-8`
