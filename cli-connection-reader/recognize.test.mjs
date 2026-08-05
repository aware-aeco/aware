// Unit tests for the pure base-plate recognition (recognize.mjs). `node --test`.
//
// Fixtures are authored in web-ifc's Y-UP frame (vertical = axis 1), because that is the frame
// web-ifc actually emits and the one these shapes read naturally in — a base plate is "thin in Y",
// a fin-plate bolt runs along X. They are rotated to the file's Z-up frame at the boundary below,
// exactly as `tessellate` now does (#347), so the recognizer sees here what it sees in production.
//
// Converting at the seam rather than re-authoring every fixture is deliberate: a rigid rotation
// preserves every length, so each test's dimensional assertions stay true and stay checkable
// against the drawing they came from. Re-deriving forty fixtures' coordinates by hand would have
// risked changing what a test measures while claiming only to change its frame.
import test from 'node:test';
import assert from 'node:assert/strict';
import {
  recognizeBasePlate as recognizeBasePlateZUp,
  recognizeShearPlate as recognizeShearPlateZUp,
  convexHull2d, minAreaRectAngle, rotateAboutVertical,
} from './recognize.mjs';

// web-ifc's Y-up -> the file's Z-up: `(x, y, z) -> (x, -z, y)`. The same mapping `tessellate`
// applies, spelled once.
const zUp = (parts) => parts.map((p) => {
  const out = [];
  for (let i = 0; i + 2 < p.positions.length; i += 3) {
    out.push(p.positions[i], -p.positions[i + 2], p.positions[i + 1]);
  }
  return { ...p, positions: out };
});
const recognizeBasePlate = (parts, ids) => recognizeBasePlateZUp(zUp(parts), ids);
const recognizeShearPlate = (parts, ids) => recognizeShearPlateZUp(zUp(parts), ids);

// An axis-aligned box part centred at (cx,cy,cz) with extents (ex,ey,ez). Only positions matter to
// recognition (it AABBs them); indices are irrelevant here.
function box(role, cx, cy, cz, ex, ey, ez) {
  const h = [ex / 2, ey / 2, ez / 2], pos = [];
  for (const sx of [-1, 1]) for (const sy of [-1, 1]) for (const sz of [-1, 1]) pos.push(cx + sx * h[0], cy + sy * h[1], cz + sz * h[2]);
  return { role, positions: pos, indices: [] };
}
// A vertical cylinder (like a real anchor): `sides` vertices ON the circle at top+bottom, so the circle
// fit recovers the true diameter — matching how web-ifc tessellates a circular profile.
function cyl(role, cx, cy, cz, dia, height, sides = 16) {
  const r = dia / 2, pos = [];
  for (const sy of [-1, 1]) for (let k = 0; k < sides; k++) {
    const t = (2 * Math.PI * k) / sides;
    pos.push(cx + r * Math.cos(t), cy + sy * height / 2, cz + r * Math.sin(t));
  }
  return { role, positions: pos, indices: [] };
}

test('recognizes a base plate with a 2×2 vertical anchor grid', () => {
  const parts = [box('plate', 0, 100, 0, 400, 25, 400)]; // horizontal 400×400×25 (thin in Y)
  for (const x of [-120, 120]) for (const z of [-120, 120]) parts.push(cyl('bolt', x, 100, z, 24, 250)); // M24 anchors on a 240×240 grid
  const r = recognizeBasePlate(parts, ['COL-GUID', 'X']);
  assert.ok(r, 'should recognize a base plate');
  assert.equal(r.kind, 'base-plate');
  assert.deepEqual(r.params, { thickness: 25, plateWidth: 400, plateDepth: 400, boltDia: 24, boltCols: 2, boltRows: 2, edgeDist: 80 });
  assert.equal(r.main, 'COL-GUID'); // advisory: the first member
});

test('rejects a vertical shear/fin plate (horizontal bolts)', () => {
  const parts = [
    box('plate', 0, 0, 0, 10, 300, 200),   // vertical fin (thin in X)
    box('bolt', 0, 60, 0, 200, 24, 24),    // horizontal bolt (long axis X, not vertical)
    box('bolt', 0, -60, 0, 200, 24, 24),
  ];
  assert.equal(recognizeBasePlate(parts, ['B']), null);
});

test('rejects a plate with no anchors (can’t fit a grid)', () => {
  assert.equal(recognizeBasePlate([box('plate', 0, 0, 0, 400, 25, 400)], ['C']), null);
});

test('rejects a base plate rotated about the vertical axis (would fit wrong dims)', () => {
  // A 400×200 plate + 2×2 grid rotated 45° about vertical: the world-axis AABB over-reads it and the grid
  // mis-clusters, so recognition must decline (→ faithful custom mesh), not emit a wrong recipe.
  const ang = Math.PI / 4, ca = Math.cos(ang), sa = Math.sin(ang);
  const rot = (x, z) => [x * ca - z * sa, x * sa + z * ca];
  function boxRot(role, cx, cy, cz, ex, ey, ez) {
    const h = [ex / 2, ey / 2, ez / 2], pos = [];
    for (const sx of [-1, 1]) for (const sy of [-1, 1]) for (const sz of [-1, 1]) {
      const [rx, rz] = rot(sx * h[0], sz * h[2]);
      pos.push(cx + rx, cy + sy * h[1], cz + rz);
    }
    return { role, positions: pos, indices: [] };
  }
  const parts = [boxRot('plate', 0, 100, 0, 400, 25, 200)];
  for (const x of [-120, 120]) for (const z of [-40, 40]) { const [rx, rz] = rot(x, z); parts.push(boxRot('bolt', rx, 100, rz, 24, 250, 24)); }
  assert.equal(recognizeBasePlate(parts, ['C']), null);
});

test('rejects vertical bolts that do not pierce the plate (off to the side)', () => {
  const parts = [
    box('plate', 0, 100, 0, 400, 25, 400),      // plate footprint x,z ∈ [-200,200]
    box('bolt', 600, 100, 0, 24, 250, 24),       // grid off to the side, outside the footprint
    box('bolt', 700, 100, 0, 24, 250, 24),
  ];
  assert.equal(recognizeBasePlate(parts, ['C']), null);
});

test('rejects vertical bolts that clear the plate (entirely above it)', () => {
  const parts = [
    box('plate', 0, 0, 0, 400, 25, 400),         // plate vertical band y ∈ [-12.5,12.5]
    box('bolt', -120, 400, -120, 24, 250, 24),   // bolts y ∈ [275,525] — never overlap the plate
    box('bolt', 120, 400, 120, 24, 250, 24),
  ];
  assert.equal(recognizeBasePlate(parts, ['C']), null);
});

test('rejects a rectangular grid with unequal per-axis margins (single edge cannot reproduce)', () => {
  // 500×300 plate, 300×150 grid → X margin 100, Y margin 75. expandBasePlate's single edge distance would
  // re-derive the X bolts at ±175 instead of ±150, so recognition must decline (→ faithful mesh).
  const parts = [box('plate', 0, 100, 0, 500, 25, 300)];
  for (const x of [-150, 150]) for (const z of [-75, 75]) parts.push(cyl('bolt', x, 100, z, 24, 250));
  assert.equal(recognizeBasePlate(parts, ['C']), null);
});

test('recognizes a rectangular plate whose symmetric grid IS reproducible', () => {
  // 500×300 plate with equal 80 mm margins both axes: X bolts ±170, Y bolts ±70 → single edge 80 rebuilds it.
  const parts = [box('plate', 0, 100, 0, 500, 25, 300)];
  for (const x of [-170, 170]) for (const z of [-70, 70]) parts.push(cyl('bolt', x, 100, z, 24, 250));
  const r = recognizeBasePlate(parts, ['C']);
  assert.ok(r, 'equal-margin rectangular grid should recognize');
  assert.deepEqual(r.params, { thickness: 25, plateWidth: 500, plateDepth: 300, boltDia: 24, boltCols: 2, boltRows: 2, edgeDist: 80 });
});

test('rejects a tiny fit outside base-plate fabrication ranges', () => {
  const parts = [
    box('plate', 0, 0, 0, 40, 5, 40),      // 40×40 — too small for a base plate (< 60 mm)
    box('bolt', 0, 0, -10, 6, 60, 6),      // clean 1×2 grid so it reaches the fabrication-range gate
    box('bolt', 0, 0, 10, 6, 60, 6),
  ];
  assert.equal(recognizeBasePlate(parts, ['D']), null);
});

// --- Shear / fin-plate recognition (recognizeShearPlate) ---------------------------------------------

// A cylinder whose LONG axis is X (axis 0) — a horizontal shear/fin-plate bolt. `sides` vertices ON the
// circle at each end cap (the circle lies in the Y-Z plane), mirroring how web-ifc tessellates a circle.
function cylX(role, cx, cy, cz, dia, len, sides = 16) {
  const r = dia / 2, pos = [];
  for (const sx of [-1, 1]) for (let k = 0; k < sides; k++) {
    const t = (2 * Math.PI * k) / sides;
    pos.push(cx + sx * len / 2, cy + r * Math.cos(t), cz + r * Math.sin(t));
  }
  return { role, positions: pos, indices: [] };
}
// A canonical fin plate: vertical plate thin in X (n=0), 210 tall (Y), 120 wide along the beam (Z), with a
// single vertical line of 3 M20 bolts (rows on Y at pitch 70, one col on Z), all piercing the plate.
function finParts() {
  return [
    box('plate', 0, 0, 0, 10, 210, 120),
    cylX('bolt', 0, -70, 0, 20, 60), cylX('bolt', 0, 0, 0, 20, 60), cylX('bolt', 0, 70, 0, 20, 60),
  ];
}

test('recognizes a vertical fin plate with a 1×3 horizontal-bolt line', () => {
  const r = recognizeShearPlate(finParts(), ['BEAM-GUID']);
  assert.ok(r, 'should recognize');
  assert.equal(r.kind, 'shear-plate');
  assert.deepEqual(r.params, {
    plateThickness: 10, plateHeight: 210, plateWidth: 120,
    boltDia: 20, boltCols: 1, boltRows: 3, boltPitch: 70, edgeDist: 35,
  });
});

test('shear-plate recognition rejects a base plate (vertical anchors) — mutually exclusive', () => {
  const base = [box('plate', 0, 0, 0, 400, 25, 400),
    cyl('bolt', -120, 0, -120, 24, 250), cyl('bolt', 120, 0, -120, 24, 250),
    cyl('bolt', -120, 0, 120, 24, 250), cyl('bolt', 120, 0, 120, 24, 250)];
  assert.equal(recognizeShearPlate(base, ['X']), null);
});

test('rejects bolts that do not pierce the fin plate (off to the side)', () => {
  const parts = [box('plate', 0, 0, 0, 10, 210, 120),
    cylX('bolt', 0, -70, 400, 20, 60), cylX('bolt', 0, 0, 400, 20, 60), cylX('bolt', 0, 70, 400, 20, 60)];
  assert.equal(recognizeShearPlate(parts, ['B']), null);
});

test('recognizes a double-column fin plate (per-axis boltColPitch)', () => {
  // 2 columns (Z=±40 → 80 mm apart) × 3 rows (Y pitch 70) on a 200-wide plate — the multi-column ceiling.
  const parts = [box('plate', 0, 0, 0, 10, 210, 200),
    cylX('bolt', 0, -70, -40, 20, 60), cylX('bolt', 0, 0, -40, 20, 60), cylX('bolt', 0, 70, -40, 20, 60),
    cylX('bolt', 0, -70, 40, 20, 60), cylX('bolt', 0, 0, 40, 20, 60), cylX('bolt', 0, 70, 40, 20, 60)];
  const r = recognizeShearPlate(parts, ['B']);
  assert.ok(r, 'a reproducible 2-column fin plate should recognize');
  assert.deepEqual(r.params, { plateThickness: 10, plateHeight: 210, plateWidth: 200, boltDia: 20, boltCols: 2, boltRows: 3, boltPitch: 70, edgeDist: 35, boltColPitch: 80 });
});

// Rotate a part's positions about the VERTICAL axis (axis 1) by `deg` degrees (in-plane skew).
function rotV(part, deg) {
  const a = (deg * Math.PI) / 180, c = Math.cos(a), s = Math.sin(a), p = part.positions, out = [];
  for (let i = 0; i + 2 < p.length; i += 3) {
    const x = p[i], y = p[i + 1], z = p[i + 2];
    out.push(x * c - z * s, y, x * s + z * c);
  }
  return { role: part.role, positions: out, indices: [] };
}

test('recognizes a fin plate YAWED 10° about the vertical axis (OBB, was the ceiling)', () => {
  const parts = finParts().map((p) => rotV(p, 10)); // a rigid 10° yaw — OBB removes it, recipe is unchanged
  const r = recognizeShearPlate(parts, ['B']);
  assert.ok(r, 'a rigidly yawed fin plate should now recognize');
  assert.deepEqual(r.params, { plateThickness: 10, plateHeight: 210, plateWidth: 120, boltDia: 20, boltCols: 1, boltRows: 3, boltPitch: 70, edgeDist: 35 });
});

test('rejects a fin plate whose single bolt line is off-centre along the beam (engine re-centres it)', () => {
  // Bolts shifted +40 mm along the beam (uAx) on a 120 mm plate: still pierce the plate with a positive side
  // margin, but the engine would rebuild the line ~centred → reject to faithful mesh instead of moving bolts.
  const parts = [box('plate', 0, 0, 0, 10, 210, 120),
    cylX('bolt', 0, -70, 40, 20, 60), cylX('bolt', 0, 0, 40, 20, 60), cylX('bolt', 0, 70, 40, 20, 60)];
  assert.equal(recognizeShearPlate(parts, ['B']), null);
});

test('rejects a non-uniform vertical pitch (single-pitch model cannot reproduce it)', () => {
  const parts = [box('plate', 0, 0, 0, 10, 260, 120),
    cylX('bolt', 0, -80, 0, 20, 60), cylX('bolt', 0, 0, 0, 20, 60), cylX('bolt', 0, 100, 0, 20, 60)];
  assert.equal(recognizeShearPlate(parts, ['B']), null);
});

test('rejects a tiny fit outside fin-plate fabrication ranges', () => {
  const parts = [box('plate', 0, 0, 0, 2, 30, 30),
    cylX('bolt', 0, -8, 0, 3, 20), cylX('bolt', 0, 8, 0, 3, 20)];
  assert.equal(recognizeShearPlate(parts, ['D']), null);
});

test('base-plate recognition still rejects a fin plate (horizontal bolts) — mutually exclusive', () => {
  assert.equal(recognizeBasePlate(finParts(), ['B']), null);
});

// --- Oriented-fit geometry helpers (convexHull2d / minAreaRectAngle / rotateAboutVertical) ------------

const rot2 = (x, z, deg) => { const a = (deg * Math.PI) / 180, c = Math.cos(a), s = Math.sin(a); return [x * c - z * s, x * s + z * c]; };
// The 4 corners of a `w`×`d` rectangle centred at origin, yawed `deg` in the plane.
const rectCorners = (w, d, deg) => [[-w / 2, -d / 2], [w / 2, -d / 2], [w / 2, d / 2], [-w / 2, d / 2]].map(([x, z]) => rot2(x, z, deg));

test('convexHull2d drops interior points and needs 3 non-collinear', () => {
  const h = convexHull2d([[0, 0], [10, 0], [10, 10], [0, 10], [5, 5]]); // 4 corners + centre
  assert.equal(h.length, 4);
  assert.equal(convexHull2d([[0, 0], [1, 1]]), null);                    // < 3
  assert.equal(convexHull2d([[0, 0], [1, 1], [2, 2], [3, 3]]), null);    // collinear
});

test('minAreaRectAngle is ~0 for an axis-aligned rectangle', () => {
  assert.ok(Math.abs(minAreaRectAngle(rectCorners(400, 200, 0))) < 1e-6);
});

test('minAreaRectAngle recovers a 30° yaw (mod 90°, into ±45°)', () => {
  assert.ok(Math.abs(minAreaRectAngle(rectCorners(400, 200, 30)) - Math.PI / 6) < 1e-6);
});

test('minAreaRectAngle maps 44°/46°/90°/135° into the canonical quarter-turn', () => {
  const near = (a, b) => Math.abs(a - b) < 1e-6;
  assert.ok(near(minAreaRectAngle(rectCorners(400, 200, 44)), 44 * Math.PI / 180));
  assert.ok(near(minAreaRectAngle(rectCorners(400, 200, 46)), -44 * Math.PI / 180)); // 46° → −44° (perp edge)
  assert.ok(Math.abs(minAreaRectAngle(rectCorners(400, 200, 90))) < 1e-6);           // 90° ≡ aligned
  assert.ok(near(minAreaRectAngle(rectCorners(400, 200, 135)), 45 * Math.PI / 180)); // 135° ≡ 45°
});

test('rotateAboutVertical(−θ) axis-aligns a yawed plate part (extents recover the true rectangle)', () => {
  const yaw = 30;
  const part = box('plate', 0, 100, 0, 400, 25, 200); // aligned 400(x)×25(y)×200(z)
  const yawed = rotateAboutVertical([part], (yaw * Math.PI) / 180, 1)[0];
  const back = rotateAboutVertical([yawed], -(yaw * Math.PI) / 180, 1)[0];
  const b = partBoxLocal(back.positions);
  assert.ok(Math.abs(b.ext[0] - 400) < 1e-6 && Math.abs(b.ext[2] - 200) < 1e-6, 'round-trips to the aligned extents');
  // and a real recognizer would derive θ from the yawed footprint then undo it:
  const yb = partBoxLocal(yawed.positions);
  assert.ok(yb.ext[0] > 400 && yb.ext[2] > 200, 'the yawed world-AABB over-reads (why OBB is needed)');
  const theta = minAreaRectAngle(footprintXZ(yawed.positions));
  const aligned = rotateAboutVertical([yawed], -theta, 1)[0];
  const ab = partBoxLocal(aligned.positions);
  assert.ok(Math.abs(ab.ext[0] - 400) < 1e-3 && Math.abs(ab.ext[2] - 200) < 1e-3, 'OBB-derived θ re-aligns to true dims');
});

// local helpers for the geometry tests (avoid importing the non-exported partBox)
function partBoxLocal(pos) {
  const min = [Infinity, Infinity, Infinity], max = [-Infinity, -Infinity, -Infinity];
  for (let i = 0; i + 2 < pos.length; i += 3) for (let k = 0; k < 3; k++) { const v = pos[i + k]; if (v < min[k]) min[k] = v; if (v > max[k]) max[k] = v; }
  return { ext: [max[0] - min[0], max[1] - min[1], max[2] - min[2]] };
}
function footprintXZ(pos) { const out = []; for (let i = 0; i + 2 < pos.length; i += 3) out.push([pos[i], pos[i + 2]]); return out; }

// --- Indexed-mesh helpers (exercise the boundary-loop rectangularity path) ----------------------------
// Ear-clip a simple CCW polygon → triangles (index-triples into the polygon).
function earClip(poly) {
  const area2 = (a, b, c) => (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
  const inTri = (p, a, b, c) => { const d1 = area2(p, a, b), d2 = area2(p, b, c), d3 = area2(p, c, a); return !((d1 < 0 || d2 < 0 || d3 < 0) && (d1 > 0 || d2 > 0 || d3 > 0)); };
  const idx = [...poly.keys()], tris = [];
  let guard = 0;
  while (idx.length > 3 && guard++ < 4000) {
    let clipped = false;
    for (let i = 0; i < idx.length; i++) {
      const pa = idx[(i - 1 + idx.length) % idx.length], pb = idx[i], pc = idx[(i + 1) % idx.length];
      const a = poly[pa], b = poly[pb], c = poly[pc];
      if (area2(a, b, c) <= 0) continue; // reflex/collinear ear
      let ok = true;
      for (const pj of idx) { if (pj === pa || pj === pb || pj === pc) continue; if (inTri(poly[pj], a, b, c)) { ok = false; break; } }
      if (ok) { tris.push([pa, pb, pc]); idx.splice(i, 1); clipped = true; break; }
    }
    if (!clipped) break;
  }
  if (idx.length === 3) tris.push([idx[0], idx[1], idx[2]]);
  return tris;
}
// A closed thin prism from a CCW face outline [[A,C]…] (in the two non-`thinAxis` axes), extruded ±t/2 along
// `thinAxis`, centred at `ctr`=[x,y,z]. Real positions + indices → drives the boundary-loop gate.
function platePrism(role, outline, thinAxis, t, ctr = [0, 0, 0]) {
  const [a, c] = [0, 1, 2].filter((k) => k !== thinAxis);
  const N = outline.length, positions = [];
  const vert = (A, C, s) => { const p = [ctr[0], ctr[1], ctr[2]]; p[a] += A; p[c] += C; p[thinAxis] += s * t / 2; positions.push(p[0], p[1], p[2]); };
  for (const [A, C] of outline) vert(A, C, +1);  // top vertices 0..N-1
  for (const [A, C] of outline) vert(A, C, -1);  // bottom vertices N..2N-1
  const indices = [];
  for (const [i, j, k] of earClip(outline)) { indices.push(i, j, k); indices.push(N + i, N + k, N + j); }
  for (let i = 0; i < N; i++) { const j = (i + 1) % N; indices.push(i, j, N + j, i, N + j, N + i); }
  return { role, positions, indices };
}
const rectOutline = (w, d) => [[-w / 2, -d / 2], [w / 2, -d / 2], [w / 2, d / 2], [-w / 2, d / 2]];
// De-index a mesh so every triangle owns its 3 vertices (no shared indices) — exactly how web-ifc emits
// geometry. The rectangularity gate must match shared edges by POSITION, not vertex index, to survive this.
function deindex(part) {
  const pos = [], idx = [];
  for (let t = 0; t + 2 < part.indices.length; t += 3) for (let e = 0; e < 3; e++) {
    const vi = part.indices[t + e]; pos.push(part.positions[vi * 3], part.positions[vi * 3 + 1], part.positions[vi * 3 + 2]); idx.push(idx.length);
  }
  return { role: part.role, positions: pos, indices: idx };
}

// --- OBB: a rotated base plate now recognizes (the ceiling this slice lifts) ---------------------------

test('recognizes a base plate YAWED 30° about the vertical axis (OBB, equal margins)', () => {
  const parts = [box('plate', 0, 100, 0, 400, 25, 200)];        // 400×200, thin in Y
  for (const x of [-140, 140]) for (const z of [-40, 40]) parts.push(cyl('bolt', x, 100, z, 24, 250)); // equal 60 margins
  const yawed = rotateAboutVertical(parts, (30 * Math.PI) / 180, 1);
  const r = recognizeBasePlate(yawed, ['COL', 'X']);
  assert.ok(r, 'a rotated base plate should recognize (was the ceiling)');
  assert.deepEqual(r.params, { thickness: 25, plateWidth: 400, plateDepth: 200, boltDia: 24, boltCols: 2, boltRows: 2, edgeDist: 60 });
});

// --- Adversarial cases the Codex plan-review demanded (rectangularity / selection / fabrication) --------

const baseAnchors = () => { const p = []; for (const x of [-140, 140]) for (const z of [-40, 40]) p.push(cyl('bolt', x, 0, z, 24, 250)); return p; };

test('recognizes a clean INDEXED base plate (boundary-loop path, ~0 deviation)', () => {
  const parts = [platePrism('plate', rectOutline(400, 200), 1, 25), ...baseAnchors()];
  const r = recognizeBasePlate(parts, ['COL']);
  assert.ok(r, 'a clean indexed plate must pass the boundary-loop gate');
  assert.deepEqual(r.params, { thickness: 25, plateWidth: 400, plateDepth: 200, boltDia: 24, boltCols: 2, boltRows: 2, edgeDist: 60 });
});

test('rejects a PARALLELOGRAM base plate (sheared 40 mm — corner residual)', () => {
  const skew = [[-220, -100], [180, -100], [220, 100], [-180, 100]]; // 400×200 sheared 40 mm, no indices → hull fallback
  const parts = [{ role: 'plate', positions: platePrism('plate', skew, 1, 25).positions, indices: [] }, ...baseAnchors()];
  assert.equal(recognizeBasePlate(parts, ['C']), null);
});

test('rejects a base plate with a thin deep EDGE SLOT (10×150 — boundary loop, not area)', () => {
  const slot = [[-200, -100], [200, -100], [200, 100], [5, 100], [5, -50], [-5, -50], [-5, 100], [-200, 100]];
  const parts = [platePrism('plate', slot, 1, 25), ...baseAnchors()];
  assert.equal(recognizeBasePlate(parts, ['C']), null); // 1.9% area lost but 150 mm inward → reject
});

test('rejects a base plate with a large 40 mm corner CHAMFER (continuous, not vertex-sampled)', () => {
  const cham = [[-200, -100], [200, -100], [200, 100], [-160, 100], [-200, 60]]; // TL corner chamfered 40 mm
  const parts = [platePrism('plate', cham, 1, 25), ...baseAnchors()];
  assert.equal(recognizeBasePlate(parts, ['C']), null); // endpoints ON the edges; midpoint ~28 mm in → reject
});

test('base recipe is INVARIANT across yaw quadrants 0/44/46/90/135°', () => {
  const base = [box('plate', 0, 100, 0, 400, 25, 200)];
  for (const x of [-140, 140]) for (const z of [-40, 40]) base.push(cyl('bolt', x, 100, z, 24, 250));
  const recipeAt = (deg) => recognizeBasePlate(rotateAboutVertical(base, (deg * Math.PI) / 180, 1), ['C']).params;
  const ref = recipeAt(0);
  for (const deg of [44, 46, 90, 135]) assert.deepEqual(recipeAt(deg), ref, `yaw ${deg}° must give the same recipe`);
});

test('base candidate iteration skips a larger UNPIERCED cover plate', () => {
  const parts = [box('plate', 0, 500, 0, 600, 25, 600), box('plate', 0, 100, 0, 400, 25, 200)]; // cover above, base below
  for (const x of [-140, 140]) for (const z of [-40, 40]) parts.push(cyl('bolt', x, 100, z, 24, 250)); // pierce only the base
  const r = recognizeBasePlate(parts, ['C']);
  assert.ok(r && r.params.plateWidth === 400, 'should fit the pierced base, not the larger cover');
});

test('rejects a near-coincident 2-column fin plate (< 2·boltDia — bolts overlap)', () => {
  const parts = [box('plate', 0, 0, 0, 10, 210, 120)];
  for (const y of [-70, 0, 70]) for (const z of [-5.5, 5.5]) parts.push(cylX('bolt', 0, y, z, 20, 60)); // 11 mm apart < 40
  assert.equal(recognizeShearPlate(parts, ['B']), null);
});

test('fin θ comes from the bolts — an orthogonal larger doubler does not hijack the frame', () => {
  const parts = [box('plate', 0, 0, 0, 10, 210, 120),                 // the fin: thin in X (n=0)
    box('plate', 300, 0, 0, 260, 260, 10),                            // a LARGER doubler thin in Z (⟂ the fin)
    cylX('bolt', 0, -70, 0, 20, 60), cylX('bolt', 0, 0, 0, 20, 60), cylX('bolt', 0, 70, 0, 20, 60)];
  const r = recognizeShearPlate(parts, ['B']);
  assert.ok(r && r.params.plateWidth === 120 && r.params.boltRows === 3, 'the fin recognizes, not the doubler');
});

test('rejects a fin plate whose bolts are TILTED 20° from horizontal', () => {
  const rotZ = (p, deg) => { const a = (deg * Math.PI) / 180, c = Math.cos(a), s = Math.sin(a), o = []; for (let i = 0; i + 2 < p.positions.length; i += 3) { const x = p.positions[i], y = p.positions[i + 1]; o.push(x * c - y * s, x * s + y * c, p.positions[i + 2]); } return { role: p.role, positions: o, indices: [] }; };
  const parts = [box('plate', 0, 0, 0, 10, 210, 120),
    ...[-70, 0, 70].map((y) => rotZ(cylX('bolt', 0, y, 0, 20, 60), 20))]; // tilt each bolt 20° about Z
  assert.equal(recognizeShearPlate(parts, ['B']), null);
});

test('base recipe stays canonical for a SQUARE plate under yaw (grid tie-break, cols≠rows)', () => {
  const base = [box('plate', 0, 100, 0, 400, 25, 400)];
  for (const x of [-140, 140]) for (const z of [-140, 0, 140]) base.push(cyl('bolt', x, 100, z, 24, 250)); // 2 cols × 3 rows, equal 60 margins
  const at = (deg) => recognizeBasePlate(rotateAboutVertical(base, (deg * Math.PI) / 180, 1), ['C']).params;
  const ref = at(0);
  assert.ok(ref && ref.boltCols === 2 && ref.boltRows === 3, 'square 2×3 recognizes');
  for (const deg of [90, 180, 270]) assert.deepEqual(at(deg), ref, `square plate at ${deg}° must give the same cols×rows`);
});

test('rejects a base plate whose anchor HOLE hangs off the edge (centre-only "inside" would pass)', () => {
  const parts = [box('plate', 0, 100, 0, 400, 25, 200)];
  for (const x of [-199, 199]) for (const z of [-99, 99]) parts.push(cyl('bolt', x, 100, z, 24, 250)); // centres 1 mm from edge
  assert.equal(recognizeBasePlate(parts, ['C']), null); // edgeDist 1 < boltDia/2+2 → mesh
});

test('rejects a base grid with MIXED anchor diameters (median would silently homogenise)', () => {
  const parts = [box('plate', 0, 100, 0, 400, 25, 200)];
  for (const [x, z] of [[-140, -40], [140, -40], [-140, 40]]) parts.push(cyl('bolt', x, 100, z, 24, 250));
  parts.push(cyl('bolt', 140, 100, 40, 30, 250)); // one M30 among three M24
  assert.equal(recognizeBasePlate(parts, ['C']), null);
});

test('rejects a NON-UNIFORM 3-column fin plate (gaps average to a pitch but the grid is irregular)', () => {
  const parts = [box('plate', 0, 0, 0, 10, 210, 300)]; // columns 40/60 apart → median 50 passes the loose cluster tol
  for (const z of [-40.317, -0.317, 59.683]) for (const y of [-70, 0, 70]) parts.push(cylX('bolt', 0, y, z, 20, 60));
  assert.equal(recognizeShearPlate(parts, ['B']), null); // engine-relative pattern gate catches the moved middle column
});

test('measures the SHANK Ø of a HEADED fastener, not the head (axial-slice min radius)', () => {
  // Ø24 shank + a larger Ø40 head + a chamfered Ø16 tip + triangulated cap-CENTRE vertices: a mean over-reads,
  // a minimum under-reads — the modal (persistent) radius must still emit M24.
  function anchorHead(cx, cz) {
    const pos = [], rs = 12, rh = 20, rt = 8, h = 250, s = 16;
    for (const sy of [-1, 1]) for (let k = 0; k < s; k++) { const t = 2 * Math.PI * k / s; pos.push(cx + rs * Math.cos(t), 100 + sy * h / 2, cz + rs * Math.sin(t)); }
    for (const dy of [12, 15, 18, 21]) for (let k = 0; k < s; k++) { const t = 2 * Math.PI * k / s; pos.push(cx + rh * Math.cos(t), 100 + h / 2 + dy, cz + rh * Math.sin(t)); } // Ø40 head — FOUR rings (more verts than the shank; span must still win)
    for (let k = 0; k < s; k++) { const t = 2 * Math.PI * k / s; pos.push(cx + rt * Math.cos(t), 100 - h / 2 - 8, cz + rt * Math.sin(t)); }   // Ø16 chamfered tip
    pos.push(cx, 100 + h / 2, cz, cx, 100 - h / 2, cz); // triangulated cap-centre vertices (r≈0)
    return { role: 'bolt', positions: pos, indices: [] };
  }
  const parts = [box('plate', 0, 100, 0, 400, 25, 200)];
  for (const x of [-140, 140]) for (const z of [-40, 40]) parts.push(anchorHead(x, z));
  const r = recognizeBasePlate(parts, ['C']);
  assert.ok(r, 'a headed anchor grid should recognize');
  assert.equal(r.params.boltDia, 24, 'boltDia is the shank Ø, not contaminated by the head');
});

test('rejects a fastener with equal washers at BOTH ends — shank Ø ambiguous → faithful mesh', () => {
  function bothWashers(cx, cz) {
    const pos = [], rs = 10, rw = 20, h = 250, s = 16;
    for (const sy of [-1, 1]) for (let k = 0; k < s; k++) { const t = 2 * Math.PI * k / s; pos.push(cx + rs * Math.cos(t), 100 + sy * h / 2, cz + rs * Math.sin(t)); }             // shank ends
    for (const sy of [-1, 1]) for (let k = 0; k < s; k++) { const t = 2 * Math.PI * k / s; pos.push(cx + rw * Math.cos(t), 100 + sy * (h / 2 + 8), cz + rw * Math.sin(t)); }     // a washer at EACH end
    return { role: 'bolt', positions: pos, indices: [] };
  }
  const parts = [box('plate', 0, 100, 0, 400, 25, 200)];
  for (const x of [-140, 140]) for (const z of [-40, 40]) parts.push(bothWashers(x, z));
  assert.equal(recognizeBasePlate(parts, ['C']), null); // shank (Ø20) vs washer (Ø40) both span the length → mesh
});

test('rejects SQUARE-prism anchors posing as round bolts (a regular polygon fools a vertex-radius check)', () => {
  const parts = [box('plate', 0, 100, 0, 400, 25, 200)];
  for (const x of [-140, 140]) for (const z of [-40, 40]) parts.push(box('bolt', x, 100, z, 24, 250, 24)); // 24×24 SQUARE prism, vertical
  assert.equal(recognizeBasePlate(parts, ['C']), null); // crossSectionCircular: 4 hull corners < 8 sides → mesh
});

test('rejects a TAPERED base plate (frustum — one-face validation would emit a full prism)', () => {
  // top face 400×200, bottom 300×100, thin in Y — a valid 2×2 anchor grid; must fall back to mesh.
  const pos = [];
  for (const [w, d, sy] of [[400, 200, 1], [300, 100, -1]]) for (const sx of [-1, 1]) for (const sz of [-1, 1]) pos.push(sx * w / 2, sy * 12.5, sz * d / 2);
  const parts = [{ role: 'plate', positions: pos, indices: [] }];
  for (const x of [-100, 100]) for (const z of [-30, 30]) parts.push(cyl('bolt', x, 0, z, 24, 250));
  assert.equal(recognizeBasePlate(parts, ['C']), null);
});

test('rejects a ZIGZAG fin grid whose row/column MEANS still average out (per-bolt reproduction)', () => {
  // 2 cols × 3 rows, but left column 5 mm low and right column 5 mm high — cluster means stay [-70,0,70]
  // and every cell is occupied, yet each bolt is 5 mm off its engine cell → reject (was accepted on means).
  const parts = [box('plate', 0, 0, 0, 10, 210, 200)];
  for (const [z, dv] of [[-40, -5], [40, 5]]) for (const y of [-70, 0, 70]) parts.push(cylX('bolt', 0, y + dv, z, 20, 60));
  assert.equal(recognizeShearPlate(parts, ['B']), null);
});

test('base recipe stays canonical for a NEAR-square plate (400×395) under 90° yaw', () => {
  const base = [box('plate', 0, 100, 0, 400, 25, 395)];
  for (const x of [-140, 140]) for (const z of [-137, 137]) base.push(cyl('bolt', x, 100, z, 24, 250)); // 2×2, equal-ish margins
  const at = (deg) => { const r = recognizeBasePlate(rotateAboutVertical(base, (deg * Math.PI) / 180, 1), ['C']); return r && r.params; };
  const ref = at(0);
  assert.ok(ref, 'near-square recognizes');
  assert.deepEqual(at(90), ref, 'near-square at 90° must give the same width/depth (not flipped)');
});

test('boundary-loop gate survives web-ifc-style DE-INDEXED meshes (edges matched by position)', () => {
  // web-ifc duplicates vertices per triangle, so no two triangles share an index — the gate must key edges
  // by position or every edge looks like a boundary (this exact bug fell out of the real 2-col fin fixture).
  const clean = [deindex(platePrism('plate', rectOutline(400, 200), 1, 25)), ...baseAnchors()];
  assert.ok(recognizeBasePlate(clean, ['C']), 'a clean de-indexed plate must still recognize');
  const slot = [[-200, -100], [200, -100], [200, 100], [5, 100], [5, -50], [-5, -50], [-5, 100], [-200, 100]];
  const notched = [deindex(platePrism('plate', slot, 1, 25)), ...baseAnchors()];
  assert.equal(recognizeBasePlate(notched, ['C']), null, 'a de-indexed slotted plate must still reject');
});
