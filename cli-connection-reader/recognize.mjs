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

  // The column member a base plate hangs off (exactly one) — advisory: the consumer overrides `main` with
  // the column the user applies the connection to in their own model.
  const main = members.length ? members[0] : null;
  return { kind: 'base-plate', params: { thickness, plateWidth, plateDepth, boltDia, boltCols, boltRows, edgeDist }, main };
}
