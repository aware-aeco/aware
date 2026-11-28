import { readFileSync } from 'node:fs';
import { pathToFileURL } from 'node:url';

import { loadClashEngine } from './snapshot-clash.mjs';
import { buildSlabs, checkSlabs } from './geometry-slabs.mjs';
import { subtractMesh, toFloat32Mesh, validateMesh } from './mesh-solid.mjs';

const limit = 2 ** 14;
const inputAllowance = 1e-5;
const edgeBudget = 4_000_000;
const scale = 10n ** 32n;
const axes = [0, 1, 2];

const occtToleranceCap = 1e-5;
const occtFuzzyFloor = 1e-7;
const seamWeld = 1e-7;
const arithmetic = 256 * limit * Number.EPSILON;

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

function onSegment(a, b, point) {
  return orientation(a, b, point) === 0n
    && [0, 1].every(axis => (point[axis] - a[axis]) * (point[axis] - b[axis]) <= 0n);
}

const equalXY = (a, b) => a[0] === b[0] && a[1] === b[1];

const next = (list, i) => list[(i + 1) % list.length];
const numeric = x => Number(x) / Number(scale);

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

function capsSeparated(a, b, budget, charge) {
  // Each footprint can move by one budget. Bound the combined displacement
  // normal to a supporting edge using the two XY allowances on the decimal grid.
  const [x, y] = budget.slice(0, 2).map(value =>
    BigInt(Math.ceil(2 * value * Number(scale) * (1 + 8 * Number.EPSILON))));
  const abs = value => value < 0n ? -value : value;
  return [[a, b], [b, a]].some(([left, right]) => left.some((start, i) => {
    const end = next(left, i);
    const allowance = abs(end[0] - start[0]) * y + abs(end[1] - start[1]) * x;
    return right.every(point => {
      charge();
      return orientation(start, end, point) < -allowance;
    });
  }));
}

function checkCoincidence(a, b, working, budget, charge) {
  const [left, right] = working.map((mesh, i) => ({
    xy: mesh.positions_mm.slice(0, [a, b][i].xy.length).map(p => p.slice(0, 2).map(exactFloat32)),
    z: [mesh.positions_mm[0][2], mesh.positions_mm[[a, b][i].xy.length][2]],
  }));
  const capMismatch = a.z.findIndex((z, i) => b.z.some((other, j) => z === other && left.z[i] !== right.z[j]));
  if (capMismatch !== -1 && !capsSeparated(a.xy, b.xy, budget, charge)) {
    reject('coincidence_mismatch', capMismatch === 0 ? 'base_cap' : 'top_cap');
  }
  if (a.z[1] < b.z[0] || b.z[1] < a.z[0]) return;
  for (const [u, v, uw, vw] of [[a, b, left, right], [b, a, right, left]]) {
    for (const [i, edge] of u.xy.entries()) for (const [j, point] of v.xy.entries()) {
      if (!onSegment(edge, next(u.xy, i), point)) continue;
      const start = uw.xy[i];
      const end = next(uw.xy, i);
      const other = vw.xy[j];
      if (!onSegment(start, end, other)) reject('coincidence_mismatch', 'side_incidence');
      if (equalXY(edge, point) !== equalXY(start, other)
          || equalXY(next(u.xy, i), point) !== equalXY(end, other)) {
        reject('coincidence_mismatch', 'vertex_alignment');
      }
    }
  }
}

function centred(operand, translate) {
  const points = operand.mesh.positions_mm.map(point => point.map((value, i) => value + translate[i]));
  const extentInKernel = extent(bounds(points));
  const ulp = ulp32(extentInKernel);
  if (extentInKernel > limit || operand.shortest < 8 * ulp) reject('budget_exceeded');
  const mesh = toFloat32Mesh({ positions_mm: points, triangles: operand.mesh.triangles });
  return { mesh, box: bounds(mesh.positions_mm) };
}

function prepareMesh({ a: left, b: right }, report = {}) {
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
  const boxScale = Math.max(extent(leftWorking.box), rightWorking ? extent(rightWorking.box) : 0);
  const conversion = axes.map(i => (ulp32(Math.max(
    Math.abs(leftWorking.box.low[i]), Math.abs(leftWorking.box.high[i]),
    Math.abs(rightWorking?.box.low[i] ?? 0), Math.abs(rightWorking?.box.high[i] ?? 0),
  )) / 2));
  // Manifold 3.5.4 SetEpsilon uses the float epsilon times Box::Scale
  // for single-precision input. The union scale bounds both operands and
  // the result. Account separately for ingress and egress float rounding.
  const manifoldTolerance = 2 ** -23 * boxScale;
  const outputConversion = ulp32(boxScale + manifoldTolerance) / 2;
  const placement = axes.map(i => (Math.abs(a.offset[i]) + Math.abs(b.offset[i])
    + Math.abs(reference[i]) + Math.abs(offset[i]) + 8 * limit) * 8 * Number.EPSILON);
  const budgets = {
    exact: { input_mm: inputAllowance + occtToleranceCap, placement_mm: placement,
      construction_mm: arithmetic, kernel_mm: occtFuzzyFloor, weld_mm: seamWeld },
    mesh: { input_mm: inputAllowance, placement_mm: placement, construction_mm: arithmetic,
      conversion_mm: conversion, output_conversion_mm: outputConversion, kernel_mm: manifoldTolerance },
  };
  const total = path => axes.map(i => Object.values(path).reduce(
    (sum, value) => sum + (Array.isArray(value) ? value[i] : value), 0) * (1 + 16 * Number.EPSILON));
  const exactBudget = total(budgets.exact);
  const meshBudget = total(budgets.mesh);
  const budget = axes.map(i => Math.max(exactBudget[i], meshBudget[i]));
  const threshold = [8 * Math.hypot(budget[0], budget[1]), 8 * Math.hypot(budget[0], budget[1]), 8 * budget[2]];
  const shift = axes.map(i => reference[i] - a.offset[i] - midpoint[i]);
  const charge = (count = 1) => {
    remaining -= count;
    if (remaining < 0) reject('budget_exceeded');
  };
  let remaining = edgeBudget - work;
  if (rightWorking) checkCoincidence(a, b, [leftWorking.mesh, rightWorking.mesh], budget, charge);
  const oracle = buildSlabs(a, b, base, shift, numeric, budget, disjoint, charge, reject);
  Object.assign(report, { budgets, threshold_mm: threshold, clearance_mm: oracle.clearance });
  if (oracle.clearance.some((width, i) => !(width > threshold[i]))) reject('budget_exceeded');
  return { leftWorking, rightWorking, oracle, budget, exactBudget, meshBudget, offset, a, charge };
}

export function inspectAdmission(request) {
  const report = {};
  try {
    prepareMesh(request, report);
    return { ...report, code: null };
  } catch (error) {
    return { ...report, ...rejected(error) };
  }
}

const triangleLoops = mesh => mesh.triangles.map(tri => [tri.map(i => mesh.positions_mm[i])]);

function rejected(error, exact = false) {
  const code = error instanceof Rejected ? error.message : 'mesh_invalid';
  return { status: 'rejected', code: exact && code === 'mesh_invalid' ? 'brep_validation_failed' : code,
    relation: error.relation ?? null, result: null };
}

export async function cutMeshRequest(request, beforeCut) {
  try {
    const { leftWorking, rightWorking, oracle, meshBudget, offset, a, charge } = prepareMesh(request);
    let result = leftWorking.mesh;
    if (rightWorking) {
      let engine;
      try {
        engine = await loadClashEngine();
      } catch {
        reject('mesh_worker_failed');
      }
      beforeCut?.();
      result = subtractMesh(engine, leftWorking.mesh, rightWorking.mesh);
    }
    const { mesh, volume } = validateMesh(result, true);
    checkSlabs(oracle, triangleLoops(mesh), volume, meshBudget, arithmetic, charge, reject);
    return { status: 'ok', code: null, relation: null, result: {
      offset_mm: oracle.volume === 0 ? a.offset : offset, brep_b64: null, mesh, volume_mm3: volume,
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
    const { oracle, exactBudget, offset, charge } = prepareMesh(request);
    const delta = result.offset_mm.map((value, i) => value - offset[i]);
    const placed = point => point.map((value, i) => value + delta[i]);
    const positions_mm = result.mesh.positions_mm.map(placed);
    const { mesh, volume } = validateMesh({ positions_mm, triangles: result.mesh.triangles }, true);
    const boundary = native_boundary.map(face => face.map(loop => loop.map(placed)));
    checkSlabs(oracle, boundary, result.volume_mm3, exactBudget, arithmetic, charge, reject);
    checkSlabs(oracle, triangleLoops(mesh), volume, exactBudget, arithmetic, charge, reject);
    const volumeBand = Math.max(1e-9, 1e-12 * Math.max(oracle.volume, result.volume_mm3));
    if (Math.abs(result.volume_mm3 - oracle.volume) + oracle.volumeError > volumeBand) {
      reject('brep_validation_failed');
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
