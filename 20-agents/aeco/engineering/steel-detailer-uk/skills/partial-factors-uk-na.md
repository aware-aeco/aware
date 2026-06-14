---
name: steel-detailer-uk-partial-factors-uk-na
description: Use for UK National Annex partial-factor (γM) questions in steel connection design — γM0, γM1, γM2, γM3 values per the UK NA to BS EN 1993. SCI P358.
---

# UK National Annex partial factors (γM)

| Factor | Value (UK NA) | Applies to |
|---|---|---|
| γM0 | **1.0** | cross-section resistance (UK NA to BS EN 1993-1-1) |
| γM1 | **1.0** | member resistance to instability (UK NA to BS EN 1993-1-1) |
| γM2 | **1.25** | resistance of **bolts, welds, pins, plates in bearing** (UK NA to BS EN 1993-1-8) |
| γM2 | **1.1** | net-section **fracture** of a member in tension (UK NA to BS EN 1993-1-1; used with fu) |
| γM3 | **1.25** | slip resistance at ULS |
| γM3,ser | **1.10** | slip resistance at serviceability |
| γMu | **1.1** | structural-integrity (tying) checks (P358 recommendation; no EN value) |

**Watch the dual γM2:** use **1.25** for all bolt / weld connection-component checks; the **1.1** value applies only to a member's net-section tension fracture (a member check, not a connector check).

## Source

- **SCI P358 §1.6 ("Major symbols")**, citing the **UK National Annex to BS EN 1993-1-1** (γM0, γM1, γM2 = 1.1) and the **UK National Annex to BS EN 1993-1-8** (γM2 = 1.25, γM3 = 1.25, γM3,ser = 1.10). Read verbatim from the genuine P358 PDF.
