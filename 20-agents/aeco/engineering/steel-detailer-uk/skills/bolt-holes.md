---
name: steel-detailer-uk-bolt-holes
description: Use for UK/Eurocode bolt hole questions — nominal hole clearances (standard d0 by bolt size), hole types (standard / oversized / slotted), and the slip-factor reduction for non-standard holes. EN 1993-1-8 §3.6.1 / EN 1090-2 Table 11 + SCI P358.
---

# Bolt holes — clearances & types (UK / Eurocode)

**Standard (normal) round-hole clearance** — d0 = d + clearance (EN 1090-2 Table 11 / EN 1993-1-8 §3.6.1):

| Bolt size | Clearance | d0 |
|---|---|---|
| M12, M14 | **+1 mm** | 13, 15 |
| M16, M18, M20, M22, M24 | **+2 mm** | 18, 20, 22, 24, 26 |
| ≥ M27 | **+3 mm** | 30, 33, … |

(The **M12 / M14 = +1 mm** case is the commonly mis-stated one — many assume +2 mm for all small bolts.)

**P358 practical convention:** d0 = d + 2 mm for d ≤ 24 mm, d + 3 mm for d > 24 mm; **holding-down (foundation) bolts d + 6 mm**. (P358 standardises on M20 / M24, so its "+2 mm for d ≤ 24" rolls up the EN +1/+2 split; for M12 / M14 use the EN +1 mm.)

**Oversized & slotted holes** *(single free source — verify against EN 1090-2 Table 11):* oversized ≈ d + 3 mm (M12) rising to d + 8 mm (≥ M27); short-slot ≈ d0 × (d + 4 mm); long-slot ≈ d0 × 2.5·d.

**Slip-factor reduction:** in preloaded / slip-resistant joints, ks < 1.0 for oversized or slotted holes (EN 1993-1-8 Table 3.6) — see `bolt-preload-and-slip`.

## Source

- Standard clearances: **EN 1993-1-8:2005 §3.6.1 / EN 1090-2 Table 11** — corroborated on free eurocodeapplied.com + SCI Guidance Note GN 5.08 (steelconstruction.info).
- P358 convention: **SCI P358 §2.2** ("d + 2 mm for d ≤ 24 mm; d + 3 mm for d > 24 mm; holding-down bolts d + 6 mm").
- Oversized / slotted exact dimensions: a single free source (eurocodeapplied.com) — treat as provisional and verify against EN 1090-2 Table 11.
