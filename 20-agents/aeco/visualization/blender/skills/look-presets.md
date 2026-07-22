---
name: blender-look-presets
description: This skill should be used when deciding how the `blender` agent shades an element — choosing between the clay, realistic and section-style presets, working out why a beam rendered as concrete or fell back to clay, reading a `scene.apply-look` receipt's `assigned` / `by-reason`, or extending the grade and class tables for a new vertical. Covers the grade → class → clay resolution order, longest-prefix matching, the EN 206 / EN 338 C-number collision, and why metals need a world gradient.
---

# Look presets

Nobody clicks materials in an unattended render, so the shading has to be deduced from
what the IFC already states: the product class and its associated material grade. That is
the mechanism that makes the whole chain autonomous — `scripts/_looks.py` is the design's
mapping table, executable.

## The three presets

| Preset | For | Character |
|---|---|---|
| `clay` | Form studies, massing, coordination reviews | Neutral matte. Everything is a slightly different warm grey; nothing reflects, nothing is transparent. Reads shape, not material. |
| `realistic` | Client-facing stills, marketing, hero shots | PBR. Steel is metallic and reflective, concrete is rough, glass is transparent, timber is brown. |
| `section-style` | Drawing-adjacent output, technical diagrams | Flat and near-Lambertian — roughness 0.95 on every opaque family, glass the only exception at 0.30 — with a dark blue for steel. The palette of a coloured section drawing rather than a photograph. |

`realistic` is the default on `scene.apply-look`, `render.still` and `render.turntable`.

Materials are shared, one Blender material per `(preset, family)` named
`AWARE_<preset>_<family>`, so a 10,000-object model carries five materials, not ten
thousand. `apply_look()` clears each object's slots before appending, so re-running
`scene.apply-look` with a different preset over the same `.blend` is a clean swap, not a
pile-up.

## Resolution order: grade → class → clay

```python
def family_for(ifc_class: str, material: str) -> tuple[str, str]:
```

1. **Grade.** The `ifc_material` string, uppercased and stripped, matched
   case-insensitively against `GRADE_FAMILIES` prefixes — **longest prefix wins**. Reason
   is `grade:<material>`.
2. **Class.** If the grade said nothing useful, `CLASS_FAMILIES[ifc_class]`. Reason is
   `class:<IfcClass>`.
3. **Clay.** Neither matched → family `default`, reason `fallback:clay`.

**The clay fallback must never fail.** An element from a class nobody anticipated, carrying
a grade string nobody has seen, renders neutral. It does not raise, it does not skip, and
it does not cost the caller the render they asked for. A picture with one grey object in it
is worth incomparably more than an error, because the caller can see the object and judge
for themselves.

The receipt reports both halves so a bad mapping is visible without opening the file:

```json
{
  "preset": "realistic",
  "assigned":  { "steel": 9 },
  "by-reason": { "grade": 9 }
}
```

`by-reason` counts the *kind* of decision — `grade`, `class`, `fallback`. A model that
comes back mostly `fallback` has a semantics problem upstream, not a shading problem: check
`by-material` on the import receipt first (see the `ifc-import-ifcopenshell` skill).

## The grade table as shipped

```python
GRADE_FAMILIES = {
    "steel": ("A992", "A500", "A36", "A572", "A53", "S355", "S275", "S235", "Q345"),
    "concrete": ("C20", "C25", "C30", "C35", "C40", "C50", "CONCRETE", "BETON"),
    "glass": ("GLASS", "GLAZING", "SZKLO"),
    "timber": (
        "TIMBER", "WOOD", "GL24", "GL28",
        "C14", "C16", "C18", "C20", "C22", "C24", "C27", "C30", "C35", "C40", "C45", "C50",
    ),
}
```

Prefix matching, not equality — real grade strings carry suffixes. `A500-GR.B` resolves
through `A500`; `C30/37` resolves through `C30`; `S355J2+N` resolves through `S355`.

Longest-prefix-wins is what decides when a grade matches more than one entry. In the table
as shipped the only live multi-match is the equal-length C-number tie below, which is why
that case needs a second tiebreak — but the rule is what keeps a future entry that extends
an existing prefix from being resolved by dict order instead of by specificity.

## The class table as shipped

```python
CLASS_FAMILIES = {
    "IfcColumn": "steel",  "IfcBeam": "steel",   "IfcMember": "steel",
    "IfcPlate": "steel",   "IfcMechanicalFastener": "steel",
    "IfcSlab": "concrete", "IfcWall": "concrete", "IfcWallStandardCase": "concrete",
    "IfcFooting": "concrete", "IfcPile": "concrete",
    "IfcWindow": "glass",  "IfcCurtainWall": "glass",
}
```

This is the *no-grade* fallback: what a framing member looks like when the producer told
us nothing about its material. Framing shapes default to steel because a bare
`IfcBeam` with no grade is, in practice, steel more often than anything else.

It has a second job — breaking grade ties. See below.

## The C-number collision, honestly

`C20`, `C30`, `C35`, `C40` and `C50` are legitimately **both** EN 206 concrete grades and
EN 338 structural-timber strength classes. Character for character, the same string. A
material graded plain `C30` cannot be resolved from the grade string alone.

Here is exactly what happens, and it is a deliberate choice, not a bug:

| Grade | Also a concrete grade? | Resolves to |
|---|---|---|
| `C14` `C16` `C18` `C22` `C24` `C27` `C45` | No | **timber**, on any IFC class |
| `C20` `C30` `C35` `C40` `C50` | Yes | **concrete**, regardless of IFC class |
| `C25` | Concrete only (no EN 338 C25) | **concrete** |
| `GL24` `GL28` `TIMBER` `WOOD` | No | **timber**, on any IFC class |

The mechanism: when the longest-matching prefix ties across families, `family_for` asks
`CLASS_FAMILIES` to break the tie, and uses the class's family **only if it is one of the
tied candidates**. For `IfcSlab` the class says concrete, which is in contention, so
concrete wins. For `IfcBeam` the class says *steel*, which is not in a concrete/timber tie
at all, so it cannot settle anything — and the fall-through takes the first tied family in
`GRADE_FAMILIES`'s declared order, which is concrete.

**No entry in `CLASS_FAMILIES` currently maps to timber.** That is why the five colliding
numbers resolve to concrete on every class, including framing members. Concrete beams and
columns are at least as common as timber ones, and a bare C-number is genuinely ambiguous,
so the commoner AECO reading wins.

### What a producer must emit to get timber on a framing member

An unambiguous signal — any one of these:

- `GL24` or `GL28` (glulam)
- `TIMBER` or `WOOD` anywhere at the **start** of the material name
- One of the seven non-colliding EN 338 classes: `C14`, `C16`, `C18`, `C22`, `C24`, `C27`,
  `C45`

Prefixing the family name is the most robust habit and sidesteps the collision entirely:
`Concrete C30/37` matches the `CONCRETE` prefix (8 characters) and beats both C-number
entries outright, so a producer that names materials that way is never ambiguous in either
direction.

**This is a known, deliberate limitation.** A glulam beam graded plain `C30` will render as
concrete, and the receipt will confidently report `grade:` as the reason — the most
convincing possible wrong answer. `tests/test_look_resolution.py` sweeps all twelve EN 338
classes against a concrete-decisive class, a framing class and no class at all, and pins
this exact shape, so a future change to either table has to touch that file on purpose.

## Metals need an environment

A `metallic 0.85` material — `realistic` steel — is almost entirely **specular**. It has
essentially no diffuse response, so what you see is nearly all what it reflects. In a
near-black world it reflects back as a flat dark object: no gradient across a curved or
angled face, no visible form, no readable material.

The `realistic` preset was genuinely bad until `setup_world()` in `render_still.py` gained
a neutral vertical gradient sky — a `Texture Coordinate → Separate XYZ → Map Range →
Color Ramp` chain that remaps the world-direction Z from `-1 … +1` to `0 … 1` and ramps
from `0.03` grey at the nadir to `0.95` at the zenith, with the horizon at the midpoint.
That gradient is what gives a metal surface something to reflect that varies with its
normal.

The gradient is strictly neutral — equal R/G/B at both stops — so it never tints the model
or reads as art direction. It is lighting infrastructure, and it is applied identically for
every preset, because on the staged-`.blend` path the preset is not knowable at render
time: the look was applied by a prior `scene.apply-look` call and is not recorded as scene
metadata.

**The general rule, for anyone extending `PALETTES`:** a new entry with a non-zero
`metallic` value will render as whatever the environment gives it. Check it against a real
render, not against the colour swatch — the base colour of a metal is close to irrelevant
compared to what is around it. Diffuse-dominant entries (`clay`, `section-style`, and the
non-steel `realistic` families) barely care.

## What the mapping gets right

- **Steel by grade.** ASTM (`A992`, `A500`, `A36`, `A572`, `A53`) and EN (`S355`, `S275`,
  `S235`) plus `Q345`, with suffixes tolerated.
- **The common no-grade case.** A Revit model of walls and slabs with no material data
  still renders as walls and slabs, not as clay.
- **Never failing.** Every object gets a material. There is no input that makes
  `apply_look` refuse.
- **Idempotence.** Re-running with a different preset is a clean swap.

## Where it is deliberately wrong

- **The five colliding C-numbers.** Documented above. Timber loses.
- **Composite elements report one material.** The import returns only the first leaf, so a
  composite wall is shaded by its first layer.
- **`IfcStyledItem` colours are not read.** The design lists a group-colour tint as a
  fallback; the shipped mapper works from class and grade only. A producer's authored
  colours do not reach the render.
- **`IfcPlate` defaults to steel, not glass.** A glazed plate needs a glass *grade* to land
  in the glazing family; only `IfcWindow` and `IfcCurtainWall` reach glass by class.
- **Family granularity is coarse.** Painted, galvanized and weathering steel are one
  family. Grade drives *family*, not finish.
- **A staged `.blend` carries no look until `scene.apply-look` runs.** `scene.import`
  writes IFC semantics, not shading — a freshly imported file rendered straight away shows
  Blender's default grey. On `render.still` / `render.turntable`, the `preset` input applies
  **only** on the `ifc-path` branch; on the `blend-path` branch it is ignored, because the
  look is expected to be baked in already.

## Extending for a new vertical

Four edits, in this order. Skipping the third is the mistake that bites: `_material_for`
does `PALETTES[preset][family]`, so a family that exists in the grade table but not in a
palette raises `KeyError` at render time, which surfaces to the caller as
`unexpected-error` with a traceback.

**1. Add the grade prefixes.** Position in the dict matters — declared order is the
last-resort tiebreak when two families tie for longest prefix, so put the new family after
the ones that should win a tie.

```python
GRADE_FAMILIES = {
    "steel": (...),
    "concrete": (...),
    "glass": (...),
    "timber": (...),
    "masonry": ("BRICK", "MASONRY", "AAC", "CEGLA"),   # new
}
```

**2. Add the class fallbacks**, for elements that arrive with no grade at all. This is also
the lever that lets a class win a grade tie — a `"IfcBeam": "timber"` entry would make
framing members resolve `C30` to timber, which is precisely why no such entry exists today.

```python
CLASS_FAMILIES = {
    ...,
    "IfcWall": "concrete",          # unchanged — masonry needs a grade to beat this
    "IfcCovering": "masonry",       # new
}
```

**3. Add the family to all three palettes** — `(base_colour_rgba, metallic, roughness)`:

```python
PALETTES = {
    "clay":          { ..., "masonry": ((0.67, 0.64, 0.61, 1.0), 0.0, 0.70) },
    "realistic":     { ..., "masonry": ((0.55, 0.34, 0.28, 1.0), 0.0, 0.80) },
    "section-style": { ..., "masonry": ((0.74, 0.56, 0.48, 1.0), 0.0, 0.95) },
}
```

Keep `metallic` at 0.0 unless the family really is a metal — and if it is not, re-read the
environment section above before shipping it.

**4. Pin the behaviour with a test.** `_looks.py` carries `from __future__ import
annotations` precisely so it can be imported with a bare `bpy` stub, which means the
resolver is testable as plain Python with no Blender at all:

```python
import sys, types
sys.path.insert(0, ".../blender/scripts")
sys.modules["bpy"] = types.ModuleType("bpy")
sys.modules["_ifc_import"] = types.ModuleType("_ifc_import")

import _looks

assert _looks.family_for("IfcWall", "BRICK")[0] == "masonry"
assert _looks.family_for("IfcCovering", "")[0] == "masonry"     # class fallback
assert _looks.family_for("IfcWall", "")[0] == "concrete"        # unchanged
assert _looks.family_for("IfcBeam", "A992")[0] == "steel"       # unchanged
```

Then run the existing suite — `tests/test_look_resolution.py` — to confirm the new prefixes
did not create a tie with an existing family. A new prefix that collides with a C-number,
or that is a prefix of an existing one, changes resolution for grades you were not thinking
about.

## Worked example

Two deliverables from one staged model, with different looks:

```yaml
nodes:
  - id: stage
    agent: blender
    command: scene.import
    inputs:
      ifc-path: '{{ inputs.model-ifc }}'
      blend-path: '{{ run.tmp-dir }}/model.blend'

  # Coordination review: form only, nothing that could be mistaken for a finish decision.
  - id: clay-look
    agent: blender
    command: scene.apply-look
    inputs:
      blend-path: '{{ nodes.stage.blend-path }}'
      preset: clay
      output-path: '{{ run.tmp-dir }}/model-clay.blend'

  - id: coordination-shot
    agent: blender
    command: render.still
    inputs:
      blend-path: '{{ nodes.clay-look.blend-path }}'
      output-path: '{{ inputs.out-dir }}/{{ run.date }}-coordination.png'
      quality: draft
      direction: iso

  # Client deck: the same geometry, shaded.
  - id: realistic-look
    agent: blender
    command: scene.apply-look
    inputs:
      blend-path: '{{ nodes.stage.blend-path }}'
      preset: realistic
      output-path: '{{ run.tmp-dir }}/model-realistic.blend'

  - id: hero
    agent: blender
    command: render.still
    inputs:
      blend-path: '{{ nodes.realistic-look.blend-path }}'
      output-path: '{{ inputs.out-dir }}/{{ run.date }}-hero.png'
      quality: production
      samples: 256
```

Note `output-path` on both look nodes: without it, `scene.apply-look` overwrites
`blend-path` in place, and the second look would be applied on top of the first rather than
to a fresh copy. The result would still be correct — `apply_look` clears slots before
assigning — but the two renders would no longer be reproducible from separate files.

Reading the receipts:

| `by-reason` | Reading |
|---|---|
| `{"grade": 9}` | Every element resolved from its material grade. Ideal. |
| `{"grade": 40, "class": 120}` | The producer emits materials for framing but not for walls and slabs — those took the class default. Usually fine. |
| `{"fallback": 160}` | Neither class nor grade landed. Something is wrong upstream: check `by-material` and `by-class` on the import receipt before touching this table. |
| `{"grade": 200}` on a timber-frame model rendering grey | The C-number collision. Ask the producer for `GL24`/`GL28`, a `TIMBER`/`WOOD` prefix, or one of the seven non-colliding EN 338 classes. |
