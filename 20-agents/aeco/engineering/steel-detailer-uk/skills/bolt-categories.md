---
name: steel-detailer-uk-bolt-categories
description: Use for UK/Eurocode bolted-connection category questions — the five categories A/B/C (shear) and D/E (tension) of EN 1993-1-8 Table 3.2 and when each applies. BS EN 1993-1-8:2005.
---

# Bolt connection categories (Eurocode 3, Table 3.2)

**Shear connections:**
- **Category A — Bearing type:** non-preloaded; bolt classes 4.6 up to and including 10.9; no slip check. Verify shear Fv,Ed ≤ Fv,Rd and bearing Fv,Ed ≤ Fb,Rd at ULS.
- **Category B — Slip-resistant at serviceability:** preloaded (8.8 / 10.9); no slip at the serviceability limit state (Fv,Ed,ser ≤ Fs,Rd,ser); also checked for shear & bearing at ULS.
- **Category C — Slip-resistant at ultimate:** preloaded (8.8 / 10.9); no slip at ULS (Fv,Ed ≤ Fs,Rd); also check bearing and net-section resistance Nnet,Rd.

**Tension connections:**
- **Category D — Non-preloaded:** classes 4.6 up to and including 10.9; no preload required; check tension Ft,Ed ≤ Ft,Rd and punching Bp,Rd.
- **Category E — Preloaded:** preloaded 8.8 / 10.9; check Ft,Rd and Bp,Rd.

(Where preload is required, the design preload is **Fp,C = 0.7·fub·As** — see `bolt-preload-and-slip`.)

## Source

- **BS EN 1993-1-8:2005 Table 3.2 (§3.4)** — categories and criteria corroborated on free steelconstruction.info + worked-example sources. P358 uses categories B / C / E operationally (it does not separately label A / D). UK NA applies.
