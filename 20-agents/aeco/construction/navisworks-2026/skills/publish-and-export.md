---
name: navisworks-publish-and-export
description: This skill should be used when composing snippets that produce shareable outputs from Navisworks — publishing a .nwd federation, exporting viewpoints as images, generating clash reports (HTML/XML/Excel), exporting BCF, exporting DWG/DGN/PDF, or publishing to BIM 360 / ACC. Encodes the difference between Save / Publish / Export, the .nwd compression options, the cache file (.nwc) lifecycle, BCF version selection, and the gotcha that "Publish" embeds source files (full federation) whereas "Save As" preserves the .nwf reference graph.
---

# Navisworks publish and export

Navisworks distinguishes between three save/share verbs that look similar but have different semantics:

| Verb | Output | Embeds source? | Use when |
|---|---|---|---|
| **Save** (`.nwf`) | Reference graph: pointers to source files | NO — source files must exist at the original path | Working session; editing federation, BCF coordination |
| **Publish** (`.nwd`) | Self-contained federation | YES — geometry baked in | Issuing for review; archive; offline review |
| **Export** | One-off file in another format (DWG, DGN, FBX, IFC, etc.) | Format-specific | Interop with other tools |

For AWARE-orchestrated coordination workflows, the typical flow is:
1. Open coordinator .nwf with attached source files
2. Run clash detection / appearance profiles / etc.
3. **Publish** as .nwd for distribution
4. **Export** BCF for issue tracking
5. **Export** viewpoint images for the RFI/markup pack

## Publishing .nwd

```csharp
using Autodesk.Navisworks.Api;

var doc = Application.ActiveDocument;

var publishOptions = new PublishSettings {
    OutputPath = @"C:\Issued\Federation-2026-05-19.nwd",
    Password = "",                              // empty = no password
    DisplayAtPasswordPrompt = "",
    Title = "Coordinator Federation",
    Author = "AWARE",
    Subject = "Weekly coordination review",
    Keywords = "AWARE; coordination",
    Comments = "...",
    DisplayUnits = "Millimeters",
    EnableExpiry = false,
    ExpiryDate = DateTime.Now.AddDays(30),
    PreventReSave = false,
    Embed = true,                                 // embed source files
    EmbedTextures = true,
    PrioritizeMetadataOverGeometry = false,
};
doc.PublishFile(publishOptions);
```

`PublishSettings.EnableExpiry = true` + `ExpiryDate` creates a self-expiring .nwd — useful for time-limited reviews.

## Exporting viewpoint images

```csharp
var doc = Application.ActiveDocument;
foreach (var vpRef in doc.SavedViewpoints.RootItem.Children)
{
    var vp = doc.SavedViewpoints.ResolveReference(vpRef) as SavedViewpoint;
    if (vp == null) continue;

    doc.SavedViewpoints.CurrentSavedViewpoint = vpRef;
    doc.Refresh();    // pump UI to apply

    var path = Path.Combine(outDir, vp.DisplayName + ".png");
    doc.WriteImage(path, 1920, 1080, true);   // (path, width, height, transparent)
}
```

`WriteImage` blocks on the render — typically 1-5 seconds per high-res image on a complex federation.

## BCF export

```csharp
// (API name varies; typically via COM interop)
var bcfPath = @"C:\Issued\coordination-2026-05-19.bcfzip";
var bcfVersion = "2.1";    // or "3.0" if all downstream tools support it

// Export all viewpoints that have comments (typical filter for "real issues")
var withComments = doc.SavedViewpoints.RootItem.Children
    .Select(r => doc.SavedViewpoints.ResolveReference(r) as SavedViewpoint)
    .Where(vp => vp != null && vp.Comments.Any())
    .ToList();

// Issue command via the BCF Manager plugin
var bcfPlugin = doc.GetPlugins().FirstOrDefault(p => p.Name == "BCF Manager");
// ... call BCF export with bcfPath + bcfVersion + viewpoint list ...
```

BCF 2.1 vs 3.0: 2.1 has wider tool compatibility; 3.0 adds rich markup support. Default to 2.1 unless you control the consumption side.

## Clash report export

Clash Detective reports support HTML / XML / CSV / Excel:

```csharp
var clashDetective = doc.GetPlugins().FirstOrDefault(p => p.Name == "Clash Detective");
// ... walk down to the clash test ...

var test = clashDetective.GetTests().First();
var reportOptions = new ClashReportOptions {
    Format = ClashReportFormat.HTML,
    OutputPath = @"C:\Issued\clash-report.html",
    GroupBy = ClashGroupBy.None,
    DisplayInvalidClashes = false,
    DisplayResolvedClashes = false,
};
test.ExportReport(reportOptions);
```

HTML reports are self-contained (one file with embedded thumbnails). XML is the machine-readable interchange. Excel feeds into PMO workflows.

## DWG / DGN / FBX export

Navisworks can export federated geometry to several CAD formats:

```csharp
// DWG: export visible-and-selected geometry
doc.DXF(@"C:\Issued\federation.dwg", DXFOptions.Default);

// FBX: for visualization tools (Twinmotion, Enscape, Unreal)
doc.FBX(@"C:\Issued\federation.fbx", FBXOptions.Default);
```

DWG export tessellates geometry — the output is mesh-faceted, not parametric. Don't expect Revit elements to round-trip as Revit objects.

## The .nwc cache file

When Navisworks attaches a source file (Revit, IFC, DWG, etc.), it caches the geometry as a sibling .nwc file. This dramatically speeds up reload. Cache lifecycle:

- Auto-created on first attach
- Auto-refreshed when the source file's modification time exceeds the cache's
- Deleted when manually purged via UI
- NOT shared across users (each user has their own cache)

For AWARE workflows, the .nwc files should be ignored — they're build artifacts. The .nwf references the source files, not the .nwc.

## Common gotchas

- **`PublishFile` with `Embed = false` produces a "Save As .nwd" — geometry NOT embedded, source files still required.** Always `Embed = true` for distribution.
- **`PublishSettings.PreventReSave = true`** prevents the .nwd from being re-saved as .nwf in another Navisworks session. Useful for issued archives, but blocks downstream coordination workflows. Default to false unless explicitly archiving.
- **`Document.WriteImage` writes to the active viewport's CURRENT camera, not to a specified viewpoint.** Set the current viewpoint first, refresh, then write.
- **BCF export drops vendor-specific extensions.** Saved viewpoints with Navisworks-specific properties (e.g. clash IDs) lose them on BCF round-trip.
- **HTML clash reports embed thumbnails as base64 in the .html file** — they grow large (10 MB+) for high-clash-count tests.

## Standard pattern

```csharp
var doc = Application.ActiveDocument;
var outDir = args["out-dir"].ToString();
System.IO.Directory.CreateDirectory(outDir);

// 1. Publish .nwd federation
var nwdPath = System.IO.Path.Combine(outDir, "federation.nwd");
doc.PublishFile(new PublishSettings {
    OutputPath = nwdPath,
    Embed = true,
    EmbedTextures = true,
    Title = args["title"]?.ToString() ?? "AWARE-published federation",
});

// 2. Export viewpoint images
var imgDir = System.IO.Path.Combine(outDir, "viewpoints");
System.IO.Directory.CreateDirectory(imgDir);
// ... loop over saved viewpoints, WriteImage each ...

return new {
    nwd_path = nwdPath,
    viewpoint_images_dir = imgDir,
};
```

## See also

- [clash-detection-workflow](./clash-detection-workflow.md) — typically run BEFORE publish
- [bcf-roundtrip](./bcf-roundtrip.md) — for the BCF export details
- [viewpoints-and-markup](./viewpoints-and-markup.md) — for what gets imaged
