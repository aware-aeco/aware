---
name: steel-detailer-us-scope-and-citation
description: Reference for what the steel-detailer-us agent covers, which authoritative US sources it draws from, how citations are formatted, and what is intentionally out of scope (and where to point the user instead). Apply when scoping an AISC connection question, formatting a citation, or deciding whether a question is answerable from this agent.
---

# AISC advisor — scope, sources & citation format

## Sources (free + authoritative)

- **AISC 360-22** — *Specification for Structural Steel Buildings* (ANSI/AISC 360-22).
  Chapter J ("Design of Connections") is the backbone. Free public PDF: `aisc.org/standards`.
- **RCSC 2020** — *Specification for Structural Joints Using High-Strength Bolts*
  (Research Council on Structural Connections). Free public PDF: `boltcouncil.org`.

Both are the legally-referenced US authorities and are citable to clause/table level.

## In scope (v1 — connection detailing)

- **Bolted connections:** minimum/maximum spacing, edge & end distance, hole types and
  nominal dimensions, pretension requirements, bearing/tearout limit states.
- **Welded connections:** minimum/maximum fillet weld size, effective throat, minimum
  length, PJP minimum effective throat.
- **Connecting elements:** block shear rupture and element strength at connections.

(Later phases broaden to drawing/detailing conventions, materials/sections, and fabrication.)

## Citation format

`AISC 360-22 §<clause>[, Table <n>]` or `RCSC 2020 §<clause>[, Table <n>]`. Always include the
edition. Where a skill stores a short source quote, include it so the user can verify against
the free PDF.

## Out of scope — name the right source, don't guess

- **Weld procedure / prequalified joint detail / weld-access-hole geometry** → AWS D1.1
  (paywalled). AISC 360-22 §J2 gives weld *strength* rules; the fabrication detail is AWS's.
- **Bolt material / grade specs** → ASTM F3125 (paywalled).
- **Ready-made connection design tables** → AISC *Steel Construction Manual* (paid). The
  underlying limit-state rules are in AISC 360-22 §J (free) — cite those.
- **Seismic connection demands** → AISC 341 / 358 (free; not in v1 scope yet).
- **UK / European practice** → `steel-detailer-uk` (UK NA — a different code entirely).

## Copyright posture

Store and relay the *rule + citation + a short quote* — never reproduce whole tables or long
passages. Point users to the free PDF to read the full clause.
