---
name: steel-detailer-eu-section-designations
description: Use for European steel section type and naming questions - IPE/HE/HEA/HEAA/HEB/HEM/IPN and channels (EN 10365), angles (EN 10056), hollow sections RHS/SHS/CHS (EN 10210 hot-finished vs EN 10219 cold-formed), the spelling variants seen on drawings versus in model catalogues, and when to refuse a guess. EN 10365 + EN 10056 + EN 10210/10219 via free secondary sources.
---

# Section types & naming (EU / EN)

Names the section families and the spellings you will meet. It gives **no section properties,
masses or dimensions** - take those from the manufacturer or standard tables, never from memory.

## Hot-rolled I / H sections and channels - EN 10365

Designation = **family letters + nominal height (mm)**, e.g. **IPE 200** (Wikipedia, *I-beam*:
"the letters are followed directly by the height of the beam's web"; IPE profiles are
"standardized under EN 10365").

| Family | What it is | Verified from |
|---|---|---|
| **IPE** | parallel-flange I section | ArcelorMittal Constructalia: "IPE - Steel sections with parallel flanges", IPE 80-750, EN 10365:2017 |
| **IPN** (older: **INP**) | I section with tapered flanges, 14% slope | Constructalia: "IPN - European standard sections", IPN 80-600, "14% flange slope", EN 10365:2017 |
| **HE** with series **A / B / M** (and **AA**) | wide-flange H sections; A = light, B = normal, M = heavy | Dlubal cross-section database lists "HE A, HE AA, HE B, HE M" under EN 10365:2017 (ArcelorMittal 2018) |
| **UPN** (older: **UNP**), **UPE** | channels; UPE = parallel flanges, UPN = tapered | ArcelorMittal literature lists UPE 80-400 and UPN 50-400 |

Series letter spelling varies: **HE 140 A = HEA140 = HEA 140** (same section).

**Not verified here - flag, do not state as fact:** that UPN and UPE are dimensioned in EN 10365 (the
source pages seen did not say so); the **UAP** channel family (ArcelorMittal name; no source seen);
the **tee** family (cut-from-I/H tees; no tee standard was verified). If asked, say so and send the
user to the current ArcelorMittal sections catalogue or EN 10365:2017.

## Angles - EN 10056

Hot-rolled equal and unequal leg angles. EN 10056-1 designation pattern: letter **L**, then leg
length(s) and leg thickness in mm - **L a x t** (equal) and **L a x b x t** (unequal), e.g.
**L 100 x 100 x 10**. Tolerances are in EN 10056-2 (not in -1). Sources: EN 10056-1:2017 scope and
designation clause as summarised by the iTeh / BSI / DIN catalogue pages.

## Hollow sections - EN 10210 (hot-finished) vs EN 10219 (cold-formed)

**RHS** rectangular, **SHS** square, **CHS** circular; e.g. **CHS 200 x 10** (outer diameter x wall,
Wikipedia, *Hollow structural section*). Dimensions are on the outside.

**The two routes are different products with different dimension tables.** steelconstruction.info
("Steel construction products"): "Dimensions for hollow sections are defined in BS EN 10210-2"
(hot-finished) and "BS EN 10219-2" (cold-formed); the process "affects residual stress in the
section" and one should not be substituted for the other "without checking the implications on
the design first".

- **Corner radii differ.** EN 10219-2 (cold-formed) external corner radius bands: T < 6 mm
  1.6T-2.4T; 6-10 mm 2.0T-3.0T; T > 10 mm 2.4T-3.6T; the internal corner profile "is not
  specified". EN 10210-2 radii were **not verified** - do not quote one.
- **Mass per metre is tabulated separately in each Part 2.** Do not reuse a mass or property
  from one route for the other.
- **A drawing that names only "RHS 200x100x5" without EN 10210 / EN 10219 (or Celsius/Hybox style
  product names, or the steel grade suffix) is ambiguous.** Say so and ask / raise an RFI; do not
  pick a route. A grade like S355J2H alone is available in both standards (see `steel-grades.md`).

## Spellings: drawing vs model catalogue

| On the drawing | In a model catalogue | Notes |
|---|---|---|
| HE 140 A | HEA140 | letter moves to the family |
| IPE 200 | IPE200 | |
| UPN 200 / UNP 200 | UPN200 | UNP is the same channel |
| RHS 200x100x5, RHS200*100*5 | e.g. Tekla NL profile **KK200/100/5** (cold-formed) / **KW200/100/5** (hot-finished) | KK/KW mapping is **unverified here**: no public source found. Confirm against the project's catalogue before relying on it. |
| RHS100*4 (two tokens) | square section, 100 x 100 x 4 | two numbers = SHS; **RHS** with two tokens denotes a square section |
| L100*8, L100x8 | L100/8 | equal angle 100 x 100 x 8 (single leg + thickness) |
| CHS 88.9x4 | N88.9/4 | diameter / wall |

The catalogue-spelling column (anything other than the standard-based forms) is project/software
convention, not EN-standard text: confirm in the model's catalogue. Never silently map a drawing
name to a catalogue name when two catalogue entries (e.g. hot vs cold route) could match - raise it.

## Source

- **EN 10365:2017** (I/H/channel dimensions), **EN 10056-1/-2** (angles), **EN 10210-2 / EN 10219-2**
  (hollow sections) - CEN-paywalled, cited by number only.
- ArcelorMittal Constructalia "I sections" (IPE/IPN), ArcelorMittal Sections & Merchant Bars catalogue;
  Dlubal cross-section properties database (HE A/AA/B/M under EN 10365:2017); steelconstruction.info
  "Steel construction products" (no oldid shown on the fetched page - cite by title);
  Wikipedia *I-beam* and *Hollow structural section*; iTeh/BSI/DIN catalogue summaries of
  EN 10056-1:2017 and EN 10219-2.
- Anything marked "not verified" above has no source in this skill: **refuse rather than guess**
  ("I don't have a verified source for that in the EU/EN recommended-value ruleset").
