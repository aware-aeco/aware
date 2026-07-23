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
import _stage  # noqa: E402


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


def _as_bool(value, default: bool = True) -> bool:
    """Coerce a JSON-ish input to a bool without surprising the caller.

    The transport hands inputs through as JSON, so `ground` normally arrives as a
    real bool -- but a caller templating `'{{ inputs.ground }}'` in a workflow
    gets a string, and `bool("false")` is True, which would silently ignore the
    one value they bothered to set.
    """
    if value is None:
        return default
    if isinstance(value, bool):
        return value
    if isinstance(value, (int, float)):
        return bool(value)
    return str(value).strip().lower() not in ("false", "0", "no", "off", "")


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
    direction = str(inputs.get("direction", "hero"))
    width = int(inputs.get("width-pixels", 1920))
    height = int(inputs.get("height-pixels", 1080))
    samples = int(inputs.get("samples", 0))
    environment = inputs.get("environment", _stage.DEFAULT_ENVIRONMENT)
    ground_enabled = _as_bool(inputs.get("ground"), default=True)

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
        _stage.tune_eevee(scene)

    environment_receipt = _stage.setup_world(environment)
    _stage.setup_key_light()

    camera = _framing.ensure_camera()
    try:
        framing = _framing.frame_camera(camera, direction)
    except ValueError as exc:
        raise _result.AwareBlenderError(_result.ERR_RENDER_FAILED, str(exc)) from exc

    # After framing, deliberately. The ground is sized from the model's bounds,
    # and it carries the aware-helper mark so `scene_bounds()` skips it -- which
    # means re-solving the fit with the ground present gives the same answer.
    # `tests/test_ground_isolation.py` pins exactly that, because if it ever
    # stopped holding every render would silently reframe.
    ground_receipt = _stage.setup_ground(ground_enabled)

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
        "environment": environment_receipt,
        "ground": ground_receipt,
    }


if __name__ == "__main__":
    _result.run(main)
