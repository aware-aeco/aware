---
name: steel-detailer-eu-bolt-holes
description: Use for Eurocode bolt hole type and clearance questions — standard, oversized, and slotted hole clearances per EN 1090-2:2018 Table 11. EN 1090-2:2018 (fabrication/execution standard).
---

# Bolt holes — clearances & types (Eurocode / EN 1090-2)

Hole sizes and clearances are set by **EN 1090-2:2018 "Execution of steel structures"**,
not by EN 1993-1-8 (which governs resistance checks). These values are fixed by EN 1090-2;
they are not NDPs in EN 1993-1-8.

## Standard clearance holes — EN 1090-2:2018 Table 11

| Bolt diameter d | Clearance (d0 − d) |
|---|---|
| M12 or M14 | **+1 mm** |
| M16 to M24 | **+2 mm** |
| M27 and above | **+3 mm** |

Examples: M16 → d0 = 18 mm; M20 → d0 = 22 mm; M24 → d0 = 26 mm; M27 → d0 = 30 mm.

## Oversized holes — EN 1090-2:2018 Table 11

| Bolt diameter d | Oversize clearance |
|---|---|
| M12 | **+3 mm** |
| M14 to M22 | **+4 mm** |
| M24 | **+6 mm** |
| M27 and above | **+8 mm** |

## Slotted holes — EN 1090-2:2018 Table 11

**Short slotted** (length ≤ 1.5·d in long direction):
- Normal direction (d0): same as standard clearance
- Long direction: min(1.33·d0, d + 4 mm) typical — verify against EN 1090-2 Table 11

**Long slotted** (length up to 2.5·d in long direction): length ≤ 2.5·d.

## Effect on slip resistance (EN 1993-1-8 §3.9.1)

The hole type sets the **ks factor** in slip resistance (see `bolt-preload-and-slip`):
- Standard clearance holes: ks = **1.00**
- Oversized or short slotted (perp. to load): ks = **0.85**
- Long slotted (perp. to load): ks = **0.70**

## Source

- **EN 1090-2:2018 Table 11** (CEN, paywalled), reproduced values via:
  - **steelconstruction.info** bolt-hole article (free)
  - **eurocodeapplied.com** connection tools (free)
- **EN 1993-1-8:2005 §3.9.1 Table 3.6** (ks values), reproduced in **JRC EUR 27346** Table 1.3.
