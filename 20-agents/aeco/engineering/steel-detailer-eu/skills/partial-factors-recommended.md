---
name: steel-detailer-eu-partial-factors-recommended
description: Use for Eurocode partial-factor (γM) questions in steel connection design — γM0, γM1, γM2, γM3 EN recommended (boxed) values per EN 1993-1-8:2005 §6.1 and EN 1993-1-1 §6.1. ALL are NDPs — flag every value with the NDP warning.
---

# Partial factors — EN 1993-1-8:2005 recommended values

**⚠ All partial factors are NDPs.** The values below are the EN 1993-1-8:2005 and
EN 1993-1-1:2005 **recommended (boxed) values**. Every National Annex may set
different values. Always confirm with the relevant NA before using in a design.

| Factor | EN recommended | Applies to | Source clause |
|---|---|---|---|
| γM0 | **1.00** | Cross-section resistance of members (gross section, bearing) | EN 1993-1-1 §6.1 |
| γM1 | **1.00** | Member resistance to instability (buckling, LTB) | EN 1993-1-1 §6.1 |
| γM2 | **1.25** | Resistance of **bolts, welds, pins, plates in bearing** | EN 1993-1-8 §6.1 |
| γM3 | **1.25** | Slip resistance at **ULS** (Category B at ULS, Category C) | EN 1993-1-8 §6.1 |
| γM3,ser | **1.10** | Slip resistance at **SLS** (Category B serviceability) | EN 1993-1-8 §6.1 |

## Known NA differences (non-exhaustive)

| Country | γM2 (connectors) | γM3 | γM3,ser | Notes |
|---|---|---|---|---|
| **UK NA** | 1.25 | 1.25 | 1.10 | Same as EN recommended for connectors; UK NA to EN 1993-1-1 splits γM2 = 1.10 for *member* net-section fracture |
| **Germany** (DIN EN NA) | 1.25 | 1.25 | 1.10 | Same as EN recommended |
| **Finland** | 1.25 | 1.25 | 1.10 | Same |
| Other countries | **verify** | **verify** | **verify** | Always check the local NA |

## Dual γM2 trap (UK-specific)

In the UK NA to BS EN 1993-1-1, γM2 = 1.10 applies to a *member's* net cross-section
fracture (a member check using fu), while γM2 = 1.25 applies to connector checks. The
EN recommended set does not make this distinction — it uses 1.25 for both.
**The EU agent uses 1.25 uniformly for all connection checks** (the EN recommended value).

## How partial factors appear in design checks

- **Bolt shear:** Fv,Rd = αv × fub × A / **γM2**
- **Bolt bearing:** Fb,Rd = k1 × αb × fu × d × t / **γM2**
- **Weld design strength:** fvw,d = (fu / √3) / (βw × **γM2**)
- **Slip resistance:** Fs,Rd = ks × n × μ × Fp,C / **γM3** (ULS); / **γM3,ser** (SLS)

## Source

- **EN 1993-1-8:2005 §6.1** (γM2, γM3, γM3,ser recommended values), via **JRC EUR 27346** p.13.
- **EN 1993-1-1:2005 §6.1** (γM0, γM1 recommended values), via **eurocodeapplied.com**.
- UK NA: **steelconstruction.info** "Partial factors for resistance" (oldid=15373).
