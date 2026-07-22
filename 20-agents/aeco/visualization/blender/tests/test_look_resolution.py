"""Regression test: EN 338 timber grades vs EN 206 concrete grades in `_looks.py`.

`GRADE_FAMILIES["concrete"]` and `GRADE_FAMILIES["timber"]` both key off bare
"C<number>" prefixes. EN 338 defines twelve structural timber strength classes
(C14-C50); five of them -- C20, C30, C35, C40, C50 -- are also, character for
character, the concrete grades this table already carried. A material graded
plain "C30" cannot be resolved from the grade string alone: `family_for` was
silently picking concrete for every one of those five, including on framing
elements, because grade matching ran before class matching and dict-iteration
order (concrete before timber) settled every tie by accident. A glulam or
solid-timber beam graded C30 rendered as concrete, with `by-reason` reporting
`grade` -- the most convincing possible wrong answer.

This sweeps every EN 338 class against a concrete-decisive IFC class
(`IfcSlab`) and a framing class (`IfcBeam`) and pins the disambiguation rule
`family_for`'s docstring now documents: the IFC class breaks a grade tie via
CLASS_FAMILIES; since no class there maps to "timber" (framing shapes map to
"steel", for the unrelated no-grade fallback case, and "steel" is never a
tied candidate in a concrete/timber collision), a class that cannot settle
the tie falls back to concrete -- so the five colliding classes read as
concrete on *both* IfcSlab and IfcBeam. Only the seven non-colliding classes
(and the unambiguous TIMBER/WOOD/GL24/GL28 signals) resolve to timber
regardless of class. This test pins that exact, documented shape so a future
change to either table has to touch this file on purpose.

Runs as plain Python -- no Blender needed. `_looks.py` only touches `bpy` at
attribute-access time inside functions, never at def/class time (the module
carries `from __future__ import annotations` precisely so a bare stub is
enough to import it), so this needs nothing beyond the standard library.
"""

import sys
import types
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE.parent / "scripts"))

sys.modules["bpy"] = types.ModuleType("bpy")  # bare stub -- no .types attribute
sys.modules["_ifc_import"] = types.ModuleType("_ifc_import")

import _looks  # noqa: E402

# All 12 EN 338 structural timber strength classes.
EN_338_CLASSES = (
    "C14", "C16", "C18", "C20", "C22", "C24", "C27", "C30", "C35", "C40", "C45", "C50",
)

# The 5 that collide, character for character, with a GRADE_FAMILIES["concrete"] entry.
COLLIDES_WITH_CONCRETE = {"C20", "C30", "C35", "C40", "C50"}

CONCRETE_CLASS = "IfcSlab"  # CLASS_FAMILIES-decisive: always concrete.
FRAMING_CLASS = "IfcBeam"  # CLASS_FAMILIES says "steel" -- not a candidate in
# a concrete/timber tie, so it cannot break one; falls back to the same
# concrete default as CONCRETE_CLASS for the 5 colliding grades.


def _expected(grade: str) -> str:
    return "concrete" if grade in COLLIDES_WITH_CONCRETE else "timber"


def main() -> int:
    cases = []

    for grade in EN_338_CLASSES:
        cases.append((grade, CONCRETE_CLASS, _expected(grade)))
        cases.append((grade, FRAMING_CLASS, _expected(grade)))
        cases.append((grade, "", _expected(grade)))  # no class at all

    # Unambiguous timber signals must win regardless of class or the collision.
    for material in ("TIMBER", "WOOD", "GL24", "GL28", "gl24", "timber panel"):
        cases.append((material, CONCRETE_CLASS, "timber"))
        cases.append((material, FRAMING_CLASS, "timber"))

    # Concrete-only grades (no EN 338 collision) must be untouched by the fix.
    for material in ("C25", "CONCRETE", "BETON"):
        cases.append((material, FRAMING_CLASS, "concrete"))

    failures = 0
    for material, ifc_class, expected in cases:
        family, reason = _looks.family_for(ifc_class, material)
        ok = family == expected
        failures += 0 if ok else 1
        print(
            f"{'PASS' if ok else 'FAIL'}  grade={material!r:14} class={ifc_class!r:10} "
            f"expected={expected!r:10} actual={family!r:10} reason={reason!r}"
        )

    total = len(cases)
    print(f"\n{total - failures}/{total} passed")
    if failures:
        print(f"FAILED: {failures} case(s) did not match the documented disambiguation rule")
        return 1
    print("OK: every EN 338 class resolves per the rule documented in _looks.py")
    return 0


sys.exit(main())
