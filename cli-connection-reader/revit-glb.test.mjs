import assert from 'node:assert/strict';
import test from 'node:test';
import { makeGlbFixture } from './model-fixtures.mjs';
import { normalizeRevitGlb, parseGlb } from './revit-glb.mjs';

function replaceGlbJson(input, jsonText) {
  const parsed = parseGlb(input);
  const jsonBytes = Buffer.from(jsonText, 'utf8');
  const padded = (jsonBytes.length + 3) & ~3;
  const bin = parsed.binary;
  const out = Buffer.alloc(12 + 8 + padded + 8 + bin.length);
  out.write('glTF', 0, 'ascii'); out.writeUInt32LE(2, 4); out.writeUInt32LE(out.length, 8);
  out.writeUInt32LE(padded, 12); out.writeUInt32LE(0x4e4f534a, 16); out.fill(0x20, 20, 20 + padded); jsonBytes.copy(out, 20);
  const offset = 20 + padded; out.writeUInt32LE(bin.length, offset); out.writeUInt32LE(0x004e4942, offset + 4); bin.copy(out, offset + 8);
  return out;
}

test('active-scene geometry is transformed from glTF Y-up metres to Z-up millimetres', () => {
  const input = makeGlbFixture({
    positions: [[0, 0, 0], [1, 0, 0], [0, 0, 2]],
    indices: [0, 1, 2],
    translation: [2, 3, 4],
    nodeName: 'wall-a',
  });
  const result = normalizeRevitGlb(input);
  assert.deepEqual(result.parts[0].positions, [
    [2000, -6000, 3000], [2000, -4000, 3000], [3000, -4000, 3000],
  ]);
  assert.deepEqual(result.parts[0].triangles, [[0, 1, 2]]);
  assert.equal(result.parts[0].nodeName, 'wall-a');
  assert.equal(parseGlb(result.glb).json.scene, 0);
});

test('declared active scene is mandatory and inactive scenes are never traversed', () => {
  assert.throws(() => normalizeRevitGlb(makeGlbFixture({ omitScene: true })), /explicit active scene/);
  const input = makeGlbFixture({
    scenes: [{ nodes: [0] }, { nodes: [1] }],
    scene: 1,
    nodes: [{ name: 'inactive', mesh: 0 }, { name: 'active', mesh: 0 }],
  });
  assert.deepEqual(normalizeRevitGlb(input).parts.map((part) => part.nodeName), ['active']);
});

test('triangle strips and fans expand deterministically and negative transforms preserve front faces', () => {
  const positions = [[0, 0, 0], [1, 0, 0], [0, 1, 0], [1, 1, 0]];
  const strip = normalizeRevitGlb(makeGlbFixture({ positions, indices: [0, 1, 2, 3], mode: 5 }));
  assert.equal(strip.parts[0].triangles.length, 2);
  const fan = normalizeRevitGlb(makeGlbFixture({ positions, indices: [0, 1, 3, 2], mode: 6 }));
  assert.equal(fan.parts[0].triangles.length, 2);
  const ordinary = normalizeRevitGlb(makeGlbFixture());
  const reflected = normalizeRevitGlb(makeGlbFixture({ scale: [-1, 1, 1] }));
  assert.deepEqual(reflected.coverage, { inputTriangles: 1, outputTriangles: 1, droppedDegenerateTriangles: 0 });
  assert.deepEqual(ordinary.parts[0].triangles, [[0, 2, 1]]);
  assert.deepEqual(reflected.parts[0].triangles, [[0, 1, 2]]);
});

test('vertex and triangle permutations produce identical canonical GLB bytes', () => {
  const first = makeGlbFixture({
    positions: [[0, 0, 0], [1, 0, 0], [0, 1, 0], [1, 1, 0]],
    indices: [0, 1, 2, 1, 3, 2],
  });
  const second = makeGlbFixture({
    positions: [[1, 1, 0], [0, 1, 0], [1, 0, 0], [0, 0, 0]],
    indices: [2, 0, 1, 3, 2, 1],
  });
  assert.deepEqual(normalizeRevitGlb(first).glb, normalizeRevitGlb(second).glb);
});

test('vertex color is multiplied by material base color and encoded as canonical RGBA', () => {
  const result = normalizeRevitGlb(makeGlbFixture({
    colors: [[1, 0.5, 0, 1], [0.5, 1, 0, 0.5], [0, 0.5, 1, 1]],
    materialColor: [0.5, 0.5, 1, 0.5],
  }));
  assert.deepEqual(result.parts[0].colors[0], [0.5, 0.25, 0, 0.5]);
  assert.deepEqual(result.parts[0].colors[1], [0, 0.25, 1, 0.5]);
  assert.deepEqual(result.parts[0].colors[2], [0.25, 0.5, 0, 0.25]);
});

test('xeoRVT material presentation survives canonicalization without provider names', () => {
  const result = normalizeRevitGlb(makeGlbFixture({
    material: {
      name: 'provider-specific glass',
      alphaMode: 'BLEND',
      doubleSided: true,
      pbrMetallicRoughness: {
        baseColorFactor: [0.25, 0.5, 0.75, 0.15],
        metallicFactor: 0,
        roughnessFactor: 0.5,
      },
    },
  }));
  const canonical = parseGlb(result.glb).json;
  assert.deepEqual(canonical.materials, [{
    alphaMode: 'BLEND',
    doubleSided: true,
    pbrMetallicRoughness: { baseColorFactor: [1, 1, 1, 1], metallicFactor: 0, roughnessFactor: 0.5 },
  }]);
  assert.equal(canonical.meshes[0].primitives[0].material, 0);
  assert.deepEqual(result.parts[0].colors[0], [0.25, 0.5, 0.75, Math.fround(0.15)]);
  assert.throws(
    () => normalizeRevitGlb(makeGlbFixture({ material: { alphaMode: 'MASK' } })),
    /alphaMode/,
  );
});

test('xeoRVT normals use inverse-transpose and survive canonicalization', () => {
  const normals = [[0, 0, 1], [0, 0, 1], [0, 0, 1]];
  const result = normalizeRevitGlb(makeGlbFixture({ normals, scale: [2, 1, 0.5] }));
  assert.deepEqual(result.parts[0].normals, [[0, -1, 0], [0, -1, 0], [0, -1, 0]]);
  const canonical = parseGlb(result.glb).json;
  assert.equal(canonical.meshes[0].primitives[0].attributes.NORMAL, 2);
  assert.equal(canonical.accessors[2].type, 'VEC3');
  assert.throws(
    () => normalizeRevitGlb(makeGlbFixture({ normals: normals.slice(0, 2) })),
    /NORMAL count/,
  );
});

test('unsafe resources, unsupported extensions, scene cycles, and malformed ranges are refused', () => {
  assert.throws(() => normalizeRevitGlb(makeGlbFixture({ externalUri: 'https://example.test/model.bin' })), /external resource/);
  assert.throws(() => normalizeRevitGlb(makeGlbFixture({ material: [] })), /material must be an object/);
  assert.throws(() => normalizeRevitGlb(makeGlbFixture({ material: { pbrMetallicRoughness: [] } })), /PBR material must be an object/);
  assert.throws(() => normalizeRevitGlb(makeGlbFixture({ material: { pbrMetallicRoughness: null } })), /PBR material must be an object/);
  assert.throws(() => normalizeRevitGlb(makeGlbFixture({ material: { emissiveFactor: null } })), /emissiveFactor/);
  assert.throws(() => normalizeRevitGlb(makeGlbFixture({ extensionsUsed: ['KHR_draco_mesh_compression'] })), /extensions/);
  assert.throws(() => normalizeRevitGlb(makeGlbFixture({ nodes: [{ name: 'cycle', mesh: 0, children: [0] }] })), /cycle/);
  const truncated = makeGlbFixture().subarray(0, -1);
  assert.throws(() => normalizeRevitGlb(truncated), /length/);
});

test('embedded data URI buffers normalize identically while malformed Base64 is refused', () => {
  const valid = makeGlbFixture();
  const parsed = parseGlb(valid);
  const document = structuredClone(parsed.json);
  document.buffers.push({
    byteLength: parsed.binary.length,
    uri: `data:application/octet-stream;base64,${parsed.binary.toString('base64')}`,
  });
  document.bufferViews = document.bufferViews.map((view) => ({ ...view, buffer: 1 }));
  const embedded = replaceGlbJson(valid, JSON.stringify(document));
  assert.deepEqual(normalizeRevitGlb(embedded).glb, normalizeRevitGlb(valid).glb);

  const dataFirst = structuredClone(parsed.json);
  dataFirst.buffers[0].uri = `data:application/octet-stream;base64,${parsed.binary.toString('base64')}`;
  assert.throws(
    () => normalizeRevitGlb(replaceGlbJson(valid, JSON.stringify(dataFirst))),
    /buffers\[0\].*BIN chunk/,
  );

  document.buffers[1].uri = 'data:application/octet-stream;base64,not-valid***';
  assert.throws(
    () => normalizeRevitGlb(replaceGlbJson(valid, JSON.stringify(document))),
    /data URI|Base64/,
  );
});

test('duplicate keys in the GLB JSON chunk are rejected before profile validation', () => {
  const valid = makeGlbFixture();
  const parsed = parseGlb(valid);
  const text = parsed.jsonText;
  const duplicate = text.replace('{"accessors"', '{"x":0,"x":1,"accessors"');
  assert.throws(() => normalizeRevitGlb(replaceGlbJson(valid, duplicate)), /duplicate JSON key/);
});

test('observed xeoRVT JSON chunk fits the default while a caller-lowered cap remains enforced', () => {
  const valid = makeGlbFixture();
  const parsed = parseGlb(valid);
  const observedChunkBytes = 8_879_700;
  const document = { ...parsed.json, asset: { ...parsed.json.asset, generator: '' } };
  const fixedBytes = Buffer.byteLength(JSON.stringify(document));
  document.asset.generator = 'x'.repeat(observedChunkBytes - fixedBytes);
  const large = replaceGlbJson(valid, JSON.stringify(document));
  assert.equal(large.readUInt32LE(12), observedChunkBytes);
  assert.equal(normalizeRevitGlb(large).parts.length, 1);
  assert.throws(
    () => normalizeRevitGlb(large, { limits: { maxGlbJsonBytes: 4 * 1024 * 1024 } }),
    /GLB JSON exceeds its byte limit/,
  );
});

test('large valid vertex sets compute bounds without variadic stack overflow', () => {
  const positions = Array.from({ length: 130_000 }, (_, index) => [index, index % 3, 0]);
  const result = normalizeRevitGlb(makeGlbFixture({ positions, indices: [0, 1, 2] }), {
    limits: { maxVertices: positions.length },
  });
  const canonical = parseGlb(result.glb).json;
  assert.deepEqual(canonical.accessors[0].min, [0, 0, 0]);
  assert.deepEqual(canonical.accessors[0].max, [129_999_000, 0, 2000]);
});

test('vertex and index limits apply to the complete active model, not each primitive', () => {
  const input = makeGlbFixture({ primitiveCopies: 2 });
  assert.throws(
    () => normalizeRevitGlb(input, { limits: { maxVertices: 3, maxIndices: 3 } }),
    /count exceeds its limit/,
  );
});

test('skins and animations are rejected instead of publishing untransformed bind-pose geometry', () => {
  assert.throws(() => normalizeRevitGlb(makeGlbFixture({ skin: 0, skins: [{ joints: [0] }] })), /skin/);
  assert.throws(() => normalizeRevitGlb(makeGlbFixture({ animations: [{ channels: [], samplers: [] }] })), /animation/);
});
