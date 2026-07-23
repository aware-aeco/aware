# Presentation visuals for the `blender` agent — approved design

**Date:** 2026-07-23
**Status:** approved (Pawel, 2026-07-23)
**Follows:** `2026-07-22-blender-visualization-agent-design.md` (the shipped v1) and
`docs/superpowers/handoffs/2026-07-22-blender-presentation-visuals.md` (the handoff)

v1 shipped a **technically correct** render: correct geometry, correct materials from IFC
semantics, a neutral studio background. This raises the output to **presentation quality**
without losing the unattended property.

## What must not move

Three properties the shipped agent guarantees. Every change below is shaped to preserve them,
and each has a test in §7.

1. **The camera fit stays rotation-invariant.** `_framing` fits the model's bounding *sphere*,
   which is what stops `render.turntable` clipping at some angle the framing pass never sampled.
2. **Nothing new can fail a render.** A missing, misspelled or unreadable environment degrades to
   the existing procedural gradient and still produces a picture. The never-fail principle already
   runs through this agent (`_looks`' clay fallback); the environment joins it.
3. **`draft` previews `production`.** EEVEE and Cycles must show the same picture, just rougher.
   This is what kills the shadow-catcher option — see §2.

## Verified facts (probed against the installed Blender, not assumed)

Measured on Blender 5.2.0 LTS, bundled Python 3.13.13, this session.

| Fact | Value | Consequence |
|---|---|---|
| Blender ships HDRIs | 8 × 1K `.exr` in `datafiles/studiolights/world/`, ~1.9 MB total | The sourcing fork dissolves — nothing to bundle, fetch or license |
| Their licence | **CC0**, Poly Haven, by Greg Zaal (`license.txt`) | No attribution obligation, no repo bytes, redistribution is Blender's problem not ours |
| Discovery API | `bpy.utils.system_resource("DATAFILES", path="studiolights/world")` | Deterministic; does not depend on user preferences the way `preferences.studio_lights` does |
| `ShaderNodeTexSky` | present; `SINGLE_SCATTERING` / `MULTIPLE_SCATTERING` / `PREETHAM` / `HOSEK_WILKIE` | A procedural physical sky exists, but is **not** used — see §1's rejected alternative |
| `object.is_shadow_catcher` | present, and **reads back `True` on both engines** | The property is *not* a capability probe |
| Shadow catcher, rendered | Cycles 19.9 % opaque frame; **EEVEE 100 %** | EEVEE ignores the flag entirely and renders the plane solid. Only a render reveals this |
| `view_transform` / `look` enums | `bl_rna` reports `['NONE']` while the scene is actually on `AgX` | Same under-reporting trap as the engine enum — never build an availability check on either |
| Active view transform | `AgX` | Modern tonemapping is already in place; highlights roll off rather than clipping |
| EEVEE quality knobs | `use_raytracing`, `ray_tracing_method`, `use_shadows`, `shadow_ray_count` | Real reflection/shadow levers exist on the draft path |
| Manifest input types | `type: boolean` with `default:` is established in sibling agents | `ground` can be a plain boolean input |
| `Material.blend_method` | still present on 5.2 (`OPAQUE`/`CLIP`/`HASHED`/`BLEND`) alongside the new `surface_render_method` (`DITHERED`/`BLENDED`) | `_looks.py`'s existing `blend_method = "BLEND"` for glass is **not** a latent crash — checked because EEVEE Next was thought to have removed it |
| `ray_tracing_method` | `PROBE` / `SCREEN` | EEVEE's raytracer is off by default and must be switched on explicitly |
| EEVEE vs Cycles on metal | centre-frame spread 0.083 vs 0.142; mean 0.28 vs 0.37 | EEVEE renders metal inherently flatter and darker. Not a bug to fix — a gap to document |
| The Light-Path split's cost to EEVEE | metal mean 0.2813 direct vs 0.2837 split | Free. The split does not poison EEVEE's reflection probe, which was the suspicion |

The shadow-catcher row is the load-bearing one, and it is the kind of fact this agent keeps
paying for: **the flag reads back `True` on an engine that ignores it.** No property
introspection distinguishes the two engines. Only rendering the scene and measuring the frame
does.

## 1. Environment lighting

### The HDRI lights the scene; the gradient stays the backdrop

**Revised after prototyping — the original draft wired the HDRI straight to `Background.Color`,
and rendering it exposed two failures a close-up shot hid entirely:**

1. **`studio.exr` is a photograph of a real room.** A wide render shows the softbox fixture and a
   light stand. As a visible backdrop it is unusable.
2. **The floor's fade ended as a hard ellipse** — a grey disc floating against the HDRI's dark
   lower hemisphere.

Both are fixed by the standard product-viz split, wired with `Light Path → Is Camera Ray` into a
`Mix` in front of `Background.Color`:

| Ray | Sees | Purpose |
|---|---|---|
| Camera | **the neutral gradient** (v1's node chain, unchanged) | the visible backdrop |
| Everything else (diffuse, glossy, transmission) | **the HDRI** | lighting and reflections |

This is strictly better than the approved draft, and it **strengthens rather than weakens the v1
"lighting infrastructure, not art direction" position**: what the viewer actually *sees* behind the
model is still exactly v1's neutral grey ramp, on every environment setting. The HDRI never appears
in frame — it only feeds the lighting solution. `sunset` now warms the light on the model instead
of pasting a sunset photograph behind it, which is what an architectural presentation actually
wants. The floor's fade also now dissolves into the gradient instead of into the HDRI's dark
underside, which is what removed the ellipse.

**Measured cost to EEVEE: none.** Metal-region mean luminance came back 0.2813 with the HDRI wired
direct and 0.2837 through the split — identical within noise. The split was suspected of poisoning
EEVEE's reflection probe; it does not.

### Resolution order

`setup_world()` gains a resolution order in front of today's procedural gradient. The gradient
code itself is unchanged and becomes both the camera-ray backdrop and the no-HDRI fallback leg.

| `environment` input | Resolves to | `source` in the receipt |
|---|---|---|
| `studio` (**new default**) | bundled `studio.exr` — a neutral grey softbox room | `blender-studiolight` |
| `courtyard` `city` `forest` `interior` `night` `sunrise` `sunset` | the matching bundled HDRI | `blender-studiolight` |
| a filesystem path | that `.exr` / `.hdr` | `custom-path` |
| `gradient` | today's procedural sky, explicitly asked for | `procedural-gradient` |
| anything unresolvable | today's procedural sky, **with the reason recorded** | `procedural-gradient` |

Precedence is that table's order, top to bottom: `gradient` and the eight bundled names are
reserved words checked *before* the value is treated as a path, so a local file coincidentally
named `studio` never shadows the bundled environment. Give it a path with a separator or an
extension to reach the `custom-path` branch.

The sun stays — it is what casts the directional contact shadow the ground plane needs — but its
energy is retuned now that the HDRI carries part of the lighting load. **That number is settled by
looking at renders, not by picking one**; the value that ships is whichever one looks right in §8.

### This does change the "lighting infrastructure, not art direction" position

The v1 design is explicit that the gradient world is deliberately neutral and preset-agnostic, and
the handoff requires any change to that to be justified rather than slipped in. The justification:

- **The default stays neutral.** `studio.exr` is a grey softbox room with no colour cast. It does
  not tint the model, so the *default* keeps the v1 philosophy intact and only stops starving
  specular materials of anything to reflect. A `metallic 0.85` steel renders as what it reflects;
  a real environment is strictly more to reflect than a vertical grey ramp.
- **Art direction becomes opt-in, and named.** `sunset` genuinely does tint the model orange. That
  is now a thing a caller asks for by name, in a receipt-visible way — not something the agent
  decides on their behalf.
- **Preset-agnosticism is preserved.** The environment is still applied identically regardless of
  which look preset is active, because on the `blend-path` branch the preset remains unknowable at
  render time (the look was applied by a prior `scene.apply-look` and is not recorded as scene
  metadata). Nothing about that reasoning changes.

### Rejected alternative: procedural physical sky

`ShaderNodeTexSky` exists here and would need no file at all. Rejected because a physical sky is
an *outdoor daylight* model — it is scenery, and a strongly directional, blue-tinted one. It would
impose more art direction than the neutral studio HDRI it replaces, while being harder to reason
about (turbidity, ozone, sun elevation). The bundled studio HDRI is both more neutral and simpler.

## 2. Ground plane and contact shadows

A neutral plane at the model's base, sized from the model-only bounds, with its material fading
radially to transparent so the edge is never visible — at any of the six directions and at every
turntable angle. Radial symmetry is what makes it safe under the orbit.

**A real, visible surface — not a shadow catcher.** `is_shadow_catcher` is a Cycles-only feature
here (measured: §"Verified facts"), so a catcher-based floor would make `draft` show a large
opaque slab where `production` shows a model floating over its own shadow. That breaks guarantee 3
outright, and the property gives no way to detect the difference and compensate. A real floor
renders the same way in both engines.

### The `aware-helper` marker — one mechanism, three call sites

This is the change that matters most, because getting it wrong is silent and ruins every render:
`_framing.scene_bounds()` walks **every** mesh object, so an added floor inflates the bounding
sphere and yanks the camera backwards, shrinking the model in frame.

Agent-created objects get an `aware-helper` custom property, and three places skip them:

| Call site | Why |
|---|---|
| `_framing.scene_bounds()` | **Load-bearing.** Keeps the fit computed from the model alone, so framing is bit-identical with the floor present or absent |
| `scene_info._inventory()` | Keeps a non-IFC mesh out of `count` / `elements` — this also closes the flag the v1 reviewer already raised about a non-IFC mesh being counted with empty fields |
| `_looks.apply_look()` | Defensive: a floor must never be repainted as steel by a future call-ordering change |

The constant lives beside the existing `PROP_*` names in `_ifc_import.py`, which both `_looks` and
`scene_info` already import — it is exactly "a custom property key on scene objects", which is what
that block of constants is.

### EEVEE needs switching on

The first prototype rendered EEVEE's contact shadow as a **hard aliased polygon** on a completely
flat frame, against Cycles' soft shadow and tonal floor. EEVEE Next ships its quality features off
or low, so the draft path sets them explicitly: `use_shadows`, `shadow_ray_count`,
`shadow_step_count`, `use_raytracing` with `ray_tracing_method = "SCREEN"`, and `use_fast_gi`.
The sun also gets a wider `angle`, which is what softens the shadow edge in both engines.

**What that does not close, honestly:** EEVEE still renders metal flatter and darker than Cycles
(measured above). That is the rasterizer, not a setting, and it is consistent with what the shipped
`headless-rendering` skill already says — draft is for iterating and previewing framing; production
is where reflections have to be correct. The gap gets documented in the skill rather than papered
over.

### Sizing and the far clip

The floor is sized from the model radius, and its fade must complete **inside the camera's far
clip**, or the clip plane cuts a hard circle across it. `frame_camera()` currently sets
`clip_end = distance + radius * 4.0`, which reaches `4r` past the model centre. The floor's mesh
extent and fade radius are chosen against that budget, and `clip_end` is widened to cover the floor
where needed — the depth-precision ratio stays far inside what a 24-bit buffer tolerates.

## 3. Camera

Two changes. Both leave the bounding-sphere solve untouched, so guarantee 1 is structurally
preserved rather than merely tested.

- **A longer lens** (Blender's default 50 mm → ~70–85 mm on the 36 mm sensor). `distance` is
  *derived* from `camera.data.angle`, so the fit adapts on its own and the framing is unchanged in
  extent — the building simply stops looking wide-angle-distorted at the edges.
- **A new `hero` direction, as the new default.** Today's `iso` is `(1, -1, 0.7)` — dead symmetric
  in plan, which reads as a CAD screenshot. `hero` is asymmetric in plan and slightly raised, which
  reads as photography. **`iso` stays byte-identical** for anyone who wants the axonometric.

**This changes output for existing callers who never set `direction`.** Accepted deliberately:
the agent shipped one day before this change (v0.102.0, 2026-07-22), adoption is effectively zero,
and shipping the better default now is cheaper than migrating later. Callers who want the old
angle name it explicitly.

### Deliberately not doing

- **No depth of field.** On a whole building it reads as a tilt-shift toy, and on a turntable the
  focus distance would have to be re-solved per frame — which is exactly the class of per-frame
  re-solve the single-solve turntable design exists to avoid.
- **No look-at offset.** Aiming above the bounding-box centre is the standard compositional move,
  but it eats asymmetrically into the `margin: 1.10` slack that guarantee 1 lives in. The floor and
  the environment do the compositional work instead, at zero risk to the orbit.

## 4. Module boundaries

`render_still.py` is 222 lines and would reach ~400 carrying environment resolution and the ground.
World, key light and ground move to a new **`scripts/_stage.py`**, giving one owner per concern:

| Module | Owns |
|---|---|
| `_ifc_import.py` | ingest — geometry and IFC semantics |
| `_looks.py` | shading — semantic material assignment |
| `_stage.py` | **staging the shot — world, key light, ground** (new) |
| `_framing.py` | the camera — fit and placement |
| `_result.py` | the sentinel protocol and named errors |

`render_still.py` and `render_turntable.py` both call `_stage`; the turntable keeps importing
`render_still` for `load_scene` and `_eevee_engine`.

## 5. New inputs

Added to `render.still` and `render.turntable`. **Neither is required, and both have a working
default** — the unattended property is preserved.

| Input | Type | Default | Notes |
|---|---|---|---|
| `environment` | `string` | `studio` | A bundled name, a file path, or `gradient`. Not an `enum`, because a path must be accepted; the receipt reports which branch resolved |
| `ground` | `boolean` | `true` | Turns the floor off for a cutout-style render |

The transport passes inputs through as JSON, so **no Rust change is needed** for either.

## 6. Receipts — how a silent degrade becomes visible

The never-fail principle means a typo'd HDRI path still returns a picture. The receipt is then the
*only* place that says it was not the picture that was asked for, so both render commands gain:

```json
"environment": {
  "requested": "studo",
  "resolved": "gradient",
  "source": "procedural-gradient",
  "path": null,
  "fallback-reason": "unknown environment `studo`; expected a bundled name, a file path, or `gradient`"
},
"ground": { "present": true, "radius": 42.4, "z": 0.0 }
```

`fallback-reason` is `null` on the happy path. This follows the house style already set by
`skipped` / `excluded` on the import receipt: two different outcomes must never look identical.

## 7. How each guarantee is tested

| Guarantee | Test |
|---|---|
| The floor never moves the camera | **`render.still` with `ground: true` and `ground: false` must return an identical `framing` receipt.** This is the highest-value new assertion — it pins the exact failure that would silently degrade every render |
| Nothing new fails a render | A deliberately-broken `environment` path returns `ok: true` with `source: procedural-gradient` and a non-null `fallback-reason` |
| Environment resolution | Unit test over the resolver: bundled name → path, `gradient` → procedural, unknown → procedural with a reason, missing file → procedural with a reason. Kept pure (the studiolight directory is a parameter) so it needs no Blender |
| Helpers stay out of the inventory | The floor must not appear in `scene.info`'s `count` / `elements` |
| The existing gate | `tests/run_smoke.py` passes unchanged — B must not regress A |
| The orbit still holds | Camera-to-pivot distance constant across turntable frames |

## 8. The verification that automation cannot do

**Numeric checks cannot validate presentation quality.** The `realistic` preset once rendered
near-black and passed every automated gate — non-flat, model centred, correct dimensions. Only
looking at the image caught it. Two further traps from that episode: a corner-sampled "background
estimate" silently breaks once the background is a gradient (it misclassified ~73 % of the frame as
model — use an alpha-silhouette `film_transparent` companion render to measure the model region),
and "low variance = murky" was the wrong proxy — mean luminance was the real signal.

So the final gate is: **render the fixture at 960×540 in both `draft` and `production`, open the
PNGs, and judge them.** State plainly whether they read as presentation quality — and if that
cannot be told, say so rather than guessing. That honesty is the point of the exercise, not a
formality.

## 9. Out of scope

- Any change to the IFC import, the look presets' palettes, or the grade/class mapping tables.
- Bundling HDRI assets in this repo (unnecessary — Blender ships them).
- Exposing environment rotation as an input. The graph carries a mapping node with a chosen default
  so rotation is a one-line change later; the named environments are the knob for now.
