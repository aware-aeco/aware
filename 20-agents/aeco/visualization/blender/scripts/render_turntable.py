"""`render.turntable` -- a 360-degree orbit MP4 around the model (EEVEE).

The camera is parented to an empty at the model centre and the EMPTY rotates, so
the framing solved once holds for every frame. Rotating the camera itself would
require re-solving the fit per frame.

Run: blender -b -P render_turntable.py -- '{"blend-path":"m.blend","output-path":"o.mp4"}'
"""

import math
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import bpy  # noqa: E402

import _framing  # noqa: E402
import _result  # noqa: E402
import _stage  # noqa: E402


def _iter_fcurves(action):
    """Yield every fcurve of `action`, across Blender's legacy and layered
    Action data models.

    Verified against this Blender 5.2 install: `Action` has no `.fcurves`
    attribute at all here -- fcurves live under `.layers[].strips[].
    channelbags[].fcurves` (the 4.4+ "layered actions" redesign; raises
    `AttributeError: 'Action' object has no attribute 'fcurves'` otherwise).
    Older Blenders (this agent targets 4.2+) keep the flat `.fcurves` list, so
    both shapes are handled.
    """
    if hasattr(action, "fcurves"):
        yield from action.fcurves
        return
    for layer in getattr(action, "layers", ()):
        for strip in getattr(layer, "strips", ()):
            for channelbag in getattr(strip, "channelbags", ()):
                yield from channelbag.fcurves


import render_still  # noqa: E402  - reuse load_scene / lighting / engine resolution


def main(inputs: dict) -> dict:
    output_path = os.path.abspath(str(_result.require(inputs, "output-path")))
    duration = float(inputs.get("duration-seconds", 8))
    fps = int(inputs.get("fps", 30))
    width = int(inputs.get("width-pixels", 1920))
    height = int(inputs.get("height-pixels", 1080))
    direction = str(inputs.get("direction", "hero"))
    samples = int(inputs.get("samples", 0))
    environment = inputs.get("environment", _stage.DEFAULT_ENVIRONMENT)
    ground_enabled = render_still._as_bool(inputs.get("ground"), default=True)

    if duration <= 0 or fps < 1:
        raise _result.AwareBlenderError(
            _result.ERR_INVALID_INPUTS,
            "duration-seconds must be > 0 and fps must be >= 1",
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
    if samples < 0:
        raise _result.AwareBlenderError(
            _result.ERR_INVALID_INPUTS, "samples must be >= 0"
        )
    frame_count = max(2, int(round(duration * fps)))

    render_still.load_scene(inputs)

    scene = bpy.context.scene
    scene.render.resolution_x = width
    scene.render.resolution_y = height
    scene.render.resolution_percentage = 100
    scene.render.fps = fps
    scene.frame_start = 1
    scene.frame_end = frame_count

    scene.render.engine = render_still._eevee_engine()
    if hasattr(scene, "eevee") and hasattr(scene.eevee, "taa_render_samples"):
        scene.eevee.taa_render_samples = samples or 16
    _stage.tune_eevee(scene)

    # Blender 5.x added a `media_type` switch on image_settings (IMAGE /
    # MULTI_LAYER_IMAGE / VIDEO) that gates which `file_format` values are
    # valid; it defaults to IMAGE, and assigning `file_format = "FFMPEG"`
    # without first switching to VIDEO raises TypeError: enum "FFMPEG" not
    # found in (...). Verified directly against this Blender 5.2 install.
    # Guarded with hasattr since the design doc targets blender@4.2+, and this
    # property may not exist on older 4.x builds where FFMPEG was unguarded.
    if hasattr(scene.render.image_settings, "media_type"):
        scene.render.image_settings.media_type = "VIDEO"
    scene.render.image_settings.file_format = "FFMPEG"
    scene.render.ffmpeg.format = "MPEG4"
    scene.render.ffmpeg.codec = "H264"
    scene.render.ffmpeg.constant_rate_factor = "MEDIUM"
    scene.render.ffmpeg.ffmpeg_preset = "GOOD"
    scene.render.filepath = output_path

    environment_receipt = _stage.setup_world(environment)
    _stage.setup_key_light()

    camera = _framing.ensure_camera()
    try:
        framing = _framing.frame_camera(camera, direction)
    except ValueError as exc:
        raise _result.AwareBlenderError(_result.ERR_RENDER_FAILED, str(exc)) from exc

    # After framing, and before the pivot: the ground is sized from the model's
    # bounds and stays OUT of the fit (aware-helper), so it neither moves the
    # camera nor rides the orbit. It is radially symmetric about the model
    # centre, which is what keeps its fade edge out of frame at every angle.
    ground_receipt = _stage.setup_ground(ground_enabled)

    # Pivot at the model centre; the camera rides it, so the fit never changes.
    pivot = bpy.data.objects.new("AwareTurntablePivot", None)
    pivot.location = framing["centre"]
    scene.collection.objects.link(pivot)

    # `frame_camera()` set camera.location/.rotation_euler directly, and
    # pivot.location was just assigned above -- in background mode neither is
    # reflected in .matrix_world until the next depsgraph evaluation. Without
    # forcing one here, `world_matrix` below reads a stale (pre-framing) value,
    # so the re-parented camera ends up wherever it was before framing (the
    # origin, for a freshly created camera) instead of at the solved position.
    bpy.context.view_layer.update()

    world_matrix = camera.matrix_world.copy()
    camera.parent = pivot
    camera.matrix_parent_inverse = pivot.matrix_world.inverted()
    camera.matrix_world = world_matrix

    pivot.rotation_euler = (0.0, 0.0, 0.0)
    pivot.keyframe_insert(data_path="rotation_euler", frame=1)
    pivot.rotation_euler = (0.0, 0.0, 2.0 * math.pi)
    pivot.keyframe_insert(data_path="rotation_euler", frame=frame_count)

    # Linear interpolation, or the orbit eases in and out and looks broken.
    for fcurve in _iter_fcurves(pivot.animation_data.action):
        for keyframe in fcurve.keyframe_points:
            keyframe.interpolation = "LINEAR"

    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    bpy.ops.render.render(animation=True)

    # Blender may append a frame range to the container name; find what landed.
    written = output_path
    if not os.path.exists(written):
        directory = os.path.dirname(output_path)
        stem = os.path.splitext(os.path.basename(output_path))[0]
        candidates = [
            os.path.join(directory, name)
            for name in os.listdir(directory)
            if name.startswith(stem) and name.lower().endswith(".mp4")
        ]
        if not candidates:
            raise _result.AwareBlenderError(
                _result.ERR_RENDER_FAILED,
                f"animation rendered but no MP4 was written near {output_path}",
            )
        written = max(candidates, key=os.path.getmtime)

    return {
        "path": written,
        "output-path": written,
        "size-bytes": os.path.getsize(written),
        "frames": frame_count,
        "fps": fps,
        "duration-seconds": round(frame_count / fps, 3),
        "width-pixels": width,
        "height-pixels": height,
        "engine": scene.render.engine,
        "framing": framing,
        "environment": environment_receipt,
        "ground": ground_receipt,
    }


if __name__ == "__main__":
    _result.run(main)
