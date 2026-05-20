# `ifc-inspector.georef.check` — is this model correctly georeferenced?

Stateless, read-only. Inspects the `IfcSite` + `IfcMapConversion` + `IfcProjectedCRS` triple and reports the survey point, project north, and the four georeferencing errors that silently break federation. The "will these models actually line up in shared coordinates?" check you run before federating disciplines.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `path` | string | `.ifc` / `.ifczip` path. |

## Outputs

```yaml
ifc-site-origin:          # IfcSite local placement origin
  x: number
  y: number
  z: number
map-conversion:           # IfcMapConversion — model→real-world transform (IFC4 LoGeoRef 50)
  easting:  number
  northing: number
  height:   number
  rotation: number        # XAxisAbscissa/Ordinate resolved to degrees of grid rotation
crs:                      # IfcProjectedCRS
  name:           string  # e.g. "EPSG:27700"
  description:    string
  geodetic-datum: string  # e.g. "OSGB36"
warnings:
  type: array
  items: string
```

## The four errors it catches

These are the failures that don't throw — the file opens fine, the model renders fine, and then two disciplines land 400 km apart or rotated. Each appears as a `warnings` entry:

1. **Missing `IfcMapConversion`** — no real-world transform. The model has only local coordinates; "shared coordinates" is impossible. (Pre-IFC4 files rely on `IfcSite.RefLatitude/RefLongitude/RefElevation` instead — the agent reports those under `map-conversion` as a best-effort and warns that it's LoGeoRef 20, not 50.)
2. **`IfcSite` at (0,0,0)** — model origin = world origin. Sometimes intentional (small project on a local grid) but usually means georeferencing was never set; flagged so a human decides.
3. **`IfcProjectedCRS` undefined** — coordinates exist but no CRS is attached, so you don't know *which* easting/northing system they're in (EPSG:27700? a local site grid?). Unusable for true georeferencing.
4. **`IfcProject` ≠ `IfcSite` `ContextOfItems`** — a rare mismatch in the geometric representation context that breaks downstream federation tooling.

A clean model returns all of `map-conversion` + `crs` populated and an empty `warnings`. See [ifc-georeferencing](../skills/ifc-georeferencing.md) for the LoGeoRef levels (0/20/30/40/50) and how to read `rotation`.

## Composition example — pre-federation gate

```yaml
- id: arch
  agent: ifc-inspector
  command: georef.check
  config: { path: "{{ inputs.arch-ifc }}" }

- id: struct
  agent: ifc-inspector
  command: georef.check
  config: { path: "{{ inputs.struct-ifc }}" }

- id: gate
  inline:
    kind: assert
    description: Both models must share a CRS and be georeferenced before federating
    code: ({arch, struct}) =>
      (arch.warnings.length === 0 && struct.warnings.length === 0
        && arch.crs.name === struct.crs.name)
      || `Georef mismatch: arch=${arch.crs.name||"none"} struct=${struct.crs.name||"none"}`
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `ifc.file-not-found` | `path` doesn't exist | Check the path |
| `ifc.no-site` | File has no `IfcSite` at all (malformed or partial export) | Re-export with the site included |
| `ifc.multiple-sites` | More than one `IfcSite` (campus models) | The agent reports the first and warns; check each site by hand |

## See also

- [ifc-georeferencing](../skills/ifc-georeferencing.md) — LoGeoRef levels, map conversion math, project vs true north
- [`file.info`](./file.info.md) — schema first (IFC2X3 georef differs from IFC4)
- [`validate.schema`](./validate.schema.md) — structural validity (separate from georef correctness)
