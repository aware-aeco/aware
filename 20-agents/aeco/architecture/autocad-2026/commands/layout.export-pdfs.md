# layout.export-pdfs

Export every paper-space layout to PDF via PlotEngine. The "issue the sheet pack" primitive.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `out-dir` | string | yes | — | Output directory; PDF filename derived from layout name |
| `pc3-name` | string | no | `"DWG to PDF.pc3"` | Plotter configuration |
| `paper-size-name` | string | yes | — | Paper size name as defined by the PC3 (e.g. `"ANSI_A_(8.50_x_11.00_Inches)"`) |
| `layout-name-filter` | string | no | `""` | Optional regex |

## Implementation (via aware-autocad exec)

```csharp
using Autodesk.AutoCAD.ApplicationServices;
using Autodesk.AutoCAD.DatabaseServices;
using Autodesk.AutoCAD.PlottingServices;
using System.IO;

var doc = Application.DocumentManager.MdiActiveDocument;
var db  = doc.Database;
var outDir       = args["out-dir"].ToString();
var pc3Name      = args.TryGetValue("pc3-name", out var p) ? p?.ToString() ?? "DWG to PDF.pc3" : "DWG to PDF.pc3";
var paperSize    = args["paper-size-name"].ToString();
var nameFilter   = args.TryGetValue("layout-name-filter", out var f) ? f?.ToString() ?? "" : "";
var rx           = string.IsNullOrEmpty(nameFilter) ? null
                   : new System.Text.RegularExpressions.Regex(nameFilter,
                       System.Text.RegularExpressions.RegexOptions.IgnoreCase);

Directory.CreateDirectory(outDir);

var written = new List<string>();
var layoutMgr = LayoutManager.Current;

if (PlotFactory.ProcessPlotState != ProcessPlotState.NotPlotting)
    return new { ok = false, error = "another plot is in progress" };

using (var docLock = doc.LockDocument())
{
    foreach (string layoutName in layoutMgr.LayoutsByName)
    {
        if (layoutName == "Model") continue;
        if (rx != null && !rx.IsMatch(layoutName)) continue;

        var safeName = System.Text.RegularExpressions.Regex.Replace(layoutName, @"[\\\/:\*\?""<>\|]+", "_");
        var pdfPath = Path.Combine(outDir, safeName + ".pdf");

        using (var t = db.TransactionManager.StartTransaction())
        {
            var layoutId = layoutMgr.GetLayoutId(layoutName);
            var layout = t.GetObject(layoutId, OpenMode.ForRead) as Layout;

            var plotInfo = new PlotInfo { Layout = layoutId };
            var plotSettings = new PlotSettings(layout.ModelType);
            plotSettings.CopyFrom(layout);

            var validator = PlotSettingsValidator.Current;
            validator.SetPlotConfigurationName(plotSettings, pc3Name, paperSize);
            validator.SetPlotPaperUnits(plotSettings, PlotPaperUnit.Inches);
            plotInfo.OverrideSettings = plotSettings;

            using (var plotEngine = PlotFactory.CreatePublishEngine())
            {
                var progress = new PlotProgressDialog(false, 1, false);
                plotEngine.BeginPlot(progress, null);
                plotEngine.BeginDocument(plotInfo, doc.Name, null, 1, true, pdfPath);

                var pageInfo = new PlotPageInfo();
                plotEngine.BeginPage(pageInfo, plotInfo, true, null);
                plotEngine.BeginGenerateGraphics(null);
                plotEngine.EndGenerateGraphics(null);
                plotEngine.EndPage(null);
                plotEngine.EndDocument(null);
                plotEngine.EndPlot(null);
            }
            written.Add(pdfPath);
            t.Commit();
        }
    }
}

return new {
    exported_count = written.Count,
    files          = written,
};
```

## See also

- [autocad-layouts-and-plotting](../skills/autocad-layouts-and-plotting.md)
