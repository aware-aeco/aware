---
name: steel-detailer-uk-weld-fillet
description: Use for UK/Eurocode fillet weld questions — throat/leg relationship, the directional method (design strength fvw,d), the βw correlation factor, and P358's throat-as-fraction-of-thickness detailing rules. SCI P358 implementing BS EN 1993-1-8 §4.5.
---

# Fillet welds (UK / Eurocode)

**Throat–leg:**  **a = 0.7 · s**  (a = effective throat, s = leg length).

**Directional method** (EN 1993-1-8 §4.5.3, Eq. 4.1): resistance per unit length **Fw,Rd = fvw,d · a**, where

> **fvw,d = (fu / √3) / (βw · γM2)**

- fu = ultimate strength of the weaker connected part: **410 N/mm² (S275)**, **470 N/mm² (S355)**.
- βw correlation factor: 0.8 (S235), **0.85 (S275)**, **0.9 (S355)**, 1.0 (S420 / S460).
- γM2 = 1.25 (UK NA).
- P358 tabulated design strengths (Table G.37): S275 → 223 (longitudinal) / 273 (transverse) N/mm²; S355 → 241 / 295 N/mm².

**Full-strength throat (P358 detailing)** — throat as a fraction of the connected web thickness tw: end-plate web welds **a ≥ 0.40·tw (S275) / 0.48·tw (S355)**; fin-plate (full-strength) welds **a ≥ 0.50·tw (S275) / 0.60·tw (S355)**.

## Source

- **SCI P358 §7 Check 4 + Appendix C**, implementing **BS EN 1993-1-8:2005 §4.5.3 (Eq. 4.1)**. a = 0.7·s, the directional method, βw = 0.85 / 0.9, fu = 410 / 470, and the throat-fraction rules were read from the genuine P358 PDF. The βw endpoints 0.8 (S235) and 1.0 (S420/S460) are the EN 1993-1-8 Table 4.1 values (P358 covers S275 / S355 only). Note: EN's "simplified method" and the minimum effective length ≥ max(30 mm, 6a) are EN 1993-1-8 §4.5.1 provisions not reproduced in P358 — cite EN directly if asked.
