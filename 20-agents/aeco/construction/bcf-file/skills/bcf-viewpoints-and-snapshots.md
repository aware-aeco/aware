---
name: bcf-viewpoints-and-snapshots
description: This skill should be used when extracting or reasoning about BCF viewpoints — the camera model (perspective vs orthogonal), how the bcf-file agent normalises both to an eye/target/up triple, the IFC-GUID component selection, clipping planes, and how embedded snapshot PNGs are extracted. Encodes the visual side of BCF behind the read.viewpoints verb.
---

# BCF viewpoints & snapshots

A BCF *viewpoint* is the visual half of a coordination issue: where the camera was, what was selected/coloured, and a snapshot PNG of that view. [`read.viewpoints`](../commands/read.viewpoints.md) surfaces all of it. This skill encodes the camera model and the gotchas.

## The two camera flavours

A `.bcfv` carries exactly one of:

```xml
<PerspectiveCamera>
  <CameraViewPoint><X>12.3</X><Y>4.5</Y><Z>1.7</Z></CameraViewPoint>   <!-- eye -->
  <CameraDirection><X>-0.7</X><Y>-0.7</Y><Z>0</Z></CameraDirection>     <!-- view dir -->
  <CameraUpVector><X>0</X><Y>0</Y><Z>1</Z></CameraUpVector>             <!-- up -->
  <FieldOfView>60</FieldOfView>
</PerspectiveCamera>
```

```xml
<OrthogonalCamera>
  ... same CameraViewPoint / Direction / UpVector ...
  <ViewToWorldScale>35.0</ViewToWorldScale>     <!-- instead of FieldOfView -->
</OrthogonalCamera>
```

`read.viewpoints` **normalises both** to a single `{ eye, target, up }` triple so callers don't branch on camera type:

- `eye` = `CameraViewPoint`
- `target` = `CameraViewPoint + CameraDirection` (a point one unit along the view direction; the agent reports a point, not a direction, so downstream renderers can `lookAt` it directly)
- `up` = `CameraUpVector`
- `camera-type` = `"perspective"` or `"orthogonal"` so you can recover `FieldOfView` vs `ViewToWorldScale` from the raw `.bcfv` if you need the projection parameter.

All camera coordinates are **world-space** (the same coordinate system as the referenced IFC geometry). They are not relative to anything.

## Component selection is by IFC GUID

```xml
<Components>
  <Selection>
    <Component IfcGuid="2O2Fr$t4X7Zf8NOew3FNr2" />
  </Selection>
  <Coloring>
    <Color Color="ff0000"><Component IfcGuid="…"/></Color>   <!-- 6-hex RGB, or optional 8-hex RRGGBBAA (alpha trailing) -->
  </Coloring>
  <Visibility DefaultVisibility="true">
    <Exceptions><Component IfcGuid="…"/></Exceptions>
  </Visibility>
</Components>
```

The selection / colouring / visibility-exception lists all reference **IFC GUIDs**, which is what makes a viewpoint portable between tools — but also what makes it fragile. If a model re-export churns those GUIDs, the camera still works while the highlight resolves to nothing. This is the silent-break failure detailed in [bcf-round-trip-interop](./bcf-round-trip-interop.md). Component colours are 6-hex RGB or an optional 8-hex form with a **trailing** alpha byte (`RRGGBBAA`, e.g. `ff000099`) — identical in 2.1 and 3.0 per `visinfo.xsd`, so [`convert`](../commands/convert.md) leaves colouring untouched (the common belief that 2.1 lacks alpha is wrong).

## Clipping planes & sections

Viewpoints may also carry `<ClippingPlanes>` (cut planes) and 2D `<Lines>` / `<Bitmaps>` markup. `read.viewpoints` returns the camera + component selection; for clipping geometry, read the raw `.bcfv` (the agent does not currently surface clipping planes in its typed output — they are preserved on disk for round-trips but not parsed into the response).

## Snapshot extraction

Each viewpoint may have an embedded PNG. [`read.viewpoints`](../commands/read.viewpoints.md) extracts these to a per-run temp directory under the workspace and returns the absolute path in `snapshot-path`. A viewpoint with no embedded PNG returns `snapshot-path: ""` while still returning the camera + selection.

**Snapshots are frozen provenance.** The PNG was captured when the viewpoint was created. If the model changed since, the image won't reflect the current state — by design (BCF prioritises a record of *what the reviewer saw* over freshness). Use snapshots for the audit trail / report gallery, not as a live model view.

## Common use — a per-issue gallery

Join [`read.topics`](../commands/read.topics.md) (the metadata) with `read.viewpoints` (the images) on `topic-guid`, then feed the `html-report` agent: each issue becomes a card with its title, status, and snapshot. This is the single most common reason to touch viewpoints.

## See also

- [`read.viewpoints`](../commands/read.viewpoints.md) — the verb
- [bcf-round-trip-interop](./bcf-round-trip-interop.md) — why IFC-GUID anchors break silently
- [bcf-format-anatomy](./bcf-format-anatomy.md) — where `.bcfv` and `.png` sit in the archive
