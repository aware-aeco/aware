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

import _ifc_import  # noqa: E402

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


def main() -> int:
    ifcopenshell = _ifc_import._import_ifcopenshell()
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
