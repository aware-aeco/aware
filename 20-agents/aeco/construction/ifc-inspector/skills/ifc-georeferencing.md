---
name: ifc-georeferencing
description: This skill should be used when checking whether IFC models will align in shared coordinates — the Levels of Georeferencing (LoGeoRef 0–50), how IfcMapConversion + IfcProjectedCRS encode the model→real-world transform, project north vs true north, reading the grid rotation, and the four silent georeferencing errors. Encodes the geodesy behind georef.check.
---

# IFC georeferencing

Federation only works if every discipline model sits in the same real-world coordinate system. Georeferencing is the data that places a local model into the world — and when it's wrong, nothing errors, the models just land in the wrong place or rotated. [`georef.check`](../commands/georef.check.md) inspects it.

## Levels of Georeferencing (LoGeoRef)

The accepted framework (Clemen & Görne, 2019) ranks how a model is georeferenced:

| Level | Mechanism | Quality |
|---|---|---|
| **LoGeoRef 0** | none — model at local origin | unusable for shared coords |
| **LoGeoRef 10** | `IfcPostalAddress` (an address string) | not geometric |
| **LoGeoRef 20** | `IfcSite.RefLatitude / RefLongitude / RefElevation` (WGS84 on the site) | low precision, geographic |
| **LoGeoRef 30** | `IfcGeometricRepresentationContext.TrueNorth` (project rotation) | direction only |
| **LoGeoRef 40** | offset/rotation baked into `IfcSite` local placement | placement-based |
| **LoGeoRef 50** | `IfcMapConversion` + `IfcProjectedCRS` | **the correct IFC4 way** |

**Target LoGeoRef 50.** [`georef.check`](../commands/georef.check.md) reports `map-conversion` + `crs` populated when the file reaches 50; on a pre-IFC4 file it falls back to reading `IfcSite.RefLatitude/Long/Elevation` (level 20) and warns that it's the weaker mechanism.

## The LoGeoRef 50 triple

```
IfcMapConversion          (the model → projected-CRS transform)
  Eastings          456000.0      # where model origin sits in the projected CRS
  Northings         172000.0
  OrthogonalHeight       0.0
  XAxisAbscissa          0.9659    # cos θ  — defines grid rotation
  XAxisOrdinate          0.2588    # sin θ
  Scale                  1.0
        │ references
        ▼
IfcProjectedCRS
  Name              "EPSG:27700"   # the projected CRS (here: British National Grid)
  GeodeticDatum     "OSGB36"
  MapProjection / MapZone …
```

`georef.check` returns `map-conversion.rotation` as **degrees**, computed `atan2(XAxisOrdinate, XAxisAbscissa)` — the grid rotation from model X to CRS east. The example above (`0.9659, 0.2588`) is +15°.

## Project north vs true north

- **Project north** = the model's +Y axis (what the drawings are drawn "up" toward — usually aligned to the building, not the world).
- **True north** = actual geographic north.
- `IfcGeometricRepresentationContext.TrueNorth` carries the rotation between them. This is *separate* from the `IfcMapConversion` grid rotation (which is model→grid-east). A model can have project north ≠ true north ≠ grid north — all three angles can differ, and confusing them is a classic federation bug.

## The four silent errors `georef.check` flags

Each opens fine and renders fine, then federation lands models in the wrong place:

1. **Missing `IfcMapConversion`** — only local coordinates exist; shared coordinates impossible.
2. **`IfcSite` at (0,0,0)** — model origin = world origin. Sometimes intentional (small local-grid project), usually means georeferencing was never set. Flagged for a human to decide.
3. **`IfcProjectedCRS` undefined** — coordinates exist but no CRS, so you don't know *which* easting/northing system they're in.
4. **`IfcProject` ≠ `IfcSite` `ContextOfItems`** — a representation-context mismatch that breaks downstream federation tooling.

A clean model: `map-conversion` + `crs` populated, empty `warnings`. The cross-model check is "do both files report the same `crs.name` *and* compatible map conversions?" — see the [`georef.check`](../commands/georef.check.md) composition example.

## See also

- [`georef.check`](../commands/georef.check.md) — the verb
- [`file.info`](../commands/file.info.md) — schema first; IFC2X3 georef (level 20) differs from IFC4 (level 50)
- [ifc-schemas-and-validation](./ifc-schemas-and-validation.md) — `IfcMapConversion` is IFC4+; 2x3 files can't reach LoGeoRef 50
