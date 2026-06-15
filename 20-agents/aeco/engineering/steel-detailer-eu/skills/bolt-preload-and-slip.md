---
name: steel-detailer-eu-bolt-preload-and-slip
description: Use for Eurocode bolt preload and slip resistance questions — the preload force Fp,C, slip resistance Fs,Rd at ULS and SLS, the ks hole factor and the μ slip-factor classes. EN 1993-1-8:2005 §3.9 + EN 1090-2 Table 18.
---

# Bolt preload & slip resistance (EN 1993-1-8:2005)

## Preload force — EN 1993-1-8:2005 §3.9.1(1)

> **Fp,C = 0.7 × fub × As**

- **fub** = ultimate tensile strength of the bolt (800 N/mm² for 8.8; 1000 N/mm² for 10.9)
- **As** = tensile stress area of the bolt

Example: M20 8.8 bolt — As = 245 mm², fub = 800 → Fp,C = 0.7 × 800 × 245 = **137 kN**.

This formula is fixed by EN 1993-1-8 §3.9.1; it is NOT an NDP. Tightening methods
(torque, combined, HRC) that achieve Fp,C are specified by EN 1090-2.

## Slip resistance — EN 1993-1-8:2005 §3.9.1

> **Fs,Rd = ks × n × μ × Fp,C / γM3** (ULS — Category C)

> **Fs,Rd,ser = ks × n × μ × Fp,C / γM3,ser** (SLS — Category B serviceability)

- **γM3 = 1.25** (ULS); **γM3,ser = 1.10** (SLS) — ⚠ both are NDPs; verify with country NA
- **n** = number of friction interfaces per bolt
- **ks** = factor for hole type (see table below)
- **μ** = characteristic slip factor for the friction surface class (see table below)

### ks — hole factor (EN 1993-1-8:2005 Table 3.6)

| Hole type | ks |
|---|---|
| Standard clearance holes | **1.00** |
| Oversized holes or short slotted (perpendicular to load) | **0.85** |
| Long slotted holes (perpendicular to load) | **0.70** |
| Short slotted (parallel to load) | **0.76** |
| Long slotted (parallel to load) | **0.63** |

### μ — slip factor classes (EN 1090-2:2018 Table 18)

| Class | μ (characteristic) | Surface treatment |
|---|---|---|
| **A** | **0.50** | Surfaces blasted (shot or grit) to Sa 2½; or hot-dip galvanised and wire-brushed |
| **B** | **0.40** | Surfaces blasted (shot or grit), with alkali-zinc silicate spray; or machine flame-cut and brushed |
| **C** | **0.30** | Surfaces cleaned by wire brushing or flame cleaning, loose scale removed |
| **D** | **0.20** | Surfaces as-rolled (no special treatment) |

The μ values are EN 1090-2 fixed values, not NDPs in EN 1993-1-8. However, execution
requirements for achieving Class A or B are set by EN 1090-2 and may have NA supplements.

## Source

- **EN 1993-1-8:2005 §3.9.1 (Fp,C formula and slip resistance)** + **Table 3.6 (ks)**,
  via **JRC EUR 27346** Design Example 1.4 (preloaded bolted connection).
- **EN 1090-2:2018 Table 18** (μ slip factors), via **steelconstruction.info**
  "Preloaded bolts" article (free, cite by ?oldid) and **eurocodeapplied.com**.
