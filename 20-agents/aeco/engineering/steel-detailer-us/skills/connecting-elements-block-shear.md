---
name: steel-detailer-us-connecting-elements-block-shear
description: Use for US/AISC connecting-element strength questions — tensile yielding/rupture, shear yielding/rupture, and block shear rupture of gusset plates, gussets, shear tabs, angles and other connecting elements. AISC 360-22 §J4.
---

# Connecting-element & block-shear strength (AISC)

Strength of affected / connecting elements (gussets, plates, angles, shear tabs) — AISC 360-22 §J4:

| Limit state | Nominal strength | φ (LRFD) / Ω (ASD) | Clause |
|---|---|---|---|
| Tensile yielding | Rₙ = Fy·Ag | 0.90 / 1.67 | §J4.1(a) |
| Tensile rupture | Rₙ = Fu·Ae | 0.75 / 2.00 | §J4.1(b) |
| Shear yielding | Rₙ = 0.60·Fy·Agv | 1.00 / 1.50 | §J4.2(a) |
| Shear rupture | Rₙ = 0.60·Fu·Anv | 0.75 / 2.00 | §J4.2(b) |

**Block shear rupture** (§J4.3, Eq. J4-5):

> **Rₙ = 0.60·Fu·Anv + Ubs·Fu·Ant ≤ 0.60·Fy·Agv + Ubs·Fu·Ant**  ·  φ = 0.75, Ω = 2.00

- **Ubs = 1.0** where the tension stress is **uniform** (most gusset plates and angles).
- **Ubs = 0.5** where the tension stress is **nonuniform** (e.g. a coped beam web with a single bolt line).
- Agv / Anv = gross / net area subject to shear; Ant = net area subject to tension.

## Source

- **AISC 360-22 §J4.1** (tension), **§J4.2** (shear), **§J4.3** (block shear, Eq. J4-5). *"Rn = 0.60Fu Anv + Ubs Fu Ant ≤ 0.60Fy Agv + Ubs Fu Ant (J4-5) … Where the tension stress is uniform, Ubs = 1; where the tension stress is nonuniform, Ubs = 0.5."*
- §J4.1–§J4.3 are unchanged in 360-22 per AISC's official 360-22↔360-16 comparison. Verify free at aisc.org/standards.
