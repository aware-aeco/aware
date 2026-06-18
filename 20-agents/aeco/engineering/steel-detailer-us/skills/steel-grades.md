---
name: steel-detailer-us-steel-grades
description: Use for US/AISC steel material-grade questions — which ASTM grade for W-shapes, HSS, plates, anchor rods, bolts and weld electrodes, and their Fy/Fu. AISC 360-22 Table A3.1 + ASTM designations.
---

# Steel grades (AISC) — which grade, and Fy / Fu

| Element | Preferred grade | Fy (ksi) | Fu (ksi) |
|---|---|---|---|
| Wide-flange **W**-shapes | **ASTM A992** | 50 | 65 |
| **HP** bearing piles; **plates / bars** | A572 Gr. 50 (plates also A36) | 50 | 65 |
| **S, M shapes; channels C / MC** | channels now commonly **A992**; legacy A36 | (A36) 36 | 58 |
| **Angles (L)** | A36 (transitioning to 50-ksi grades) | 36 | 58 |
| Weathering (exposed) | A588 | 50 | 70 † |
| **HSS** rectangular / square | **A500 Gr. C** | 50 | 62 |
| **HSS** round | A500 Gr. C | 46 | 62 |
| Premium HSS (tight tolerance) | A1085 | 50 | 65 † |
| **Pipe** (round) | A53 Gr. B | 35 | 60 |
| **Anchor rods** | F1554 (Gr. 36 / 55 / 105 = Fy) | 36 / 55 / 105 | — |
| **Bolts** | F3125 — A325 → "Group 120"; A490 → "Group 150" | — | 120 / 150 tensile |
| **Weld electrodes** | E70XX | — | FEXX = 70 |

360-22 updated several preferences vs older editions: **channels → A992**, **plates → A572-50**, **HSS → A500 Grade C** (was Grade B: rect 46/58, round 42/58). A992 caps the yield-to-tensile ratio at 0.85 for ductility.

## Source

- **AISC 360-22 Table A3.1** (approved material specifications) + the AISC "Basic Design Values" card + AISC SteelWise *"Are You Properly Specifying Materials?"* (Modern Steel Construction, June 2022 — the 360-22 companion). Fy/Fu come from these **free** AISC sources; the ASTM spec **text** (A992, A572, A36, A588, A500, A1085, A53, F1554, F3125) is paywalled — the designations + AISC-listed Fy/Fu are public. **†** A588 Fu = 70 and A1085 Fu = 65 are supplier/companion-corroborated, not read from an AISC table. Verify free at aisc.org.
