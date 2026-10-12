"""`exec` (#692) against a REAL Blender: the wrapper, the ctx contract, progress.

What this proves, each against a real `blender -b -P exec.py` launch exactly as
the transport makes it (inputs staged as `@file`, result sliced from between the
sentinels):

  * the happy path returns `run(ctx)`'s value, an explicit outputs receipt and
    the frame total -- and starts from an EMPTY scene when no source is given
    (no default cube, camera or light);
  * a runtime exception is a structured `exec-error` whose line number counts in
    the AUTHORED code, with a traceback that quotes it;
  * a syntax error is reported as one, with its authored line;
  * an output outside `output_dir` (or never written) is listed `verified: false`
    with the reason -- never dropped, never passed off as good;
  * a `blend-path` opens that file as the starting scene;
  * a result over 256 KiB fails as `result-too-large`;
  * an 8-frame 64x64 EEVEE animation writes exactly 8 `render` progress records,
    ending `done == total == 8`, on a version where frame counting is measured
    (`_progress.COUNTED_VERSIONS`); on any other version it must write phase-only
    records and NO counts -- that is what keeps the allow-list honest.

Plus, without Blender: `_progress` is a no-op with no channel, writes one
bounded line per record with one, and coalesces past 5,000 frames.

The `ifc-path` scene path needs a generated IFC and so lives in
`test_exec_ifc_scene.py`, which also needs a built CLI.

Run: python test_exec.py [--blender <path>]
Exits 0 when every check passes, 1 on any failure, 77 (SKIP) when no Blender
can be found.
"""

import argparse
import json
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
SCRIPTS = HERE.parent / "scripts"
sys.path.insert(0, str(SCRIPTS))
sys.path.insert(0, str(HERE))

import _progress  # noqa: E402 -- no bpy at module level, safe in plain Python
import _result  # noqa: E402
from run_smoke import detect_blender_version, resolve_blender  # noqa: E402

SKIP_EXIT = 77
FAILURES: list[str] = []


def check(label: str, condition: bool, detail=None) -> None:
    if condition:
        print(f"  ok   {label}")
    else:
        print(f"  FAIL {label}")
        if detail is not None:
            print(f"       {detail}")
        FAILURES.append(label)


def run_exec(blender: str, work: Path, inputs: dict, progress: Path | None = None) -> dict:
    """One real `exec` launch; returns the framed payload."""
    staged = work / f"inputs-{len(list(work.glob('inputs-*.json')))}.json"
    staged.write_text(json.dumps(inputs), encoding="utf-8")
    env = dict(os.environ)
    env.pop(_progress.ENV, None)
    if progress is not None:
        env[_progress.ENV] = str(progress)
    proc = subprocess.run(
        [blender, "-b", "-P", str(SCRIPTS / "exec.py"), "--", f"@{staged}"],
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
        env=env,
        check=False,
    )
    out = proc.stdout
    if _result.RESULT_BEGIN not in out or _result.RESULT_END not in out:
        raise AssertionError(
            f"no framed result (exit {proc.returncode})\n--- stdout tail ---\n{out[-2000:]}"
            f"\n--- stderr tail ---\n{proc.stderr[-2000:]}"
        )
    body = out.split(_result.RESULT_BEGIN, 1)[1].split(_result.RESULT_END, 1)[0]
    return json.loads(body)


def progress_records(path: Path) -> list[dict]:
    if not path.exists():
        return []
    return [
        json.loads(line)[_progress.RECORD_KEY]
        for line in path.read_text(encoding="utf-8").splitlines()
        if line.strip()
    ]


# ---------------------------------------------------------------------------
# _progress, no Blender needed
# ---------------------------------------------------------------------------


def check_progress_writer(work: Path) -> None:
    print("_progress (plain Python)")
    channel = work / "plain-progress.jsonl"
    saved = os.environ.pop(_progress.ENV, None)
    try:
        _progress.emit("render", 1, 2, "pass 1 of ?")
        check("no channel -> emit is a no-op", not channel.exists())
        check("no channel -> not enabled", _progress.enabled() is False)

        os.environ[_progress.ENV] = str(channel)
        _progress.emit("render", 1, 2, "pass 1 of ?")
        _progress.emit("setup", message="x" * 20_000)
        records = progress_records(channel)
        check("with a channel -> one line per record", len(records) == 2, records)
        check(
            "the record carries phase/done/total/message",
            records[0] == {"phase": "render", "done": 1, "total": 2, "message": "pass 1 of ?"},
            records[0],
        )
        longest = max(len(l.encode("utf-8")) for l in channel.read_text(encoding="utf-8").splitlines())
        check(
            "a huge message is truncated under the runtime's 8 KiB record cap",
            longest <= _progress.MAX_RECORD_BYTES,
            longest,
        )
    finally:
        os.environ.pop(_progress.ENV, None)
        if saved is not None:
            os.environ[_progress.ENV] = saved

    check("<= 5000 frames: every frame reported", _progress.stride_for(5000) == 1)
    check("12000 frames: every 3rd frame", _progress.stride_for(12000) == 3)
    worst = 10_000_000
    check(
        "coalescing keeps any pass under the runtime's 10,000-record cap",
        worst // _progress.stride_for(worst) + 1 <= 10_000,
        _progress.stride_for(worst),
    )
    check("5.2 is a measured version", _progress.counting_supported((5, 2, 0)))
    check("4.2 is NOT a measured version", not _progress.counting_supported((4, 2, 0)))


# ---------------------------------------------------------------------------
# exec, real Blender
# ---------------------------------------------------------------------------

HAPPY = """\
import bpy

def run(ctx):
    started_with = len(bpy.data.objects)
    bpy.ops.mesh.primitive_cube_add(size=2)
    s = ctx.scene
    s.render.resolution_x = 64
    s.render.resolution_y = 64
    sun = bpy.data.objects.new("Sun", bpy.data.lights.new("Sun", "SUN"))
    s.collection.objects.link(sun)
    still = ctx.render_still("hero.png")
    # A file the code wrote OUTSIDE output_dir, and one it never wrote.
    outside = ctx.args["outside"]
    with open(outside, "w") as handle:
        handle.write("not mine to publish")
    ctx.add_output(outside, "file")
    ctx.add_output("never-written.json", "file")
    return {"started_with": started_with, "storeys": ctx.args["storeys"], "still": still["verified"]}
"""

RUNTIME_ERROR = """\
def helper():
    return 1 / 0

def run(ctx):
    x = 1
    return helper() + x
"""

SYNTAX_ERROR = """\
def run(ctx):
    value = 1
    return (value
"""

TOO_LARGE = """\
def run(ctx):
    return {"blob": "x" * (300 * 1024)}
"""

SAVE_BLEND = """\
import bpy

def run(ctx):
    bpy.ops.mesh.primitive_monkey_add()
    bpy.context.active_object.name = "AwareMarker"
    path = ctx.output_dir + "/marker.blend"
    bpy.ops.wm.save_as_mainfile(filepath=path)
    ctx.add_output(path, "blend")
    return {"saved": path}
"""

OPEN_BLEND = """\
import bpy

def run(ctx):
    return {"names": sorted(o.name for o in bpy.data.objects)}
"""

ANIMATION = """\
import bpy

def run(ctx):
    bpy.ops.mesh.primitive_cube_add(size=2)
    cube = bpy.context.active_object
    s = ctx.scene
    s.render.resolution_x = 64
    s.render.resolution_y = 64
    s.frame_start = 1
    s.frame_end = 8
    cube.location = (0, 0, 0)
    cube.keyframe_insert("location", frame=1)
    cube.location = (2, 0, 0)
    cube.keyframe_insert("location", frame=8)
    sun = bpy.data.objects.new("Sun", bpy.data.lights.new("Sun", "SUN"))
    s.collection.objects.link(sun)
    ctx.plan_passes(1)
    video = ctx.render_animation("sequence.mp4")
    return {"video": video}
"""


def check_exec(blender: str, work: Path) -> None:
    version = detect_blender_version(blender)
    print(f"exec against Blender {version}")
    out_dir = work / "out"
    outside = work / "elsewhere" / "stray.txt"
    outside.parent.mkdir(parents=True, exist_ok=True)

    # -- happy path, empty scene, outputs receipt --------------------------------
    payload = run_exec(
        blender,
        work,
        {
            "code": HAPPY,
            "args": {"storeys": 3, "outside": str(outside)},
            "output-dir": str(out_dir),
        },
    )
    check("happy path reports ok", payload.get("ok") is True, payload)
    result = payload.get("result") or {}
    check("run(ctx)'s return value comes back as `result`", result.get("storeys") == 3, result)
    check(
        "with no source the code starts from an EMPTY scene",
        result.get("started_with") == 0,
        result,
    )
    check("the scene receipt says so", payload.get("scene", {}).get("source") == "empty", payload.get("scene"))
    outputs = payload.get("outputs") or []
    by_name = {Path(o["path"]).name: o for o in outputs}
    hero = by_name.get("hero.png", {})
    check("render_still is recorded and verified", hero.get("verified") is True and hero.get("size-bytes", 0) > 0, hero)
    check("...with kind image", hero.get("kind") == "image", hero)
    stray = by_name.get("stray.txt", {})
    check("an output outside output_dir is listed verified:false", stray.get("verified") is False, stray)
    check("...with the reason", "outside output-dir" in str(stray.get("reason")), stray)
    missing = by_name.get("never-written.json", {})
    check("a registered file that was never written is verified:false", missing.get("verified") is False, missing)
    check("...and says it was not written", "not written" in str(missing.get("reason")), missing)
    check("nothing else is listed (no directory diffing)", len(outputs) == 3, outputs)
    check("frames counts animation frames only", payload.get("frames") == 0, payload.get("frames"))

    # -- a runtime exception -------------------------------------------------------
    payload = run_exec(blender, work, {"code": RUNTIME_ERROR, "output-dir": str(out_dir)})
    check("a raised exception is a structured failure", payload.get("ok") is False, payload)
    check("...coded exec-error", payload.get("code") == "exec-error", payload.get("code"))
    check(
        "...pointing at the authored line that raised (line 2)",
        payload.get("line") == 2,
        payload.get("line"),
    )
    tb = payload.get("traceback", "")
    check("the traceback names the authored code", "<aware-exec>" in tb, tb)
    check("the traceback quotes the failing authored line", "return 1 / 0" in tb, tb)
    check("the traceback hides the wrapper's own frames", "exec.py" not in tb, tb)
    check("the message names the exception", "ZeroDivisionError" in payload.get("message", ""), payload.get("message"))

    # -- a syntax error ---------------------------------------------------------------
    payload = run_exec(blender, work, {"code": SYNTAX_ERROR, "output-dir": str(out_dir)})
    check("a syntax error is exec-error", payload.get("code") == "exec-error", payload)
    check("...reported as a SyntaxError", "SyntaxError" in payload.get("message", ""), payload.get("message"))
    check("...on its authored line (3)", payload.get("line") == 3, payload.get("line"))

    # -- result too large ---------------------------------------------------------------
    payload = run_exec(blender, work, {"code": TOO_LARGE, "output-dir": str(out_dir)})
    check("a >256 KiB result fails", payload.get("ok") is False, payload.get("code"))
    check("...as result-too-large", payload.get("code") == "result-too-large", payload.get("code"))
    check("...telling the author to add_output a file", "add_output" in payload.get("hint", ""), payload.get("hint"))

    # -- blend-path scene init ----------------------------------------------------------
    saved = run_exec(blender, work, {"code": SAVE_BLEND, "output-dir": str(out_dir)})
    blend = (saved.get("result") or {}).get("saved")
    check("a .blend can be written and registered", saved.get("ok") and blend and Path(blend).exists(), saved)
    opened = run_exec(
        blender, work, {"code": OPEN_BLEND, "blend-path": blend, "output-dir": str(out_dir)}
    )
    names = (opened.get("result") or {}).get("names", [])
    check("blend-path opens that file as the starting scene", "AwareMarker" in names, names)
    check("...and says so", opened.get("scene", {}).get("source") == "blend-path", opened.get("scene"))

    # -- the 8-frame animation and its progress records ------------------------------
    channel = work / "anim-progress.jsonl"
    payload = run_exec(
        blender,
        work,
        {"code": ANIMATION, "output-dir": str(out_dir)},
        progress=channel,
    )
    check("the animation renders", payload.get("ok") is True, payload)
    video = (payload.get("result") or {}).get("video", {})
    check("the MP4 is verified", video.get("verified") is True and video.get("size-bytes", 0) > 0, video)
    check("the receipt carries the frame count", video.get("frames") == 8, video)
    check("`frames` totals the animation", payload.get("frames") == 8, payload.get("frames"))
    records = progress_records(channel)
    renders = [r for r in records if r["phase"] == "render"]
    phases = [r["phase"] for r in records]
    check("the setup phase is reported before rendering", phases[:1] == ["setup"], phases)
    if version is not None and _progress.counting_supported(version):
        check("exactly 8 render records", len(renders) == 8, renders)
        check(
            "done counts 1..8 in order",
            [r.get("done") for r in renders] == list(range(1, 9)),
            renders,
        )
        check(
            "the last record is done == total == 8",
            bool(renders) and renders[-1].get("done") == renders[-1].get("total") == 8,
            renders[-1:] if renders else renders,
        )
        check(
            "each record names its pass",
            all(r.get("message") == "pass 1 of 1" for r in renders),
            renders,
        )
    else:
        check(
            "an unmeasured version reports phases only -- no counts",
            renders and all("done" not in r and "total" not in r for r in renders),
            renders,
        )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--blender", help="path to the Blender executable")
    opts = parser.parse_args()

    work = Path(tempfile.mkdtemp(prefix="aware-exec-"))
    try:
        check_progress_writer(work)
        blender = resolve_blender(opts.blender)
        if blender is None:
            if FAILURES:
                print("\nEXEC FAIL")
                return 1
            print("SKIP: no Blender available (pass --blender <path> to run this harness)")
            return SKIP_EXIT
        try:
            check_exec(blender, work)
        except AssertionError as exc:
            FAILURES.append(str(exc))
            print(f"  FAIL {exc}")
    finally:
        shutil.rmtree(work, ignore_errors=True)

    print("\nEXEC FAIL" if FAILURES else "\nEXEC PASS")
    return 1 if FAILURES else 0


if __name__ == "__main__":
    sys.exit(main())
