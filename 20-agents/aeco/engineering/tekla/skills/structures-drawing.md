---
name: tekla-structures-drawing
description: Tekla Open API - Tekla.Structures.Drawing namespace. Drawing objects, views, marks, dimensions, symbols, lines, text, annotations, labels, notes, drawing handler, automation, GA drawings, detail drawings, single part drawings, assembly drawings, cast unit drawings. Read drawing status (up to date, ready for issue, issued, locked, frozen, approved, for checking) via direct properties. This skill should be used when creating, modifying, automating, or reporting status of Tekla drawings.
---

# Tekla.Structures.Drawing API Reference (v2025)

## Canonical Identity Key for Drawings

**Default comparison / lookup key: `drawing.Mark`** (stable business key). Use `drawing.Name` when the caller supplies a name. `Drawing` has NO `Identifier`, `Id`, or `GUID` — see the bullets below for details and for identifying the underlying MODEL object.

## Critical Patterns

- Use `DrawingHandler` to access drawings and active drawing
- Drawing objects exist within Views - always get the view first
- Use `DrawingObjectEnumerator` with `while (enumerator.MoveNext())` then `enumerator.Current`
- Drawings have Sheets, Sheets have Views, Views contain DrawingObjects
- Mark/dimension attributes control appearance (line type, color, font)
- Use `DrawingHandler.GetActiveDrawing()` to get the currently open drawing
- Use `DrawingHandler.SetActiveDrawing(drawing)` to open a drawing
- Call `drawing.CommitChanges()` after batch modifications (when using BY_COMMIT mode)
- Drawing coordinates are in paper millimeters unless noted otherwise
- All Insert/Modify/Delete methods return `bool` indicating success
- Use `DatabaseObject.GetUserProperty()` / `SetUserProperty()` for reading/writing **user-defined attributes (UDAs)** on drawings — custom fields like `USER_FIELD_1` or your own attributes. NOT for system status flags.
- Read **system status** (up-to-date, ready for issue, issued, locked, frozen) from direct properties on `Drawing`: `UpToDateStatus`, `IsReadyForIssue`, `IsIssued`, `IsIssuedButModified`, `IsLocked`, `IsFrozen`. Do NOT use `GetReportProperty` — that method exists only on `Tekla.Structures.Model.ModelObject` and is not available on `Drawing`.
- **Drawings have NO `Identifier`, `Id`, or `GUID` property.** `Tekla.Structures.Drawing.Drawing` exposes only business properties (`Name`, `Mark`, `Title1/2/3`, status flags, dates, `CommitMessage`). `Tekla.Structures.Drawing.DatabaseObject` — the base class — exposes only `QueryReturnValue`. This is fundamentally different from `Tekla.Structures.Model.ModelObject`, which does have `.Identifier`. Never write `drawing.Identifier`, `drawing.Id`, `drawing.GUID`, or cast a `Drawing` to `DatabaseObject` to reach an identifier — none of those exist.
- **To find a drawing at runtime**, enumerate `drawingHandler.GetDrawings()` and match by `Name` or `Mark` (the stable business keys). See `tekla-structures-drawing-snippets` for the canonical pattern.
- **To reference the underlying MODEL object** that a drawing is built from, use the drawing subclass's model identifier: `AssemblyDrawing.AssemblyIdentifier`, `SinglePartDrawing.PartIdentifier`, `CastUnitDrawing.CastUnitIdentifier`. These point to the Tekla **model** object — not to the drawing itself.
- **UDAs on drawings** use `drawing.GetUserProperty(name, ref value)` / `drawing.SetUserProperty(name, value)` — inherited from `Tekla.Structures.Drawing.DatabaseObject`.

> Detailed API reference (all classes, methods, properties, constructors) is available in the supporting file `tekla-structures-drawing-api-reference.md`.
> Canonical code snippets (drawing lookup, UDA reads, status translation) are in `tekla-structures-drawing-snippets.md`.
> Workflow patterns (create drawings, read status, export PDF, dimensions) are in the `tekla-drawing-workflows` skill.

