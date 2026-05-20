---
name: autocad-layouts-and-plotting
description: This skill should be used when composing AutoCAD snippets that work with paper space — listing layouts, creating viewports, configuring page setups, batch-plotting to PDF/DWF/PLT, or any sheet-output workflow. Encodes the Model Space vs Paper Space distinction (a foundational AutoCAD concept), the Layout / PlotInfo / PageSetup hierarchy, the PlotEngine programmatic plotting API (the only path for non-interactive plot), and the gotcha that Layouts are physically BlockTableRecords with a 1:1 sibling Layout object.
---

# AutoCAD layouts and plotting

AutoCAD has TWO completely separate workspaces in every drawing: **Model Space** (where you draw at full scale) and **Paper Space** (where you compose sheets at sheet scale). Confusing them is one of the most common bugs in newcomer scripts.

## Model space vs paper space

| Space | What | API entry |
|---|---|---|
| **Model space** | The 1:1 scale drawing canvas | `bt[BlockTableRecord.ModelSpace]` |
| **Paper space** | One or more sheet layouts | `bt[BlockTableRecord.PaperSpace]` (initial) or a Layout's BlockTableRecord |
| **Layouts** | Named sheet compositions in paper space | `LayoutManager.Current.LayoutDictionary` |

A "Layout" is named, has a page setup, contains viewports that look back into model space. The default `*Model_Space` and `*Paper_Space` are special BTRs that AutoCAD always provides; user-created layouts add more.

## Listing layouts

```csharp
using Autodesk.AutoCAD.DatabaseServices;

var layoutMgr = LayoutManager.Current;
foreach (string layoutName in layoutMgr.LayoutsByName)
{
    Console.WriteLine(layoutName);   // includes "Model" and all paper-space sheets
}
```

Skip "Model" to get just sheet layouts. The current visible layout is `layoutMgr.CurrentLayout`.

## Creating a new layout

```csharp
using (var t = db.TransactionManager.StartTransaction())
{
    var layoutMgr = LayoutManager.Current;
    if (!layoutMgr.LayoutExists("A-101"))
    {
        var layoutId = layoutMgr.CreateLayout("A-101");
        var layout = t.GetObject(layoutId, OpenMode.ForWrite) as Layout;

        // Configure page setup
        layout.SetPlotSettings("ANSI_C_(22.00_x_17.00_Inches)", "DWG to PDF.pc3");
        // Or modify PlotInfo directly:
        var ps = new PlotSettings(true);
        ps.CopyFrom(layout);
        ps.SetPlotPaperUnits(PlotPaperUnit.Millimeters);
        ps.SetPlotConfigurationName("DWG to PDF.pc3", "ISO_full_bleed_A1_(841.00_x_594.00_MM)");
        layout.CopyFrom(ps);
    }
    t.Commit();
}
```

`PlotSettings` is the container for all page-setup info: plotter, paper size, area, scale, orientation, plot style table. It's a heavyweight object — use `SetPlotPaperUnits / SetPlotConfigurationName / SetPlotArea` etc. to configure.

## Adding viewports to a layout

```csharp
using (var t = db.TransactionManager.StartTransaction())
{
    var layout = t.GetObject(layoutId, OpenMode.ForRead) as Layout;
    var paperSpaceBtr = t.GetObject(layout.BlockTableRecordId, OpenMode.ForWrite) as BlockTableRecord;

    // Create a new viewport at the center of the layout
    var viewport = new Viewport();
    viewport.CenterPoint = new Point3d(100, 100, 0);   // mm, in paper space
    viewport.Width = 180;
    viewport.Height = 120;
    viewport.Layer = "0";

    paperSpaceBtr.AppendEntity(viewport);
    t.AddNewlyCreatedDBObject(viewport, true);

    // Activate the viewport so we can set its view
    viewport.On = true;          // critical — viewports start "off"
    viewport.CustomScale = 0.02; // 1:50 ratio
    viewport.ViewCenter = new Point2d(0, 0);  // model-space coord at viewport center

    t.Commit();
}
```

A viewport with `On = false` shows as a black rectangle in paper space. Always turn it on.

## Programmatic plotting

The `PlotEngine` is AutoCAD's non-interactive plot pipeline:

```csharp
using Autodesk.AutoCAD.PlottingServices;

using (var t = db.TransactionManager.StartTransaction())
{
    var layout = t.GetObject(layoutId, OpenMode.ForRead) as Layout;

    var plotInfo = new PlotInfo();
    plotInfo.Layout = layoutId;

    var plotSettings = new PlotSettings(layout.ModelType);
    plotSettings.CopyFrom(layout);
    plotInfo.OverrideSettings = plotSettings;

    var validator = PlotSettingsValidator.Current;
    validator.SetPlotConfigurationName(plotSettings, "DWG to PDF.pc3", "ISO_A1_(841.00_x_594.00_MM)");
    validator.SetPlotPaperUnits(plotSettings, PlotPaperUnit.Millimeters);

    var plotInfoValidator = new PlotInfoValidator();
    plotInfoValidator.Validate(plotInfo);

    if (!PlotFactory.ProcessPlotState.NotPlotting.Equals(PlotFactory.ProcessPlotState.NotPlotting))
        return new { ok = false, error = "another plot is in progress" };

    using (var plotEngine = PlotFactory.CreatePublishEngine())
    {
        var progressDialog = new PlotProgressDialog(false, 1, true);
        progressDialog.set_PlotMsgString(PlotMessageIndex.DialogTitle, "AWARE plot");
        plotEngine.BeginPlot(progressDialog, null);
        plotEngine.BeginDocument(plotInfo, doc.Name, null, 1, true, @"C:\out\A-101.pdf");

        var pageInfo = new PlotPageInfo();
        plotEngine.BeginPage(pageInfo, plotInfo, true, null);
        plotEngine.BeginGenerateGraphics(null);
        plotEngine.EndGenerateGraphics(null);
        plotEngine.EndPage(null);
        plotEngine.EndDocument(null);
        plotEngine.EndPlot(null);
    }
    t.Commit();
}
```

This is INVOLVED. For most batch PDF workflows, the `PUBLISH` command (via `SendStringToExecute`) is simpler:

```csharp
doc.SendStringToExecute("-PUBLISH\n...\n", true, false, false);
```

But `SendStringToExecute` is asynchronous — the script returns before plot completes. For synchronous AWARE workflows, use the PlotEngine API.

## PDF as the universal target

The "DWG to PDF.pc3" is shipped with AutoCAD and produces a PDF on disk. For DWF (legacy), use "DWF6 ePlot.pc3". For PLT (HPGL/2 to a plotter), use a configured plotter from `PageSetup`.

## Common gotchas

- **`viewport.On = false` means the viewport renders as a black rectangle.** Always `On = true` after creation.
- **`Layout.SetPlotSettings` requires both an existing PC3 file AND a paper size name** that matches what the PC3 defines. Typos = silent failure.
- **`PlotFactory.ProcessPlotState` must be checked before starting a plot.** Concurrent plots crash AutoCAD.
- **DWG to PDF.pc3 writes to the path specified in the PlotEngine.BeginDocument call**, NOT to the path in the layout's PlotSettings. The layout's path is only used by interactive plots.
- **Layouts are special BTRs.** Modifying a Layout (the metadata) vs modifying its BTR's entities (the paper-space objects) are different operations on different objects.

## Standard pattern (read layouts + their viewports)

```csharp
var doc = Application.DocumentManager.MdiActiveDocument;
var db  = doc.Database;

using (var t = db.TransactionManager.StartTransaction())
{
    var layoutMgr = LayoutManager.Current;
    var rows = new List<object>();

    foreach (string layoutName in layoutMgr.LayoutsByName)
    {
        if (layoutName == "Model") continue;
        var layoutId = layoutMgr.GetLayoutId(layoutName);
        var layout = t.GetObject(layoutId, OpenMode.ForRead) as Layout;
        var ps = t.GetObject(layout.BlockTableRecordId, OpenMode.ForRead) as BlockTableRecord;

        int vpCount = 0;
        foreach (ObjectId id in ps)
        {
            var v = t.GetObject(id, OpenMode.ForRead) as Entity;
            if (v is Viewport) vpCount++;
        }

        rows.Add(new {
            name = layoutName,
            tab_order = layout.TabOrder,
            viewport_count = vpCount,
            plot_config = layout.PlotConfigurationName,
            paper_size = layout.CanonicalMediaName,
        });
    }
    t.Commit();
    return new { layout_count = rows.Count, layouts = rows };
}
```

## See also

- [autocad-document-and-database](./autocad-document-and-database.md) — where layouts live
- [autocad-transactions-and-symbol-tables](./autocad-transactions-and-symbol-tables.md) — Layouts are managed in a SymbolTable
