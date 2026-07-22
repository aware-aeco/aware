"""`scene.apply-look` -- assign preset materials from IFC semantics.

Run: blender -b -P scene_apply_look.py -- '{"blend-path":"m.blend","preset":"realistic"}'
"""

import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import bpy  # noqa: E402

import _looks  # noqa: E402
import _result  # noqa: E402


def main(inputs: dict) -> dict:
    blend_path = str(_result.require(inputs, "blend-path"))
    preset = str(inputs.get("preset", "realistic"))
    out_path = str(inputs.get("output-path") or blend_path)

    if not os.path.exists(blend_path):
        raise _result.AwareBlenderError(
            _result.ERR_BLEND_UNREADABLE, f".blend not found: {blend_path}"
        )
    if preset not in _looks.PRESETS:
        raise _result.AwareBlenderError(
            _result.ERR_INVALID_INPUTS,
            f"unknown preset `{preset}`; expected one of {sorted(_looks.PRESETS)}",
        )

    bpy.ops.wm.open_mainfile(filepath=os.path.abspath(blend_path))
    receipt = _looks.apply_look(preset)
    bpy.ops.wm.save_as_mainfile(filepath=os.path.abspath(out_path))

    receipt["blend-path"] = os.path.abspath(out_path)
    receipt["path"] = receipt["blend-path"]
    return receipt


_result.run(main)
