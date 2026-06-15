---
name: steel-detailer-eu-steel-grades
description: Use for European steel material-grade questions — EN 10025-2 grades (S235/S275/S355/S420/S460), the designation meaning, design values fy and fu by thickness from EN 1993-1-1 Table 3.1, the toughness subgrades (JR/J0/J2/K2), hollow-section grades, and bolt property classes. EN 1993-1-1 Table 3.1 + EN 10025.
---

# Steel grades (Eurocode 3 — EN design values)

**Designation:** `S` + number = nominal yield strength **fy** (N/mm²) for the *thinnest*
thickness range (S = structural). Common grades per EN 10025-2: S235, S275, S355, S420, S460.

## Design values fy and fu — EN 1993-1-1:2005 Table 3.1

Values reproduced in JRC EUR 27346 and eurocodeapplied.com:

| Grade | fy (t ≤ 16) | fy (16–40) | fy (40–63) | fy (63–80) |
|---|---|---|---|---|
| **S235** | 235 | 225 | 215 | 215 |
| **S275** | 275 | 265 | 255 | 245 |
| **S355** | 355 | 345 | 335 | 325 |
| **S420** | 420 | 400 | 390 | 380 |
| **S460** | 460 | 440 | 430 | 410 |

| Grade | fu (t ≤ 40) | fu (40–80) |
|---|---|---|
| **S235** | 360 | 340 |
| **S275** | 430 | 410 |
| **S355** | 510 | 490 |
| **S420** | 520 | 500 |
| **S460** | 550 | 530 |

*All values in N/mm².*

**Note — fu and weld design:** The fu values above (from EN 1993-1-1 Table 3.1) are the
design nominal values used in EN 1993-1-8 weld calculations (Table 4.1 βw correlation).
EN 10025-2 gives **minimum** specified tensile strength (e.g. S275 min fu = 410 N/mm²
for all thicknesses), which is lower than the Table 3.1 nominal. Weld design uses the
Table 3.1 value (430 for S275 t ≤ 40 mm); member fracture checks also use these values.

## Toughness subgrades — Charpy impact energy (EN 10025-2)

| Subgrade | Energy | Test temperature |
|---|---|---|
| **JR** | 27 J | +20 °C |
| **J0** | 27 J | 0 °C |
| **J2** | 27 J | −20 °C |
| **K2** | 40 J | −20 °C |

Subgrade selection is governed by minimum service temperature and reference temperature
as per EN 1993-1-10 (not covered by this agent — see EN 1993-1-10 directly).

## Hollow sections

- **EN 10210** (hot-finished): grades S275J2H, S355J2H, S355K2H (H = hollow section)
- **EN 10219** (cold-formed): e.g. S275J2H, S355J0H

## Bolt property classes and standards

| Class | fub (N/mm²) | Standard |
|---|---|---|
| 4.6 | 400 | Non-preloaded assemblies (BS EN 15048) |
| 8.8 | 800 | Non-preloaded or preloaded (BS EN 15048 / BS EN 14399) |
| 10.9 | 1000 | Preloaded assemblies (BS EN 14399) |

fu for bolt = fub for shear/bearing checks.

## Source

- **EN 1993-1-1:2005 Table 3.1** (fy / fu design values), via **JRC EUR 27346** Table 0.1
  and **eurocodeapplied.com** material properties page (free).
- **EN 10025-2** (minimum specified tensile — distinct from Table 3.1 nominal); cited
  by reference only (CEN-paywalled).
- Subgrades: **steelconstruction.info** "Steel material properties" (free, cite ?oldid).
- Hollow sections: **EN 10210 / EN 10219** cited by reference; grade designations via
  steelconstruction.info.
