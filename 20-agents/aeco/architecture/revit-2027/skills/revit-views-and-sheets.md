---
name: revit-views-and-sheets
description: This skill should be used when composing snippets that touch Revit's view system — listing views, creating sheets, placing viewports on sheets, exporting drawings to PDF/DWG, capturing view images, or managing view templates. Encodes the View / ViewSheet / Viewport hierarchy, the difference between view-types (plan, elevation, section, 3D), the `ViewSheet.Create(doc, titleblockTypeId)` + `Viewport.Create(doc, sheetId, viewId, location)` pattern, the gotcha that exporting requires a non-detached document open in the UI, and AWARE's curated `sheet.list` / `sheet.export-pdfs` workflow.
---

# Revit views and sheets

The view system in Revit has more nuance than most newcomers expect. A "drawing" in the printed sense is a `ViewSheet` containing one or more `Viewport`s that reference views. Each component has its own lifecycle.

## The view hierarchy

```
View (abstract base class)
├── ViewPlan          — floor plans, ceiling plans, area plans
├── ViewSection       — building sections, callouts
├── ViewDrafting      — 2D drafting views (no model reference)
├── View3D            — perspective + orthographic 3D views
├── ViewSheet         — sheets (the things that print)
├── ViewSchedule      — schedules (tables)
└── ... (a couple more obscure types)
```

`ViewSheet` is special — it's a view that contains other views (via Viewports), plus a titleblock family instance, plus annotations/legends. Sheets are what you EXPORT to PDF.

## Listing views and sheets

```csharp
// All views (excluding templates)
var views = new FilteredElementCollector(doc)
    .OfClass(typeof(View))
    .Cast<View>()
    .Where(v => !v.IsTemplate)
    .ToList();

// All sheets
var sheets = new FilteredElementCollector(doc)
    .OfClass(typeof(ViewSheet))
    .Cast<ViewSheet>()
    .ToList();

// Sheets with their viewports
foreach (var sheet in sheets) {
    foreach (var vpId in sheet.GetAllViewports()) {
        var vp = doc.GetElement(vpId) as Viewport;
        var view = doc.GetElement(vp.ViewId) as View;
        // ...
    }
}
```

## Creating a sheet

```csharp
using (var t = new Transaction(doc, "AWARE: create-sheet"))
{
    t.Start();

    // Find a titleblock symbol
    var titleblock = new FilteredElementCollector(doc)
        .OfClass(typeof(FamilySymbol))
        .OfCategory(BuiltInCategory.OST_TitleBlocks)
        .Cast<FamilySymbol>()
        .First();

    if (!titleblock.IsActive) titleblock.Activate();
    doc.Regenerate();

    var sheet = ViewSheet.Create(doc, titleblock.Id);
    sheet.SheetNumber = "A-101";
    sheet.Name = "Ground Floor Plan";

    // Place a viewport on the sheet
    var planView = ... ;   // an existing ViewPlan
    var sheetCenter = new XYZ(0, 0, 0);    // model units (feet, internal)
    var viewport = Viewport.Create(doc, sheet.Id, planView.Id, sheetCenter);

    t.Commit();
}
```

## Sheet identity

| Field | Type | Notes |
|---|---|---|
| `sheet.SheetNumber` | string | Primary identifier; users see this on the sheet itself ("A-101") |
| `sheet.Name` | string | The descriptive title ("Ground Floor Plan") |
| `sheet.UniqueId` | string | Cross-session stable GUID — use for cross-session lookups |
| `sheet.Id` | ElementId | Session-local int id |

Sort sheets by SheetNumber for issue-pack consistency. SheetNumber is NOT auto-numeric — it's an arbitrary string ("A-101", "S-2.04", "MEP-09b").

## Sheet parameters

Sheets carry the issue-management metadata:

```csharp
// Revision letter (current)
var currentRevision = sheet.GetCurrentRevision();     // returns ElementId of latest Revision
var revisionObj     = doc.GetElement(currentRevision) as Revision;
var revLetter       = revisionObj?.RevisionNumber;    // e.g. "C"

// Issue-related built-in parameters
var issueDateParam   = sheet.get_Parameter(BuiltInParameter.SHEET_ISSUE_DATE);
var issuedForParam   = sheet.get_Parameter(BuiltInParameter.SHEET_ISSUED_FOR);
var drawnByParam     = sheet.get_Parameter(BuiltInParameter.SHEET_DRAWN_BY);
var checkedByParam   = sheet.get_Parameter(BuiltInParameter.SHEET_CHECKED_BY);
var disciplineParam  = sheet.get_Parameter(BuiltInParameter.SHEET_DISCIPLINE);
```

These power the curated `sheet.list` verb's filter fields.

## View templates

Many views inherit settings from a `ViewTemplate`. When changing visibility/graphics, edit the template — not the view itself — so changes apply consistently:

```csharp
var view = ... ;
var templateId = view.ViewTemplateId;
if (templateId != ElementId.InvalidElementId) {
    var template = doc.GetElement(templateId) as View;
    // Modify template properties; all views using it inherit.
}
```

A view with `templateId == ElementId.InvalidElementId` has no template and edits apply only to itself.

## Exporting to PDF

```csharp
using (var t = new Transaction(doc, "AWARE: export-pdf"))
{
    t.Start();

    var sheetIds = new List<ElementId> { sheet.Id };   // can be multiple
    var options = new PDFExportOptions {
        FileName = "Issue-Pack",         // base filename (Revit appends .pdf)
        StopOnError = false,
        Combine = true,                   // single multi-page PDF
        PaperFormat = ExportPaperFormat.Default,
        Quality = PDFExportQualityType.DPI300,
    };

    doc.Export(@"C:\Issue\2026-05-19", sheetIds, options);

    t.Commit();
}
```

`PDFExportOptions` requires Revit 2022+. Earlier versions used `PrintManager` with a virtual PDF printer — much messier.

**Gotcha:** `doc.Export` REQUIRES the document to be open and undamaged. Detached documents (those opened via `OpenOptions.DetachAndPreserveWorksets`) cannot export.

## Exporting view to image

```csharp
var view = doc.ActiveView;
var imgOptions = new ImageExportOptions {
    FilePath = @"C:\out\view.png",
    ExportRange = ExportRange.SetOfViews,
    ImageResolution = ImageResolution.DPI_300,
    PixelSize = 1920,
    ZoomType = ZoomFitType.FitToPage,
    HLRandWFViewsFileType = ImageFileType.PNG,
};
imgOptions.SetViewsAndSheets(new List<ElementId> { view.Id });
doc.ExportImage(imgOptions);
```

No transaction required for export — it's a read-only operation against the document.

## Common gotchas

- **`ViewSheet.Create` REQUIRES a titleblock.** Use `FamilySymbol`s of `OST_TitleBlocks`; if none exist, load one via `doc.LoadFamily` first.
- **`Viewport.Create` PLACEMENT center** is in MODEL units (feet, internal), not paper-space coordinates. To place at sheet origin, use `XYZ.Zero`.
- **A view can be placed on AT MOST one sheet** — Revit enforces this. Reuse via "Duplicate" / "Duplicate with Detailing" to get multiple placements.
- **Sheet exports run in BATCH mode** through `PDFExportOptions.Combine = true`. Single sheet at a time: `Combine = false` and one ID at a time.
- **View templates lock fields they manage.** Trying to set a locked field on a view directly returns false (Set returns false; no exception).

## Standard pattern in `aware-revit exec` snippets

```csharp
var doc = uiapp.ActiveUIDocument.Document;
var filterIssuedFor = args.TryGetValue("filter-issued-for", out var f) ? f?.ToString() : null;

var sheets = new FilteredElementCollector(doc)
    .OfClass(typeof(ViewSheet))
    .Cast<ViewSheet>()
    .Where(s => string.IsNullOrEmpty(filterIssuedFor)
              || (s.get_Parameter(BuiltInParameter.SHEET_ISSUED_FOR)?.AsString() ?? "")
                   .Equals(filterIssuedFor, StringComparison.OrdinalIgnoreCase))
    .OrderBy(s => s.SheetNumber)
    .Select(s => new {
        sheet_number    = s.SheetNumber,
        sheet_name      = s.Name,
        current_revision = (doc.GetElement(s.GetCurrentRevision()) as Revision)?.RevisionNumber ?? "",
        issue_date      = s.get_Parameter(BuiltInParameter.SHEET_ISSUE_DATE)?.AsString() ?? "",
        issued_for      = s.get_Parameter(BuiltInParameter.SHEET_ISSUED_FOR)?.AsString() ?? "",
    })
    .ToList();

return new { count = sheets.Count, sheets = sheets };
```

## See also

- [`sheet.list` curated verb](../commands/sheet.list.md)
- [`sheet.export-pdfs` curated verb](../commands/sheet.export-pdfs.md)
- [revit-element-collector](./revit-element-collector.md) — generalises the FilteredElementCollector
