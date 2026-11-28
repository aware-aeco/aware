import { readFileSync } from 'node:fs';
import { pathToFileURL } from 'node:url';

import { loadClashEngine } from './snapshot-clash.mjs';
import { checkMesh } from './snapshot-clash-geometry.mjs';
import { subtractMesh, toFloat32Mesh, validateMesh } from './mesh-solid.mjs';

const limit = 2 ** 14;
const inputAllowance = 1e-5;
const edgeBudget = 4_000_000;
const scale = 10n ** 32n;
const axes = [0, 1, 2];

class Rejected extends Error {
  constructor(code, relation = null) {
    super(code);
    this.relation = relation;
  }
}

function reject(code, relation) {
  throw new Rejected(code, relation);
}

function decimal(text) {
  const [whole, fraction = ''] = text.replace(/^[+-]/, '').split('.');
  return (text.startsWith('-') ? -1n : 1n) * (BigInt(whole || '0') * scale + BigInt(fraction.padEnd(32, '0')));
}

function orientation(a, b, p) {
  return (b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0]);
}

const sign = x => x < 0n ? -1 : Number(x > 0n);
const next = (list, i) => list[(i + 1) % list.length];
const numeric = x => Number(x) / Number(scale);
const distance = (a, b) => Math.hypot(...a.map((v, i) => v - b[i]));

function bounds(points) {
  const low = [Infinity, Infinity, Infinity];
  const high = [-Infinity, -Infinity, -Infinity];
  for (const point of points) for (const axis of axes) {
    low[axis] = Math.min(low[axis], point[axis]);
    high[axis] = Math.max(high[axis], point[axis]);
  }
  return { low, high };
}

function extent(box) {
  return Math.max(...box.low.map(Math.abs), ...box.high.map(Math.abs));
}

function ulp32(maximum) {
  return 2 ** Math.max(-149, Math.floor(Math.log2(maximum)) - 23);
}

function triangleKey(face) {
  const first = face.indexOf(Math.min(...face));
  return face.map((_, i) => face[(first + i) % 3]).join(',');
}

function checkProducerFaces(mesh, n) {
  const expected = new Set();
  const add = (...face) => expected.add(triangleKey(face));
  for (let i = 1; i < n - 1; i += 1) {
    add(0, i + 1, i);
    add(n, n + i, n + i + 1);
  }
  for (let i = 0; i < n; i += 1) {
    const j = (i + 1) % n;
    add(i, j, j + n);
    add(i, j + n, i + n);
  }
  if (expected.size !== mesh.triangles.length || mesh.triangles.some(face => !expected.delete(triangleKey(face)))) reject('mesh_invalid');
}

function inspect(operand, base, anchor) {
  const { mesh } = validateMesh(operand.mesh);
  const box = bounds(mesh.positions_mm);
  if (extent(box) > limit) reject('budget_exceeded');
  const xy = operand.profile.xy_mm.map(point => point.map(decimal));
  const z = operand.profile.z_mm.map(decimal);
  const n = xy.length;
  if (z[0] >= z[1] || mesh.positions_mm.length !== 2 * n) reject('mesh_invalid');
  checkProducerFaces(mesh, n);
  for (const [i, vertex] of xy.entries()) for (const [j, p] of xy.entries()) {
    if (i !== j && (i + 1) % n !== j && orientation(vertex, next(xy, i), p) <= 0n) reject('mesh_invalid');
  }
  const local = (coordinate, axis) => numeric(coordinate - base[axis]) + (anchor[axis] - operand.offset_mm[axis]);
  const placementError = operand.offset_mm.map((coordinate, axis) => (
    (Math.abs(coordinate) + Math.abs(anchor[axis])) * 4 * Number.EPSILON
  ));
  const shortest = Math.min(numeric(z[1] - z[0]), ...xy.map((p, i) => (
    Math.hypot(...p.map((v, axis) => numeric(v - next(xy, i)[axis])))
  )));
  if (shortest < 8 * ulp32(extent(box))) reject('budget_exceeded');
  for (const [i, point] of mesh.positions_mm.entries()) {
    const expected = [...xy[i % n], z[Math.floor(i / n)]];
    if (point.some((value, axis) => Math.abs(value - local(expected[axis], axis)) > inputAllowance + placementError[axis])) {
      reject('mesh_invalid');
    }
    if (point[0] !== mesh.positions_mm[i % n][0] || point[1] !== mesh.positions_mm[i % n][1]
        || point[2] !== mesh.positions_mm[Math.floor(i / n) * n][2]) reject('mesh_invalid');
  }
  return { mesh, box, xy, z, shortest, offset: operand.offset_mm };
}

const view = new DataView(new ArrayBuffer(4));
function exactFloat32(value) {
  view.setFloat32(0, value);
  const bits = view.getUint32(0);
  const exponent = (bits >>> 23) & 255;
  const mantissa = (bits & 0x7fffff) + (exponent === 0 ? 0 : 0x800000);
  return (bits >>> 31 ? -1n : 1n) * (BigInt(mantissa) << BigInt(Math.max(0, exponent - 1)));
}

function checkCoincidence(a, b, working) {
  const [left, right] = working.map((mesh, i) => ({
    xy: mesh.positions_mm.slice(0, [a, b][i].xy.length).map(p => p.slice(0, 2).map(exactFloat32)),
    z: [mesh.positions_mm[0][2], mesh.positions_mm[[a, b][i].xy.length][2]],
  }));
  for (const i of [0, 1]) for (const j of [0, 1]) {
    if (a.z[i] === b.z[j] && left.z[i] !== right.z[j]) reject('coincidence_mismatch', i === 0 ? 'base_cap' : 'top_cap');
  }
  for (const [u, v, uw, vw] of [[a, b, left, right], [b, a, right, left]]) {
    for (const [i, edge] of u.xy.entries()) for (const [j, point] of v.xy.entries()) {
      if (orientation(edge, next(u.xy, i), point) === 0n && orientation(uw.xy[i], next(uw.xy, i), vw.xy[j]) !== 0n) {
        reject('coincidence_mismatch', 'side_incidence');
      }
      for (const axis of [0, 1]) {
        if (edge[axis] === point[axis] && uw.xy[i][axis] !== vw.xy[j][axis]) reject('coincidence_mismatch', 'vertex_alignment');
      }
    }
  }
}

function centred(operand, translate) {
  const points = operand.mesh.positions_mm.map(point => point.map((value, i) => value + translate[i]));
  const extentInKernel = extent(bounds(points));
  const ulp = ulp32(extentInKernel);
  if (extentInKernel > limit || operand.shortest < 8 * ulp) reject('budget_exceeded');
  return { mesh: toFloat32Mesh({ positions_mm: points, triangles: operand.mesh.triangles }), ulp };
}

function expectedPrism(a, b, disjoint, local, clearance) {
  const unchanged = { xy: a.xy.map(point => point.map(local)), z: a.z.map((v, i) => local(v, 2)) };
  if (disjoint || a.z[1] <= b.z[0] || b.z[1] <= a.z[0]) return unchanged;
  const outside = [];
  for (const [i, start] of b.xy.entries()) {
    const end = next(b.xy, i);
    const sides = a.xy.map(p => orientation(start, end, p));
    if (sides.every(side => side <= 0n)) return unchanged;
    if (sides.some(side => side < 0n)) outside.push(sides);
  }
  const levels = [...new Set([...a.z, ...b.z])].filter(level => level >= a.z[0] && level <= a.z[1]).sort((a, b) => sign(a - b));
  if (levels.some((level, i) => i > 0 && numeric(level - levels[i - 1]) <= clearance)) reject('budget_exceeded');
  if (b.z[0] > a.z[0] && b.z[1] < a.z[1]) {
    const sides = a.xy.map((point, i) => b.xy.map(p => orientation(point, next(a.xy, i), p)));
    if (sides.every(side => side.every(value => value >= 0n))) {
      // An edge tangent to the exterior has a disconnected vertex fan in
      // a transversal section. A shared face needs a separate slab check.
      if (sides.some(side => side.filter(value => value === 0n).length === 1)) reject('mesh_invalid');
      if (sides.every(side => side.every(value => value > 0n))) {
        for (const [i, side] of sides.entries()) {
          const length = distance(unchanged.xy[i], next(unchanged.xy, i));
          if (side.some(value => numeric(value) / Number(scale) / length <= clearance)) reject('budget_exceeded');
        }
        if (b.shortest <= clearance) reject('budget_exceeded');
        return { ...unchanged, cavity: {xy: b.xy.map(point => point.map(local)), z: b.z.map(v => local(v, 2))} };
      }
    }
  }
  if (outside.length === 0) {
    if (b.z[0] <= a.z[0] && b.z[1] >= a.z[1]) return null;
    if (b.z[0] <= a.z[0]) return { ...unchanged, z: [local(b.z[1], 2), unchanged.z[1]] };
    if (b.z[1] >= a.z[1]) return { ...unchanged, z: [unchanged.z[0], local(b.z[0], 2)] };
  }
  // A single violated clipping half-plane proves a convex retained section
  // on the full-height path. Defer more complex slab topology until it can
  // be verified independently.
  if (outside.length !== 1 || b.z[0] > a.z[0] || b.z[1] < a.z[1]) reject('mesh_worker_failed');
  const [side] = outside;
  const clipped = [];
  for (const [i, point] of unchanged.xy.entries()) {
    const here = side[i];
    const there = next(side, i);
    if (here <= 0n) clipped.push(point);
    if (sign(here) * sign(there) < 0) {
      const fraction = Number(here) / Number(here - there);
      clipped.push(point.map((value, axis) => value + fraction * (next(unchanged.xy, i)[axis] - value)));
    }
  }
  return { ...unchanged, xy: clipped };
}

function planarBoundary({ xy, z }, mesh, allowance) {
  const h = z[1] - z[0];
  const perimeter = xy.reduce((sum, p, i) => sum + distance(p, next(xy, i)), 0);
  const area = xy.reduce((sum, p, i) => sum + p[0] * next(xy, i)[1] - p[1] * next(xy, i)[0], 0) / 2;
  const surfaces = [[0, 0, -1, -z[0], area], [0, 0, 1, z[1], area]];
  for (const [i, a] of xy.entries()) {
    const b = next(xy, i);
    const length = distance(a, b);
    if (length <= allowance * 8) reject('budget_exceeded');
    const nx = (b[1] - a[1]) / length;
    const ny = (a[0] - b[0]) / length;
    surfaces.push([nx, ny, 0, nx * a[0] + ny * a[1], length * h]);
  }
  if (h <= allowance * 8 || area <= 0) reject('budget_exceeded');
  if (surfaces.length * (mesh.positions_mm.length + mesh.triangles.length) > edgeBudget) reject('budget_exceeded');
  const measures = surfaces.map(() => 0);
  const offsets = mesh.positions_mm.map(([x, y, z]) => surfaces.map(([nx, ny, nz, d]) => nx * x + ny * y + nz * z - d));
  if (offsets.some(perVertex => perVertex.some(value => value > allowance))) reject('mesh_invalid');
  for (const face of mesh.triangles) {
    const matches = surfaces.map((_, i) => i).filter(i => face.every(vertex => Math.abs(offsets[vertex][i]) <= allowance));
    if (matches.length !== 1) reject('mesh_invalid');
    const [index] = matches;
    const [nx, ny, nz] = surfaces[index];
    const [a, b, c] = face.map(i => mesh.positions_mm[i]);
    const [ux, uy, uz] = b.map((v, i) => v - a[i]);
    const [vx, vy, vz] = c.map((v, i) => v - a[i]);
    const projected = (nx * (uy * vz - uz * vy) + ny * (uz * vx - ux * vz) + nz * (ux * vy - uy * vx)) / 2;
    if (projected <= 0) reject('mesh_invalid');
    measures[index] += projected;
  }
  if (surfaces.some((surface, i) => Math.abs(surface[4] - measures[i]) > 4 * allowance * (perimeter + h))) reject('mesh_invalid');
  return { volume: area * h, allowance: 4 * allowance * (2 * area + perimeter * h + 1) };
}

function prepareMesh({ a: left, b: right }) {
    const base = [...left.profile.xy_mm[0], left.profile.z_mm[0]].map(decimal);
    // Use the first authoritative corner as the decimal subtraction anchor
    // before converting any differences to doubles.
    const reference = base.map(numeric);
    const work = (left.profile.xy_mm.length + right.profile.xy_mm.length) ** 2;
    if (work > edgeBudget) reject('budget_exceeded');
    const a = inspect(left, base, reference);
    const b = inspect(right, base, reference);
    const relative = b.offset.map((value, axis) => value - a.offset[axis]);
    const margin = a.offset.map((value, axis) => inputAllowance * 2 + (Math.abs(value) + Math.abs(b.offset[axis])) * 8 * Number.EPSILON);
    const bLow = b.box.low.map((v, axis) => v + relative[axis]);
    const bHigh = b.box.high.map((v, axis) => v + relative[axis]);
    const disjoint = axes.some(i => a.box.high[i] + margin[i] < bLow[i] || bHigh[i] + margin[i] < a.box.low[i]);
    const midpoint = axes.map(i => (disjoint
      ? a.box.low[i] + a.box.high[i]
      : Math.min(a.box.low[i] - margin[i], bLow[i] - margin[i]) + Math.max(a.box.high[i] + margin[i], bHigh[i] + margin[i])) / 2);
    const offset = a.offset.map((v, axis) => v + midpoint[axis]);
    if (offset.some((v, axis) => !Number.isFinite(v) || Math.abs((v - a.offset[axis]) - midpoint[axis]) > inputAllowance)) reject('budget_exceeded');
    const leftWorking = centred(a, midpoint.map(v => -v));
    const rightWorking = disjoint ? null : centred(b, midpoint.map((v, i) => relative[i] - v));
    if (rightWorking) checkCoincidence(a, b, [leftWorking.mesh, rightWorking.mesh]);
    const allowance = Math.max(...margin) + 4 * Math.max(leftWorking.ulp, rightWorking?.ulp ?? 0);
    const local = (value, axis) => numeric(value - base[axis]) + (reference[axis] - a.offset[axis]) - midpoint[axis];
    const expected = expectedPrism(a, b, disjoint, local, 8 * allowance);
    if (expected && (expected.z[1] - expected.z[0] <= 8 * allowance
        || expected.xy.some((p, i) => distance(p, next(expected.xy, i)) <= 8 * allowance))) reject('budget_exceeded');
    return { leftWorking, rightWorking, expected, allowance, offset, a };
}

function checkCandidate(expected, mesh, allowance, volume) {
  if (expected === null) {
    if (volume !== 0) reject('mesh_invalid');
    return { volume: 0, allowance: 0 };
  }
  if (volume <= 0) reject('mesh_invalid');
  const boundary = shells(mesh);
  if (boundary.length !== (expected.cavity ? 2 : 1)) reject('mesh_invalid');
  boundary.sort((a, b) => b.volume - a.volume);
  const [outer, inner] = boundary;
  if (outer.volume <= 0 || (inner && inner.volume >= 0)) reject('mesh_invalid');
  const reference = planarBoundary(expected, outer.mesh, allowance);
  if (inner) {
    const cavity = planarBoundary(expected.cavity, inner.mesh, allowance);
    reference.volume -= cavity.volume;
    reference.allowance += cavity.allowance;
  }
  if (Math.abs(volume - reference.volume) > reference.allowance) reject('mesh_invalid');
  return reference;
}

function shells({positions_mm, triangles}) {
  const incidence = positions_mm.map(() => []);
  for (const [i, face] of triangles.entries()) for (const vertex of face) incidence[vertex].push(i);
  const visited = new Set();
  const result = [];
  for (const [seed] of triangles.entries()) {
    if (visited.has(seed)) continue;
    const allFaces = [seed];
    const vertices = new Map();
    const points = [];
    const faces = [];
    visited.add(seed);
    for (let i = 0; i < allFaces.length; i += 1) {
      faces.push(triangles[allFaces[i]].map(vertex => {
        if (!vertices.has(vertex)) {
          vertices.set(vertex, points.length);
          points.push(positions_mm[vertex]);
          for (const neighbour of incidence[vertex]) if (!visited.has(neighbour)) {
            visited.add(neighbour);
            allFaces.push(neighbour);
          }
        }
        return vertices.get(vertex);
      }));
    }
    const { volume } = checkMesh(points.flat(), faces.flat(), 0);
    if (!Number.isFinite(volume) || volume === 0) reject('mesh_invalid');
    if (volume < 0) for (const face of faces) face.reverse();
    result.push({volume, mesh: {positions_mm: points, triangles: faces}});
  }
  return result;
}

function rejected(error, exact = false) {
  const code = error instanceof Rejected ? error.message : 'mesh_invalid';
  return { status: 'rejected', code: exact && code === 'mesh_invalid' ? 'brep_validation_failed' : code,
    relation: error.relation ?? null, result: null };
}

export async function cutMeshRequest(request) {
  try {
    const { leftWorking, rightWorking, expected, allowance, offset, a } = prepareMesh(request);
    const result = rightWorking
      ? subtractMesh(await loadClashEngine(), leftWorking.mesh, rightWorking.mesh)
      : leftWorking.mesh;
    const { mesh, volume } = validateMesh(result, true);
    checkCandidate(expected, mesh, allowance, volume);
    return { status: 'ok', code: null, relation: null, result: {
      offset_mm: expected === null ? a.offset : offset, brep_b64: null, mesh, volume_mm3: volume,
    } };
  } catch (error) {
    return rejected(error);
  }
}

function preflight(request) {
  try {
    prepareMesh(request);
    return { status: 'admitted' };
  } catch (error) {
    return rejected(error);
  }
}

export function verifyExact({ request, result, native_boundary }) {
  try {
    const { expected, allowance, offset } = prepareMesh(request);
    const delta = result.offset_mm.map((value, i) => value - offset[i]);
    for (const artifact of [native_boundary, result.mesh]) {
      const positions_mm = artifact.positions_mm.map(point => point.map((value, i) => value + delta[i]));
      const { mesh, volume } = validateMesh({ positions_mm, triangles: artifact.triangles }, true);
      const reference = checkCandidate(expected, mesh, allowance, volume);
      if (Math.abs(result.volume_mm3 - reference.volume) > Math.max(1e-9, 1e-12 * Math.max(reference.volume, result.volume_mm3))) {
        reject('brep_validation_failed');
      }
      if (Math.abs(result.volume_mm3 - volume) > reference.allowance) reject('brep_validation_failed');
    }
    return { status: 'ok', code: null, relation: null, result };
  } catch (error) {
    return rejected(error, true);
  }
}

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  const manifest = JSON.parse(readFileSync(new URL('./node_modules/manifold-3d/package.json', import.meta.url), 'utf8'));
  if (process.versions.node !== '24.14.0' || manifest.version !== '3.5.4') throw new Error('unexpected staged geometry dependency version');
  const chunks = [];
  for await (const chunk of process.stdin) chunks.push(chunk);
  const request = JSON.parse(Buffer.concat(chunks).toString('utf8'));
  const reply = process.argv[2] === '--preflight' ? preflight(request)
    : process.argv[2] === '--verify-exact' ? verifyExact(request) : await cutMeshRequest(request);
  process.stdout.write(`${JSON.stringify(reply)}\n`);
}
