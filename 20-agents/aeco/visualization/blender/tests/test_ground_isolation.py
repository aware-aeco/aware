"""The ground plane must be invisible to everything except the renderer.

Two guarantees, both of which fail SILENTLY and neither of which any existing
check would catch:

1. **The ground must not move the camera.** `_framing.scene_bounds()` walks every
   mesh object, and the ground is sized FROM the fit that walk produces. If the
   `aware-helper` skip is ever lost, the floor inflates the bounding sphere,
   which pushes the camera back, which enlarges the floor -- on every render,
   with nothing raised and no error anywhere. So: render the same model with the
   ground on and off, and require the `framing` receipts to be IDENTICAL. That
   comparison is the whole point of this file.

2. **The ground must not become an element.** A non-IFC mesh reaching
   `scene.info`'s `_inventory()` would inflate `count` and add a row with empty
   guid/class/material to `elements` -- a fabricated element in the one command
   that exists to tell the truth about what is in the scene.

Plus the never-fail property the environment inherits from `_looks`' clay
fallback: a broken environment path still returns a picture, and says why in the
receipt rather than only in the pixels.

Run: python test_ground_isolation.py --aware-bin <path/to/aware[.exe]> [--blender <path>]
Prints a PASS/FAIL summary and exits 0 only if every check passes. Prints SKIP
and exits 0 if Blender cannot be found, so this does not fail CI on a machine
without Blender installed.
"""

import argparse
import json
import shutil
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))

from make_fixture import build_fixture  # noqa: E402
from run_smoke import (  # noqa: E402
    ChainError,
    expect_ok,
    resolve_blender,
    run_command,
)

# Small enough to keep this fast; the assertions are about the receipt, not the
# pixels, and framing is resolution-independent only insofar as both renders use
# the SAME resolution -- which is the point, so they must not diverge here.
WIDTH, HEIGHT = 320, 240
EXPECTED_ELEMENT_COUNT = 9

FAILURES: list[str] = []


def check(label: str, condition: bool, detail=None) -> None:
    if condition:
        print(f"  ok   {label}")
    else:
        print(f"  FAIL {label}")
        if detail is not None:
            print(f"       {detail}")
        FAILURES.append(label)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--blender", help="path to the Blender executable")
    parser.add_argument("--aware-bin", required=True, help="path to the aware CLI")
    opts = parser.parse_args()

    blender = resolve_blender(opts.blender)
    if blender is None:
        print("SKIP: no Blender available (pass --blender <path> to run this harness)")
        return 0

    work = Path(tempfile.mkdtemp(prefix="aware-ground-"))
    try:
        ifc = work / "fixture.ifc"
        build_fixture(opts.aware_bin, ifc)

        common = {
            "ifc-path": str(ifc),
            "quality": "draft",
            "width-pixels": WIDTH,
            "height-pixels": HEIGHT,
        }

        with_ground = expect_ok(
            "render.still (ground on)",
            run_command(blender, "render_still.py", {
                **common, "output-path": str(work / "on.png"), "ground": True,
            }),
        )
        without_ground = expect_ok(
            "render.still (ground off)",
            run_command(blender, "render_still.py", {
                **common, "output-path": str(work / "off.png"), "ground": False,
            }),
        )

        # ---- 1. the ground must not move the camera -------------------------
        check(
            "framing is identical with and without the ground",
            with_ground["framing"] == without_ground["framing"],
            "the ground plane is inside scene_bounds() -- every render is now "
            "reframed by the floor it stands on:\n"
            f"       ground on:  {json.dumps(with_ground['framing'], sort_keys=True)}\n"
            f"       ground off: {json.dumps(without_ground['framing'], sort_keys=True)}",
        )
        check(
            "`ground: true` actually produced a ground",
            with_ground["ground"].get("present") is True,
            with_ground["ground"],
        )
        check(
            "`ground: false` actually suppressed it",
            without_ground["ground"].get("present") is False,
            without_ground["ground"],
        )
        check(
            "the ground is sized from the model, not from thin air",
            with_ground["ground"].get("radius", 0) > with_ground["framing"]["radius"],
            with_ground["ground"],
        )

        # ---- 2. the ground must not become an element -----------------------
        info = expect_ok(
            "scene.info", run_command(blender, "scene_info.py", {"ifc-path": str(ifc)})
        )
        check(
            "scene.info still counts exactly the model's elements",
            info["count"] == EXPECTED_ELEMENT_COUNT,
            f"expected {EXPECTED_ELEMENT_COUNT}, got {info['count']}",
        )
        check(
            "no element in the inventory has an empty IFC class",
            all(element["class"] for element in info["elements"]),
            [e for e in info["elements"] if not e["class"]],
        )

        # ---- 3. never-fail: a broken environment still renders --------------
        broken = expect_ok(
            "render.still (broken environment)",
            run_command(blender, "render_still.py", {
                **common,
                "output-path": str(work / "broken.png"),
                "environment": "/no/such/hdri.exr",
            }),
        )
        environment = broken["environment"]
        check(
            "a broken environment path still returns a picture",
            Path(broken["path"]).exists() and broken["size-bytes"] > 0,
            broken.get("size-bytes"),
        )
        check(
            "...and degrades to the procedural gradient",
            environment["source"] == "procedural-gradient",
            environment,
        )
        check(
            "...and says why, so the degrade is visible without opening the PNG",
            bool(environment["fallback-reason"]),
            environment,
        )

        # ---- 4. the default resolves to a real bundled HDRI -----------------
        default_environment = with_ground["environment"]
        check(
            "the default environment resolves to one of Blender's bundled HDRIs",
            default_environment["source"] == "blender-studiolight",
            default_environment,
        )
        check(
            "...with no fallback reason on the happy path",
            default_environment["fallback-reason"] is None,
            default_environment,
        )

    except ChainError as exc:
        print(f"\n  FAIL {exc}")
        FAILURES.append(str(exc))
    finally:
        shutil.rmtree(work, ignore_errors=True)

    print("\nGROUND ISOLATION FAIL" if FAILURES else "\nGROUND ISOLATION PASS")
    return 1 if FAILURES else 0


if __name__ == "__main__":
    sys.exit(main())
