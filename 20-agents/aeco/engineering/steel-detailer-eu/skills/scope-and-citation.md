---
name: steel-detailer-eu-scope-and-citation
description: Reference for what the steel-detailer-eu agent covers, the free EU/Eurocode sources it draws from, how citations are formatted, what is out of scope (and where to point the user), and the NDP posture. Apply when scoping a Eurocode connection question or formatting a citation.
---

# EU/Eurocode advisor — scope, sources & citation format

## Sources

- **JRC EUR 27346 "Design of Steel Structures — Part 1-8: Design of Joints" Worked Examples**
  — FREE (DOI: 10.2788/605700; Luxembourg: Publications Office of the European Union, 2015;
  JRC Science and Policy Reports EUR 27346 EN). Authoritative secondary source that reproduces
  EN 1993-1-8:2005 rules with worked examples and direct clause citations. The primary basis
  for this agent.
- **steelconstruction.info** (the free SCI/BCSA/Tata wiki) and **eurocodeapplied.com** —
  free corroborating sources that also reproduce EN clauses. Cite steelconstruction.info
  by its `?oldid=` permalink so the version is stable.
- The underlying **EN 1993-1-8:2005 "Design of joints"** is CEN/BSI-paywalled — cited by
  clause and table number, never reproduced verbatim in quantity.
- **EN 1090-2** (execution/fabrication, hole clearances): CEN-paywalled; hole clearance
  values sourced via JRC EUR 27346 and steelconstruction.info.
- **EN 1993-1-1** (material properties, Table 3.1): paywalled; values reproduced in JRC
  EUR 27346 and eurocodeapplied.com.

## In scope (v1 — EN recommended values)

- Bolted connections: spacing & edge/end distances (EN Table 3.3), hole clearances &
  types (EN 1090-2 Table 11), categories A–E, shear & bearing resistance formulae,
  preload & slip resistance.
- Welded connections: fillet weld throat/leg relationship, the directional method
  (EN §4.5.3.2), βw correlation factors (EN Table 4.1).
- Partial factors — the EN *recommended* (boxed) values: γM0, γM1, γM2, γM3, γM3,ser.
  All are NDPs; see `ndp-sensitive-rules.md`.
- Steel material properties: S235 / S275 / S355 / S420 / S460 fy and fu by thickness,
  from EN 1993-1-1 Table 3.1 as reproduced in JRC EUR 27346.

## Citation format

`EN 1993-1-8:2005 cl. X.X / Table X.X (JRC EUR 27346 p.NN)` and/or
`EN 1993-1-8:2005 via eurocodeapplied.com / steelconstruction.info (oldid=NNNN)`.
Always state the EN edition (2005). Always note when a value is the EN **recommended**
value (boxed) and that a National Annex may override it.

## NDP posture

Whenever giving an EN recommended value that is an NDP, add:

> *"⚠ NDP: This is the EN 1993-1-8:2005 recommended value. The actual value in your
> country is set by its National Annex and may differ. Check the relevant NA before
> designing — e.g. the UK NA sets γM2 = 1.25 (same here), but some countries differ."*

## Out of scope — name the right source, don't guess

- **Country-specific NA values** → use the country agent (e.g. `steel-detailer-uk`
  for UK NA via P358). This agent gives EN recommended values only.
- **US practice** → `steel-detailer-us` (different code entirely).
- **Member / section design** (Part 1-1, buckling etc.) → not connections.
- **Seismic connections** → EN 1998-1 / the dedicated country EN 8 agent.
- **Fatigue** → EN 1993-1-9.
- **Detailed weld procedure qualification** → EN ISO 15614-1.

## Copyright posture

Store and relay the *rule + citation + a short quote from the free source* — never
reproduce long passages. Point users to the free JRC EUR 27346 PDF or eurocodeapplied.com
to read the full clause.
