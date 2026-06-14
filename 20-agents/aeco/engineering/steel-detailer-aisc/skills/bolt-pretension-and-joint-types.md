---
name: steel-detailer-aisc-bolt-pretension-and-joint-types
description: Use for US/AISC bolted-joint type questions (snug-tightened vs pretensioned vs slip-critical), when each is required or permitted, and the minimum bolt pretension values. AISC 360-22 Table J3.1 / RCSC 2020 §4 + Table 5.2.
---

# Bolt joint types & minimum pretension (AISC / RCSC)

**Three joint types** (RCSC 2020 §4):
- **Snug-tightened** — the default; permitted except where §4.2 / §4.3 require otherwise.
- **Pretensioned** — required for: significant load reversal; fatigue load without reversal of direction; Group 120 assemblies in tensile fatigue; Group 144/150 assemblies in tension or combined shear + tension; or where the governing code/spec requires pretension.
- **Slip-critical** — required (shear or shear + tension) for: fatigue with reversal of loading direction; joints with **oversized holes**; joints with **slotted holes** (except where the load is approximately normal, 80–100°, to the slot); or where slip at the faying surfaces would be detrimental. Faying surfaces prepared per §3.2.2.

**Minimum bolt pretension** Tₘ (pretensioned & slip-critical joints) — AISC 360-22 Table J3.1 = RCSC 2020 Table 5.2; values are **0.70 × the specified minimum tensile strength**, rounded to the nearest kip:

| Bolt dia. (in) | Group 120 (e.g. A325) | Group 144 & 150 (e.g. A490) |
|---|---|---|
| 1/2 | 12 | 15 |
| 5/8 | 19 | 24 |
| 3/4 | 28 | 35 |
| 7/8 | 39 | 49 |
| 1 | 51 | 64 |
| 1 1/8 | 64 | 80 |
| 1 1/4 | 81 | 102 |
| 1 3/8 | 97 | 121 |
| 1 1/2 | 118 | 148 |

(kips. Group 144 = ASTM F3148 Grade 144; it takes the same pretension as Group 150.)

## Source

- Joint types: **RCSC 2020 §4** (and Table 4.1).
- Pretension: **RCSC 2020 Table 5.2** ( = AISC 360-22 Table J3.1). *"The specified minimum pretensions … are based on 70 percent of the tensile strength … rounded to the nearest kip."* **Note:** in RCSC 2020 the minimum-pretension table is **Table 5.2** (it was Table 8.1 in the 2014 edition; in 2020 Table 8.1 is the turn-of-nut rotation table). Verified against the genuine RCSC 2020 PDF.
- Verify free at boltcouncil.org and aisc.org/standards.
