"""Running a command must not write into the agent's package (#692 review).

On a run, the scripts execute from the agent's immutable stored package
(`agent-store-v2/...`, #626), whose files are hashed at snapshot time. Python
caches bytecode in a `__pycache__` next to every helper it imports, so one run
used to change the package's hash and the NEXT `aware app compile` refused the
store with E_AGENT_STORE_INVALID.

Two checks:

1. Static, no Blender needed: every command entry script (a `scripts/*.py` not
   starting with `_`) sets `sys.dont_write_bytecode = True` before importing
   anything but `sys` -- after the first helper import it is too late.
2. Real Blender: copy `scripts/` to a scratch "package", launch every entry
   script from it exactly as the transport does (`blender -b -P <script> --
   @inputs`), and require that no `__pycache__` (or any other new file)
   appears in it. Inputs are `{}`, so each command imports all its helpers
   and then fails cleanly on validation -- the import is what writes bytecode.

Run: python test_package_stays_immutable.py [--blender <path>]
Exits 0 on pass, 1 on failure, 77 (SKIP) for the Blender half when no Blender
is found (the static half still runs and can still fail).
"""

import argparse
import ast
import json
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

sys.dont_write_bytecode = True  # this harness imports from scripts/ too

HERE = Path(__file__).resolve().parent
SCRIPTS = HERE.parent / "scripts"
sys.path.insert(0, str(HERE))

from run_smoke import resolve_blender  # noqa: E402

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


def entry_scripts() -> list[Path]:
    return sorted(p for p in SCRIPTS.glob("*.py") if not p.name.startswith("_"))


def sets_flag_before_other_imports(source: str) -> bool:
    module = ast.parse(source)
    for statement in module.body:
        if isinstance(statement, ast.Expr) and isinstance(statement.value, ast.Constant):
            continue  # the docstring
        if isinstance(statement, ast.Import) and [a.name for a in statement.names] == ["sys"]:
            continue
        if (
            isinstance(statement, ast.Assign)
            and len(statement.targets) == 1
            and ast.unparse(statement.targets[0]) == "sys.dont_write_bytecode"
            and isinstance(statement.value, ast.Constant)
            and statement.value.value is True
        ):
            return True
        return False  # anything else first -- an import, a path insert -- is too late
    return False


def snapshot(root: Path) -> set[str]:
    return {str(p.relative_to(root)) for p in root.rglob("*")}


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--blender", help="path to the Blender executable")
    opts = parser.parse_args()

    entries = entry_scripts()
    print("static: every entry script disables bytecode first")
    check("there are entry scripts to check", len(entries) >= 6, [p.name for p in entries])
    for path in entries:
        check(
            f"{path.name} sets sys.dont_write_bytecode before any other import",
            sets_flag_before_other_imports(path.read_text(encoding="utf-8")),
        )

    blender = resolve_blender(opts.blender)
    if blender is None:
        if FAILURES:
            print("\nPACKAGE IMMUTABLE FAIL")
            return 1
        print("SKIP: no Blender available (pass --blender <path> to run this harness)")
        return SKIP_EXIT

    work = Path(tempfile.mkdtemp(prefix="aware-package-"))
    try:
        package = work / "blender" / "scripts"
        shutil.copytree(SCRIPTS, package, ignore=shutil.ignore_patterns("__pycache__"))
        before = snapshot(package.parent)
        inputs = work / "inputs.json"
        inputs.write_text(json.dumps({}), encoding="utf-8")
        print("real Blender: launching every command from a scratch package")
        for path in entries:
            proc = subprocess.run(
                [blender, "-b", "-P", str(package / path.name), "--", f"@{inputs}"],
                capture_output=True,
                text=True,
                encoding="utf-8",
                errors="replace",
                check=False,
            )
            check(
                f"{path.name} ran and framed its result",
                "<<<AWARE_RESULT>>>" in proc.stdout,
                proc.stdout[-800:] + proc.stderr[-800:],
            )
        after = snapshot(package.parent)
        added = sorted(after - before)
        check("no file was written into the package", added == [], added)
    finally:
        shutil.rmtree(work, ignore_errors=True)

    print("\nPACKAGE IMMUTABLE FAIL" if FAILURES else "\nPACKAGE IMMUTABLE PASS")
    return 1 if FAILURES else 0


if __name__ == "__main__":
    sys.exit(main())
