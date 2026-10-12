"""`exec`'s `ifc-path` scene path (#692) against a REAL Blender.

The third of `exec`'s scene-initialisation paths (the empty and `blend-path`
paths are in `test_exec.py`): with `ifc-path`, the wrapper resets to an empty
factory scene, imports the IFC with its properties stamped on every object and
applies the `preset` look -- BEFORE the author's code runs. So the code must see:

  * exactly the fixture's elements through `ctx.elements()`, each carrying its
    IFC class, material, storey and GUID (and no factory cube, camera or light);
  * the look already applied (every element has a material);
  * `ctx.frame()` able to fit a camera to that model;
  * `import` and `look` progress phases reported before `setup`.

Run: python test_exec_ifc_scene.py --aware-bin <path/to/aware[.exe]> [--blender <path>]
Exits 0 when every check passes, 1 on any failure, 77 (SKIP) without Blender.
"""

import argparse
import shutil
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))

from make_fixture import build_fixture  # noqa: E402
from run_smoke import resolve_blender  # noqa: E402
from test_exec import FAILURES, SKIP_EXIT, check, progress_records, run_exec  # noqa: E402

EXPECTED_CLASSES = {"IfcBeam": 4, "IfcColumn": 4, "IfcMember": 1}

INSPECT = """\
import bpy

def run(ctx):
    elements = ctx.elements()
    by_class = {}
    for obj, props in elements:
        by_class[props["class"]] = by_class.get(props["class"], 0) + 1
    framing = ctx.frame("iso")
    return {
        "by_class": by_class,
        "guids": sorted(p["guid"] for _, p in elements),
        "storeys": sorted({p["storey"] for _, p in elements}),
        "materials": sorted({p["material"] for _, p in elements}),
        "unshaded": [o.name for o, _ in elements if not o.data.materials],
        "non_elements": sorted(o.name for o in bpy.data.objects
                               if o.type == "MESH" and not o.get("ifc_class")),
        "has_camera": ctx.scene.camera is not None,
        "radius": framing["radius"],
    }
"""


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--blender", help="path to the Blender executable")
    parser.add_argument("--aware-bin", required=True, help="path to the aware CLI")
    opts = parser.parse_args()

    blender = resolve_blender(opts.blender)
    if blender is None:
        print("SKIP: no Blender available (pass --blender <path> to run this harness)")
        return SKIP_EXIT

    work = Path(tempfile.mkdtemp(prefix="aware-exec-ifc-"))
    try:
        ifc = work / "fixture.ifc"
        build_fixture(opts.aware_bin, ifc)
        channel = work / "progress.jsonl"
        payload = run_exec(
            blender,
            work,
            {
                "code": INSPECT,
                "ifc-path": str(ifc),
                "preset": "clay",
                "output-dir": str(work / "out"),
            },
            progress=channel,
        )
        print("exec with ifc-path")
        check("the run succeeds", payload.get("ok") is True, payload)
        result = payload.get("result") or {}
        check(
            "ctx.elements() is exactly the fixture's elements",
            result.get("by_class") == EXPECTED_CLASSES,
            result.get("by_class"),
        )
        check("every element carries its GUID", len(set(result.get("guids", []))) == 9, result.get("guids"))
        check("every element carries its storey", result.get("storeys") == ["Storey"], result.get("storeys"))
        check(
            "every element carries its material grade",
            result.get("materials") == ["A36", "A500-GR.B", "A992"],
            result.get("materials"),
        )
        check("the preset look was applied before run(ctx)", result.get("unshaded") == [], result.get("unshaded"))
        check(
            "the factory scene was reset first (no stray meshes)",
            result.get("non_elements") == [],
            result.get("non_elements"),
        )
        check("ctx.frame() fitted a camera", result.get("has_camera") is True and result.get("radius", 0) > 0, result)
        scene = payload.get("scene", {})
        check("the scene receipt names the source", scene.get("source") == "ifc-path", scene)
        check("...and the preset", scene.get("preset") == "clay", scene)
        phases = [r["phase"] for r in progress_records(channel)]
        check(
            "import, then look, then setup are reported in order",
            phases[:3] == ["import", "look", "setup"],
            phases,
        )
    except (AssertionError, RuntimeError) as exc:
        FAILURES.append(str(exc))
        print(f"  FAIL {exc}")
    finally:
        shutil.rmtree(work, ignore_errors=True)

    print("\nEXEC IFC SCENE FAIL" if FAILURES else "\nEXEC IFC SCENE PASS")
    return 1 if FAILURES else 0


if __name__ == "__main__":
    sys.exit(main())
