# Blender visualization agent — design

**Date:** 2026-07-22
**Status:** approved direction, pre-implementation
**Author:** brainstormed with Pawel (session-63df38)

## Goal

Add a `blender` agent under `20-agents/aeco/visualization/blender/` — the first visualization
agent in the substrate that goes **model in → finished renders out with no human in the loop**.
Input is IFC; output is production stills (PNG) and turntable videos (MP4).

The existing visualization agents (`enscape-prep`, `twinmotion-prep`) are honest *prep-only*
agents — their manifests say it outright: "AWARE preps the scene + the user presses Render",
because Enscape/Twinmotion cannot render headlessly. Blender can: `blender -b -P script.py`
runs import → materials → camera → render entirely unattended, with two production renderers
in the box (EEVEE for fast drafts/turntables, Cycles for GPU path-traced hero shots).

## The pipeline this serves

```
floless steel editor → steel.takeoff/v1 contract → contract-to-scene.ts (bake)
    → generic scene JSON → aware ifc.write (cli/src/render/ifc.rs) → IFC4
    → blender agent → hero still / turntable MP4
```

Everything left of the Blender agent already exists and is tested. `ifc.write` emits
deterministic IFC4: `IfcColumn`/`IfcBeam`/`IfcMember` with parametric `IfcProfileDef`s
(i/channel/angle/rhs/chs/rect), `IfcMaterial` from the ASTM grade, `IfcStyledItem` from group
colours, meshes as `IfcTriangulatedFaceSet`. That is exactly the semantic payload an unattended
renderer needs for automatic look assignment.

The agent stays **generic** (decalog: substrate-first, no FloLess leakage): it consumes any IFC —
FloLess scene-IFC, Tekla export, Revit export — identically. FloLess is simply the first producer.

## Input format decision: IFC only in v1 (glTF rejected)

Verified facts behind the decision (checked 2026-07-22):

- **Tekla Structures does not export glTF** — glTF is import-only; exports are IFC 2x3/4/4.3,
  OBJ, DAE, SKP, STEP, DWG ([Tekla 2025 format table](https://support.tekla.com/doc/tekla-structures/2025/int_compatible_software)).
- **Collada (.dae) was removed in Blender 5.0** (deprecated since 4.2; last supported in
  4.5 LTS) — the DAE path is a dead end.
- **The FloLess pipeline already ships IFC** via `ifc.write`; a glTF path would require writing
  a *new* CLI exporter that carries *less* semantic data to feed the same agent.
- **IFC semantics are the material-assignment mechanism.** With no human clicking materials,
  the agent maps `IfcColumn`/`IfcBeam` class + `IfcMaterial` grade + profile to looks. glTF
  input would degrade to name-string heuristics or a clay render.
- **glTF's import-speed advantage is irrelevant at this scale.** A steel takeoff scene is
  hundreds to low-thousands of members; `ifcopenshell` tessellates parametric profiles in
  seconds. Pre-tessellation only matters for 50k+-part federated models.

glTF may return later as a *secondary* input for models arriving from non-BIM/web sources
(native Blender importer, near-zero cost) — explicitly out of scope for v1.

## IFC import implementation: raw ifcopenshell, not Bonsai

Three options were evaluated:

1. **Raw `ifcopenshell` in Blender's bundled Python** *(chosen)* — pip-install `ifcopenshell`
   into Blender's Python, use the `ifcopenshell.geom` iterator to tessellate and build Blender
   meshes directly, reading class/material/storey from the same file handle. Fully
   headless-proven; no add-on dependency.
2. **IfcConvert preprocessing** (IFC → glTF + JSON semantics sidecar) — fastest for huge
   models; more moving parts. Escape hatch for later, not v1.
3. **Bonsai add-on** — GUI-oriented, documented instability under load, no confirmed clean
   story for `blender -b` background mode. Rejected for the agent; it is for humans.

## Agent shape

Directory: `20-agents/aeco/visualization/blender/` (plain `blender`, matching `xeokit`/`three`
naming — the directory already namespaces it under `visualization/`).

Manifest pattern follows `twinmotion-prep/manifest.yaml` (hand-curated provenance, `requires`,
CLI transport). Transport: `aware-blender` CLI that shells out to
`blender -b -P scripts/<command>.py -- <json-args>`; the `bpy` scripts ship inside the agent
directory under `scripts/`.

### Commands (v1)

| Command | Mode | Description |
|---|---|---|
| `scene.import` | write | IFC → staged `.blend` (meshes + semantics as custom properties) |
| `scene.apply-look` | write | Preset looks: `clay`, `realistic`, `section-style` — assigned by IFC class + material grade, group-colour tint as fallback |
| `render.still` | write | Camera-framed still; `quality: draft` (EEVEE) / `production` (Cycles); PNG out |
| `render.turntable` | write | 360° orbit MP4 around the model (EEVEE); duration/fps/resolution inputs |
| `scene.info` | read | List imported elements by class/material/storey (verification + debugging) |

Deferred (v2+): `render.panorama` (equirectangular 360), walkthrough paths, per-storey
isolation renders, IfcConvert fast path.

### Semantic look mapping (the core skill)

| IFC signal | Look decision |
|---|---|
| `IfcColumn` / `IfcBeam` / `IfcMember` + steel grade (A992/A500/A36/S355…) | steel finish (painted / galvanized preset) |
| `IfcSlab` / concrete grade | concrete |
| `IfcPlate` + glass material | glazing |
| `IfcStyledItem` colour | fallback tint when class/material is uninformative |
| nothing usable | clay (neutral matte) — never fail the render |

### Requires

- `blender@4.2+|5.x` (headless-capable; scripts target the stable `bpy` API surface)
- Python package `ifcopenshell` in Blender's bundled Python (the agent's install step handles
  `blender -b --python-expr "…pip install ifcopenshell…"` or documents the one-liner)

## Delivery plan

1. **Prototype the scripts first** (all real risk lives here): 3–4 standalone `bpy` scripts —
   IFC import via `ifcopenshell.geom`, camera auto-framing, EEVEE still, turntable — verified
   headless against a local Blender with the reference fixture.
2. **Agent**: `manifest.yaml` + skills (`headless-rendering.md`, `ifc-import-ifcopenshell.md`,
   `look-presets.md`) + the proven `scripts/`, registered in `registry-index.json`, stats
   bumped (`agents_total`, `agents_curated`).
3. **Example app**: `30-apps/_examples/model-to-renders.app` — IFC in → hero still +
   turntable out. First fully autonomous visualization chain in the substrate.

### Verification

- Reference fixture: a small scene JSON checked in next to the agent's tests, run through
  `ifc.write` at test time (deterministic bytes → a stable IFC asset without committing binaries).
- Headless smoke test: import fixture → `render.still` at low res (EEVEE) → assert PNG exists
  and is non-trivial (dimensions + not-all-one-colour).
- Look-mapping unit check: `scene.info` output matches the fixture's known class/material
  inventory.

## Error handling

- Missing Blender binary / wrong version → named error with install hint (mirror the CLI's
  "named error for a missing agent" convention).
- `ifcopenshell` not installed in Blender's Python → named error + the exact install one-liner.
- Unparseable IFC element → skip + count, never abort the whole render; report skipped GUIDs
  in the command output.
- Render timeout → configurable per command; kill the Blender process, surface partial state.

## Out of scope (explicitly)

- glTF/OBJ/FBX input (v2 candidates, see decision above)
- Enscape/Twinmotion-style interactive quality — this is batch/unattended rendering
- Reflecting the `bpy` API into skills (wrong altitude; generic 3D, not AECO-shaped)
- Any FloLess-specific behaviour in the agent (it consumes generic IFC, period)
