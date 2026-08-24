#!/usr/bin/env python3
"""Run every Python test the repo carries, and fail if any of them fails.

The repo's agents are not all Rust and .NET. `20-agents/aeco/visualization/
blender/` ships seven `test_*.py` regression tests -- and NOTHING ran them. Not
`ci.yml`, not `release.yml`, not `cargo test`, which never leaves `cli/`. Their
green-ness rested entirely on whoever last remembered to run them by hand.

That is the fourth instance of one hole, and each of the previous three was
found only after it had already shipped a real defect through it:

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

The remaining four are not run HERE. Two need a real Blender (with
`ifcopenshell`); two mark `--aware-bin` `required=True` and need a built CLI.
Neither is impossible on a hosted runner -- `bpy` and `ifcopenshell` both ship
manylinux wheels, and CI already builds the CLI in another job -- so this is a
cost trade-off, not an impossibility: a 355 MB `bpy` download per run, or a
second Rust build, to reach tests that would then still need a display-less
render path. Stated plainly because "the runner cannot" would be false.

Every one of them is reported as skipped BY NAME rather than folded into a pass
count -- a silent skip is how "covered" and "never ran" come to look alike,
which is the failure this whole script exists to prevent.

So CI proves the host-free tests and proves the rest are still *invokable*. That
second half is not nothing: it is exactly what had rotted. Before this,
`test_material_resolution.py` and `test_import_reconciliation.py` died on a
`ModuleNotFoundError` traceback the moment they were launched outside Blender.
No sibling did: `test_scene_info_skips.py` announces the missing host and exits
0, `test_camera_lens.py` and `test_ground_isolation.py` do the same once given
`--aware-bin`, and the remaining two need no host at all. So a contributor
without Blender could not tell "needs a host" from "broken", and nothing said
which.

## Contract

Exit status of a test file:
  * `0`  -- passed, UNLESS its last line announces a skip, which several tests
           predating `SKIP_EXIT` do; those are counted as skips, because
           counting them as passes made the headline claim verification that
           had not happened;
  * `77` -- skipped, the conventional "could not run here" code `run_smoke.py`
           already documents and uses for exactly this;
  * anything else -- failed.

This script exits non-zero if any test failed, if discovery found no tests at
all, and if no discovered test ran to a pass. All three are the same rule: a
gate that verified nothing must not report a clean repo. The last of them is
the one review had to add -- refusing an empty tree while accepting a tree in
which every single test skipped was the same hole, one step further along.

`--self-test` drives the discovery and classification below over a synthetic
tree carrying a known-passing, known-skipping, known-failing and known-CRASHING
test -- plus the argparse shapes that review found being mis-skipped -- and
asserts each is called correctly. The crashing case is the load-bearing one: an
uncaught traceback must count as failed, not skipped. Run it first (`ci.yml` does): without it, a
runner that had stopped classifying failures as failures would report every
repo green forever.

Usage:
    python3 scripts/run-agent-python-tests.py [--aware-bin PATH]
    python3 scripts/run-agent-python-tests.py --self-test
    python3 scripts/run-agent-python-tests.py --list
"""

from __future__ import annotations

import argparse
import ast
import os
import subprocess
import sys
import tempfile
from collections.abc import Iterator
from pathlib import Path

# Skipped wholesale during discovery: build output, dependency trees and version
# control, none of which hold repo source, and the big ones (`target/`,
# `node_modules/`, virtualenvs) turn a millisecond walk into a slow one.
#
# `bin` and `obj` were here for .NET build output and have been REMOVED (review,
# PR #444): these names match by directory name at ANY depth, so a real test at
# `.../tests/bin/test_x.py` was silently undiscovered — a gate whose whole thesis
# is that silent skips are the enemy, skipping silently. Nothing in this repo
# keeps Python under a `bin/`, and the .NET build output holds no `test_*.py`, so
# dropping them costs nothing and closes that.
#
# The virtualenv names are the other half, and matter for the opposite reason:
# `venv/` (not just `.venv/`) is untracked-but-expected local state, and without
# it discovery reached `site-packages` and EXECUTED vendored third-party
# `test_*.py` as a subprocess.
SKIP_DIRS = {
    ".git",
    ".tox",
    ".venv",
    "__pycache__",
    "node_modules",
    "site-packages",
    "target",
    "venv",
}

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
    a *different* agent tomorrow is picked up with no edit here.

    The convention it keys on is `test_*.py`, and that is the part to be careful
    about. `dotnet_suites_gate.rs` records a gate that matched `*.Tests.csproj`
    and reported the repo clean while `cli-sidecar/Ingest/Generator/Tests` --
    whose file is plain `Tests.csproj` -- ran nowhere. A filename convention is
    not a fact about the file. Python has no equivalent of `Test.Sdk` to read
    instead, so this keeps the convention and makes it load-bearing elsewhere:
    [`looks_like_an_unrunnable_pytest_file`] refuses a discovered file that
    would not actually execute, and [`run_all`] refuses a run in which nothing
    executed.
    """
    found: list[Path] = []
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames[:] = sorted(d for d in dirnames if d not in SKIP_DIRS)
        for name in sorted(filenames):
            if name.startswith("test_") and name.endswith(".py"):
                found.append(Path(dirpath) / name)
    return sorted(found)


def read_source(path: Path) -> str:
    """The test file's text.

    An `OSError` is deliberately NOT swallowed (review, PR #444). A file the
    runner cannot read — a broken symlink named `test_x.py`, which `os.walk`
    happily lists, or a permissions problem — is a defect, and answering a
    default to "what does this file say?" is the stub-value-on-error this whole
    script exists to argue against. Let it surface with the path in the
    traceback.
    """
    return path.read_text(encoding="utf-8", errors="replace")


def declares_aware_bin(source: str) -> bool:
    """Whether `--aware-bin` appears anywhere in the file's text.

    A plain substring search, docstrings and comments included — deliberately
    stated that way rather than as "the argument argparse is configured with",
    which it is not (review, PR #444): two files in the same directory name the
    flag in prose.

    That looseness is safe because of what this decides — only whether a
    SUPPLIED binary is worth forwarding, never whether the test can run without
    one. A flag forwarded to a test that does not accept it draws
    `unrecognized arguments`, exit 2, and no required-arguments line, so it
    lands as failed rather than as a silent skip. See [`run_one`].
    """
    return "--aware-bin" in source


# argparse's exit status for a usage error, including a missing required option.
ARGPARSE_USAGE_EXIT = 2

# The literal argparse emits before listing the options it found missing.
ARGPARSE_REQUIRED = "the following arguments are required"


def missing_required_options(stderr: str, program: str) -> set[str]:
    """The options argparse itself named as missing, parsed from its error line.

    Three narrowings, each closing a measured silent-skip (Codex review and
    review panel, PR #444). Every one of them was reachable, and every one
    turned a genuinely broken test into a green skip:

      * **The error line, not the usage block.** argparse prints a `usage:` block
        naming EVERY option the parser accepts, then
        `error: the following arguments are required: --a`. Only the second says
        what is missing. Matching them independently meant a test declaring an
        optional `--aware-bin` while requiring `--fixture` satisfied both halves
        and was skipped.
      * **argparse's line, not the test's.** The line must start with
        `<program>: error: `, argparse's own prefix, and is read from STDERR
        only. A test asserting on some other tool's `--aware-bin` handling
        prints that sentence on stdout as data; without the anchor the runner
        read the test's own output as a verdict about the test.
      * **The last such line, not the first.** A test that prints one before
        failing for real would otherwise shadow argparse's.

    An unparsable or wrapped list yields nothing here, which classifies the run
    as failed. That is the safe direction: a skip must be earned.
    """
    prefix = f"{program}: error: {ARGPARSE_REQUIRED}:"
    for line in reversed(stderr.splitlines()):
        stripped = line.strip()
        if stripped.startswith(prefix):
            listed = stripped[len(prefix) :]
            return {item.strip() for item in listed.split(",") if item.strip()}
    return set()


def looks_like_an_unrunnable_pytest_file(source: str) -> bool:
    """`True` for a file whose `def test_` bodies a bare interpreter never runs.

    This runner executes each file as a script. A file written in the dominant
    pytest convention — `def test_x(): assert ...` with no `if __name__` block
    and no `sys.exit(main())` — therefore defines its functions, calls none of
    them, and exits 0 having asserted NOTHING (review, PR #444).

    That matters precisely because discovery is sold on picking up a test added
    under a different agent tomorrow with no edit here: the next contributor is
    likely to write one this way and get a permanent, meaningless green. Naming
    it is better than running it wrong — the seven tests in the repo today are
    all script-style and unaffected.

    Decided from the module's AST, not from substrings (Codex review, PR #447).
    The first version asked whether the text contained `__main__` or `sys.exit(`,
    and was wrong in both directions: a perfectly good script-style test that
    defines `def test_x()` and then calls `test_x()` at top level carries
    neither marker and would have been refused unrun, while either string
    appearing in a comment, a docstring or a test body would have waved through
    a file that still executes nothing. The AST answers the question actually
    being asked — is there a top-level statement that CALLS anything — so both
    directions come out right.

    A file that will not parse is not pre-judged here: it is left to run and
    fail on its own syntax error, which is louder and more accurate than
    anything this function could say about it.
    """
    try:
        module = ast.parse(source)
    except SyntaxError:
        return False

    def is_test_function(node: ast.AST) -> bool:
        return isinstance(
            node, (ast.FunctionDef, ast.AsyncFunctionDef)
        ) and node.name.startswith("test_")

    def module_executed(body: list[ast.stmt]) -> Iterator[ast.stmt]:
        """Statements that run when the module is imported, control flow included.

        Recurses through `if` / `try` / `with` / `for` / `while`, because a
        declaration inside one still happens at import — `if sys.platform ==
        "win32": def test_windows(): ...` is a real shape, and scanning only
        `module.body` missed it even on the platform where the function is
        created (Codex review, PR #447).

        It deliberately does NOT descend into `def` or `class` bodies: those run
        when called or defined-as-a-namespace, not as module-level execution, so
        a `def test_x` nested inside a function is not a module-level test.
        """
        for statement in body:
            yield statement
            if isinstance(statement, (ast.If, ast.Try, ast.With, ast.For, ast.While)):
                yield from module_executed(statement.body)
                yield from module_executed(getattr(statement, "orelse", []))
                yield from module_executed(getattr(statement, "finalbody", []))
                for handler in getattr(statement, "handlers", []):
                    yield from module_executed(handler.body)

    # Both pytest shapes, not just the bare-function one (Codex review, PR #447).
    # `class TestMath: def test_bad(self): ...` is the other common form, and
    # looking only at module-level `FunctionDef`s missed it entirely — such a
    # file was executed as a script and recorded exit 0 with no method having
    # run, which is the exact silent pass this check exists to refuse.
    defines_a_test = any(
        is_test_function(node)
        or (
            isinstance(node, ast.ClassDef)
            and any(is_test_function(member) for member in node.body)
        )
        for node in module_executed(module.body)
    )
    if not defines_a_test:
        return False

    # The declared test names, split by sync/async and gathered across
    # module-executed control flow so a test declared under `if sys.platform`
    # still counts. `class` methods are collected too, but only for the
    # `defines_a_test` gate above — they cannot be invoked by a bare module-level
    # name, so a class-based file with no runner is correctly refused below.
    sync_tests: set[str] = set()
    async_tests: set[str] = set()
    for node in module_executed(module.body):
        if isinstance(node, ast.AsyncFunctionDef) and node.name.startswith("test_"):
            async_tests.add(node.name)
        elif isinstance(node, ast.FunctionDef) and node.name.startswith("test_"):
            sync_tests.add(node.name)

    # The two stdlib calls that actually drive a coroutine to completion:
    # `asyncio.run(coro)` and `loop.run_until_complete(coro)`. Matched on the
    # callee's final name so a bound `loop.run_until_complete` qualifies too.
    coroutine_drivers = {"run", "run_until_complete"}

    def called_name(call: ast.Call) -> str | None:
        """The bare name a call invokes: `f()` -> 'f', `a.b.f()` -> 'f'."""
        if isinstance(call.func, ast.Name):
            return call.func.id
        if isinstance(call.func, ast.Attribute):
            return call.func.attr
        return None

    def invokes_a_test(node: ast.AST, driven: bool) -> bool:
        """`True` if `node`, as it runs at import, executes a declared test body.

        Not "does it call *something*" — the earlier version of this check
        accepted any module-level call, so `logging.basicConfig()` beside an
        uncalled failing `test_*` was reported as passing, and the coroutine
        guard was bypassed by `print(test_broken())` where the outer call
        tripped the unconditional accept (Codex review, PR #447). The question
        is now tied to the declared test names.

        A sync test counts wherever it is *called*: `test_x()`, `print(test_x())`
        and `sys.exit(test_x())` all evaluate `test_x()` before the outer call,
        so the assertions run. A coroutine test counts only when `driven` — under
        an `await`, or in the argument of a driver call — because calling it bare
        merely builds a coroutine the interpreter discards after a warning.

        `driven` propagates into the arguments of a driver call and through an
        `await`, and never into a `def`/`class` body, whose statements do not run
        where they are written.
        """
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
            return False
        if isinstance(node, ast.Await):
            return any(
                invokes_a_test(child, True) for child in ast.iter_child_nodes(node)
            )
        if isinstance(node, ast.Call):
            name = called_name(node)
            if name in sync_tests:
                return True
            if name in async_tests and driven:
                return True
            child_driven = driven or name in coroutine_drivers
            args = [*node.args, *(kw.value for kw in node.keywords)]
            if any(invokes_a_test(arg, child_driven) for arg in args):
                return True
            return invokes_a_test(node.func, driven)
        return any(
            invokes_a_test(child, driven) for child in ast.iter_child_nodes(node)
        )

    def is_main_guard(statement: ast.stmt) -> bool:
        """`True` for `if __name__ == "__main__":` — the documented entry point.

        The error this check raises tells authors to add exactly this guard, so a
        file that has one carrying any call is trusted to be a real entry point
        (`if __name__ == "__main__": sys.exit(main())` is the repo's own pattern,
        and `main` is not a `test_*` name to resolve). The trust is bounded to a
        guard that contains a call — an empty `__main__` still runs nothing — and
        stated as the one residual: this check cannot follow `main()` inside to
        confirm it reaches the tests, so a guard is taken at its word.
        """
        if not isinstance(statement, ast.If):
            return False
        if not isinstance(statement.test, ast.Compare) or len(statement.test.ops) != 1:
            return False
        if not isinstance(statement.test.ops[0], ast.Eq):
            return False
        sides = (statement.test.left, statement.test.comparators[0])
        has_name = any(
            isinstance(s, ast.Name) and s.id == "__name__" for s in sides
        )
        has_main = any(
            isinstance(s, ast.Constant) and s.value == "__main__" for s in sides
        )
        return has_name and has_main

    def has_runner() -> bool:
        for statement in module_executed(module.body):
            if is_main_guard(statement) and any(
                isinstance(inner, ast.Call) for inner in ast.walk(statement)
            ):
                return True
        return any(invokes_a_test(statement, False) for statement in module.body)

    return not has_runner()


def run_one(path: Path, aware_bin: str | None) -> tuple[str, str]:
    """Run one test file. Returns `(status, detail)`.

    `status` is one of `passed` / `skipped` / `failed`.

    Each test runs with its own directory as the working directory. Every test
    in the repo today resolves its paths from `__file__` and is indifferent to
    this; setting it means one added later that reads a fixture by relative path
    behaves the same under the runner as it does when run by hand.

    **Whether a test needs `--aware-bin` is asked of the test, not guessed from
    its source** (Codex review, PR #444). Mentioning the flag and *requiring* it
    are different facts, and this originally conflated them: it pre-skipped
    every file whose text contained the string. `test_scene_info_skips.py`
    declares `--aware-bin` with `default="aware"` and checks for Blender before
    touching the binary, so it can run and self-skip on a hosted runner — and
    pre-skipping it meant a syntax, import or startup regression in that test
    stayed green. Its two siblings really do declare `required=True`.

    So the test is launched, and a skip requires ALL of: no binary supplied,
    argparse's own exit status, and `--aware-bin` being the *entire* set of
    options argparse named as missing (see [`missing_required_options`]).
    Requiring the whole set — rather than membership in it — is what stops a
    test missing `--aware-bin` AND `--fixture` from reading as a clean skip
    when only one of the two is something this runner could ever supply.
    """
    source = read_source(path)
    if looks_like_an_unrunnable_pytest_file(source):
        return "failed", (
            "defines `def test_` functions but never calls them — this runner "
            "executes each file as a script, so a bare interpreter would exit 0 "
            "having asserted nothing. Give it an `if __name__ == \"__main__\"` "
            "entry point (see the other tests in this repo) rather than relying "
            "on a pytest collector that nothing here runs."
        )

    argv = [sys.executable, str(path)]
    if aware_bin and declares_aware_bin(source):
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
        # Exit 0 whose last line announces a skip IS a skip (review, PR #444).
        # Five tests here predate `SKIP_EXIT` and self-skip with `return 0`, and
        # counting those as passes made the headline read "3 passed" when two
        # files had verified anything — while the announced skip never reached
        # the named-skips list below. That is exactly the "covered" / "never ran"
        # conflation this script exists to prevent, committed by the script.
        # The `SKIP:` announcement format specifically, colon included (Codex
        # review, PR #447). A bare `SKIP` prefix also swallowed ordinary output
        # like `SKIPPED CHECKS: 0` or `skipping optional checks: none`, which
        # would move a genuinely passing test into the skip column — and, if it
        # were the only host-free test, the execution floor below would then
        # fail an otherwise green suite.
        if last_line.upper().startswith("SKIP:"):
            return "skipped", last_line
        return "passed", last_line
    if completed.returncode == SKIP_EXIT:
        return "skipped", last_line or f"exit {SKIP_EXIT}"
    if (
        not aware_bin
        and completed.returncode == ARGPARSE_USAGE_EXIT
        and missing_required_options(completed.stderr or "", path.name)
        == {"--aware-bin"}
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

    # The floor is on EXECUTION, not on discovery (review, PR #444). Refusing an
    # empty tree above was the right instinct applied one step too early: a run
    # where every discovered file skipped has verified exactly as much as a run
    # that found no files, and returned 0 for it. That is reachable — delete the
    # two host-free tests and five skipping files remain, green forever — and it
    # is the same "reports a clean repo while checking nothing" this script was
    # written to make impossible.
    if not passed:
        print(
            f"\nERROR: {len(tests)} test file(s) were discovered and NONE of them "
            "ran to a pass — every one skipped. A run that verified nothing is "
            "not a green run, whatever the skip reasons say. If the host-free "
            "tests were removed or renamed, this gate is now measuring nothing "
            "and needs re-pointing; if a host went missing on a machine that "
            "should have one, fix the host.",
            file=sys.stderr,
        )
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
# Requires `--aware-bin` AND another argument. `--aware-bin` IS in argparse's
# required list here, so membership is satisfied — but supplying a binary would
# still leave the test broken, so this must fail rather than read as a clean
# skip (review, PR #444).
_TWO_REQUIRED_ARGS = (
    "import argparse, sys\n"
    "p = argparse.ArgumentParser()\n"
    "p.add_argument('--aware-bin', required=True)\n"
    "p.add_argument('--fixture', required=True)\n"
    "p.parse_args()\n"
    "sys.exit(0)\n"
)
# Prints argparse's sentence as DATA on stdout and exits 2 — the shape of a test
# asserting on some other tool's `--aware-bin` handling. Reading the test's own
# output as a verdict about the test turned this into a green skip (review).
_ECHOES_ARGPARSE_TEXT = (
    "import sys\n"
    "print('expected: t: error: "
    "the following arguments are required: --aware-bin')\n"
    "print('REGRESSION: the wrapper accepted a missing binary')\n"
    "sys.exit(2)\n"
)
# pytest-style: defines tests, calls none. A bare interpreter exits 0 having
# asserted nothing, so this must be REFUSED rather than counted as a pass.
_PYTEST_STYLE = (
    "def test_two_plus_two():\n"
    "    assert 2 + 2 == 5, 'arithmetic is broken'\n"
    "\n"
    "def test_other():\n"
    "    raise AssertionError('also broken')\n"
)
# Exit 0 with a last line announcing a skip — the shape of the five tests that
# predate SKIP_EXIT. Must land in the skipped bucket, not the pass count.
_EXIT_ZERO_SKIP = "import sys\nprint('SKIP: no Blender available')\nsys.exit(0)\n"
# Passes, and happens to end on a line beginning with the letters SKIP. Matching
# a bare `SKIP` prefix moved this into the skip column, and with the execution
# floor that could fail an otherwise green suite (Codex review, PR #447).
_SKIPLIKE_BUT_PASSING = (
    "import sys\nprint('all checks ran')\nprint('SKIPPED CHECKS: 0')\nsys.exit(0)\n"
)
# Script-style: defines a test and CALLS it at top level. No `__main__` block and
# no `sys.exit(` anywhere, so the old substring heuristic refused it unrun even
# though its assertions execute (Codex review, PR #447).
_CALLS_ITS_TEST = (
    "def test_arithmetic():\n"
    "    assert 2 + 2 == 4\n"
    "    print('arithmetic holds')\n"
    "\n"
    "test_arithmetic()\n"
)
# Class-based pytest: no module-level `def test_`, so a check looking only at
# top-level functions missed it and the file ran as a script asserting nothing
# (Codex review, PR #447).
_PYTEST_CLASS_STYLE = (
    "class TestMath:\n"
    "    def test_bad(self):\n"
    "        assert 2 + 2 == 5\n"
)
# A test declared inside a conditional block. It IS created at import on the
# matching platform, so scanning only `module.body` missed it and the file ran
# as a script asserting nothing (Codex review, PR #447).
_CONDITIONAL_TEST = (
    "import sys\n"
    "\n"
    "if sys.platform is not None:\n"
    "    def test_conditional():\n"
    "        assert 2 + 2 == 5\n"
)
# A bare call to an `async def` builds a coroutine and runs nothing: Python
# exits 0 after a RuntimeWarning with the assertion never evaluated, so this
# must be refused, not recorded as a pass (Codex review, PR #447).
_UNAWAITED_ASYNC = (
    "async def test_broken():\n"
    "    assert 2 + 2 == 5\n"
    "\n"
    "test_broken()\n"
)
# The same file with the coroutine actually driven. This one DOES run, so it
# must be executed and allowed to fail on its own assertion — the control that
# keeps the fix above from simply refusing all async tests.
_DRIVEN_ASYNC = (
    "import asyncio\n"
    "\n"
    "async def test_broken():\n"
    "    assert 2 + 2 == 5\n"
    "\n"
    "asyncio.run(test_broken())\n"
)
# A top-level `try:` that only imports. The block is not an entry point — an
# Import is not a Call — so this file still asserts nothing and must be refused.
_TRY_IMPORT_ONLY = (
    "try:\n"
    "    import json\n"
    "except ImportError:\n"
    "    json = None\n"
    "\n"
    "def test_bad():\n"
    "    assert 2 + 2 == 5\n"
)
# Defines a failing test, calls it nowhere, but DOES make an unrelated
# module-level call. The old "any call is an entry point" rule reported this as
# passed; the call must resolve to the declared test to count (Codex review,
# PR #447).
_UNRELATED_CALL_ONLY = (
    "import logging\n"
    "\n"
    "logging.basicConfig()\n"
    "\n"
    "def test_bad():\n"
    "    assert 2 + 2 == 5\n"
)
# An async test whose coroutine is wrapped in an unrelated call rather than
# driven. `print(test_broken())` builds the coroutine, prints it, and never
# runs the body — the outer `print` call used to trip the accept.
_ASYNC_WRAPPED_UNDRIVEN = (
    "async def test_broken():\n"
    "    assert 2 + 2 == 5\n"
    "\n"
    "print(test_broken())\n"
)
# A `main()` dispatcher behind the documented `__main__` guard, in a file that
# also declares `test_*`. `main` is not a test name to resolve, so this is
# honored via the guard — and it genuinely runs, failing on its assertion.
_MAIN_DISPATCHER = (
    "def test_bad():\n"
    "    assert 2 + 2 == 5\n"
    "\n"
    "def main():\n"
    "    test_bad()\n"
    "\n"
    'if __name__ == "__main__":\n'
    "    main()\n"
)
# Defines a test, calls nothing — but mentions `__main__` and `sys.exit(` inside
# a docstring. The old heuristic read those as an entry point and waved it
# through; it still asserts nothing.
_FAKE_ENTRY_POINT_IN_PROSE = (
    '"""Run me with `python -m x` — the __main__ guard calls sys.exit(main()).\n'
    'Except it does not: there is no such guard below.\n"""\n'
    "\n"
    "def test_two_plus_two():\n"
    "    assert 2 + 2 == 5\n"
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
        # Must NOT be swallowed: `bin`/`obj` used to be pruned by name at any
        # depth, which hid a real test (review, PR #444).
        _write(root, "agent/tests/bin/test_under_bin.py", _PASSING)
        # Must be pruned: vendored third-party tests are not ours to execute.
        _write(root, "venv/lib/python3.11/site-packages/pkg/test_vendored.py", _FAILING)

        discovered = [p.relative_to(root).as_posix() for p in discover(root)]
        check(
            discovered
            == [
                "agent/tests/bin/test_under_bin.py",
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
        _write(root, "tests/test_two_required.py", _TWO_REQUIRED_ARGS)
        _write(root, "tests/test_echoes.py", _ECHOES_ARGPARSE_TEXT)
        _write(root, "tests/test_pytest_style.py", _PYTEST_STYLE)
        _write(root, "tests/test_exit_zero_skip.py", _EXIT_ZERO_SKIP)
        _write(root, "tests/test_skiplike_pass.py", _SKIPLIKE_BUT_PASSING)
        _write(root, "tests/test_calls_its_test.py", _CALLS_ITS_TEST)
        _write(root, "tests/test_fake_entry.py", _FAKE_ENTRY_POINT_IN_PROSE)
        _write(root, "tests/test_class_style.py", _PYTEST_CLASS_STYLE)
        _write(root, "tests/test_try_import.py", _TRY_IMPORT_ONLY)
        _write(root, "tests/test_conditional_decl.py", _CONDITIONAL_TEST)
        _write(root, "tests/test_unawaited_async.py", _UNAWAITED_ASYNC)
        _write(root, "tests/test_driven_async.py", _DRIVEN_ASYNC)
        _write(root, "tests/test_unrelated_call.py", _UNRELATED_CALL_ONLY)
        _write(root, "tests/test_async_wrapped.py", _ASYNC_WRAPPED_UNDRIVEN)
        _write(root, "tests/test_main_dispatcher.py", _MAIN_DISPATCHER)

        check(
            declares_aware_bin(read_source(root / "tests/test_needs_bin.py"))
            and not declares_aware_bin(read_source(root / "tests/test_plain.py")),
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
                "usage: t.py [--aware-bin AWARE_BIN] --fixture F\n"
                "t.py: error: the following arguments are required: --fixture\n",
                "t.py",
            )
            == {"--fixture"},
            "only argparse's required-arguments list is read, not its usage block",
        )
        # ...and it must be ARGPARSE's line, not a line the test printed itself.
        check(
            missing_required_options(
                "the following arguments are required: --aware-bin\n", "t.py"
            )
            == set(),
            "an unprefixed required-arguments line is not argparse's verdict",
        )
        status, _ = run_one(root / "tests/test_other_required.py", None)
        check(
            status == "failed",
            "a test requiring a DIFFERENT argument fails, and is not mis-skipped "
            "as needing --aware-bin",
        )
        status, _ = run_one(root / "tests/test_two_required.py", None)
        check(
            status == "failed",
            "a test missing --aware-bin AND another argument fails: supplying a "
            "binary would not make it runnable, so it is not a clean skip",
        )
        status, _ = run_one(root / "tests/test_echoes.py", None)
        check(
            status == "failed",
            "a test that PRINTS argparse's sentence as data is not skipped on it",
        )
        status, detail = run_one(root / "tests/test_pytest_style.py", None)
        check(
            status == "failed" and "never calls them" in detail,
            "a pytest-style file that would exit 0 asserting nothing is refused",
        )
        status, detail = run_one(root / "tests/test_exit_zero_skip.py", None)
        check(
            status == "skipped",
            "exit 0 whose last line announces a SKIP lands in the skipped bucket",
        )
        # ...but only on the real `SKIP:` announcement, not any line starting
        # with those letters (Codex review, PR #447).
        status, detail = run_one(root / "tests/test_skiplike_pass.py", None)
        check(
            status == "passed",
            "a passing test ending in `SKIPPED CHECKS: 0` is not read as a skip",
        )
        # The entry-point check reads the AST, so a test that calls itself runs...
        status, detail = run_one(root / "tests/test_calls_its_test.py", None)
        check(
            status == "passed" and "arithmetic holds" in detail,
            "a script-style test that calls its own test function is run, not refused",
        )
        # ...and `__main__`/`sys.exit(` in prose is not an entry point.
        status, detail = run_one(root / "tests/test_fake_entry.py", None)
        check(
            status == "failed" and "never calls them" in detail,
            "`__main__` inside a docstring does not count as an entry point",
        )
        # Class-based pytest is the other common shape and must not slip past.
        status, detail = run_one(root / "tests/test_class_style.py", None)
        check(
            status == "failed" and "never calls them" in detail,
            "a class-based pytest file is refused, not run as a silent pass",
        )
        # A block is only an entry point if it CALLS something; an import is not.
        status, detail = run_one(root / "tests/test_try_import.py", None)
        check(
            status == "failed" and "never calls them" in detail,
            "a top-level `try: import ...` is not treated as an entry point",
        )
        # A bare `test_x()` on a coroutine function runs nothing.
        status, detail = run_one(root / "tests/test_unawaited_async.py", None)
        check(
            status == "failed" and "never calls them" in detail,
            "an unawaited async test call is refused, not recorded as a pass",
        )
        # ...but a driven one is real execution and must be allowed to run.
        status, detail = run_one(root / "tests/test_driven_async.py", None)
        check(
            status == "failed" and "never calls them" not in detail,
            "an `asyncio.run(...)` async test is executed, not refused",
        )
        # An unrelated module-level call is not an entry point on its own.
        status, detail = run_one(root / "tests/test_unrelated_call.py", None)
        check(
            status == "failed" and "never calls them" in detail,
            "a call unrelated to any declared test is not an entry point",
        )
        # A coroutine wrapped in `print(...)` is built, not driven.
        status, detail = run_one(root / "tests/test_async_wrapped.py", None)
        check(
            status == "failed" and "never calls them" in detail,
            "a coroutine wrapped in an unrelated call is refused, not run",
        )
        # A `main()` dispatcher behind the `__main__` guard is honored, and runs.
        status, detail = run_one(root / "tests/test_main_dispatcher.py", None)
        check(
            status == "failed" and "never calls them" not in detail,
            "a `__main__`-guarded `main()` dispatcher is honored, not refused",
        )
        # A test declared inside a conditional block is still declared.
        status, detail = run_one(root / "tests/test_conditional_decl.py", None)
        check(
            status == "failed" and "never calls them" in detail,
            "a test declared inside an `if` block is seen, not run as a silent pass",
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

    # The execution floor. A tree whose every test skips has verified exactly as
    # much as an empty tree, and used to return 0 for it (review, PR #444).
    with tempfile.TemporaryDirectory(prefix="aware-pytest-selftest-allskip-") as tmp:
        root = Path(tmp)
        _write(root, "tests/test_skip_one.py", _SKIPPING)
        _write(root, "tests/test_skip_two.py", _EXIT_ZERO_SKIP)
        check(
            run_all(root, None) == 1,
            "a tree where every test skips fails: nothing was verified",
        )
        # ...and one real pass is enough to clear that floor.
        _write(root, "tests/test_real.py", _PASSING)
        check(
            run_all(root, None) == 0,
            "one genuinely passing test clears the execution floor",
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
