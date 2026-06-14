---
name: steel-detailer-aisc-bolt-edge-distance
description: Use for US/AISC bolt edge-distance / end-distance questions — minimum edge distance from a bolt hole to a part edge (by bolt diameter), the one-diameter absolute floor, and the maximum edge distance. AISC 360-22 Table J3.4.
---

# Bolt edge distance — minimum & maximum (AISC)

**Minimum edge distance** from the center of a **standard** hole to any edge — AISC 360-22 **Table J3.4** (one value per bolt diameter):

| Bolt dia. | Min edge dist. | | Bolt dia. (metric) | Min edge dist. |
|---|---|---|---|---|
| 1/2 in | 3/4 in | | M12 | 18 mm |
| 5/8 in | 7/8 in | | M16 | 22 mm |
| 3/4 in | 1 in | | M20 | 26 mm |
| 7/8 in | 1 1/8 in | | M22 | 28 mm |
| 1 in | 1 1/4 in | | M24 | 30 mm |
| 1 1/8 in | 1 1/2 in | | M27 | 34 mm |
| 1 1/4 in | 1 5/8 in | | M30 | 38 mm |
| over 1 1/4 in | **1.25·d** | | M36 | 46 mm |
| | | | over M36 | 1.25·d |

- **An edge distance less than one bolt diameter (1·d) is not permitted without the approval of the engineer of record.**
- For **oversized or slotted holes**, add the increment **C₂** from Table J3.5.
- These are the **single-column** (post-2010) values. Do **not** use the old two-column "sheared edge / rolled or gas-cut edge" set (where over-1¼-in → 1¾·d) — that is pre-360-10 and is widely mis-cited online as current.

**Maximum edge distance** = **12·t**, but **≤ 6 in. (150 mm)**, where *t* = thickness of the connected part. — AISC 360-22 §J3.6

## Source

- Minimum: **AISC 360-22 §J3.5, Table J3.4 / Table J3.4M.** *"The distance from the center of a standard hole to an edge of a connected part in any direction shall not be less than either the applicable value from Table J3.4 or Table J3.4M, or as required in Section J3.11."* Table footnote: *"edge distances less than one bolt diameter are not permitted without approval from the engineer of record."*
- Maximum: **AISC 360-22 §J3.6.**
- Verify free at aisc.org/standards. *(360-22 numbering; 360-16 numbered minimum edge distance §J3.4.)*
