"""Regression test: `scene.info`'s `ifc-path` branch must surface import
skips, not silently report a smaller-but-clean-looking model.

Corrupts one element of the reference fixture -- a zero-depth extrusion on
the brace, which tessellates to nothing but is otherwise a well-formed IFC
product -- so only 8 of the fixture's 9 elements can be imported. Runs the
real `scene_info.py` `ifc-path` branch against the corrupted file (the same
way the CLI transport does: `blender -b -P scene_info.py -- <json>`) and
asserts the payload accounts for all 9 source elements, either imported or
skipped. A payload that reports `count: 8` with no trace of the ninth is
exactly the failure mode this guards against: it is indistinguishable from
a model that legitimately has 8 elements.

Also checks the `excluded` / `excluded-count` pair (deliberately-not-
imported products, e.g. openings and spaces) is present and correctly
empty on this fixture, which has none -- a lighter sibling assertion to
the skip-visibility one, same reasoning: an inventory command must not
make two different facts ("failed" vs "never meant to render") look alike.

Run: python test_scene_info_skips.py --aware-bin <path/to/aware[.exe]> [--blender <path>]
Prints a PASS/FAIL summary and exits 0 only if every check passes. Prints a
SKIP message and exits 0 if Blender cannot be found, so this does not fail
CI on a machine without Blender installed.
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

SCENE_INFO = HERE.parent / "scripts" / "scene_info.py"

# Verified location from the Task 1 toolchain probe; used only as a last
# resort when neither --blender nor PATH gives us one.
_KNOWN_BLENDER_WINDOWS = r"C:\Program Files\Blender Foundation\Blender 5.2\blender.exe"

# The brace's extrusion depth is hypot(6000, 3000) mm -- the only non-round
# depth in the fixture, which makes it a safe, self-checking anchor for a
# text-level substitution: if `ifc.write`'s output ever changes shape, the
# uniqueness check below fails loudly instead of corrupting the wrong entity.
_BRACE_DEPTH_ANCHOR = "6708.20393249937"


def find_blender(explicit: str | None) -> str:
    """Resolve a usable Blender executable, or raise with a clear reason."""
    if explicit:
        if not Path(explicit).exists():
            raise FileNotFoundError(f"--blender path does not exist: {explicit}")
        return explicit
    found = shutil.which("blender")
    if found:
        return found
    if Path(_KNOWN_BLENDER_WINDOWS).exists():
        return _KNOWN_BLENDER_WINDOWS
    raise FileNotFoundError("no --blender given, `blender` not on PATH")


def corrupt_brace_extrusion(ifc_path: Path) -> None:
    """Zero out the brace's `IfcExtrudedAreaSolid` depth in the STEP text.

    `ifc.write` itself refuses to emit a degenerate scene -- coincident
    from/to points fail scene validation outright, and a non-positive
    `section.w`/`section.d` is silently clamped back to 100mm before the
    rectangle-fallback profile is built -- so there is no scene-JSON input
    that reaches this state through the normal write path. The corruption
    has to land in the already-emitted STEP text, standing in for what a
    rounding bug in some *other* real producing application could hand the
    agent as a perfectly well-formed but geometrically empty product.
    """
    text = ifc_path.read_text(encoding="ascii")
    count = text.count(_BRACE_DEPTH_ANCHOR)
    if count != 1:
        raise RuntimeError(
            f"expected exactly one occurrence of the brace's extrusion depth "
            f"{_BRACE_DEPTH_ANCHOR!r} in {ifc_path}, found {count} -- fixture "
            "geometry or ifc.write's number formatting changed; update the "
            "anchor in this test"
        )
    corrupted = text.replace(f",{_BRACE_DEPTH_ANCHOR})", ",0.0)")
    ifc_path.write_text(corrupted, encoding="ascii")


def run_scene_info_ifc_path(blender: str, ifc_path: Path) -> tuple[int, dict]:
    """Invoke the real `scene_info.py` headlessly and parse its framed payload."""
    args_json = json.dumps({"ifc-path": str(ifc_path)})
    proc = subprocess.run(
        [blender, "-b", "-P", str(SCENE_INFO), "--", args_json],
        capture_output=True,
        text=True,
        check=False,
    )
    out = proc.stdout
    begin, end = "<<<AWARE_RESULT>>>", "<<<AWARE_RESULT_END>>>"
    if begin not in out or end not in out:
        raise RuntimeError(
            f"no framed result in scene_info.py output (exit {proc.returncode}):\n"
            f"--- stdout ---\n{out}\n--- stderr ---\n{proc.stderr}"
        )
    raw = out[out.index(begin) + len(begin) : out.index(end)]
    return proc.returncode, json.loads(raw)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--aware-bin", default="aware", help="path to the aware binary")
    parser.add_argument("--blender", default=None, help="path to the blender executable")
    opts = parser.parse_args()

    try:
        blender = find_blender(opts.blender)
    except FileNotFoundError as exc:
        print(f"SKIP: {exc}")
        return 0

    checks: list[tuple[str, bool, str]] = []

    with tempfile.TemporaryDirectory(prefix="aware-scene-info-skip-test-") as tmp:
        ifc_path = Path(tmp) / "corrupted.ifc"
        receipt = build_fixture(opts.aware_bin, ifc_path)
        total = len(receipt.get("emitted", []))
        checks.append(("reference fixture has 9 elements", total == 9, f"got {total}"))

        corrupt_brace_extrusion(ifc_path)

        exit_code, payload = run_scene_info_ifc_path(blender, ifc_path)

        checks.append(("scene_info.py exits 0 despite the bad element", exit_code == 0, f"exit={exit_code}"))
        checks.append(("payload `ok` is true", payload.get("ok") is True, f"ok={payload.get('ok')!r}"))
        checks.append((
            "payload `source` is `ifc-path`",
            payload.get("source") == "ifc-path",
            f"source={payload.get('source')!r}",
        ))
        checks.append((
            "`skipped` / `skipped-count` keys are present on the ifc-path branch",
            "skipped" in payload and "skipped-count" in payload,
            f"keys={sorted(payload.keys())}",
        ))
        checks.append((
            "`excluded` / `excluded-count` keys are present on the ifc-path branch",
            "excluded" in payload and "excluded-count" in payload,
            f"keys={sorted(payload.keys())}",
        ))
        checks.append((
            "the fixture has no non-visual products, so excluded-count is 0",
            payload.get("excluded") == {} and payload.get("excluded-count") == 0,
            f"excluded={payload.get('excluded')!r} excluded-count={payload.get('excluded-count')!r}",
        ))

        count = payload.get("count")
        skipped_count = payload.get("skipped-count")
        checks.append((
            "the brace failed to import (count is 8, not 9)",
            count == total - 1,
            f"count={count}",
        ))

        accounted = (count or 0) + (skipped_count or 0)
        checks.append((
            "every source element is accounted for: imported + skipped == total",
            accounted == total,
            f"count={count} skipped-count={skipped_count} total={total} "
            f"(accounted={accounted}) -- the dropped brace must show up in "
            "`skipped`, not vanish from the payload with no trace",
        ))

        if skipped_count:
            entries_ok = all(entry.get("guid") for entry in payload.get("skipped", []))
            checks.append((
                "each `skipped` entry carries a non-empty guid",
                entries_ok,
                f"skipped={payload.get('skipped')}",
            ))

    passed = sum(1 for _, ok, _ in checks if ok)
    verdict = "PASS" if passed == len(checks) else "FAIL"
    print(f"\n{verdict}: {passed}/{len(checks)} checks passed\n")
    for name, ok, detail in checks:
        mark = "PASS" if ok else "FAIL"
        line = f"  [{mark}] {name}"
        if detail and not ok:
            line += f"\n         {detail}"
        print(line)
    print()

    return 0 if passed == len(checks) else 1


if __name__ == "__main__":
    sys.exit(main())
