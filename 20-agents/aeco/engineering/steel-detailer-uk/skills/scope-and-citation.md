---
name: steel-detailer-uk-scope-and-citation
description: Reference for what the steel-detailer-uk agent covers, the free UK/Eurocode sources it draws from, how citations are formatted, and what is out of scope (and where to point the user). Apply when scoping a UK/Eurocode connection question or formatting a citation.
---

# UK/Eurocode advisor — scope, sources & citation format

## Sources

- **SCI/BCSA P358 "Joints in Steel Construction: Simple Joints to Eurocode 3"** — FREE PDF
  (steelconstruction.info). The primary source: reproduces EN 1993-1-8 + the UK NA for
  simple connections (resistances, preload/slip, welds, block tearing, NA partial factors).
- **steelconstruction.info** (the free SCI/BCSA/Tata wiki) and **eurocodeapplied.com** —
  used to corroborate the **EN 1993-1-8 Table 3.3** spacing/edge geometry that P358 cites
  but does not tabulate. Both cite the EN clauses.
- The underlying **BS EN 1993-1-8:2005** and the **UK National Annex** are BSI-paywalled —
  we cite them *via* the free sources that reproduce them, never copy them.
- (SCI P398 moment joints and P363 / eBlueBook capacities, both free, are for later phases.)

## In scope (v1 — connection detailing)

- Bolted: spacing & edge/end distances (EN Table 3.3 + P358 practice), hole clearances &
  types, categories A–E, shear & bearing resistance, preload & slip resistance.
- Welded: fillet weld throat/leg, the directional method, the throat-as-fraction-of-thickness
  detailing rules.
- Block tearing; and the **UK National Annex partial factors**.

## Citation format

`SCI P358 §X / Check Y / Table Z` and/or `BS EN 1993-1-8:2005 cl. … / Table 3.3` (note
"via steelconstruction.info / eurocodeapplied.com" where the value came from a free
secondary source). Always state the edition (2005) and that it is the **UK NA**.

## Out of scope — name the right source, don't guess

- **US practice** → `steel-detailer-aisc` (different code).
- **Other national annexes** (DE/FR/…) → not UK; the NDPs differ.
- **Fabrication / execution, tolerances, hole-making, weld procedure** → BS EN 1090-2 (paywalled).
- **Bolt assemblies / specs** → BS EN 14399 (preloaded), BS EN 15048 (non-preloaded).
- **Member / section design** (not connections) → EN 1993-1-1 / the Blue Book (P363).

## Copyright posture

Store and relay the *rule + citation + a short quote* — never reproduce whole tables or long
passages. Point users to the free P358 / steelconstruction.info to read the full clause.
