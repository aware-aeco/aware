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

// ── Oriented-fit geometry (OBB slice) ─────────────────────────────────────────────────────────────
// Plates only YAW about the vertical axis (a base plate is horizontal, a fin plate vertical — neither
// tilts), so an oriented fit is a 2-D minimum-area rectangle in the plate's face plane: find the yaw θ,
// rotate every part by −θ, and the existing world-axis body measures the now-aligned plate correctly. θ
// never enters the recipe (the consumer re-derives from its own member); it only lets us MEASURE a rotated
// plate. All pure + unit-testable, folded into the SEA.

// Convex hull (Andrew monotone chain) of 2-D points [[x,y]…] → hull vertices CCW, or null when there are
// fewer than 3 non-collinear points (a degenerate footprint → the caller bails to faithful mesh).
export function convexHull2d(pts) {
  const seen = new Set(), uniq = [];
  for (const p of pts) { const k = `${p[0].toFixed(4)},${p[1].toFixed(4)}`; if (!seen.has(k)) { seen.add(k); uniq.push(p); } }
  if (uniq.length < 3) return null;
  uniq.sort((a, b) => a[0] - b[0] || a[1] - b[1]);
  const cross = (o, a, b) => (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);
  const lower = [];
  for (const p of uniq) { while (lower.length >= 2 && cross(lower[lower.length - 2], lower[lower.length - 1], p) <= 0) lower.pop(); lower.push(p); }
  const upper = [];
  for (let i = uniq.length - 1; i >= 0; i--) { const p = uniq[i]; while (upper.length >= 2 && cross(upper[upper.length - 2], upper[upper.length - 1], p) <= 0) upper.pop(); upper.push(p); }
  lower.pop(); upper.pop();
  const hull = lower.concat(upper);
  return hull.length >= 3 ? hull : null; // all-collinear → null
}

// Minimum-area enclosing rectangle orientation of 2-D points → yaw θ (rad). Rotating the points by −θ
// axis-aligns the rectangle. A rectangle is 90°-symmetric, so θ is normalised into (−π/4, π/4] and the
// smaller |θ| wins an area tie — an already-aligned box therefore returns exactly 0 (identity rotation →
// existing aligned fixtures measure unchanged). Returns 0 on a degenerate hull.
export function minAreaRectAngle(pts) {
  const hull = convexHull2d(pts);
  if (!hull) return 0;
  let best = Infinity, bestAng = 0;
  for (let i = 0; i < hull.length; i++) {
    const a = hull[i], b = hull[(i + 1) % hull.length];
    let ex = b[0] - a[0], ey = b[1] - a[1];
    const len = Math.hypot(ex, ey);
    if (len < 1e-9) continue;
    ex /= len; ey /= len;
    let minU = Infinity, maxU = -Infinity, minV = Infinity, maxV = -Infinity;
    for (const p of hull) {
      const u = p[0] * ex + p[1] * ey, v = -p[0] * ey + p[1] * ex;
      if (u < minU) minU = u; if (u > maxU) maxU = u;
      if (v < minV) minV = v; if (v > maxV) maxV = v;
    }
    const area = (maxU - minU) * (maxV - minV);
    let ang = Math.atan2(ey, ex);
    ang = ((ang % (Math.PI / 2)) + Math.PI / 2) % (Math.PI / 2); // → [0, π/2)
    if (ang > Math.PI / 4) ang -= Math.PI / 2;                   // → (−π/4, π/4]
    if (area < best - 1e-6 || (Math.abs(area - best) <= 1e-6 && Math.abs(ang) < Math.abs(bestAng))) { best = area; bestAng = ang; }
  }
  return bestAng;
}

// Rotate each part's positions by `theta` (rad) about the vertical axis `va` (the two in-plane axes rotate).
// Immutable — returns fresh parts so a recognizer can measure in the plate-aligned frame without mutating
// the caller's geometry.
export function rotateAboutVertical(parts, theta, va) {
  const [a, c] = [0, 1, 2].filter((k) => k !== va);
  const cos = Math.cos(theta), sin = Math.sin(theta);
  return parts.map((p) => {
    const pos = p.positions.slice();
    for (let i = 0; i + 2 < pos.length; i += 3) {
      const ua = pos[i + a], uc = pos[i + c];
      pos[i + a] = ua * cos - uc * sin;
      pos[i + c] = ua * sin + uc * cos;
    }
    return { ...p, positions: pos };
  });
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

// ── Rectangularity gate (reject parallelograms / trapezoids / notches / slots / copes) ──────────────
// A recognized recipe rebuilds a FULL rectangular plate, so a plate that isn't a clean rectangle in its own
// face plane must fall back to faithful mesh. We measure how far the plate's true outer outline caves INWARD
// of its fitted rectangle: a rectangle deviates 0, a parallelogram/trapezoid by its skew, a notch/slot/cope
// by its depth. A convex hull would hide a concave defect, so with real mesh `indices` we walk the actual
// boundary loop; a synthetic part with no indices (unit-test clean boxes) falls back to a hull corner check.
const polyArea = (pts) => { let s = 0; for (let i = 0; i < pts.length; i++) { const a = pts[i], b = pts[(i + 1) % pts.length]; s += a[0] * b[1] - b[0] * a[1]; } return Math.abs(s) / 2; };

// Convex-hull area of a part's footprint on the plane ⟂ `va` — a rotation-invariant size for ranking plate
// candidates (a yaw doesn't change it, unlike the world-AABB area which inflates elongated rectangles).
function footprintHullArea(part, va) {
  const [a, c] = [0, 1, 2].filter((k) => k !== va);
  const pts = [];
  for (let i = 0; i + 2 < part.positions.length; i += 3) pts.push([part.positions[i + a], part.positions[i + c]]);
  const h = convexHull2d(pts);
  return h ? polyArea(h) : 0;
}

// The outer boundary loop of the plate's one extreme major face (normal = `va`), as face-plane points
// [[A,C]…], or null when the part carries no usable `indices`. Selects the +`va` extreme face, keeps edges
// that belong to exactly one of its triangles (the boundary), chains them into loops, and returns the
// largest-area loop — so interior bolt-hole loops are dropped.
function majorFaceLoop(part, va) {
  const idx = part.indices, pos = part.positions;
  if (!idx || idx.length < 3) return null;
  const [a, c] = [0, 1, 2].filter((k) => k !== va);
  let vmax = -Infinity, vmin = Infinity;
  for (let i = 0; i + 2 < pos.length; i += 3) { const v = pos[i + va]; if (v > vmax) vmax = v; if (v < vmin) vmin = v; }
  const band = Math.max(1e-6, (vmax - vmin) * 0.05);
  const onTop = (vi) => Math.abs(pos[vi * 3 + va] - vmax) <= band;
  // web-ifc DUPLICATES vertices per triangle (no index sharing), so two geometrically-adjacent triangles use
  // DIFFERENT indices for a shared edge → an index-keyed edge would never match. Key vertices by quantised
  // face POSITION instead, so a shared edge is detected and only the true outer edges are boundary.
  const q = (vi) => `${Math.round(pos[vi * 3 + a] * 100)}_${Math.round(pos[vi * 3 + c] * 100)}`;
  const ptOf = new Map();
  const ek = (u, w) => (u < w ? u + '|' + w : w + '|' + u);
  const count = new Map();
  for (let t = 0; t + 2 < idx.length; t += 3) {
    const tri = [idx[t], idx[t + 1], idx[t + 2]];
    if (!(onTop(tri[0]) && onTop(tri[1]) && onTop(tri[2]))) continue;
    const k = tri.map(q);
    for (let e = 0; e < 3; e++) { ptOf.set(k[e], [pos[tri[e] * 3 + a], pos[tri[e] * 3 + c]]); const key = ek(k[e], k[(e + 1) % 3]); count.set(key, (count.get(key) || 0) + 1); }
  }
  const boundary = [];
  for (const [key, m] of count) if (m === 1) boundary.push(key.split('|'));
  if (boundary.length < 3) return null;
  const adj = new Map();
  const push = (u, w) => { if (!adj.has(u)) adj.set(u, []); adj.get(u).push(w); };
  for (const [u, w] of boundary) { push(u, w); push(w, u); }
  const used = new Set(), loops = [];
  for (const [u0, w0] of boundary) {
    if (used.has(ek(u0, w0))) continue;
    used.add(ek(u0, w0));
    const loop = [u0]; let prev = u0, cur = w0;
    while (cur !== u0 && loop.length <= boundary.length) {
      loop.push(cur);
      let nxt = null;
      for (const nb of adj.get(cur) || []) if (nb !== prev && !used.has(ek(cur, nb))) { nxt = nb; break; }
      if (nxt === null) break;
      used.add(ek(cur, nxt)); prev = cur; cur = nxt;
    }
    if (cur === u0 && loop.length >= 3) loops.push(loop);
  }
  if (!loops.length) return null;
  const asPts = (loop) => loop.map((key) => ptOf.get(key));
  loops.sort((x, y) => polyArea(asPts(y)) - polyArea(asPts(x)));
  return asPts(loops[0]);
}

// Max INWARD deviation of a closed loop from the axis-aligned rectangle `rect` {minA,maxA,minC,maxC}. The
// inward distance d(A,C)=min(A−minA, maxA−A, C−minC, maxC−C) is concave (min of 4 linear terms), so its max
// along a segment is at an endpoint OR a crossover where the nearest edge switches — sampling only vertices
// would miss a large chamfer (endpoints ON the edges, midpoint bulged in). We sample endpoints + crossovers.
function loopInwardDeviation(pts, rect) {
  const d = (A, C) => Math.min(A - rect.minA, rect.maxA - A, C - rect.minC, rect.maxC - C);
  let maxDev = 0;
  for (let i = 0; i < pts.length; i++) {
    const p0 = pts[i], p1 = pts[(i + 1) % pts.length];
    const dA = p1[0] - p0[0], dC = p1[1] - p0[1];
    const lin = [ { b: p0[0] - rect.minA, m: dA }, { b: rect.maxA - p0[0], m: -dA }, { b: p0[1] - rect.minC, m: dC }, { b: rect.maxC - p0[1], m: -dC } ];
    const ts = [0, 1];
    for (let x = 0; x < 4; x++) for (let y = x + 1; y < 4; y++) {
      const dm = lin[x].m - lin[y].m;
      if (Math.abs(dm) < 1e-12) continue;
      const t = (lin[y].b - lin[x].b) / dm;
      if (t > 0 && t < 1) ts.push(t);
    }
    for (const t of ts) { const dev = d(p0[0] + dA * t, p0[1] + dC * t); if (dev > maxDev) maxDev = dev; }
  }
  return maxDev;
}

// Allowed inward deviation of a plate outline from its fitted rectangle (mm). A true rectangle tessellates
// EXACTLY (its corners are exact), so this only absorbs tessellation + the 0.01 mm position quantisation —
// NOT a real contour feature. Any notch / cope / snipe / chamfer beyond this → faithful mesh (the recipe
// would otherwise fill it with solid steel). Deliberately tight (2 mm), per the post-code review.
const RECT_DEV_TOL = 2;

// Is an ALIGNED plate's face (normal = `va`) a clean-enough rectangle? Fits the rectangle as the aligned
// AABB on the two non-`va` axes, then measures the plate's true outline inward deviation from it: the
// boundary-loop inward-Hausdorff with mesh `indices`, else a hull corner-residual (synthetic clean boxes).
// Deviation ≤ rectTol(shorter edge) passes. The rect and the outline share the SAME two face axes.
// A CONSTANT-thickness prism? Compares the (a,c) bounding box of the vertices near the two extreme faces
// along `va`. A tapered plate / frustum / blind pocket (e.g. a 400×200 top over a 300×100 bottom) has
// mismatched faces and must fall back to mesh (validating one face would emit a full rectangular prism).
function prismUniform(part, va) {
  const pos = part.positions;
  let vmin = Infinity, vmax = -Infinity;
  for (let i = 0; i + 2 < pos.length; i += 3) { const v = pos[i + va]; if (v < vmin) vmin = v; if (v > vmax) vmax = v; }
  const band = Math.max(1e-6, (vmax - vmin) * 0.2);
  const [a, c] = [0, 1, 2].filter((k) => k !== va);
  const faceBox = (lo, hi) => {
    let na = Infinity, xa = -Infinity, nc = Infinity, xc = -Infinity, cnt = 0;
    for (let i = 0; i + 2 < pos.length; i += 3) { const v = pos[i + va]; if (v >= lo && v <= hi) { cnt++; const A = pos[i + a], C = pos[i + c]; if (A < na) na = A; if (A > xa) xa = A; if (C < nc) nc = C; if (C > xc) xc = C; } }
    return cnt ? [xa - na, xc - nc] : null;
  };
  const mc = (vmin + vmax) / 2;
  const top = faceBox(vmax - band, vmax), bot = faceBox(vmin, vmin + band), mid = faceBox(mc - band / 2, mc + band / 2);
  if (!top || !bot || !mid) return true;
  const near = (x, y) => Math.abs(x[0] - y[0]) <= 2 && Math.abs(x[1] - y[1]) <= 2;
  return near(top, bot) && near(top, mid); // also the MIDDLE — catches a waisted/necked plate the two extremes miss
}
function rectOK(alignedPlate, va) {
  const b = partBox(alignedPlate);
  const [a, c] = [0, 1, 2].filter((k) => k !== va);
  if (!prismUniform(alignedPlate, va)) return false; // tapered / frustum / pocket → mesh
  const rect = { minA: b.min[a], maxA: b.max[a], minC: b.min[c], maxC: b.max[c] };
  const loop = majorFaceLoop(alignedPlate, va);
  if (loop) return loopInwardDeviation(loop, rect) <= RECT_DEV_TOL;
  // No clean outer major-face loop. For REAL indexed geometry this is a red flag (a plate tilted out of plane,
  // warped, or degenerate — no complete face sits in the extreme band), so FAIL CLOSED → faithful mesh; the
  // convex-hull path can't see a concave defect and must not be trusted for production meshes. The hull
  // corner-residual is only for synthetic parts with no indices (unit-test clean boxes).
  if (alignedPlate.indices && alignedPlate.indices.length >= 3) return false;
  const pts = [];
  for (let i = 0; i + 2 < alignedPlate.positions.length; i += 3) pts.push([alignedPlate.positions[i + a], alignedPlate.positions[i + c]]);
  const hull = convexHull2d(pts);
  if (!hull) return false;
  const corners = [[rect.minA, rect.minC], [rect.maxA, rect.minC], [rect.maxA, rect.maxC], [rect.minA, rect.maxC]];
  let dev = 0;
  for (const cor of corners) { let best = Infinity; for (const hp of hull) best = Math.min(best, Math.hypot(hp[0] - cor[0], hp[1] - cor[1])); if (best > dev) dev = best; }
  return dev <= RECT_DEV_TOL;
}

// ── Bolt-axis yaw for a fin plate (3D, doubler-immune) ─────────────────────────────────────────────
// A fin plate's frame is defined by its coherent HORIZONTAL bolt group, not by any plate (a support doubler
// can be orthogonal to the fin). `argMax` can't recover a 45°-yawed bolt's direction (its X/Z AABB extents
// tie), so we take each bolt's dominant covariance eigenvector in 3D. A tilted or vertical anchor (>15° from
// horizontal) or a non-elongated shape (eigenvalue ratio < 3) is rejected — those aren't clean fin bolts.
const matVec = (M, v) => [M[0][0] * v[0] + M[0][1] * v[1] + M[0][2] * v[2], M[1][0] * v[0] + M[1][1] * v[1] + M[1][2] * v[2], M[2][0] * v[0] + M[2][1] * v[1] + M[2][2] * v[2]];
const unit3 = (v) => { const n = Math.hypot(v[0], v[1], v[2]) || 1; return [v[0] / n, v[1] / n, v[2] / n]; };
function powerEig(M, seed) {
  let v = unit3(seed);
  for (let k = 0; k < 60; k++) v = unit3(matVec(M, v));
  const Mv = matVec(M, v);
  return { vec: v, val: v[0] * Mv[0] + v[1] * Mv[1] + v[2] * Mv[2] };
}
// Dominant eigenpair of a symmetric 3×3: run power iteration from ALL THREE basis seeds and keep the largest
// Rayleigh quotient. The dominant eigenvector is non-orthogonal to at least one basis vector, so this never
// stalls on a minor eigenvector (a single fixed seed can, for a specific elliptical covariance).
function powerEigBest(M) {
  let best = null;
  for (const seed of [[1, 0, 0], [0, 1, 0], [0, 0, 1]]) { const e = powerEig(M, seed); if (!best || e.val > best.val) best = e; }
  return best;
}
// Dominant axis (unit vec) + anisotropy ratio λ1/λ2 of a point set's covariance.
function dominantAxis(pos) {
  let cx = 0, cy = 0, cz = 0, n = 0;
  for (let i = 0; i + 2 < pos.length; i += 3) { cx += pos[i]; cy += pos[i + 1]; cz += pos[i + 2]; n++; }
  cx /= n; cy /= n; cz /= n;
  let xx = 0, xy = 0, xz = 0, yy = 0, yz = 0, zz = 0;
  for (let i = 0; i + 2 < pos.length; i += 3) { const dx = pos[i] - cx, dy = pos[i + 1] - cy, dz = pos[i + 2] - cz; xx += dx * dx; xy += dx * dy; xz += dx * dz; yy += dy * dy; yz += dy * dz; zz += dz * dz; }
  const C = [[xx / n, xy / n, xz / n], [xy / n, yy / n, yz / n], [xz / n, yz / n, zz / n]];
  const e1 = powerEigBest(C);
  const C2 = C.map((row, i) => row.map((x, j) => x - e1.val * e1.vec[i] * e1.vec[j])); // deflate
  const e2 = powerEigBest(C2);
  return { dir: e1.vec, ratio: e2.val > 1e-9 ? e1.val / e2.val : Infinity };
}
// Is a part's cross-section in the (a,c) plane a real tessellated CIRCLE (not a square/hex/elliptical prism)?
// Vertex-radius spread can't tell — every regular polygon has equidistant vertices — so we test the convex
// hull's RESOLUTION (≥ 8 sides) and how fully it fills the equal-area circle (a square hull fills ~0.64, a
// hexagon ~0.83, an octagon ~0.90, a real ≥16-gon bolt ~0.99). Gate ≥ 0.85 + ≥ 8 sides.
function crossSectionCircular(pos, a, c) {
  const pts = [];
  for (let i = 0; i + 2 < pos.length; i += 3) pts.push([pos[i + a], pos[i + c]]);
  const hull = convexHull2d(pts);
  if (!hull || hull.length < 8) return false;
  const area = polyArea(hull);
  let cx = 0, cc2 = 0;
  for (const p of hull) { cx += p[0]; cc2 += p[1]; }
  cx /= hull.length; cc2 /= hull.length;
  let sr = 0;
  for (const p of hull) sr += Math.hypot(p[0] - cx, p[1] - cc2);
  const r = sr / hull.length;
  return r > 1e-6 && area >= 0.85 * Math.PI * r * r;
}
// Yaw (rad, about vertical) of the coherent bolt group, or null if the bolts aren't clean horizontal
// cylinders sharing one axis. Rotating parts by −yaw aligns the bolt axis to a world horizontal axis.
function shearBoltYaw(boltParts) {
  const [ha, hc] = [0, 1, 2].filter((k) => k !== VERTICAL);
  const sin15 = Math.sin((15 * Math.PI) / 180), spreadTol = 2 * ((5 * Math.PI) / 180);
  let sx = 0, sy = 0; const angs = [];
  for (const p of boltParts) {
    const { dir, ratio } = dominantAxis(p.positions);
    if (Math.abs(dir[VERTICAL]) > sin15 || ratio < 3) return null; // tilted/vertical or not elongated
    const ang = Math.atan2(dir[hc], dir[ha]);
    angs.push(ang); sx += Math.cos(2 * ang); sy += Math.sin(2 * ang); // doubled-angle (signless axis)
  }
  if (!angs.length) return null;
  const avg2 = Math.atan2(sy, sx);
  for (const ang of angs) { const d = Math.abs(((2 * ang - avg2 + Math.PI) % (2 * Math.PI) + 2 * Math.PI) % (2 * Math.PI) - Math.PI); if (d > spreadTol) return null; }
  return avg2 / 2;
}

// Recognize a base plate from tessellated parts: a horizontal plate with a VERTICAL anchor-bolt grid
// passing through it. Handles a plate YAWED about the vertical axis — each plate candidate is fitted in its
// OWN min-area-rectangle frame (rotate by −θ, then measure), so a rotated base plate recognizes with correct
// dims instead of the world-AABB over-read. Returns { kind:'base-plate', params, main } or null (→ mesh).
export function recognizeBasePlate(parts, members) {
  const bolts = parts.filter((p) => p.role === 'bolt');
  const plates = parts.filter((p) => p.role === 'plate');
  if (plates.length < 1 || bolts.length < 2) return null;
  // Every anchor a TIGHTLY-vertical, elongated, ROUND cylinder (a coherent anchor group). Verticality is
  // yaw-invariant so the world-frame test holds under plate rotation; the covariance axis + roundness reject a
  // tilted anchor (whose horizontal radius would over-read) or a square/elliptical prism (whose diagonal would
  // read as the diameter). A fin plate's horizontal bolts are rejected here, never mis-read as a base plate.
  const [ha, hc] = [0, 2];
  if (!bolts.every((p) => {
    const { dir, ratio } = dominantAxis(p.positions);
    return Math.abs(dir[VERTICAL]) >= 0.996 && ratio >= 3 && crossSectionCircular(p.positions, ha, hc);
  })) return null;
  // Try each plate as the frame-defining candidate, largest CONVEX-HULL area first (rotation-invariant — a
  // yaw doesn't change it, so a real base plate still outranks a washer/doubler). Accept the first that fits
  // every gate; a larger unpierced cover plate fails pierce/reproduction and the next candidate is tried.
  const candidates = plates
    .map((p) => ({ p, area: footprintHullArea(p, VERTICAL) }))
    .filter((x) => x.area > 0)
    .sort((x, y) => y.area - x.area);
  for (const cand of candidates) {
    const r = fitBasePlate(parts, cand.p, members);
    if (r) return r;
  }
  return null;
}

// Fit a base-plate recipe with `candidatePlate` as the frame-defining plate — CANDIDATE-BOUND (it measures
// the given plate, never re-selects the largest), so candidate iteration actually tries each one. null on
// any gate → the caller moves on / falls back to faithful mesh.
function fitBasePlate(parts, candidatePlate, members) {
  const [a, c] = [0, 2]; // the two horizontal axes
  // θ from the candidate plate's horizontal footprint; rotate every part by −θ into that aligned frame.
  const foot = [];
  for (let i = 0; i + 2 < candidatePlate.positions.length; i += 3) foot.push([candidatePlate.positions[i + a], candidatePlate.positions[i + c]]);
  if (!convexHull2d(foot)) return null; // degenerate footprint → mesh
  const theta = minAreaRectAngle(foot);
  const R = rotateAboutVertical(parts, -theta, VERTICAL);
  const alignedPlate = R[parts.indexOf(candidatePlate)];
  const plate = partBox(alignedPlate);

  // Candidate must be a flat plate thin in the vertical axis (⟂ the anchors), wide in both horizontals.
  if (!(argMin(plate.ext) === VERTICAL && plate.ext[a] > 3 * plate.ext[VERTICAL] && plate.ext[c] > 3 * plate.ext[VERTICAL])) return null;

  // Rectangularity: the aligned plate face must be a clean rectangle (rejects a parallelogram/trapezoid/
  // notch/slot/cope that a full-rectangle recipe can't reproduce).
  if (!rectOK(alignedPlate, VERTICAL)) return null;

  // Anchors, measured in the aligned frame (true circle centre + Ø).
  const bolts = [];
  for (let bi = 0; bi < parts.length; bi++) if (parts[bi].role === 'bolt') bolts.push({ b: partBox(R[bi]), m: boltCentreDia(R[bi], a, c) });
  if (bolts.length < 2) return null;

  // Pierce: every anchor centroid inside the plate footprint AND its vertical span overlapping the plate
  // thickness band (an unrelated grid off to the side / above / below → rejected to faithful mesh).
  const overlapsV = (x) => x.b.min[VERTICAL] <= plate.max[VERTICAL] && x.b.max[VERTICAL] >= plate.min[VERTICAL];
  const inside = (x) => Math.abs(x.m.ca - plate.ctr[a]) <= plate.ext[a] / 2 && Math.abs(x.m.cc - plate.ctr[c]) <= plate.ext[c] / 2;
  if (!bolts.every((x) => overlapsV(x) && inside(x))) return null;

  const thickness = Math.round(plate.ext[VERTICAL]);
  const tol = 10; // mm — merge near-coincident grid lines
  const clA = cluster1d(bolts.map((x) => x.m.ca), tol).length;
  const clC = cluster1d(bolts.map((x) => x.m.cc), tol).length;
  // CANONICAL ordering — longer aligned horizontal extent = plateWidth (+ its bolt-line count = boltCols),
  // swapped together, so the SAME physical plate at any yaw quadrant yields ONE recipe. For a near-SQUARE
  // plate (extents equal within tol) "longer" can't reliably disambiguate: tie-break by the GRID (fewer lines
  // = cols) when the counts differ, else by the longer RAW extent (so a 400×395 2×2 and its 90°-yaw copy still
  // give one recipe — the count tie-break alone flipped width/depth for a near-square equal-count grid).
  const square = Math.abs(plate.ext[a] - plate.ext[c]) <= tol;
  const wAxis = !square ? (plate.ext[a] > plate.ext[c] ? a : c)
    : (clA !== clC ? (clA < clC ? a : c) : (plate.ext[a] >= plate.ext[c] ? a : c));
  const dAxis = wAxis === a ? c : a;
  const cAlong = (m, ax) => (ax === a ? m.ca : m.cc);
  const bw = bolts.map((x) => ({ w: cAlong(x.m, wAxis) - plate.ctr[wAxis], d: cAlong(x.m, dAxis) - plate.ctr[dAxis], dia: x.m.dia }));
  const plateWidth = Math.round(plate.ext[wAxis]);
  const plateDepth = Math.round(plate.ext[dAxis]);
  const boltCols = Math.max(1, cluster1d(bw.map((x) => x.w), tol).length);
  const boltRows = Math.max(1, cluster1d(bw.map((x) => x.d), tol).length);
  if (boltCols * boltRows !== bolts.length) return null; // complete rectangular grid only
  // Uniform anchor Ø — a mixed-diameter grid isn't one fitted kit; the median would silently homogenise it
  // into a wrong recipe. Gate the raw-measured SPREAD tight (≤ 1.5 mm) — standard nominal sizes are ≥ 3 mm
  // apart (M24/M27), so this separates them while absorbing tessellation noise.
  const dias = bw.map((x) => x.dia);
  if (Math.max(...dias) - Math.min(...dias) > 1.5) return null;
  const boltDia = Math.round(median(dias));
  const offW = Math.max(...bw.map((x) => Math.abs(x.w)));
  const offD = Math.max(...bw.map((x) => Math.abs(x.d)));
  const edgeDist = Math.round(Math.max(0, Math.min(plateWidth / 2 - offW, plateDepth / 2 - offD)));

  // Plausibility gate — real base-plate fabrication ranges (mm). edgeDist must clear the drilled HOLE, not
  // just the bolt CENTRE: require ≥ boltDia/2 + 3 (the engine's anchor hole is ~boltDia+6, so its radius is
  // boltDia/2+3) so the hole sits fully on the plate — a centre-only "inside" test would pass a hole that
  // hangs off the edge.
  if (!(thickness > 3 && thickness < 200) || !(plateWidth > 60) || !(plateDepth > 60) || !(boltDia > 4 && boltDia < 120) || !(edgeDist >= boltDia / 2 + 3)) return null;

  // Reproduction gate: the consumer's expandBasePlate places anchors on a SYMMETRIC, centred grid with ONE
  // edge distance (evenly spread from half-plate − edge on BOTH axes). Emit only if that model rebuilds the
  // observed grid to within a few mm — so unequal per-axis margins / off-centre / non-uniform pitch fall back
  // to faithful mesh instead of re-deriving with moved bolts.
  const spread = (nn, half) => (nn <= 1 ? [0] : Array.from({ length: nn }, (_, i) => -half + (2 * half * i) / (nn - 1)));
  const expected = [];
  for (const ow of spread(boltCols, Math.max(0, plateWidth / 2 - edgeDist))) {
    for (const od of spread(boltRows, Math.max(0, plateDepth / 2 - edgeDist))) expected.push([ow, od]);
  }
  const gridTol = 3; // mm — absorb tessellation + whole-mm rounding
  const used = new Array(expected.length).fill(false);
  for (const x of bw) {
    const hit = expected.findIndex((e, i) => !used[i] && Math.abs(x.w - e[0]) <= gridTol && Math.abs(x.d - e[1]) <= gridTol);
    if (hit < 0) return null; // a bolt the symmetric single-edge model can't place → not reproducible
    used[hit] = true;
  }

  // The column member a base plate hangs off (advisory — the consumer overrides `main` with the column the
  // user applies the connection to in their own model).
  const main = members.length ? members[0] : null;
  return { kind: 'base-plate', params: { thickness, plateWidth, plateDepth, boltDia, boltCols, boltRows, edgeDist }, main };
}

// Recognize a bolted shear / fin plate: a VERTICAL plate lapping a beam web, pierced by a grid of HORIZONTAL
// bolts. The inverse of the base plate's vertical-anchor test (here bolts are horizontal, sharing ONE
// web-normal axis), so the two recognizers are mutually exclusive. Handles a plate YAWED about the vertical
// axis (θ from the bolt axis) and a MULTI-column bolt grid (per-axis `boltColPitch`). Returns
// { kind:'shear-plate', params, main } or null (→ faithful mesh).
export function recognizeShearPlate(parts, members) {
  const bolts = parts.filter((p) => p.role === 'bolt');
  const plates = parts.filter((p) => p.role === 'plate');
  if (plates.length < 1 || bolts.length < 2) return null;
  // Any VERTICAL bolt → a base plate's anchor group, not a fin plate → reject (mutually exclusive).
  if (bolts.some((p) => argMax(partBox(p).ext) === VERTICAL)) return null;
  // θ from the coherent horizontal bolt axis (3D, doubler-immune); align every part into that frame.
  const yaw = shearBoltYaw(bolts);
  if (yaw === null) return null;
  const R = rotateAboutVertical(parts, -yaw, VERTICAL);
  const rbolts = [];
  for (let i = 0; i < parts.length; i++) if (parts[i].role === 'bolt') rbolts.push(R[i]);
  const n = argMax(partBox(rbolts[0]).ext); // bolt/web-normal axis, now a world horizontal axis
  if (n === VERTICAL || !rbolts.every((rp) => argMax(partBox(rp).ext) === n)) return null;
  const vAx = VERTICAL, uAx = [0, 1, 2].find((k) => k !== n && k !== VERTICAL);
  // Candidate plates ⟂ n (thin on the bolt axis), largest face-area first; iterate so a larger PARALLEL but
  // unpierced cover plate can't block the real fin plate (candidate-bound fit).
  const cands = [];
  for (let i = 0; i < parts.length; i++) if (parts[i].role === 'plate') {
    const b = partBox(R[i]);
    if (argMin(b.ext) === n) cands.push({ rp: R[i], b, area: b.ext[vAx] * b.ext[uAx] });
  }
  cands.sort((x, y) => y.area - x.area);
  for (const cand of cands) {
    const r = fitShearPlate(rbolts, cand, n, uAx, vAx, members);
    if (r) return r;
  }
  return null;
}

// Fit a shear-plate recipe using `cand` as the ⟂-n plate (candidate-bound). Bolts already rotated into the
// aligned frame; measure the grid, gate rectangularity + pierce + engine reproduction. null on any gate.
function fitShearPlate(rbolts, cand, n, uAx, vAx, members) {
  const plate = cand.b;
  // Flat plate thin on `n`, both large extents ≥ 3× thickness.
  if (!(plate.ext[vAx] > 3 * plate.ext[n] && plate.ext[uAx] > 3 * plate.ext[n])) return null;
  const bolts = rbolts.map((rp) => ({ rp, b: partBox(rp), m: boltCentreDia(rp, uAx, vAx) }));
  // Each bolt a real tessellated CIRCLE in cross-section (rejects a square/hex/elliptical prism whose diagonal
  // would read as the diameter). A rigid yaw is already undone by the alignment, so this guards real distortion.
  if (!bolts.every((x) => crossSectionCircular(x.rp.positions, uAx, vAx))) return null;

  // Rectangularity of the plate's (u,v) FACE (normal = n) — rejects a parallelogram / trapezoid / notch.
  if (!rectOK(cand.rp, n)) return null;

  // Pierce: centroid inside the (u,v) footprint AND n-span overlaps the plate thickness band.
  const overlapsN = (x) => x.b.min[n] <= plate.max[n] && x.b.max[n] >= plate.min[n];
  const inside = (x) => Math.abs(x.m.ca - plate.ctr[uAx]) <= plate.ext[uAx] / 2 && Math.abs(x.m.cc - plate.ctr[vAx]) <= plate.ext[vAx] / 2;
  if (!bolts.every((x) => overlapsN(x) && inside(x))) return null;

  const plateThickness = Math.round(plate.ext[n]);
  const plateHeight = Math.round(plate.ext[vAx]);
  const plateWidth = Math.round(plate.ext[uAx]);
  const tol = 10; // mm — merge near-coincident grid lines
  const vLines = cluster1d(bolts.map((x) => x.m.cc), tol); // rows (vertical)
  const uLines = cluster1d(bolts.map((x) => x.m.ca), tol); // columns (along the beam)
  const rows = vLines.length, cols = uLines.length;
  if (rows < 2 || cols * rows !== bolts.length) return null; // ≥2 rows + a complete grid count
  // Complete rectangular grid: every (col,row) cell occupied exactly once (count alone allows a degenerate
  // arrangement — e.g. two bolts in one cell).
  const nearIdx = (v, lines) => lines.reduce((bi, _, i) => (Math.abs(v - lines[i]) < Math.abs(v - lines[bi]) ? i : bi), 0);
  const cell = new Array(rows * cols).fill(0);
  for (const x of bolts) cell[nearIdx(x.m.cc, vLines) * cols + nearIdx(x.m.ca, uLines)]++;
  if (!cell.every((k) => k === 1)) return null;

  const sdias = bolts.map((x) => x.m.dia);
  // Uniform Ø — a mixed-diameter bolt group isn't one fitted kit; gate the raw SPREAD tight (≤ 1.5 mm; nominal
  // sizes are ≥ 3 mm apart) so the median can't silently homogenise M20+M24 into a wrong recipe.
  if (Math.max(...sdias) - Math.min(...sdias) > 1.5) return null;
  const boltDia = Math.round(median(sdias));
  const vGaps = vLines.slice(1).map((v, i) => v - vLines[i]);
  const boltPitch = Math.round(median(vGaps));
  const offV = Math.max(...bolts.map((x) => Math.abs(x.m.cc - plate.ctr[vAx])));
  const offU = Math.max(...bolts.map((x) => Math.abs(x.m.ca - plate.ctr[uAx])));
  const edgeVMargin = plateHeight / 2 - offV;
  const edgeUMargin = plateWidth / 2 - offU;   // the horizontal side margin — plausibility only
  const edgeDist = Math.round(Math.max(0, edgeVMargin));
  // Multi-column: an independent HORIZONTAL pitch, present only when cols>1 (median([]) is NaN for one col).
  let boltColPitch;
  if (cols > 1) {
    const uGaps = uLines.slice(1).map((u, i) => u - uLines[i]);
    boltColPitch = Math.round(median(uGaps));
    if (!uGaps.every((g) => Math.abs(g - boltColPitch) <= tol)) return null; // non-uniform horizontal pitch → mesh
    if (boltColPitch < 2 * boltDia) return null; // fabrication non-overlap gate (bolts/holes can't overlap)
  }

  // Plausibility gate — real fin-plate fabrication ranges (mm). Each margin must clear the HOLE (≥ boltDia/2
  // + 2), not just the bolt CENTRE — a centre-only test passes a bolt whose hole hangs off the plate edge.
  const edgeMin = boltDia / 2 + 2;
  if (!(plateThickness > 3 && plateThickness < 60) || !(plateHeight > 40) || !(plateWidth > 40)
    || !(boltDia > 4 && boltDia < 60) || !(boltPitch > 20) || !(edgeVMargin >= edgeMin) || !(edgeUMargin >= edgeMin)) return null;

  // Reproduction (engine-relative, clamp-aware). expandShearPlate places rows on a SYMMETRIC centred
  // single-pitch line, and seats the column group at a fabrication-derived cU (NOT recipe-steerable) spread at
  // pU=min(boltColPitch, region/(cols−1)). The along-beam GROUP POSITION is deliberately RE-DERIVED — the
  // recognizer can't observe which way the beam runs, and the user applies the connection to a DIFFERENT beam
  // whose clearance sets it (this matches the shipped single-column behaviour); we bound the group's
  // distance-FROM-CENTRE, not its absolute place. (A ≤ clearance/2 along-beam fidelity offset is the documented
  // ceiling — a centred observed group is re-seated at the engine's fabrication position.)
  const half = ((rows - 1) * boltPitch) / 2;
  const expectedV = Array.from({ length: rows }, (_, i) => plate.ctr[vAx] - half + i * boltPitch);
  const clearance = 12.7; // engine default
  const edgeU = Math.min(edgeDist, plateWidth * 0.4);
  const regHi = Math.max(edgeU, plateWidth - edgeU);
  const regLo = Math.min(clearance + edgeU, regHi);
  const cU = (regLo + regHi) / 2;
  const cDispl = Math.abs(cU - plateWidth / 2);            // engine's intrinsic group offset from plate centre
  const uMean = uLines.reduce((s, u) => s + u, 0) / cols;
  const offUGroup = Math.abs(uMean - plate.ctr[uAx]);
  if (Math.abs(offUGroup - cDispl) > Math.max(3, cDispl)) return null; // group too far from where the engine seats it
  const pU = cols > 1 ? Math.min(boltColPitch, (regHi - regLo) / (cols - 1)) : 0; // engine's actual (clamped) pitch
  // PER-BOLT reproduction — every bolt's (u,v) must match its engine-expanded cell (row = expectedV[i];
  // column, relative to the group centroid, = (j−(cols−1)/2)·pU), not merely the clustered LINE MEANS. A
  // zigzag whose per-row/column means still average out (Codex) is caught here, as is a clamped pitch.
  // 2 mm tolerance (was 3): the engine's bolt hole has only ~1 mm radial clearance, so a bolt that reproduces
  // >2 mm off its cell can prevent assembly. 2 mm still absorbs tessellation + whole-mm pitch rounding.
  for (const x of bolts) {
    const irow = nearIdx(x.m.cc, vLines), jcol = nearIdx(x.m.ca, uLines);
    if (Math.abs(x.m.cc - expectedV[irow]) > 2 || Math.abs((x.m.ca - uMean) - (jcol - (cols - 1) / 2) * pU) > 2) return null;
  }

  // The beam a fin plate hangs off (advisory — the consumer overrides `main` with the beam the user applies
  // the connection to in their own model).
  const main = members.length ? members[0] : null;
  const params = { plateThickness, plateHeight, plateWidth, boltDia, boltCols: cols, boltRows: rows, boltPitch, edgeDist };
  if (cols > 1) params.boltColPitch = boltColPitch;
  return { kind: 'shear-plate', params, main };
}
