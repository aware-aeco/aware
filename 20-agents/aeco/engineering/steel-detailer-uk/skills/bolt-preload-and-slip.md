---
name: steel-detailer-uk-bolt-preload-and-slip
description: Use for UK/Eurocode preloaded-bolt and slip-resistance questions — the design preload Fp,C, slip resistance Fs,Rd, the ks / n / μ factors, slip-factor surface classes, and γM3. SCI P358 implementing BS EN 1993-1-8 cl. 3.9.
---

# Preloaded bolts & slip resistance (UK / Eurocode)

**Design preload:**  **Fp,C = 0.7 · fub · As**  (fub = bolt ultimate strength; As = tensile stress area). Preloaded assemblies are property class **8.8 or 10.9** (to BS EN 14399).

**Slip resistance of one preloaded bolt:**  **Fs,Rd = (ks · n · μ / γM3) · Fp,C**
- **ks** = 1.0 for fasteners in standard clearance holes (reduced for oversized / slotted holes — EN 1993-1-8 Table 3.6).
- **n** = number of friction (faying) surfaces.
- **μ** = slip factor, by faying-surface preparation (BS EN 1090-2 Table 18): **0.5** (blasted, loose rust removed, not pitted), **0.4** (blasted + alkali-zinc-silicate or metallised), **0.3** (wire-brushed / flame-cleaned), **0.2** (as-rolled) — commonly labelled Classes A / B / C / D respectively.
- **γM3 = 1.25** at ULS; **γM3,ser = 1.10** at serviceability (UK NA).

Use preloaded / slip-resistant bolts where slip must be prevented — Category B (no slip at serviceability) or Category C (no slip at ULS).

## Source

- **SCI P358 §6 Check 4** (slip resistance), implementing **BS EN 1993-1-8:2005 cl. 3.9.1** (Fs,Rd, Fp,C) and Table 3.6 (ks); slip factors per **BS EN 1090-2 Table 18**. γM3 = 1.25 / γM3,ser = 1.10 per the UK NA. Read from the genuine P358 PDF. (P358 gives μ by the EN 1090-2 numeric values; the A–D surface-class letters are the EN 1090-2 labels.)
