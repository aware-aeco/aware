#!/usr/bin/env python3
"""End-to-end check of `kind:"extrusion"` through the real `aware` CLI and ifcopenshell (#3296 S3).

ONE entry point, a pre-PR gate run on demand (not CI: it needs `ifcopenshell`, which the workflows do
not install). It:

1. installs the WORKTREE's `ifc` agent into a throwaway AWARE_HOME and writes a scene with a wall (one
   item, an axis), a slab with two openings, a pad footing, a strip footing with an axis, a
   USERDEFINED banded wall and a generic shape, through `aware agent invoke ifc write`;
2. validates the written file against the IFC4 schema (`ifcopenshell.validate`, EXPRESS rules
   included where the installed ifcopenshell supports them);
3. reads it back: each element's class, PredefinedType and ObjectType, its quantity set, LoadBearing,
   material, axis representation and FloLess label set, and the VOLUME of its solids as computed by
   ifcopenshell's geometry kernel, compared with the scene's `quantities`;
4. runs two negative controls that must FAIL the same read-back: the scene without the wall's
   `meta.properties` (the label check), the slab with a `netVolumeM3` that ignores its openings (the volume
   check), and a wall typed FLOOR (the schema check).

Usage:  python cli/tests/ifc/extrusion_readback.py --aware <path to the built aware binary> [--keep <dir>]
Exit 0 only when every check passes and both controls fail as they must.
"""

import argparse
import copy
import json
import os
import shutil
import subprocess
import sys
import tempfile

import ifcopenshell
import ifcopenshell.geom
import ifcopenshell.util.element
import ifcopenshell.util.shape
import ifcopenshell.validate

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", "..", ".."))
AGENT = os.path.join(ROOT, "20-agents", "_core", "ifc")


def frame(origin):
    return {"origin": origin, "uDir": [1, 0, 0], "vDir": [0, 1, 0], "normal": [0, 0, 1]}


def scene():
    return {
        "meta": {"name": "Walls slabs footings E2E", "units": "mm", "up": "z",
                 "sourceId": "e2e", "sceneHash": "e2e"},
        "groups": [{"key": "wall", "label": "Walls", "color": "#94a3b8"},
                   {"key": "slab", "label": "Slabs", "color": "#cbd5e1"}],
        "elements": [
            {"id": "W-1", "kind": "extrusion", "class": "wall", "predefinedType": "SOLIDWALL",
             "group": "wall", "frame": frame([1000, 2000, 0]),
             "items": [{"outline": [[0, -100], [6000, -100], [6000, 100], [0, 100]],
                        "fromMm": 0, "depthMm": 3000}],
             "axis": {"u0": 0, "u1": 6000}, "loadBearing": True,
             "material": {"name": "Concrete block"},
             "quantities": {"lengthMm": 6000, "widthMm": 200, "heightMm": 3000,
                            "grossSideAreaM2": 18, "netSideAreaM2": 18,
                            "grossVolumeM3": 3.6, "netVolumeM3": 3.6},
             "meta": {"description": "Wall W-1 on grid A, 200 CMU",
                      "properties": {"name": "FloLess",
                                     "properties": {"DefinitionId": "def-cmu-200",
                                                    "AddOn": "walls-slabs",
                                                    "Hold": "none"}}}},
            {"id": "S-1", "kind": "extrusion", "class": "slab", "predefinedType": "FLOOR",
             "group": "slab", "frame": frame([0, 0, 2850]),
             "items": [{"outline": [[0, 0], [8000, 0], [8000, 6000], [0, 6000]],
                        "holes": [[[1000, 1000], [1000, 2000], [2000, 2000], [2000, 1000]],
                                  [[5000, 3000], [5000, 4000], [6000, 4000], [6000, 3000]]],
                        "fromMm": 0, "depthMm": 150}],
             "loadBearing": True, "material": {"name": "Concrete C30/37"},
             "quantities": {"widthMm": 150, "perimeterMm": 28000, "grossAreaM2": 48,
                            "netAreaM2": 46, "grossVolumeM3": 7.2, "netVolumeM3": 6.9,
                            "grossWeightKg": 17280, "netWeightKg": 16560}},
            {"id": "F-PAD", "kind": "extrusion", "class": "footing", "predefinedType": "PAD_FOOTING",
             "frame": frame([-1000, -1000, -600]),
             "items": [{"outline": [[0, 0], [2000, 0], [2000, 2000], [0, 2000]],
                        "fromMm": 0, "depthMm": 600}],
             "loadBearing": True, "material": {"name": "Concrete C30/37"},
             "quantities": {"lengthMm": 2000, "widthMm": 2000, "heightMm": 600,
                            "crossSectionAreaM2": 4, "grossVolumeM3": 2.4, "netVolumeM3": 2.4,
                            "grossWeightKg": 5760, "netWeightKg": 5760}},
            {"id": "F-STRIP", "kind": "extrusion", "class": "footing",
             "predefinedType": "STRIP_FOOTING", "frame": frame([1000, 2000, -500]),
             "items": [{"outline": [[0, -300], [6000, -300], [6000, 300], [0, 300]],
                        "fromMm": 0, "depthMm": 500}],
             "axis": {"u0": 0, "u1": 6000}, "loadBearing": True,
             "quantities": {"lengthMm": 6000, "widthMm": 600, "heightMm": 500,
                            "grossVolumeM3": 1.8}},
            {"id": "W-UD", "kind": "extrusion", "class": "wall", "predefinedType": "USERDEFINED",
             "objectType": "Acme CMU 200", "frame": frame([0, 8000, 0]),
             "items": [{"outline": [[0, -100], [3000, -100], [3000, 100], [0, 100]],
                        "fromMm": 0, "depthMm": 1000},
                       {"outline": [[0, -100], [2500, -100], [2500, 100], [0, 100]],
                        "fromMm": 1000, "depthMm": 2000}],
             "quantities": {"grossVolumeM3": 1.6, "grossWeightKg": 3200}},
            {"id": "X-1", "kind": "extrusion", "frame": frame([0, 0, 0]),
             "items": [{"outline": [[0, 0], [100, 0], [0, 100]], "fromMm": 0, "depthMm": 10}]},
        ],
    }


EXPECTED = {
    "W-1": ("IfcWall", "SOLIDWALL", None, "Qto_WallBaseQuantities", "Pset_WallCommon", True),
    "S-1": ("IfcSlab", "FLOOR", None, "Qto_SlabBaseQuantities", "Pset_SlabCommon", False),
    "F-PAD": ("IfcFooting", "PAD_FOOTING", None, "Qto_FootingBaseQuantities", "Pset_FootingCommon", False),
    "F-STRIP": ("IfcFooting", "STRIP_FOOTING", None, "Qto_FootingBaseQuantities", "Pset_FootingCommon", True),
    "W-UD": ("IfcWall", "USERDEFINED", "Acme CMU 200", "Qto_WallBaseQuantities", None, False),
    "X-1": ("IfcBuildingElementProxy", None, None, None, None, False),
}
QTO_NAMES = {"lengthMm": "Length", "widthMm": "Width", "heightMm": "Height",
             "grossSideAreaM2": "GrossSideArea", "netSideAreaM2": "NetSideArea",
             "perimeterMm": "Perimeter", "grossAreaM2": "GrossArea", "netAreaM2": "NetArea",
             "crossSectionAreaM2": "CrossSectionArea", "grossVolumeM3": "GrossVolume",
             "netVolumeM3": "NetVolume", "grossWeightKg": "GrossWeight", "netWeightKg": "NetWeight"}


def ring_area(ring):
    return abs(sum(x0 * y1 - x1 * y0 for (x0, y0), (x1, y1) in zip(ring, ring[1:] + ring[:1]))) / 2.0


def scene_volume_m3(element):
    """Net ring area x depth summed over the items, straight from the scene (the items are disjoint)."""
    return sum((ring_area(i["outline"]) - sum(ring_area(h) for h in i.get("holes", []))) * i["depthMm"]
               for i in element["items"]) / 1.0e9


def write_ifc(aware, the_scene, work, tag):
    home = os.path.join(work, f"home-{tag}")
    os.makedirs(home, exist_ok=True)
    env = dict(os.environ, AWARE_HOME=home)
    subprocess.run([aware, "agent", "install", AGENT], env=env, check=True, capture_output=True)
    out_path = os.path.join(work, f"{tag}.ifc")
    inputs = os.path.join(work, f"{tag}.inputs.json")
    with open(inputs, "w", encoding="utf-8") as handle:
        json.dump({"scene": the_scene, "output-path": out_path}, handle)
    run = subprocess.run([aware, "agent", "invoke", "ifc", "write", "--inputs", f"@{inputs}", "--json"],
                         env=env, capture_output=True, text=True)
    if run.returncode != 0:
        raise RuntimeError(f"aware ifc write failed: {run.stdout}\n{run.stderr}")
    receipt = json.loads(run.stdout)
    return out_path, receipt


def schema_errors(path):
    # Validate the FILE, not an opened model: only a file read reports a literal the schema refuses (an
    # enumeration value, a malformed record); an opened model has already dropped it.
    try:
        logger = ifcopenshell.validate.json_logger()
        ifcopenshell.validate.validate(path, logger, express_rules=True)
        rules = "schema + EXPRESS rules"
    except (TypeError, ImportError):
        # The EXPRESS WHERE-rule executor needs pytest; without it, the schema check still runs
        # (entity types, attribute types and counts, enumerations, inverse cardinalities).
        logger = ifcopenshell.validate.json_logger()
        ifcopenshell.validate.validate(path, logger)
        rules = "schema only (EXPRESS rules need pytest)"
    return rules, [s for s in logger.statements if s.get("level", "").lower() == "error"]


def read_back(path, the_scene):
    """Every finding as a string; an empty list is a pass."""
    model = ifcopenshell.open(path)
    findings = []
    settings = ifcopenshell.geom.settings()
    by_name = {}
    for product in model.by_type("IfcProduct"):
        if product.Name:
            by_name[product.Name] = product
    for element in the_scene["elements"]:
        eid = element["id"]
        entity, predefined, object_type, qto, common, has_axis = EXPECTED[eid]
        product = by_name.get(eid)
        if product is None:
            findings.append(f"{eid}: not in the file")
            continue
        if not product.is_a(entity) or product.is_a() != entity:
            findings.append(f"{eid}: is {product.is_a()}, expected {entity}")
        if product.PredefinedType != predefined:
            findings.append(f"{eid}: PredefinedType {product.PredefinedType}, expected {predefined}")
        if product.ObjectType != object_type:
            findings.append(f"{eid}: ObjectType {product.ObjectType}, expected {object_type}")
        # Volume from the geometry kernel (in metres), against the scene's own count.
        shape = ifcopenshell.geom.create_shape(settings, product)
        volume = ifcopenshell.util.shape.get_volume(shape.geometry)
        if abs(volume - scene_volume_m3(element)) > 1e-9:
            findings.append(f"{eid}: the solids enclose {volume:.9f} m3, the scene's rings give {scene_volume_m3(element):.9f}")
        quantities = element.get("quantities", {})
        # Net volume is what the solids enclose; gross is the same solids with their openings filled,
        # read from the file's own profiles (outer polyline area x depth).
        gross = sum(ring_area([tuple(p.Coordinates) for p in item.SweptArea.OuterCurve.Points][:-1]) * item.Depth
                    for rep in product.Representation.Representations if rep.RepresentationIdentifier == "Body"
                    for item in rep.Items) / 1.0e9
        has_holes = any(i.get("holes") for i in element["items"])
        net_count = quantities.get("netVolumeM3", None if has_holes else quantities.get("grossVolumeM3"))
        if net_count is not None and abs(volume - net_count) > 1e-6:
            findings.append(f"{eid}: the solids enclose {volume:.9f} m3, the scene counts {net_count} net")
        if "grossVolumeM3" in quantities and abs(gross - quantities["grossVolumeM3"]) > 1e-6:
            findings.append(f"{eid}: the solids with openings filled give {gross:.9f} m3, the scene counts "
                            f"{quantities['grossVolumeM3']} gross")
        psets = ifcopenshell.util.element.get_psets(product)
        if qto:
            written = psets.get(qto, {})
            for key, value in quantities.items():
                name = QTO_NAMES[key]
                if name not in written or abs(written[name] - value) > 1e-9:
                    findings.append(f"{eid}: {qto}.{name} is {written.get(name)}, scene says {value}")
            extra = set(written) - {QTO_NAMES[k] for k in quantities} - {"id"}
            if extra:
                findings.append(f"{eid}: {qto} carries keys the scene did not send: {sorted(extra)}")
        if common:
            if psets.get(common, {}).get("LoadBearing") is not element.get("loadBearing"):
                findings.append(f"{eid}: {common}.LoadBearing is {psets.get(common, {}).get('LoadBearing')}")
        material = ifcopenshell.util.element.get_material(product)
        want = element.get("material", {}).get("name")
        if (material.Name if material else None) != want:
            findings.append(f"{eid}: material {material.Name if material else None}, expected {want}")
        reps = {r.RepresentationIdentifier: r for r in product.Representation.Representations}
        if has_axis != ("Axis" in reps):
            findings.append(f"{eid}: axis representation present={('Axis' in reps)}, expected {has_axis}")
        if "Axis" in reps:
            points = [tuple(p.Coordinates) for p in reps["Axis"].Items[0].Points]
            want_axis = [(float(element["axis"]["u0"]), 0.0), (float(element["axis"]["u1"]), 0.0)]
            if points != want_axis:
                findings.append(f"{eid}: axis {points}, expected {want_axis}")
        body = reps.get("Body")
        if body is None or len(body.Items) != len(element["items"]):
            findings.append(f"{eid}: Body items {len(body.Items) if body else 0}, expected {len(element['items'])}")
        holes = sum(len(i.get("holes", [])) for i in element["items"])
        voids = sum(len(i.SweptArea.InnerCurves) for i in body.Items
                    if i.SweptArea.is_a("IfcArbitraryProfileDefWithVoids")) if body else 0
        if holes != voids:
            findings.append(f"{eid}: {voids} voids written, the scene has {holes} openings")
        labels = element.get("meta", {}).get("properties")
        if eid == "W-1":
            floless = psets.get("FloLess")
            if floless is None:
                findings.append("W-1: label read-back: no FloLess property set")
            else:
                for key, value in {"DefinitionId": "def-cmu-200", "AddOn": "walls-slabs", "Hold": "none"}.items():
                    if floless.get(key) != value:
                        findings.append(f"W-1: label read-back: FloLess.{key} is {floless.get(key)}")
            if product.Description != "Wall W-1 on grid A, 200 CMU":
                findings.append(f"W-1: Description {product.Description}")
        elif labels:
            findings.append(f"{eid}: unexpected labels in fixture")
    return findings


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--aware", required=True)
    parser.add_argument("--keep")
    args = parser.parse_args()
    work = args.keep or tempfile.mkdtemp(prefix="aware-extrusion-e2e-")
    os.makedirs(work, exist_ok=True)
    failures = 0

    def check(name, ok, detail=""):
        nonlocal failures
        print(f"  {'PASS' if ok else 'FAIL'}  {name}{(' - ' + detail) if detail and not ok else ''}")
        failures += 0 if ok else 1

    try:
        base = scene()
        path, receipt = write_ifc(args.aware, base, work, "main")
        print(f"wrote {path} ({receipt['data']['bytes']} bytes), extrusions={receipt['data'].get('extrusions')}")
        check("receipt counts six extrusions", receipt["data"].get("extrusions") == 6, json.dumps(receipt["data"])[:400])
        rules, errors = schema_errors(path)
        check(f"IFC4 validation ({rules}) reports no error", not errors, json.dumps(errors)[:800])
        findings = read_back(path, base)
        check("read-back: classes, types, quantities, LoadBearing, material, axis, voids, labels, volumes",
              not findings, "; ".join(findings))

        # Negative control 1: no meta.properties -> the label read-back must fail.
        unlabelled = copy.deepcopy(base)
        del unlabelled["elements"][0]["meta"]["properties"]
        path2, _ = write_ifc(args.aware, unlabelled, work, "control-labels")
        control = read_back(path2, unlabelled)
        check("control: without meta.properties the label read-back fails",
              any("label read-back" in f for f in control), "; ".join(control) or "no finding")

        # Negative control 2: a wrong volume -> the volume read-back must fail.
        wrong = copy.deepcopy(base)
        wrong["elements"][1]["quantities"]["netVolumeM3"] = 7.2
        path3, _ = write_ifc(args.aware, wrong, work, "control-volume")
        control = read_back(path3, wrong)
        check("control: a slab netVolumeM3 that ignores its openings fails the volume read-back",
              any("the solids enclose" in f for f in control), "; ".join(control) or "no finding")
        # Negative control 3: the schema check must see a literal the schema refuses.
        broken = os.path.join(work, "control-schema.ifc")
        with open(path, encoding="utf-8") as src, open(broken, "w", encoding="utf-8") as dst:
            dst.write(src.read().replace(".SOLIDWALL.", ".FLOOR.", 1))
        _, errors = schema_errors(broken)
        check("control: a wall typed FLOOR fails the IFC4 schema check", bool(errors), "no error reported")
    finally:
        if not args.keep:
            shutil.rmtree(work, ignore_errors=True)
    print(f"\n{'OK' if failures == 0 else f'{failures} FAILED'}")
    return 0 if failures == 0 else 1


if __name__ == "__main__":
    sys.exit(main())
