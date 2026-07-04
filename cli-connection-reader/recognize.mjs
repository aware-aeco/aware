// recognize.mjs — fit a parametric connection recipe from tessellated mesh parts.
//
// Pure (no IFC / web-ifc / fs), so it is unit-testable in isolation and the SEA bundler folds it into
// the bridge. All fitted params are frame-independent scalars (mm): the consumer re-derives geometry from
// ITS OWN column, so the mesh's coordinate frame never leaks into the recipe. v1 recognizes a base plate;
// grow one supported type at a time.

// web-ifc emits mesh geometry in a fixed Y-UP frame (it always converts IFC's Z-up world to its own Y-up
// output), so "up" is always axis 1. A base plate is a HORIZONTAL plate with VERTICAL anchor bolts — the
// vertical-anchor test is THE discriminator from a vertical shear/fin plate (whose bolts are horizontal),
// which stops shear connections from being mis-read as base plates. If a web-ifc upgrade ever changed the
// output frame, this one constant is the single knob to turn.
export const VERTICAL = 1;

function median(a) {
  const s = [...a].sort((x, y) => x - y);
  const m = s.length >> 1;
  return s.length % 2 ? s[m] : (s[m - 1] + s[m]) / 2;
}
// AABB of a part's flat [x,y,z…] positions → {min,max,ext,ctr} per axis (mm).
export function partBox(p) {
  const min = [Infinity, Infinity, Infinity], max = [-Infinity, -Infinity, -Infinity];
  for (let i = 0; i + 2 < p.positions.length; i += 3) {
    for (let k = 0; k < 3; k++) {
      const v = p.positions[i + k];
      if (v < min[k]) min[k] = v;
      if (v > max[k]) max[k] = v;
    }
  }
  return {
    min, max,
    ext: [max[0] - min[0], max[1] - min[1], max[2] - min[2]],
    ctr: [(min[0] + max[0]) / 2, (min[1] + max[1]) / 2, (min[2] + max[2]) / 2],
  };
}
const argMax = (a) => (a[0] >= a[1] && a[0] >= a[2] ? 0 : a[1] >= a[2] ? 1 : 2);
const argMin = (a) => (a[0] <= a[1] && a[0] <= a[2] ? 0 : a[1] <= a[2] ? 1 : 2);
// Cluster 1-D values into groups within `tol`, returning each group's mean (the grid lines).
function cluster1d(vals, tol) {
  const out = [];
  for (const v of [...vals].sort((a, b) => a - b)) {
    const last = out[out.length - 1];
    if (last && Math.abs(v - last.mean) <= tol) { last.sum += v; last.n++; last.mean = last.sum / last.n; }
    else out.push({ sum: v, n: 1, mean: v });
  }
  return out.map((g) => g.mean);
}

// True horizontal centre + diameter of a tessellated cylindrical bolt. web-ifc polygonises a circular
// profile with vertices at slightly varying radii (its edges neither perfectly inscribe nor circumscribe
// the circle: an M24 anchor's AABB flat-to-flat under-reads to ~23.5, its circumradius over-reads to
// ~24.5), so the robust true radius is the MEAN vertex distance from the centroid (~24.0) — and it is
// convention-independent (no standard-size snapping, so imperial anchors fit too).
function boltCentreDia(p, a, c) {
  let sa = 0, sc = 0, n = 0;
  for (let i = 0; i + 2 < p.positions.length; i += 3) { sa += p.positions[i + a]; sc += p.positions[i + c]; n++; }
  const ca = sa / n, cc = sc / n;
  let sr = 0;
  for (let i = 0; i + 2 < p.positions.length; i += 3) sr += Math.hypot(p.positions[i + a] - ca, p.positions[i + c] - cc);
  return { ca, cc, dia: (2 * sr) / n }; // mean vertex radius × 2
}

// Recognize a base plate from tessellated parts: a horizontal plate with a VERTICAL anchor-bolt grid
// passing through it. Returns { kind:'base-plate', params, main } when confident, else null (the consumer
// then keeps it as opaque `custom` mesh — Slice B behaviour).
export function recognizeBasePlate(parts, members) {
  const bolts = parts.filter((p) => p.role === 'bolt').map((p) => ({ p, b: partBox(p) }));
  const plates = parts.filter((p) => p.role === 'plate').map((p) => ({ p, b: partBox(p) }));
  if (plates.length < 1 || bolts.length < 2) return null; // no anchor grid to fit → not recognizable

  // Every anchor's long axis must be vertical (a coherent vertical anchor group). A shear/fin plate's
  // bolts run horizontal → rejected here, never mis-read as a base plate.
  if (!bolts.every((x) => argMax(x.b.ext) === VERTICAL)) return null;
  const [a, c] = [0, 2]; // the two horizontal (in-plane) axes
  for (const x of bolts) x.m = boltCentreDia(x.p, a, c); // true circle centre + Ø (not the polygon AABB)

  // Base plate = a flat plate thin in the vertical axis (⟂ the anchors), largest horizontal footprint if
  // several (washer plates are smaller). A plate not ⟂ the anchors → not the base plate.
  const flats = plates.filter((x) => argMin(x.b.ext) === VERTICAL
    && x.b.ext[a] > 3 * x.b.ext[VERTICAL] && x.b.ext[c] > 3 * x.b.ext[VERTICAL]);
  if (!flats.length) return null;
  flats.sort((x, y) => (y.b.ext[a] * y.b.ext[c]) - (x.b.ext[a] * x.b.ext[c]));
  const plate = flats[0].b;

  // Every anchor must actually PIERCE this plate — its centroid inside the plate footprint AND its vertical
  // span overlapping the plate's thickness band. Without this, a flat plate plus an unrelated vertical bolt
  // grid (off to the side, or above/below) would still fit a bogus base-plate recipe; reject those to the
  // faithful mesh fallback.
  const overlapsV = (x) => x.b.min[VERTICAL] <= plate.max[VERTICAL] && x.b.max[VERTICAL] >= plate.min[VERTICAL];
  const insidePlate = (x) => Math.abs(x.m.ca - plate.ctr[a]) <= plate.ext[a] / 2
    && Math.abs(x.m.cc - plate.ctr[c]) <= plate.ext[c] / 2;
  if (!bolts.every((x) => overlapsV(x) && insidePlate(x))) return null;

  // Emitted dimensions are rounded to whole millimetres — these are FITTED, editable starting values (a
  // detailer specs whole-mm plates/anchors, not the tessellation's sub-mm noise); the consumer can adjust.
  const thickness = Math.round(plate.ext[VERTICAL]);
  const plateWidth = Math.round(plate.ext[a]);
  const plateDepth = Math.round(plate.ext[c]);

  // Anchor grid: cluster the bolt centroids on each horizontal axis → cols × rows; bolt Ø from the bolt's
  // horizontal extent; edge distance from the outermost bolt to the nearest plate edge.
  const tol = 10; // mm — merge near-coincident grid lines
  const boltCols = Math.max(1, cluster1d(bolts.map((x) => x.m.ca), tol).length);
  const boltRows = Math.max(1, cluster1d(bolts.map((x) => x.m.cc), tol).length);
  // Only a COMPLETE axis-aligned rectangular grid is a confident fit. A plate ROTATED about the vertical
  // axis (and its grid) clusters on the world X/Z axes into more cells than there are bolts, and its AABB
  // over-reads the plate (a 45° 400×200 plate AABBs to ~424×424) — so an off-axis base plate would recognize
  // with WRONG dims. Reject those (cols×rows ≠ bolt count) to the faithful custom-mesh fallback rather than
  // emit a wrong recipe. (A future 2D-OBB fit in the plate's OWN axes could recognize rotated plates too —
  // grow that when a real skewed-column case lands; the common orthogonal-grid case is correct here.)
  if (boltCols * boltRows !== bolts.length) return null;
  const boltDia = Math.round(median(bolts.map((x) => x.m.dia)));
  const offA = Math.max(...bolts.map((x) => Math.abs(x.m.ca - plate.ctr[a])));
  const offC = Math.max(...bolts.map((x) => Math.abs(x.m.cc - plate.ctr[c])));
  const edgeDist = Math.round(Math.max(0, Math.min(plateWidth / 2 - offA, plateDepth / 2 - offC)));

  // Plausibility gate — reject a fit outside real base-plate fabrication ranges (mm). edgeDist must be > 0:
  // a 0 means an anchor sits on the plate edge (half off it), i.e. not a real anchor-through-plate pattern.
  if (!(thickness > 3 && thickness < 200) || !(plateWidth > 60) || !(plateDepth > 60) || !(boltDia > 4 && boltDia < 120) || !(edgeDist > 0)) return null;

  // Reproduction gate: the consumer's expandBasePlate places anchors on a SYMMETRIC, centred grid with
  // ONE edge distance (outer offset = half-plate − edge on BOTH axes, evenly spread). Emit the recipe only
  // if that model actually rebuilds the observed grid to within a few mm — so a grid it CAN'T reproduce
  // (unequal per-axis margins, off-centre, or non-uniform pitch — e.g. a 500×300 plate with a 300×150 grid)
  // falls back to faithful mesh instead of re-deriving with moved bolts. (A richer per-axis-pitch recipe
  // could recognize those later.)
  const spread = (nn, half) => (nn <= 1 ? [0] : Array.from({ length: nn }, (_, i) => -half + (2 * half * i) / (nn - 1)));
  const expected = [];
  for (const ox of spread(boltCols, Math.max(0, plateWidth / 2 - edgeDist))) {
    for (const oy of spread(boltRows, Math.max(0, plateDepth / 2 - edgeDist))) expected.push([plate.ctr[a] + ox, plate.ctr[c] + oy]);
  }
  const gridTol = 3; // mm — absorb tessellation + whole-mm rounding
  const used = new Array(expected.length).fill(false);
  for (const x of bolts) {
    const hit = expected.findIndex((e, i) => !used[i] && Math.abs(x.m.ca - e[0]) <= gridTol && Math.abs(x.m.cc - e[1]) <= gridTol);
    if (hit < 0) return null; // a bolt the symmetric single-edge model can't place → not reproducible
    used[hit] = true;
  }

  // The column member a base plate hangs off (exactly one) — advisory: the consumer overrides `main` with
  // the column the user applies the connection to in their own model.
  const main = members.length ? members[0] : null;
  return { kind: 'base-plate', params: { thickness, plateWidth, plateDepth, boltDia, boltCols, boltRows, edgeDist }, main };
}

// Recognize a bolted shear / fin plate: a VERTICAL plate lapping a beam web, pierced by a single vertical
// line of HORIZONTAL bolts. The inverse of the base plate's vertical-anchor test (here every bolt is
// horizontal and shares ONE web-normal axis), so the two recognizers are mutually exclusive — a base plate
// never fits here and a fin plate never fits recognizeBasePlate. Returns { kind:'shear-plate', params, main }
// when confident, else null (the consumer then keeps it as opaque `custom` mesh — Slice B behaviour).
export function recognizeShearPlate(parts, members) {
  const bolts = parts.filter((p) => p.role === 'bolt').map((p) => ({ p, b: partBox(p) }));
  const plates = parts.filter((p) => p.role === 'plate').map((p) => ({ p, b: partBox(p) }));
  if (plates.length < 1 || bolts.length < 2) return null;

  // Every bolt HORIZONTAL and sharing ONE long axis `n` (the web-normal / bolt axis). Any vertical bolt →
  // that's a base plate's anchor group, not a fin plate → reject.
  if (bolts.some((x) => argMax(x.b.ext) === VERTICAL)) return null;
  const n = argMax(bolts[0].b.ext);
  if (!bolts.every((x) => argMax(x.b.ext) === n)) return null;
  const vAx = VERTICAL;                          // plate height axis
  const uAx = [0, 1, 2].find((k) => k !== n && k !== VERTICAL); // the in-plate horizontal (along the beam)
  for (const x of bolts) x.m = boltCentreDia(x.p, uAx, vAx);    // circle centre (ca=uAx, cc=vAx) + true Ø

  // Each bolt must run cleanly ALONG axis n: its cross-section (the two in-plate axes) must be near-circular
  // (both ≈ Ø). The bolts pass perpendicular through the plate, so a fin plate skewed a few degrees about the
  // vertical axis tilts them — inflating the in-plane cross extent, and equally inflating the plate's AABB
  // thin axis so `plate.ext[n]` would over-read the thickness. Reject a skewed group to faithful mesh (real
  // tessellation keeps the cross-section within ~4% of round; the base plate likewise rejects rotated grids).
  if (!bolts.every((x) => Math.max(x.b.ext[uAx], x.b.ext[vAx]) <= 1.15 * Math.min(x.b.ext[uAx], x.b.ext[vAx]))) return null;

  // Fin plate = a flat plate thin on the bolt axis `n`, its two large extents VERTICAL + along the beam.
  // Largest such plate if several (a doubler/washer is smaller). A plate not ⟂ the bolts → not the fin plate.
  const flats = plates.filter((x) => argMin(x.b.ext) === n
    && x.b.ext[vAx] > 3 * x.b.ext[n] && x.b.ext[uAx] > 3 * x.b.ext[n]);
  if (!flats.length) return null;
  flats.sort((x, y) => (y.b.ext[vAx] * y.b.ext[uAx]) - (x.b.ext[vAx] * x.b.ext[uAx]));
  const plate = flats[0].b;

  // Every bolt must PIERCE the plate: centroid inside its (u,v) footprint AND its n-span overlapping the
  // plate thickness band. Without this, a fin plate plus an unrelated horizontal bolt group off to the side
  // would still fit; reject those to the faithful mesh fallback.
  const overlapsN = (x) => x.b.min[n] <= plate.max[n] && x.b.max[n] >= plate.min[n];
  const insidePlate = (x) => Math.abs(x.m.ca - plate.ctr[uAx]) <= plate.ext[uAx] / 2
    && Math.abs(x.m.cc - plate.ctr[vAx]) <= plate.ext[vAx] / 2;
  if (!bolts.every((x) => overlapsN(x) && insidePlate(x))) return null;

  // Fitted dimensions rounded to whole mm — editable starting values a detailer can adjust.
  const plateThickness = Math.round(plate.ext[n]);
  const plateHeight = Math.round(plate.ext[vAx]);
  const plateWidth = Math.round(plate.ext[uAx]);

  const tol = 10; // mm — merge near-coincident grid lines
  const vLines = cluster1d(bolts.map((x) => x.m.cc), tol);      // vertical grid lines (rows), sorted ascending
  const rows = Math.max(1, vLines.length);
  const cols = Math.max(1, cluster1d(bolts.map((x) => x.m.ca), tol).length); // columns along the beam
  // ponytail: single vertical bolt line only — expandShearPlate centres bolts VERTICALLY (symmetric, one
  // pitch) but offsets the HORIZONTAL column toward the far edge of the bolted region, so a multi-column grid
  // can't be faithfully rebuilt from one edge/pitch. Reject cols>1 to faithful mesh; add a per-axis-pitch
  // recipe when a real double-column fin plate lands. (Also demands ≥2 rows + a COMPLETE 1×rows line.)
  if (cols !== 1 || rows < 2 || cols * rows !== bolts.length) return null;

  const boltDia = Math.round(median(bolts.map((x) => x.m.dia)));
  const gaps = vLines.slice(1).map((v, i) => v - vLines[i]);
  const boltPitch = Math.round(median(gaps));
  const offV = Math.max(...bolts.map((x) => Math.abs(x.m.cc - plate.ctr[vAx])));
  const offU = Math.max(...bolts.map((x) => Math.abs(x.m.ca - plate.ctr[uAx])));
  const edgeDist = Math.round(Math.max(0, Math.min(plateHeight / 2 - offV, plateWidth / 2 - offU)));

  // Plausibility gate — reject a fit outside real fin-plate fabrication ranges (mm). edgeDist must be > 0:
  // a 0 means a bolt sits on the plate edge (half off it), i.e. not a real bolt-through-plate pattern.
  if (!(plateThickness > 3 && plateThickness < 60) || !(plateHeight > 40) || !(plateWidth > 40)
    || !(boltDia > 4 && boltDia < 60) || !(boltPitch > 20) || !(edgeDist > 0)) return null;

  // Reproduction gate (VERTICAL axis — the one the engine centres): the consumer's expandShearPlate places
  // bolts on a SYMMETRIC, centred vertical line with ONE pitch. Emit the recipe only if that model actually
  // rebuilds the observed line to within a few mm — so an off-centre or non-uniform-pitch group (which the
  // single-pitch model would silently move) falls back to faithful mesh instead.
  const half = ((rows - 1) * boltPitch) / 2;
  const expectedV = Array.from({ length: rows }, (_, i) => plate.ctr[vAx] - half + i * boltPitch);
  const used = new Array(rows).fill(false);
  for (const x of bolts) {
    const hit = expectedV.findIndex((e, i) => !used[i] && Math.abs(x.m.cc - e) <= 3);
    if (hit < 0) return null; // a bolt the symmetric single-pitch model can't place → not reproducible
    used[hit] = true;
  }

  // The beam a fin plate hangs off (advisory — the consumer overrides `main` with the beam the user applies
  // the connection to in their own model).
  const main = members.length ? members[0] : null;
  return { kind: 'shear-plate', params: { plateThickness, plateHeight, plateWidth, boltDia, boltCols: cols, boltRows: rows, boltPitch, edgeDist }, main };
}
