---
name: steel-detailer-us-lookup-usage
description: Use when the user wants deterministic, machine-readable rule values from the AISC database — for checker workflows, scripting, or when you need a hard provenance guarantee. Explains how to invoke the lookup command and what it returns.
---

# Deterministic lookup command (AISC)

The `steel-detailer-us` agent ships a **`lookup` CLI command** (`aware-steel-detailer-us.exe`) that queries a verified, versioned rules database without any LLM. It is the hard, lock-able provenance guarantee that sits below the advisory skill layer.

## CLI contract

```
aware-steel-detailer-us lookup --rule <id>
aware-steel-detailer-us lookup --category <cat>
aware-steel-detailer-us lookup --list
aware-steel-detailer-us describe
```

**Exit codes:** `0` = found (or listing), `1` = not found, `2` = error.

**Output:** JSON to stdout matching the schema:

```json
{
  "id": "bolt.pretension.group120.0.75in",
  "category": "bolts",
  "rule": "Minimum pretension — Group 120, ¾ in. bolt",
  "value": "28 kips",
  "units": "kips",
  "citation": "RCSC 2020 Table 5.2 (= AISC 360-22 Table J3.1)",
  "source_quote": "Table 5.2: ¾ in., Group 120 = 28 kips",
  "found": true
}
```

When `found: false`, all fields except `id` are `null` — the caller **must refuse or flag**, not interpolate.

**`sections` rules carry an extra `properties` object** with typed numeric fields, so a
consumer reads machine values directly (not by parsing the `value` string):

```json
{
  "id": "section.W16X26",
  "category": "sections",
  "value": "26 lb/ft; depth d = 15.7 in; area A = 7.68 in²",
  "units": "imperial (lb/ft, in, in², in³, in⁴, in⁶)",
  "properties": { "type": "W", "weight_plf": 26.0, "depth_in": 15.7,
                  "width_in": 5.5, "area_in2": 7.68, "web_in": 0.25, "flange_in": 0.345,
                  "Ix_in4": 301.0, "Sx_in3": 38.4, "Zx_in3": 44.2, "rx_in": 6.26,
                  "Iy_in4": 9.59, "Sy_in3": 3.49, "Zy_in3": 5.48, "ry_in": 1.12,
                  "J_in4": 0.262, "Cw_in6": 565.0,
                  "T_in": 13.625, "kdes_in": 0.747, "kdet_in": 1.0625,
                  "k1_in": 0.75, "WGi_in": 3.5 },
  "citation": "AISC Shapes Database v15.0 (US)",
  "found": true
}
```

The rule `id` is `section.<AISC label>` (e.g. `section.HSS6X6X3/8`); uppercase the `x` in a
drawing designation before the lookup. `wall_in` (HSS/pipe design wall) appears instead of
`web_in`/`flange_in` for tubes; **`wall_in` and the HSS/pipe `weight_plf` are the AISC
design-wall basis** (0.93× nominal for A500; nominal for A1085 — see `section-designations`).
The bulk of weights/depths are *not* derivable from the designation (HSS, angles, pipe) —
that is exactly why this lookup exists.

The strength/stiffness keys are present only where the AISC database defines them for
that family — absence is meaningful, not missing data: `Iz_in4`/`Sz_in3`/`rz_in` are the
single-angle principal axes (L only); `C_in3` is the HSS torsional constant (closed
sections only); `Cw_in6` is the warping constant, which open sections have and closed
ones do not. Read the key you need and refuse if it is absent — never substitute a
neighbouring axis.

**Detailing dimensions** ride alongside the design properties, on the shapes that have
rolled-in fillets (none of them appear on HSS or pipe):

| key | what it is | present on |
|---|---|---|
| `kdes_in` | design k — decimal, for **calculations** (web local yielding, etc.) | rolled shapes + angles |
| `kdet_in` | detailing k — the tabulated fractional value; lay out copes/clips to this one | rolled shapes + angles |
| `T_in` | clear web depth between flange fillets — the room a connection actually gets | W/M/S/HP/C/MC |
| `k1_in` | web centreline to flange-fillet toe — flange bolt clearance | rolled I-shapes |
| `WGi_in` / `WGo_in` | inner / outer workable flange gage | `WGo` only on wide flanges |
| `ddet_in` `bfdet_in` `twdet_in` `tfdet_in` | d / bf / tw / tf rounded to the nearest 1/16" for shop drawings | rolled shapes |
| `x_in` / `y_in` | centroid from the reference face — angle heel, channel web back, tee flange face | L/2L/C/MC, tees |
| `eo_in` | shear-centre offset; load applied off it twists the member | C/MC only |
| `xp_in` / `yp_in` | plastic neutral axis location | as `x`/`y` |
| `tan_alpha` | tangent of the principal-axis angle — **dimensionless**; the orientation `Iz`/`Sz`/`rz` are measured about | single angles |
| `ho_in` | distance between flange centroids | W/M/S/HP/C/MC |
| `flat_h_in` / `flat_b_in` | HSS flat depth / width (`Ht−3t`, `B−3t`) — the flat a connection actually lands on | HSS |
| `leg1_in` / `leg2_in` | first / second leg **as written in the designation**, longer first — `L6X4X1/2` gives 6 and 4. Read from the designation, not from `d`/`b`, whose axis roles swap between 2L LLBB and SLBB | L, 2L |
| `angle_t_in` | leg thickness | L, 2L |
| `ID_in` | pipe inside diameter | Pipe |
| `PB_in` / `PD_in` | full shape perimeter / box perimeter `2(d+bf)` | see below |
| `PA_in` / `PC_in` | the same two minus one flange face (3-sided contour and box fireproofing) | see below |
| `PA2_in` | single angles: `PB` minus the **first (longer)** leg, where `PA_in` drops the **second (shorter)** one | L only |

The `*det` keys **pair with** the decimal design values, they do not replace them —
W16X26 carries both `flange_in` 0.345 and `tfdet_in` 0.375 for the same flange. The rule
is the same one that separates `kdes` from `kdet`:

> **decimal for the calculation, detailing value for the layout.**

**`kdes` and `kdet` are not interchangeable** — a bare "k" on a drawing means the
detailing k. Likewise a strength check run on `tfdet_in` overstates the flange by 1/32"
on a W16X26. Pick the key by what the answer is for, and say which one was used when
citing a number.

Do **not** assume `kdet` is the larger of the two. It usually is (613 shapes), but it is
equal on 128 and smaller on 144 — every C, MC, M, S, ST, MT and most of the angles. Most
of those are the 3-significant-figure decimal of the same fraction (C12X25: `kdes` 1.13
vs `kdet` 1 1/8), though four L12X12 shapes differ by a real ~1/32". Read the key you
need; never pick "the bigger one" as a proxy for it.

`T_in` is a **tabulated** AISC value, not `d − 2k` — do not recompute it, and do not
derive one of these from another; if the key you need is absent for that shape, refuse.

The **surface perimeters** are for paint / galvanizing / fireproofing quantity, in inches
of perimeter (multiply by length for area). Pick by how the member is actually coated:
`PB` all four sides, `PA` three sides where a slab or deck covers the top flange, and the
`PD`/`PC` box pair where the coating boxes the shape rather than following its contour.
Their identities (`PA = PB − bf`, `PD = 2(d+bf)`, `PC = PD − bf`) are asserted against
every flanged rolled shape when the table is generated, so a shifted column fails the
build.

**Eight HP shapes have no perimeter keys at all** — `HP14X117/102/89/73` and
`HP12X84/74/63/53`, whose `PA` is a verbatim copy of that row's `rts` (4.15 in where the
perimeter is 70.5 in). The defect is in the AISC database itself and is present in **both
v15.0 and v16.0**, verified against a v16.0 export, so there is no edition to recover the
value from. The keys are dropped from the typed properties **and** from `source_quote`,
so there is nothing corrupt left to quote. A lookup returns them absent: **refuse the
coating question for those eight shapes** and say the source data is bad — do not
substitute a neighbouring size and do not compute the perimeter yourself. Every other
property on them is unaffected and safe to use.

`S24X90` was a ninth case and is now **corrected**, not quarantined: v15.0 gave `PB` as
725 for 72.5 (a dropped decimal), which v16.0 fixes. That rule's citation and quote both
disclose the correction, so a reader can see the value did not come from the vendored
v15.0 file. The full lists are in the dataset's `data-quality` block.

On a **single angle** the excluded face is a leg, not a flange, and which leg differs
between the two keys: `PA = PB − leg2` (drops the shorter leg), `PA2 = PB − leg1` (drops
the longer one). For an `L6X4X1/2` that is 16 in vs 14 in — picking the wrong one is a
12% error in coating area. Equal-leg angles hide the distinction, so decide from the
designation, not from a symmetric example. These identities are likewise asserted across
all 137 single angles at generation time.

For a **double angle**, `depth_in` is the assembled section's depth and so depends on the
back-to-back orientation — `2L8X6X1LLBB` is 8 in deep, `2L8X6X1SLBB` is 6 in — while
`leg1_in`/`leg2_in` stay 8 and 6 for both, because they name the designation's legs.
Use the legs to identify the angle and `depth_in` to fit the assembled member.

`tan_alpha` is the one **dimensionless** key — it carries no `_in` suffix for that
reason. A single angle's `Iz`/`Sz`/`rz` are meaningless without it.

## Available categories

- `bolts` — spacing, edge distances, hole sizes, pretension values
- `welds` — fillet sizes, throat, length limits, PJP throat
- `connection-strength` — bearing and tearout nominal strength equations (§J3.11)
- `materials` — preferred ASTM grades and Fy/Fu by member type
- `sections` — section properties (weight/ft, depth, width, area, thicknesses, the
  strength/stiffness set I, S, Z, r about both axes plus J/Cw/C, and the detailing
  dimensions T, kdes/kdet, k1, the workable gages, the rounded ddet/bfdet/twdet/tfdet,
  the centroid / shear-centre / principal-axis locations and the surface perimeters for
  coating quantity) for every
  AISC shape, from the AISC Shapes Database (W, M, S, HP, C, MC, L, 2L, WT/MT/ST, HSS,
  Pipe). `lookup --rule section.<label>` or `--category sections`.

Run `aware-steel-detailer-us lookup --list` for all rule IDs (66 connection rules +
the ~2,090-shape AISC section table).

## Rules database location

- `~/.aware/agents/steel-detailer-us/rules/aisc-360-22.json` — the 66 curated,
  hand-verified connection rules (citation + source quote each).
- `~/.aware/agents/steel-detailer-us/rules/aisc-shapes-v15.json` — the generated AISC
  section table (`sections` category), merged into the same lookup at runtime. Missing is
  fine (connection rules still work); present-but-invalid is a hard error.

## How to use the lookup result

The `lookup` command is a standalone deterministic CLI (decalog #9 — no LLM in the run path): invoke it directly, or from a checker script, and consume its typed JSON. A checker compares extracted model values against `result.value`:

```
[model read] → [lookup bolt.spacing.min] → [compare & report]
```

If `found: false`, the consumer reports "rule not in verified database" and does NOT fall back to inference.

> The agent is `status: planned`: today the lookup is a standalone CLI (build it per the install note below). Composing it as a first-class node in a runnable AWARE `.app` is planned — it lights up when the agent becomes `available`.

## Install note

Binary: `~/.aware/bin/aware-steel-detailer-us.exe` (build from `20-agents/aeco/engineering/steel-detailer-lookup/` via `cargo build --release`). Requires `~/.aware/bin/` on PATH. Both rules files (`aisc-360-22.json` + `aisc-shapes-v15.json`) are installed by `aware agent install steel-detailer-us`.

## Source

- Connection rules: `20-agents/aeco/engineering/steel-detailer-us/rules/aisc-360-22.json` (verified 2026-06-14; all rules traced to free AISC / RCSC documents).
- Section table: `.../rules/aisc-shapes-v15.json` — generated from the AISC Shapes Database v15.0 (US) by `steel-detailer-lookup/tools/gen_sections.py` (vendored source CSV under `steel-detailer-lookup/data/`). Section geometry is edition-stable; v16.0 supersedes v15.0 with no change to existing shapes' dimensional properties.
- CLI source: `20-agents/aeco/engineering/steel-detailer-lookup/src/main.rs` (Rust, no LLM in the run path; decalog #9 compliant).
