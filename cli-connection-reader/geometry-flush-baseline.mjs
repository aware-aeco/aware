import { readFileSync } from 'node:fs';
import { loadClashEngine } from './snapshot-clash.mjs';
import { checkMesh } from './snapshot-clash-geometry.mjs';

const fixture = JSON.parse(readFileSync(new URL('../geometry-worker/fixtures/flush-stress.json', import.meta.url), 'utf8'));

function rawPrism(engine, polygon, offset) {
  const count = polygon.length;
  const vertices = fixture.z_mm.flatMap(z => polygon.flatMap(([x, y]) => [x, y, z].map((v, i) => Number(v) + offset[i])));
  const indices = [];
  for (let i = 1; i < count - 1; i += 1) {
    indices.push(0, i + 1, i, count, count + i, count + i + 1);
  }
  for (let i = 0; i < count; i += 1) {
    const next = (i + 1) % count;
    indices.push(i, next, next + count, i, next + count, i + count);
  }
  return new engine.Manifold(new engine.Mesh({
    numProp: 3,
    vertProperties: Float32Array.from(vertices),
    triVerts: Uint32Array.from(indices),
  }));
}

export async function replayFlushBaseline(offset = fixture.offset_mm.map(Number)) {
  const engine = await loadClashEngine();
  const a = rawPrism(engine, fixture.plate_xy_mm, offset);
  try {
    const b = rawPrism(engine, fixture.clip_xy_mm, offset);
    try {
      const status = [a.status(), b.status()];
      const reference_mm3 = fixture.retained_mm3;
      if (status.some(value => value !== 'NoError')) return { status, reference_mm3, failed_invariant: 'constructor_status' };
      const cut = a.subtract(b);
      try {
        status.push(cut.status());
        if (status[2] !== 'NoError') return { status, reference_mm3, failed_invariant: 'result_status' };
        const mesh = cut.getMesh();
        const { volume } = checkMesh(mesh.vertProperties, mesh.triVerts, 0);
        const failed_invariant = Math.abs(volume - reference_mm3) > 0.01 ? 'analytic_volume' : null;
        return { status, reference_mm3, member_volume_mm3: volume, failed_invariant };
      } finally {
        cut.delete();
      }
    } finally {
      b.delete();
    }
  } finally {
    a.delete();
  }
}

export async function sweepFlushBaseline() {
  let state = fixture.trial_seed;
  for (let row = 0; row < 100; row += 1) {
    const offset = [8192, 5000, 16384].map((anchor, axis) => {
      state = (Math.imul(1664525, state) + 1013904223) >>> 0;
      return (anchor * 10000 + (axis === 1 ? 1 : -1) * (1 + state % 20)) / 10000;
    });
    const result = await replayFlushBaseline(offset);
    if (result.failed_invariant === 'analytic_volume') return { row, offset, ...result };
  }
  return null;
}
