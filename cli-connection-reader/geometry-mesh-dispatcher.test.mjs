import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

const fixture = JSON.parse(readFileSync(new URL('../geometry-worker/fixtures/flush-stress.json', import.meta.url), 'utf8'));

function operand(id, xy, z, offset, flipDiagonal = false) {
  const positions_mm = z.flatMap(level => xy.map(([x, y]) => [x, y, level]));
  const triangles = [];
  const n = xy.length;
  for (let i = 1; i < n - 1; i += 1) triangles.push([0, i + 1, i], [n, n + i, n + i + 1]);
  for (let i = 0; i < n; i += 1) {
    const j = (i + 1) % n;
    if (flipDiagonal) triangles.push([i, j, i + n], [j, j + n, i + n]);
    else triangles.push([i, j, j + n], [i, j + n, i + n]);
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

const nativeFaces = mesh => mesh.triangles.map(tri => [tri.map(i => mesh.positions_mm[i])]);

function cavityCandidate(outer, cutter) {
  const count = outer.mesh.positions_mm.length;
  return {
    positions_mm: [...outer.mesh.positions_mm, ...cutter.mesh.positions_mm],
    triangles: [...outer.mesh.triangles, ...cutter.mesh.triangles.map(face => face.map(index => index + count).reverse())],
  };
}

test('seeded_prism_slices_are_independent_of_side_diagonals', async () => {
  const { verifyExact } = await import('./geometry-mesh-dispatcher.mjs');
  const seed = 872;
  let state = seed;
  const integer = (min, max) => {
    state = (Math.imul(state, 1664525) + 1013904223) >>> 0;
    return min + Math.floor(state / 2 ** 32 * (max - min + 1));
  };
  const square = [[0, 0], [100, 0], [100, 100], [0, 100]];
  const origin = [0, 0, 0];
  for (const rotated of [false, true]) {
    for (let sample = 0; sample < 64; sample += 1) {
      const x = integer(25, 75);
      const y = integer(25, 75);
      const u = integer(2, 8);
      const v = rotated ? integer(1, 7) : 0;
      const scale = integer(1, 2);
      const halfHeight = integer(2, 18);
      // Align the axis-aligned pocket's midpoint witnesses with the cube's
      // section through a side diagonal. The ray then meets a shared edge.
      const midZ = rotated ? integer(25, 75) : y;
      const z = [midZ - halfHeight, midZ + halfHeight];
      const xy = [[-1, -1], [1, -1], [1, 1], [-1, 1]]
        .map(([s, t]) => [x + s * u - t * v * scale, y + s * v + t * u * scale]);
      const volume = 1_000_000 - 8 * (u ** 2 + v ** 2) * scale * halfHeight;
      // Admission requires the producer's canonical triangulation.
      const a = operand('cube', square, [0, 100], origin);
      const b = operand('cutter', xy, z, origin);
      for (const flipDiagonal of [false, true]) {
        const outer = operand('cube', square, [0, 100], origin, flipDiagonal);
        const cutter = operand('cutter', xy, z, origin, flipDiagonal);
        const mesh = cavityCandidate(outer, cutter);
        const result = { offset_mm: origin, brep_b64: 'YQ==', mesh, volume_mm3: volume };
        const reply = verifyExact({ request: { a, b }, result, native_boundary: nativeFaces(mesh) });
        const context = JSON.stringify({ seed, rotated, sample, flipDiagonal, xy, z, code: reply.code });
        assert.equal(reply.status, 'ok', context);
      }
    }
  }
});

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
  assert.equal(verifyExact({request: {a, b}, result: {...result, mesh: original}, native_boundary: nativeFaces(original)}).status, 'ok');
  const reply = verifyExact({request: {a, b}, result, native_boundary: nativeFaces(shifted)});
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
  const reply = verifyExact({request: {a, b}, result, native_boundary: nativeFaces(native)});
  assert.equal(reply.code, 'brep_validation_failed');
  assert.equal(reply.result, null);
});

test('general_slabs_and_separated_components', async () => {
  const { cutMeshRequest } = await import('./geometry-mesh-dispatcher.mjs');
  const { a } = flush([0, 0, 0]);
  const strip = [[180, -20], [200, -20], [200, 270], [180, 270]];
  for (const [z, volume] of [[[0, 20], 1_900_000], [[6, 14], 1_960_000]]) {
    const reply = await cutMeshRequest({a, b: operand('strip', strip, z, [0, 0, 0])});
    assert.equal(reply.status, 'ok', JSON.stringify(reply));
    assert.ok(Math.abs(reply.result.volume_mm3 - volume) < 1e-8);
  }
  const b = operand('through hole', [[50, 50], [90, 50], [90, 100], [50, 100]], [0, 20], [0, 0, 0]);
  const reply = await cutMeshRequest({a, b});
  assert.equal(reply.status, 'ok', JSON.stringify(reply));
  assert.ok(Math.abs(reply.result.volume_mm3 - 1_960_000) < 1e-8);
});

test('thin_overlap_and_residual_clearance', async () => {
  const { cutMeshRequest, inspectAdmission } = await import('./geometry-mesh-dispatcher.mjs');
  const xy = [[0, 0], [100, 0], [100, 100], [0, 100]];
  const a = operand('plate', xy, [0, 100], [0, 0, 0]);
  for (const kind of ['overlap', 'residual']) {
    const pair = width => {
      const z = kind === 'overlap' ? [100 - width, 200 - width] : [width, 100 + width];
      const b = operand('clip', xy, [0, 100], [0, 0, z[0]]);
      b.profile.z_mm = z.map(v => v.toFixed(12));
      return {a, b};
    };
    const threshold = inspectAdmission(pair(0.01)).threshold_mm[2];
    for (const [width, admitted] of [[1e-8, false], [0.95 * threshold, false], [1.05 * threshold, true]]) {
      const request = pair(width);
      const admission = inspectAdmission(request);
      assert.ok(Math.abs(admission.clearance_mm[2] - width) < 1e-10);
      assert.equal(admission.code, admitted ? null : 'budget_exceeded');
      let calls = 0;
      const reply = await cutMeshRequest(request, () => { calls += 1; });
      assert.equal(calls, Number(admitted));
      assert.equal(reply.status, admitted ? 'ok' : 'rejected', JSON.stringify(reply));
      if (!admitted) assert.equal(reply.result, null);
    }
  }
});



test('correlated_false_success', async () => {
  const { verifyExact } = await import('./geometry-mesh-dispatcher.mjs');
  const { a, b } = flush([0, 0, 0]);
  const unchanged = { offset_mm: a.offset_mm, brep_b64: 'YQ==', mesh: a.mesh, volume_mm3: 2_000_000 };
  assert.equal(unchanged.volume_mm3 + 0, 2_000_000); // Correlated empty Common would pass conservation.
  assert.equal(verifyExact({ request: {a, b}, result: unchanged, native_boundary: nativeFaces(a.mesh) }).code,
    'brep_validation_failed');
});

test('false_empty', async () => {
  const { verifyExact } = await import('./geometry-mesh-dispatcher.mjs');
  const request = flush([0, 0, 0]);
  const result = { offset_mm: [0, 0, 0], brep_b64: null, mesh: {positions_mm: [], triangles: []}, volume_mm3: 0 };
  assert.equal(verifyExact({ request, result, native_boundary: [] }).code, 'brep_validation_failed');
});

test('confirmed_empty', async () => {
  const { cutMeshRequest } = await import('./geometry-mesh-dispatcher.mjs');
  const { a, b } = flush([0, 0, 0]);
  assert.equal((await cutMeshRequest({a: b, b: a})).result.volume_mm3, 0);
});

test('confirmed_unchanged', async () => {
  const { cutMeshRequest } = await import('./geometry-mesh-dispatcher.mjs');
  const request = flush([5000000, 0, 0], [-5000000, 0, 0]);
  let attempts = 0;
  const reply = await cutMeshRequest(request, () => { attempts += 1; });
  assert.equal(attempts, 0);
  assert.ok(Math.abs(reply.result.volume_mm3 - 2_000_000) < 1e-8);
});

test('duplicate_internal_cap_is_not_hidden_at_a_slab_transition', async () => {
  const { verifyExact } = await import('./geometry-mesh-dispatcher.mjs');
  const { a } = flush([0, 0, 0]);
  const b = pocket(50);
  const mesh = cavityCandidate(a, b);
  const native = nativeFaces(mesh);
  native.push(native.find(([face]) => face.every(point => point[2] === 6)));
  const result = { offset_mm: [0, 0, 0], brep_b64: 'YQ==', mesh, volume_mm3: 1_984_000 };
  assert.equal(verifyExact({ request: {a, b}, result, native_boundary: native }).code, 'brep_validation_failed');
});

test('thin_extra_shell_cannot_hide_inside_a_cap_allowance', async () => {
  const { verifyExact } = await import('./geometry-mesh-dispatcher.mjs');
  const xy = [[0, 0], [100, 0], [100, 100], [0, 100]];
  const a = operand('whole', xy, [0, 100], [0, 0, 0]);
  const b = operand('disjoint', xy, [0, 100], [500, 0, 0]);
  const lower = operand('lower', xy, [0, 100 - 3e-6], [0, 0, 0]).mesh;
  const upper = operand('upper', xy, [100 - 2e-6, 100], [0, 0, 0]).mesh;
  const mesh = { positions_mm: [...lower.positions_mm, ...upper.positions_mm],
    triangles: [...lower.triangles, ...upper.triangles.map(face => face.map(i => i + lower.positions_mm.length))] };
  const result = { offset_mm: [0, 0, 0], brep_b64: 'YQ==', mesh, volume_mm3: 1_000_000 };
  assert.equal(verifyExact({ request: {a, b}, result, native_boundary: nativeFaces(a.mesh) }).code,
    'brep_validation_failed');
});
