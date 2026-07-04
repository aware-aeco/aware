// Unit tests for the pure base-plate recognition (recognize.mjs). Synthetic parts in web-ifc's Y-up
// frame (vertical = axis 1). `node --test`.
import test from 'node:test';
import assert from 'node:assert/strict';
import { recognizeBasePlate } from './recognize.mjs';

// An axis-aligned box part centred at (cx,cy,cz) with extents (ex,ey,ez). Only positions matter to
// recognition (it AABBs them); indices are irrelevant here.
function box(role, cx, cy, cz, ex, ey, ez) {
  const h = [ex / 2, ey / 2, ez / 2], pos = [];
  for (const sx of [-1, 1]) for (const sy of [-1, 1]) for (const sz of [-1, 1]) pos.push(cx + sx * h[0], cy + sy * h[1], cz + sz * h[2]);
  return { role, positions: pos, indices: [] };
}

test('recognizes a base plate with a 2×2 vertical anchor grid', () => {
  const parts = [
    box('plate', 0, 100, 0, 400, 25, 400),      // horizontal 400×400×25 (thin in Y)
    box('bolt', -120, 100, -120, 24, 250, 24),  // vertical anchors (long in Y) on a 240×240 grid
    box('bolt', 120, 100, -120, 24, 250, 24),
    box('bolt', -120, 100, 120, 24, 250, 24),
    box('bolt', 120, 100, 120, 24, 250, 24),
  ];
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

test('rejects a tiny fit outside base-plate fabrication ranges', () => {
  const parts = [
    box('plate', 0, 0, 0, 40, 5, 40),      // 40×40 — too small for a base plate
    box('bolt', -10, 0, -10, 6, 60, 6),
    box('bolt', 10, 0, 10, 6, 60, 6),
  ];
  assert.equal(recognizeBasePlate(parts, ['D']), null);
});
