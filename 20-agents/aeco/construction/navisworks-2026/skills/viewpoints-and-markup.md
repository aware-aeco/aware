---
name: navisworks-viewpoints-and-markup
description: This skill should be used when composing snippets that work with Navisworks viewpoints, animations, redlines, and saved tags — the artifacts every coordination workflow produces. Encodes the SavedViewpoint vs CurrentViewpoint distinction, the AnimationView (camera path) data model, the Comment / Markup hierarchy on viewpoints, the BCF-export round-trip (which serializes saved viewpoints into BCF topics), and the gotcha that viewpoint cameras use Navisworks's right-handed coordinate system (Z up) but render planes are saved with their own UCS.
---

# Navisworks viewpoints and markup

Viewpoints are how coordinators communicate issues. A viewpoint freezes camera position + visibility + hide state + cross-section + selection at a moment in time. Markups (redlines, text, dimensions) and Comments attach to a viewpoint to explain the issue.

## The viewpoint hierarchy

| Type | Class | Notes |
|---|---|---|
| Current | `Document.CurrentViewpoint` | Live; reflects the user's current camera |
| Saved | `Document.SavedViewpoints` (`SavedViewpoint`) | Persisted in the .nwd/.nwf; can have comments + markup |
| Folder | `SavedItemReference.IsGroup == true` | Container for organizing saved viewpoints |
| Animation | `Document.SavedViewpoints` (`AnimationView`) | Sequence of viewpoints + transitions; renderable as a flythrough |
| Redline | `LineSegment`, `Cone`, etc. attached to a viewpoint | 3D markup geometry stored with the viewpoint |
| Comment | `Comment` on a viewpoint or model item | Threaded text discussions |

## Save current viewpoint as a SavedViewpoint

```csharp
using Autodesk.Navisworks.Api;
using Autodesk.Navisworks.Api.ComApi;

var doc = Application.ActiveDocument;

var current = doc.CurrentViewpoint.CreateCopy();   // CRITICAL: copy or you'll modify the live view
current.DisplayName = "Clash 0042 — beam crash";
doc.SavedViewpoints.AddCopy(current);
```

`CreateCopy()` is required — `CurrentViewpoint` returns a live ref. Modifying it directly changes what the user sees.

## Camera basics

Navisworks viewpoints have:

| Property | What |
|---|---|
| `Position` | Camera location (Point3D, world coords) |
| `Forward` | Camera direction (UnitVector3D) |
| `Up` | Camera up vector |
| `Projection` | Perspective vs Orthographic enum |
| `ApertureType` | NaturalLens (FoV) vs Focal length |
| `FocalDistance` | For depth-of-field rendering |

Navisworks's world coordinate system is **Z-up, right-handed** (unlike SketchUp's Z-up-left-handed, or Tekla's Z-up arbitrary). Imported coordinates respect the source file's units AND Navisworks's transform-on-attach.

## Adding markup to a viewpoint

Markup is 3D geometry living in the viewpoint, NOT in the model. So when the viewpoint is recalled, the markup shows; when another viewpoint is active, it doesn't.

```csharp
using Autodesk.Navisworks.Api.Interop.LcOpRedline;   // older COM API

// (Pseudocode — the actual API path varies by Navisworks version)
var redline = vp.AddRedline(new RedlineLineSegment(start, end), Color.Red);
```

The COM API surface is gnarly. For programmatic markup creation, BCF round-trip is often easier: create a BCF topic with viewpoint + comment, import the BCF into Navisworks, the markup arrives correctly.

## Animations (camera fly-throughs)

```csharp
var animation = new AnimationView(doc, "Walkthrough A");
animation.AddCopy(vp1);
animation.AddCopy(vp2);
animation.AddCopy(vp3);
animation.Duration = TimeSpan.FromSeconds(30);
doc.SavedViewpoints.AddCopy(animation);
```

The interpolation between viewpoints is automatic (linear in camera position; great-arc in rotation). For non-linear paths, use multiple short animations spliced.

## BCF round-trip

The BCF (BIM Collaboration Format) is OASIS's vendor-neutral way to exchange issues + viewpoints between Navisworks / Revit / Solibri / Tekla / etc. A BCF topic contains:

- Title + description + assigned-to + due-date + status
- Zero or more viewpoint snapshots (camera state)
- Zero or more comment threads
- Markup that travels as overlay images per viewpoint

To EXPORT Navisworks viewpoints as BCF: AWARE's curated `bcf.export` verb (in the navisworks agent) wraps the COM `BCFExporter` interop.

To IMPORT BCF into Navisworks: `bcf.import` reads the .bcfzip and creates SavedViewpoints + Comments.

The trick: BCF GUIDs are stable across tools. A BCF topic created in Navisworks, exported, imported into Revit, edited there, re-exported, re-imported into Navisworks — should round-trip with the same GUID and incremental comment chain. In practice, vendor implementations have BCF compliance bugs. Test the round-trip with your specific vendor matrix.

## Comments

Comments thread on viewpoints (and sometimes on selected items). Each comment has author + timestamp + status + body. The comment lifecycle:

```csharp
var comment = new Comment("Please reposition this beam", CommentStatus.New, "pawel");
viewpoint.Comments.Add(comment);

// Mark resolved
var existing = viewpoint.Comments.First();
existing.Status = CommentStatus.Resolved;
```

CommentStatus values: New, Active, Approved, Resolved.

## Common gotchas

- **`CurrentViewpoint` returns a live ref**, not a snapshot. Always `.CreateCopy()` before modifying.
- **Saved viewpoints persist the hide-state of items.** Hiding an item AFTER saving a viewpoint, then recalling, restores the saved hide-state — unhides what you hid.
- **Markup is per-viewpoint, not per-model.** A redline on viewpoint A doesn't show on viewpoint B.
- **Animations have a Duration but don't auto-record the framing.** Setting `Duration = 60s` doesn't redistribute the keyframes; you control timing via the AddCopy order + relative-time fields.
- **BCF version matters.** Navisworks 2026 exports BCF 2.1 by default; tooling that expects 3.0 may complain. Set version explicitly in the export options.

## Standard pattern

```csharp
var doc = Application.ActiveDocument;

// Save the current view as a named viewpoint
var vp = doc.CurrentViewpoint.CreateCopy();
vp.DisplayName = args["name"].ToString();
doc.SavedViewpoints.AddCopy(vp);

return new {
    name = vp.DisplayName,
    saved = true,
    camera = new {
        x = vp.Position.X, y = vp.Position.Y, z = vp.Position.Z,
    },
};
```

## See also

- [bcf-roundtrip](./bcf-roundtrip.md) — the BCF export/import details
- [clash-detection-workflow](./clash-detection-workflow.md) — clashes auto-create viewpoints
- [federation-and-coordinates](./federation-and-coordinates.md) — what `Position` is relative to
