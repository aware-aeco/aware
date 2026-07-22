"""`scene.info` -- inventory a staged .blend (or an IFC directly) by class,
material and storey. Read-only.

Run: blender -b -P scene_info.py -- '{"blend-path":"m.blend"}'
  or blender -b -P scene_info.py -- '{"ifc-path":"m.ifc"}'

Exactly one of `blend-path` / `ifc-path` is required. Passing both is
rejected rather than silently preferring one -- otherwise which source was
actually read is undetectable from the payload after the fact.

On the `ifc-path` branch, `_ifc_import.import_ifc()`'s own records travel
into the payload as two independent pairs: `skipped` / `skipped-count` and
`excluded` / `excluded-count`. This command is the verification and
debugging surface for the rest of the agent, so a partially-failed import
must never look identical to a smaller-but-intact model, and someone
asking "why is there no wall here" must be able to tell "failed to
tessellate" apart from "deliberately never imported" (openings, spaces --
see `_ifc_import._NON_VISUAL_CLASSES`). The `blend-path` branch has no such
records -- the import already happened in a prior `scene.import` call --
so it omits all four keys rather than inventing zeroed counts that would
falsely assert nothing was ever dropped or excluded.
"""

import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import bpy  # noqa: E402

import _ifc_import  # noqa: E402
import _result  # noqa: E402


def _inventory() -> dict:
    """Tally the current scene's objects by their IFC custom properties."""
    by_class: dict[str, int] = {}
    by_material: dict[str, int] = {}
    by_storey: dict[str, int] = {}
    elements = []

    for obj in bpy.data.objects:
        if obj.type != "MESH":
            continue
        ifc_class = obj.get(_ifc_import.PROP_CLASS, "")
        material = obj.get(_ifc_import.PROP_MATERIAL, "")
        storey = obj.get(_ifc_import.PROP_STOREY, "")
        if ifc_class:
            by_class[ifc_class] = by_class.get(ifc_class, 0) + 1
        if material:
            by_material[material] = by_material.get(material, 0) + 1
        if storey:
            by_storey[storey] = by_storey.get(storey, 0) + 1
        elements.append(
            {
                "guid": obj.get(_ifc_import.PROP_GUID, ""),
                "name": obj.get(_ifc_import.PROP_NAME, "") or obj.name,
                "class": ifc_class,
                "material": material,
                "storey": storey,
            }
        )

    return {
        "count": len(elements),
        "by-class": dict(sorted(by_class.items())),
        "by-material": dict(sorted(by_material.items())),
        "by-storey": dict(sorted(by_storey.items())),
        "elements": sorted(elements, key=lambda e: (e["class"], e["name"])),
    }


def main(inputs: dict) -> dict:
    blend_path = inputs.get("blend-path")
    ifc_path = inputs.get("ifc-path")
    if not blend_path and not ifc_path:
        raise _result.AwareBlenderError(
            _result.ERR_INVALID_INPUTS,
            "one of `blend-path` or `ifc-path` is required",
        )
    if blend_path and ifc_path:
        # Preferring one silently would make the source actually read
        # undetectable from the payload after the fact -- reject instead of
        # guessing; the docstring already documents an either/or contract.
        raise _result.AwareBlenderError(
            _result.ERR_INVALID_INPUTS,
            "pass exactly one of `blend-path` or `ifc-path`, not both",
        )

    if blend_path:
        if not os.path.exists(str(blend_path)):
            raise _result.AwareBlenderError(
                _result.ERR_BLEND_UNREADABLE, f".blend not found: {blend_path}"
            )
        bpy.ops.wm.open_mainfile(filepath=os.path.abspath(str(blend_path)))
        payload = _inventory()
        payload["source"] = "blend-path"
        return payload

    if not os.path.exists(str(ifc_path)):
        raise _result.AwareBlenderError(
            _result.ERR_IFC_UNREADABLE, f"IFC not found: {ifc_path}"
        )
    _ifc_import.clear_scene()
    import_receipt = _ifc_import.import_ifc(
        str(ifc_path), unit_scale=float(inputs.get("unit-scale", 1.0))
    )
    payload = _inventory()
    # The import just ran in-process, so both records are live -- surface
    # them rather than letting a partially failed import masquerade as a
    # smaller-but-complete model, or an intentional exclusion masquerade as
    # a failure (see module docstring).
    payload["skipped"] = import_receipt["skipped"]
    payload["skipped-count"] = import_receipt["skipped-count"]
    payload["excluded"] = import_receipt["excluded"]
    payload["excluded-count"] = import_receipt["excluded-count"]
    payload["source"] = "ifc-path"
    return payload


if __name__ == "__main__":
    _result.run(main)
