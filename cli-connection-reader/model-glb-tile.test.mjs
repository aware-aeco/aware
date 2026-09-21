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
  assert.deepEqual(validateGlbTile(paddedTile()), [1, 2, 3, 1, 2, 3]);
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
