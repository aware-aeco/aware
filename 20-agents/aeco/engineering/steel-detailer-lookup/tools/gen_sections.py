#!/usr/bin/env python3
"""Generate the AISC section-properties lookup rules from the AISC Shapes Database
(US / imperial) CSV into the steel-detailer-us rules file `aisc-shapes-v15.json`.

Each row becomes a deterministic, citation-backed `sections` rule consumed by the
`aware-steel-detailer-us lookup` command — designation -> verified weight, depth,
width, area, thicknesses, plus the strength/stiffness set (I, S, Z, r, J, Cw).
No engine logic here; this is a pure data transform so the import is reproducible
and auditable.

Run:  python gen_sections.py
"""
import csv, hashlib, json, os, re

HERE = os.path.dirname(os.path.abspath(__file__))
CSV = os.path.normpath(os.path.join(HERE, "..", "data", "aisc-shapes-database-v15.0-us.csv"))
OUT = os.path.normpath(os.path.join(
    HERE, "..", "..", "steel-detailer-us", "rules", "aisc-shapes-v15.json"))


_NA = {"", "-", "–", "—", "N/A", "n/a"}

# Strength / stiffness columns, copied straight across when the CSV has them.
# (csv column, property key). Units are in the key suffix, matching the dimensional
# props above (weight_plf / depth_in / area_in2). Blank cells are family-dependent —
# Iz/rz/Sz are single-angle principal axes, C is the HSS torsional constant, Cw does
# not apply to closed sections — so "absent" is normal and simply omits the key.
SECTION_PROPS = [
    ("Ix", "Ix_in4"), ("Sx", "Sx_in3"), ("Zx", "Zx_in3"), ("rx", "rx_in"),
    ("Iy", "Iy_in4"), ("Sy", "Sy_in3"), ("Zy", "Zy_in3"), ("ry", "ry_in"),
    ("Iz", "Iz_in4"), ("Sz", "Sz_in3"), ("rz", "rz_in"),
    ("J", "J_in4"), ("Cw", "Cw_in6"), ("C", "C_in3"),

    # Detailing dimensions — what a detailer lays out to, rather than what a designer
    # calculates with. `kdes` is the decimal design k; `kdet` is the (larger, fractional)
    # detailing k and the one to lay out from — both are served so a consumer picks
    # deliberately instead of guessing which k a bare "k" meant. `T` is the clear web
    # depth between flange fillets, `k1` the web-centreline-to-flange-fillet-toe
    # clearance, `WGi`/`WGo` the inner/outer workable flange gages.
    ("T", "T_in"), ("kdes", "kdes_in"), ("kdet", "kdet_in"), ("k1", "k1_in"),
    ("WGi", "WGi_in"), ("WGo", "WGo_in"),

    # Detailing counterparts of d / bf / tw / tf — the same dimensions rounded to the
    # nearest 1/16" for shop drawings. They travel with the decimal design values rather
    # than replacing them: `tf` 0.345 and `tfdet` 0.375 are the same flange, one for a
    # strength calc and one for a layout. Rolled shapes only.
    ("ddet", "ddet_in"), ("bfdet", "bfdet_in"),
    ("twdet", "twdet_in"), ("tfdet", "tfdet_in"),

    # Centroid / shear-centre / plastic-axis locations. `x` and `y` are the centroid
    # measured from the reference face (angle heel, channel web back, tee flange face) —
    # what a member is actually located and gaged from. `eo` is the shear-centre offset
    # (channels only): load applied off it twists the member. `xp`/`yp` locate the
    # plastic neutral axis.
    ("x", "x_in"), ("y", "y_in"), ("eo", "eo_in"),
    ("xp", "xp_in"), ("yp", "yp_in"),

    # Principal-axis angle for single angles — dimensionless, and the orientation the
    # already-served Iz/Sz/rz are measured about, so those are unusable without it.
    ("tan(a)", "tan_alpha"),

    # Distance between flange centroids.
    ("ho", "ho_in"),

    # Surface perimeters, for paint / galvanizing / fireproofing quantity. Relationships
    # verified against the data below: PB is the full shape perimeter, PD the box
    # perimeter 2(d+bf), and PA/PC are those minus one flange face (3-sided contour and
    # box fireproofing). PA2 is the single-angle variant.
    ("PA", "PA_in"), ("PA2", "PA2_in"), ("PB", "PB_in"),
    ("PC", "PC_in"), ("PD", "PD_in"),
]


def num(s):
    """CSV cell -> float, or None for a known N/A token. Raises on anything else, so a
    surprising / corrupt cell is never silently dropped (verified-data discipline)."""
    if s is None:
        return None
    s = s.strip()
    if s in _NA:
        return None
    try:
        return float(s)
    except ValueError:
        raise ValueError(f"unparseable numeric cell {s!r}")


def first(*vals):
    for v in vals:
        if v is not None:
            return v
    return None


def fmt(v, unit):
    if v is None:
        return None
    s = f"{v:g}"
    return f"{s} {unit}"


def main():
    csv_sha256 = hashlib.sha256(open(CSV, "rb").read()).hexdigest()
    with open(CSV, newline="", encoding="utf-8-sig") as f:
        rows = list(csv.DictReader(f))

    rules = []
    defin = {"W", "M", "S", "HP", "C", "MC", "WT", "MT", "ST"}
    defin_ok = defin_bad = 0

    for r in rows:
        label = r.get("AISC_Manual_Label", "").strip()
        typ = r.get("Type", "").strip()
        if not label or not typ:
            continue

        weight = num(r.get("W"))
        area = num(r.get("A"))
        # depth: I-shapes/channels/tees/angles=d; HSS rect=Ht; round HSS/pipe=OD
        depth = first(num(r.get("d")), num(r.get("Ht")), num(r.get("OD")))
        # width: flange bf; HSS rect B; round OD
        width = first(num(r.get("bf")), num(r.get("B")), num(r.get("OD")))
        tw = num(r.get("tw"))      # web thickness (I-shapes/channels)
        tf = num(r.get("tf"))      # flange thickness
        wall = first(num(r.get("tdes")), num(r.get("tnom")))  # HSS/pipe design wall

        # cross-check: definitional weight (trailing number == W) for I-family
        if typ in defin:
            m = re.search(r"[Xx]([0-9.]+)$", label)
            if m and weight is not None:
                if abs(float(m.group(1)) - weight) < 0.6:
                    defin_ok += 1
                else:
                    defin_bad += 1

        props = {"type": typ}
        if weight is not None: props["weight_plf"] = weight
        if depth is not None: props["depth_in"] = depth
        if width is not None: props["width_in"] = width
        if area is not None: props["area_in2"] = area
        if tw is not None: props["web_in"] = tw
        if tf is not None: props["flange_in"] = tf
        if wall is not None: props["wall_in"] = wall

        strength = {k: v for col, k in SECTION_PROPS if (v := num(r.get(col))) is not None}
        props.update(strength)

        # Family-dependent columns, renamed by meaning rather than served under one
        # ambiguous key. `h`/`b` are the HSS flat dimensions (Ht-3t, B-3t) — the flat
        # a connection actually lands on — but `b` on an angle is the second leg, a
        # different quantity entirely. `t` is the angle leg thickness, which no other
        # key carries for L/2L; it is NOT named `t_in`, because a key differing from
        # `T_in` (clear web depth) only by case is a misread waiting to happen.
        if typ == "HSS":
            family = [("h", "flat_h_in"), ("b", "flat_b_in")]
        elif typ in ("L", "2L"):
            family = [("b", "leg2_in"), ("t", "angle_t_in")]
        elif typ == "PIPE":
            family = [("ID", "ID_in")]
        else:
            family = []
        props.update({k: v for col, k in family if (v := num(r.get(col))) is not None})

        # `d` means different things by family: leg for an angle, OD for a round shape,
        # depth otherwise. Label the human-readable string honestly (properties are typed).
        is_round = typ in ("HSS", "PIPE") and num(r.get("OD")) is not None
        dlabel = "leg" if typ == "L" else ("OD" if is_round else "depth d")
        value = "; ".join(x for x in (
            fmt(weight, "lb/ft"),
            f"{dlabel} = {depth:g} in" if depth is not None else None,
            f"area A = {area:g} in²" if area is not None else None,
        ) if x)

        # The quote is what a reader verifies the served properties against, so every
        # property we serve appears in it — including the strength/stiffness set.
        quote = ", ".join(
            f"{k}={v:g}" for k, v in [
                ("W", weight), ("A", area), ("d", num(r.get("d"))),
                ("Ht", num(r.get("Ht"))), ("OD", num(r.get("OD"))),
                ("bf", num(r.get("bf"))), ("B", num(r.get("B"))),
                ("tw", tw), ("tf", tf), ("tdes", num(r.get("tdes"))),
            ] + [(col, num(r.get(col))) for col, _ in SECTION_PROPS + family]
            if v is not None)

        rules.append({
            "id": f"section.{label}",
            "category": "sections",
            "rule": f"{label} — section properties (AISC)",
            "value": value,
            "units": "imperial (lb/ft, in, in², in³, in⁴, in⁶)",
            "properties": props,
            "citation": "AISC Shapes Database v15.0 (US)",
            "source_quote": f"{label}: {quote}",
            "found": True,
        })

    # Surface-perimeter identities, checked on every W-shape. The AISC CSV ships no data
    # dictionary, so these relationships are what pin down which column is which: PB is
    # the full perimeter, PA = PB - bf (one flange face off, contour fireproofing), PD is
    # the box perimeter 2(d+bf), PC = PD - bf. If the P-block ever shifts, this fails
    # loudly instead of mislabelling paint areas.
    perim_checked = 0
    for p in (r["properties"] for r in rules if r["properties"]["type"] == "W"):
        d, bf = p["depth_in"], p["width_in"]
        for label, got, want in (
            ("PA = PB - bf", p["PA_in"], p["PB_in"] - bf),
            ("PD = 2(d+bf)", p["PD_in"], 2 * (d + bf)),
            ("PC = PD - bf", p["PC_in"], p["PD_in"] - bf),
        ):
            # 1.5% relative. Measured worst case across all 283 W-shapes is 0.51%
            # (W44X230) — AISC tabulates these to 3 significant figures and appears to
            # take the box perimeter off the detailing dimensions, not decimal d/bf. A
            # genuinely mislabelled column would be out by tens of percent, so this sits
            # well clear of the noise and still catches a shift.
            if abs(got - want) > max(0.015 * abs(want), 0.25):
                raise SystemExit(
                    f"ABORT: perimeter identity {label} fails ({got} vs {want:.3f}) — "
                    f"the PA/PB/PC/PD columns are not what they are labelled.")
        perim_checked += 1
    print(f"perimeter identities: {perim_checked} W-shapes ok")

    if defin_bad:
        raise SystemExit(
            f"ABORT: {defin_bad} I-family shapes whose trailing designation number "
            f"!= W column — CSV may be metric or corrupt (got {defin_ok} ok).")
    print(f"definitional cross-check: {defin_ok} ok / {defin_bad} bad")

    # Hard imperial sentinel: known v15 US values. Guards against a metric / wrong file
    # (the definitional check alone passes on metric data, which also encodes weight).
    # Each sentinel also pins strength/stiffness values, so a mis-mapped column (Ix
    # read as Iy, Zx as Sx, HSS `C` as `Cw`) aborts instead of shipping wrong numbers.
    by_id = {r["id"]: r["properties"] for r in rules}
    for sid, expect in {
        "section.W16X26": {"weight_plf": 26.0, "depth_in": 15.7, "area_in2": 7.68,
                           "Ix_in4": 301, "Sx_in3": 38.4, "Zx_in3": 44.2,
                           "Iy_in4": 9.59, "ry_in": 1.12, "Cw_in6": 565,
                           "T_in": 13.625, "kdes_in": 0.747, "kdet_in": 1.0625,
                           "k1_in": 0.75, "WGi_in": 3.5,
                           # detailing dims round the decimals above: tf 0.345 -> 3/8
                           "ddet_in": 15.75, "bfdet_in": 5.5,
                           "twdet_in": 0.25, "tfdet_in": 0.375,
                           "ho_in": 15.4, "PA_in": 46.7, "PB_in": 52.2,
                           "PC_in": 36.9, "PD_in": 42.4},
        "section.C12X25": {"weight_plf": 25.0, "x_in": 0.674, "eo_in": 0.746,
                           "xp_in": 0.306, "T_in": 9.75},
        "section.HSS6X6X3/8": {"weight_plf": 27.48, "depth_in": 6.0, "area_in2": 7.58,
                               "Ix_in4": 39.5, "Zx_in3": 15.8, "J_in4": 64.6,
                               "C_in3": 22.1,
                               # flats = Ht-3t / B-3t, the face a connection lands on
                               "flat_h_in": 4.95, "flat_b_in": 4.95},
        "section.L4X4X1/4": {"weight_plf": 6.6, "depth_in": 4.0, "area_in2": 1.93,
                             "Ix_in4": 3.0, "Iz_in4": 1.19, "rz_in": 0.783,
                             "kdes_in": 0.625, "kdet_in": 0.625,
                             "x_in": 1.08, "y_in": 1.08, "tan_alpha": 1.0,
                             "angle_t_in": 0.25, "leg2_in": 4.0, "PA2_in": 12.0},
    }.items():
        p = by_id.get(sid)
        if not p:
            raise SystemExit(f"ABORT: sentinel {sid} missing from generated rules.")
        for k, want in expect.items():
            got = p.get(k)
            # 0.5% relative — tight enough to catch a swapped column, loose enough for
            # the CSV's 3-significant-figure rounding.
            if got is None or abs(got - want) > max(0.005 * abs(want), 0.005):
                raise SystemExit(
                    f"ABORT: sentinel {sid}.{k} = {got!r}, expected {want} — "
                    f"wrong/metric CSV or a mis-mapped column?")
    # Keys that must be ABSENT for a family — a closed section has no warping constant
    # and no rolled-in fillet, so a k / T / gage on an HSS means the columns have drifted.
    for sid, absent in {
        "section.HSS6X6X3/8": ["Cw_in6", "kdes_in", "kdet_in", "T_in", "WGi_in",
                               "ddet_in", "twdet_in"],
        "section.L4X4X1/4": ["C_in3", "T_in", "ddet_in", "tfdet_in"],
    }.items():
        for k in absent:
            if k in by_id[sid]:
                raise SystemExit(
                    f"ABORT: {sid} has {k}, which does not apply to that family — "
                    f"column mis-mapped?")

    db = {
        "agent": "steel-detailer-us",
        "dataset": "aisc-shapes-database-v15.0-us",
        "standard": "aisc-shapes-database",
        "version": "15.0",
        "last_verified": "2026-06-17",
        "sources": [
            {
                "name": "AISC Shapes Database v15.0 (US, imperial)",
                "note": "Section dimensions & properties per the AISC Steel Construction "
                        "Manual, 15th ed. (free at aisc.org/manualresources). Section "
                        "geometry is edition-stable; the v15.0 dimensional properties of "
                        "the shapes used here were spot-checked against v16.0 — a full "
                        "v15-vs-v16 diff was not performed.",
                "access": {
                    "mirror": "github.com/ambaker1/aisc-csv — v15.0/Shapes-US.csv",
                    "mirror_commit": "38d68425df714141465897b885a9e7a208c1161c",
                    "accessed": "2026-06-17",
                    "vendored": "steel-detailer-lookup/data/aisc-shapes-database-v15.0-us.csv",
                    "vendored_sha256": csv_sha256,
                },
            }
        ],
        "rules": rules,
    }
    with open(OUT, "w", encoding="utf-8") as f:
        json.dump(db, f, indent=1, ensure_ascii=False)
    print(f"wrote {len(rules)} section rules -> {OUT}")


if __name__ == "__main__":
    main()
