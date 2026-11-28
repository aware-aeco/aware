import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { loadClashEngine } from './snapshot-clash.mjs';

const adapter = () => import('./mesh-solid.mjs');

function tetra(delta = 0) {
  return {
    positions_mm: [[0, 0, 0], [1, 0, 0], [0, 1, 0], [0, 0, 1]].map(([x, y, z]) => [x + delta, y, z]),
    triangles: [[0, 2, 1], [0, 1, 3], [0, 3, 2], [1, 2, 3]],
  };
}

test('raw_difference', async () => {
  const { subtractMesh, validateMesh } = await adapter();
  const result = subtractMesh(await loadClashEngine(), tetra(), tetra(0.5));
  assert.ok(result.triangles.length > 0);
  assert.ok(Math.abs(validateMesh(result).volume - 7 / 48) < 1e-12);
});

test('invalid_mesh', async () => {
  const { validateMesh } = await adapter();
  let mesh = tetra();
  mesh.positions_mm[0][0] = Infinity;
  assert.throws(() => validateMesh(mesh), /finite position/);
  mesh = tetra();
  mesh.triangles[0][0] = 4;
  assert.throws(() => validateMesh(mesh), /index/);
  mesh = tetra();
  mesh.triangles.pop();
  assert.throws(() => validateMesh(mesh), /edge/);
  mesh = tetra();
  const other = tetra();
  mesh.positions_mm.push(...other.positions_mm.slice(1).map(p => p.map(v => -v)));
  mesh.triangles.push(...other.triangles.map(t => t.map(i => i === 0 ? 0 : i + 3).reverse()));
  assert.throws(() => validateMesh(mesh), /vertex fan/);
});

test('status_before_empty', async () => {
  const { readManifoldMesh } = await adapter();
  assert.throws(() => readManifoldMesh({ status: () => 'NotManifold', getMesh: () => assert.fail('read mesh before status') }), /NotManifold/);
});

test('float32_collapse', async () => {
  const { toFloat32Mesh } = await adapter();
  assert.throws(() => toFloat32Mesh(tetra(2 ** 24)), /collapse/);
});

test('uncentred_flush_baseline', async () => {
  const { replayFlushBaseline, sweepFlushBaseline } = await import('./geometry-flush-baseline.mjs');
  const fixture = JSON.parse(readFileSync(new URL('../geometry-worker/fixtures/flush-stress.json', import.meta.url)));
  const result = await replayFlushBaseline();
  assert.deepEqual(result.status, ['NoError', 'NoError', 'NoError']);
  assert.equal(result.reference_mm3, 1_973_000);
  assert.equal(result.failed_invariant, 'analytic_volume');
  assert.ok(Math.abs(result.member_volume_mm3 - result.reference_mm3) > 0.01);
  assert.equal(result.member_volume_mm3, fixture.raw_member_volume_mm3);
  assert.deepEqual(await sweepFlushBaseline(), {row: fixture.sweep_row, offset: fixture.offset_mm.map(Number), ...result});
});
