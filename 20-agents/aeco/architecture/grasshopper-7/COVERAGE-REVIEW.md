# Coverage review — grasshopper-7

**Verdict:** PASS
**Reviewer:** codex-rescue, run 2026-05-19T03:16:56Z
**IR source:** `cli-sidecar/Ingest/Output/grasshopper-7.0.ir.json` (sha256: `3d9b1a70efb8453887ffed4599dcf7d36c2a7dcd85c18d22a919f45d8ac7d053`)

## Step 1 — Type enumeration

- Vendor index: `https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/7/api/grasshopper/html/723c01da-9986-4db2-8f53-6f3a7494df75.htm`
- Namespaces walked: 23
- Vendor type count: 650
- Catalog type count: 650
- **missing_types_count:** 0 ✓

Sandcastle `T_*.htm` enumeration. The Grasshopper API root page is a
"Welcome / Development / Examples" landing — it does not carry a
`table#namespaceList`. Namespace anchors were taken from the sidebar TOC
links matching `N_*.htm`. For each namespace, parsed
`table.members#classList|interfaceList|structureList|enumerationList|delegateList`.

## Step 2 — Deep-check (50 random types)

- Seed: `1033575024` (= int32 of first 8 hex of IR sha256)
- Sampled (50): `Grasshopper.Kernel.GH_RelevantObjectFilter, Grasshopper.Kernel.IGH_VariableParameterComponent, GH_IO.Serialization.GH_Compression, Grasshopper.Kernel.GH_PreviewMode, Grasshopper.GUI.GH_VerticalScrollBar.ScrollRatioChangedEventHandler, Grasshopper.Kernel.Types.GH_Box, GH_IO.Types.GH_Item, Grasshopper.GUI.GH_MouseTracker, Grasshopper.Kernel.GH_ComponentServer.GHAFileLoadedEventHandler, Grasshopper.Kernel.GH_SolutionEventArgs, GH_IO.UserInterface.GH_DeveloperDetails, Grasshopper.Kernel.GH_Component, Grasshopper.Kernel.GH_BitmapChannel, Grasshopper.Kernel.Types.GH_Point.GH_PointProxy, Grasshopper.GUI.GH_Tooltip, Grasshopper.Kernel.Data.GH_RuleComplex, Grasshopper.Kernel.Attributes.GH_ResizableAttributes<T>, Grasshopper.Kernel.GH_LibraryLicense, Grasshopper.CentralSettings.PreviewPointStyleChangedEventHandler, Grasshopper.GUI.Canvas.GH_CanvasWidgetListEventArgs, Grasshopper.GUI.GH_PanelEditorControl, Grasshopper.Kernel.GH_State, Grasshopper.Kernel.Undo.GH_ArchivedUndoAction, Grasshopper.GUI.GH_FadePhase, Grasshopper.CentralSettings.PruderyLevelChangedEventHandler, Grasshopper.Kernel.GH_Param<T>, Grasshopper.Kernel.GH_Attributes<T>, Grasshopper.Kernel.GH_Document.ModifiedChangedEventHandler, Grasshopper.Kernel.IGH_DocumentObject.PingDocumentEventHandler, Grasshopper.GUI.GH_WindowsFormUtil, Grasshopper.Kernel.GH_SettingsType, Grasshopper.CentralSettings.CanvasFancyWiresChangedEventHandler, Grasshopper.Kernel.GH_DocumentAssociations, Grasshopper.GUI.Theme.GH_WireSettings, Grasshopper.GUI.Canvas.GH_Canvas.CanvasPaintEndEventHandler, Grasshopper.Kernel.GH_Arrange, Grasshopper.Kernel.IGH_InstanceDescription, Grasshopper.Kernel.GH_FileWatcher.FileChangedSimple, Grasshopper.Kernel.Types.GH_Culture, Grasshopper.Kernel.GH_AutoSaveSettings.SaveOnObjectAddedChangedEventHandler, Grasshopper.Kernel.GH_Document.EnableSolutionsChangedEventHandler, Grasshopper.Kernel.IGH_ValueProxy, Grasshopper.GUI.GH_GDI_Util, Grasshopper.Kernel.Types.GH_Interval, Grasshopper.Kernel.Expressions.GH_CharType, Grasshopper.GUI.IGH_FileDropTarget, Grasshopper.GUI.Canvas.TagArtists.GH_TagArtist_WirePainter, Grasshopper.Kernel.Expressions.GH_ParserPrecedence, Grasshopper.Kernel.IGH_ObjectProxy, Grasshopper.Kernel.Expressions.GH_Variant`
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
- 650 catalog files validated against `cli-sidecar/Ingest/Schema/host-coverage-type.schema.json`
- Files validated: 651
- **schema_violations:** 0 ✓

## Re-run command

`aware coverage review grasshopper-7`
