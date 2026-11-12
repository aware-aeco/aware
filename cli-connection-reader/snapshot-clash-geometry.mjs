/**
 * The geometry half of `check-snapshots` clash: reading a GLB tile into named triangle meshes, welding and
 * checking a mesh, exact triangle-to-triangle distance, and the pinned rounding. Pure: bytes in, numbers out.
 * Everything here is deterministic (fixed iteration orders, no randomness, no clock).
 */
import { fail } from './snapshot-compare.mjs';

// ─── rounding ───────────────────────────────────────────────────────────────────────────────────────

/** Round half to even at `1 / scale` (scale 100 = 0.01 mm, scale 1 = whole numbers). Never returns -0. */
export function roundHalfEven(value, scale) {
  if (value === null) return null;
  const scaled = value * scale;
  const floor = Math.floor(scaled);
  const diff = scaled - floor;
  let rounded;
  if (Math.abs(diff - 0.5) < 1e-7) rounded = floor % 2 === 0 ? floor : floor + 1;
  else rounded = Math.round(scaled);
  return rounded === 0 ? 0 : rounded / scale;
}
export const mm = (value) => roundHalfEven(value, 100);
export const mm3 = (value) => roundHalfEven(value, 1);
export const point = (p) => (p === null ? null : [mm(p[0]), mm(p[1]), mm(p[2])]);

// ─── GLB ────────────────────────────────────────────────────────────────────────────────────────────

const GLB_MAGIC = 0x46546c67;
const COMPONENT_BYTES = { 5120: 1, 5121: 1, 5122: 2, 5123: 2, 5125: 4, 5126: 4 };
const TYPE_COMPONENTS = { SCALAR: 1, VEC2: 2, VEC3: 3, VEC4: 4 };
const bad = (message) => fail('geometry-invalid', message);

function container(bytes, label) {
  const buf = Buffer.from(bytes);
  if (buf.length < 20 || buf.readUInt32LE(0) !== GLB_MAGIC || buf.readUInt32LE(4) !== 2 || buf.readUInt32LE(8) !== buf.length) {
    bad(`${label} is not a GLB 2.0 file`);
  }
  let cursor = 12;
  let json = null;
  let binary = Buffer.alloc(0);
  while (cursor < buf.length) {
    if (cursor + 8 > buf.length) bad(`${label} has a truncated chunk header`);
    const length = buf.readUInt32LE(cursor);
    const type = buf.readUInt32LE(cursor + 4);
    cursor += 8;
    if (length % 4 !== 0 || cursor + length > buf.length) bad(`${label} has a chunk that is misaligned or runs past the end`);
    const payload = buf.subarray(cursor, cursor + length);
    cursor += length;
    if (type === 0x4e4f534a) {
      if (json !== null) bad(`${label} has two JSON chunks`);
      try { json = JSON.parse(payload.toString('utf8')); } catch { bad(`${label} has a JSON chunk that is not JSON`); }
    } else if (type === 0x004e4942) {
      binary = payload;
    } else bad(`${label} has an unsupported chunk`);
  }
  if (json === null || typeof json !== 'object') bad(`${label} has no JSON chunk`);
  return { json, binary };
}

function accessorData(doc, binary, index, label) {
  const accessor = doc.accessors?.[index];
  const bytesPer = COMPONENT_BYTES[accessor?.componentType];
  const comps = TYPE_COMPONENTS[accessor?.type];
  if (!accessor || !bytesPer || !comps || accessor.sparse !== undefined || !Number.isSafeInteger(accessor.count) || accessor.count < 0) {
    bad(`${label} accessor is unsupported`);
  }
  const view = doc.bufferViews?.[accessor.bufferView];
  if (!view || (view.buffer ?? 0) !== 0) bad(`${label} accessor has no buffer view`);
  const stride = view.byteStride ?? bytesPer * comps;
  const base = (view.byteOffset ?? 0) + (accessor.byteOffset ?? 0);
  const last = base + (accessor.count - 1) * stride + bytesPer * comps;
  if (accessor.count > 0 && (base < 0 || last > binary.length)) bad(`${label} accessor runs outside the binary chunk`);
  return { accessor, base, stride, comps, bytesPer };
}

function readFloat3(doc, binary, index) {
  const { accessor, base, stride, comps } = accessorData(doc, binary, index, 'POSITION');
  if (accessor.type !== 'VEC3' || accessor.componentType !== 5126 || accessor.normalized === true || comps !== 3) bad('POSITION must be float32 VEC3');
  const out = new Float32Array(accessor.count * 3);
  for (let i = 0; i < accessor.count; i += 1) {
    for (let axis = 0; axis < 3; axis += 1) out[i * 3 + axis] = binary.readFloatLE(base + i * stride + axis * 4);
  }
  return out;
}

function readIndices(doc, binary, index, vertexCount) {
  const { accessor, base, stride, bytesPer } = accessorData(doc, binary, index, 'indices');
  if (accessor.type !== 'SCALAR' || ![5121, 5123, 5125].includes(accessor.componentType)) bad('indices must be an unsigned SCALAR');
  const out = new Uint32Array(accessor.count);
  for (let i = 0; i < accessor.count; i += 1) {
    const at = base + i * stride;
    const value = bytesPer === 1 ? binary.readUInt8(at) : bytesPer === 2 ? binary.readUInt16LE(at) : binary.readUInt32LE(at);
    if (value >= vertexCount) bad('an index runs past the vertex count');
    out[i] = value;
  }
  return out;
}

const IDENTITY = [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1];
function multiply(a, b) {   // column-major a * b
  const out = new Array(16).fill(0);
  for (let col = 0; col < 4; col += 1) for (let row = 0; row < 4; row += 1) {
    let sum = 0;
    for (let k = 0; k < 4; k += 1) sum += a[k * 4 + row] * b[col * 4 + k];
    out[col * 4 + row] = sum;
  }
  return out;
}
function localMatrix(node) {
  if (node.matrix !== undefined) {
    if (!Array.isArray(node.matrix) || node.matrix.length !== 16 || node.matrix.some((n) => typeof n !== 'number' || !Number.isFinite(n))) bad('a node matrix is invalid');
    return node.matrix;
  }
  const t = node.translation ?? [0, 0, 0];
  const q = node.rotation ?? [0, 0, 0, 1];
  const s = node.scale ?? [1, 1, 1];
  for (const list of [t, s]) if (!Array.isArray(list) || list.length !== 3 || list.some((n) => typeof n !== 'number' || !Number.isFinite(n))) bad('a node translation or scale is invalid');
  if (!Array.isArray(q) || q.length !== 4 || q.some((n) => typeof n !== 'number' || !Number.isFinite(n))) bad('a node rotation is invalid');
  const [x, y, z, w] = q;
  const r = [
    1 - 2 * (y * y + z * z), 2 * (x * y + z * w), 2 * (x * z - y * w), 0,
    2 * (x * y - z * w), 1 - 2 * (x * x + z * z), 2 * (y * z + x * w), 0,
    2 * (x * z + y * w), 2 * (y * z - x * w), 1 - 2 * (x * x + y * y), 0,
    0, 0, 0, 1,
  ];
  for (let col = 0; col < 3; col += 1) for (let row = 0; row < 3; row += 1) r[col * 4 + row] *= s[col];
  r[12] = t[0]; r[13] = t[1]; r[14] = t[2];
  return r;
}
const isIdentity = (m) => m.every((value, i) => value === IDENTITY[i]);
const determinant3 = (m) => m[0] * (m[5] * m[10] - m[9] * m[6]) - m[4] * (m[1] * m[10] - m[9] * m[2]) + m[8] * (m[1] * m[6] - m[5] * m[2]);

/**
 * Every named mesh of one tile: `Map name -> { positions (Float32Array|Float64Array, xyz), tris (Uint32Array) }`.
 * A mesh belongs to the node that carries it, or to the nearest named ancestor when its own node has no name.
 * Node transforms (matrix, or translation/rotation/scale) are applied; a mirroring transform flips the winding.
 * Only triangle lists (mode 4, the default) are accepted.
 */
export function readTileMeshes(bytes, label) {
  const { json: doc, binary } = container(bytes, label);
  const nodes = Array.isArray(doc.nodes) ? doc.nodes : [];
  const meshes = Array.isArray(doc.meshes) ? doc.meshes : [];
  const children = new Set();
  for (const node of nodes) if (Array.isArray(node?.children)) for (const c of node.children) children.add(c);
  let roots;
  if (Array.isArray(doc.scenes) && doc.scenes.length > 0) {
    const scene = doc.scenes[doc.scene ?? 0];
    if (!scene || !Array.isArray(scene.nodes)) bad(`${label} has no usable scene`);
    roots = scene.nodes;
  } else roots = nodes.map((_, i) => i).filter((i) => !children.has(i));

  const pieces = new Map();   // name -> piece[]
  const seen = new Set();
  const stack = roots.map((index) => ({ index, parentName: null, parentMatrix: IDENTITY })).reverse();
  while (stack.length > 0) {
    const { index, parentName, parentMatrix } = stack.pop();
    const node = nodes[index];
    if (!node || typeof node !== 'object' || seen.has(index)) bad(`${label} has a node that is missing or visited twice`);
    seen.add(index);
    const name = typeof node.name === 'string' && node.name !== '' ? node.name : parentName;
    const world = multiply(parentMatrix, localMatrix(node));
    if (node.mesh !== undefined) {
      const mesh = meshes[node.mesh];
      if (!mesh || !Array.isArray(mesh.primitives)) bad(`${label} node refers to a mesh that does not exist`);
      if (name !== null) {
        for (const primitive of mesh.primitives) {
          if ((primitive.mode ?? 4) !== 4) bad(`${label} has a primitive that is not a triangle list`);
          if (!Number.isSafeInteger(primitive.attributes?.POSITION)) bad(`${label} has a primitive without POSITION`);
          let positions = readFloat3(doc, binary, primitive.attributes.POSITION);
          const count = positions.length / 3;
          const tris = primitive.indices === undefined
            ? Uint32Array.from({ length: count - (count % 3) }, (_, i) => i)
            : readIndices(doc, binary, primitive.indices, count);
          if (tris.length % 3 !== 0) bad(`${label} has an index list that is not a multiple of three`);
          let flip = false;
          if (!isIdentity(world)) {
            const moved = new Float64Array(positions.length);
            for (let i = 0; i < count; i += 1) {
              const x = positions[i * 3]; const y = positions[i * 3 + 1]; const z = positions[i * 3 + 2];
              moved[i * 3] = world[0] * x + world[4] * y + world[8] * z + world[12];
              moved[i * 3 + 1] = world[1] * x + world[5] * y + world[9] * z + world[13];
              moved[i * 3 + 2] = world[2] * x + world[6] * y + world[10] * z + world[14];
            }
            positions = moved;
            flip = determinant3(world) < 0;
          }
          const list = pieces.get(name);
          const piece = { positions, tris, flip };
          if (list) list.push(piece); else pieces.set(name, [piece]);
        }
      }
    }
    if (Array.isArray(node.children)) for (const child of [...node.children].reverse()) stack.push({ index: child, parentName: name, parentMatrix: world });
  }

  const out = new Map();
  for (const [name, list] of pieces) {
    if (list.length === 1 && !list[0].flip) { out.set(name, { positions: list[0].positions, tris: list[0].tris }); continue; }
    let vertexTotal = 0; let triTotal = 0;
    for (const piece of list) { vertexTotal += piece.positions.length / 3; triTotal += piece.tris.length; }
    const positions = new Float64Array(vertexTotal * 3);
    const tris = new Uint32Array(triTotal);
    let v = 0; let t = 0;
    for (const piece of list) {
      positions.set(piece.positions, v * 3);
      for (let i = 0; i < piece.tris.length; i += 3) {
        const a = piece.tris[i] + v; const b = piece.tris[i + 1] + v; const c = piece.tris[i + 2] + v;
        if (piece.flip) { tris[t++] = a; tris[t++] = c; tris[t++] = b; } else { tris[t++] = a; tris[t++] = b; tris[t++] = c; }
      }
      v += piece.positions.length / 3;
    }
    out.set(name, { positions, tris });
  }
  return out;
}

/** The axis-aligned box of a raw position array, or null when it is empty. */
export function boxOf(positions) {
  if (positions.length === 0) return null;
  const box = [Infinity, Infinity, Infinity, -Infinity, -Infinity, -Infinity];
  for (let i = 0; i < positions.length; i += 3) {
    for (let axis = 0; axis < 3; axis += 1) {
      const value = positions[i + axis];
      if (value < box[axis]) box[axis] = value;
      if (value > box[axis + 3]) box[axis + 3] = value;
    }
  }
  return box;
}

// ─── welding + the mesh check ───────────────────────────────────────────────────────────────────────

/**
 * Weld vertices closer than `tolerance` (union by first-seen representative, scanning in index order, so the result is
 * the same for the same input). Returns the welded vertex array and triangles indexing it.
 */
export function weld(positions, tris, tolerance) {
  const count = positions.length / 3;
  const exact = new Map();
  const cells = new Map();
  const remap = new Uint32Array(count);
  const verts = [];
  const cell = (value) => Math.floor(value / tolerance);
  const tolSq = tolerance * tolerance;
  for (let i = 0; i < count; i += 1) {
    const x = positions[i * 3]; const y = positions[i * 3 + 1]; const z = positions[i * 3 + 2];
    const exactKey = `${x},${y},${z}`;
    const known = exact.get(exactKey);
    if (known !== undefined) { remap[i] = known; continue; }
    const cx = cell(x); const cy = cell(y); const cz = cell(z);
    let found = -1;
    for (let dx = -1; dx <= 1 && found < 0; dx += 1) for (let dy = -1; dy <= 1 && found < 0; dy += 1) for (let dz = -1; dz <= 1 && found < 0; dz += 1) {
      const list = cells.get(`${cx + dx},${cy + dy},${cz + dz}`);
      if (!list) continue;
      for (const rep of list) {
        const ex = verts[rep * 3] - x; const ey = verts[rep * 3 + 1] - y; const ez = verts[rep * 3 + 2] - z;
        if (ex * ex + ey * ey + ez * ez <= tolSq) { found = rep; break; }
      }
    }
    let index = found;
    if (index < 0) {
      index = verts.length / 3;
      verts.push(x, y, z);
      const key = `${cx},${cy},${cz}`;
      const list = cells.get(key);
      if (list) list.push(index); else cells.set(key, [index]);
    }
    exact.set(exactKey, index);
    remap[i] = index;
  }
  const welded = new Uint32Array(tris.length);
  for (let i = 0; i < tris.length; i += 1) welded[i] = remap[tris[i]];
  return { verts: Float64Array.from(verts), tris: welded };
}

const crossOf = (verts, a, b, c) => {
  const ux = verts[b * 3] - verts[a * 3]; const uy = verts[b * 3 + 1] - verts[a * 3 + 1]; const uz = verts[b * 3 + 2] - verts[a * 3 + 2];
  const vx = verts[c * 3] - verts[a * 3]; const vy = verts[c * 3 + 1] - verts[a * 3 + 1]; const vz = verts[c * 3 + 2] - verts[a * 3 + 2];
  return [uy * vz - uz * vy, uz * vx - ux * vz, ux * vy - uy * vx];
};

/**
 * AWARE's own closed-solid check on a welded mesh: no degenerate triangle; every undirected edge used by exactly two
 * triangles, once in each direction; positive signed volume. Returns `{ reason: null | 'mesh-degenerate' | 'mesh-open'
 * | 'mesh-orientation', volume, area }`.
 */
export function checkMesh(verts, tris) {
  let area = 0;
  let degenerate = false;
  const vertexCount = verts.length / 3;
  const edges = new Map();
  let open = false;
  let orientation = false;
  for (let t = 0; t < tris.length; t += 3) {
    const a = tris[t]; const b = tris[t + 1]; const c = tris[t + 2];
    const cr = crossOf(verts, a, b, c);
    const twice = Math.hypot(cr[0], cr[1], cr[2]);
    area += twice / 2;
    if (a === b || b === c || a === c || twice / 2 <= 1e-9) { degenerate = true; continue; }
    for (const [u, v] of [[a, b], [b, c], [c, a]]) {
      const low = Math.min(u, v); const high = Math.max(u, v);
      const key = low * vertexCount + high;
      const entry = edges.get(key) ?? [0, 0];
      entry[u < v ? 0 : 1] += 1;
      edges.set(key, entry);
    }
  }
  for (const [forward, backward] of edges.values()) {
    if (forward + backward !== 2) open = true;
    else if (forward !== 1) orientation = true;
  }
  let volume = 0;
  const ox = verts[0] ?? 0; const oy = verts[1] ?? 0; const oz = verts[2] ?? 0;
  for (let t = 0; t < tris.length; t += 3) {
    const a = tris[t]; const b = tris[t + 1]; const c = tris[t + 2];
    const ax = verts[a * 3] - ox; const ay = verts[a * 3 + 1] - oy; const az = verts[a * 3 + 2] - oz;
    const bx = verts[b * 3] - ox; const by = verts[b * 3 + 1] - oy; const bz = verts[b * 3 + 2] - oz;
    const cx = verts[c * 3] - ox; const cy = verts[c * 3 + 1] - oy; const cz = verts[c * 3 + 2] - oz;
    volume += (ax * (by * cz - bz * cy) - ay * (bx * cz - bz * cx) + az * (bx * cy - by * cx)) / 6;
  }
  let reason = null;
  if (degenerate || tris.length === 0) reason = 'mesh-degenerate';
  else if (open) reason = 'mesh-open';
  else if (orientation || volume < 0) reason = 'mesh-orientation';
  else if (volume === 0) reason = 'mesh-degenerate';
  return { reason, volume, area };
}

/** One unit normal per distinct face direction (a normal and its opposite are the same direction), in triangle order. */
export function faceDirections(verts, tris, decimals = 1e9) {
  const seen = new Map();
  for (let t = 0; t < tris.length; t += 3) {
    const cr = crossOf(verts, tris[t], tris[t + 1], tris[t + 2]);
    const length = Math.hypot(cr[0], cr[1], cr[2]);
    if (length <= 1e-12) continue;
    let n = [cr[0] / length, cr[1] / length, cr[2] / length];
    const lead = n.find((c) => Math.abs(c) > 1e-9);
    if (lead !== undefined && lead < 0) n = n.map((c) => -c);
    const key = n.map((c) => { const r = Math.round(c * decimals); return r === 0 ? 0 : r; }).join(',');
    if (!seen.has(key)) seen.set(key, n);
  }
  return [...seen.values()];
}

// ─── exact triangle-to-triangle distance ────────────────────────────────────────────────────────────

const sub = (a, b) => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const dot = (a, b) => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
const add = (a, b) => [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
const scale = (a, s) => [a[0] * s, a[1] * s, a[2] * s];
const distSq = (a, b) => { const d = sub(a, b); return dot(d, d); };

/** Closest point on triangle abc to p (Ericson, Real-Time Collision Detection 5.1.5). */
function closestOnTriangle(p, a, b, c) {
  const ab = sub(b, a); const ac = sub(c, a); const ap = sub(p, a);
  const d1 = dot(ab, ap); const d2 = dot(ac, ap);
  if (d1 <= 0 && d2 <= 0) return a;
  const bp = sub(p, b);
  const d3 = dot(ab, bp); const d4 = dot(ac, bp);
  if (d3 >= 0 && d4 <= d3) return b;
  const vc = d1 * d4 - d3 * d2;
  if (vc <= 0 && d1 >= 0 && d3 <= 0) return add(a, scale(ab, d1 / (d1 - d3)));
  const cp = sub(p, c);
  const d5 = dot(ab, cp); const d6 = dot(ac, cp);
  if (d6 >= 0 && d5 <= d6) return c;
  const vb = d5 * d2 - d1 * d6;
  if (vb <= 0 && d2 >= 0 && d6 <= 0) return add(a, scale(ac, d2 / (d2 - d6)));
  const va = d3 * d6 - d5 * d4;
  if (va <= 0 && d4 - d3 >= 0 && d5 - d6 >= 0) return add(b, scale(sub(c, b), (d4 - d3) / ((d4 - d3) + (d5 - d6))));
  const denominator = 1 / (va + vb + vc);
  return add(a, add(scale(ab, vb * denominator), scale(ac, vc * denominator)));
}

/** Closest points between segments p1q1 and p2q2 (Ericson 5.1.9): `[onFirst, onSecond]`. */
function closestOnSegments(p1, q1, p2, q2) {
  const d1 = sub(q1, p1); const d2 = sub(q2, p2); const r = sub(p1, p2);
  const a = dot(d1, d1); const e = dot(d2, d2); const f = dot(d2, r);
  let s; let t;
  if (a <= 1e-18 && e <= 1e-18) return [p1, p2];
  if (a <= 1e-18) { s = 0; t = Math.min(1, Math.max(0, f / e)); }
  else {
    const c = dot(d1, r);
    if (e <= 1e-18) { t = 0; s = Math.min(1, Math.max(0, -c / a)); }
    else {
      const b = dot(d1, d2);
      const denom = a * e - b * b;
      s = denom > 1e-18 ? Math.min(1, Math.max(0, (b * f - c * e) / denom)) : 0;
      t = (b * s + f) / e;
      if (t < 0) { t = 0; s = Math.min(1, Math.max(0, -c / a)); }
      else if (t > 1) { t = 1; s = Math.min(1, Math.max(0, (b - c) / a)); }
    }
  }
  return [add(p1, scale(d1, s)), add(p2, scale(d2, t))];
}

/** Where segment pq pierces triangle abc, or null (Moller-Trumbore on the segment; parallel is left to the distance tests). */
function pierce(p, q, a, b, c) {
  const dir = sub(q, p);
  const e1 = sub(b, a); const e2 = sub(c, a);
  const h = [dir[1] * e2[2] - dir[2] * e2[1], dir[2] * e2[0] - dir[0] * e2[2], dir[0] * e2[1] - dir[1] * e2[0]];
  const det = dot(e1, h);
  if (Math.abs(det) < 1e-14) return null;
  const inv = 1 / det;
  const s = sub(p, a);
  const u = inv * dot(s, h);
  if (u < 0 || u > 1) return null;
  const qv = [s[1] * e1[2] - s[2] * e1[1], s[2] * e1[0] - s[0] * e1[2], s[0] * e1[1] - s[1] * e1[0]];
  const v = inv * dot(dir, qv);
  if (v < 0 || u + v > 1) return null;
  const t = inv * dot(e2, qv);
  if (t < 0 || t > 1) return null;
  return add(p, scale(dir, t));
}

/** `{ d2, a, b }`: squared distance and the closest point on each triangle; two triangles that cross give 0 at the crossing. */
export function triangleDistance(A, B) {
  const edgesA = [[A[0], A[1]], [A[1], A[2]], [A[2], A[0]]];
  const edgesB = [[B[0], B[1]], [B[1], B[2]], [B[2], B[0]]];
  for (const [p, q] of edgesA) { const hit = pierce(p, q, B[0], B[1], B[2]); if (hit) return { d2: 0, a: hit, b: hit }; }
  for (const [p, q] of edgesB) { const hit = pierce(p, q, A[0], A[1], A[2]); if (hit) return { d2: 0, a: hit, b: hit }; }
  let best = { d2: Infinity, a: null, b: null };
  for (const v of A) { const c = closestOnTriangle(v, B[0], B[1], B[2]); const d2 = distSq(v, c); if (d2 < best.d2) best = { d2, a: v, b: c }; }
  for (const v of B) { const c = closestOnTriangle(v, A[0], A[1], A[2]); const d2 = distSq(v, c); if (d2 < best.d2) best = { d2, a: c, b: v }; }
  for (const [p1, q1] of edgesA) for (const [p2, q2] of edgesB) {
    const [x, y] = closestOnSegments(p1, q1, p2, q2);
    const d2 = distSq(x, y);
    if (d2 < best.d2) best = { d2, a: x, b: y };
  }
  return best;
}

/** Axis-aligned boxes of every triangle, `Float64Array(6 * n)`. */
export function triangleBoxes(verts, tris) {
  const n = tris.length / 3;
  const out = new Float64Array(n * 6);
  for (let t = 0; t < n; t += 1) {
    for (let axis = 0; axis < 3; axis += 1) {
      const a = verts[tris[t * 3] * 3 + axis]; const b = verts[tris[t * 3 + 1] * 3 + axis]; const c = verts[tris[t * 3 + 2] * 3 + axis];
      out[t * 6 + axis] = Math.min(a, b, c);
      out[t * 6 + axis + 3] = Math.max(a, b, c);
    }
  }
  return out;
}

/**
 * The minimum surface distance between two welded meshes when it is at most `cap`, with its closest points, else null.
 * Triangles are scanned in index order and only a STRICTLY smaller distance replaces the best, so a tie keeps the
 * first pair in that order; boxes grown by the best distance so far prune the scan.
 */
export function meshDistance(A, B, cap) {
  const capSq = cap * cap;
  const grow = cap;
  const boxA = A.box; const boxB = B.box;
  const reach = (small, other) => small[0] <= other[3] + grow && small[3] >= other[0] - grow
    && small[1] <= other[4] + grow && small[4] >= other[1] - grow && small[2] <= other[5] + grow && small[5] >= other[2] - grow;
  if (!reach(boxA, boxB)) return null;
  const aBoxes = A.triBoxes; const bBoxes = B.triBoxes;
  const near = (boxes, n, other) => {
    const list = [];
    for (let t = 0; t < n; t += 1) {
      const o = t * 6;
      if (boxes[o] <= other[3] + grow && boxes[o + 3] >= other[0] - grow && boxes[o + 1] <= other[4] + grow && boxes[o + 4] >= other[1] - grow
        && boxes[o + 2] <= other[5] + grow && boxes[o + 5] >= other[2] - grow) list.push(t);
    }
    return list;
  };
  const aList = near(aBoxes, A.tris.length / 3, boxB);
  const bList = near(bBoxes, B.tris.length / 3, boxA);
  const tri = (S, t) => [0, 1, 2].map((k) => [S.verts[S.tris[t * 3 + k] * 3], S.verts[S.tris[t * 3 + k] * 3 + 1], S.verts[S.tris[t * 3 + k] * 3 + 2]]);
  let best = { d2: Infinity, a: null, b: null };
  for (const ta of aList) {
    const ao = ta * 6;
    for (const tb of bList) {
      const reachNow = Math.sqrt(Math.min(best.d2, capSq));
      const bo = tb * 6;
      if (aBoxes[ao] > bBoxes[bo + 3] + reachNow || bBoxes[bo] > aBoxes[ao + 3] + reachNow
        || aBoxes[ao + 1] > bBoxes[bo + 4] + reachNow || bBoxes[bo + 1] > aBoxes[ao + 4] + reachNow
        || aBoxes[ao + 2] > bBoxes[bo + 5] + reachNow || bBoxes[bo + 2] > aBoxes[ao + 5] + reachNow) continue;
      const r = triangleDistance(tri(A, ta), tri(B, tb));
      if (r.d2 < best.d2) { best = r; if (r.d2 === 0) return finish(best); }
    }
  }
  return best.d2 <= capSq ? finish(best) : null;
}
function finish(best) {
  return {
    distance: Math.sqrt(best.d2),
    location: [(best.a[0] + best.b[0]) / 2, (best.a[1] + best.b[1]) / 2, (best.a[2] + best.b[2]) / 2],
  };
}
