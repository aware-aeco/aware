---
name: steel-detailer-us-weld-length-and-pjp
description: Use for US/AISC weld length and groove-weld questions — minimum fillet weld length, the long-weld length reduction factor, intermittent welds, and the minimum effective throat of partial-joint-penetration (PJP) groove welds. AISC 360-22 §J2.
---

# Fillet weld length & groove welds (AISC)

**Fillet weld length** (§J2.2b):
- **Minimum length** of a fillet weld designed for strength = **≥ 4 × the nominal weld size**; otherwise the effective size is taken as **¼ of its length**.
- **End-loaded** fillet welds: length ≤ 100w → effective length = actual; length > 100w → multiply by **β = 1.2 − 0.002(l/w) ≤ 1.0** (Eq. J2-1); length > 300w → effective length = **180w** (where *w* = leg size, *l* = length).
- **Intermittent** fillet weld segment: **≥ 4 × the weld size, minimum 1½ in. (38 mm)**.

**Groove welds** (§J2.1):
- **CJP** (complete-joint-penetration) effective throat = **thickness of the thinner part joined** (§J2.1a).
- **PJP** minimum effective throat — **Table J2.3**, by thickness of the thinner part joined:

| Thickness of thinner part | Min. effective throat |
|---|---|
| to 1/4 in. (6 mm) | 1/8 in. (3 mm) |
| over 1/4 to 1/2 in. (13 mm) | 3/16 in. (5 mm) |
| over 1/2 to 3/4 in. (19 mm) | 1/4 in. (6 mm) |
| over 3/4 to 1 1/2 in. (38 mm) | 5/16 in. (8 mm) |
| over 1 1/2 to 2 1/4 in. (57 mm) | 3/8 in. (10 mm) |
| over 2 1/4 to 6 in. (150 mm) | 1/2 in. (13 mm) |
| over 6 in. (150 mm) | 5/8 in. (16 mm) |

## Source

- Length / β / intermittent: **AISC 360-22 §J2.2b** (Eq. J2-1). Groove-weld throat: **§J2.1a**; PJP minimum effective throat: **Table J2.3**.
- Wording verified current for 360-22 via AISC's official 360-22↔360-16 comparison (numbers unchanged); Table J2.3 reproduced and labelled AISC 360-22 by IDEA StatiCa. Verify free at aisc.org/standards.
