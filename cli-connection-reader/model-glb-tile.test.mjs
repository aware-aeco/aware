import assert from 'node:assert/strict';
import test from 'node:test';

import { canonicalJsonBytes } from './model-contract.mjs';
import { validateGlbTile } from './model-glb-tile.mjs';

function paddedTile() {
  const binary = Buffer.alloc(16);
  binary.writeFloatLE(1, 0); binary.writeFloatLE(2, 4); binary.writeFloatLE(3, 8);
  binary[12] = 7;
  const document = {
    asset: { version: '2.0' }, scene: 0, scenes: [{ nodes: [0] }],
    nodes: [{ mesh: 0 }], meshes: [{ primitives: [{ attributes: { POSITION: 0 } }] }],
    buffers: [{ byteLength: 13 }],
    bufferViews: [{ buffer: 0, byteOffset: 0, byteLength: 12 }],
    accessors: [{ bufferView: 0, componentType: 5126, count: 1, type: 'VEC3' }],
  };
  let json = canonicalJsonBytes(document);
  json = Buffer.concat([json, Buffer.alloc((4 - (json.length % 4)) % 4, 0x20)]);
  const output = Buffer.alloc(28 + json.length + binary.length);
  output.writeUInt32LE(0x46546c67, 0); output.writeUInt32LE(2, 4);
  output.writeUInt32LE(output.length, 8); output.writeUInt32LE(json.length, 12);
  output.writeUInt32LE(0x4e4f534a, 16); json.copy(output, 20);
  output.writeUInt32LE(binary.length, 20 + json.length);
  output.writeUInt32LE(0x004e4942, 24 + json.length); binary.copy(output, 28 + json.length);
  return output;
}

test('GLB tiles admit up to three legal BIN padding bytes beyond buffers[0].byteLength', () => {
  assert.deepEqual(validateGlbTile(paddedTile()), {
    bounds: [1, 2, 3, 1, 2, 3], primitiveCount: 1,
  });
});

test('GLB tiles refuse a BIN chunk with more than three undeclared bytes', () => {
  const tile = paddedTile();
  const jsonLength = tile.readUInt32LE(12);
  const document = JSON.parse(tile.subarray(20, 20 + jsonLength).toString('utf8'));
  document.buffers[0].byteLength = 12;
  let json = canonicalJsonBytes(document);
  json = Buffer.concat([json, Buffer.alloc((4 - (json.length % 4)) % 4, 0x20)]);
  const binary = tile.subarray(28 + jsonLength);
  const output = Buffer.alloc(28 + json.length + binary.length);
  output.writeUInt32LE(0x46546c67, 0); output.writeUInt32LE(2, 4);
  output.writeUInt32LE(output.length, 8); output.writeUInt32LE(json.length, 12);
  output.writeUInt32LE(0x4e4f534a, 16); json.copy(output, 20);
  output.writeUInt32LE(binary.length, 20 + json.length);
  output.writeUInt32LE(0x004e4942, 24 + json.length); binary.copy(output, 28 + json.length);
  assert.throws(() => validateGlbTile(output), /self-contained GLB/);
});

test('GLB tiles refuse deformation and malformed primitive accessors', () => {
  const tile = paddedTile();
  const jsonLength = tile.readUInt32LE(12);
  const original = JSON.parse(tile.subarray(20, 20 + jsonLength).toString('utf8'));
  const binary = tile.subarray(28 + jsonLength);
  for (const mutate of [
    (document) => { document.meshes[0].primitives[0].targets = [{ POSITION: 0 }]; },
    (document) => { document.nodes[0].skin = 0; document.skins = [{ joints: [0] }]; },
    (document) => { document.animations = [{ channels: [], samplers: [] }]; },
    (document) => { document.meshes[0].primitives[0].attributes.NORMAL = 99; },
    (document) => {
      document.bufferViews.push({ buffer: 0, byteOffset: 12, byteLength: 1 });
      document.accessors.push({ bufferView: 1, componentType: 5121, count: 1, type: 'SCALAR' });
      document.meshes[0].primitives[0].indices = 1;
    },
  ]) {
    const document = structuredClone(original); mutate(document);
    let json = canonicalJsonBytes(document);
    json = Buffer.concat([json, Buffer.alloc((4 - (json.length % 4)) % 4, 0x20)]);
    const output = Buffer.alloc(28 + json.length + binary.length);
    output.writeUInt32LE(0x46546c67, 0); output.writeUInt32LE(2, 4);
    output.writeUInt32LE(output.length, 8); output.writeUInt32LE(json.length, 12);
    output.writeUInt32LE(0x4e4f534a, 16); json.copy(output, 20);
    output.writeUInt32LE(binary.length, 20 + json.length);
    output.writeUInt32LE(0x004e4942, 24 + json.length); binary.copy(output, 28 + json.length);
    assert.throws(() => validateGlbTile(output), /flattened|primitive|accessor/);
  }
});

test('GLB tiles refuse geometry outside the rendered scene', () => {
  const tile = paddedTile();
  const jsonLength = tile.readUInt32LE(12);
  const document = JSON.parse(tile.subarray(20, 20 + jsonLength).toString('utf8'));
  document.scenes[0].nodes = [];
  let json = canonicalJsonBytes(document);
  json = Buffer.concat([json, Buffer.alloc((4 - (json.length % 4)) % 4, 0x20)]);
  const binary = tile.subarray(28 + jsonLength);
  const output = Buffer.alloc(28 + json.length + binary.length);
  output.writeUInt32LE(0x46546c67, 0); output.writeUInt32LE(2, 4);
  output.writeUInt32LE(output.length, 8); output.writeUInt32LE(json.length, 12);
  output.writeUInt32LE(0x4e4f534a, 16); json.copy(output, 20);
  output.writeUInt32LE(binary.length, 20 + json.length);
  output.writeUInt32LE(0x004e4942, 24 + json.length); binary.copy(output, 28 + json.length);
  assert.throws(() => validateGlbTile(output), /rendered scene/);
});

function jsonOnlyGlb(document) {
  let json = canonicalJsonBytes(document);
  json = Buffer.concat([json, Buffer.alloc((4 - (json.length % 4)) % 4, 0x20)]);
  const output = Buffer.alloc(20 + json.length);
  output.writeUInt32LE(0x46546c67, 0); output.writeUInt32LE(2, 4);
  output.writeUInt32LE(output.length, 8); output.writeUInt32LE(json.length, 12);
  output.writeUInt32LE(0x4e4f534a, 16); json.copy(output, 20);
  return output;
}

test('an empty GLB tile is never admitted: "no geometry" is zero tiles, not an empty tile (#604)', () => {
  const refused = (tile) => assert.throws(() => validateGlbTile(tile),
    (error) => error.code === 'reference-geometry-invalid');
  // The shape a provider might emit for a model with nothing drawable.
  refused(jsonOnlyGlb({ asset: { version: '2.0' }, scene: 0, scenes: [{}] }));
  // Even a structurally complete document with an empty buffer and no meshes is refused.
  refused(jsonOnlyGlb({
    asset: { version: '2.0' }, scene: 0, scenes: [{ nodes: [] }], nodes: [], meshes: [],
    buffers: [{ byteLength: 0 }], bufferViews: [], accessors: [],
  }));
  refused(jsonOnlyGlb({
    asset: { version: '2.0' }, scene: 0, scenes: [{ nodes: [0] }], nodes: [{}], meshes: [],
    buffers: [{ byteLength: 0 }], bufferViews: [], accessors: [],
  }));
});

// --- #679: typed-array POSITION scan vs per-coordinate reads -----------------------------------

function positionTile({ points, stride, viewOffset = 0, accessorOffset = 0, lead = 0 }) {
  const elementBytes = 12;
  const rowBytes = stride ?? elementBytes;
  const dataStart = viewOffset;
  const binary = Buffer.alloc(dataStart + accessorOffset + rowBytes * points.length + 4, 0xab);
  points.forEach((point, index) => {
    point.forEach((value, axis) => binary.writeFloatLE(value, dataStart + accessorOffset + index * rowBytes + axis * 4));
  });
  const bufferView = { buffer: 0, byteOffset: dataStart, byteLength: binary.length - dataStart };
  if (stride !== undefined) bufferView.byteStride = stride;
  const document = {
    asset: { version: '2.0' }, scene: 0, scenes: [{ nodes: [0] }],
    nodes: [{ mesh: 0 }], meshes: [{ primitives: [{ attributes: { POSITION: 0 } }] }],
    buffers: [{ byteLength: binary.length }],
    bufferViews: [bufferView],
    accessors: [{ bufferView: 0, byteOffset: accessorOffset, componentType: 5126, count: points.length, type: 'VEC3' }],
  };
  let json = canonicalJsonBytes(document);
  json = Buffer.concat([json, Buffer.alloc((4 - (json.length % 4)) % 4, 0x20)]);
  const output = Buffer.alloc(lead + 28 + json.length + binary.length);
  const at = lead;
  output.writeUInt32LE(0x46546c67, at); output.writeUInt32LE(2, at + 4);
  output.writeUInt32LE(output.length - lead, at + 8); output.writeUInt32LE(json.length, at + 12);
  output.writeUInt32LE(0x4e4f534a, at + 16); json.copy(output, at + 20);
  output.writeUInt32LE(binary.length, at + 20 + json.length);
  output.writeUInt32LE(0x004e4942, at + 24 + json.length); binary.copy(output, at + 28 + json.length);
  return output.subarray(lead);
}

const POINTS = [[-0, 2.5, -7], [10, -0, 3], [0.125, 9, -0], [-3.5, 1e6, 2]];
const POINT_BOUNDS = [-3.5, 0, -7, 10, 1e6, 3];

test('POSITION bounds agree across packed, strided, offset and unaligned layouts (#679)', () => {
  for (const layout of [
    { points: POINTS },
    { points: POINTS, stride: 16 },
    { points: POINTS, stride: 24 },
    { points: POINTS, viewOffset: 4, accessorOffset: 8 },
    { points: POINTS, stride: 20, viewOffset: 8 },
  ]) {
    const expected = { bounds: POINT_BOUNDS, primitiveCount: 1 };
    const result = validateGlbTile(positionTile(layout));
    assert.deepEqual(result, expected, JSON.stringify(layout));
    assert.ok(result.bounds.every((value) => !Object.is(value, -0)), 'negative zero is canonicalized to zero');
    // A Buffer whose underlying memory is not 4-byte aligned takes the per-coordinate path.
    for (const lead of [1, 2, 3]) {
      assert.deepEqual(validateGlbTile(positionTile({ ...layout, lead })), expected, `${JSON.stringify(layout)} lead ${lead}`);
    }
  }
});

test('POSITION scan refuses non-finite and unsafe coordinates on every layout (#679)', () => {
  for (const bad of [Number.NaN, Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY, 3.4e38]) {
    for (const layout of [{}, { stride: 16 }, { lead: 1 }]) {
      assert.throws(() => validateGlbTile(positionTile({ points: [[0, 0, 0], [1, bad, 2]], ...layout })),
        /non-canonical coordinate/, `${bad} ${JSON.stringify(layout)}`);
    }
  }
});
