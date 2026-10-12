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
      with an exception;
  (e) a render hook writes a BIGGER file of another type into the scratch
      directory: only files of the format being rendered are candidates, so
      the receipt still points at this render's MP4;
  (f) another writer replaces the requested path right after the move: the
      file there no longer has the identity the render wrote, so the receipt
      says `verified: false` with a plain reason.

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

# (e) A render hook drops a large file of ANOTHER type into the scratch dir.
SIDE_FILE = SCENE + """
def run(ctx):
    scene(ctx)

    def side_file(*_args):
        folder = os.path.dirname(bpy.context.scene.render.filepath)
        with open(os.path.join(folder, "notes.txt"), "wb") as handle:
            handle.write(b"x" * (1024 * 1024))

    bpy.app.handlers.render_write.append(side_file)
    try:
        video = ctx.render_animation("side.mp4")
    finally:
        bpy.app.handlers.render_write.remove(side_file)
    with open(os.path.join(ctx.output_dir, "side.mp4"), "rb") as handle:
        head = handle.read(16)
    return {"video": video, "head": head.hex(), "listing": listing(ctx)}
"""

# (f) Another writer replaces the requested path the moment the render lands.
REPLACED = SCENE + """
def run(ctx):
    scene(ctx)
    target = os.path.join(ctx.output_dir, "still.png")
    real_replace = os.replace

    def replace_then_overwrite(src, dst):
        real_replace(src, dst)
        if os.path.abspath(dst) == os.path.abspath(target):
            with open(dst, "wb") as handle:
                handle.write(b"another writer's bytes")

    os.replace = replace_then_overwrite
    try:
        still = ctx.render_still("still.png")
    finally:
        os.replace = real_replace
    return {"still": still, "listing": listing(ctx)}
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
            video.get("reason") == "no render output file was found",
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

        print("(e) a render hook writes a bigger file of another type into the scratch dir")
        payload = run_exec(blender, work, {"code": SIDE_FILE, "output-dir": str(work / "e")})
        result = payload.get("result") or {}
        video = result.get("video", {})
        check(
            "the file at the requested path is this render's MP4, not the side file",
            bytes.fromhex(result.get("head", ""))[4:8] == b"ftyp",
            result.get("head"),
        )
        check(
            "the receipt verifies it at the requested path, at the MP4's size",
            Path(video.get("path", "")).name == "side.mp4"
            and video.get("verified") is True
            and 0 < video.get("size-bytes", 0) < 1024 * 1024,
            video,
        )
        check("nothing else is left beside it", result.get("listing") == ["side.mp4"], result.get("listing"))

        print("(f) another writer replaces the requested path right after the move")
        payload = run_exec(blender, work, {"code": REPLACED, "output-dir": str(work / "f")})
        result = payload.get("result") or {}
        still = result.get("still", {})
        check("the receipt says verified:false", still.get("verified") is False, still)
        check(
            "...with a plain reason naming the path",
            str(still.get("reason", "")).startswith("the file at ")
            and str(still.get("reason", "")).endswith("changed after the render wrote it")
            and "still.png" in str(still.get("reason", "")),
            still.get("reason"),
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
