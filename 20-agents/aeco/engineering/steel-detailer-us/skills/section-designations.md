---
name: steel-detailer-us-section-designations
description: Use for US/AISC steel section type and naming questions — the shape families (W, HSS, C, L, WT, etc.), the size-naming convention, and the free AISC Shapes Database property source. AISC Manual / Shapes Database v16.
---

# Section types & naming (AISC)

**Shape families:** **W, M, S, HP** (I-shapes); **C, MC** (channels); **L** (angles, equal/unequal); **WT / MT / ST** (structural tees, split from a W/M/S); **HSS** (rectangular/square/round); **Pipe**; **PL** (plate); bars; **2L** (double angles).

**Naming convention:**
- **W16×40** = W-shape, nominal depth ≈ 16 in × weight 40 lb/ft. (Depth is *nominal* — the actual `d` varies by weight within a series.)
- **C12×20.7** = American Standard channel, 12 in deep × 20.7 lb/ft.
- **L4×4×1/4** = angle, 4 in × 4 in legs × 1/4 in thick (equal-leg).
- **HSS6×6×3/8** = square HSS, 6×6 in outside × 3/8 in *nominal* wall (rectangular: depth × width × wall; round: HSS O.D. × wall).
- **Pipe** by nominal diameter + weight class (Std / XS / XXS).

**Wall-thickness note:** HSS thickness is *nominal*; the **design** wall = **0.93 × nominal for A500**, but = **nominal for A1085**.

**Free property data:** the **AISC Shapes Database** (free, aisc.org) gives every shape's dimensions + section properties + the detailing **T** dimension and the **workable gage** (WGi / WGo) — the source to drive a section lookup without hardcoding property tables.

**Deterministic section lookup:** this agent ships those properties as the `sections`
category of the `lookup` command — `lookup --rule section.W16X26` → typed JSON with
`weight_plf`, `depth_in`, `width_in`, `area_in2`, thicknesses, the strength/stiffness set
(`Ix_in4`, `Sx_in3`, `Zx_in3`, `rx_in` and the y/z counterparts, `J_in4`, `Cw_in6`,
`C_in3`), and the detailing dimensions (`T_in`, `kdes_in`/`kdet_in`, `k1_in`,
`WGi_in`/`WGo_in`, and `ddet_in`/`bfdet_in`/`twdet_in`/`tfdet_in` — d/bf/tw/tf rounded to
the nearest 1/16" for shop drawings) + citation (generated from the AISC Shapes Database
v15.0 US). Use it
for weight/depth a designation does **not** encode (HSS, angles, pipe), and for the
section properties and detailing dimensions a check needs without hardcoding a table.
Note `kdes` (design, decimal) and `kdet` (detailing, fractional, larger) are **different
values** — a drawing's bare "k" is `kdet`. See the `lookup-usage` skill.

## Source

- **AISC Shapes Database v16.0** (free; readme Aug 2023), the AISC **"Naming Convention for Structural Steel Products for EDI"**, and AISC Steel Construction Manual 16th ed. Part 1. General dimensional / mill-tolerance requirements: **ASTM A6/A6M** (paid; designation public). Verify free at aisc.org/manualresources.
