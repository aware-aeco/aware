---
name: rhino-units-and-tolerances
description: This skill should be used when writing snippets that depend on numerical correctness in Rhino — geometry distances, area / volume computations, boolean operations, intersection tests, mesh repair, snap-and-merge logic, or anything that calls a `RhinoCommon` method with a `tolerance` parameter. Encodes Rhino 8's model-units system, the relationship between `ModelAbsoluteTolerance` / `ModelAngleToleranceDegrees` / `ModelRelativeTolerance`, when each one is the right knob, the gotchas around imported models with mismatched units, and the recommended pattern for tolerance arguments in `aware-rhino exec` scripts.
---

# Rhino units and tolerances

Most "this works in the UI but my script returns wrong numbers" bugs in Rhino are tolerance bugs. Get these right early.

## The unit system

Every `RhinoDoc` has a `ModelUnitSystem` enum value. Common values:

| UnitSystem | Typical use |
|---|---|
| `Millimeters` | UK/EU construction, mechanical, fabrication. Most common in AECO. |
| `Centimeters` | Architectural in some EU countries. Less common. |
| `Meters` | Large-scale (urban, civil). Imported IFC often arrives in Meters. |
| `Feet` / `Inches` | US construction. |

The unit system affects **interpretation of coordinates** but not the underlying double-precision math. A Point3d at `(1000, 0, 0)` is "one thousand millimeters" in a mm doc and "one thousand feet" in a ft doc — exactly the same internal representation, different meaning.

**When two documents have different unit systems and you copy geometry from one to the other, you MUST scale**. `Rhino.RhinoMath.UnitScale(fromUnit, toUnit)` returns the multiplier (e.g. `1000.0` to go from meters to millimeters).

## The three tolerances

`RhinoDoc` carries three tolerance values. They're not interchangeable.

| Tolerance | Type | Default (mm doc) | Used by |
|---|---|---|---|
| `ModelAbsoluteTolerance` | distance | `0.001` (= 0.001 mm = 1 μm) | All geometry distance checks (`Brep.IsValid`, `Curve.IsClosed`, `Mesh.FillHoles`, BREP boolean ops, `Intersect`) |
| `ModelAngleToleranceDegrees` (radians via `ModelAngleToleranceRadians`) | angle | `1.0` degree | Angle comparisons (parallel-curve detection, plane coplanarity, isocurve direction) |
| `ModelRelativeTolerance` | percent | `0.01` (= 1%) | Mass-properties (area/volume) precision, surface knot insertion |

**Almost every RhinoCommon method that takes a `tolerance` parameter expects ModelAbsoluteTolerance.** When in doubt, pass `doc.ModelAbsoluteTolerance`.

## Tolerance and unit interaction

The "default" tolerance changes with the document's units:

| Units | Default `ModelAbsoluteTolerance` |
|---|---|
| Millimeters | `0.001` mm |
| Meters | `0.001` m = 1 mm |
| Feet | `0.001` ft ≈ 0.305 mm |
| Inches | `0.0001` in ≈ 0.00254 mm |

Imported models often have wildly wrong tolerances. An IFC import marked as Meters but with `ModelAbsoluteTolerance = 0.001` (= 1mm) will mis-classify a brick-wall joint as "open". Always check before doing precision work.

## The standard pattern in `aware-rhino exec` snippets

```csharp
var doc = Rhino.RhinoDoc.ActiveDoc;
var tol = doc.ModelAbsoluteTolerance;       // never hard-code; always read
var ang = doc.ModelAngleToleranceRadians;   // RAD, not degrees, in RhinoCommon API

// Example: boolean union with the right tolerance.
var result = Rhino.Geometry.Brep.CreateBooleanUnion(breps, tol);

// Example: intersection with the right tolerances.
var inter = Rhino.Geometry.Intersect.Intersection.MeshMesh(meshA, meshB, tol);

// Example: angle-aware welding.
mesh.Weld(ang);   // welds vertices whose face-normal angles are within ModelAngleToleranceRadians
```

## When the script needs to coerce input to a known unit

For verbs like `view.capture-bitmap` (resolution in pixels — unit-free) the unit doesn't matter. For verbs like `layer.set-by-pattern` (no geometry math) the unit doesn't matter.

For verbs that DO touch geometry (`mesh.repair`, `selection.by-attribute` with bbox filters, `export.layouts-to-pdf` with page sizes), assume the user's document unit. Don't hard-code millimeters.

## Snapping gotcha

Rhino's snap is governed by separate user settings — not the doc tolerances. Snapping in the UI may collapse points that the API treats as distinct. If a user complains "but in the UI they looked identical", check `doc.ModelAbsoluteTolerance` vs the snap radius.

## Persistence

Tolerance values are saved with the document. Changing them mid-session changes future ops but doesn't re-evaluate existing geometry. Use with caution.
