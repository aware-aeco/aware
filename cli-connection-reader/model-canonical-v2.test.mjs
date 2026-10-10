import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';

import { canonicalJsonBytes, safeErrorEnvelope, sha256 } from './model-contract.mjs';
import {
  canonicalizeProviderOutput, frame, partitionSortedForTesting, publishOne,
} from './model-canonical-v2.mjs';
import { externalSortMetadataRecords } from './model-metadata-sort.mjs';

function glb(points) {
  const binary = Buffer.alloc(points.length * 12);
  points.flat().forEach((value, position) => binary.writeFloatLE(value, position * 4));
  const json = Buffer.from(JSON.stringify({
    asset: { version: '2.0' }, buffers: [{ byteLength: binary.length }],
    scene: 0, scenes: [{ nodes: [0] }],
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

test('a mid-write ENOSPC leaves no published artifact and a retry succeeds', async (t) => {
  const directory = await fs.mkdtemp(path.join(await fs.realpath(os.tmpdir()), 'aware-publish-v2-'));
  t.after(() => fs.rm(directory, { recursive: true, force: true }));
  const bytes = Buffer.from('complete canonical artifact');
  const digest = sha256(bytes);
  const io = {
    ...fs,
    open: async (...args) => {
      const handle = await fs.open(...args);
      return {
        writeFile: async (content) => {
          await handle.writeFile(content.subarray(0, 8));
          throw Object.assign(new Error('disk full'), { code: 'ENOSPC' });
        },
        sync: () => handle.sync(), close: () => handle.close(),
      };
    },
  };
  await assert.rejects(() => publishOne(directory, 'properties.json', bytes, digest, io), (error) => {
    const envelope = safeErrorEnvelope(error);
    return envelope.code === 'reference-artifact-write-failed'
      && envelope.message.includes('ENOSPC') && error.unsafeDetails.code === 'ENOSPC';
  });
  assert.deepEqual(await fs.readdir(directory), []);
  const descriptor = await publishOne(directory, 'properties.json', bytes, digest);
  assert.deepEqual(await fs.readFile(path.join(directory, descriptor.id)), bytes);
  assert.deepEqual(await fs.readdir(directory), [descriptor.id]);
});

test('complete existing canonical artifacts are reused, while incomplete and changed ones are distinguished', async (t) => {
  const directory = await fs.mkdtemp(path.join(await fs.realpath(os.tmpdir()), 'aware-publish-v2-'));
  t.after(() => fs.rm(directory, { recursive: true, force: true }));
  const bytes = Buffer.from('canonical artifact');
  const digest = sha256(bytes);
  const descriptor = await publishOne(directory, 'properties.json', bytes, digest);
  const noSpace = { ...fs, open: async () => { throw Object.assign(new Error('disk full'), { code: 'ENOSPC' }); } };
  assert.deepEqual(await publishOne(directory, 'properties.json', bytes, digest, noSpace), descriptor);

  const target = path.join(directory, descriptor.id);
  await fs.writeFile(target, bytes.subarray(0, 8));
  await assert.rejects(() => publishOne(directory, 'properties.json', bytes, digest),
    (error) => error.code === 'reference-artifact-incomplete');
  assert.deepEqual(await fs.readFile(target), bytes.subarray(0, 8));

  const changed = Buffer.alloc(bytes.length, 0x78);
  await fs.writeFile(target, changed);
  await assert.rejects(() => publishOne(directory, 'properties.json', bytes, digest),
    (error) => error.code === 'reference-artifact-collision');
  assert.deepEqual(await fs.readFile(target), changed);
  assert.deepEqual(await fs.readdir(directory), [descriptor.id]);
});

test('cleanup failure after linking does not report a published artifact as failed', async (t) => {
  const directory = await fs.mkdtemp(path.join(await fs.realpath(os.tmpdir()), 'aware-publish-v2-'));
  t.after(() => fs.rm(directory, { recursive: true, force: true }));
  const bytes = Buffer.from('signed root manifest');
  const digest = sha256(bytes);
  const io = { ...fs, rm: async () => { throw Object.assign(new Error('locked temp'), { code: 'EPERM' }); } };
  const descriptor = await publishOne(directory, 'model-reference-manifest.json', bytes, digest, io);
  assert.deepEqual(await fs.readFile(path.join(directory, descriptor.id)), bytes);
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

test('keeps provider-explicit relationships and their providerRelationKind (#590)', async (t) => {
  const value = await fixture(t, { relationships: [
    { from: 'entity:a', id: 'relation:1', kind: 'provider-explicit', providerRelationKind: 'tekla-component-child', to: 'entity:b' },
    { from: 'entity:a', id: 'relation:2', kind: 'contains', to: 'entity:b' },
  ] });
  const result = await canonicalizeProviderOutput({
    ...value, formatId: 'format.synthetic', capabilityId: 'capability.synthetic',
    conversionRequestSha256: sha256(Buffer.from('request')),
  });
  const shard = JSON.parse(await fs.readFile(result.objects.find((entry) => entry.receipt?.logicalKind === 'relationships-shard').pathname));
  assert.deepEqual(shard.records, [
    { from: 'entity:a', id: 'relation:2', kind: 'contains', to: 'entity:b' },
    { from: 'entity:a', id: 'relation:1', kind: 'provider-explicit', providerRelationKind: 'tekla-component-child', to: 'entity:b' },
  ]);
});

test('relationships are a closed union keyed by kind (#590)', async (t) => {
  const edge = { id: 'relation:1', from: 'entity:a', to: 'entity:b' };
  for (const relationship of [
    { ...edge, kind: 'provider-explicit' },
    { ...edge, kind: 'provider-explicit', providerRelationKind: '' },
    { ...edge, kind: 'provider-explicit', providerRelationKind: 'x'.repeat(257) },
    { ...edge, kind: 'provider-explicit', providerRelationKind: 'a\u0001b' },
    { ...edge, kind: 'provider-explicit', providerRelationKind: 7 },
    { ...edge, kind: 'contains', providerRelationKind: 'tekla-component-child' },
    { ...edge, kind: 'references' },
    { ...edge, kind: 'hosts', extra: true },
  ]) {
    const value = await fixture(t, { relationships: [relationship] });
    await assert.rejects(() => canonicalizeProviderOutput({
      ...value, formatId: 'format.synthetic', capabilityId: 'capability.synthetic',
      conversionRequestSha256: sha256(Buffer.from('request')),
    }), (error) => error.code === 'reference-metadata-semantic-invalid', JSON.stringify(relationship));
  }
  const longest = await fixture(t, { relationships: [
    { ...edge, kind: 'provider-explicit', providerRelationKind: '中'.repeat(256) },
  ] });
  await canonicalizeProviderOutput({
    ...longest, formatId: 'format.synthetic', capabilityId: 'capability.synthetic',
    conversionRequestSha256: sha256(Buffer.from('request')),
  });
});

const NO_GEOMETRY_ENTITIES = [
  { id: 'entity:b', type: 'member', name: 'B', geometry: [] },
  { id: 'entity:a', type: 'member', name: 'A', geometry: [] },
];

function degraded(value) {
  value.effectiveSource = {
    ...value.effectiveSource, completeness: 'degraded',
    absent: [{ role: 'catalogue.profile', classification: 'degraded', affectedDomains: ['geometry.profile'] }],
  };
  return value;
}

function canonicalize(value) {
  return canonicalizeProviderOutput({
    ...value, formatId: 'format.synthetic', capabilityId: 'capability.synthetic',
    conversionRequestSha256: sha256(Buffer.from('request')),
  });
}

test('a degraded conversion with nothing drawable publishes a zero-tile geometry family (#604)', async (t) => {
  const value = degraded(await fixture(t, { geometry: [], entities: NO_GEOMETRY_ENTITIES }));
  const result = await canonicalize(value);
  assert.equal(result.root.manifest.completeness, 'degraded');
  assert.deepEqual(result.indexes.geometry.index.objects, []);
  assert.equal(result.indexes.geometry.index.itemCount, 0);
  assert.equal(result.root.manifest.indexes.geometry.itemCount, 0);
  assert.equal(result.root.manifest.indexes.geometry.bounds, undefined);
  assert.equal(result.objects.some((entry) => entry.receipt.logicalKind === 'geometry-tile'), false);
  assert.equal(result.indexes.entities.index.itemCount, 2);
});

test('a complete conversion with zero geometry tiles is still refused (#604)', async (t) => {
  const value = await fixture(t, { geometry: [], entities: NO_GEOMETRY_ENTITIES });
  await assert.rejects(() => canonicalize(value), (error) => error.code === 'reference-geometry-invalid'
    && error.message === 'A complete model conversion requires at least one geometry tile.');
});

test('with zero tiles no entity may claim geometry, even when degraded (#604)', async (t) => {
  const value = degraded(await fixture(t, {
    geometry: [],
    entities: [{ id: 'entity:a', type: 'member', name: 'A', geometry: [{ tileOrdinal: 0, bounds: [0, 0, 0, 1, 1, 1] }] }],
    properties: [], relationships: [],
  }));
  await assert.rejects(() => canonicalize(value), (error) => error.code === 'reference-metadata-semantic-invalid');
});

test('a degraded conversion still may not ship an empty GLB tile (#604)', async (t) => {
  let json = Buffer.from(JSON.stringify({ asset: { version: '2.0' }, scene: 0, scenes: [{}] }));
  json = Buffer.concat([json, Buffer.alloc((4 - json.length % 4) % 4, 0x20)]);
  const header = Buffer.alloc(20); header.writeUInt32LE(0x46546c67, 0); header.writeUInt32LE(2, 4);
  header.writeUInt32LE(20 + json.length, 8); header.writeUInt32LE(json.length, 12);
  header.writeUInt32LE(0x4e4f534a, 16);
  const empty = Buffer.concat([header, json]);
  const value = degraded(await fixture(t, {
    geometry: [{ bytes: empty, bounds: [0, 0, 0, 0, 0, 0] }], entities: NO_GEOMETRY_ENTITIES,
  }));
  value.output.files.find((entry) => entry.kind === 'geometry').count = 0;
  await assert.rejects(() => canonicalize(value), (error) => error.code === 'reference-geometry-invalid');
});

test('a verified record naming an entity with U+2028 is not split into two lines (#679)', async (t) => {
  // The provider-output verifier frames JSONL on LF only; readline also broke lines at U+2028/U+2029,
  // so a legal record was refused here after being admitted there.
  const name = 'Beam\u2028A\u2029B';
  const value = await fixture(t, {
    entities: [
      { id: 'entity:a', type: 'member', name, geometry: [{ tileOrdinal: 0, bounds: [0, 0, 0, 1, 1, 1] }] },
      { id: 'entity:b', type: 'member', name: 'B', geometry: [{ tileOrdinal: 1, bounds: [10, 0, 0, 11, 1, 1] }] },
    ],
  });
  const result = await canonicalize(value);
  const shard = result.objects.find((entry) => entry.receipt?.logicalKind === 'entities-shard');
  const records = JSON.parse(await fs.readFile(shard.pathname, 'utf8')).records;
  assert.deepEqual(records.map((entry) => entry.name), [name, 'B']);
});

test('shard frames equal canonicalJsonBytes of the same shard object (#679)', () => {
  const sets = [
    [],
    [{ id: 'a' }],
    [{ b: 1, a: [1, 2.5, null, true, 'x"y\\z'] }, { id: '\u00e9\u4e2d\ud83d\ude00', '10': 1, '2': 2, z: { '1': 0 } }],
    Array.from({ length: 50 }, (_, index) => ({ id: `p:${index}`, value: index % 3 === 0 ? null : index / 7 })),
  ];
  for (const records of sets) {
    for (const family of ['entities', 'properties', 'relationships']) {
      const expected = canonicalJsonBytes({ family, records, schemaVersion: 'aware.model-metadata-shard/v2' });
      const actual = frame(family, records.map((record) => canonicalJsonBytes(record).toString('utf8')));
      assert.deepEqual(actual, expected);
    }
  }
});

test('partitioning reads the sorted run back byte-for-byte and refuses one that changed (#679)', async (t) => {
  const root = await fs.mkdtemp(path.join(await fs.realpath(os.tmpdir()), 'aware-partition-v2-'));
  t.after(() => fs.rm(root, { recursive: true, force: true }));
  const records = Array.from({ length: 40 }, (_, index) => ({
    key: `entity:${String(index).padStart(3, '0')}`,
    record: { id: `entity:${String(index).padStart(3, '0')}`, name: `n\u00e9${index}\u2028`, tags: [index, null, 'a"b'] },
  }));
  const sorted = await externalSortMetadataRecords([...records].reverse(), { tempParent: root });
  assert.equal(sorted.sha256, sha256(await fs.readFile(sorted.pathname)));
  const limits = { shardBytes: 1024, shardRecords: 7, recordBytes: 1024 * 1024 };
  const canonicalRoot = path.join(root, 'canonical');
  await fs.mkdir(canonicalRoot);
  const result = await partitionSortedForTesting('entities', sorted, canonicalRoot, limits);
  assert.ok(result.shards.length > 1);
  const shardRecords = [];
  for (const shard of result.shards) {
    const bytes = await fs.readFile(shard.pathname);
    const parsed = JSON.parse(bytes.toString('utf8'));
    assert.deepEqual(bytes, canonicalJsonBytes(parsed));
    assert.ok(parsed.records.length <= 7 && bytes.length <= 1024);
    shardRecords.push(...parsed.records);
  }
  assert.deepEqual(shardRecords, records.map((entry) => entry.record));
  assert.deepEqual(result.shards.map((entry) => entry.receipt.idRange.first),
    result.shards.map((entry, index) => shardRecords[index === 0 ? 0 : result.shards.slice(0, index)
      .reduce((total, shard) => total + shard.receipt.itemCount, 0)].id));

  // A byte changed after the sort wrote the run is caught by the digest, not parsed past.
  const original = await fs.readFile(sorted.pathname);
  const tampered = Buffer.from(original);
  tampered[tampered.indexOf(0x6e, 40)] = 0x4e;
  await fs.writeFile(sorted.pathname, tampered);
  await assert.rejects(
    () => partitionSortedForTesting('entities', sorted, path.join(root, 'canonical-2'), limits),
    (error) => error.code === 'reference-artifact-v2-invalid',
  );
  await fs.writeFile(sorted.pathname, original.subarray(0, original.length - 1));
  await assert.rejects(
    () => partitionSortedForTesting('entities', sorted, path.join(root, 'canonical-3'), limits),
    (error) => error.code === 'reference-artifact-v2-invalid',
  );
});
