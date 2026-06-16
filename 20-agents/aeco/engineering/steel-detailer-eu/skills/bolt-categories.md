---
name: steel-detailer-eu-bolt-categories
description: Use for Eurocode bolt joint-category questions — shear connections (Categories A, B, C) and tension connections (Categories D, E), their preloading requirements, and which resistance check governs. EN 1993-1-8:2005 §3.4.
---

# Bolt categories — shear and tension connections (EN 1993-1-8:2005)

## Shear connections — EN 1993-1-8:2005 §3.4.1

| Category | Type | Requirement |
|---|---|---|
| **A** | Bearing type | Non-preloaded (or preloaded for fit); shear governs via Fv,Rd and bearing Fb,Rd. No slip criterion. Simplest category. |
| **B** | Slip-critical at SLS | Preloaded (8.8 or 10.9); slip resistance Fs,Rd,ser must not be exceeded at serviceability limit state. Shear/bearing checked at ULS as Category A. |
| **C** | Slip-critical at ULS | Preloaded (8.8 or 10.9); slip resistance Fs,Rd must not be exceeded at ULS. Net-section resistance Fnet,Rd also checked. |

Selecting B or C requires **preloaded bolts** (BS EN 14399 grade 8.8 or 10.9) and a
controlled tightening method (torque, combined, or HRC/DTI). Category A allows
non-preloaded bolts (e.g. grade 4.6 or 8.8 non-preloaded, BS EN 15048).

## Tension connections — EN 1993-1-8:2005 §3.4.2

| Category | Type | Requirement |
|---|---|---|
| **D** | Non-preloaded tension | Bolt in pure tension; Ft,Rd governs. Grade 4.6 or 8.8 acceptable. |
| **E** | Preloaded tension | Preloaded bolt in tension; bolt tension + prying governed by Ft,Rd per EN 1993-1-8. |

Combined shear + tension (categories B/E or C/E overlap): interaction check required
(EN 1993-1-8 Table 3.4 interaction formula).

## Bolt grades and standards

- Non-preloaded (Cat A, D): **4.6** or **8.8** per BS EN 15048 (non-preloaded assemblies)
- Preloaded (Cat B, C, E): **8.8** or **10.9** per BS EN 14399 (preloaded assemblies —
  System HR or HRC for torque-controlled preloading)

## Source

- **EN 1993-1-8:2005 §3.4.1 (shear) and §3.4.2 (tension)**, reproduced in:
  - **JRC EUR 27346** Section 1 (bolt category summary table)
  - **eurocodeapplied.com** bolt categories guide
  - **steelconstruction.info** "Simple connections" articles
