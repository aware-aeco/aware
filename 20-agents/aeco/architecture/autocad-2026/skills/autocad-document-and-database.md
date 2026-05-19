---
name: autocad-document-and-database
description: This skill should be used when composing snippets that touch AutoCAD's Document / Database / Editor objects — the three foundational entry points for any .NET-based AutoCAD automation. Encodes the difference between Document (UI handle) and Database (in-memory drawing), the DocumentManager / DocumentCollection lifecycle, the role of the Editor (command-line, prompt, selection), and the gotcha that Database can outlive Document (sideload databases for read-only access without opening in the UI).
---

# AutoCAD Document and Database

AutoCAD's .NET API splits the "drawing" concept into THREE distinct objects. Every snippet starts with picking the right one.

## The three objects

| Object | Class | What it represents |
|---|---|---|
| **Application** | `Application` (static) | The AutoCAD process; entry to everything else |
| **Document** | `Document` | A drawing open in a UI window — has Editor (command line), Window |
| **Database** | `Database` | The in-memory representation of a .dwg file's data |
| **Editor** | `Editor` | UI for command-line, prompts, selection on a Document |

**A Document HAS a Database.** `doc.Database` returns the Database for that open drawing.

**A Database can exist WITHOUT a Document.** Side-loaded databases (`Database db = new Database(false, true)`; `db.ReadDwgFile(path, FileShare.Read, true, "")`) live in memory for reading without opening in the UI. Essential for batch operations across hundreds of drawings.

## DocumentManager

`Application.DocumentManager` is the collection of open Documents:

```csharp
using Autodesk.AutoCAD.ApplicationServices;
using Autodesk.AutoCAD.DatabaseServices;

var docMgr = Application.DocumentManager;
var docCount = docMgr.Count;
var currentDoc = docMgr.MdiActiveDocument;        // the one the user sees

foreach (Document doc in docMgr)
{
    Console.WriteLine(doc.Name);                   // file path
}
```

`MdiActiveDocument` can be null if no drawing is open. Always null-check.

## Switching the active document

```csharp
var targetDoc = docMgr.OfType<Document>().FirstOrDefault(d => d.Name.EndsWith("project.dwg"));
if (targetDoc != null)
    docMgr.MdiActiveDocument = targetDoc;
```

Switching is synchronous — the UI updates immediately on the next message-loop iteration.

## Document.LockDocument

Multi-threaded automation requires explicit document locking — without it, writing from a background thread crashes AutoCAD:

```csharp
using (DocumentLock docLock = doc.LockDocument())
{
    using (var t = db.TransactionManager.StartTransaction())
    {
        // ... reads + writes ...
        t.Commit();
    }
}
```

If you're already on the main UI thread (called from a command), the lock is implicit — but using an explicit one is harmless and forward-compatible.

## The Editor — for prompts + selection

`doc.Editor` is the UI for command-line interaction:

```csharp
var ed = doc.Editor;

// Print to command line
ed.WriteMessage("\n[AWARE] Processing 47 walls...");

// Prompt for a point
var ppr = ed.GetPoint("\nSpecify origin: ");
if (ppr.Status == PromptStatus.OK)
{
    var origin = ppr.Value;
}

// Get user's selection
var selRes = ed.SelectAll();
if (selRes.Status == PromptStatus.OK)
{
    var ids = selRes.Value.GetObjectIds();
}
```

For non-interactive AWARE scripts, prefer `SelectAll()` or filter-based selection over `GetSelection` (which prompts the user).

## Side-loaded Database (read-only batch)

```csharp
using (var db = new Database(false, true))
{
    db.ReadDwgFile(@"C:\Projects\big-model.dwg", FileShare.Read, true, "");

    using (var t = db.TransactionManager.StartTransaction())
    {
        var blockTable = t.GetObject(db.BlockTableId, OpenMode.ForRead) as BlockTable;
        var modelSpace = t.GetObject(blockTable[BlockTableRecord.ModelSpace], OpenMode.ForRead) as BlockTableRecord;

        foreach (ObjectId id in modelSpace)
        {
            var entity = t.GetObject(id, OpenMode.ForRead) as Entity;
            // read-only access — never call Upgrade/Erase/etc.
        }
        t.Commit();
    }
}
```

Side-loaded databases are the right path for "process 1000 .dwg files in a batch" — never open each in the UI.

## Open vs side-loaded — write semantics

| Operation | Open document | Side-loaded database |
|---|---|---|
| Read | Yes | Yes |
| Write | Yes (with LockDocument) | Yes, but writes go to in-memory db ONLY — call `db.SaveAs(path)` to persist |
| AutoCAD command (`SendStringToExecute`) | Yes | NO — commands need the Editor |
| Plot / Print | Yes | Yes (uses PlotEngine; harder) |

For "open, modify, save, close" workflows on many files, the side-loaded path is dramatically faster and parallelizable.

## Common gotchas

- **`Application.DocumentManager.MdiActiveDocument` is NULL when no drawing is open.** Always null-check before deref.
- **Database from `new Database(false, true)` requires `ReadDwgFile` before use.** Constructing alone doesn't load anything.
- **`db.SaveAs(path, DwgVersion.Current)` preserves whatever version AutoCAD is currently using.** For interop with older AutoCAD, pass an explicit `DwgVersion`.
- **Side-loaded Database is NOT shown in the UI.** Users can't see your work-in-progress. Save and re-open if you want them to.
- **Each Database has its own TransactionManager.** Don't mix transactions across databases.

## Standard pattern

```csharp
// Foreground: edit the currently-open drawing
var doc = Application.DocumentManager.MdiActiveDocument
          ?? throw new InvalidOperationException("no drawing open");
var db  = doc.Database;
var ed  = doc.Editor;

using (var docLock = doc.LockDocument())
using (var t = db.TransactionManager.StartTransaction())
{
    var blockTable = t.GetObject(db.BlockTableId, OpenMode.ForRead) as BlockTable;
    var ms = t.GetObject(blockTable[BlockTableRecord.ModelSpace], OpenMode.ForWrite) as BlockTableRecord;

    // ... modify ms ...

    t.Commit();
}
ed.WriteMessage("\n[AWARE] done");
```

## See also

- [autocad-transactions-and-symbol-tables](./autocad-transactions-and-symbol-tables.md) — the transaction model in detail
- [autocad-paper-space-and-layouts](./autocad-paper-space-and-layouts.md) — for plot/print workflows
