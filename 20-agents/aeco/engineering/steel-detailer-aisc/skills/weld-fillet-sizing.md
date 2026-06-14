---
name: steel-detailer-aisc-weld-fillet-sizing
description: Use for US/AISC fillet weld sizing questions — minimum fillet weld size by material thickness, maximum fillet size along an edge, and effective throat. AISC 360-22 §J2.
---

# Fillet weld sizing (AISC)

**Minimum fillet weld size** (leg), by thickness of the **thinner** part joined — AISC 360-22 **Table J2.4**:

| Thickness of thinner part | Min. fillet leg |
|---|---|
| to 1/4 in. (6 mm) inclusive | 1/8 in. (3 mm) |
| over 1/4 in. to 1/2 in. (13 mm) | 3/16 in. (5 mm) |
| over 1/2 in. to 3/4 in. (19 mm) | 1/4 in. (6 mm) |
| over 3/4 in. (19 mm) | 5/16 in. (8 mm) |

(The minimum size need not exceed the thickness of the thinner part; single-pass.)

**Maximum fillet weld size along edges** of connected parts (§J2.2b):
- Edge of material **< 1/4 in. (6 mm)** thick → weld **≤ the material thickness**.
- Edge of material **≥ 1/4 in.** thick → weld **≤ thickness − 1/16 in. (2 mm)** (unless the weld is detailed on the drawings to be built out to full-throat thickness).

**Effective throat** of a fillet weld = the **shortest distance from the root to the face** of the diagrammatic weld; for an equal-leg fillet this is ≈ **0.707 × leg**. (§J2.2a)

## Source

- Min size: **AISC 360-22 Table J2.4**; max size along edges: **§J2.2b**; effective throat: **§J2.2a**.
- These weld provisions' wording is verified current for 360-22 via AISC's official "Comparison of ANSI/AISC 360-22 to 360-16" (the numeric limits are unchanged from 360-16). Verify free at aisc.org/standards.
