import assert from 'node:assert/strict';
import test from 'node:test';

import { canonicalJsonBytes, sha256 } from './model-contract.mjs';
import {
  artifactV2Receipt, buildArtifactV2Index, buildArtifactV2Root,
} from './model-artifact-v2.mjs';

const DIGESTS = Object.freeze({
  provider: sha256(Buffer.from('provider')),
  conversion: sha256(Buffer.from('conversion')),
});

function payload(family, ordinal, content, itemCount, extra = {}) {
  const names = {
    geometry: [`geometry/${String(ordinal).padStart(6, '0')}.glb`, 'geometry-tile', 'model/gltf-binary'],
    entities: [`metadata/entities-${String(ordinal).padStart(6, '0')}.json`, 'entities-shard', 'application/json'],
    properties: [`metadata/properties-${String(ordinal).padStart(6, '0')}.json`, 'properties-shard', 'application/json'],
    relationships: [`metadata/relationships-${String(ordinal).padStart(6, '0')}.json`, 'relationships-shard', 'application/json'],
  };
  return artifactV2Receipt({
    logicalPath: names[family][0], logicalKind: names[family][1], mediaType: names[family][2],
    ordinal, content: Buffer.from(content), itemCount, ...extra,
  });
}

function artifact() {
  const sourceManifest = {
    schemaVersion: 'model-effective-source/v2', formatId: 'format.synthetic', protocolVersion: '3',
    capabilityId: 'capability.synthetic', providerFingerprintSha256: sha256(Buffer.from('provider')),
    providerPackageManifestSha256: DIGESTS.provider,
    discoveryPolicy: { policyId: 'policy.synthetic', sha256: sha256(Buffer.from('policy')) },
    completeness: 'complete', primary: { namespaceId: 'model', path: 'model.db1', role: 'primary' },
    consumed: [], absent: [], unsupportedExternal: [], authentication: [], crossFileEvidence: [],
  };
  const sourceBytes = canonicalJsonBytes(sourceManifest);
  const sourceReceipt = artifactV2Receipt({
    logicalPath: 'effective-source.json', logicalKind: 'effective-source', ordinal: 0,
    mediaType: 'application/json', content: sourceBytes, itemCount: 1,
  });
  const effectiveSource = { manifest: sourceManifest, bytes: sourceBytes, receipt: sourceReceipt };
  const indexes = {
    geometry: buildArtifactV2Index('geometry', [
      payload('geometry', 1, 'glb-one', 4, { bounds: [10, 0, 0, 20, 10, 10] }),
      payload('geometry', 0, 'glb-zero', 3, { bounds: [0, 0, 0, 10, 10, 10] }),
    ]),
    entities: buildArtifactV2Index('entities', [
      payload('entities', 0, '{"entities":[]}', 2, { idRange: { first: 'entity:1', last: 'entity:2' } }),
    ]),
    properties: buildArtifactV2Index('properties', []),
    relationships: buildArtifactV2Index('relationships', []),
  };
  return {
    options: {
      formatId: 'format.synthetic', capabilityId: 'capability.synthetic',
      providerPackageManifestSha256: DIGESTS.provider,
      effectiveSourceSha256: sourceReceipt.sha256,
      conversionRequestSha256: DIGESTS.conversion,
      effectiveSource, indexes,
    },
    sourceBytes,
  };
}

test('canonical index sorts contiguous payload receipts and has frozen golden bytes', () => {
  const value = artifact();
  const index = value.options.indexes.geometry;
  assert.deepEqual(index.index.objects.map((entry) => entry.ordinal), [0, 1]);
  assert.equal(index.index.itemCount, 7);
  assert.equal(index.bytes.toString('utf8'),
    '{"family":"geometry","itemCount":7,"objects":[{"bounds":[0,0,0,10,10,10],"bytes":8,"itemCount":3,"logicalKind":"geometry-tile","logicalPath":"geometry/000000.glb","mediaType":"model/gltf-binary","ordinal":0,"sha256":"925c96e13c501a58df6a881d065f6bf222c887f780d56ede7eb117051719b133"},{"bounds":[10,0,0,20,10,10],"bytes":7,"itemCount":4,"logicalKind":"geometry-tile","logicalPath":"geometry/000001.glb","mediaType":"model/gltf-binary","ordinal":1,"sha256":"63406825832b022589f6ae52ea0b3760ddb272ab4c80747e02cdf1d2d105dc2f"}],"schemaVersion":"aware.model-artifact-index/v2"}');
});

test('canonical root enumerates exactly the source, indexes and payload objects', () => {
  const value = artifact();
  const result = buildArtifactV2Root(value.options);
  assert.equal(result.sha256, sha256(result.bytes));
  assert.equal(result.manifest.schemaVersion, 'model-reference-manifest/v2');
  assert.deepEqual(result.manifest.discoveryPolicy, value.options.effectiveSource.manifest.discoveryPolicy);
  assert.deepEqual(result.manifest.absent, []);
  assert.deepEqual(result.manifest.objects.map((entry) => entry.logicalPath), [
    'effective-source.json', 'entities.index.json', 'geometry.index.json', 'geometry/000000.glb',
    'geometry/000001.glb', 'metadata/entities-000000.json', 'properties.index.json',
    'relationships.index.json',
  ]);
  assert.equal(result.bytes.includes(Buffer.from('runId')), false);
  assert.equal(result.bytes.includes(Buffer.from('artifactId')), false);
});

test('root derives degraded coverage from authenticated effective-source bytes', () => {
  const value = artifact();
  value.options.effectiveSource.manifest.absent = [{
    role: 'catalogue.profile', classification: 'degraded', affectedDomains: ['geometry.profile'],
  }];
  value.options.effectiveSource.manifest.completeness = 'degraded';
  value.options.effectiveSource.bytes = canonicalJsonBytes(value.options.effectiveSource.manifest);
  value.options.effectiveSourceSha256 = sha256(value.options.effectiveSource.bytes);
  value.options.effectiveSource.receipt = artifactV2Receipt({
    logicalPath: 'effective-source.json', logicalKind: 'effective-source', ordinal: 0,
    mediaType: 'application/json', content: value.options.effectiveSource.bytes, itemCount: 1,
  });
  const result = buildArtifactV2Root(value.options);
  assert.equal(result.manifest.completeness, 'degraded');
  assert.deepEqual(result.manifest.absent, value.options.effectiveSource.manifest.absent);

  value.options.effectiveSource.manifest.absent[0].affectedDomains = [];
  value.options.effectiveSource.bytes = canonicalJsonBytes(value.options.effectiveSource.manifest);
  value.options.effectiveSourceSha256 = sha256(value.options.effectiveSource.bytes);
  value.options.effectiveSource.receipt = artifactV2Receipt({
    logicalPath: 'effective-source.json', logicalKind: 'effective-source', ordinal: 0,
    mediaType: 'application/json', content: value.options.effectiveSource.bytes, itemCount: 1,
  });
  assert.throws(
    () => buildArtifactV2Root(value.options),
    (error) => error.code === 'reference-artifact-v2-invalid',
  );
});

test('root accepts the effective-source producer role ordering', () => {
  const value = artifact();
  value.options.effectiveSource.manifest.absent = [
    { role: 'a', classification: 'optional', affectedDomains: [] },
    { role: 'A', classification: 'optional', affectedDomains: [] },
  ].sort((left, right) => left.role.localeCompare(right.role, 'en'));
  value.options.effectiveSource.bytes = canonicalJsonBytes(value.options.effectiveSource.manifest);
  value.options.effectiveSourceSha256 = sha256(value.options.effectiveSource.bytes);
  value.options.effectiveSource.receipt = artifactV2Receipt({
    logicalPath: 'effective-source.json', logicalKind: 'effective-source', ordinal: 0,
    mediaType: 'application/json', content: value.options.effectiveSource.bytes, itemCount: 1,
  });
  assert.doesNotThrow(() => buildArtifactV2Root(value.options));
});

test('root binds the effective source to its format, capability and provider package', () => {
  for (const [field, value] of [
    ['formatId', 'format.other'],
    ['capabilityId', 'capability.other'],
    ['providerPackageManifestSha256', sha256(Buffer.from('other package'))],
  ]) {
    const artifactValue = artifact();
    artifactValue.options.effectiveSource.manifest[field] = value;
    artifactValue.options.effectiveSource.bytes = canonicalJsonBytes(artifactValue.options.effectiveSource.manifest);
    artifactValue.options.effectiveSourceSha256 = sha256(artifactValue.options.effectiveSource.bytes);
    artifactValue.options.effectiveSource.receipt = artifactV2Receipt({
      logicalPath: 'effective-source.json', logicalKind: 'effective-source', ordinal: 0,
      mediaType: 'application/json', content: artifactValue.options.effectiveSource.bytes, itemCount: 1,
    });
    assert.throws(
      () => buildArtifactV2Root(artifactValue.options),
      (error) => error.code === 'reference-artifact-v2-invalid',
    );
  }
});

test('root bytes are independent of caller receipt and object order', () => {
  const first = artifact();
  const second = artifact();
  second.options.indexes.geometry.index.objects.reverse();
  const rebuilt = buildArtifactV2Index('geometry', second.options.indexes.geometry.index.objects);
  second.options.indexes.geometry = rebuilt;
  assert.deepEqual(buildArtifactV2Root(first.options).bytes, buildArtifactV2Root(second.options).bytes);
});

test('indexes refuse gaps, wrong names and missing required families', () => {
  assert.throws(
    () => buildArtifactV2Index('entities', new Array(1)),
    (error) => error.code === 'reference-artifact-v2-invalid',
  );
  assert.throws(
    () => buildArtifactV2Index('geometry', [payload('geometry', 1, 'glb', 1)]),
    (error) => error.code === 'reference-artifact-v2-invalid',
  );
  const wrong = payload('entities', 0, '{}', 1);
  wrong.logicalPath = 'metadata/entities-000001.json';
  assert.throws(
    () => buildArtifactV2Index('entities', [wrong]),
    (error) => error.code === 'reference-artifact-v2-invalid',
  );
  assert.throws(
    () => buildArtifactV2Index('entities', []),
    (error) => error.code === 'reference-artifact-v2-invalid',
  );
});

test('root refuses tampered index bytes and a mismatched effective-source identity', () => {
  const tampered = artifact();
  tampered.options.indexes.entities.bytes = Buffer.from('{}');
  assert.throws(
    () => buildArtifactV2Root(tampered.options),
    (error) => error.code === 'reference-artifact-v2-invalid',
  );
  const missingReceipt = artifact();
  missingReceipt.options.indexes.entities.receipt = null;
  assert.throws(
    () => buildArtifactV2Root(missingReceipt.options),
    (error) => error.code === 'reference-artifact-v2-invalid',
  );
  const mismatched = artifact();
  mismatched.options.effectiveSourceSha256 = sha256(Buffer.from('other source'));
  assert.throws(
    () => buildArtifactV2Root(mismatched.options),
    (error) => error.code === 'reference-artifact-v2-invalid',
  );
});

test('root refuses a self-consistent index package whose bytes are not canonical', () => {
  const value = artifact();
  const geometry = value.options.indexes.geometry;
  geometry.index.objects.reverse();
  geometry.bytes = canonicalJsonBytes(geometry.index);
  geometry.receipt = artifactV2Receipt({
    logicalPath: 'geometry.index.json', logicalKind: 'geometry-index', ordinal: 0,
    mediaType: 'application/json', content: geometry.bytes, itemCount: geometry.index.itemCount,
  });
  assert.throws(
    () => buildArtifactV2Root(value.options),
    (error) => error.code === 'reference-artifact-v2-invalid' && /not canonical/.test(error.message),
  );
});

test('receipts reject unsafe numbers, inverted bounds and invalid ID ranges', () => {
  const sparseBounds = new Array(6);
  sparseBounds[0] = 0; sparseBounds[3] = 1;
  assert.throws(
    () => payload('geometry', 0, 'glb', 1, { bounds: sparseBounds }),
    (error) => error.code === 'reference-artifact-v2-invalid',
  );
  assert.throws(
    () => payload('geometry', 0, 'glb', 1, { bounds: [1, 0, 0, 0, 1, 1] }),
    (error) => error.code === 'reference-artifact-v2-invalid',
  );
  assert.throws(
    () => payload('entities', 0, '{}', Number.MAX_SAFE_INTEGER + 1),
    (error) => error.code === 'reference-artifact-v2-invalid',
  );
  assert.throws(
    () => payload('entities', 0, '{}', 1, { idRange: { first: 'z', last: 'a' } }),
    (error) => error.code === 'reference-artifact-v2-invalid',
  );
  assert.throws(
    () => artifactV2Receipt({
      logicalPath: 'metadata/entities-000000.json', logicalKind: '\ud800', ordinal: 0,
      mediaType: 'application/json', content: Buffer.from('{}'), itemCount: 1,
    }),
    (error) => error.code === 'reference-artifact-v2-invalid',
  );
  assert.throws(
    () => payload('entities', 0, '{}', 1, { idRange: { first: '\ud800', last: '\ud800' } }),
    (error) => error.code === 'reference-artifact-v2-invalid',
  );
});

test('root rejects sparse effective-source coverage arrays', () => {
  for (const absent of [
    new Array(1),
    [{ role: 'catalogue', classification: 'degraded', affectedDomains: new Array(1) }],
  ]) {
    const value = artifact();
    value.options.effectiveSource.manifest.absent = absent;
    value.options.effectiveSource.manifest.completeness = 'degraded';
    value.options.effectiveSource.bytes = canonicalJsonBytes(value.options.effectiveSource.manifest);
    value.options.effectiveSourceSha256 = sha256(value.options.effectiveSource.bytes);
    value.options.effectiveSource.receipt = artifactV2Receipt({
      logicalPath: 'effective-source.json', logicalKind: 'effective-source', ordinal: 0,
      mediaType: 'application/json', content: value.options.effectiveSource.bytes, itemCount: 1,
    });
    assert.throws(
      () => buildArtifactV2Root(value.options),
      (error) => error.code === 'reference-artifact-v2-invalid',
    );
  }
});
