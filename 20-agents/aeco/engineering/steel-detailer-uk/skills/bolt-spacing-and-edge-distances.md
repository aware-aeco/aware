---
name: steel-detailer-uk-bolt-spacing-and-edge-distances
description: Use for UK/Eurocode bolt spacing, edge-distance and end-distance questions — minimum and maximum e1/e2/p1/p2 per EN 1993-1-8 Table 3.3, plus P358's practical detailing recommendations. BS EN 1993-1-8:2005 + UK NA.
---

# Bolt spacing & edge/end distances (UK / Eurocode 3)

Notation: **d0** = hole diameter, **d** = bolt diameter, **t** = thickness of the thinner outer connected part.

**Minimums — BS EN 1993-1-8:2005 Table 3.3:**

| Quantity | Minimum |
|---|---|
| End distance e1 (parallel to load) | **1.2·d0** |
| Edge distance e2 (perpendicular to load) | **1.2·d0** |
| Slotted-hole distances e3, e4 (to slot centre) | **1.5·d0** |
| Spacing p1 (in direction of load) | **2.2·d0** |
| Spacing p2 (perpendicular) | **2.4·d0** |

**Maximums — Table 3.3:**
- **e1, e2:** **4t + 40 mm** for members *exposed* to weather / corrosion; **the larger of 8t or 125 mm** if *not* exposed.
- **p1, p2:** **the smaller of 14t or 200 mm**. Compression members — inner rows in the direction of stress: **the smaller of 28t or 400 mm**; outer rows: 14t / 200 mm. Weathering steel (EN 10025-5): the 200 mm cap tightens to 175 mm.

**Bearing-unaffected thresholds (NOT the minimums) — Table 3.4:** for bearing resistance not to be reduced, e1, e2 ≥ 1.5·d0 and p1 ≥ 3.75·d0, p2 ≥ 3.0·d0. Below these, the bearing factors αb / k1 reduce (see `bolt-shear-and-bearing`).

**P358 practical detailing** (its standardised connections): e2 ≥ 1.2·d0; minimise e1; bolt pitch **p ≥ 2.5·d** (and ≤ 10·d for hollow-section splices); cover-plate maximum vertical spacing **p1 = 14·t**; for plates in compression check plate buckling **p1,j/t ≤ 9·ε**, ε = √(235/fy).

## Source

- Minimums & maximums: **BS EN 1993-1-8:2005 Table 3.3 (§3.5)** — values corroborated across the free SCI/BCSA **steelconstruction.info** + **eurocodeapplied.com** (the standard itself is BSI-paywalled). *Single-source residual:* the compression p1,0 / p1,i split (14t/200 vs 28t/400) was found verbatim on one strong free source — verify against EN 1993-1-8 Table 3.3.
- P358 practice: **SCI P358** §2 / §5 / §6 (Checks + worked examples).
- **Edition trap:** the pre-2005 ENV draft gave different numbers (min e2 = 1.5·d0; max = 12t / 150 mm) — superseded; use the 2005 values above.
