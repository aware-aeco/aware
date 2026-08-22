"""Regression test: a silently-dropped element must still be accounted for.

Run headless:
    blender -b -P 20-agents/aeco/visualization/blender/tests/test_import_reconciliation.py

`ifcopenshell.geom.iterator` excludes a product whose geometry is unfeasible --
a zero-depth extrusion, a zero-area profile -- at the C++ level. It is never
yielded and nothing is raised, so `import_ifc`'s try/except cannot see it. Before
the reconciliation pass such an element vanished from the receipt entirely:
`imported` dropped by one while `skipped` stayed empty, which is the one outcome
a skip-and-count contract must never produce.

The invariant under test:

    imported + skipped-count == len(_geometry_bearing_products(ifc_file))

The clean case matters as much as the degenerate one. Reconciling against every
IfcProduct rather than only those claiming a shape would report IfcSite,
IfcBuilding and IfcBuildingStorey as skips on every import, so this builds those
spatial products explicitly and asserts they generate no false positives.
"""

import os
import shutil
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.normpath(os.path.join(HERE, os.pardir, "scripts")))

import _result  # noqa: E402

# The conventional "test could not run here" status, as `run_smoke.py` documents
# and uses -- so a human glancing at `$?`, and `scripts/run-agent-python-tests.py`
# reading the same code, do not mistake "nothing to check here" for a red gate.
SKIP_EXIT = 77

# `_ifc_import` imports `bpy` at module scope, so this line is where the harness
# meets its host. Launched the documented way (`blender -b -P ...`) that import
# succeeds; launched on a plain interpreter -- which is what
# `scripts/run-agent-python-tests.py` does to every `test_*.py` in the repo --
# it raised an uncaught `ModuleNotFoundError` traceback and exited 1. That is
# indistinguishable, to a contributor without Blender, from the test being
# broken, and it is what kept this file out of any automated runner. Its five
# siblings already announce a missing host and exit cleanly; this now does too.
try:
    import _ifc_import  # noqa: E402
except ImportError as exc:
    print(f"SKIP: needs Blender's Python ({exc}); run: blender -b -P {__file__}")
    sys.exit(SKIP_EXIT)

VALID_BEAMS = 3


def _build(ifcopenshell, path: str, degenerate: bool) -> tuple[str, int]:
    """Write a small IFC4 model; return (degenerate GUID or "", geometry count)."""
    import ifcopenshell.guid

    model = ifcopenshell.file(schema="IFC4")
    guid = ifcopenshell.guid.new

    def point(*coords):
        return model.create_entity("IfcCartesianPoint", Coordinates=list(coords))

    def placement(z=0.0):
        return model.create_entity("IfcAxis2Placement3D", Location=point(0.0, 0.0, z))

    context = model.create_entity(
        "IfcGeometricRepresentationContext",
        ContextType="Model",
        CoordinateSpaceDimension=3,
        Precision=1e-5,
        WorldCoordinateSystem=placement(),
    )
    model.create_entity(
        "IfcProject",
        GlobalId=guid(),
        Name="reconciliation",
        RepresentationContexts=[context],
        UnitsInContext=model.create_entity(
            "IfcUnitAssignment",
            Units=[
                model.create_entity("IfcSIUnit", UnitType="LENGTHUNIT", Name="METRE")
            ],
        ),
    )

    # Spatial products carry no representation and must never be counted as
    # skips -- this is the false-positive guard.
    for spatial in ("IfcSite", "IfcBuilding", "IfcBuildingStorey"):
        model.create_entity(spatial, GlobalId=guid(), Name=spatial)

    def beam(name: str, depth: float, offset: float) -> str:
        beam_guid = guid()
        solid = model.create_entity(
            "IfcExtrudedAreaSolid",
            SweptArea=model.create_entity(
                "IfcRectangleProfileDef", ProfileType="AREA", XDim=0.2, YDim=0.4
            ),
            Position=placement(offset),
            ExtrudedDirection=model.create_entity(
                "IfcDirection", DirectionRatios=[0.0, 0.0, 1.0]
            ),
            Depth=depth,
        )
        model.create_entity(
            "IfcBeam",
            GlobalId=beam_guid,
            Name=name,
            ObjectPlacement=model.create_entity(
                "IfcLocalPlacement", RelativePlacement=placement()
            ),
            Representation=model.create_entity(
                "IfcProductDefinitionShape",
                Representations=[
                    model.create_entity(
                        "IfcShapeRepresentation",
                        ContextOfItems=context,
                        RepresentationIdentifier="Body",
                        RepresentationType="SweptSolid",
                        Items=[solid],
                    )
                ],
            ),
        )
        return beam_guid

    for index in range(VALID_BEAMS):
        beam(f"B{index + 1}", 3.0, float(index))

    # A zero-depth extrusion is unfeasible: the iterator drops it without a word.
    bad_guid = beam("DEGENERATE", 0.0, 9.0) if degenerate else ""

    model.write(path)
    return bad_guid, VALID_BEAMS + (1 if degenerate else 0)


def _build_non_visual(ifcopenshell, path: str) -> None:
    """One real beam plus an opening and a space, all with valid geometry.

    The opening and the space tessellate perfectly well -- that is the whole
    problem. They must be excluded by class, not by any geometric test.
    """
    import ifcopenshell.guid

    model = ifcopenshell.file(schema="IFC4")
    guid = ifcopenshell.guid.new

    def point(*coords):
        return model.create_entity("IfcCartesianPoint", Coordinates=list(coords))

    def placement(z=0.0):
        return model.create_entity("IfcAxis2Placement3D", Location=point(0.0, 0.0, z))

    context = model.create_entity(
        "IfcGeometricRepresentationContext",
        ContextType="Model",
        CoordinateSpaceDimension=3,
        Precision=1e-5,
        WorldCoordinateSystem=placement(),
    )
    model.create_entity(
        "IfcProject",
        GlobalId=guid(),
        Name="non-visual",
        RepresentationContexts=[context],
        UnitsInContext=model.create_entity(
            "IfcUnitAssignment",
            Units=[
                model.create_entity("IfcSIUnit", UnitType="LENGTHUNIT", Name="METRE")
            ],
        ),
    )

    def build(ifc_class: str, name: str, offset: float):
        solid = model.create_entity(
            "IfcExtrudedAreaSolid",
            SweptArea=model.create_entity(
                "IfcRectangleProfileDef", ProfileType="AREA", XDim=0.5, YDim=0.5
            ),
            Position=placement(offset),
            ExtrudedDirection=model.create_entity(
                "IfcDirection", DirectionRatios=[0.0, 0.0, 1.0]
            ),
            Depth=2.0,
        )
        return model.create_entity(
            ifc_class,
            GlobalId=guid(),
            Name=name,
            ObjectPlacement=model.create_entity(
                "IfcLocalPlacement", RelativePlacement=placement()
            ),
            Representation=model.create_entity(
                "IfcProductDefinitionShape",
                Representations=[
                    model.create_entity(
                        "IfcShapeRepresentation",
                        ContextOfItems=context,
                        RepresentationIdentifier="Body",
                        RepresentationType="SweptSolid",
                        Items=[solid],
                    )
                ],
            ),
        )

    wall = build("IfcWall", "W1", 0.0)
    opening = build("IfcOpeningElement", "O1", 3.0)
    build("IfcSpace", "Room 1", 6.0)
    model.create_entity(
        "IfcRelVoidsElement",
        GlobalId=guid(),
        RelatingBuildingElement=wall,
        RelatedOpeningElement=opening,
    )
    model.write(path)


def main() -> int:
    # The second host dependency, and the same reasoning as the guarded import
    # above: a Blender without ifcopenshell installed is a machine that cannot
    # run this test, not a machine on which it fails.
    try:
        ifcopenshell = _ifc_import._import_ifcopenshell()
    except _result.AwareBlenderError as exc:
        if exc.code != _result.ERR_IFCOPENSHELL_MISSING:
            raise
        print(f"SKIP: {exc.message}")
        return SKIP_EXIT

    workdir = tempfile.mkdtemp(prefix="aware-blender-recon-")
    checks: list[tuple[bool, str]] = []

    def check(ok: bool, label: str, detail: str = "") -> None:
        checks.append((ok, f"{label}{'  ' + detail if detail else ''}"))

    try:
        for degenerate in (False, True):
            tag = "degenerate" if degenerate else "clean"
            path = os.path.join(workdir, f"{tag}.ifc")
            bad_guid, claiming = _build(ifcopenshell, path, degenerate)

            receipt = _ifc_import.import_ifc(path)
            imported = receipt["imported"]
            skipped_count = receipt["skipped-count"]
            guids = [entry["guid"] for entry in receipt["skipped"]]

            reopened = ifcopenshell.open(path)
            bearing = len(_ifc_import._geometry_bearing_products(reopened))

            check(
                bearing == claiming,
                f"[{tag}] geometry-bearing predicate ignores spatial products",
                f"expected={claiming} actual={bearing}",
            )
            check(
                imported == VALID_BEAMS,
                f"[{tag}] imported count",
                f"expected={VALID_BEAMS} actual={imported}",
            )
            check(
                skipped_count == (1 if degenerate else 0),
                f"[{tag}] skipped-count",
                f"expected={1 if degenerate else 0} actual={skipped_count}",
            )
            check(
                imported + skipped_count == bearing,
                f"[{tag}] INVARIANT imported + skipped-count == geometry-bearing",
                f"{imported} + {skipped_count} == {bearing}",
            )
            if degenerate:
                check(
                    guids == [bad_guid],
                    "[degenerate] the dropped element's GUID is reported",
                    f"expected=[{bad_guid}] actual={guids}",
                )
                reason = receipt["skipped"][0]["reason"] if receipt["skipped"] else ""
                check(
                    "not yielded" in reason,
                    "[degenerate] reason is readable",
                    f"reason={reason!r}",
                )
            else:
                check(
                    not guids,
                    "[clean] NO FALSE SKIPS",
                    f"actual={guids}",
                )

        # Non-visual products: real geometry, deliberately never rendered.
        path = os.path.join(workdir, "non-visual.ifc")
        _build_non_visual(ifcopenshell, path)
        receipt = _ifc_import.import_ifc(path)
        by_class = receipt["by-class"]
        excluded = receipt["excluded"]
        reopened = ifcopenshell.open(path)
        bearing = len(_ifc_import._geometry_bearing_products(reopened))

        check(
            receipt["imported"] == 1,
            "[non-visual] only the real element imports",
            f"expected=1 actual={receipt['imported']}",
        )
        check(
            by_class == {"IfcWall": 1},
            "[non-visual] by-class holds ONLY the wall",
            f"actual={by_class}",
        )
        check(
            receipt["skipped-count"] == 0,
            "[non-visual] excluded products are NOT reported as skipped",
            f"actual={receipt['skipped']}",
        )
        check(
            excluded == {"IfcOpeningElement": 1, "IfcSpace": 1},
            "[non-visual] excluded reports both, by class",
            f"actual={excluded}",
        )
        check(
            receipt["excluded-count"] == 2,
            "[non-visual] excluded-count",
            f"expected=2 actual={receipt['excluded-count']}",
        )
        check(
            bearing == 1,
            "[non-visual] geometry-bearing predicate excludes them too",
            f"expected=1 actual={bearing}",
        )
        check(
            receipt["imported"] + receipt["skipped-count"] == bearing,
            "[non-visual] INVARIANT still holds",
            f"{receipt['imported']} + {receipt['skipped-count']} == {bearing}",
        )

        failures = sum(1 for ok, _ in checks if not ok)
        for ok, label in checks:
            print(f"{'PASS' if ok else 'FAIL'}  {label}")
        print(f"\n{len(checks) - failures}/{len(checks)} passed")
        if failures:
            print(f"FAILED: {failures} check(s)")
            return 1
        print("OK: every geometry-bearing product is accounted for")
        return 0
    finally:
        shutil.rmtree(workdir, ignore_errors=True)


sys.exit(main())
