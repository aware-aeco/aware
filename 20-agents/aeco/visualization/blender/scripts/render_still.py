"""`render.still` -- a framed PNG of a staged .blend (or an IFC directly).

Run: blender -b -P render_still.py -- '{"blend-path":"m.blend","output-path":"o.png"}'
"""

import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import bpy  # noqa: E402

import _framing  # noqa: E402
import _ifc_import  # noqa: E402
import _looks  # noqa: E402
import _result  # noqa: E402

# Set from the Task 1 probe: Blender 4.2+ renamed EEVEE to BLENDER_EEVEE_NEXT.
# Resolved at runtime so one script works across the 4.x/5.x split.
def _eevee_engine() -> str:
    identifiers = {
        item.identifier
        for item in bpy.types.RenderSettings.bl_rna.properties["engine"].enum_items
    }
    for candidate in ("BLENDER_EEVEE_NEXT", "BLENDER_EEVEE"):
        if candidate in identifiers:
            return candidate
    raise _result.AwareBlenderError(
        _result.ERR_RENDER_FAILED,
        f"no EEVEE engine in this Blender; available: {sorted(identifiers)}",
    )


def setup_world(strength: float = 1.0) -> None:
    """A vertical grey gradient sky, not a flat colour.

    A flat single-colour world starves specular materials: a metal has
    almost no diffuse response, so what you see is nearly all what it
    reflects, and a flat dark environment reflects back as a flat dark
    object -- no gradient across a curved or angled face, no visible form.
    Clay and section-style are diffuse-dominant and barely need this;
    realistic steel (metallic 0.85) needs it to read as metal at all. The
    gradient is strictly neutral (equal R/G/B at both stops) so it never
    tints the model or reads as a "look" -- it is lighting infrastructure,
    not art direction, and applies the same way regardless of which preset
    is active (preset is not knowable here on the staged-.blend path: the
    look was applied by a prior `scene.apply-look` call and isn't recorded
    as scene metadata).
    """
    world = bpy.context.scene.world
    if world is None:
        world = bpy.data.worlds.new("AwareWorld")
        bpy.context.scene.world = world
    world.use_nodes = True
    nodes = world.node_tree.nodes
    links = world.node_tree.links
    background = nodes.get("Background")
    if background is None:
        return

    # Named and reused rather than always-`new`, so calling setup_world()
    # again in the same session rewires the same nodes instead of leaving
    # orphaned duplicates behind.
    coord = nodes.get("AwareSkyCoord") or nodes.new("ShaderNodeTexCoord")
    coord.name = "AwareSkyCoord"
    separate = nodes.get("AwareSkySeparateZ") or nodes.new("ShaderNodeSeparateXYZ")
    separate.name = "AwareSkySeparateZ"
    remap = nodes.get("AwareSkyRemap") or nodes.new("ShaderNodeMapRange")
    remap.name = "AwareSkyRemap"
    ramp_node = nodes.get("AwareSkyRamp") or nodes.new("ShaderNodeValToRGB")
    ramp_node.name = "AwareSkyRamp"

    # Z is a unit direction (-1 straight down .. +1 straight up); remap to
    # 0..1 so the ramp below spans the whole visible sky with the horizon
    # at the midpoint, rather than clamping the entire lower hemisphere to
    # a single flat colour.
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
    links.new(ramp_node.outputs["Color"], background.inputs["Color"])
    background.inputs["Strength"].default_value = strength


def setup_key_light() -> None:
    """One sun, angled to match the default `iso` camera direction."""
    if any(obj.type == "LIGHT" for obj in bpy.data.objects):
        return
    light_data = bpy.data.lights.new("AwareSun", type="SUN")
    light_data.energy = 3.0
    light_data.angle = 0.15
    light = bpy.data.objects.new("AwareSun", light_data)
    light.rotation_euler = (0.9, 0.0, -0.8)
    bpy.context.scene.collection.objects.link(light)


def load_scene(inputs: dict) -> None:
    """Open the staged .blend, or import an IFC and apply a look on the fly."""
    blend_path = inputs.get("blend-path")
    ifc_path = inputs.get("ifc-path")
    if not blend_path and not ifc_path:
        raise _result.AwareBlenderError(
            _result.ERR_INVALID_INPUTS,
            "one of `blend-path` or `ifc-path` is required",
        )
    # Both given is an error, not a silent preference. The manifest declares these
    # mutually exclusive, and quietly opening the staged .blend would render a stale
    # model while the caller believes they asked for the IFC. `scene.info` rejects
    # this the same way; render.turntable inherits the check through this loader.
    if blend_path and ifc_path:
        raise _result.AwareBlenderError(
            _result.ERR_INVALID_INPUTS,
            "`blend-path` and `ifc-path` are mutually exclusive; give exactly one",
        )

    if blend_path:
        if not os.path.exists(str(blend_path)):
            raise _result.AwareBlenderError(
                _result.ERR_BLEND_UNREADABLE, f".blend not found: {blend_path}"
            )
        bpy.ops.wm.open_mainfile(filepath=os.path.abspath(str(blend_path)))
        return

    if not os.path.exists(str(ifc_path)):
        raise _result.AwareBlenderError(
            _result.ERR_IFC_UNREADABLE, f"IFC not found: {ifc_path}"
        )
    preset = str(inputs.get("preset", "realistic"))
    if preset not in _looks.PRESETS:
        raise _result.AwareBlenderError(
            _result.ERR_INVALID_INPUTS,
            f"unknown preset `{preset}`; expected one of {sorted(_looks.PRESETS)}",
        )
    _ifc_import.clear_scene()
    _ifc_import.import_ifc(
        str(ifc_path), unit_scale=float(inputs.get("unit-scale", 1.0))
    )
    _looks.apply_look(preset)


def main(inputs: dict) -> dict:
    output_path = os.path.abspath(str(_result.require(inputs, "output-path")))
    quality = str(inputs.get("quality", "draft"))
    direction = str(inputs.get("direction", "iso"))
    width = int(inputs.get("width-pixels", 1920))
    height = int(inputs.get("height-pixels", 1080))
    samples = int(inputs.get("samples", 0))

    if quality not in ("draft", "production"):
        raise _result.AwareBlenderError(
            _result.ERR_INVALID_INPUTS,
            f"unknown quality `{quality}`; expected draft or production",
        )
    if direction not in _framing.DIRECTIONS:
        raise _result.AwareBlenderError(
            _result.ERR_INVALID_INPUTS,
            f"unknown direction `{direction}`; expected one of {sorted(_framing.DIRECTIONS)}",
        )
    if width < 1 or height < 1:
        raise _result.AwareBlenderError(
            _result.ERR_INVALID_INPUTS, "width-pixels and height-pixels must be >= 1"
        )

    load_scene(inputs)

    scene = bpy.context.scene
    scene.render.resolution_x = width
    scene.render.resolution_y = height
    scene.render.resolution_percentage = 100
    scene.render.image_settings.file_format = "PNG"
    scene.render.filepath = output_path

    if quality == "production":
        scene.render.engine = "CYCLES"
        scene.cycles.samples = samples or 128
    else:
        scene.render.engine = _eevee_engine()
        if hasattr(scene, "eevee") and hasattr(scene.eevee, "taa_render_samples"):
            scene.eevee.taa_render_samples = samples or 32

    setup_world()
    setup_key_light()

    camera = _framing.ensure_camera()
    try:
        framing = _framing.frame_camera(camera, direction)
    except ValueError as exc:
        raise _result.AwareBlenderError(_result.ERR_RENDER_FAILED, str(exc)) from exc

    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    bpy.ops.render.render(write_still=True)

    if not os.path.exists(output_path):
        raise _result.AwareBlenderError(
            _result.ERR_RENDER_FAILED,
            f"render completed but {output_path} was not written",
        )

    return {
        "path": output_path,
        "output-path": output_path,
        "size-bytes": os.path.getsize(output_path),
        "width-pixels": width,
        "height-pixels": height,
        "engine": scene.render.engine,
        "quality": quality,
        "framing": framing,
    }


if __name__ == "__main__":
    _result.run(main)
