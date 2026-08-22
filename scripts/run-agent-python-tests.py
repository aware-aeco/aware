#!/usr/bin/env python3
"""Run every Python test the repo carries, and fail if any of them fails.

The repo's agents are not all Rust and .NET. `20-agents/aeco/visualization/
blender/` ships seven `test_*.py` regression tests -- and NOTHING ran them. Not
`ci.yml`, not `release.yml`, not `cargo test`, which never leaves `cli/`. Their
green-ness rested entirely on whoever last remembered to run them by hand.

That is the same hole this repo has now closed four times, each after it had
already shipped a real defect through it:

  * `cli-connection-reader` (#343) -- `read-model` returned a different
    coordinate frame from `probe`, and the bridge's own tests encoded the wrong
    frame because nothing re-read them;
  * `20-agents/aeco/engineering/steel-detailer-lookup` -- accumulated six
    `unwrap()` calls against CLAUDE.md §Code style and sat on edition 2021
    against the 2024 pin, with every check green;
  * six of seven .NET suites (`cli/tests/dotnet_suites_gate.rs`) -- 529 tests
    run by nothing; the v0.125.0 scene-contract change edited one of their files
    and shipped with nothing having executed it.

CLAUDE.md §Engineering rules -- "Verify before answering", "No corner-cutting" --
is what a suite nobody runs defeats: a green PR check is the claim that the
tests passed, and for these seven files that claim was never evaluated.

## What this runs, and what it can only skip

Three of the seven run on a bare runner. `test_look_resolution.py` and
`test_environment_resolution.py` need nothing but the standard library and were
written to run anywhere -- they say so in their own docstrings.
`test_scene_info_skips.py` defaults its `--aware-bin` and looks for Blender
before using it, so it starts, gets through its imports, and self-skips: real
coverage of that file's syntax, imports and startup path, which an earlier
version of this runner threw away by pre-skipping every file whose text
mentioned the flag (Codex review, PR #444).

The remaining four need a host CI does not have: a real Blender (with `bpy`, and
`ifcopenshell` for two of them), or an `--aware-bin` they mark `required=True`.
This reports them as skipped BY NAME rather than folding them into a pass count
-- a silent skip is how "covered" and "never ran" come to look alike, which is
the failure this whole script exists to prevent.

So CI proves the host-free tests and proves the rest are still *invokable*. That
second half is not nothing: it is exactly what had rotted. Before this,
`test_material_resolution.py` and `test_import_reconciliation.py` died on a
`ModuleNotFoundError` traceback the moment they were launched outside Blender,
while their five siblings skipped cleanly -- so a contributor without Blender
could not tell "needs a host" from "broken", and nothing said which.

## Contract

Exit status of a test file:
  * `0`  -- passed (or self-skipped and said so; several print `SKIP:` and exit
           0, which predates this runner and stays valid -- the last line of
           their output is echoed below so the log still distinguishes them);
  * `77` -- skipped, the conventional "could not run here" code `run_smoke.py`
           already documents and uses for exactly this;
  * anything else -- failed.

This script exits non-zero if any test failed, and ALSO if discovery found no
tests at all. A gate that silently matches nothing reports a clean repo while
checking exactly nothing, which is worse than no gate.

`--self-test` drives the discovery and classification below over a synthetic
tree with a known-passing, known-skipping and known-failing test in it, and
asserts each is called correctly. Run it first (`ci.yml` does): without it, a
runner that had stopped classifying failures as failures would report every
repo green forever.

Usage:
    python3 scripts/run-agent-python-tests.py [--aware-bin PATH]
    python3 scripts/run-agent-python-tests.py --self-test
    python3 scripts/run-agent-python-tests.py --list
"""

from __future__ import annotations

import argparse
import os
import subprocess
import sys
import tempfile
from pathlib import Path

# Skipped wholesale during discovery. `target/` and `node_modules/` are large
# enough to turn a millisecond walk into a slow one, and nothing under them is
# repo source; `.git` likewise.
SKIP_DIRS = {".git", "node_modules", "target", "bin", "obj", "__pycache__", ".venv"}

# The conventional "test could not run here" status. Chosen by `run_smoke.py`
# before this script existed, for the reason its docstring gives: so a human
# glancing at `$?` does not mistake "nothing to check here" for a red gate.
SKIP_EXIT = 77

# A hung test must not sit on a runner until the job's own timeout. Generous
# enough that a Blender-backed test on a machine that HAS Blender still fits.
PER_TEST_TIMEOUT_SECONDS = 900


def discover(root: Path) -> list[Path]:
    """Every `test_*.py` under `root`, sorted, excluding `SKIP_DIRS`.

    Named for what it is rather than for a directory: a Python test added under
    a *different* agent tomorrow is picked up with no edit here. That is the
    difference between this and the enumerate-the-known-suites gate that let
    `cli-sidecar/Ingest/Generator/Tests` sit unrun -- discovery cannot go stale
    the way a list can.
    """
    found: list[Path] = []
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames[:] = sorted(d for d in dirnames if d not in SKIP_DIRS)
        for name in sorted(filenames):
            if name.startswith("test_") and name.endswith(".py"):
                found.append(Path(dirpath) / name)
    return sorted(found)


def declares_aware_bin(path: Path) -> bool:
    """Whether `path` accepts an `--aware-bin` argument at all.

    Read from the file's own source: the flag is the argument name argparse is
    configured with, so its presence in the text is a fact about the file's
    interface. This decides only whether a supplied binary is worth FORWARDING —
    never whether the test can run without one. See [`run_one`].
    """
    try:
        return "--aware-bin" in path.read_text(encoding="utf-8", errors="replace")
    except OSError:
        return False


# argparse's exit status for a usage error, including a missing required option.
ARGPARSE_USAGE_EXIT = 2

# The literal argparse emits before listing the options it found missing.
ARGPARSE_REQUIRED = "the following arguments are required"


def missing_required_options(output: str) -> set[str]:
    """The options argparse itself named as missing, parsed from its error line.

    argparse prints two things on a usage error: a `usage:` block listing EVERY
    option the parser accepts, and then
    `error: the following arguments are required: --a, --b`. Only the second is
    evidence about what is missing.

    Reading the whole output instead conflated them (Codex review, PR #444): a
    test declaring an OPTIONAL `--aware-bin` while requiring some other argument
    prints `--aware-bin` in its usage block and `required: --fixture` in its
    error, so two independent substring checks called that failure "needs
    --aware-bin" and skipped a test that was genuinely broken — the precise
    silent-skip this runner exists to prevent.

    A wrapped or unparsable list yields nothing here, which classifies the run
    as failed. That is the safe direction: a skip must be earned.
    """
    for line in output.splitlines():
        _, separator, listed = line.partition(ARGPARSE_REQUIRED + ":")
        if separator:
            return {item.strip() for item in listed.split(",") if item.strip()}
    return set()


def run_one(path: Path, aware_bin: str | None) -> tuple[str, str]:
    """Run one test file. Returns `(status, detail)`.

    `status` is one of `passed` / `skipped` / `failed`.

    Each test runs with its own directory as the working directory: several
    resolve fixture paths relative to `__file__` but shell out to Blender, and
    the ones that do not are indifferent to it.

    **Whether a test needs `--aware-bin` is asked of the test, not guessed from
    its source** (Codex review, PR #444). Mentioning the flag and *requiring* it
    are different facts, and this originally conflated them: it pre-skipped
    every file whose text contained the string. `test_scene_info_skips.py`
    declares `--aware-bin` with `default="aware"` and checks for Blender before
    touching the binary, so it can run and self-skip on a hosted runner — and
    pre-skipping it meant a syntax, import or startup regression in that test
    stayed green. Its two siblings really do declare `required=True`.

    So the test is launched, and a skip requires all three of: argparse's own
    exit status, `--aware-bin` named in argparse's own required-arguments list
    (see [`missing_required_options`] — not merely present in the output), and
    no binary having been supplied to begin with. Every other non-zero exit
    stays a failure.
    """
    argv = [sys.executable, str(path)]
    if aware_bin and declares_aware_bin(path):
        argv += ["--aware-bin", aware_bin]

    try:
        completed = subprocess.run(
            argv,
            cwd=path.parent,
            capture_output=True,
            text=True,
            timeout=PER_TEST_TIMEOUT_SECONDS,
        )
    except subprocess.TimeoutExpired:
        return "failed", f"timed out after {PER_TEST_TIMEOUT_SECONDS}s"

    output = (completed.stdout or "") + (completed.stderr or "")
    last_line = next(
        (line.strip() for line in reversed(output.splitlines()) if line.strip()), ""
    )

    if completed.returncode == 0:
        return "passed", last_line
    if completed.returncode == SKIP_EXIT:
        return "skipped", last_line or f"exit {SKIP_EXIT}"
    if (
        not aware_bin
        and completed.returncode == ARGPARSE_USAGE_EXIT
        and "--aware-bin" in missing_required_options(output)
    ):
        return "skipped", "needs --aware-bin (not supplied)"
    return "failed", f"exit {completed.returncode}\n{output.rstrip()}"


def run_all(root: Path, aware_bin: str | None) -> int:
    """Run every discovered test under `root`; return a process exit code."""
    tests = discover(root)
    if not tests:
        print(
            f"ERROR: no test_*.py files found under {root} — this runner is "
            "wired into CI as a gate, and a gate that matches nothing reports a "
            "clean repo while checking nothing. If the tests genuinely moved, "
            "point this at their new home; do not leave it matching zero.",
            file=sys.stderr,
        )
        return 1

    print(f"discovered {len(tests)} Python test file(s) under {root}\n")

    passed: list[str] = []
    skipped: list[tuple[str, str]] = []
    failed: list[tuple[str, str]] = []

    for path in tests:
        relative = path.relative_to(root).as_posix()
        status, detail = run_one(path, aware_bin)
        if status == "passed":
            passed.append(relative)
            print(f"PASS  {relative}  {detail}")
        elif status == "skipped":
            skipped.append((relative, detail))
            print(f"SKIP  {relative}  {detail}")
        else:
            failed.append((relative, detail))
            print(f"FAIL  {relative}  {detail}")

    print(f"\n{len(passed)} passed, {len(skipped)} skipped, {len(failed)} failed")

    # Name every skip. Folding them into a total is how a suite that stopped
    # running anything anywhere still reads as a green check.
    for relative, detail in skipped:
        print(f"  skipped: {relative} — {detail}")

    if failed:
        print("\nfailures:", file=sys.stderr)
        for relative, detail in failed:
            print(f"\n--- {relative} ---\n{detail}", file=sys.stderr)
        return 1
    return 0


# ---------------------------------------------------------------------------
# Self-test: the negative control.
# ---------------------------------------------------------------------------

_PASSING = "import sys\nprint('all good')\nsys.exit(0)\n"
_SKIPPING = "import sys\nprint('SKIP: no host here')\nsys.exit(77)\n"
_FAILING = "import sys\nprint('assertion blew up')\nsys.exit(1)\n"
_CRASHING = "raise ModuleNotFoundError('No module named \\'bpy\\'')\n"
_NEEDS_BIN = (
    "import argparse, sys\n"
    "p = argparse.ArgumentParser()\n"
    "p.add_argument('--aware-bin', required=True)\n"
    "a = p.parse_args()\n"
    "print('got', a.aware_bin)\n"
    "sys.exit(0)\n"
)
# Declares `--aware-bin` but defaults it — the shape of
# `test_scene_info_skips.py`. It must RUN when no binary is supplied, not be
# pre-skipped for merely containing the string (Codex review, PR #444).
_OPTIONAL_BIN = (
    "import argparse, sys\n"
    "p = argparse.ArgumentParser()\n"
    "p.add_argument('--aware-bin', default='aware')\n"
    "a = p.parse_args()\n"
    "print('ran with', a.aware_bin)\n"
    "sys.exit(0)\n"
)
# Exits 2 for a reason that has nothing to do with a missing argument. The
# skip-on-usage-error branch must not swallow this.
_USAGE_EXIT_FAILURE = "import sys\nprint('unrelated exit 2')\nsys.exit(2)\n"
# Declares `--aware-bin` as OPTIONAL but requires a different argument. argparse
# prints `--aware-bin` in its usage block and `required: --fixture` in its
# error, so a runner matching those two substrings independently calls this
# "needs --aware-bin" and skips a genuinely broken test (Codex review, PR #444).
_OTHER_REQUIRED_ARG = (
    "import argparse, sys\n"
    "p = argparse.ArgumentParser()\n"
    "p.add_argument('--aware-bin', default='aware')\n"
    "p.add_argument('--fixture', required=True)\n"
    "p.parse_args()\n"
    "sys.exit(0)\n"
)


def _write(root: Path, relative: str, source: str) -> None:
    path = root / relative
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(source, encoding="utf-8")


def self_test() -> int:
    """Prove this runner still reports a failing test as a failure.

    Every assertion here exists because its absence would make the CI step look
    like a gate while being none:

      * a failing test must make the whole run fail -- without this, a runner
        that stopped propagating exit codes greenlights a broken suite forever;
      * a *crashing* test (an uncaught traceback, which is exactly how
        `test_material_resolution.py` behaved off-Blender) must count as failed,
        not as skipped -- the distinction this whole change turns on;
      * `77` must count as skipped, or the host-dependent tests turn CI red on
        every runner without Blender and the step gets deleted within a week;
      * discovery must find tests nested at any depth, must ignore non-`test_*`
        files, and must not descend into `node_modules`/`target`;
      * an empty tree must FAIL rather than pass vacuously;
      * `--aware-bin` must reach the test that declares it and only that one.
    """
    failures: list[str] = []

    def check(condition: bool, label: str) -> None:
        print(f"{'PASS' if condition else 'FAIL'}  {label}")
        if not condition:
            failures.append(label)

    with tempfile.TemporaryDirectory(prefix="aware-pytest-selftest-") as tmp:
        root = Path(tmp)
        _write(root, "agent/tests/test_pass.py", _PASSING)
        _write(root, "agent/tests/test_skip.py", _SKIPPING)
        _write(root, "deep/nested/further/test_pass_two.py", _PASSING)
        _write(root, "agent/tests/helper.py", _FAILING)  # not a test_*.py
        _write(root, "agent/tests/pass_test.py", _FAILING)  # wrong convention
        _write(root, "node_modules/pkg/test_ignored.py", _FAILING)
        _write(root, "target/debug/test_ignored.py", _FAILING)

        discovered = [p.relative_to(root).as_posix() for p in discover(root)]
        check(
            discovered
            == [
                "agent/tests/test_pass.py",
                "agent/tests/test_skip.py",
                "deep/nested/further/test_pass_two.py",
            ],
            f"discovery finds exactly the nested test_*.py files (got {discovered})",
        )

        check(run_all(root, None) == 0, "a tree of passing/skipping tests succeeds")

        status, _ = run_one(root / "agent/tests/test_pass.py", None)
        check(status == "passed", "exit 0 classifies as passed")
        status, _ = run_one(root / "agent/tests/test_skip.py", None)
        check(status == "skipped", "exit 77 classifies as skipped")

        # The negative control proper: put a failing test in and the runner must
        # go red. If this ever passes, the CI step below is decoration.
        _write(root, "agent/tests/test_broken.py", _FAILING)
        status, _ = run_one(root / "agent/tests/test_broken.py", None)
        check(status == "failed", "a non-zero, non-77 exit classifies as failed")
        check(run_all(root, None) == 1, "one failing test fails the whole run")

        # An uncaught ImportError traceback is a failure, not a skip. This is
        # the exact shape the two Blender tests had before this change.
        _write(root, "agent/tests/test_crash.py", _CRASHING)
        status, _ = run_one(root / "agent/tests/test_crash.py", None)
        check(status == "failed", "an uncaught traceback classifies as failed")

    with tempfile.TemporaryDirectory(prefix="aware-pytest-selftest-empty-") as tmp:
        check(
            run_all(Path(tmp), None) == 1,
            "a tree with no tests fails rather than passing vacuously",
        )

    with tempfile.TemporaryDirectory(prefix="aware-pytest-selftest-bin-") as tmp:
        root = Path(tmp)
        _write(root, "tests/test_needs_bin.py", _NEEDS_BIN)
        _write(root, "tests/test_plain.py", _PASSING)

        _write(root, "tests/test_optional_bin.py", _OPTIONAL_BIN)
        _write(root, "tests/test_usage_exit.py", _USAGE_EXIT_FAILURE)
        _write(root, "tests/test_other_required.py", _OTHER_REQUIRED_ARG)

        check(
            declares_aware_bin(root / "tests/test_needs_bin.py")
            and not declares_aware_bin(root / "tests/test_plain.py"),
            "the --aware-bin declaration is read from the test's own source",
        )
        status, detail = run_one(root / "tests/test_needs_bin.py", None)
        check(
            status == "skipped" and "needs --aware-bin" in detail,
            "a test REQUIRING --aware-bin is skipped, not failed, when none is given",
        )
        # Codex review, PR #444: declaring the flag is not requiring it. A test
        # that defaults it must actually run, or a syntax/import/startup
        # regression in it stays green behind a pre-emptive skip.
        status, detail = run_one(root / "tests/test_optional_bin.py", None)
        check(
            status == "passed" and "ran with aware" in detail,
            "a test that merely DECLARES --aware-bin still runs when none is given",
        )
        # ...and the skip branch must not swallow an unrelated exit 2.
        status, _ = run_one(root / "tests/test_usage_exit.py", None)
        check(
            status == "failed",
            "an exit 2 that is not a missing-argument error stays a failure",
        )
        # The skip must be earned by argparse NAMING --aware-bin as missing, not
        # by the flag appearing anywhere in the usage block.
        check(
            missing_required_options(
                "usage: t [--aware-bin AWARE_BIN] --fixture F\n"
                "t: error: the following arguments are required: --fixture\n"
            )
            == {"--fixture"},
            "only argparse's required-arguments list is read, not its usage block",
        )
        status, _ = run_one(root / "tests/test_other_required.py", None)
        check(
            status == "failed",
            "a test requiring a DIFFERENT argument fails, and is not mis-skipped "
            "as needing --aware-bin",
        )
        status, detail = run_one(root / "tests/test_needs_bin.py", "/some/aware")
        check(
            status == "passed" and "/some/aware" in detail,
            "--aware-bin is forwarded to the test that declares it",
        )

        # The unrelated-exit-2 test is a genuine failure, so the tree must be red.
        check(
            run_all(root, "/some/aware") == 1,
            "a tree containing a real failure fails even with a binary supplied",
        )

    print(f"\nself-test: {'FAILED' if failures else 'OK'}")
    for label in failures:
        print(f"  failed: {label}", file=sys.stderr)
    return 1 if failures else 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument(
        "--root",
        default=str(Path(__file__).resolve().parent.parent),
        help="tree to discover tests under (default: the repo root)",
    )
    parser.add_argument(
        "--aware-bin",
        default=None,
        help="path to a built `aware` binary, forwarded to tests that take one",
    )
    parser.add_argument(
        "--self-test",
        action="store_true",
        help="prove this runner still reports a failing test as a failure",
    )
    parser.add_argument(
        "--list",
        action="store_true",
        help="print the discovered test files and exit",
    )
    args = parser.parse_args()

    if args.self_test:
        return self_test()

    root = Path(args.root).resolve()
    if args.list:
        tests = discover(root)
        for path in tests:
            print(path.relative_to(root).as_posix())
        return 0 if tests else 1

    return run_all(root, args.aware_bin)


if __name__ == "__main__":
    sys.exit(main())
