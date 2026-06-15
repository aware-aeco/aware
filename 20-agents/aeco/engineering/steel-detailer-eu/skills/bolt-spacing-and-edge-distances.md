---
name: steel-detailer-eu-bolt-spacing-and-edge-distances
description: Use for Eurocode bolt spacing, edge-distance and end-distance questions — minimum and maximum e1/e2/p1/p2 per EN 1993-1-8:2005 Table 3.3. These are fixed EN values (not NDPs); they apply uniformly in all countries using EN 1993-1-8. EN 1993-1-8:2005 §3.5.
---

# Bolt spacing & edge/end distances (Eurocode 3 — EN recommended values)

Notation: **d0** = hole diameter, **t** = thickness of the thinnest outer connected part.
These limits are fixed EN values, **not NDPs** — they are the same regardless of which
country's National Annex is applied.

## Minimums — EN 1993-1-8:2005 Table 3.3 §3.5

| Quantity | Minimum |
|---|---|
| End distance **e1** (parallel to load) | **1.2·d0** |
| Edge distance **e2** (perpendicular to load) | **1.2·d0** |
| Distance to slotted-hole centre **e3** (end) | **1.5·d0** |
| Distance to slotted-hole centre **e4** (edge) | **1.5·d0** |
| Bolt pitch **p1** (in direction of load) | **2.2·d0** |
| Bolt pitch **p2** (perpendicular to load) | **2.4·d0** |

**Edition trap:** The pre-2005 ENV 1993 draft gave different minimums (e.g. e2 ≥ 1.5·d0
for edge distance). Those values are superseded. Always use the 2005 Table 3.3 values.

## Maximums — Table 3.3 §3.5

**Edge and end distances (e1, e2):**
- Parts *exposed to weather or other corrosive influences*: **4t + 40 mm** (larger of)
- Parts *not exposed*: **min(8t, 125 mm)** wait — actually the larger of **8t** or **125 mm**
  is the limit. Check: Table 3.3 gives the larger of 8t and 125 mm for un-exposed.
  *Correction*: **max(8t, 125 mm)**.

**Bolt pitch p1, p2:**
- Outer rows: **min(14t, 200 mm)** (whichever is smaller)
- Inner rows, compression members (in the direction of compression):
  - 14t / 200 mm in outer rows
  - **28t / 400 mm** in inner rows where all plate elements are in compression

**Weathering steel (EN 10025-5):** the 200 mm upper limit tightens to **175 mm** (Table 3.3 note).

## Bearing-resistance threshold (Table 3.4) — not the minimum, but affects bearing Fb,Rd

For bearing resistance not to be reduced below the basic formula: e1 ≥ 1.5·d0 and
e2 ≥ 1.5·d0; p1 ≥ 3.75·d0; p2 ≥ 3.0·d0. Below these, the factors αb / k1 reduce.
(See `bolt-shear-and-bearing`.)

## Source

- **EN 1993-1-8:2005 Table 3.3 §3.5**, as reproduced in:
  - **JRC EUR 27346** (DOI 10.2788/605700), Table 1.2 / worked examples Section 1
  - **eurocodeapplied.com** EN 1993-1-8 bolt layout page (free)
  - **steelconstruction.info** connection design articles
- The standard text is CEN/BSI-paywalled; values corroborated from the free sources above.
- The pre-2005 ENV values (e2 ≥ 1.5·d0; max = 12t / 150 mm) are superseded.
