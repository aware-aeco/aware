// Tests for `extract` — ONE connection, tessellated for import, optionally fitted to a recipe.
//
// These exist because of a gap the #347 review found: `recognize.test.mjs` rotates its fixtures at
// the test seam, so it proves the RECOGNIZER is frame-consistent and says nothing about what
// `tessellate` actually emits. Reverting the rotation in `index.mjs` left all 105 tests green while
// every base plate silently read as a fin plate. The coupling between `tessellate`'s frame and
// `recognize.mjs`'s `VERTICAL` is the whole substance of #347, and it needs a test that executes
// both — which means running the real bridge against a real file.
//
// In-repo fixtures, so this runs everywhere.
import test from 'node:test';
import assert from 'node:assert/strict';
import { join } from 'node:path';
import { openApi, closeApi, listConnections, extractConnection, probeModel, toWebIfcYUp } from './index.mjs';

async function extractFirst(name) {
  const h = await openApi(join('test-fixtures', name));
  try {
    const list = listConnections(h.api, h.modelID);
    assert.ok(list.connections.length > 0, `${name}: fixture should carry a connection`);
    return {
      out: extractConnection(h.api, h.modelID, list.connections[0].id),
      probe: probeModel(h.api, h.modelID),
    };
  } finally {
    closeApi(h);
  }
}

const allPositions = (parts, map) => {
  const out = [];
  for (const p of parts) {
    for (let i = 0; i + 2 < p.positions.length; i += 3) {
      const v = map ? map([p.positions[i], p.positions[i + 1], p.positions[i + 2]])
                    : [p.positions[i], p.positions[i + 1], p.positions[i + 2]];
      out.push(v);
    }
  }
  return out;
};
const range = (pts, axis) => {
  let mn = Infinity, mx = -Infinity;
  for (const p of pts) { if (p[axis] < mn) mn = p[axis]; if (p[axis] > mx) mx = p[axis]; }
  return [mn, mx];
};
const overlaps = (a, b) => a[0] <= b[1] && b[0] <= a[1];

// Same three the #343 read-model frame test uses, plus the 2-column shear plate, so BOTH recipe
// kinds are covered — the discriminator that breaks if the rotation and `VERTICAL` disagree is
// exactly "are these bolts vertical", and only a base plate AND a fin plate together can catch an
// inversion in either direction.
const FRAME_FIXTURES = [
  ['baseplate-bp1.ifc', 'base-plate'],
  ['baseplate-rot.ifc', 'base-plate'],
  ['shearplate-sp1.ifc', 'shear-plate'],
  ['shearplate-2col.ifc', 'shear-plate'],
];

test('extract answers in the file\'s own Z-up frame (#347)', async () => {
  for (const [name] of FRAME_FIXTURES) {
    const { out, probe } = await extractFirst(name);
    assert.equal(out.connection.frame, 'z-up', `${name}: extract must STATE the frame it sent`);
    assert.ok(probe.bbox, `${name}: probe should establish a bbox`);
    const probeZ = [probe.bbox.min[2], probe.bbox.max[2]];
    const pts = allPositions(out.connection.parts);
    assert.ok(pts.length > 0, `${name}: extract should return mesh geometry`);

    // Height, not plan position: `baseplate-rot` is yawed 30° and every fixture sits at a site
    // offset, so a plan claim would be about placement composition rather than about the frame. The
    // vertical range is yaw-invariant, which isolates the one question — which axis is up?
    assert.ok(overlaps(range(pts, 2), probeZ),
      `${name}: extract spans Z ${range(pts, 2).map(Math.round)} but the file's own points span ` +
      `${probeZ.map(Math.round)} — extract is not answering in the file's frame`);

    // The arm that makes it discriminating: the SAME parts re-read in web-ifc's Y-up frame — exactly
    // what this command returned before #347 — must FAIL. Without it a fixture that happened to hold
    // in either frame would look like proof and prove nothing. (This is the shape #343's read-model
    // test established; the gap was that `extract` never got one.)
    const y = range(allPositions(out.connection.parts, toWebIfcYUp), 2);
    assert.ok(!overlaps(y, probeZ),
      `${name}: the pre-#347 Y-up reading spans Z ${y.map(Math.round)} and still overlaps the file's ` +
      `${probeZ.map(Math.round)} — this fixture cannot tell the two frames apart, so it does not ` +
      `belong in FRAME_FIXTURES`);
  }
});

test('the recognizer still fits BOTH kinds in that frame — the coupling #347 is about', async () => {
  // `recognize.mjs` discriminates a base plate from a fin plate by whether the bolts are vertical, so
  // rotating `tessellate` without moving `VERTICAL` (or the reverse) inverts the discriminator: every
  // base plate reads as a fin plate, or nothing recognizes at all. Neither shows up as an error —
  // `extract` just falls back to faithful mesh, quietly losing the recipe import.
  //
  // Asserting BOTH kinds is what makes this catch an inversion in either direction.
  for (const [name, kind] of FRAME_FIXTURES) {
    const { out } = await extractFirst(name);
    assert.ok(out.connection.recipe,
      `${name}: should still fit a recipe — a silent fall back to mesh is what a frame/VERTICAL ` +
      `mismatch looks like from outside`);
    assert.equal(out.connection.recipe.kind, kind, `${name}: wrong recipe kind`);
  }
});

test('the recipe is frame-independent, so the import path a consumer uses is untouched', async () => {
  // The params are scalars in millimetres that the consumer re-derives on its own member, so #347
  // moved no number a recipe import reads. Pinned because it is the load-bearing half of the
  // BREAKING.md's "if you import recipes, you are affected only on the mesh fallback path".
  const { out } = await extractFirst('shearplate-sp1.ifc');
  assert.deepEqual(out.connection.recipe.params, {
    plateThickness: 10, plateHeight: 210, plateWidth: 120,
    boltDia: 20, boltCols: 1, boltRows: 3, boltPitch: 70, edgeDist: 35,
  });
});
