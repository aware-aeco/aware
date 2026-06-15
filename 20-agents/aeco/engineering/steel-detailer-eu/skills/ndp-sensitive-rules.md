---
name: steel-detailer-eu-ndp-sensitive-rules
description: Use when a user asks which EN 1993-1-8 rules vary by country, or when flagging a specific value as a Nationally Determined Parameter. Lists all NDP parameters in EN 1993-1-8:2005 relevant to connection detailing, the EN recommended value, and what may differ in a National Annex.
---

# NDP-sensitive rules in EN 1993-1-8:2005

Nationally Determined Parameters (NDPs) are clauses where the EN body text gives a
**recommended (boxed)** value but explicitly permits National Annexes to set a different
value. You MUST flag these whenever stating their value.

## Partial factors (the main NDPs — EN 1993-1-8 §6.1 + EN 1993-1-1 §6.1)

| Symbol | EN recommended | Applies to | Clause |
|---|---|---|---|
| **γM0** | **1.00** | Cross-section resistance of members | EN 1993-1-1 §6.1 |
| **γM1** | **1.00** | Member instability (buckling) | EN 1993-1-1 §6.1 |
| **γM2** | **1.25** | Resistance of bolts, welds, pins, plates in bearing | EN 1993-1-8 §6.1 |
| **γM3** | **1.25** | Slip resistance at ULS | EN 1993-1-8 §6.1 |
| **γM3,ser** | **1.10** | Slip resistance at SLS | EN 1993-1-8 §6.1 |

**Known NA deviations:**
- UK NA (to BS EN 1993-1-1): γM2 = **1.10** for net-section *member* fracture (not
  connector resistance) — a split not made in the EN recommended set.
- Most EU NAs adopt γM2 = 1.25 for connectors (same as EN recommended).
- Some countries modified γM3 / γM3,ser — always check the local NA before design.

## Connection geometry (EN 1993-1-8 Table 3.3) — NOT NDPs

The bolt spacing / edge-distance **minimums and maximums** in Table 3.3 are fixed
EN values, NOT NDPs. Countries do not set different values. Practical detailing
guides (SCI P358, DAST-012) may recommend tighter values for specific connection
types, but the code minima are uniform across all complying NAs.

## Weld design — partial NDP dependency

The weld design resistance formula (EN §4.5.3.2) is fixed. However, the computed
design strength varies by γM2, which IS an NDP. If a country adopts γM2 ≠ 1.25,
the fvw,d value changes. **Flag this whenever giving a tabulated design strength.**

The βw correlation factors (EN Table 4.1) and the fu values used with them are
fixed EN values (not NDPs).

## Slip factors μ (EN 1090-2 Table 18) — not EN 1993-1-8 NDPs

The characteristic slip factors for friction surfaces (Class A–D) are defined
in EN 1090-2, not EN 1993-1-8. They are NOT NDPs within EN 1093-1-8 — they are
fixed by EN 1090-2. However, individual NAs to EN 1090-2 may add execution
requirements that affect which friction class is achievable. The tabulated μ
values (0.50 / 0.40 / 0.30 / 0.20) are the EN 1090-2 characteristic values.

## Summary: what to check in the country NA before designing

1. **γM2** — will your country match the EN recommended 1.25 for connectors?
2. **γM3 / γM3,ser** — ULS 1.25 / SLS 1.10 confirmed?
3. **Execution classes** (EN 1090-2 NA): which EXC applies to your project?
4. **Seismic**: if EC8 applies, additional connection rules may supersede EN 1993-1-8.

## Source

- **EN 1993-1-8:2005 §6.1** (lists NDP partial factors for connections) via
  **JRC EUR 27346** p.13 and eurocodeapplied.com.
- **EN 1993-1-1 §6.1** (lists γM0 / γM1 / γM2 for member design) — same JRC source.
- NAs noted above sourced via steelconstruction.info (UK NA) and publicly available
  national EN publications.
