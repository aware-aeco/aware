"""`scene.import` -- IFC in, staged .blend out.

Run: blender -b -P scene_import.py -- '{"ifc-path":"m.ifc","blend-path":"m.blend"}'
"""

import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import bpy  # noqa: E402

import _ifc_import  # noqa: E402
import _result  # noqa: E402


def main(inputs: dict) -> dict:
    ifc_path = str(_result.require(inputs, "ifc-path"))
    blend_path = str(_result.require(inputs, "blend-path"))
    unit_scale = float(inputs.get("unit-scale", 1.0))

    if not os.path.exists(ifc_path):
        raise _result.AwareBlenderError(
            _result.ERR_IFC_UNREADABLE, f"IFC not found: {ifc_path}"
        )

    _ifc_import.clear_scene()
    receipt = _ifc_import.import_ifc(ifc_path, unit_scale=unit_scale)

    os.makedirs(os.path.dirname(os.path.abspath(blend_path)), exist_ok=True)
    bpy.ops.wm.save_as_mainfile(filepath=os.path.abspath(blend_path))

    receipt["blend-path"] = os.path.abspath(blend_path)
    receipt["path"] = receipt["blend-path"]
    return receipt


if __name__ == "__main__":
    _result.run(main)
