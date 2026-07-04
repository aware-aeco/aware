// Unit tests for the pure base-plate recognition (recognize.mjs). Synthetic parts in web-ifc's Y-up
// frame (vertical = axis 1). `node --test`.
import test from 'node:test';
import assert from 'node:assert/strict';
import { recognizeBasePlate, recognizeShearPlate } from './recognize.mjs';

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

test('rejects a double-column fin plate (cols>1 not faithfully reproducible)', () => {
  const parts = [box('plate', 0, 0, 0, 10, 210, 200),
    cylX('bolt', 0, -70, -40, 20, 60), cylX('bolt', 0, 0, -40, 20, 60), cylX('bolt', 0, 70, -40, 20, 60),
    cylX('bolt', 0, -70, 40, 20, 60), cylX('bolt', 0, 0, 40, 20, 60), cylX('bolt', 0, 70, 40, 20, 60)];
  assert.equal(recognizeShearPlate(parts, ['B']), null);
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

test('rejects a fin plate skewed about the vertical axis (AABB thickness would over-read)', () => {
  const parts = finParts().map((p) => rotV(p, 10)); // a 10° in-plane skew inflates plate.ext[n] to ~31 mm
  assert.equal(recognizeShearPlate(parts, ['B']), null);
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
