---
name: sketchup-units-and-precision
description: This skill should be used when writing snippets that depend on numerical correctness in SketchUp — geometry distances, area / volume / face counts, intersection tests, snap behavior, exports, or anything that reads / writes a numeric value with units attached. Encodes SketchUp's internal-INCH representation (everything is inches internally regardless of UI display), the `Length` class for round-trip-safe arithmetic, the `model.options["UnitsOptions"]` dictionary, the precision setting (display only — math is double-precision regardless), and the gotchas around imported models with mismatched display units.
---

# SketchUp units and precision

SketchUp has a single peculiar property that confuses every newcomer: **everything is stored in inches internally**, regardless of what the UI displays. The unit system controls display formatting only; the underlying math always operates on inch values.

## The internal-inch invariant

```ruby
model = Sketchup.active_model
# Even if the model is set to Millimeters in the UI:
p = Geom::Point3d.new(1000, 0, 0)
# This is a point at 1000 INCHES, not 1000 mm. ~25.4 meters away.
```

When passing literal numbers to SketchUp APIs, **always convert to inches** unless you know you're in an inch-units model:

```ruby
# Length helpers (added by Numeric in SketchUp's Ruby env):
1000.mm        # => 39.37 (a Length in inches)
1.m            # => 39.37
24.inch        # => 24
24.feet        # => 288
1.yard         # => 36

# Or with the Length constructor:
Geom::Point3d.new(1000.mm, 0, 0)   # right
Geom::Point3d.new(1000, 0, 0)      # WRONG (1000 inches if you meant 1000 mm)
```

For values READ from SketchUp, convert OUT:

```ruby
length_inches = (p1 - p2).length        # Length object, value in inches
length_mm     = length_inches.to_mm     # Float in mm
length_m      = length_inches.to_m      # Float in m
```

## The `Length` class

SketchUp's `Length` is a subclass of `Float` that knows it represents inches. Arithmetic returns plain Floats (losing unit awareness), which is the source of many bugs. Always re-wrap if you need a Length result:

```ruby
a = 3.feet                # Length, value = 36.0
b = a * 2                 # Float, value = 72.0  (NOT a Length)
c = (a * 2).to_l          # Length, value = 72.0 inches

# Or:
total = 1.m + 2.feet      # Float, value = 39.37 + 24 = 63.37 inches
total.to_mm               # 1609.59 mm
```

## Reading the model's display units

```ruby
opts = model.options["UnitsOptions"]
length_unit_idx     = opts["LengthUnit"]              # 0=in 1=ft 2=mm 3=cm 4=m
length_precision    = opts["LengthPrecision"]          # decimal places
length_format       = opts["LengthFormat"]             # 0=Decimal 1=Architectural 2=Engineering 3=Fractional
angle_precision     = opts["AnglePrecision"]
volume_unit_idx     = opts["VolumeUnit"]               # 1=in3 2=ft3 3=mm3 4=cm3 5=m3 6=L 7=US-gal
suppress_units      = opts["SuppressUnitsDisplay"]     # true/false
force_inch_display  = opts["ForceInchDisplay"]         # 2018+
```

Precision affects DISPLAY only (rounding shown in dialogs and inferences). Underlying math is always double-precision IEEE 754.

## Tolerance is implicit

Unlike Rhino's explicit `ModelAbsoluteTolerance`, SketchUp uses HARD-CODED tolerances for geometry merging and snap. These are NOT user-configurable through the Ruby API:

- Coincident point tolerance: ~0.001 inch (regardless of model units)
- Coplanar face tolerance: a few millionths of an inch
- Edge length tolerance: 0.001 inch

For most AECO work this is fine. For mm-units models with sub-millimeter features (rare in architecture, common in mechanical), the inch-tolerance can collapse intended distinctions.

## Volume and area gotchas

`face.area` returns the area in **square inches**. `group.bounds.volume` returns the volume in **cubic inches**. Convert via:

```ruby
face_area_in   = face.area
face_area_m2   = face_area_in / 1550.0031   # 1 sq inch ≈ 0.000645 m², so divide by ~1550

bb_volume_in3  = group.bounds.volume
bb_volume_m3   = bb_volume_in3 / 61023.74   # 1 cubic inch ≈ 1.6387e-5 m³
```

SketchUp's `Geom.unit_conversion` is the canonical helper for these — never hardcode the constants in production scripts.

## Import gotchas

- **IFC imports arrive in their authored units** (typically meters or millimeters). SketchUp converts on import but the underlying coordinate values are scaled to inches. Round-trip is precise.
- **DWG imports without a recorded unit** default to inches. Models that look "wrong" after DWG import are usually a unit mismatch — set the DWG import dialog correctly.
- **Skp models from other authors may have non-standard display precision.** Don't trust `LengthPrecision` from imports for your own calc precision.

## The standard pattern in `aware-sketchup exec` snippets

```ruby
model = Sketchup.active_model
opts  = model.options["UnitsOptions"]
unit_idx = opts["LengthUnit"]
unit_map = %w[Inches Feet Millimeters Centimeters Meters]
display_unit = unit_map[unit_idx]

# When ACCEPTING dimensions from args, convert based on display_unit:
input_mm = args["width-mm"].to_f
width = input_mm.mm   # converts to inches for SketchUp's internal use

# When RETURNING dimensions, convert OUT to a known unit:
result_inches = some_geometry.length
result_mm     = result_inches.to_mm
# Send result_mm to the caller for cross-vendor consistency
```

## See also

- [`model-info.summary`](../commands/model-info.summary.md) — reads the units field for the caller
- [`entity.list-by-tag`](../commands/entity.list-by-tag.md) — bbox values are in inches; convert for cross-vendor result
