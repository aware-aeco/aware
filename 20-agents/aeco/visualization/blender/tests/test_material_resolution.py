"""Regression test: material resolution through every IfcMaterialSelect wrapper.

Run headless:
    blender -b -P 20-agents/aeco/visualization/blender/tests/test_material_resolution.py

`_material_of` originally probed a fixed attribute one level deep. That resolved
a plain IfcMaterial -- what `ifc.write` emits -- but returned "" for the
layer-set and profile-set wrappers Revit and Tekla actually emit. The failure was
silent: geometry still imported, only `ifc_material` went blank, and the look
mapper quietly fell back to the class family. These cases pin the recursive walk
so that regression cannot come back unnoticed.

Each case builds a synthetic IFC4 product in memory, round-trips it through the
SPF serializer so the inverse `HasAssociations` is resolved the same way a real
file resolves it, then runs the real `_material_of` against the reloaded product.
"""

import os
import shutil
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.normpath(os.path.join(HERE, os.pardir, "scripts")))

import _ifc_import  # noqa: E402


def _build_model(ifcopenshell):
    """Assemble one IFC4 model carrying every association shape under test.

    Returns (model, cases) where each case is (label, guid, expected_name).
    """
    import ifcopenshell.guid

    model = ifcopenshell.file(schema="IFC4")
    cases = []

    def product(ifc_class: str):
        guid = ifcopenshell.guid.new()
        return model.create_entity(ifc_class, GlobalId=guid), guid

    def associate(entity, relating_material):
        model.create_entity(
            "IfcRelAssociatesMaterial",
            GlobalId=ifcopenshell.guid.new(),
            RelatedObjects=[entity],
            RelatingMaterial=relating_material,
        )

    def material(name: str):
        return model.create_entity("IfcMaterial", Name=name)

    def profile_def():
        return model.create_entity(
            "IfcRectangleProfileDef", ProfileType="AREA", XDim=0.1, YDim=0.2
        )

    def case(label, ifc_class, relating_material, expected):
        entity, guid = product(ifc_class)
        associate(entity, relating_material)
        cases.append((label, guid, expected))

    # 1. Plain IfcMaterial -- what `ifc.write` emits; the pre-existing fast path.
    case("plain IfcMaterial", "IfcBeam", material("A992"), "A992")

    # 2. IfcMaterialLayerSetUsage -- Revit / ArchiCAD walls and slabs.
    layer_set = model.create_entity(
        "IfcMaterialLayerSet",
        MaterialLayers=[
            model.create_entity(
                "IfcMaterialLayer",
                Material=material("Concrete C30/37"),
                LayerThickness=0.2,
            )
        ],
        LayerSetName="Wall buildup",
    )
    case(
        "IfcMaterialLayerSetUsage",
        "IfcWall",
        model.create_entity(
            "IfcMaterialLayerSetUsage",
            ForLayerSet=layer_set,
            LayerSetDirection="AXIS2",
            DirectionSense="POSITIVE",
            OffsetFromReferenceLine=0.0,
        ),
        "Concrete C30/37",
    )

    # 3. IfcMaterialProfileSetUsage -- how Revit and Tekla carry material for
    #    structural framing. The shape this agent meets most often in the wild.
    profile_set = model.create_entity(
        "IfcMaterialProfileSet",
        Name="W16X26 buildup",
        MaterialProfiles=[
            model.create_entity(
                "IfcMaterialProfile", Material=material("A992"), Profile=profile_def()
            )
        ],
    )
    case(
        "IfcMaterialProfileSetUsage",
        "IfcBeam",
        model.create_entity("IfcMaterialProfileSetUsage", ForProfileSet=profile_set),
        "A992",
    )

    # 4. IfcMaterialLayerSet associated directly, with no usage wrapper.
    case(
        "IfcMaterialLayerSet (direct)",
        "IfcSlab",
        model.create_entity(
            "IfcMaterialLayerSet",
            MaterialLayers=[
                model.create_entity(
                    "IfcMaterialLayer",
                    Material=material("Concrete C25/30"),
                    LayerThickness=0.15,
                )
            ],
        ),
        "Concrete C25/30",
    )

    # 5. IfcMaterialProfileSet associated directly.
    case(
        "IfcMaterialProfileSet (direct)",
        "IfcColumn",
        model.create_entity(
            "IfcMaterialProfileSet",
            MaterialProfiles=[
                model.create_entity(
                    "IfcMaterialProfile",
                    Material=material("S355"),
                    Profile=profile_def(),
                )
            ],
        ),
        "S355",
    )

    # 6. IfcMaterialList -- legacy IFC2X3-era producers.
    case(
        "IfcMaterialList",
        "IfcMember",
        model.create_entity("IfcMaterialList", Materials=[material("A36")]),
        "A36",
    )

    # 7. IfcMaterialConstituentSet -- IFC4 curtain walls and composite plates.
    case(
        "IfcMaterialConstituentSet",
        "IfcPlate",
        model.create_entity(
            "IfcMaterialConstituentSet",
            MaterialConstituents=[
                model.create_entity(
                    "IfcMaterialConstituent", Name="Pane", Material=material("Glass")
                )
            ],
        ),
        "Glass",
    )

    # 8. The trap: IfcMaterialLayer.Name is the LAYER name. Lifting `.Name` off
    #    the wrapper returns "Core" -- plausible-looking and wrong.
    case(
        "layer Name is not the material",
        "IfcWall",
        model.create_entity(
            "IfcMaterialLayerSet",
            MaterialLayers=[
                model.create_entity(
                    "IfcMaterialLayer",
                    Name="Core",
                    Material=material("Concrete C30/37"),
                    LayerThickness=0.2,
                )
            ],
        ),
        "Concrete C30/37",
    )

    # 9. Same trap on the profile side: the profile is named for the section.
    case(
        "profile Name is not the material",
        "IfcBeam",
        model.create_entity(
            "IfcMaterialProfileSet",
            MaterialProfiles=[
                model.create_entity(
                    "IfcMaterialProfile",
                    Name="W16X26",
                    Material=material("A992"),
                    Profile=profile_def(),
                )
            ],
        ),
        "A992",
    )

    # 10. No association at all resolves to "", not a crash.
    _, orphan_guid = product("IfcBeam")
    cases.append(("no material association", orphan_guid, ""))

    return model, cases


def main() -> int:
    ifcopenshell = _ifc_import._import_ifcopenshell()

    workdir = tempfile.mkdtemp(prefix="aware-blender-mattest-")
    try:
        model, cases = _build_model(ifcopenshell)

        # Round-trip through SPF: inverse attributes such as HasAssociations must
        # resolve off a parsed file exactly as they do in production.
        path = os.path.join(workdir, "materials.ifc")
        model.write(path)
        reloaded = ifcopenshell.open(path)

        failures = 0
        for label, guid, expected in cases:
            product = reloaded.by_guid(guid)
            actual = _ifc_import._material_of(product)
            ok = actual == expected
            failures += 0 if ok else 1
            print(
                f"{'PASS' if ok else 'FAIL'}  {label:<34} "
                f"expected={expected!r:<20} actual={actual!r}"
            )

        total = len(cases)
        print(f"\n{total - failures}/{total} passed")
        if failures:
            print(f"FAILED: {failures} material shape(s) did not resolve")
            return 1
        print("OK: every association shape resolved to its leaf IfcMaterial")
        return 0
    finally:
        shutil.rmtree(workdir, ignore_errors=True)


sys.exit(main())
