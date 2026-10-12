"""How `ctx.render_still` / `ctx.render_animation` land their file (#692 review 3).

Each render call writes into a scratch directory that only that call uses, next
to the requested path. Afterwards the ONLY candidates are the files in that
directory -- whatever name or frame-range suffix Blender chose -- and the one
output is moved with `os.replace` onto the requested path, which it overwrites
by design. The scratch directory is removed whatever happens. So nothing else
writing in `output_dir` -- an earlier run, a concurrent one -- can be mistaken
for this render's output, and no timestamp or before/after comparison is needed.

Against a REAL Blender, each check below:

  (a) a concurrent writer creates the exact target DURING the render: the
      receipt and the file at the requested path are this render's MP4 bytes;
  (b) Blender appends a frame range to the movie name: the final file is at the
      requested path anyway, and no suffixed file is left beside it;
  (c) the render writes nothing: `verified: false` with a plain reason, no file
      at the requested path, and no scratch directory left;
  (d) no scratch directory is left after a success, nor after the move fails
      with an exception.

Run: python test_exec_render_output.py [--blender <path>]
Exits 0 on pass, 1 on failure, 77 (SKIP) when no Blender can be found.
"""

import argparse
import shutil
import sys
import tempfile
from pathlib import Path

sys.dont_write_bytecode = True  # importing from scripts/ must not write into the package

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))

from run_smoke import resolve_blender  # noqa: E402
from test_exec import FAILURES, SKIP_EXIT, check, run_exec  # noqa: E402

SCENE = """\
import os
import bpy

def scene(ctx, frames=2):
    bpy.ops.mesh.primitive_cube_add(size=2)
    s = ctx.scene
    s.render.resolution_x = 32
    s.render.resolution_y = 32
    s.frame_start, s.frame_end = 1, frames
    sun = bpy.data.objects.new("Sun", bpy.data.lights.new("Sun", "SUN"))
    s.collection.objects.link(sun)

def listing(ctx):
    return sorted(os.listdir(ctx.output_dir))
"""

# (a) A writer that is NOT this render creates the exact target mid-render.
CONCURRENT = SCENE + """
def run(ctx):
    scene(ctx)
    target = os.path.join(ctx.output_dir, "seq")

    def intruder(*_args):
        with open(target, "wb") as handle:
            handle.write(b"another writer's bytes")

    bpy.app.handlers.render_write.append(intruder)
    try:
        video = ctx.render_animation("seq")
    finally:
        bpy.app.handlers.render_write.remove(intruder)
    with open(target, "rb") as handle:
        head = handle.read(16)
    return {"video": video, "head": head.hex(), "listing": listing(ctx)}
"""

# (b) The scratch file name carries no extension, so Blender writes the movie as
# <name><start>-<end>.mp4; the requested name has an extension of its own.
SUFFIXED = SCENE + """
def run(ctx):
    scene(ctx)
    video = ctx.render_animation("clip.mp4")
    return {"video": video, "listing": listing(ctx)}
"""

# (c) Something deletes whatever the render wrote before ctx collects it.
NOTHING = SCENE + """
def run(ctx):
    scene(ctx)

    def vanish(*_args):
        folder = os.path.dirname(bpy.context.scene.render.filepath)
        for name in os.listdir(folder):
            os.remove(os.path.join(folder, name))

    bpy.app.handlers.render_complete.append(vanish)
    try:
        video = ctx.render_animation("nothing.mp4")
    finally:
        bpy.app.handlers.render_complete.remove(vanish)
    return {"video": video, "listing": listing(ctx)}
"""

# (d) The final move fails: the requested path is a DIRECTORY.
MOVE_FAILS = SCENE + """
def run(ctx):
    scene(ctx)
    os.makedirs(os.path.join(ctx.output_dir, "blocked.png"))
    try:
        ctx.render_still("blocked.png")
        raised = None
    except Exception as exc:
        raised = type(exc).__name__
    return {"raised": raised, "listing": listing(ctx)}
"""


def no_scratch(listing) -> bool:
    return not any(name.startswith(".aware-render-") for name in listing)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--blender", help="path to the Blender executable")
    opts = parser.parse_args()

    blender = resolve_blender(opts.blender)
    if blender is None:
        print("SKIP: no Blender available (pass --blender <path> to run this harness)")
        return SKIP_EXIT

    work = Path(tempfile.mkdtemp(prefix="aware-render-output-"))
    try:
        print("(a) a concurrent writer creates the exact target during the render")
        payload = run_exec(blender, work, {"code": CONCURRENT, "output-dir": str(work / "a")})
        result = payload.get("result") or {}
        video = result.get("video", {})
        check("the run succeeds", payload.get("ok") is True, payload)
        check(
            "the file at the requested path is this render's MP4, not the other writer's bytes",
            bytes.fromhex(result.get("head", ""))[4:8] == b"ftyp",
            result.get("head"),
        )
        check(
            "the receipt names the requested path and verifies it",
            Path(video.get("path", "")).name == "seq" and video.get("verified") is True,
            video,
        )
        check("no scratch directory is left (success)", no_scratch(result.get("listing", [])), result.get("listing"))

        print("(b) Blender writes a frame-suffixed name")
        payload = run_exec(blender, work, {"code": SUFFIXED, "output-dir": str(work / "b")})
        result = payload.get("result") or {}
        video = result.get("video", {})
        check(
            "the final file is at exactly the requested path",
            Path(video.get("path", "")).name == "clip.mp4" and video.get("verified") is True,
            video,
        )
        check("nothing else is left beside it", result.get("listing") == ["clip.mp4"], result.get("listing"))
        check("the receipt keeps the frame count", video.get("frames") == 2, video)

        print("(c) the render writes nothing")
        payload = run_exec(blender, work, {"code": NOTHING, "output-dir": str(work / "c")})
        result = payload.get("result") or {}
        video = result.get("video", {})
        check("the receipt says verified:false", video.get("verified") is False, video)
        check(
            "...with a plain reason",
            "wrote no file" in str(video.get("reason", "")),
            video.get("reason"),
        )
        check("nothing appears at the requested path, and no scratch is left", result.get("listing") == [], result.get("listing"))

        print("(d) the move fails with an exception")
        payload = run_exec(blender, work, {"code": MOVE_FAILS, "output-dir": str(work / "d")})
        result = payload.get("result") or {}
        check("the failure surfaces as an exception", bool(result.get("raised")), result)
        check(
            "no scratch directory is left (exception)",
            result.get("listing") == ["blocked.png"],
            result.get("listing"),
        )
    except AssertionError as exc:
        FAILURES.append(str(exc))
        print(f"  FAIL {exc}")
    finally:
        shutil.rmtree(work, ignore_errors=True)

    print("\nRENDER OUTPUT FAIL" if FAILURES else "\nRENDER OUTPUT PASS")
    return 1 if FAILURES else 0


if __name__ == "__main__":
    sys.exit(main())
