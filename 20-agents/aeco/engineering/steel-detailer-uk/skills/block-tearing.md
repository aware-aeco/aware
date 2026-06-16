---
name: steel-detailer-uk-block-tearing
description: Use for UK/Eurocode block tearing (block shear) questions — the concentric (Veff,1,Rd) and eccentric (Veff,2,Rd) resistances of a bolt-group block. SCI P358 implementing BS EN 1993-1-8 §3.10.2.
---

# Block tearing (UK / Eurocode)

Resistance to tearing-out of a bolt-group block (a tension failure path + a shear failure path).

- **Concentric loading** (symmetric bolt group, e.g. tying) — EN 1993-1-8 **Veff,1,Rd**:
  > Veff,1,Rd = fu·Ant / γM2 + (1/√3)·fy·Anv / γM0
- **Eccentric loading** (e.g. a fin plate in vertical shear) — EN 1993-1-8 **Veff,2,Rd**:
  > Veff,2,Rd = 0.5·fu·Ant / γM2 + (1/√3)·fy·Anv / γM0

where **Ant** = net area subject to tension, **Anv** = net area subject to shear; γM2 = 1.25, γM0 = 1.0 (UK NA). The companion net-section tension check: **FRd,n = 0.9·Anet·fu / γM2**.

## Source

- **SCI P358** §4 Check 9 (end plate / fin plate in shear) and the structural-integrity tying checks, implementing **BS EN 1993-1-8:2005 §3.10.2**. The concentric (full fu·Ant) and eccentric (0.5·fu·Ant) forms were read from the genuine P358 PDF. (P358 prints these as VRd,b / FRd,b rather than the EN symbols Veff,1,Rd / Veff,2,Rd.) UK NA partial factors apply.
