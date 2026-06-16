---
name: steel-detailer-eu-bolt-shear-and-bearing
description: Use for Eurocode bolt shear and bearing resistance questions — Fv,Rd formula and αv coefficient, Fb,Rd formula with αb and k1 factors. EN 1993-1-8:2005 Table 3.4. Does NOT cover slip resistance (see bolt-preload-and-slip).
---

# Bolt shear & bearing resistance (EN 1993-1-8:2005)

## Shear resistance per shear plane — EN 1993-1-8:2005 §3.6.1, Table 3.4

> **Fv,Rd = αv × fub × A / γM2**

Where:
- **fub** = ultimate tensile strength of the bolt (N/mm²): 400 (4.6), 800 (8.8), 1000 (10.9)
- **A** = area used for shear:
  - Shear plane through the **threaded part**: use tensile stress area **As**
  - Shear plane through the **shank** (unthreaded): use gross area **Ag = π·d²/4**
- **αv** depends on bolt class and shear plane location:
  - αv = **0.6** — shear through **shank** (all bolt classes)
  - αv = **0.6** — shear through **threads**, bolt classes **8.8 or 10.9**
  - αv = **0.5** — shear through **threads**, bolt classes **4.6, 5.6 or 6.8**
- **γM2** = 1.25 (EN recommended; ⚠ NDP — verify with country NA)

**Practical note:** In simple connections, shear is commonly assumed to act through the
threaded portion. For 8.8 bolts (the most common preloaded grade), αv = 0.6 × As.

## Bearing resistance — EN 1993-1-8:2005 §3.6.1, Table 3.4

> **Fb,Rd = k1 × αb × fu × d × t / γM2**

Where:
- **fu** = ultimate strength of the connected plate (weaker part), in N/mm²
- **d** = nominal bolt diameter
- **t** = thickness of the plate in bearing

**αb** (direction parallel to load, end bolts):
> αb = min(**e1 / (3·d0)**, **fub / fu**, 1.0)

**αb** (direction parallel to load, inner bolts):
> αb = min(**p1 / (3·d0) − 1/4**, **fub / fu**, 1.0)

**k1** (direction perpendicular to load, edge bolts):
> k1 = min(**2.8·e2 / d0 − 1.7**, **2.5**)

**k1** (direction perpendicular to load, inner bolts):
> k1 = min(**1.4·p2 / d0 − 1.7**, **2.5**)

**Key point:** when e1 = e2 = 1.2·d0 (minimum), αb ≈ 0.4 and k1 ≈ 1.66 — bearing is
significantly reduced at minimum geometry. Increase e1, p1 to recover full bearing.

## Source

- **EN 1993-1-8:2005 §3.6.1 and Table 3.4**, via **JRC EUR 27346** Section 1
  (Design Example 1.1 worked shear/bearing calculations).
- **eurocodeapplied.com** bolt resistance calculator (free, cross-checks).
