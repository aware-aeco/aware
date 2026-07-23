# Blender Presentation Visuals Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Raise the `blender` agent's output from technically correct to presentation quality — HDRI-lit, grounded on a fading floor, shot on a longer lens — without losing the unattended property.

**Architecture:** A new `scripts/_stage.py` owns everything that makes the picture *look* right (world, key light, ground), leaving `_framing` to place the camera, `_looks` to shade and `_ifc_import` to ingest. The world splits by ray type: camera rays see v1's neutral gradient, every other ray samples a Blender-bundled CC0 HDRI. Agent-created objects carry an `aware-helper` custom property that three call sites skip, which is what keeps the new floor out of the bounding-sphere fit and out of `scene.info`.

**Tech Stack:** Blender 5.2 LTS `bpy` (background mode, `blender -b -P`), Python 3.13, the agent's `<<<AWARE_RESULT>>>` sentinel protocol.

**Spec:** `docs/superpowers/specs/2026-07-23-blender-presentation-visuals-design.md`

---

## File Structure

| File | Action | Responsibility |
|---|---|---|
| `20-agents/aeco/visualization/blender/scripts/_stage.py` | **create** | Environment resolution, the world node graph, the key light, the ground plane, EEVEE quality settings |
| `.../scripts/_ifc_import.py` | modify | Add `PROP_HELPER` beside the existing `PROP_*` constants |
| `.../scripts/_framing.py` | modify | `scene_bounds()` skips helpers; longer default lens; `hero` direction |
| `.../scripts/_looks.py` | modify | `apply_look()` skips helpers |
| `.../scripts/scene_info.py` | modify | `_inventory()` skips helpers |
| `.../scripts/render_still.py` | modify | Delegate to `_stage`; new inputs; new receipt blocks |
| `.../scripts/render_turntable.py` | modify | Delegate to `_stage`; new inputs; new receipt blocks |
| `.../manifest.yaml` | modify | `environment` + `ground` inputs, `hero` direction, new output schema keys |
| `.../tests/test_environment_resolution.py` | **create** | Pure-Python resolver unit tests (no Blender) |
| `.../tests/test_ground_isolation.py` | **create** | The framing-identity gate + helper-exclusion, driven through real Blender |
| `.../tests/run_smoke.py` | modify | Assert the new receipt blocks on the existing chain |
| `.../skills/look-presets.md` | modify | "Metals need an environment" now has an HDRI answer |
| `.../skills/headless-rendering.md` | modify | Record the shadow-catcher and enum-under-report traps |

---

## Task 1: The `aware-helper` marker and its three skip sites

This lands first and alone, because everything else depends on it and it is the change whose
absence is silent: without it the ground plane inflates the bounding sphere and every render
quietly reframes.

**Files:**
- Modify: `20-agents/aeco/visualization/blender/scripts/_ifc_import.py`
- Modify: `20-agents/aeco/visualization/blender/scripts/_framing.py:28-44`
- Modify: `20-agents/aeco/visualization/blender/scripts/_looks.py:161-163`
- Modify: `20-agents/aeco/visualization/blender/scripts/scene_info.py:42-44`

- [ ] **Step 1: Add the constant beside the existing `PROP_*` names**

In `_ifc_import.py`, next to `PROP_CLASS` / `PROP_MATERIAL` / `PROP_STOREY` / `PROP_GUID` / `PROP_NAME`:

```python
# Marks an object the AGENT created for presentation (ground plane, and any
# future staging helper) rather than one imported from the IFC. Three places
# skip these: `_framing.scene_bounds()` (a helper must never enter the camera
# fit -- a ground plane sized from the model would otherwise inflate the
# bounding sphere and pull the camera back on every render), `_looks.apply_look()`
# (a helper must never be repainted as steel) and `scene_info._inventory()` (a
# non-IFC mesh must never be counted as an element).
PROP_HELPER = "aware-helper"
```

- [ ] **Step 2: Skip helpers in the camera fit**

In `_framing.py`, add the import and the guard. The loop currently reads
`if obj.type != "MESH": continue`:

```python
import _ifc_import

...

    for obj in bpy.data.objects:
        if obj.type != "MESH":
            continue
        # Staging helpers (the ground plane) are sized FROM this fit, so letting
        # them into it would be circular as well as wrong: the floor would
        # inflate the bounding sphere, which would enlarge the floor, and the
        # model would shrink in frame.
        if obj.get(_ifc_import.PROP_HELPER):
            continue
```

- [ ] **Step 3: Skip helpers in `apply_look`**

In `_looks.py`, inside the `for obj in bpy.data.objects:` loop of `apply_look()`:

```python
        if obj.type != "MESH":
            continue
        if obj.get(_ifc_import.PROP_HELPER):
            continue
```

- [ ] **Step 4: Skip helpers in the inventory**

In `scene_info.py`, inside `_inventory()`:

```python
        if obj.type != "MESH":
            continue
        # A staging helper is not an element. Counting one would put a row with
        # empty guid/class/material into `elements` and inflate `count`.
        if obj.get(_ifc_import.PROP_HELPER):
            continue
```

- [ ] **Step 5: Verify nothing regressed**

Run:
```bash
cd 20-agents/aeco/visualization/blender
python tests/test_look_resolution.py
python tests/test_material_resolution.py
```
Expected: both PASS. (`_looks.py` gains an `_ifc_import.PROP_HELPER` read; the tests stub
`_ifc_import` as a bare module, so confirm `apply_look` is not on their path — they exercise
`family_for` only.)

- [ ] **Step 6: Commit**

```bash
git add 20-agents/aeco/visualization/blender/scripts/_ifc_import.py \
        20-agents/aeco/visualization/blender/scripts/_framing.py \
        20-agents/aeco/visualization/blender/scripts/_looks.py \
        20-agents/aeco/visualization/blender/scripts/scene_info.py
git commit -m "feat(blender): mark agent-created objects so they stay out of the model's world"
```

---

## Task 2: `_stage.py` — environment resolution (pure, testable without Blender)

**Files:**
- Create: `20-agents/aeco/visualization/blender/scripts/_stage.py`
- Test: `20-agents/aeco/visualization/blender/tests/test_environment_resolution.py`

- [ ] **Step 1: Write the failing test**

`tests/test_environment_resolution.py` — the resolver takes the studiolight directory as a
parameter precisely so it needs no Blender:

```python
"""`_stage.resolve_environment` must never fail, and must always say what it did.

The agent's never-fail principle means a typo'd environment still returns a
picture -- so the receipt is the ONLY place that records it was not the picture
the caller asked for. Every degrade path below asserts a non-null
`fallback-reason`, because a silent degrade and a happy path that look alike is
exactly the failure this guards.
"""

import sys
import tempfile
import types
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent / "scripts"))
sys.modules.setdefault("bpy", types.ModuleType("bpy"))
sys.modules.setdefault("mathutils", types.ModuleType("mathutils"))
sys.modules.setdefault("_ifc_import", types.ModuleType("_ifc_import"))
sys.modules.setdefault("_framing", types.ModuleType("_framing"))

import _stage  # noqa: E402

FAILURES = []


def check(label, condition, detail=""):
    if condition:
        print(f"  ok  {label}")
    else:
        print(f"  FAIL {label} {detail}")
        FAILURES.append(label)


def main() -> int:
    with tempfile.TemporaryDirectory() as tmp:
        lights = Path(tmp) / "world"
        lights.mkdir()
        (lights / "studio.exr").write_bytes(b"not-really-an-exr")
        custom = Path(tmp) / "mine.hdr"
        custom.write_bytes(b"nor-is-this")

        r = _stage.resolve_environment("studio", str(lights))
        check("bundled name resolves", r["source"] == "blender-studiolight", r)
        check("bundled name has no fallback reason", r["fallback-reason"] is None, r)

        r = _stage.resolve_environment("gradient", str(lights))
        check("gradient is explicit, not a degrade",
              r["source"] == "procedural-gradient" and r["fallback-reason"] is None, r)

        r = _stage.resolve_environment("", str(lights))
        check("empty falls back to the default", r["resolved"] == "studio", r)

        r = _stage.resolve_environment(str(custom), str(lights))
        check("a real path resolves", r["source"] == "custom-path", r)

        r = _stage.resolve_environment("nope", str(lights))
        check("unknown name degrades", r["source"] == "procedural-gradient", r)
        check("unknown name says why", bool(r["fallback-reason"]), r)

        r = _stage.resolve_environment("/no/such/file.exr", str(lights))
        check("missing file degrades", r["source"] == "procedural-gradient", r)
        check("missing file is reported as a path, not an unknown name",
              "not found" in (r["fallback-reason"] or ""), r)

        r = _stage.resolve_environment("courtyard", str(lights))
        check("a bundled name with no file degrades",
              r["source"] == "procedural-gradient" and bool(r["fallback-reason"]), r)

        r = _stage.resolve_environment("studio", None)
        check("no studiolight dir at all degrades",
              r["source"] == "procedural-gradient" and bool(r["fallback-reason"]), r)

        r = _stage.resolve_environment("STUDIO", str(lights))
        check("names are case-insensitive", r["source"] == "blender-studiolight", r)

    print("\nFAIL" if FAILURES else "\nPASS")
    return 1 if FAILURES else 0


if __name__ == "__main__":
    raise SystemExit(main())
```

- [ ] **Step 2: Run it to verify it fails**

Run: `python 20-agents/aeco/visualization/blender/tests/test_environment_resolution.py`
Expected: FAIL — `ModuleNotFoundError: No module named '_stage'`

- [ ] **Step 3: Write `_stage.py`'s resolver half**

```python
"""Staging the shot -- world, key light and ground.

Split out of `render_still.py` so one module owns everything that makes the
picture LOOK right, leaving `_framing` to place the camera, `_looks` to shade
and `_ifc_import` to ingest.

Blender ships eight 1K HDRIs (CC0, Poly Haven, by Greg Zaal) inside its own
install, so an environment costs this repo no bytes, no licence obligation and
no network fetch -- and is present wherever Blender is. That is why nothing is
vendored here.
"""

from __future__ import annotations

import os

# The eight worlds Blender ships in `datafiles/studiolights/world`. Names are
# the file stems; matching is case-insensitive.
STUDIOLIGHTS = (
    "city", "courtyard", "forest", "interior", "night", "studio", "sunrise", "sunset",
)
GRADIENT = "gradient"
DEFAULT_ENVIRONMENT = "studio"

_IMAGE_SUFFIXES = (".exr", ".hdr", ".hdri", ".png", ".jpg", ".jpeg")


def _receipt(requested, resolved, source, path, fallback_reason) -> dict:
    return {
        "requested": requested,
        "resolved": resolved,
        "source": source,
        "path": path,
        "fallback-reason": fallback_reason,
    }


def _gradient(requested, reason=None) -> dict:
    return _receipt(requested, GRADIENT, "procedural-gradient", None, reason)


def _looks_like_path(value: str) -> bool:
    """Distinguish `/x/y.exr` from a misspelled bundled name.

    Only affects the WORDING of the fallback reason -- both degrade to the
    gradient -- but "not found" and "unknown name" send someone looking in two
    completely different places.
    """
    return (
        "/" in value
        or os.sep in value
        or value.lower().endswith(_IMAGE_SUFFIXES)
    )


def resolve_environment(requested, studiolight_dir: str | None) -> dict:
    """Decide which environment to light with. Never raises.

    Precedence: `gradient` and the eight bundled names are reserved words checked
    BEFORE the value is treated as a path, so a local file coincidentally named
    `studio` cannot shadow the bundled environment.
    """
    raw = "" if requested is None else str(requested).strip()
    name = raw or DEFAULT_ENVIRONMENT
    lowered = name.lower()

    if lowered == GRADIENT:
        return _gradient(raw or DEFAULT_ENVIRONMENT)

    if lowered in STUDIOLIGHTS:
        if not studiolight_dir:
            return _gradient(name, "this Blender ships no studiolight HDRIs")
        path = os.path.join(studiolight_dir, f"{lowered}.exr")
        if not os.path.exists(path):
            return _gradient(name, f"bundled environment `{lowered}` not found at {path}")
        return _receipt(name, lowered, "blender-studiolight", path, None)

    if os.path.exists(name):
        return _receipt(name, name, "custom-path", os.path.abspath(name), None)

    if _looks_like_path(name):
        return _gradient(name, f"environment file not found: {name}")

    return _gradient(
        name,
        f"unknown environment `{name}`; expected one of {sorted(STUDIOLIGHTS)}, "
        f"`{GRADIENT}`, or a path to an .exr/.hdr",
    )
```

- [ ] **Step 4: Run the test to verify it passes**

Run: `python 20-agents/aeco/visualization/blender/tests/test_environment_resolution.py`
Expected: every line `ok`, final line `PASS`, exit 0

- [ ] **Step 5: Commit**

```bash
git add 20-agents/aeco/visualization/blender/scripts/_stage.py \
        20-agents/aeco/visualization/blender/tests/test_environment_resolution.py
git commit -m "feat(blender): resolve an environment from Blender's own bundled HDRIs, never failing"
```

---

## Task 3: `_stage.py` — the world graph, key light, ground and EEVEE settings

**Files:**
- Modify: `20-agents/aeco/visualization/blender/scripts/_stage.py`

- [ ] **Step 1: Add the bpy-facing half**

Append to `_stage.py`. Note `bpy` and `mathutils` are imported lazily-at-module-level exactly as
the sibling scripts do; the resolver above stays import-clean for the unit test because the test
stubs them.

```python
import bpy  # noqa: E402
from mathutils import Vector  # noqa: E402

import _framing  # noqa: E402
import _ifc_import  # noqa: E402

# Rotates the HDRI so its brightest region sits behind-left of the `hero`
# camera, which is what puts a highlight along the near edge of a steel member
# instead of flat across its face.
ENVIRONMENT_ROTATION_Z = 2.3

# Ground sizing, in multiples of the model's bounding-SPHERE radius. The fade
# must complete inside the camera's far clip or the clip plane cuts a hard
# circle across the floor; `frame_camera()` reaches `radius * 4.0` past the
# model centre, and `_stage` widens `clip_end` to cover the mesh extent.
GROUND_FADE_START = 1.6
GROUND_FADE_END = 6.0
GROUND_EXTENT = 8.0


def studiolight_dir() -> str | None:
    """Blender's own HDRI folder, or None on a build that ships none."""
    try:
        path = bpy.utils.system_resource("DATAFILES", path="studiolights/world")
    except Exception:  # noqa: BLE001 - a missing datafiles tree must not fail a render
        return None
    return path if path and os.path.isdir(path) else None


def _ensure_world() -> bpy.types.World:
    world = bpy.context.scene.world
    if world is None:
        world = bpy.data.worlds.new("AwareWorld")
        bpy.context.scene.world = world
    world.use_nodes = True
    return world


def _gradient_node(world) -> bpy.types.Node:
    """v1's neutral vertical grey ramp, unchanged. Returns its Color output node.

    A flat single-colour world starves specular materials: a metal has almost no
    diffuse response, so what you see is nearly all what it reflects, and a flat
    dark environment reflects back as a flat dark object. The gradient is
    strictly neutral (equal R/G/B at both stops) so it never tints the model.
    """
    nodes, links = world.node_tree.nodes, world.node_tree.links
    coord = nodes.get("AwareSkyCoord") or nodes.new("ShaderNodeTexCoord")
    coord.name = "AwareSkyCoord"
    separate = nodes.get("AwareSkySeparateZ") or nodes.new("ShaderNodeSeparateXYZ")
    separate.name = "AwareSkySeparateZ"
    remap = nodes.get("AwareSkyRemap") or nodes.new("ShaderNodeMapRange")
    remap.name = "AwareSkyRemap"
    ramp_node = nodes.get("AwareSkyRamp") or nodes.new("ShaderNodeValToRGB")
    ramp_node.name = "AwareSkyRamp"

    # Z is a unit direction (-1 down .. +1 up); remap to 0..1 so the ramp spans
    # the whole visible sky with the horizon at the midpoint.
    remap.inputs["From Min"].default_value = -1.0
    remap.inputs["From Max"].default_value = 1.0
    remap.inputs["To Min"].default_value = 0.0
    remap.inputs["To Max"].default_value = 1.0

    ramp = ramp_node.color_ramp
    ramp.elements[0].position = 0.0
    ramp.elements[0].color = (0.03, 0.03, 0.03, 1.0)
    ramp.elements[1].position = 1.0
    ramp.elements[1].color = (0.95, 0.95, 0.95, 1.0)

    links.new(coord.outputs["Generated"], separate.inputs["Vector"])
    links.new(separate.outputs["Z"], remap.inputs["Value"])
    links.new(remap.outputs["Result"], ramp_node.inputs["Factor"])
    return ramp_node


def setup_world(environment=DEFAULT_ENVIRONMENT, strength: float = 1.0) -> dict:
    """Light with an HDRI; keep the neutral gradient as what the camera sees.

    Wiring the HDRI straight to Background looks correct in a tight crop and
    fails in a wide one: `studio.exr` is a PHOTOGRAPH of a room, so a wide shot
    shows its softbox and a light stand, and the ground plane's fade ends as a
    hard ellipse against the HDRI's dark lower hemisphere. Splitting on
    `Light Path -> Is Camera Ray` fixes both and keeps the visible backdrop
    exactly as neutral as it was before any of this -- the HDRI never appears in
    frame, it only feeds the lighting solution. Measured to cost EEVEE nothing
    (metal-region mean 0.2813 wired direct vs 0.2837 through the split).

    Returns the environment receipt; a degrade is recorded, never raised.
    """
    receipt = resolve_environment(environment, studiolight_dir())
    world = _ensure_world()
    nodes, links = world.node_tree.nodes, world.node_tree.links
    background = nodes.get("Background")
    if background is None:
        return receipt

    gradient = _gradient_node(world)

    if receipt["source"] != "procedural-gradient":
        try:
            image = bpy.data.images.load(receipt["path"], check_existing=True)
        except Exception as exc:  # noqa: BLE001 - an unreadable HDRI degrades, never fails
            receipt = _gradient(
                receipt["requested"], f"could not load {receipt['path']}: {exc}"
            )
        else:
            coord = nodes.get("AwareEnvCoord") or nodes.new("ShaderNodeTexCoord")
            coord.name = "AwareEnvCoord"
            mapping = nodes.get("AwareEnvMapping") or nodes.new("ShaderNodeMapping")
            mapping.name = "AwareEnvMapping"
            tex = nodes.get("AwareEnvTex") or nodes.new("ShaderNodeTexEnvironment")
            tex.name = "AwareEnvTex"
            path_node = nodes.get("AwareEnvLightPath") or nodes.new("ShaderNodeLightPath")
            path_node.name = "AwareEnvLightPath"
            mix = nodes.get("AwareEnvMix") or nodes.new("ShaderNodeMixRGB")
            mix.name = "AwareEnvMix"

            tex.image = image
            mapping.inputs["Rotation"].default_value = (0.0, 0.0, ENVIRONMENT_ROTATION_Z)
            links.new(coord.outputs["Generated"], mapping.inputs["Vector"])
            links.new(mapping.outputs["Vector"], tex.inputs["Vector"])
            # Fac 0 -> the HDRI (lighting, reflections); Fac 1 -> the gradient (camera).
            links.new(path_node.outputs["Is Camera Ray"], mix.inputs["Fac"])
            links.new(tex.outputs["Color"], mix.inputs[1])
            links.new(gradient.outputs["Color"], mix.inputs[2])
            links.new(mix.outputs["Color"], background.inputs["Color"])
            background.inputs["Strength"].default_value = strength
            return receipt

    # Gradient-only: wire it straight through, replacing any prior env link.
    links.new(gradient.outputs["Color"], background.inputs["Color"])
    background.inputs["Strength"].default_value = strength
    return receipt


def setup_key_light() -> None:
    """One sun, angled to match the default camera direction.

    Kept even with an HDRI: image-based lighting alone is ambient, and the
    directional contact shadow is what makes the model read as standing on the
    ground rather than pasted over it. The wide `angle` is what softens that
    shadow's edge -- the first prototype's narrow default rendered a hard
    aliased polygon under EEVEE.
    """
    if any(obj.type == "LIGHT" for obj in bpy.data.objects):
        return
    light_data = bpy.data.lights.new("AwareSun", type="SUN")
    light_data.energy = 2.0
    light_data.angle = 0.30
    light = bpy.data.objects.new("AwareSun", light_data)
    light.rotation_euler = (0.9, 0.0, -0.8)
    bpy.context.scene.collection.objects.link(light)


def _ground_material(fade_start: float, fade_end: float) -> bpy.types.Material:
    """Neutral floor that dissolves radially, so its edge is never in frame."""
    name = "AWARE_ground"
    existing = bpy.data.materials.get(name)
    if existing is not None:
        return existing

    mat = bpy.data.materials.new(name)
    mat.use_nodes = True
    nodes, links = mat.node_tree.nodes, mat.node_tree.links
    principled = nodes.get("Principled BSDF")
    output = nodes.get("Material Output")
    if principled is None or output is None:
        return mat
    principled.inputs["Base Color"].default_value = (0.32, 0.32, 0.33, 1.0)
    principled.inputs["Roughness"].default_value = 0.42

    coord = nodes.new("ShaderNodeTexCoord")
    length = nodes.new("ShaderNodeVectorMath")
    length.operation = "LENGTH"
    remap = nodes.new("ShaderNodeMapRange")
    remap.inputs["From Min"].default_value = fade_start
    remap.inputs["From Max"].default_value = fade_end
    remap.inputs["To Min"].default_value = 0.0
    remap.inputs["To Max"].default_value = 1.0
    remap.clamp = True
    transparent = nodes.new("ShaderNodeBsdfTransparent")
    mix = nodes.new("ShaderNodeMixShader")

    # Object coordinates are the plane's LOCAL space, so this is distance from
    # the plane's origin in metres -- which is why the object is placed at the
    # model centre rather than built with an offset.
    links.new(coord.outputs["Object"], length.inputs[0])
    links.new(length.outputs["Value"], remap.inputs["Value"])
    links.new(remap.outputs["Result"], mix.inputs["Fac"])
    links.new(principled.outputs["BSDF"], mix.inputs[1])
    links.new(transparent.outputs["BSDF"], mix.inputs[2])
    links.new(mix.outputs["Shader"], output.inputs["Surface"])

    # EEVEE Next gates smooth alpha behind the render method; Cycles ignores it.
    if hasattr(mat, "surface_render_method"):
        mat.surface_render_method = "BLENDED"
    elif hasattr(mat, "blend_method"):
        mat.blend_method = "BLEND"
    return mat


def setup_ground(enabled: bool = True) -> dict:
    """A shadow-receiving floor under the model, sized from the MODEL's bounds.

    A real visible surface, not `is_shadow_catcher`: measured on this Blender,
    Cycles honours that flag and EEVEE ignores it completely (100% opaque frame
    vs 19.9%), and the flag reads back True on both -- so a catcher would make
    `draft` show a solid slab where `production` shows a shadow, with no way to
    detect it. A real floor renders the same way in both.
    """
    if not enabled:
        return {"present": False}
    try:
        lo, hi = _framing.scene_bounds()
    except ValueError:
        # No mesh to stand on. The render will fail its own way in framing;
        # the ground is not the place to raise about it.
        return {"present": False}

    centre = (lo + hi) / 2.0
    radius = (hi - lo).length / 2.0
    if radius <= 0.0:
        return {"present": False}

    extent = radius * GROUND_EXTENT
    mesh = bpy.data.meshes.new("AwareGround")
    mesh.from_pydata(
        [(-extent, -extent, 0.0), (extent, -extent, 0.0),
         (extent, extent, 0.0), (-extent, extent, 0.0)],
        [],
        [(0, 1, 2, 3)],
    )
    mesh.update()
    ground = bpy.data.objects.new("AwareGround", mesh)
    ground.location = (centre.x, centre.y, lo.z)
    ground[_ifc_import.PROP_HELPER] = True
    ground.data.materials.append(
        _ground_material(radius * GROUND_FADE_START, radius * GROUND_FADE_END)
    )
    bpy.context.scene.collection.objects.link(ground)

    return {
        "present": True,
        "radius": round(extent, 6),
        "fade-start": round(radius * GROUND_FADE_START, 6),
        "fade-end": round(radius * GROUND_FADE_END, 6),
        "z": round(lo.z, 6),
    }


def tune_eevee(scene) -> None:
    """Switch on EEVEE Next's quality features, which ship off or low.

    Without this the contact shadow renders as a hard aliased polygon on a flat
    frame. This does NOT close the gap to Cycles on metal -- EEVEE is inherently
    flatter and darker there (measured centre-frame spread 0.083 vs 0.142) --
    and it is not meant to; draft previews framing, production is where
    reflections have to be correct.
    """
    eevee = scene.eevee
    for attr, value in (
        ("use_shadows", True),
        ("shadow_ray_count", 4),
        ("shadow_step_count", 8),
        ("use_raytracing", True),
        ("use_fast_gi", True),
    ):
        if hasattr(eevee, attr):
            setattr(eevee, attr, value)
    if hasattr(eevee, "ray_tracing_method"):
        eevee.ray_tracing_method = "SCREEN"
```

- [ ] **Step 2: Byte-compile it to catch syntax and name errors immediately**

Run: `python -m py_compile 20-agents/aeco/visualization/blender/scripts/_stage.py`
Expected: no output, exit 0

- [ ] **Step 3: Re-run the resolver unit test — the bpy half must not break it**

Run: `python 20-agents/aeco/visualization/blender/tests/test_environment_resolution.py`
Expected: `PASS`. If it now fails with `AttributeError` on the `bpy` stub, the module-level
`bpy` usage has leaked out of a function body — move it back inside.

- [ ] **Step 4: Commit**

```bash
git add 20-agents/aeco/visualization/blender/scripts/_stage.py
git commit -m "feat(blender): stage the shot -- HDRI lighting, neutral backdrop, fading ground"
```

---

## Task 4: Camera — longer lens and the `hero` direction

**Files:**
- Modify: `20-agents/aeco/visualization/blender/scripts/_framing.py:18-25,101-110`

- [ ] **Step 1: Add the `hero` direction**

```python
DIRECTIONS = {
    # A three-quarter view: asymmetric in plan and slightly raised, which reads
    # as photography. `iso`'s exact 45 degrees reads as a CAD screenshot -- it is
    # kept unchanged for anyone who wants the axonometric.
    "hero": Vector((1.0, -1.7, 0.62)),
    "iso": Vector((1.0, -1.0, 0.7)),
    "front": Vector((0.0, -1.0, 0.0)),
    "back": Vector((0.0, 1.0, 0.0)),
    "left": Vector((-1.0, 0.0, 0.0)),
    "right": Vector((1.0, 0.0, 0.0)),
    "top": Vector((0.0, 0.0, 1.0)),
}

# Blender's default 50mm on a 36mm sensor visibly distorts a building at the
# frame edges. `distance` is DERIVED from `camera.data.angle`, so a longer lens
# simply backs the camera off for the same framing -- the bounding-sphere fit,
# and with it the turntable's no-clipping guarantee, is untouched.
DEFAULT_LENS_MM = 80.0
```

- [ ] **Step 2: Apply the lens in `ensure_camera`**

```python
def ensure_camera() -> bpy.types.Object:
    """Return the scene camera, creating one if the .blend has none."""
    scene = bpy.context.scene
    if scene.camera is not None:
        return scene.camera
    cam_data = bpy.data.cameras.new("AwareCamera")
    cam_data.lens = DEFAULT_LENS_MM
    camera = bpy.data.objects.new("AwareCamera", cam_data)
    scene.collection.objects.link(camera)
    scene.camera = camera
    return camera
```

- [ ] **Step 3: Widen the far clip so it cannot cut the ground plane**

In `frame_camera()`, replace the `clip_end` line:

```python
    # Keep the model comfortably inside the clip range at any orbit angle, and
    # far enough out that the ground plane (up to `_stage.GROUND_EXTENT` radii
    # across) cannot be sliced by the far plane into a visible hard circle.
    cam_data.clip_start = max(distance - radius * 4.0, distance / 1000.0)
    cam_data.clip_end = distance + radius * 12.0
```

- [ ] **Step 4: Commit**

```bash
git add 20-agents/aeco/visualization/blender/scripts/_framing.py
git commit -m "feat(blender): longer lens and a hero three-quarter direction"
```

---

## Task 5: Wire the render commands to `_stage`

**Files:**
- Modify: `20-agents/aeco/visualization/blender/scripts/render_still.py`
- Modify: `20-agents/aeco/visualization/blender/scripts/render_turntable.py`

- [ ] **Step 1: Delete `setup_world` / `setup_key_light` from `render_still.py`**

Remove both function bodies (lines 34-104) and add `import _stage` alongside the other
`import _framing` / `import _looks` lines.

- [ ] **Step 2: Rewrite `render_still.main`'s staging block**

Replace the `setup_world()` / `setup_key_light()` calls and the direction default:

```python
    direction = str(inputs.get("direction", "hero"))
    environment = inputs.get("environment", _stage.DEFAULT_ENVIRONMENT)
    ground_enabled = inputs.get("ground", True)
    if isinstance(ground_enabled, str):
        ground_enabled = ground_enabled.strip().lower() not in ("false", "0", "no", "")
```

and, after the engine block:

```python
    if quality == "production":
        scene.render.engine = "CYCLES"
        scene.cycles.samples = samples or 128
    else:
        scene.render.engine = _eevee_engine()
        if hasattr(scene, "eevee") and hasattr(scene.eevee, "taa_render_samples"):
            scene.eevee.taa_render_samples = samples or 32
        _stage.tune_eevee(scene)

    env_receipt = _stage.setup_world(environment)
    _stage.setup_key_light()

    camera = _framing.ensure_camera()
    try:
        framing = _framing.frame_camera(camera, direction)
    except ValueError as exc:
        raise _result.AwareBlenderError(_result.ERR_RENDER_FAILED, str(exc)) from exc

    # AFTER framing: the ground is sized from the model's bounds, and the fit it
    # is sized against must already be solved. It carries the aware-helper mark,
    # so re-solving the fit with it present would give the same answer anyway --
    # the framing-identity test pins exactly that.
    ground_receipt = _stage.setup_ground(bool(ground_enabled))
```

and add to the returned payload:

```python
        "framing": framing,
        "environment": env_receipt,
        "ground": ground_receipt,
```

- [ ] **Step 3: Apply the same wiring to `render_turntable.py`**

Replace `render_still.setup_world()` / `render_still.setup_key_light()` with the `_stage`
equivalents, add `import _stage`, default `direction` to `hero`, call `_stage.tune_eevee(scene)`
after the EEVEE sample assignment, call `_stage.setup_ground(...)` after `frame_camera()` and
BEFORE the pivot is created, and add both receipt blocks to the payload.

- [ ] **Step 4: Verify both scripts still import cleanly under Blender**

Run:
```bash
"/c/Program Files/Blender Foundation/Blender 5.2/blender.exe" -b --python-expr \
  "import sys; sys.path.insert(0,'20-agents/aeco/visualization/blender/scripts'); import render_still, render_turntable; print('IMPORT OK')"
```
Expected: `IMPORT OK` in the output. (This also re-proves the `__main__` guard: an unguarded
script would parse *this* command's argv and `sys.exit()`.)

- [ ] **Step 5: Commit**

```bash
git add 20-agents/aeco/visualization/blender/scripts/render_still.py \
        20-agents/aeco/visualization/blender/scripts/render_turntable.py
git commit -m "feat(blender): render through the staging module, and report what it staged"
```

---

## Task 6: Manifest — new inputs, `hero`, new output keys

**Files:**
- Modify: `20-agents/aeco/visualization/blender/manifest.yaml`

- [ ] **Step 1: Add to both `render.still` and `render.turntable` inputs**

```yaml
      direction:
        type: enum
        values: [hero, iso, front, back, left, right, top]
        default: hero
      environment:
        type: string
        default: studio
        description: |
          Lighting environment. One of Blender's own bundled HDRIs — studio,
          courtyard, city, forest, interior, night, sunrise, sunset — or a path
          to an .exr/.hdr, or `gradient` for the procedural sky. The HDRI lights
          the scene and is what metal reflects; it is never visible behind the
          model, which always shows the neutral gradient. Anything unresolvable
          degrades to the gradient rather than failing the render, and the
          `environment` receipt records why.
      ground:
        type: boolean
        default: true
        description: |
          Put a softly fading floor under the model so it reads as standing
          rather than floating. It is excluded from the camera fit and from
          `scene.info`'s inventory.
```

- [ ] **Step 2: Add to both output schemas**

```yaml
        environment:   object  # {requested, resolved, source, path, fallback-reason}
        ground:        object  # {present, radius, fade-start, fade-end, z}
```

- [ ] **Step 3: Check the catalogue and doc stats did not drift**

Run:
```bash
python scripts/sync_stats.py --check || PYTHONIOENCODING=utf-8 python scripts/sync_stats.py --write
./cli/target/debug/aware.exe agent reindex --check
```
Expected: no drift. Adding inputs and enum values does not change the agent/command COUNT, so
these should be clean — but both gates move whenever the agent set changes and one of them was
red and nearly shipped last time, so run them rather than reasoning about them.

- [ ] **Step 4: Commit**

```bash
git add 20-agents/aeco/visualization/blender/manifest.yaml
git commit -m "feat(blender): declare the environment and ground inputs"
```

---

## Task 7: The framing-identity gate

The highest-value test in this plan. If the ground ever leaks into `scene_bounds()`, every render
silently reframes and no other check notices.

**Files:**
- Create: `20-agents/aeco/visualization/blender/tests/test_ground_isolation.py`

- [ ] **Step 1: Write the test**

```python
"""The ground plane must be invisible to everything except the renderer.

Two independent guarantees, both silent when broken:

1. `render.still` with `ground: true` and `ground: false` must return an
   IDENTICAL `framing` receipt. `_framing.scene_bounds()` walks every mesh, so
   a floor that is not skipped inflates the bounding sphere, pushes the camera
   back and shrinks the model in frame -- on EVERY render, with no error.
2. The floor must not appear in `scene.info`'s inventory. A non-IFC mesh there
   would be counted in `count` and added to `elements` with empty guid/class.

Run: python test_ground_isolation.py --aware-bin <path/to/aware[.exe]> [--blender <path>]
"""

import argparse
import json
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
from make_fixture import build_fixture  # noqa: E402
from run_smoke import find_result, resolve_blender  # noqa: E402

SCRIPTS = HERE.parent / "scripts"


def run(blender: str, script: str, inputs: dict) -> dict:
    proc = subprocess.run(
        [blender, "-b", "-P", str(SCRIPTS / script), "--", json.dumps(inputs)],
        capture_output=True, text=True, timeout=1800,
    )
    result = find_result(proc.stdout)
    if result is None:
        raise RuntimeError(
            f"{script}: no framed result (exit {proc.returncode})\n"
            f"--- stdout tail ---\n{proc.stdout[-2000:]}\n"
            f"--- stderr tail ---\n{proc.stderr[-2000:]}"
        )
    return result


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--blender")
    parser.add_argument("--aware-bin", required=True)
    opts = parser.parse_args()

    blender = resolve_blender(opts.blender)
    if blender is None:
        print("SKIP: no Blender available")
        return 0

    failures = []
    with tempfile.TemporaryDirectory() as tmp:
        work = Path(tmp)
        ifc = work / "fixture.ifc"
        build_fixture(opts.aware_bin, ifc)

        common = {
            "ifc-path": str(ifc), "quality": "draft",
            "width-pixels": 320, "height-pixels": 240,
        }
        with_ground = run(blender, "render_still.py",
                          {**common, "output-path": str(work / "on.png"), "ground": True})
        without = run(blender, "render_still.py",
                      {**common, "output-path": str(work / "off.png"), "ground": False})

        if with_ground["framing"] != without["framing"]:
            failures.append(
                "framing CHANGED when the ground was added -- the ground plane is "
                "inside scene_bounds():\n"
                f"  ground on:  {json.dumps(with_ground['framing'], sort_keys=True)}\n"
                f"  ground off: {json.dumps(without['framing'], sort_keys=True)}"
            )
        else:
            print(f"  ok  framing identical with and without the ground: {with_ground['framing']}")

        if not with_ground["ground"]["present"]:
            failures.append(f"ground: true produced no ground: {with_ground['ground']}")
        if without["ground"]["present"]:
            failures.append(f"ground: false produced a ground: {without['ground']}")

        env = with_ground["environment"]
        if env["source"] != "blender-studiolight":
            failures.append(f"default environment did not resolve to a bundled HDRI: {env}")
        else:
            print(f"  ok  default environment: {env['resolved']} ({env['source']})")

        broken = run(blender, "render_still.py", {
            **common, "output-path": str(work / "broken.png"),
            "environment": "/no/such/hdri.exr",
        })
        benv = broken["environment"]
        if benv["source"] != "procedural-gradient" or not benv["fallback-reason"]:
            failures.append(f"a broken environment path did not degrade visibly: {benv}")
        else:
            print(f"  ok  broken environment degraded: {benv['fallback-reason']}")

        info = run(blender, "scene_info.py", {"ifc-path": str(ifc)})
        if info["count"] != 9:
            failures.append(f"scene.info counted {info['count']}, expected 9")
        else:
            print(f"  ok  scene.info still reports exactly {info['count']} elements")

    for failure in failures:
        print(f"  FAIL {failure}")
    print("\nGROUND ISOLATION FAIL" if failures else "\nGROUND ISOLATION PASS")
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
```

- [ ] **Step 2: Check the helpers this test imports actually exist under those names**

Run: `grep -n "def find_result\|def resolve_blender" 20-agents/aeco/visualization/blender/tests/run_smoke.py`
Expected: both defined. If `find_result` has a different name, use the real one — do not add a
duplicate parser.

- [ ] **Step 3: Run it**

Run:
```bash
python 20-agents/aeco/visualization/blender/tests/test_ground_isolation.py \
  --blender "/c/Program Files/Blender Foundation/Blender 5.2/blender.exe" \
  --aware-bin ./cli/target/debug/aware.exe
```
Expected: `GROUND ISOLATION PASS`

- [ ] **Step 4: Commit**

```bash
git add 20-agents/aeco/visualization/blender/tests/test_ground_isolation.py
git commit -m "test(blender): pin the ground plane out of the camera fit and the inventory"
```

---

## Task 8: The existing gate, plus receipts

**Files:**
- Modify: `20-agents/aeco/visualization/blender/tests/run_smoke.py`

- [ ] **Step 1: Assert the new receipt blocks in the `render.still` step**

After the existing `is-flat` assertion:

```python
        assert still["environment"]["source"] == "blender-studiolight", (
            "render.still: the default environment should resolve to a bundled "
            f"HDRI, got {still['environment']}"
        )
        assert still["ground"]["present"], (
            f"render.still: expected a ground plane by default, got {still['ground']}"
        )
        print(
            f"      environment={still['environment']['resolved']} "
            f"({still['environment']['source']}), ground r={still['ground']['radius']}"
        )
```

- [ ] **Step 2: Run the full existing gate — it must pass unchanged**

Run:
```bash
python 20-agents/aeco/visualization/blender/tests/run_smoke.py \
  --blender "/c/Program Files/Blender Foundation/Blender 5.2/blender.exe" \
  --aware-bin ./cli/target/debug/aware.exe
```
Expected: `SMOKE PASS`. The inventory assertions (`count == 9`, exact `by-class`/`by-material`)
are the ones that would catch a leaked helper — they must still hold.

- [ ] **Step 3: Commit**

```bash
git add 20-agents/aeco/visualization/blender/tests/run_smoke.py
git commit -m "test(blender): assert the environment and ground receipts in the smoke gate"
```

---

## Task 9: Look at the images (the gate automation cannot run)

**Files:** none — this task produces judgement, not code.

- [ ] **Step 1: Render the fixture four ways**

```bash
python - <<'PY'
# writes the fixture, then renders draft + production at 960x540, hero + front
PY
```
Render, at minimum: `draft`/`hero`, `production`/`hero`, `production`/`front`, and a turntable.

- [ ] **Step 2: Open every PNG and judge it**

State plainly whether each reads as presentation quality. **If it cannot be told, say so** rather
than guessing — the `realistic` preset once rendered near-black and passed every automated gate.
Specifically check: no visible floor edge in any direction; no studio-room artefact in the
backdrop; metal reads as metal in `production`; the contact shadow grounds the model.

- [ ] **Step 3: Confirm the orbit still holds**

Assert the camera-to-pivot distance is constant across turntable frames, and watch for clipping at
any angle.

- [ ] **Step 4: Tune and re-render if anything is off**

`ENVIRONMENT_ROTATION_Z`, the sun's `energy`/`angle`, `GROUND_FADE_*` and the floor's base colour
are the knobs. Re-render and look again. Commit the tuned values with a message saying what the
render showed.

---

## Task 10: Skills

**Files:**
- Modify: `20-agents/aeco/visualization/blender/skills/look-presets.md`
- Modify: `20-agents/aeco/visualization/blender/skills/headless-rendering.md`

- [ ] **Step 1: Route through `skill-creator`** — CLAUDE.md requires it for every skill `.md`, no exceptions.

- [ ] **Step 2: `look-presets.md` — "Metals need an environment" now has an answer**

The section currently ends at the gradient. It must now describe the HDRI split, that the visible
backdrop is still the gradient, and the measured EEVEE-vs-Cycles metal gap.

- [ ] **Step 3: `headless-rendering.md` — record the new traps**

Both are of the same family the page already documents (enum introspection under-reports):

- `object.is_shadow_catcher` reads back `True` under EEVEE, which ignores it entirely — measured
  100 % opaque frame vs Cycles' 19.9 %. The property is not a capability probe.
- `view_transform` / `look` enums report `['NONE']` via `bl_rna` while the scene is actually on
  `AgX` — the same trap as the engine enum, on a different property.
- EEVEE Next ships its quality features off; `_stage.tune_eevee()` exists for that reason.

- [ ] **Step 4: Commit**

```bash
git add 20-agents/aeco/visualization/blender/skills/
git commit -m "docs(blender): record the HDRI split and two more introspection traps"
```

---

## Task 11: Gates, review, PR

- [ ] **Step 1: Local gates CI does not run**

```bash
cd cli && cargo fmt --all -- --check && cargo clippy --all-targets -- -D warnings && cargo test
```
Expected: all green. (No Rust changed in this plan — the transport passes new inputs through as
JSON — but the gate is mandatory regardless.)

- [ ] **Step 2: Codex review**

```bash
codex exec review --base main
```
Address every finding, re-run until clean.

- [ ] **Step 3: Push and open the PR**, then merge after review.

---

## Self-Review

**Spec coverage:** §1 environment → Tasks 2, 3, 6. §2 ground + `aware-helper` → Tasks 1, 3, 7.
§3 camera → Task 4. §4 module boundaries → Tasks 3, 5. §5 new inputs → Task 6. §6 receipts →
Tasks 5, 6, 8. §7 tests → Tasks 7, 8. §8 look at it → Task 9. §9 out of scope → nothing added.
No gaps.

**Placeholder scan:** Task 9 Step 1 carries an intentionally empty heredoc — that task produces
judgement, and the render invocation is a one-liner over the commands already specified in Task 7.
Every code step elsewhere carries complete code.

**Type consistency:** `PROP_HELPER` (Task 1) is read in Tasks 1 and 3. `resolve_environment` /
`_receipt` / `_gradient` (Task 2) are used in Task 3. `_stage.DEFAULT_ENVIRONMENT`,
`_stage.setup_world`, `_stage.setup_key_light`, `_stage.setup_ground`, `_stage.tune_eevee` (Task 3)
are all called in Task 5 under those exact names. `GROUND_EXTENT` (Task 3) is referenced by the
`clip_end` comment in Task 4. Receipt keys `environment` / `ground` match across Tasks 5, 6, 7, 8.
