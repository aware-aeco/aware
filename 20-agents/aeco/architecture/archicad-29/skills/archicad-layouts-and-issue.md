---
name: archicad-layouts-and-issue
description: This skill should be used when composing Archicad automation that produces deliverables — PDF sheets, DWG batches, BIMx packages, IFC exports, or any other non-interactive output. Encodes Archicad's Layout Book structure (Subset → Layout → Drawing), the Publisher Sets model (the sanctioned issuance unit), the difference between View and Layout, layout revisioning, the relationship between Drawings on a layout and the Views they reference, and why `PublishPublisherSet` is the only headless path to a PDF sheet pack.
---

# Archicad layouts and issue workflow

When the architect "issues drawings for tender," what actually happens in Archicad:

```
Project
├── View Map                    (saved views of the model — plans, sections, 3Ds)
│   └── View "GA Plan – Level 02 – 1:50"   (a captured state: which combination of layers, scale, settings)
├── Layout Book                 (paper sheets the views get placed on)
│   └── Subset "100-series – General Arrangement"
│       └── Layout "A101 – Level 02 GA Plan"
│           └── Drawing  (placed reference to the View, with its own title block / scale override)
└── Publisher Sets              (output recipes — "this sheet pack as PDFs in this folder")
    └── PublisherSet "Issue – Stage 4 PDF"
        └── PublisherSetItem  (one item per output file)
            ├── points at  Layout "A101 – Level 02 GA Plan"
            └── format: PDF, output path: /Issue/Stage4/
```

Five concepts. They look similar but have distinct roles, and getting them mixed up is a common source of bugs.

## View vs Layout vs Drawing — the three-step distinction

| Concept | What it is | Lives in | Example |
|---|---|---|---|
| **View** | A saved configuration of the model display: which view (plan / section / 3D), which layer combination, which scale, which model-view-options, which dimension settings | View Map | "GA Plan – Level 02 – 1:50, Color Layer Combo" |
| **Layout** | A paper sheet with a title block and zero-or-more Drawings | Layout Book | "A101 – Level 02 GA Plan" — has the firm's title block and one big plan view |
| **Drawing** | A placed reference to a View on a Layout, with optional title block override and scale | The Layout | The placement of "GA Plan – Level 02" view on layout A101 |

A View is the model state. A Layout is the paper. A Drawing is "this View, on that Layout."

## The Layout Book structure

```
Layout Book (single per project)
├── Master Layouts          (title-block / template sheets the Layouts inherit from)
│   ├── Master "A0 – Generic"
│   └── Master "A1 – Generic"
├── Subset "100-series"     (folder grouping — usually by discipline / stage)
│   ├── Layout "A101"
│   ├── Layout "A102"
│   └── Layout "A103"
├── Subset "200-series – Sections"
│   ├── Layout "A201"
│   └── Layout "A202"
└── Subset "500-series – Details"
    └── …
```

Tapir's `NavigatorCommands.CreateLayouts` / `CreateSubsets` create these programmatically. The official `LayoutBookCommands.CreateLayout` / `CreateLayoutSubset` do the same job from the Graphisoft side.

## Publisher Sets — the actual output recipe

Archicad has **two** related concepts that sound the same:

- **Layout Book** = the sheets exist. They render in Archicad's Navigator and can be printed individually.
- **Publisher Set** = a batch recipe that says "publish these layouts (and/or these views, and/or these BIMx exports) to PDF/DWG/BIMx/etc. into this directory with this naming convention."

A project usually has multiple Publisher Sets:
- "Working – PDF preview" — fast draft for internal review
- "Issue – Stage 4 PDF" — final tender pack at high DPI
- "Issue – Stage 4 DWG" — same layouts as DWG for consultants
- "Model – BIMx Hyper-Model" — packaged BIMx for client
- "Model – IFC4 Coordination" — IFC export for clash detection

Headless automation triggers a Publisher Set, never a Layout directly:

```http
POST http://127.0.0.1:19723/
Content-Type: application/json

{
  "command": "API.ExecuteAddOnCommand",
  "parameters": {
    "addOnCommandId": {
      "commandNamespace": "TapirCommand",
      "commandName":      "PublishPublisherSet"
    },
    "addOnCommandParameters": {
      "publisherSetName": "Issue – Stage 4 PDF"
    }
  }
}
```

There is **no `LayoutToPDF` / `ExportLayoutAsPdf` verb**. The Publisher Set IS the export verb. The architect (or template designer) authors the recipe once; automation triggers it.

## Why Publisher Sets and not direct layout export

This is Archicad-specific and bites every engineer expecting Revit's "Export PDF" dialog:

1. **Per-item config.** Different sheets in a pack often need different DPIs, color modes, page sizes, save-as options. The Publisher Set carries the per-item config so the pack is reproducible without re-entering 200 values per issue.
2. **Naming convention.** The Publisher Set rule — "filename = `<project-code>_<sheet-number>_<revision>_<date>.pdf`" — is part of the recipe. Headless export without a set would have to re-implement the firm's naming.
3. **Output path.** Each set points at a directory; the firm's QA workflow expects issued sheets to land in a known place.
4. **Revisioning.** The set carries the issue date / stamp / revision letter that goes on each title block.

Trying to bypass Publisher Sets means re-implementing all of this — and the AI orchestrator should NOT be the source of "this firm's filenames look different from every other issue." Use the existing Set.

## Layout settings and revision metadata

Each Layout has settings that control its rendering:

- Page size (A0 / A1 / A2 / A3 / A4 / Arch D / Arch E1 / custom)
- Master Layout reference (which title block to use)
- Revision number / letter / date
- Issue list (the running revision history)
- Drawing cache state (current / out-of-date)

Query / modify with `LayoutBookCommands.GetLayoutSettings` / `SetLayoutSettings`.

## Drawing cache — staleness handling

Each Drawing on a Layout caches a rendered version of its source View. When the model changes, the cache goes stale; the Drawing shows the OLD image until refreshed. Headless publish of a layout with stale drawings publishes the stale image.

Refresh with Tapir's `NavigatorCommands.UpdateDrawings`:

```http
POST http://127.0.0.1:19723/
Content-Type: application/json

{
  "command": "API.ExecuteAddOnCommand",
  "parameters": {
    "addOnCommandId": {
      "commandNamespace": "TapirCommand",
      "commandName":      "UpdateDrawings"
    },
    "addOnCommandParameters": {
      "elements": [
        { "elementId": { "guid": "drawing-guid-here" } }
      ]
    }
  }
}
```

`UpdateDrawings` takes a `Drawing` ElementId (the placed drawing on the layout), NOT a Layout id. Walk every layout's drawings via `GetSubelementsOfHierarchicalElements`-style descent, or use the View Map → Drawing-on-Layout cross-reference.

For an automated issue workflow, ALWAYS update drawings before publish.

## PrintView — when to use it (rarely)

Tapir has `ProjectCommands.PrintView`:

```http
POST http://127.0.0.1:19723/
Content-Type: application/json

{
  "command": "API.ExecuteAddOnCommand",
  "parameters": {
    "addOnCommandId": {
      "commandNamespace": "TapirCommand",
      "commandName":      "PrintView"
    },
    "addOnCommandParameters": {
      "scale":     50,
      "grid":      false,
      "fixText":   false,
      "printArea": "EntireDrawing"
    }
  }
}
```

This sends the currently-active view to the system print queue (PDF printer, real printer, etc.). Use cases:
- One-off print of a single view (an RFI marked-up plan)
- A current-view PDF dump for AI captioning

Do NOT use PrintView for sheet packs. The Publisher Set workflow exists precisely because PrintView lacks per-item config and naming.

## Gotchas

- **PublishPublisherSet returns nothing useful** — no file list, no success object beyond the bare ExecutionResult. Confirm output by filesystem-watching the target dir.
- **The Set must exist.** This whole workflow assumes the firm has set up Publisher Sets once; the AI orchestrator does NOT author them.
- **Layouts ≠ Views.** Don't pass a View GUID where a Layout GUID is expected.
- **Drawings of stale views publish stale.** Always `UpdateDrawings` before `PublishPublisherSet` if the model changed.
- **Master Layout changes propagate.** Editing the master title block changes every Layout that references it — useful for batch revisions, dangerous if unintended.
- **Publishing is synchronous.** A 200-sheet PDF batch blocks Archicad for minutes; size HTTP client timeouts accordingly.
- **Layout numbering and Sheet numbering can diverge.** A Layout's id ("A101") is the firm's sheet number; its position in the Navigator tree may be different ("3rd item in 100-series Subset"). Filename templates should use the Layout's `id` field, not its tree position.

## See also

- [archicad-scripting-context](./archicad-scripting-context.md) — Tapir + JSON-RPC transport
- [archicad-hotlinks-and-teamwork](./archicad-hotlinks-and-teamwork.md) — TeamworkReceive before publish for shared projects
- [view.publish-set](../commands/view.publish-set.md) — the curated issuance verb
