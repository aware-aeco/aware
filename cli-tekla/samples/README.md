# Captured samples (live Tekla)

Real output of `profile-catalog` (with `parametric: true`) and `section-measure`, captured from a
live Tekla Structures 2026.0 in its `default` environment (model `Feasibility`) on 2026-11-10, for
aware-aeco/aware#799 and #800. They are evidence and reference data for callers, not test fixtures.

- `profile-catalog-parametric.tekla-2026.0-default.json`: every parametric shape (284 Tekla items,
  244 shapes), plus the resolution of the seven names below.
- `section-measure.request.json`: PL20*200, HI400-10-15*200, L100*80*8, T200-10-15-100,
  U200*100*5, C200*100*5 and Z100*80*5, each under 4 rotations x 3 depth positions.
- `section-measure.tekla-2026.0-default.json`: its result (91 temporary members inserted and deleted).

Not captured: the UK and USA environments, which need Tekla restarted in that environment.
