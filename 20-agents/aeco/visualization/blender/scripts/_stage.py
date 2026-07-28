"""Staging the shot -- world, key light and ground.

Split out of `render_still.py` so one module owns everything that makes the
picture LOOK right, leaving `_framing` to place the camera, `_looks` to shade and
`_ifc_import` to ingest.

Blender ships eight 1K HDRIs (CC0, Poly Haven, by Greg Zaal) inside its own
install, under `datafiles/studiolights/world`. So an environment costs this repo
no bytes, no licence obligation and no network fetch, and is present wherever
Blender is -- which is why nothing is vendored here and nothing is downloaded.

Everything in this module degrades rather than raises. A missing HDRI, an
unreadable one, a Blender build that ships none, a scene with no geometry to
stand on -- each falls back to something that still renders, and says so in the
receipt it returns. That matches the agent's existing posture (`_looks`' clay
fallback): a picture with one thing wrong in it is worth incomparably more than
an error, because the caller can see it and judge for themselves.
"""

from __future__ import annotations

import os

import bpy

import _framing
import _ifc_import

# The eight worlds Blender ships. Names are the file stems, matched
# case-insensitively.
STUDIOLIGHTS = (
    "city",
    "courtyard",
    "forest",
    "interior",
    "night",
    "sunrise",
    "sunset",
    "studio",
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
    """Tell `/x/y.exr` from a misspelled bundled name.

    Only changes the WORDING of the fallback reason -- both degrade to the
    gradient -- but "not found" and "unknown name" send someone looking in two
    completely different places.
    """
    return "/" in value or os.sep in value or value.lower().endswith(_IMAGE_SUFFIXES)


def resolve_environment(requested, studiolight_dir: str | None) -> dict:
    """Decide which environment to light with. Never raises.

    Precedence: `gradient` and the eight bundled names are reserved words,
    checked BEFORE the value is treated as a path, so a local file coincidentally
    named `studio` cannot shadow the bundled environment. Reach the path branch
    with a separator or an image extension.
    """
    raw = "" if requested is None else str(requested).strip()
    name = raw or DEFAULT_ENVIRONMENT
    lowered = name.lower()

    if lowered == GRADIENT:
        return _gradient(name)

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


# Rotates the HDRI so its brightest region sits behind-left of the `hero` camera,
# which puts a highlight along the near edge of a steel member rather than flat
# across its face.
ENVIRONMENT_ROTATION_Z = 2.3

# Ground sizing, in multiples of the model's bounding-SPHERE radius (the same
# radius `_framing` fits to). The fade has to complete inside the camera's far
# clip or the clip plane slices a hard circle across the floor -- `_framing`
# widens `clip_end` to cover `GROUND_EXTENT` for exactly that reason.
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


def _gradient_node(world):
    """Build v1's neutral vertical grey ramp and return its output node.

    Unchanged from the shipped agent, and still doing its original job: a flat
    single-colour world starves specular materials, because a metal has almost
    no diffuse response and reflects a flat dark environment back as a flat dark
    object. The ramp is strictly neutral -- equal R/G/B at both stops -- so it
    never tints the model or reads as a look.

    Nodes are named and reused rather than always-`new`, so calling this twice in
    one session rewires the same nodes instead of leaving orphaned duplicates.
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

    # Z is a unit direction (-1 straight down .. +1 straight up); remap to 0..1
    # so the ramp spans the whole visible sky with the horizon at the midpoint,
    # rather than clamping the entire lower hemisphere to one flat colour.
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
    fails in a wide one. `studio.exr` is a PHOTOGRAPH of a room, so a wide shot
    shows its softbox and a light stand standing behind the building, and the
    ground plane's fade ends as a hard ellipse against the HDRI's dark lower
    hemisphere.

    Splitting on `Light Path -> Is Camera Ray` fixes both: camera rays see the
    gradient, every other ray samples the HDRI. The visible backdrop therefore
    stays exactly as neutral as it was before image-based lighting existed here
    -- the HDRI never appears in frame, it only feeds the lighting solution, so
    `sunset` warms the light on the model instead of pasting a sunset behind it.
    Measured to cost EEVEE nothing: metal-region mean luminance came back 0.2813
    with the HDRI wired direct and 0.2837 through the split, which cleared the
    suspicion that the split poisons EEVEE's reflection probe.

    Returns the environment receipt. A degrade is recorded in it, never raised.
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
            # Blender 5.2 converts the HDRI's brightest excess into a synthetic
            # world sun. Its one-pixel PCF default aliases into regular dashes
            # along thin I-section webs; four pixels removes the pattern while
            # retaining the world sun's directional contact shadows (#314).
            if hasattr(world, "sun_shadow_filter_radius"):
                world.sun_shadow_filter_radius = 4.0
            coord = nodes.get("AwareEnvCoord") or nodes.new("ShaderNodeTexCoord")
            coord.name = "AwareEnvCoord"
            mapping = nodes.get("AwareEnvMapping") or nodes.new("ShaderNodeMapping")
            mapping.name = "AwareEnvMapping"
            tex = nodes.get("AwareEnvTex") or nodes.new("ShaderNodeTexEnvironment")
            tex.name = "AwareEnvTex"
            light_path = nodes.get("AwareEnvLightPath") or nodes.new("ShaderNodeLightPath")
            light_path.name = "AwareEnvLightPath"
            mix = nodes.get("AwareEnvMix") or nodes.new("ShaderNodeMixRGB")
            mix.name = "AwareEnvMix"

            tex.image = image
            mapping.inputs["Rotation"].default_value = (0.0, 0.0, ENVIRONMENT_ROTATION_Z)
            links.new(coord.outputs["Generated"], mapping.inputs["Vector"])
            links.new(mapping.outputs["Vector"], tex.inputs["Vector"])
            # Fac 0 -> the HDRI (lighting, reflections). Fac 1 -> the gradient
            # (what the camera sees).
            links.new(light_path.outputs["Is Camera Ray"], mix.inputs["Fac"])
            links.new(tex.outputs["Color"], mix.inputs[1])
            links.new(gradient.outputs["Color"], mix.inputs[2])
            links.new(mix.outputs["Color"], background.inputs["Color"])
            background.inputs["Strength"].default_value = strength
            return receipt

    # Gradient only. Linking here replaces any environment link on the same
    # input, so a session that already staged an HDRI rewires cleanly.
    links.new(gradient.outputs["Color"], background.inputs["Color"])
    background.inputs["Strength"].default_value = strength
    return receipt


def setup_key_light() -> None:
    """One sun, angled to match the default camera direction.

    Kept even with an HDRI in play: image-based lighting on its own is ambient,
    and the directional contact shadow is what makes the model read as standing
    on the ground rather than pasted over it.

    `energy` drops from v1's 3.0 because the HDRI now carries part of the load
    and the sun is no longer the only thing lighting the model. `angle` stays at
    v1's 0.15: a wider sun was tried, on the theory that it would soften the
    shadow edge, and measured on the fixture at 0.30 it was indistinguishable
    (floor spread 0.2204 against 0.2244) -- so it is left alone rather than
    changed for a benefit that could not be shown.
    """
    if any(obj.type == "LIGHT" for obj in bpy.data.objects):
        return
    light_data = bpy.data.lights.new("AwareSun", type="SUN")
    light_data.energy = 2.0
    light_data.angle = 0.15
    light = bpy.data.objects.new("AwareSun", light_data)
    light.rotation_euler = (0.9, 0.0, -0.8)
    bpy.context.scene.collection.objects.link(light)


def _ground_material(fade_start: float, fade_end: float):
    """A neutral floor that dissolves radially, so its edge is never in frame."""
    name = "AWARE_ground"
    existing = bpy.data.materials.get(name)
    if existing is not None:
        return existing

    material = bpy.data.materials.new(name)
    material.use_nodes = True
    nodes, links = material.node_tree.nodes, material.node_tree.links
    principled = nodes.get("Principled BSDF")
    output = nodes.get("Material Output")
    if principled is None or output is None:
        return material

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

    # `Object` output is the plane's LOCAL space, so this length is distance from
    # the plane's origin in metres. That is why the object is positioned at the
    # model centre rather than built with the offset baked into its vertices.
    links.new(coord.outputs["Object"], length.inputs[0])
    links.new(length.outputs["Value"], remap.inputs["Value"])
    links.new(remap.outputs["Result"], mix.inputs["Fac"])
    links.new(principled.outputs["BSDF"], mix.inputs[1])
    links.new(transparent.outputs["BSDF"], mix.inputs[2])
    links.new(mix.outputs["Shader"], output.inputs["Surface"])

    # EEVEE Next gates smooth alpha behind the render method; Cycles ignores it.
    # Both properties exist on 5.2 -- `blend_method` was NOT removed -- so prefer
    # the current one and keep the older name as the 4.x path.
    if hasattr(material, "surface_render_method"):
        material.surface_render_method = "BLENDED"
    elif hasattr(material, "blend_method"):
        material.blend_method = "BLEND"
    return material


def setup_ground(enabled: bool = True) -> dict:
    """A shadow-receiving floor under the model, sized from the MODEL's bounds.

    A real visible surface, not `is_shadow_catcher`. Measured on this Blender:
    Cycles honours that flag (19.9% opaque frame) and EEVEE ignores it entirely
    (100% opaque, the plane rendered solid) -- while the flag reads back True on
    BOTH, so nothing short of rendering distinguishes them. A catcher would
    therefore make `draft` show a solid slab where `production` shows a model
    over its own shadow, with no way to detect it. A real floor renders the same
    way in both engines.
    """
    if not enabled:
        return {"present": False}
    try:
        lo, hi = _framing.scene_bounds()
    except ValueError:
        # Nothing to stand on. Framing will raise about the empty scene on its
        # own terms; the ground is not the place to duplicate that error.
        return {"present": False}

    centre = (lo + hi) / 2.0
    radius = (hi - lo).length / 2.0
    if radius <= 0.0:
        return {"present": False}

    extent = radius * GROUND_EXTENT
    mesh = bpy.data.meshes.new("AwareGround")
    mesh.from_pydata(
        [
            (-extent, -extent, 0.0),
            (extent, -extent, 0.0),
            (extent, extent, 0.0),
            (-extent, extent, 0.0),
        ],
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
    """Guarantee EEVEE's shadows are on. Deliberately nothing else.

    A much larger version of this function was written and then deleted: it set
    `use_raytracing`, `ray_tracing_method = "SCREEN"`, `use_fast_gi`,
    `shadow_ray_count = 4` and `shadow_step_count = 8`. Measured A/B against the
    real fixture, all of it together left the render **identical to four decimal
    places** (floor-region mean 0.7327 either way). PROBE-vs-SCREEN tracing,
    `shadow_resolution_scale = 2.0` and a finer `shadow_maximum_resolution` were
    measured too, and changed nothing visible. None of it is shipped, because
    settings whose only effect is to look diligent in a diff are worse than no
    settings at all -- they invite the next person to build on a false premise.

    What survives is the one line with a real failure mode behind it: the agent
    deliberately runs WITHOUT `--factory-startup`, so the user's startup file is
    loaded and `use_shadows` is not guaranteed on. Shadowless drafts would be a
    genuine, silent degradation.

    What none of this closes is the gap to Cycles on metal: on the fixture,
    steel-only mean luminance is 0.2705 under EEVEE against 0.4774 under Cycles
    -- EEVEE renders it 43% darker. That is the rasterizer, not a setting, and
    it matches what this agent already documents: draft previews framing,
    production is where reflections have to be correct.
    """
    eevee = getattr(scene, "eevee", None)
    if eevee is not None and hasattr(eevee, "use_shadows"):
        eevee.use_shadows = True
