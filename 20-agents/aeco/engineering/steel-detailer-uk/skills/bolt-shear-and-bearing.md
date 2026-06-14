---
name: steel-detailer-uk-bolt-shear-and-bearing
description: Use for UK/Eurocode bolt shear and bearing resistance questions — Fv,Rd (shear) and Fb,Rd (bearing) formulas, the αv / αb / k1 factors, and γM2. SCI P358 implementing BS EN 1993-1-8 Table 3.4.
---

# Bolt shear & bearing resistance (UK / Eurocode)

**Shear resistance of one bolt:**  **Fv,Rd = αv · fub · A / γM2**
- αv = **0.6** for classes 4.6, 5.6, 8.8; αv = **0.5** for class 10.9 (and 6.8).
- A = tensile stress area **As** if the threads are in the shear plane (P358's standard assumption); the shank area A if the shank is in the shear plane.
- γM2 = 1.25 (UK NA). fub = bolt ultimate strength (800 N/mm² for 8.8, 1000 for 10.9).

**Bearing resistance of one bolt:**  **Fb,Rd = k1 · αb · fu · d · t / γM2**
- **αb** (end bolt) = min( e1/(3·d0) ; p1/(3·d0) − 1/4 ; fub/fu ; 1.0 ). For an *inner* bolt, omit the e1/(3·d0) term.
- **k1** (edge bolt) = min( 2.8·e2/d0 − 1.7 ; 1.4·p2/d0 − 1.7 ; 2.5 ). For an *inner* bolt, use the 1.4·p2/d0 − 1.7 and 2.5 terms.
- fu = ultimate strength of the connected **plate**; d = bolt diameter; t = plate thickness; γM2 = 1.25.

## Source

- **SCI P358 §4 Check 8** (shear & bearing resistance), implementing **BS EN 1993-1-8:2005 Table 3.4** (αv, αb, k1) and Table 3.1 (fub). The formulas and factor terms were read from the genuine P358 PDF (rendered page). αv = 0.5 for class 10.9 is the EN 1993-1-8 Table 3.4 value (P358 standardises on class 8.8, αv = 0.6). γM2 = 1.25 per the UK NA.
