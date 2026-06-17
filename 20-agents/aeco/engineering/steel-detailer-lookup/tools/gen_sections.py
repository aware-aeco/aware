#!/usr/bin/env python3
"""Generate the AISC section-properties lookup rules from the AISC Shapes Database
(US / imperial) CSV into the steel-detailer-aisc rules file `aisc-shapes-v15.json`.

Each row becomes a deterministic, citation-backed `sections` rule consumed by the
`aware-steel-detailer-aisc lookup` command — designation -> verified weight, depth,
width, area, thicknesses. No engine logic here; this is a pure data transform so the
import is reproducible and auditable.

Run:  python gen_sections.py
"""
import csv, hashlib, json, os, re

HERE = os.path.dirname(os.path.abspath(__file__))
CSV = os.path.normpath(os.path.join(HERE, "..", "data", "aisc-shapes-database-v15.0-us.csv"))
OUT = os.path.normpath(os.path.join(
    HERE, "..", "..", "steel-detailer-aisc", "rules", "aisc-shapes-v15.json"))


_NA = {"", "-", "–", "—", "N/A", "n/a"}


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

        # `d` means different things by family: leg for an angle, OD for a round shape,
        # depth otherwise. Label the human-readable string honestly (properties are typed).
        is_round = typ in ("HSS", "PIPE") and num(r.get("OD")) is not None
        dlabel = "leg" if typ == "L" else ("OD" if is_round else "depth d")
        value = "; ".join(x for x in (
            fmt(weight, "lb/ft"),
            f"{dlabel} = {depth:g} in" if depth is not None else None,
            f"area A = {area:g} in²" if area is not None else None,
        ) if x)

        quote = ", ".join(
            f"{k}={v:g}" for k, v in (
                ("W", weight), ("A", area), ("d", num(r.get("d"))),
                ("Ht", num(r.get("Ht"))), ("OD", num(r.get("OD"))),
                ("bf", num(r.get("bf"))), ("B", num(r.get("B"))),
                ("tw", tw), ("tf", tf), ("tdes", num(r.get("tdes"))),
            ) if v is not None)

        rules.append({
            "id": f"section.{label}",
            "category": "sections",
            "rule": f"{label} — section properties (AISC)",
            "value": value,
            "units": "imperial (lb/ft, in, in²)",
            "properties": props,
            "citation": "AISC Shapes Database v15.0 (US)",
            "source_quote": f"{label}: {quote}",
            "found": True,
        })

    if defin_bad:
        raise SystemExit(
            f"ABORT: {defin_bad} I-family shapes whose trailing designation number "
            f"!= W column — CSV may be metric or corrupt (got {defin_ok} ok).")
    print(f"definitional cross-check: {defin_ok} ok / {defin_bad} bad")

    # Hard imperial sentinel: known v15 US values. Guards against a metric / wrong file
    # (the definitional check alone passes on metric data, which also encodes weight).
    by_id = {r["id"]: r["properties"] for r in rules}
    for sid, (w, d, a) in {
        "section.W16X26": (26.0, 15.7, 7.68),
        "section.HSS6X6X3/8": (27.48, 6.0, 7.58),
        "section.L4X4X1/4": (6.6, 4.0, 1.93),
    }.items():
        p = by_id.get(sid)
        if (not p or abs(p["weight_plf"] - w) > 0.01 or abs(p["depth_in"] - d) > 0.05
                or abs(p["area_in2"] - a) > 0.01):
            raise SystemExit(
                f"ABORT: imperial sentinel {sid} mismatch (got {p}) — wrong/metric CSV?")

    db = {
        "agent": "steel-detailer-aisc",
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
