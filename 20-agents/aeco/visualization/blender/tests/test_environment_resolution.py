"""`_stage.resolve_environment` must never fail, and must always say what it did.

The agent's never-fail principle means a typo'd environment still returns a
picture. That makes the receipt the ONLY place recording that it was not the
picture the caller asked for -- so every degrade path below asserts a non-null
`fallback-reason`. A silent degrade and a happy path that look alike is exactly
the failure this guards against, and it is the same reasoning that put
`skipped` / `excluded` on the import receipt.

Runs as plain Python with no Blender: `resolve_environment` takes the
studiolight directory as a parameter precisely so it can be tested this way,
and the modules it imports are stubbed below.

Run: python test_environment_resolution.py
"""

import sys
import tempfile
import types
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent / "scripts"))

# `_stage` reaches for bpy only inside function bodies, so bare stubs are enough
# to import it. If this file ever starts failing with AttributeError on one of
# these, module-level bpy usage has leaked out of a function.
sys.modules.setdefault("bpy", types.ModuleType("bpy"))
sys.modules.setdefault("_framing", types.ModuleType("_framing"))
sys.modules.setdefault("_ifc_import", types.ModuleType("_ifc_import"))

import _stage  # noqa: E402

FAILURES = []


def check(label: str, condition: bool, detail=None) -> None:
    if condition:
        print(f"  ok   {label}")
    else:
        print(f"  FAIL {label}\n       {detail}")
        FAILURES.append(label)


def main() -> int:
    with tempfile.TemporaryDirectory() as tmp:
        lights = Path(tmp) / "world"
        lights.mkdir()
        # Contents are irrelevant -- the resolver checks existence, and loading
        # is the caller's problem (and its own degrade path).
        (lights / "studio.exr").write_bytes(b"not-really-an-exr")
        custom = Path(tmp) / "mine.hdr"
        custom.write_bytes(b"nor-is-this")
        lights_dir = str(lights)

        r = _stage.resolve_environment("studio", lights_dir)
        check("a bundled name resolves to the bundled file",
              r["source"] == "blender-studiolight", r)
        check("a resolved environment carries no fallback reason",
              r["fallback-reason"] is None, r)

        r = _stage.resolve_environment("STUDIO", lights_dir)
        check("bundled names are case-insensitive",
              r["source"] == "blender-studiolight", r)

        r = _stage.resolve_environment("gradient", lights_dir)
        check("`gradient` is an explicit choice, not a degrade",
              r["source"] == "procedural-gradient" and r["fallback-reason"] is None, r)

        r = _stage.resolve_environment("", lights_dir)
        check("empty input takes the default", r["resolved"] == "studio", r)

        r = _stage.resolve_environment(None, lights_dir)
        check("None input takes the default", r["resolved"] == "studio", r)

        r = _stage.resolve_environment(str(custom), lights_dir)
        check("an existing path resolves as a custom environment",
              r["source"] == "custom-path", r)

        r = _stage.resolve_environment("nope", lights_dir)
        check("an unknown name degrades to the gradient",
              r["source"] == "procedural-gradient", r)
        check("an unknown name says it was not recognised",
              "unknown environment" in (r["fallback-reason"] or ""), r)

        r = _stage.resolve_environment("/no/such/file.exr", lights_dir)
        check("a missing file degrades to the gradient",
              r["source"] == "procedural-gradient", r)
        check("a missing file is reported as a path, not as an unknown name",
              "not found" in (r["fallback-reason"] or ""), r)

        r = _stage.resolve_environment("courtyard", lights_dir)
        check("a bundled name whose file is absent degrades, with a reason",
              r["source"] == "procedural-gradient" and bool(r["fallback-reason"]), r)

        r = _stage.resolve_environment("studio", None)
        check("a Blender shipping no studiolights degrades, with a reason",
              r["source"] == "procedural-gradient" and bool(r["fallback-reason"]), r)

        # Precedence: reserved names are checked BEFORE the path branch, so a
        # local file named `studio` must not shadow the bundled environment.
        shadow = Path(tmp) / "studio"
        shadow.write_bytes(b"a decoy")
        import os

        cwd = os.getcwd()
        try:
            os.chdir(tmp)
            r = _stage.resolve_environment("studio", lights_dir)
            check("a local file named `studio` does not shadow the bundled one",
                  r["source"] == "blender-studiolight", r)
        finally:
            os.chdir(cwd)

        for name in _stage.STUDIOLIGHTS:
            r = _stage.resolve_environment(name, lights_dir)
            expected = "blender-studiolight" if name == "studio" else "procedural-gradient"
            if r["source"] != expected:
                check(f"`{name}` resolves consistently", False, r)
        check("every declared bundled name resolves without raising", True)

    print("\nENVIRONMENT RESOLUTION FAIL" if FAILURES else "\nENVIRONMENT RESOLUTION PASS")
    return 1 if FAILURES else 0


if __name__ == "__main__":
    raise SystemExit(main())
