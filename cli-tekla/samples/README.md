# Captured samples (live Tekla)

Real output of `profile-catalog` (with `parametric: true`) and `section-measure`, captured from a
live Tekla Structures 2026.0 on 2026-11-10 for aware-aeco/aware#799 and #800. They are evidence and
reference data for callers, not test fixtures.

| Environment | Model | profile-catalog | section-measure |
|---|---|---|---|
| `default` | Feasibility | `profile-catalog-parametric.tekla-2026.0-default.json` (284 items, 244 shapes) | `section-measure.tekla-2026.0-default.json` |
| `uk` | FloLess UK (empty) | `profile-catalog-parametric.tekla-2026.0-uk.json` (280 items, 197 shapes) | `section-measure.tekla-2026.0-uk.json` |
| `usa` | FloLess US (empty) | `profile-catalog-parametric.tekla-2026.0-usa.json` (272 items, 218 shapes) | `section-measure.tekla-2026.0-usa.json` |

`section-measure.request.json` is the request used for every environment: PL20*200, HI400-10-15*200,
L100*80*8, T200-10-15-100, U200*100*5, C200*100*5 and Z100*80*5, each under 4 rotations x 3 depth
positions (91 temporary members per run, all deleted).

The sections and placements are identical in all three environments for all seven names, and so are
the built-in shapes' name orders and parameters (33 built-in shapes each). The environments differ only
in alias prefixes: `uk` adds, for example, `WPB` and `FABA` to HI/WI, `CMF-C` to C and `GLASS`/`GRTG`
to the plate shape; `usa` adds `BAR`, `SQBR`, `CHKD_PLT`, `FLOOR_PLT` and others to the plate shape,
`RB`, `AB`, `CABLE` and `STUD` to the round bar, and `CCL` to L. Neither has `CUT`. Profile strings
through the Open API are read in millimetres in the `usa` environment too (`PL20*200` is 20 x 200 mm).

The `temporary_members.ledger` paths in the samples are the capturing machine's own and mean nothing
elsewhere. The `default` sample was captured before the last change to that block, so its
`temporary_members` carries the earlier field names (`removed_from_earlier_runs`,
`earlier_run_cleanup_failures`); the measurements are unaffected. The `uk` and `usa` samples have the shipped shape.
