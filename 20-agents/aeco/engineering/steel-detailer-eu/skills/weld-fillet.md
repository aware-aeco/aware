---
name: steel-detailer-eu-weld-fillet
description: Use for Eurocode fillet weld questions — throat/leg relationship, the directional method (design strength fvw,d), the βw correlation factor from EN 1993-1-8:2005 Table 4.1, and the minimum effective length rule. EN 1993-1-8:2005 §4.5.
---

# Fillet welds (Eurocode 3 — EN 1993-1-8:2005)

## Throat–leg relationship — EN 1993-1-8:2005 §4.5.2

> **a = 0.7 · s** (a = effective throat; s = leg length)

## Directional method — EN 1993-1-8:2005 §4.5.3.2, Eq. (4.1)

Resistance per unit length: **Fw,Rd = fvw,d · a**, where

> **fvw,d = (fu / √3) / (βw · γM2)**

- **fu** = characteristic ultimate tensile strength of the **weaker connected part** (N/mm²)
  — use EN 1993-1-1 Table 3.1 values (reproduced in JRC EUR 27346):
  - S235: **fu = 360 N/mm²**; S275: **fu = 430 N/mm²**; S355: **fu = 510 N/mm²**
  - S420: **fu = 520 N/mm²**; S460: **fu ≈ 540–550 N/mm²** (EN 10025-3/4 grade dependent)
- **βw** = correlation factor from EN 1993-1-8:2005 Table 4.1 (see below)
- **γM2 = 1.25** (EN recommended; ⚠ NDP — fvw,d changes if country NA adopts a different γM2)

### βw correlation factors — EN 1993-1-8:2005 Table 4.1

| Steel grade | fu (EN 1993-1-1 Table 3.1) | βw |
|---|---|---|
| S235 | 360 N/mm² | **0.80** |
| S275 | 430 N/mm² | **0.85** |
| S355 | 510 N/mm² | **0.90** |
| S420 (N/NL/M/ML) | 520 N/mm² | **1.00** |
| S460 (N/NL/M/ML/Q/QL/QL1) | ~540–550 N/mm² | **1.00** |

**Note — UK/P358 difference:** SCI P358 (UK) uses fu = 410 (S275) / 470 (S355) from
EN 10025-2 minimum, giving lower (more conservative) design strengths. The EN
Table 4.1 values above (430/510) are the nominal design values from EN 1993-1-1 Table 3.1.
When working to a UK NA, use P358 values and the `steel-detailer-uk` agent.

### Indicative design strengths fvw,d (at γM2 = 1.25, directional/longitudinal component)

| Grade | fvw,d (N/mm²) |
|---|---|
| S235 | (360/√3) / (0.80 × 1.25) = **208 N/mm²** |
| S275 | (430/√3) / (0.85 × 1.25) = **234 N/mm²** |
| S355 | (510/√3) / (0.90 × 1.25) = **262 N/mm²** |
| S420 | (520/√3) / (1.00 × 1.25) = **240 N/mm²** |
| S460 | (540/√3) / (1.00 × 1.25) = **249 N/mm²** |

S420/S460 fvw,d is lower than S355: the βw = 1.0 penalty for higher-strength steels
reflects weld efficiency. Match weld electrode to parent material grade.

## Minimum effective length — EN 1993-1-8:2005 §4.5.1

> **Minimum effective length ≥ max(6a, 30 mm)**

Short welds below this threshold must NOT be used in the effective length calculation.

## Simplified method — EN 1993-1-8:2005 §4.5.3.3

The simplified method uses fvw,d uniformly regardless of load direction (no directional
bonus). Yields slightly lower (more conservative) design resistance than the directional
method for transverse welds.

## Source

- **EN 1993-1-8:2005 §4.5.2 (throat), §4.5.3.2 (directional method, Eq. 4.1), Table 4.1
  (βw factors)**, via **JRC EUR 27346** Design Example 1.3 (welded connection checks) +
  **eurocodeapplied.com** fillet weld page (free corroboration).
- fu values from **EN 1993-1-1:2005 Table 3.1**, reproduced in JRC EUR 27346 Table 0.1.
- Minimum effective length: EN 1993-1-8:2005 §4.5.1 via JRC EUR 27346.
