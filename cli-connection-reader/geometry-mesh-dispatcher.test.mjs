import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

const fixture = JSON.parse(readFileSync(new URL('../geometry-worker/fixtures/flush-stress.json', import.meta.url), 'utf8'));

function operand(id, xy, z, offset) {
  const positions_mm = z.flatMap(level => xy.map(([x, y]) => [x, y, level]));
  const triangles = [];
  const n = xy.length;
  for (let i = 1; i < n - 1; i += 1) triangles.push([0, i + 1, i], [n, n + i, n + i + 1]);
  for (let i = 0; i < n; i += 1) {
    const j = (i + 1) % n;
    triangles.push([i, j, j + n], [i, j + n, i + n]);
  }
  return { id, offset_mm: offset, brep_b64: null, mesh: { positions_mm, triangles }, profile: {
    xy_mm: xy.map(point => point.map((v, axis) => (v + offset[axis]).toFixed(4))),
    z_mm: z.map(v => (v + offset[2]).toFixed(4)),
  } };
}

function flush(aOffset, bOffset = aOffset, aX = 0) {
  return {
    a: operand('plate', fixture.plate_xy_mm.map(([x, y]) => [Number(x) + aX, Number(y)]), [0, 20], aOffset),
    b: operand('clip', fixture.clip_xy_mm.map(point => point.map(Number)), [0, 20], bOffset),
  };
}

test('centred_flush', async () => {
  const { cutMeshRequest } = await import('./geometry-mesh-dispatcher.mjs');
  for (const pair of [flush(fixture.offset_mm.map(Number)), flush([5e6, 0, 0], [5e6 + 0.125, 0, 0], 0.125)]) {
    const reply = await cutMeshRequest(pair);
    assert.equal(reply.status, 'ok', JSON.stringify(reply));
    assert.equal(reply.result.brep_b64, null);
    assert.ok(reply.result.mesh.triangles.length > 0);
    assert.ok(Math.abs(reply.result.volume_mm3 - fixture.retained_mm3) < 1e-8);
    const memberX = reply.result.mesh.positions_mm.map(([x]) => x + reply.result.offset_mm[0]);
    assert.ok(Math.abs(Math.min(...memberX) - Number(pair.a.profile.xy_mm[0][0])) < 1e-8);
    assert.ok(Math.abs(Math.max(...memberX) - Number(pair.a.profile.xy_mm[1][0])) < 1e-8);
  }
});

test('through_cut_residual_skin_mesh_admission', async () => {
  const { cutMeshRequest } = await import('./geometry-mesh-dispatcher.mjs');
  const pair = flush([0, 0, 0]);
  pair.b = operand('strip', [[180, -20], [200, -20], [200, 270], [180, 270]], [0, 20], [0, 0, 0]);
  for (const point of pair.b.mesh.positions_mm.slice(4)) point[2] -= 5e-6;
  const reply = await cutMeshRequest(pair);
  assert.equal(reply.status, 'rejected');
  assert.equal(reply.code, 'coincidence_mismatch');
  assert.equal(reply.relation, 'top_cap');
  assert.equal(reply.result, null);
});

test('mesh_disjoint_both_orders', async () => {
  const { cutMeshRequest } = await import('./geometry-mesh-dispatcher.mjs');
  const pair = flush([5e6, 0, 0], [-5e6, 0, 0]);
  assert.ok(Math.abs((await cutMeshRequest(pair)).result.volume_mm3 - 2_000_000) < 1e-8);
  assert.ok(Math.abs((await cutMeshRequest({a: pair.b, b: pair.a})).result.volume_mm3 - 27_000) < 1e-8);
});

test('enclosed_cavity_mesh', async () => {
  const { cutMeshRequest } = await import('./geometry-mesh-dispatcher.mjs');
  const { a } = flush([0, 0, 0]);
  const b = operand('internal pocket', [[50, 50], [90, 50], [90, 100], [50, 100]], [6, 14], [0, 0, 0]);
  const reply = await cutMeshRequest({a, b});
  assert.equal(reply.status, 'ok', JSON.stringify(reply));
  assert.ok(Math.abs(reply.result.volume_mm3 - 1_984_000) < 1e-8);
});

test('tangent_cavity_mesh', async () => {
  const { cutMeshRequest } = await import('./geometry-mesh-dispatcher.mjs');
  const { a } = flush([0, 0, 0]);
  const b = operand('tangent pocket', [[0, 100], [30, 70], [60, 100], [30, 130]], [6, 14], [0, 0, 0]);
  const reply = await cutMeshRequest({a, b});
  assert.equal(reply.code, 'mesh_invalid');
  assert.equal(reply.result, null);
});

function cavityCandidate(outer, cutter) {
  const count = outer.mesh.positions_mm.length;
  return {
    positions_mm: [...outer.mesh.positions_mm, ...cutter.mesh.positions_mm],
    triangles: [...outer.mesh.triangles, ...cutter.mesh.triangles.map(face => face.map(index => index + count).reverse())],
  };
}

function pocket(x) {
  return operand('pocket', [[x, 50], [x + 40, 50], [x + 40, 100], [x, 100]], [6, 14], [0, 0, 0]);
}

test('equal_volume_shifted_pocket_rejected_by_artifact_check', async () => {
  const { verifyExact } = await import('./geometry-mesh-dispatcher.mjs');
  const { a } = flush([0, 0, 0]);
  const b = pocket(50);
  const shifted = cavityCandidate(a, pocket(55));
  const original = cavityCandidate(a, b);
  const result = { offset_mm: [0, 0, 0], brep_b64: 'YQ==', mesh: shifted, volume_mm3: 1_984_000 };
  assert.equal(verifyExact({request: {a, b}, result: {...result, mesh: original}, native_boundary: original}).status, 'ok');
  const reply = verifyExact({request: {a, b}, result, native_boundary: shifted});
  assert.equal(reply.code, 'brep_validation_failed');
  assert.equal(reply.result, null);
});

test('shifted_exact_tessellation_rejected_by_artifact_check', async () => {
  const { verifyExact } = await import('./geometry-mesh-dispatcher.mjs');
  const { a } = flush([0, 0, 0]);
  const b = pocket(50);
  const native = cavityCandidate(a, b);
  const mesh = { ...native, positions_mm: native.positions_mm.map(([x, y, z]) => [x + 1, y, z]) };
  const result = { offset_mm: [0, 0, 0], brep_b64: 'YQ==', mesh, volume_mm3: 1_984_000 };
  const reply = verifyExact({request: {a, b}, result, native_boundary: native});
  assert.equal(reply.code, 'brep_validation_failed');
  assert.equal(reply.result, null);
});
