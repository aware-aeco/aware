import assert from 'node:assert/strict';
import test from 'node:test';
import { MODEL_LIMITS, canonicalArtifactLimits, lowerableLimits } from './model-contract.mjs';
import { makeGlbFixture } from './model-fixtures.mjs';
import { checkedCanonicalWorkBytes, normalizeRevitGlb, parseGlb } from './revit-glb.mjs';

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

function makeGlb(document, binary) {
  const jsonBytes = Buffer.from(JSON.stringify(document), 'utf8');
  const jsonLength = (jsonBytes.length + 3) & ~3;
  const binaryLength = (binary.length + 3) & ~3;
  const out = Buffer.alloc(12 + 8 + jsonLength + 8 + binaryLength);
  out.write('glTF', 0, 'ascii'); out.writeUInt32LE(2, 4); out.writeUInt32LE(out.length, 8);
  out.writeUInt32LE(jsonLength, 12); out.writeUInt32LE(0x4e4f534a, 16); out.fill(0x20, 20, 20 + jsonLength); jsonBytes.copy(out, 20);
  const offset = 20 + jsonLength; out.writeUInt32LE(binaryLength, offset); out.writeUInt32LE(0x004e4942, offset + 4); binary.copy(out, offset + 8);
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
    [2000, -4000, 3000], [2000, -6000, 3000], [3000, -4000, 3000],
  ]);
  assert.deepEqual(result.parts[0].triangles, [[0, 2, 1]]);
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
  assert.throws(
    () => normalizeRevitGlb(makeGlbFixture({ positions, indices: [0, 1, 2, 3], mode: 5 }), { limits: { maxIndices: 4 } }),
    (error) => error.code === 'reference-output-too-large' && /expanded model index count/.test(error.message),
  );
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
  assert.throws(() => normalizeRevitGlb(makeGlbFixture({ material: { pbrMetallicRoughness: { baseColorTexture: { index: 0 } } } })), /PBR property/);
  assert.throws(() => normalizeRevitGlb(makeGlbFixture({ material: { normalTexture: { index: 0 } } })), /material property/);
  assert.throws(() => normalizeRevitGlb(makeGlbFixture({ material: { emissiveFactor: null } })), /emissiveFactor/);
  assert.throws(() => normalizeRevitGlb(makeGlbFixture({ extensionsUsed: ['KHR_draco_mesh_compression'] })), /extensions/);
  assert.throws(() => normalizeRevitGlb(makeGlbFixture({ nodes: [{ name: 'cycle', mesh: 0, children: [0] }] })), /cycle/);
  const truncated = makeGlbFixture().subarray(0, -1);
  assert.throws(() => normalizeRevitGlb(truncated), /length/);
});

test('zero-length accessors are rejected before canonical GLB publication', () => {
  const fixture = makeGlbFixture();
  const parsed = parseGlb(fixture);
  parsed.json.accessors[0].count = 0;
  assert.throws(() => normalizeRevitGlb(makeGlb(parsed.json, parsed.binary)), /accessor count/);
});

test('degenerate-only primitives publish an empty canonical GLB without zero-count accessors', () => {
  const result = normalizeRevitGlb(makeGlbFixture({ indices: [0, 0, 0] }));
  assert.deepEqual(result.coverage, { inputTriangles: 1, outputTriangles: 0, droppedDegenerateTriangles: 1 });
  assert.equal(result.parts.length, 1);
  const canonical = parseGlb(result.glb).json;
  assert.deepEqual(canonical.accessors, []);
  assert.deepEqual(canonical.nodes, []);
  const repeated = normalizeRevitGlb(result.glb);
  assert.deepEqual(repeated.glb, result.glb);
});

test('canonical vertices are ordered by their final little-endian float32 bytes', () => {
  const result = normalizeRevitGlb(makeGlbFixture({
    positions: [[0, 0, 0], [-0.001, 0, 0], [0.001, 0, 0]],
    indices: [0, 1, 2],
  }));
  // Revit Y-up metres become Z-up millimetres. Float32 LE encodes 0 before -1,
  // even though numeric ordering would place -1 first.
  assert.deepEqual(result.parts[0].positions.map((position) => position[0]), [0, 1, -1]);
});

test('null scene, buffer views, meshes, and primitives retain typed geometry errors', () => {
  const valid = makeGlbFixture();
  const parsed = parseGlb(valid);
  const nullScene = structuredClone(parsed.json);
  nullScene.scenes[nullScene.scene] = null;
  assert.throws(
    () => normalizeRevitGlb(replaceGlbJson(valid, JSON.stringify(nullScene))),
    (error) => error.code === 'reference-geometry-invalid' && /active scene must be an object/.test(error.message),
  );
  const nullView = structuredClone(parsed.json);
  nullView.bufferViews[0] = null;
  assert.throws(
    () => normalizeRevitGlb(replaceGlbJson(valid, JSON.stringify(nullView))),
    (error) => error.code === 'reference-geometry-invalid' && /bufferView must be an object/.test(error.message),
  );
  const nullMesh = structuredClone(parsed.json);
  nullMesh.meshes[0] = null;
  assert.throws(
    () => normalizeRevitGlb(replaceGlbJson(valid, JSON.stringify(nullMesh))),
    (error) => error.code === 'reference-geometry-invalid' && /mesh must be an object/.test(error.message),
  );
  const nullPrimitive = structuredClone(parsed.json);
  nullPrimitive.meshes[0].primitives[0] = null;
  assert.throws(
    () => normalizeRevitGlb(replaceGlbJson(valid, JSON.stringify(nullPrimitive))),
    (error) => error.code === 'reference-geometry-invalid' && /primitive must be an object/.test(error.message),
  );
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

test('the 64 MiB GLB JSON boundary is accepted exactly and refused above the cap', () => {
  const valid = makeGlbFixture();
  const parsed = parseGlb(valid);
  const target = 64 * 1024 * 1024;
  const document = { ...parsed.json, asset: { ...parsed.json.asset, generator: '' } };
  const fixed = Buffer.byteLength(JSON.stringify(document));
  document.asset.generator = 'x'.repeat(target - fixed);
  const atLimit = replaceGlbJson(valid, JSON.stringify(document));
  assert.equal(atLimit.readUInt32LE(12), target);
  assert.doesNotThrow(() => parseGlb(atLimit));
  document.asset.generator += 'xxxx';
  const aboveLimit = replaceGlbJson(valid, JSON.stringify(document));
  assert.equal(aboveLimit.readUInt32LE(12), target + 4);
  assert.throws(() => parseGlb(aboveLimit), /GLB JSON exceeds its byte limit/);
});

test('canonical work accounting accepts 4 GiB exactly and refuses one more byte before allocation', () => {
  const limit = MODEL_LIMITS.maxCanonicalWorkBytes.default;
  assert.equal(checkedCanonicalWorkBytes(limit - 4, 1, 4, limit), limit);
  assert.throws(() => checkedCanonicalWorkBytes(limit, 1, 1, limit),
    (error) => error.code === 'reference-output-too-large' && /4294967296-byte limit/.test(error.message));
});

test('large valid vertex sets compute bounds without variadic stack overflow', () => {
  const positions = Array.from({ length: 130_000 }, (_, index) => [index, index % 3, 0]);
  const result = normalizeRevitGlb(makeGlbFixture({ positions, indices: [0, 1, 3] }), {
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

test('malformed UTF-8 in the GLB JSON chunk is rejected without replacement decoding', () => {
  const input = Buffer.from(makeGlbFixture());
  const nameOffset = input.indexOf(Buffer.from('part-a'));
  assert.notEqual(nameOffset, -1);
  input[nameOffset] = 0x80;
  assert.throws(() => normalizeRevitGlb(input), /valid UTF-8/);
});

test('multiple primitives remain grouped under one canonical node and can be normalized again', () => {
  const first = normalizeRevitGlb(makeGlbFixture({ primitiveCopies: 2 }));
  const document = parseGlb(first.glb).json;
  assert.equal(document.nodes.length, 1);
  assert.equal(document.meshes.length, 1);
  assert.equal(document.meshes[0].primitives.length, 2);
  assert.deepEqual(first.parts.map((part) => part.primitiveOrdinal), [0, 1]);
  const second = normalizeRevitGlb(first.glb);
  assert.deepEqual(second.parts.map((part) => [part.nodeName, part.primitiveOrdinal]),
    first.parts.map((part) => [part.nodeName, part.primitiveOrdinal]));
  const roundTripDocument = parseGlb(second.glb).json;
  assert.equal(roundTripDocument.nodes.length, 1);
  assert.equal(roundTripDocument.meshes[0].primitives.length, 2);
});

test('canonical output must remain inside a caller-lowered JSON profile', () => {
  assert.throws(
    () => normalizeRevitGlb(makeGlbFixture({ primitiveCopies: 32_000 }), { limits: { maxCanonicalGlbJsonBytes: 8 * 1024 * 1024 } }),
    (error) => error.code === 'reference-output-too-large' && /canonical GLB JSON/.test(error.message),
  );
});

test('canonical work accounting refuses overflow against the declared limit before allocation', () => {
  assert.throws(
    () => checkedCanonicalWorkBytes(7_933, 1, 4, 7_936),
    (error) => error.code === 'reference-output-too-large'
      && error.message === 'canonical geometry working set exceeds its 7936-byte limit',
  );
});

test('the default canonical JSON profile bounds a 32k-primitive expansion', () => {
  assert.throws(
    () => normalizeRevitGlb(makeGlbFixture({ primitiveCopies: 32_000 })),
    (error) => error.code === 'reference-output-too-large' && /canonical GLB JSON/.test(error.message),
  );
});

test('canonical object expansion is refused by the working-set gate before allocation', () => {
  const parsed = parseGlb(makeGlbFixture());
  const document = structuredClone(parsed.json);
  const count = 1_048_573;
  const indexOffset = count * 12;
  const binary = Buffer.alloc(indexOffset + 12);
  binary.writeUInt32LE(0, indexOffset); binary.writeUInt32LE(1, indexOffset + 4); binary.writeUInt32LE(2, indexOffset + 8);
  document.buffers[0].byteLength = binary.length;
  document.bufferViews[0].byteLength = indexOffset;
  document.bufferViews[1].byteOffset = indexOffset;
  document.accessors[0].count = count;
  // The declared budget is what refuses it, so the refusal must survive being pinned to that budget
  // rather than to the 1 GiB constant this gate used to carry.
  assert.throws(
    () => normalizeRevitGlb(makeGlb(document, binary), { limits: { maxCanonicalWorkBytes: 1024 * 1024 * 1024 } }),
    (error) => error.code === 'reference-output-too-large' && /working set exceeds/.test(error.message),
  );
});

test('the working-set gate is driven by its declared limit and names the ceiling it enforced', () => {
  // The default fixture reserves exactly 4096 (one primitive) + 3*1024 (vertices) + 3*128 (indices)
  // + 3*128 (expanded indices) = 7,936 estimated bytes, so the gate's boundary is exact and testable
  // without allocating anything near it.
  const input = makeGlbFixture();
  assert.equal(normalizeRevitGlb(input, { limits: { maxCanonicalWorkBytes: 7_936 } }).parts.length, 1);
  assert.throws(
    () => normalizeRevitGlb(input, { limits: { maxCanonicalWorkBytes: 7_935 } }),
    (error) => error.code === 'reference-output-too-large'
      // Pinned verbatim: a message naming a ceiling other than the one actually enforced is how the
      // fixed "1 GiB" wording lied under every override.
      && error.message === 'canonical geometry working set exceeds its 7935-byte limit',
  );
});

test('a model inside the declared vertex limit is not refused by an undeclared working-set ceiling', () => {
  // 4,194,303 indices over 999 vertices: inside maxVertices (5,000,000) and maxIndices (15,000,000),
  // and inside maxInputGlbBytes at ~16 MB. Its estimate is 1.001 GiB, which the superseded fixed
  // 1 GiB gate refused — so a model within every PUBLISHED limit was refused by an unpublished one.
  // Measured resident cost of this normalization is ~474 MiB, against the 1.001 GiB it reserves.
  const vertices = 999;
  const indexCount = 4_194_303;
  const positions = Array.from({ length: vertices }, (_, index) => [index % 97, (index * 7) % 89, (index * 13) % 83]);
  const indices = Array.from({ length: indexCount }, (_, index) => index % vertices);
  const estimate = vertices * 1024 + indexCount * 128 * 2 + 4096;
  assert.ok(estimate > 1024 * 1024 * 1024, 'fixture must exceed the superseded 1 GiB gate to prove anything');
  assert.ok(estimate <= MODEL_LIMITS.maxCanonicalWorkBytes.default);
  assert.ok(vertices <= MODEL_LIMITS.maxVertices.default && indexCount <= MODEL_LIMITS.maxIndices.default);
  const result = normalizeRevitGlb(makeGlbFixture({ positions, indices }));
  assert.equal(result.parts.length, 1);
  assert.equal(result.parts[0].triangles.length + result.coverage.droppedDegenerateTriangles, indexCount / 3);
});

test('the input GLB JSON bound admits a chunk that the total-GLB bound already admits', () => {
  // The figures are the authenticated Snowdon Towers provider artifact from #517: a 41,926,848-byte
  // GLB — inside maxInputGlbBytes' 128 MiB default — whose 23,576,276-byte JSON chunk the former
  // 16 MiB sub-limit refused. A sub-limit below what the enclosing limit admits is the defect.
  assert.ok(23_576_276 <= MODEL_LIMITS.maxGlbJsonBytes.default);
  assert.ok(41_926_848 <= MODEL_LIMITS.maxInputGlbBytes.default);
  assert.ok(MODEL_LIMITS.maxGlbJsonBytes.default <= MODEL_LIMITS.maxInputGlbBytes.default,
    'the JSON chunk is part of the GLB, so its bound may never exceed the GLB bound');
  // Still fails closed: the bound is enforced, not merely declared.
  const valid = makeGlbFixture();
  assert.throws(
    () => normalizeRevitGlb(valid, { limits: { maxGlbJsonBytes: 8 } }),
    (error) => error.code === 'reference-output-too-large' && /GLB JSON exceeds its byte limit/.test(error.message),
  );
});

test('the canonical output JSON bound is its own budget, not the input one', () => {
  // Folding both into one knob meant raising the input bound to admit a real provider GLB silently
  // raised the output bound too. These must move independently.
  assert.notEqual(MODEL_LIMITS.maxCanonicalGlbJsonBytes.default, MODEL_LIMITS.maxGlbJsonBytes.default);
  const wide = makeGlbFixture({ primitiveCopies: 32_000 });
  // Refused on the OUTPUT budget while the INPUT budget is at its (larger) default.
  assert.throws(
    () => normalizeRevitGlb(wide),
    (error) => error.code === 'reference-output-too-large' && /canonical GLB JSON/.test(error.message),
  );
  // ... and raising only the output budget is what lets it through, proving which bound refused it.
  assert.equal(normalizeRevitGlb(wide, { limits: { maxCanonicalGlbJsonBytes: 64 * 1024 * 1024 } }).parts.length, 32_000);
});

test('a canonical artifact re-reads under the canonical budgets, not the input ones', () => {
  // The reader parses its own emitted GLB again in canonicalGeometryBounds() to report bounds. Under
  // the INPUT budgets that reparse can refuse an artifact normalization just legitimately published —
  // and it happens after the cache entry is written, so the command fails on a model it accepted.
  // Both pairs are independently configurable and their defaults already differ, so the two budgets
  // cannot be left to callers keeping them in step.
  const input = makeGlbFixture({ primitiveCopies: 1000 });
  // Input 51,508 bytes / 51,432-byte JSON chunk; canonical 630,648 bytes / 534,620-byte JSON chunk.
  // Each budget below admits the input and would refuse the canonical output.
  for (const tight of [{ maxGlbJsonBytes: 100_000 }, { maxInputGlbBytes: 100_000 }]) {
    const limits = lowerableLimits(tight);
    const canonical = normalizeRevitGlb(input, { limits }).glb;
    assert.throws(() => parseGlb(canonical, { limits }), /exceeds its byte limit/,
      'the input budget must still bind an input — this is the condition being guarded against');
    assert.equal(parseGlb(canonical, { limits: canonicalArtifactLimits(limits) }).json.scene, 0);
  }
});

test('the canonical JSON budget is applied to the padded chunk that is actually written', () => {
  // The chunk is 4-byte aligned and its DECLARED length is the padded one, which is what parseGlb
  // reads back. Budgeting the unpadded length admits a document whose emitted chunk is up to three
  // bytes over its own ceiling, and the canonical re-read then refuses it after publication.
  const input = makeGlbFixture({ nodeName: 'a' });
  const emitted = normalizeRevitGlb(input).glb;
  const padded = emitted.readUInt32LE(12);
  let end = 20 + padded;
  while (emitted[end - 1] === 0x20) end -= 1;
  const unpadded = end - 20;
  // The fixture is chosen so the two differ — without that this test proves nothing.
  assert.equal(unpadded, 691);
  assert.equal(padded, 692);
  assert.notEqual(unpadded, padded);
  // At exactly the unpadded length the document must be refused, because it cannot be written.
  assert.throws(
    () => normalizeRevitGlb(input, { limits: { maxCanonicalGlbJsonBytes: unpadded } }),
    (error) => error.code === 'reference-output-too-large' && /canonical GLB JSON/.test(error.message),
  );
  // At the padded length it is admitted, and the artifact it publishes re-reads under that same
  // budget — the round trip the previous behaviour broke.
  const limits = lowerableLimits({ maxCanonicalGlbJsonBytes: padded });
  const out = normalizeRevitGlb(input, { limits }).glb;
  assert.equal(out.readUInt32LE(12), padded);
  assert.equal(parseGlb(out, { limits: canonicalArtifactLimits(limits) }).json.scene, 0);
});

test('the canonical artifact budgets map to the canonical ceilings and change nothing else', () => {
  const limits = lowerableLimits({});
  const mapped = canonicalArtifactLimits(limits);
  assert.equal(mapped.maxGlbJsonBytes, limits.maxCanonicalGlbJsonBytes);
  assert.equal(mapped.maxInputGlbBytes, limits.maxCanonicalGlbBytes);
  // Every other bound is carried through untouched, so this is a re-aim and not a second profile.
  for (const name of Object.keys(MODEL_LIMITS)) {
    if (name === 'maxGlbJsonBytes' || name === 'maxInputGlbBytes') continue;
    assert.equal(mapped[name], limits[name], `${name} must not be altered`);
  }
  // The mapping can never produce a limits object lowerableLimits would reject, whatever the caller
  // configured: each canonical ceiling is at or below the hard ceiling of the bound it replaces.
  assert.ok(MODEL_LIMITS.maxCanonicalGlbJsonBytes.hard <= MODEL_LIMITS.maxGlbJsonBytes.hard);
  assert.ok(MODEL_LIMITS.maxCanonicalGlbBytes.hard <= MODEL_LIMITS.maxInputGlbBytes.hard);
  assert.deepEqual(lowerableLimits(mapped), mapped);
});

test('active nodes are closed objects and refuse extensions with stable geometry errors', () => {
  assert.throws(
    () => normalizeRevitGlb(makeGlbFixture({ nodes: [null] })),
    (error) => error.code === 'reference-geometry-invalid' && /node must be an object/.test(error.message),
  );
  assert.throws(
    () => normalizeRevitGlb(makeGlbFixture({
      nodes: [{ name: 'extended', mesh: 0, extensions: { EXT_mesh_gpu_instancing: {} } }],
    })),
    (error) => error.code === 'reference-geometry-unsupported' && /unsupported property/.test(error.message),
  );
});

test('skins and animations are rejected instead of publishing untransformed bind-pose geometry', () => {
  assert.throws(() => normalizeRevitGlb(makeGlbFixture({ skin: 0, skins: [{ joints: [0] }] })), /skin/);
  assert.throws(() => normalizeRevitGlb(makeGlbFixture({ animations: [{ channels: [], samplers: [] }] })), /animation/);
});

// #519: Math.fround(1e20) is a finite float32 but an unsafe integer, so the old finiteness-only gate let
// it through into the accessor min/max and the entity bounds, and canonicalJsonBytes then threw a bare
// TypeError once the artifacts were already being serialized (masked as reference-internal-error).
test('a coordinate that is finite in float32 but an unsafe integer is refused as invalid geometry', () => {
  assert.throws(
    () => normalizeRevitGlb(makeGlbFixture({ positions: [[0, 0, 0], [1e20, 0, 0], [0, 1, 0]] })),
    (error) => error.name === 'ModelReaderError'
      && error.code === 'reference-geometry-invalid'
      && error.phase === 'normalize-geometry'
      && /transformed coordinate must be a safe integer/.test(error.message),
  );
});

test('a large but safe-integral coordinate still normalizes, so the gate is not a magnitude gate', () => {
  // 2^23 is the largest integer float32 holds exactly; scaled to mm it is 8_388_608_000 — nine digits, far
  // from the origin, and still a safe integer, so canonical JSON carries it. A guard keyed on magnitude
  // rather than on safe-integrality would refuse legitimate site coordinates at that distance.
  const result = normalizeRevitGlb(makeGlbFixture({ positions: [[0, 0, 0], [8388608, 0, 0], [0, 1, 0]] }));
  assert.deepEqual(result.parts[0].positions, [[0, 0, 0], [0, 0, 1000], [8388608000, 0, 0]]);
});
