---
name: idea-rcs-cross-section-design
description: This skill should be used when composing snippets that work with IDEA StatiCa RCS (Reinforced Cross-Section) — for reinforced-concrete beam / column / slab design at the section level. Encodes the RCS workflow (define section geometry → assign reinforcement → set load combos → analyze ULS/SLS → consume code-check report), the difference from the Connection module (RCS is section-level, Connection is node-level), the supported codes (EN 1992 / ACI 318 / IS 456 / AS 3600), and the gotcha that crack-width checks (SLS) often dominate the design but require explicit load-case definition.
---

# IDEA StatiCa RCS cross-section design

IDEA StatiCa RCS handles the OTHER half of structural design: reinforced-concrete cross-sections. While the Connection module deals with steel-to-steel node behavior, RCS deals with beam / column / slab cross-section behavior under combined N / M / V actions.

## What RCS does

For a given cross-section (rectangular, T-shaped, circular, custom polygon) with reinforcement:

1. Computes ULS (Ultimate Limit State) capacity under N+M+V combined actions
2. Computes SLS (Serviceability Limit State): crack widths, stress limits, deflection contribution
3. Generates code-compliance report per the chosen design code

Output is per-component utilization (like Connection module): rebar capacity, concrete capacity, crack width, etc.

## Supported codes (RCS 26)

| Code | Use in |
|---|---|
| EN 1992-1-1 + NAs | EU + UK |
| ACI 318-22 | US + Canada |
| AS 3600 | Australia |
| IS 456 | India |
| SP 63.13330.2018 | Russia |

The code choice affects: partial factors, allowable crack widths, minimum reinforcement ratios, stress-strain curves. NAs (National Annexes) further modify EN — always specify the country.

## The workflow shape

```
1. Define cross-section geometry      (rect/T/circular/polygon + dimensions)
2. Specify concrete material           (C30/37, etc.)
3. Specify reinforcement layout        (bar count, size, position, cover)
4. Add stirrups / links (V check)
5. Specify load cases                  (per-case M + N + V values)
6. Specify load combinations           (per code: 1.35G + 1.5Q, etc.)
7. Run analysis                        (per-combination iteration)
8. Consume report                      (ULS pass/fail + SLS crack widths)
```

## RCS API endpoint shape

```
POST /api/rcs/create-section          { geometry, material, code }
POST /api/rcs/{id}/add-reinforcement  { bars, stirrups }
POST /api/rcs/{id}/add-load-cases     { cases: [...] }
POST /api/rcs/{id}/add-combinations   { combos: [...] }
POST /api/rcs/{id}/analyze
GET  /api/rcs/{id}/status
GET  /api/rcs/{id}/uls-utilization
GET  /api/rcs/{id}/sls-utilization
GET  /api/rcs/{id}/report?format=pdf
```

## ULS vs SLS

| Limit state | Checks | Typical governs |
|---|---|---|
| **ULS** (Ultimate) | Flexure / shear / axial capacity vs factored loads | Heavy beam → flexure |
| **SLS** (Serviceability) | Crack width, deflection, stress under unfactored loads | Slab → crack width / deflection |
| **Fatigue** | Stress range under repeated loading | Industrial floor; rare |
| **Fire** | Reduced strength + cover under elevated temp | Underground / refuge |

For most building projects, ULS bounds the design; SLS controls only in slabs and exposed elements.

**Crack-width SLS is the most common AWARE-flagged issue** for un-coordinated designs. Engineers often forget to specify the SLS load combination, then later get surprised when the section fails crack-width.

## Loading combinations

Each design code has its own combination conventions. EN 1990 Section 6.4.3:

```
Combination type     | Formula
---------------------|----------------------------------
Fundamental (ULS)    | 1.35 Gk + 1.5 Qk,1 + 1.5 sum(psi_0,i Qk,i)
Characteristic (SLS) | Gk + Qk,1 + sum(psi_0,i Qk,i)
Frequent (SLS)       | Gk + psi_1,1 Qk,1 + sum(psi_2,i Qk,i)
Quasi-permanent (SLS)| Gk + sum(psi_2,i Qk,i)
```

For a building beam, you typically specify:
- 1x Fundamental ULS combo → ULS check
- 1x Characteristic SLS combo → max stresses
- 1x Quasi-permanent SLS combo → long-term crack width

If you don't specify SLS combos, IDEA does NOT auto-derive them. The report shows "SLS not checked" — easy to miss in a hurry.

## Bar layout patterns

RCS's `add-reinforcement` accepts:

| Pattern | Use for |
|---|---|
| Rectangular grid | Beam, slab |
| Circular ring (perimeter) | Round column |
| Diagonal grid | Skew slab edges |
| Free-form polygon | Custom (e.g. T-junction confinement) |

For a typical AWARE workflow, the engineer authors the bar layout in the UI; AWARE scripts then RE-ANALYZE under varied load combinations (parametric study) without changing reinforcement.

## Common gotchas

- **The cross-section is ASSUMED PLANE.** Bending out-of-plane / warping torsion isn't handled by RCS — use IDEA's Beam module or a global FEM for that.
- **Reinforcement cover is to the BAR CENTER, not the bar surface.** A 25mm cover with 16mm bars puts the bar SURFACE at 17mm cover. Be precise.
- **Crack-width formula is per the chosen code's MAX rebar spacing rule.** Increasing rebar count usually REDUCES crack width (more bars = better stress distribution).
- **`uls-utilization` is in [0,1] for OK; >1 for FAIL.** SLS is similar but the "FAIL" threshold may be code-defined (e.g. crack width 0.3mm for exposed concrete).
- **Concrete creep + shrinkage matter for SLS deflection.** Specify them in the project config or accept IDEA defaults.

## See also

- `idea-connection-design-flow.md` — Connection module (node-level vs RCS section-level)
- `ideastatica-rcsapi-api.md` — auto-generated REST API reference
- `ideastatica-rcsapi-model.md` — auto-generated DTO reference
