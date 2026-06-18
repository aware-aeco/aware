---
name: steel-detailer-us-bolt-holes
description: Use for US/AISC bolt hole questions — the four hole types (standard, oversized, short-slotted, long-slotted), their nominal dimensions by bolt diameter, and where each hole type is permitted. AISC 360-22 Table J3.3 / RCSC 2020 Table 3.1.
---

# Bolt holes — types & nominal dimensions (AISC / RCSC)

**Nominal hole dimensions** — AISC Table J3.3 = RCSC Table 3.1 (identical), all in inches:

| Bolt dia. | Standard (dia.) | Oversized (dia.) | Short-slot (W × L) | Long-slot (W × L) |
|---|---|---|---|---|
| 1/2 | 9/16 | 5/8 | 9/16 × 11/16 | 9/16 × 1 1/4 |
| 5/8 | 11/16 | 13/16 | 11/16 × 7/8 | 11/16 × 1 9/16 |
| 3/4 | 13/16 | 15/16 | 13/16 × 1 | 13/16 × 1 7/8 |
| 7/8 | 15/16 | 1 1/16 | 15/16 × 1 1/8 | 15/16 × 2 3/16 |
| 1 | 1 1/8 | 1 1/4 | 1 1/8 × 1 5/16 | 1 1/8 × 2 1/2 |
| ≥ 1 1/8 | d + 1/8 | d + 5/16 | (d + 1/8) × (d + 3/8) | (d + 1/8) × 2.5·d |

The standard-hole increment is **+1/16 in for bolts up to 7/8 in**, and **+1/8 in for bolts 1 in and larger** — so a **1 in bolt takes a 1 1/8 in standard hole**; a 1 1/4 in bolt takes 1 3/8 in.

**Where each hole type is permitted** (RCSC 2020 §3.3):
- **Standard** — permitted in all plies of all joints.
- **Oversized** — permitted in slip-critical joints (any/all plies, with EOR approval); **not permitted in bearing-type (snug-tightened / pretensioned shear) joints.**
- **Short-slotted** — in snug-tightened/pretensioned joints: one ply at a faying surface, with the load ~perpendicular (80–100°) to the slot axis (more plies with EOR approval); in slip-critical: any/all plies with EOR approval.
- **Long-slotted** — with EOR approval; one ply at any faying surface only.

## Source

- Dimensions: **AISC 360-22 Table J3.3** ( = **RCSC 2020 Table 3.1**; AISC's engineering FAQ states Table J3.3 *"is based upon the RCSC Specification Table 3.1"*). Values verified against the genuine RCSC 2020 PDF (boltcouncil.org).
- Hole-type permissions: **RCSC 2020 §3.3.** *"Oversized holes are permitted in any or all plies of slip-critical connections, but they shall not be used in bearing-type connections."*
- Verify free at aisc.org/standards and boltcouncil.org.
