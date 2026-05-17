# DSTV vs Peddimat — what changes between them

DSTV is the open industry standard (DIN 9404, AISC adopted). Peddimat is Peddinghaus's vendor extension on top — same overall structure, additional lines that the Peddinghaus controllers know how to interpret.

## DSTV structure

```
ST                       <- Start marker
piece_mark                  ← header block (IK)
order                       ← project / order ref
material grade
profile                     ← e.g. UB 305x165x40
length (mm)
weight (kg/m)
...
BO                       <- Hole block
  side  x  y  diameter
  ...
SI                       <- Sign / piece-mark block
  text  font  size  x  y
KO                       <- Notch block
  side  x_min  x_max  y_min  y_max
AK                       <- Outer contour
EN                       <- End marker
```

## What Peddimat adds

Peddimat keeps everything above and adds vendor blocks the Peddinghaus controllers need:

| Block | Meaning | Required by |
|---|---|---|
| `PE` | Piece-end mark (orientation indicator) | All machines |
| `FB` | Saw stop position + cut angle | Saw lines |
| `M1` | First-side machining operations | Saw + drill combo |
| `M2` | Second-side machining (after flip) | Saw + drill combo |
| `DC` | Drill-line controller params (feed, RPM) | Drill lines |
| `PL` | Plasma cut parameters (current, gas) | Plasma lines |
| `KP` | Cope geometry with controller-specific encoding | Coper lines |

The transport adds the right blocks per `machine-type`:

| `machine-type` | Emits |
|---|---|
| `saw` | DSTV + PE + FB + M1 |
| `drill` | DSTV + PE + DC |
| `plasma` | DSTV + PE + PL |
| `coper` | DSTV + PE + KP |
| `beam-line` | DSTV + PE + FB + M1 + M2 + DC |

## What gets lost in round-trip

DSTV → Peddimat is lossless (everything in DSTV maps cleanly + new blocks added).

Peddimat → DSTV **loses the vendor blocks**. The DSTV emitted is valid (any saw line accepts it) but won't carry forward the Peddinghaus-specific controller params. For the detailer this is fine: source-of-truth is the Tekla model, not the controller.

## Kerf

DSTV doesn't carry the kerf width — assumes "machine knows." Peddimat encodes it in the `FB` block. The `kerf-mm` input on `to-peddimat` is the source.

Common values:
- Bandsaw: 1.5–2.5 mm
- Coldsaw: 2.0–3.5 mm
- Plasma: 3.0–6.0 mm (heat-affected zone)

If unsure, ask the shop floor. The default `3.0` mm covers most bandsaw / coldsaw setups.
