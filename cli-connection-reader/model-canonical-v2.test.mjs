import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';

import { canonicalJsonBytes, sha256 } from './model-contract.mjs';
import { canonicalizeProviderOutput } from './model-canonical-v2.mjs';

function glb(points) {
  const binary = Buffer.alloc(points.length * 12);
  points.flat().forEach((value, position) => binary.writeFloatLE(value, position * 4));
  const json = Buffer.from(JSON.stringify({
    asset: { version: '2.0' }, buffers: [{ byteLength: binary.length }],
    bufferViews: [{ buffer: 0, byteLength: binary.length }],
    accessors: [{ bufferView: 0, componentType: 5126, count: points.length, type: 'VEC3' }],
    meshes: [{ primitives: [{ attributes: { POSITION: 0 } }] }], nodes: [{ mesh: 0 }],
  }));
  const jsonPadding = (4 - json.length % 4) % 4;
  const paddedJson = Buffer.concat([json, Buffer.alloc(jsonPadding, 0x20)]);
  const total = 12 + 8 + paddedJson.length + 8 + binary.length;
  const header = Buffer.alloc(12); header.writeUInt32LE(0x46546c67, 0);
  header.writeUInt32LE(2, 4); header.writeUInt32LE(total, 8);
  const jsonHeader = Buffer.alloc(8); jsonHeader.writeUInt32LE(paddedJson.length, 0);
  jsonHeader.writeUInt32LE(0x4e4f534a, 4);
  const binaryHeader = Buffer.alloc(8); binaryHeader.writeUInt32LE(binary.length, 0);
  binaryHeader.writeUInt32LE(0x004e4942, 4);
  return Buffer.concat([header, jsonHeader, paddedJson, binaryHeader, binary]);
}

async function fixture(t, overrides = {}) {
  const root = await fs.mkdtemp(path.join(await fs.realpath(os.tmpdir()), 'aware-canonical-v2-'));
  t.after(() => fs.rm(root, { recursive: true, force: true }));
  const outputRoot = path.join(root, 'output'); const workRoot = path.join(root, 'work');
  await fs.mkdir(path.join(outputRoot, 'geometry'), { recursive: true });
  await fs.mkdir(path.join(outputRoot, 'metadata'), { recursive: true });
  await fs.mkdir(workRoot);
  const geometry = overrides.geometry ?? [
    { bytes: glb([[0, 0, 0], [10, 10, 10]]), bounds: [0, 0, 0, 10, 10, 10] },
    { bytes: glb([[10, 0, 0], [20, 10, 10]]), bounds: [10, 0, 0, 20, 10, 10] },
  ];
  const entities = overrides.entities ?? [
    { id: 'entity:b', type: 'member', name: 'B', geometry: [{ tileOrdinal: 1, bounds: [10, 0, 0, 11, 1, 1] }] },
    { id: 'entity:a', type: 'member', name: 'A', geometry: [{ tileOrdinal: 0, bounds: [0, 0, 0, 1, 1, 1] }] },
  ];
  const properties = overrides.properties ?? [{
    id: 'property:1', entityId: 'entity:a', name: 'Mark', normalizedName: 'mark', value: 'A1',
    valueType: 'string', unit: null, source: 'provider', status: 'readable', provenance: 'model',
  }];
  const relationships = overrides.relationships ?? [{ id: 'relation:1', kind: 'contains', from: 'entity:a', to: 'entity:b' }];
  const files = [];
  for (const [ordinal, tile] of geometry.entries()) {
    const relative = `geometry/${String(ordinal).padStart(6, '0')}.glb`;
    await fs.writeFile(path.join(outputRoot, ...relative.split('/')), tile.bytes);
    files.push({ path: relative, kind: 'geometry', ordinal, mediaType: 'model/gltf-binary',
      bytes: tile.bytes.length, sha256: sha256(tile.bytes), count: 1, bounds: tile.bounds });
  }
  for (const [kind, records] of Object.entries({ entities, properties, relationships })) {
    if (!records.length && kind !== 'entities') continue;
    const bytes = Buffer.from(records.map((record) => canonicalJsonBytes(record).toString('utf8')).join('\n') + '\n');
    const relative = `metadata/${kind}-000000.jsonl`;
    await fs.writeFile(path.join(outputRoot, ...relative.split('/')), bytes);
    files.push({ path: relative, kind, ordinal: 0, mediaType: 'application/x-ndjson',
      bytes: bytes.length, sha256: sha256(bytes), count: records.length });
  }
  const providerPackageManifestSha256 = sha256(Buffer.from('package'));
  const effectiveSource = {
    schemaVersion: 'model-effective-source/v2', formatId: 'format.synthetic', protocolVersion: '3',
    capabilityId: 'capability.synthetic', providerFingerprintSha256: sha256(Buffer.from('provider')),
    providerPackageManifestSha256,
    discoveryPolicy: { policyId: 'policy.synthetic', sha256: sha256(Buffer.from('policy')) },
    completeness: 'complete', primary: { namespaceId: 'model', path: 'model.db', role: 'primary' },
    consumed: [], absent: [], unsupportedExternal: [], authentication: [], crossFileEvidence: [],
  };
  return { output: { root: outputRoot, files }, workRoot, effectiveSource, providerPackageManifestSha256 };
}

test('canonicalizes unsorted metadata and multiple geometry tiles into one v2 root', async (t) => {
  const value = await fixture(t);
  const result = await canonicalizeProviderOutput({
    ...value, formatId: 'format.synthetic', capabilityId: 'capability.synthetic',
    conversionRequestSha256: sha256(Buffer.from('request')),
  });
  assert.equal(result.root.manifest.schemaVersion, 'model-reference-manifest/v2');
  assert.equal(result.indexes.geometry.index.objects.length, 2);
  assert.deepEqual(result.indexes.entities.index.objects.map((entry) => entry.itemCount), [2]);
  const entityShard = JSON.parse(await fs.readFile(result.objects.find((entry) => entry.receipt?.logicalKind === 'entities-shard').pathname));
  assert.deepEqual(entityShard.records.map((entry) => entry.id), ['entity:a', 'entity:b']);
  assert.equal(result.root.sha256, sha256(result.root.bytes));
});

test('refuses dangling metadata and geometry ownership outside its tile', async (t) => {
  const dangling = await fixture(t, { properties: [{
    id: 'property:1', entityId: 'missing', name: 'Mark', normalizedName: 'mark', value: null,
    valueType: 'null', unit: null, source: 'provider', status: 'unreadable', provenance: 'model',
  }] });
  await assert.rejects(() => canonicalizeProviderOutput({
    ...dangling, formatId: 'format.synthetic', capabilityId: 'capability.synthetic',
    conversionRequestSha256: sha256(Buffer.from('request')),
  }), (error) => error.code === 'reference-metadata-semantic-dangling');

  const escaped = await fixture(t, { entities: [{
    id: 'entity:a', type: 'member', name: null,
    geometry: [{ tileOrdinal: 0, bounds: [-1, 0, 0, 1, 1, 1] }],
  }] });
  await assert.rejects(() => canonicalizeProviderOutput({
    ...escaped, formatId: 'format.synthetic', capabilityId: 'capability.synthetic',
    conversionRequestSha256: sha256(Buffer.from('request')),
  }), (error) => error.code === 'reference-metadata-semantic-invalid');

  const falseBounds = await fixture(t, { geometry: [{
    bytes: glb([[0, 0, 0], [1, 1, 1]]), bounds: [0, 0, 0, 2, 2, 2],
  }] });
  await assert.rejects(() => canonicalizeProviderOutput({
    ...falseBounds, formatId: 'format.synthetic', capabilityId: 'capability.synthetic',
    conversionRequestSha256: sha256(Buffer.from('request')),
  }), (error) => error.code === 'reference-geometry-invalid');

  const falseCount = await fixture(t);
  falseCount.output.files.find((entry) => entry.kind === 'geometry').count = 2;
  await assert.rejects(() => canonicalizeProviderOutput({
    ...falseCount, formatId: 'format.synthetic', capabilityId: 'capability.synthetic',
    conversionRequestSha256: sha256(Buffer.from('request')),
  }), (error) => error.code === 'reference-geometry-invalid');
});

test('refuses duplicate entity and relationship identities', async (t) => {
  const duplicate = await fixture(t, { entities: [
    { id: 'entity:a', type: 'member', name: null, geometry: [] },
    { id: 'entity:a', type: 'member', name: null, geometry: [] },
  ], relationships: [] });
  await assert.rejects(() => canonicalizeProviderOutput({
    ...duplicate, formatId: 'format.synthetic', capabilityId: 'capability.synthetic',
    conversionRequestSha256: sha256(Buffer.from('request')),
  }), (error) => error.code === 'reference-metadata-semantic-duplicate'
    || error.code === 'reference-artifact-v2-duplicate');

  const property = {
    id: 'property:1', entityId: 'entity:a', name: 'Mark', normalizedName: 'mark', value: 'A1',
    valueType: 'string', unit: null, source: 'provider', status: 'readable', provenance: 'model',
  };
  const duplicateProperty = await fixture(t, {
    entities: [{ id: 'entity:a', type: 'member', name: null, geometry: [] }],
    properties: [property, { ...property, name: 'Type', normalizedName: 'type' }], relationships: [],
  });
  await assert.rejects(() => canonicalizeProviderOutput({
    ...duplicateProperty, formatId: 'format.synthetic', capabilityId: 'capability.synthetic',
    conversionRequestSha256: sha256(Buffer.from('request')),
  }), (error) => error.code === 'reference-metadata-semantic-duplicate');
});
