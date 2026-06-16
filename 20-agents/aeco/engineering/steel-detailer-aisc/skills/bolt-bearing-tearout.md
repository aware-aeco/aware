---
name: steel-detailer-aisc-bolt-bearing-tearout
description: Use for US/AISC bolt bearing and tearout (bolt-hole) strength questions — the nominal-strength equations at bolt holes, the deformation-considered vs not cases, and the clear-distance (lc) basis. AISC 360-22 §J3.11.
---

# Bearing & tearout strength at bolt holes (AISC)

At each bolt hole the available strength is the **lesser of bearing and tearout**, checked for **both** bearing-type and slip-critical connections. **φ = 0.75 (LRFD), Ω = 2.00 (ASD).**

**Standard / oversized / short-slotted holes, or a long-slot parallel to the force** (§J3.11a(1)):
- Bearing — deformation at the hole at service load **is** a design consideration: **Rₙ = 2.4·d·t·Fu** (Eq. J3-6a)
- Bearing — deformation **not** a consideration: **Rₙ = 3.0·d·t·Fu** (Eq. J3-6b)
- Tearout — deformation a consideration: **Rₙ = 1.2·lc·t·Fu** (Eq. J3-6c)
- Tearout — not a consideration: **Rₙ = 1.5·lc·t·Fu** (Eq. J3-6d)

**Long-slotted hole, slot perpendicular to the force** (§J3.11a(2)): bearing **Rₙ = 2.0·d·t·Fu**; tearout **Rₙ = 1.0·lc·t·Fu**.

where **lc** = clear distance, in the direction of the force, between the edge of the hole and the edge of the adjacent hole or the edge of the material; *t* = ply thickness; *d* = bolt diameter; *Fu* = specified minimum tensile strength of the connected material.

## How to apply

- Tearout depends on **lc** (clear distance to the next hole or edge), so it is tied directly to your edge distance and spacing — a tight edge distance is usually governed by tearout, not bearing.
- Use the "deformation is a design consideration" forms (2.4 / 1.2) for normal service-load design unless elongation at the hole is explicitly acceptable.

## Source

- **AISC 360-22 §J3.11** (Equations J3-6a … J3-6f). *"… Rn = 2.4dtFu (J3-6a) … Rn = 3.0dtFu (J3-6b) … Rn = 1.2 lc t Fu (J3-6c) … Rn = 1.5 lc t Fu (J3-6d)."*
- Verify free at aisc.org/standards. *(360-22 numbering; 360-16 numbered this §J3.10.)*
