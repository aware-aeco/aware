// Fixtures for the clash tests: closed solids built from numbers (boxes, extrusions of a polygon), a GLB writer that
// stores them the way the app's tile does (flat-shaded, float32 millimetres, nodes named object-<n>), and a "world"
// builder that turns a list of described parts into the three facade documents plus the geometry tile. The facade keys
// are Tekla-looking DATA; snapshot-clash.mjs reads them only because the check profile says so.
import { createHash } from 'node:crypto';
import { GUID_KEY } from './snapshot-compare-fixtures.mjs';
import { CHECK_PROFILE_SCHEMA } from './snapshot-check.mjs';

// ─── solids ─────────────────────────────────────────────────────────────────────────────────────────

const BOX_FACES = [   // corner (x,y,z) bits, outward, counter-clockwise
  [[1, 0, 0], [1, 1, 0], [1, 1, 1], [1, 0, 1]], [[0, 0, 0], [0, 0, 1], [0, 1, 1], [0, 1, 0]],
  [[0, 1, 0], [0, 1, 1], [1, 1, 1], [1, 1, 0]], [[0, 0, 0], [1, 0, 0], [1, 0, 1], [0, 0, 1]],
  [[0, 0, 1], [1, 0, 1], [1, 1, 1], [0, 1, 1]], [[0, 0, 0], [0, 1, 0], [1, 1, 0], [1, 0, 0]],
];

/** A closed box as a triangle list: `{ tris: [[p, p, p], ...] }` with p = [x, y, z]. */
export function box(min, max) {
  const tris = [];
  for (const face of BOX_FACES) {
    const q = face.map(([x, y, z]) => [x ? max[0] : min[0], y ? max[1] : min[1], z ? max[2] : min[2]]);
    tris.push([q[0], q[1], q[2]], [q[0], q[2], q[3]]);
  }
  return { tris };
}

const signedArea = (poly) => poly.reduce((sum, p, i) => { const q = poly[(i + 1) % poly.length]; return sum + (p[0] * q[1] - q[0] * p[1]); }, 0) / 2;
const insideTriangle = (p, a, b, c) => {
  const s = (u, v, w) => (u[0] - w[0]) * (v[1] - w[1]) - (v[0] - w[0]) * (u[1] - w[1]);
  const d1 = s(p, a, b); const d2 = s(p, b, c); const d3 = s(p, c, a);
  return !((d1 < 0 || d2 < 0 || d3 < 0) && (d1 > 0 || d2 > 0 || d3 > 0));
};

/** Ear clipping of a simple polygon; any winding. Returns triangles counter-clockwise. */
function earClip(input) {
  const poly = signedArea(input) < 0 ? [...input].reverse() : [...input];
  const out = [];
  let guard = 0;
  while (poly.length > 3 && guard++ < 1000) {
    let clipped = false;
    for (let i = 0; i < poly.length; i += 1) {
      const a = poly[(i + poly.length - 1) % poly.length]; const b = poly[i]; const c = poly[(i + 1) % poly.length];
      if ((b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]) <= 0) continue;
      if (poly.some((p) => p !== a && p !== b && p !== c && insideTriangle(p, a, b, c))) continue;
      out.push([a, b, c]);
      poly.splice(i, 1);
      clipped = true;
      break;
    }
    if (!clipped) throw new Error('ear clipping failed');
  }
  out.push([poly[0], poly[1], poly[2]]);
  return out;
}

/**
 * Extrude a polygon (given in two of the three axes) between two values of the third.
 * `axes` = [u, v, w]: polygon points are (u, v) pairs; the extrusion runs along w from w0 to w1.
 */
export function extrude(poly, axes, w0, w1) {
  const place = ([u, v], w) => { const p = [0, 0, 0]; p[axes[0]] = u; p[axes[1]] = v; p[axes[2]] = w; return p; };
  const tris = [];
  for (const [a, b, c] of earClip(poly)) {
    tris.push([place(a, w1), place(b, w1), place(c, w1)]);        // cap facing +w (counter-clockwise in u-v)
    tris.push([place(a, w0), place(c, w0), place(b, w0)]);        // cap facing -w
  }
  const ccw = signedArea(poly) >= 0 ? poly : [...poly].reverse();
  for (let i = 0; i < ccw.length; i += 1) {
    const p = ccw[i]; const q = ccw[(i + 1) % ccw.length];
    tris.push([place(p, w0), place(q, w0), place(q, w1)], [place(p, w0), place(q, w1), place(p, w1)]);
  }
  // The mapping of (u, v, w) onto (x, y, z) may mirror the solid: make every triangle face outward.
  let volume = 0;
  for (const [a, b, c] of tris) volume += (a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0]) + a[2] * (b[0] * c[1] - b[1] * c[0])) / 6;
  return { tris: volume < 0 ? tris.map(([a, b, c]) => [a, c, b]) : tris };
}

/** An I-section of flange width `bf`, depth `h`, flange `tf` and web `tw`, centred on (cx, cy), running along z from z0 to z1. */
export function iSection(cx, cy, bf, h, tf, tw, z0, z1) {
  const poly = [
    [-bf / 2, -h / 2], [bf / 2, -h / 2], [bf / 2, -h / 2 + tf], [tw / 2, -h / 2 + tf], [tw / 2, h / 2 - tf], [bf / 2, h / 2 - tf],
    [bf / 2, h / 2], [-bf / 2, h / 2], [-bf / 2, h / 2 - tf], [-tw / 2, h / 2 - tf], [-tw / 2, -h / 2 + tf], [-bf / 2, -h / 2 + tf],
  ].map(([x, y]) => [x + cx, y + cy]);
  return extrude(poly, [0, 1, 2], z0, z1);
}

export const translate = (solid, [dx, dy, dz]) => ({ tris: solid.tris.map((t) => t.map(([x, y, z]) => [x + dx, y + dy, z + dz])) });
/** Remove the triangle at `index` (an open mesh). */
export const withoutTriangle = (solid, index) => ({ tris: solid.tris.filter((_, i) => i !== index) });
/** Reverse one triangle's winding (a flipped triangle). */
export const withFlippedTriangle = (solid, index) => ({ tris: solid.tris.map((t, i) => (i === index ? [t[0], t[2], t[1]] : t)) });

// ─── GLB ────────────────────────────────────────────────────────────────────────────────────────────

/**
 * One tile. Each node: `{ name, solid, indexed?: bool, matrix?: number[16] }`. Flat-shaded (3 vertices per triangle)
 * unless `indexed`, which shares corners and writes 16-bit indices.
 */
export function glb(nodes) {
  const views = []; const accessors = []; const meshes = []; const gltfNodes = [];
  const chunks = []; let offset = 0;
  const push = (bytes) => { const pad = (4 - (bytes.length % 4)) % 4; const padded = Buffer.concat([bytes, Buffer.alloc(pad)]); chunks.push(padded); const at = offset; offset += padded.length; return [at, bytes.length]; };
  for (const node of nodes) {
    let positions; let indices = null;
    if (node.indexed) {
      const map = new Map(); const list = []; indices = [];
      for (const tri of node.solid.tris) for (const p of tri) {
        const key = p.join(',');
        if (!map.has(key)) { map.set(key, list.length); list.push(p); }
        indices.push(map.get(key));
      }
      positions = Float32Array.from(list.flat());
    } else positions = Float32Array.from(node.solid.tris.flat(2));
    const [posAt, posLen] = push(Buffer.from(positions.buffer));
    views.push({ buffer: 0, byteOffset: posAt, byteLength: posLen, target: 34962 });
    const count = positions.length / 3;
    const min = [Infinity, Infinity, Infinity]; const max = [-Infinity, -Infinity, -Infinity];
    for (let i = 0; i < positions.length; i += 3) for (let a = 0; a < 3; a += 1) { min[a] = Math.min(min[a], positions[i + a]); max[a] = Math.max(max[a], positions[i + a]); }
    accessors.push({ bufferView: views.length - 1, componentType: 5126, count, type: 'VEC3', min, max });
    const primitive = { attributes: { POSITION: accessors.length - 1 }, mode: 4 };
    if (indices) {
      const [idxAt, idxLen] = push(Buffer.from(Uint16Array.from(indices).buffer));
      views.push({ buffer: 0, byteOffset: idxAt, byteLength: idxLen, target: 34963 });
      accessors.push({ bufferView: views.length - 1, componentType: 5123, count: indices.length, type: 'SCALAR' });
      primitive.indices = accessors.length - 1;
    }
    meshes.push({ primitives: [primitive] });
    const gltfNode = { name: node.name, mesh: meshes.length - 1 };
    if (node.matrix) gltfNode.matrix = node.matrix;
    gltfNodes.push(gltfNode);
  }
  const binary = Buffer.concat(chunks);
  const json = { asset: { version: '2.0' }, scene: 0, scenes: [{ nodes: gltfNodes.map((_, i) => i) }], nodes: gltfNodes, meshes, accessors, bufferViews: views, buffers: [{ byteLength: binary.length }] };
  let jsonBytes = Buffer.from(JSON.stringify(json), 'utf8');
  jsonBytes = Buffer.concat([jsonBytes, Buffer.alloc((4 - (jsonBytes.length % 4)) % 4, 0x20)]);
  const total = 12 + 8 + jsonBytes.length + 8 + binary.length;
  const header = Buffer.alloc(12); header.writeUInt32LE(0x46546c67, 0); header.writeUInt32LE(2, 4); header.writeUInt32LE(total, 8);
  const chunkHead = (len, type) => { const b = Buffer.alloc(8); b.writeUInt32LE(len, 0); b.writeUInt32LE(type, 4); return b; };
  return Buffer.concat([header, chunkHead(jsonBytes.length, 0x4e4f534a), jsonBytes, chunkHead(binary.length, 0x004e4942), binary]);
}

export const sha = (bytes) => createHash('sha256').update(bytes).digest('hex');
export const tileOf = (bytes, file = 'tile-0.glb', ordinal = 0) => ({ ordinal, file, sha256: sha(bytes), bytes });

// ─── the world ──────────────────────────────────────────────────────────────────────────────────────

export const CHECK_NAMES = ['geometry-reconstruction', 'operations-evaluated', 'recipes-evaluated', 'placement-confirmed'];
export const okChecks = () => CHECK_NAMES.map((name) => ({ name, state: 'pass', code: 'ok' }));
export const openCheck = (name, code, state = 'open') => CHECK_NAMES.map((n) => (n === name ? { name: n, state, code } : { name: n, state: 'pass', code: 'ok' }));

/**
 * Parts: `{ guid, type = 'Beam', solid?, nodeSolid? (raw triangle list for the tile, defaults to `solid`),
 *   indexed?, coverage = 'ok' | null | checks[], bolted?: guid[], welded?: guid[], name? }`.
 * `hosts`: `[[assemblyGuid, childGuid], ...]`. Entity ids are element:<position>, node names object-<position>.
 */
export function world(parts, { hosts = [], contains = [] } = {}) {
  const entities = []; const properties = []; const ids = new Map(); const nodes = [];
  parts.forEach((part, index) => {
    const entityId = `element:${index + 1}`;
    ids.set(part.guid, entityId);
    const entity = { id: entityId, type: part.type ?? 'Beam', name: part.name ?? `part ${part.guid}` };
    if (part.solid) {
      const bounds = [Infinity, Infinity, Infinity, -Infinity, -Infinity, -Infinity];
      for (const tri of part.solid.tris) for (const p of tri) for (let a = 0; a < 3; a += 1) { bounds[a] = Math.min(bounds[a], p[a]); bounds[a + 3] = Math.max(bounds[a + 3], p[a]); }
      entity.geometry = [{ bounds }];   // what the facade carries, so a moved part is a changed part
    }
    entities.push(entity);
    let n = 0;
    const push = (sourceKey, value) => {
      properties.push({ id: `prop:${entityId}:${n}`, entityId, groupName: 'g', groupOrdinal: 0, name: sourceKey, value, valueType: 'string', unit: null, propertyOrdinal: n, semantic: 'x', dimension: 'none', isUda: false, sourceKey });
      n += 1;
    };
    push(GUID_KEY, part.guid);
    const coverage = part.coverage === undefined ? 'ok' : part.coverage;
    // The app writes its coverage rows with an id and NO sourceKey.
    if (coverage !== null) {
      properties.push({ entityId, id: `floless:tekla:coverage:${index + 1}`, name: `floless:tekla:coverage:${index + 1}`, source: 'Tekla Parser audit', status: 'read-only', unit: null, valueType: 'string', value: JSON.stringify({ checks: coverage === 'ok' ? okChecks() : coverage }) });
    }
    if (part.bolted !== undefined) push('native:bolted-parts', part.bolted.join(','));
    if (part.welded !== undefined) push('native:welded-parts', part.welded.join(','));
    if (part.solid) nodes.push({ name: `object-${index + 1}`, solid: part.solid, indexed: part.indexed });
  });
  const relationships = [];
  for (const [kind, list] of [['hosts', hosts], ['contains', contains]]) {
    for (const [from, to] of list) if (ids.has(from) && ids.has(to)) relationships.push({ id: `rel:${relationships.length}`, kind, from: ids.get(from), to: ids.get(to) });
  }
  const docs = { entities: { entities }, properties: { properties }, relationships: { relationships } };
  return { docs, tiles: { tiles: nodes.length === 0 ? [] : [tileOf(glb(nodes))] } };
}

// ─── the check profile ──────────────────────────────────────────────────────────────────────────────

export const SOLID_TYPES = ['Beam', 'Column', 'Contour plate', 'Physical', 'Hardware', 'Bolt group', 'Item'];

export const geometryBlock = (patch = {}) => ({
  node: { entityIdPattern: '^element:([0-9]+)$', nodeName: 'object-$1' },
  units: 'mm', contactTolerance: 0.1, weldTolerance: 0.001,
  qualification: { propertyKeyPattern: '^floless:tekla:coverage:', propertyField: 'id', json: 'checks', checks: CHECK_NAMES, passStates: ['pass', 'not-applicable'] },
  joins: {
    bolt: { types: ['Bolt group'], plies: { property: 'native:bolted-parts', format: 'identity-key-list' } },
    weld: { types: ['Physical'], parts: { property: 'native:welded-parts', format: 'identity-key-list' }, unknownWhen: { property: 'native:welded-parts', absent: true } },
  },
  groups: [{ kind: 'hosts', name: 'same assembly' }, { kind: 'contains', name: 'same component' }],
  budgets: { trianglesPerObject: 200000, trianglePairsPerPair: 4000000, gapSearchMm: 50, pairs: 200000, focusSets: 20000 },
  ...patch,
});

export const CLASH_RULES = {
  overlap: { id: 'clash.interpenetration', version: '1.0.0', severity: 'error', predicate: 'clash-overlap', target: 'solid', appliesTo: { types: SOLID_TYPES }, worsening: { path: 'overlapThicknessMm', direction: 'higher-is-worse' } },
  contact: { id: 'clash.unjoined-contact', version: '1.0.0', severity: 'warning', predicate: 'clash-contact', target: 'solid', appliesTo: { types: SOLID_TYPES }, worsening: null },
  bolted: { id: 'clash.bolted-plies-apart', version: '1.0.0', severity: 'warning', predicate: 'clash-bolted-plies', target: 'solid', appliesTo: { types: ['Bolt group'] }, worsening: { path: 'gapMm', direction: 'higher-is-worse' } },
};

export const clashChecks = ({ rules = ['overlap', 'contact', 'bolted'], geometry = {}, budgets = {} } = {}) => ({
  schema: CHECK_PROFILE_SCHEMA, id: 'floless.change-checks', version: '1.1.0',
  budgets: { findings: 2000, focusResults: 20000, ...budgets },
  rules: rules.map((key) => CLASH_RULES[key]),
  geometry: geometryBlock(geometry),
});

// ─── the pre-clash scenario (pins that a profile WITHOUT clash rules is unchanged) ───────────────────

function legacySide(objects) {
  const entities = []; const properties = [];
  objects.forEach((o, index) => {
    const entityId = `element:${index + 1}`;
    const entity = { id: entityId, type: o.type ?? 'Beam' };
    if (o.name !== undefined) entity.name = o.name;
    entities.push(entity);
    let n = 0;
    const push = (sourceKey, value) => {
      properties.push({ id: `prop:${entityId}:${n}`, entityId, groupName: 'g', groupOrdinal: 0, name: sourceKey, value, valueType: 'string', unit: null, propertyOrdinal: n, semantic: 'x', dimension: 'none', isUda: false, sourceKey });
      n += 1;
    };
    push(GUID_KEY, o.guid);
    push('native:profile', o.profile ?? 'HEA200');
  });
  return { entities: { entities }, properties: { properties }, relationships: { relationships: [] } };
}

/** Every presence a `blank` rule can produce, plus an absent and an excluded focus identity. */
export function legacyScenario() {
  const base = legacySide([
    { guid: 'G2', name: '' }, { guid: 'G3', name: '' }, { guid: 'G4', name: 'Fine' }, { guid: 'G5', name: '' }, { guid: 'G6', name: '' }, { guid: 'G7', name: 'Fine' },
  ]);
  const revised = legacySide([
    { guid: 'G1', name: '' }, { guid: 'G2', name: '', profile: 'HEB300' }, { guid: 'G3', name: ' ' }, { guid: 'G4', name: '' },
    { guid: 'G5', name: 'Now named' }, { guid: 'G6', name: '' }, { guid: 'G7', name: 'Fine' },
  ]);
  const checkProfile = {
    schema: CHECK_PROFILE_SCHEMA, id: 'floless.change-checks', version: '1.0.0',
    budgets: { findings: 2000, focusResults: 20000 },
    rules: [{ id: 'integrity.part-name-blank', version: '1.0.0', severity: 'warning', predicate: 'blank', target: 'entity.name', appliesTo: { types: ['Beam', 'Column'] }, worsening: null }],
  };
  return { base, revised, checkProfile, focus: { identities: ['lineage-A:G6', 'lineage-A:G9'] } };
}
