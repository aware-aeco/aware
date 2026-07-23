"""The framing must not depend on a lens the input `.blend` happened to carry.

`ensure_camera()` returns a camera the `.blend` already had, untouched. So if
the presentation lens were applied there, it would reach only scenes the agent
built itself, and any camera-bearing `.blend` would keep rendering at whatever
FOV that file happened to have -- Blender's wide 50mm default, usually. The same
`render.still` request would then produce two different pictures depending on a
property of the input nobody thinks about. Found by review, not by a render,
because the fixture path never exercises it: `scene.import` writes no camera.

The pin: stage the SAME model into two `.blend`s whose cameras differ as widely
as the property allows -- 24mm and 200mm -- and require both renders to come
back with an identical `framing` receipt. `frame_camera()` derives `distance`
from `camera.data.angle`, which derives from the lens, so a lens leaking through
moves `distance` by a factor of eight and this fails loudly.

Run: python test_camera_lens.py --aware-bin <path/to/aware[.exe]> [--blender <path>]
"""

import argparse
import json
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))

from make_fixture import build_fixture  # noqa: E402
from run_smoke import ChainError, expect_ok, resolve_blender, run_command  # noqa: E402

LENSES_MM = (24.0, 200.0)

FAILURES: list[str] = []


def check(label: str, condition: bool, detail=None) -> None:
    if condition:
        print(f"  ok   {label}")
    else:
        print(f"  FAIL {label}")
        if detail is not None:
            print(f"       {detail}")
        FAILURES.append(label)


def stage_blend_with_camera(blender: str, source: Path, target: Path, lens_mm: float) -> None:
    """Copy a staged .blend and give it a camera at `lens_mm`."""
    shutil.copyfile(source, target)
    script = (
        "import bpy, sys\n"
        f"bpy.ops.wm.open_mainfile(filepath=r'{target}')\n"
        f"data = bpy.data.cameras.new('PreExisting')\n"
        f"data.lens = {lens_mm}\n"
        "camera = bpy.data.objects.new('PreExisting', data)\n"
        "bpy.context.scene.collection.objects.link(camera)\n"
        "bpy.context.scene.camera = camera\n"
        f"bpy.ops.wm.save_as_mainfile(filepath=r'{target}')\n"
        "print('STAGED_OK')\n"
    )
    proc = subprocess.run(
        [blender, "-b", "--python-expr", script], capture_output=True, text=True
    )
    if "STAGED_OK" not in proc.stdout:
        raise ChainError(
            f"could not stage a .blend with a {lens_mm}mm camera:\n"
            f"--- stdout tail ---\n{proc.stdout[-1500:]}\n"
            f"--- stderr tail ---\n{proc.stderr[-1500:]}"
        )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--blender")
    parser.add_argument("--aware-bin", required=True)
    opts = parser.parse_args()

    blender = resolve_blender(opts.blender)
    if blender is None:
        print("SKIP: no Blender available (pass --blender <path> to run this harness)")
        return 0

    work = Path(tempfile.mkdtemp(prefix="aware-lens-"))
    try:
        ifc = work / "fixture.ifc"
        build_fixture(opts.aware_bin, ifc)

        staged = expect_ok(
            "scene.import",
            run_command(blender, "scene_import.py", {
                "ifc-path": str(ifc), "blend-path": str(work / "staged.blend"),
            }),
        )
        source = Path(staged["blend-path"])

        framings = {}
        for lens_mm in LENSES_MM:
            blend = work / f"camera-{int(lens_mm)}mm.blend"
            stage_blend_with_camera(blender, source, blend, lens_mm)
            payload = expect_ok(
                f"render.still (.blend carrying a {int(lens_mm)}mm camera)",
                run_command(blender, "render_still.py", {
                    "blend-path": str(blend),
                    "output-path": str(work / f"{int(lens_mm)}.png"),
                    "quality": "draft", "width-pixels": 320, "height-pixels": 240,
                }),
            )
            framings[lens_mm] = payload["framing"]

        first, second = (framings[lens] for lens in LENSES_MM)
        check(
            "framing ignores the lens the input .blend carried",
            first == second,
            "the input file's camera is leaking into the fit -- the same request "
            "renders differently depending on the .blend it was handed:\n"
            f"       {int(LENSES_MM[0])}mm: {json.dumps(first, sort_keys=True)}\n"
            f"       {int(LENSES_MM[1])}mm: {json.dumps(second, sort_keys=True)}",
        )

    except ChainError as exc:
        print(f"\n  FAIL {exc}")
        FAILURES.append(str(exc))
    finally:
        shutil.rmtree(work, ignore_errors=True)

    print("\nCAMERA LENS FAIL" if FAILURES else "\nCAMERA LENS PASS")
    return 1 if FAILURES else 0


if __name__ == "__main__":
    sys.exit(main())
