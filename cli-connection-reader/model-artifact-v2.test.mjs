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
    assert.throws(
      () => buildArtifactV2Root(value.options),
      (error) => error.code === 'reference-artifact-v2-invalid',
    );
  }
});

function degrade(value) {
  const source = value.options.effectiveSource;
  source.manifest.absent = [{
    role: 'catalogue.profile', classification: 'degraded', affectedDomains: ['geometry.profile'],
  }];
  source.manifest.completeness = 'degraded';
  source.bytes = canonicalJsonBytes(source.manifest);
  value.options.effectiveSourceSha256 = sha256(source.bytes);
  source.receipt = artifactV2Receipt({
    logicalPath: 'effective-source.json', logicalKind: 'effective-source', ordinal: 0,
    mediaType: 'application/json', content: source.bytes, itemCount: 1,
  });
  return value;
}

test('an empty geometry index is refused unless the caller admits it, and only for geometry (#604)', () => {
  assert.throws(
    () => buildArtifactV2Index('geometry', []),
    (error) => error.code === 'reference-artifact-v2-invalid',
  );
  assert.throws(
    () => buildArtifactV2Index('geometry', [], { allowEmptyGeometry: 'yes' }),
    (error) => error.code === 'reference-artifact-v2-invalid',
  );
  assert.throws(
    () => buildArtifactV2Index('entities', [], { allowEmptyGeometry: true }),
    (error) => error.code === 'reference-artifact-v2-invalid',
  );
  const empty = buildArtifactV2Index('geometry', [], { allowEmptyGeometry: true });
  assert.equal(empty.bytes.toString('utf8'),
    '{"family":"geometry","itemCount":0,"objects":[],"schemaVersion":"aware.model-artifact-index/v2"}');
  assert.equal(empty.receipt.itemCount, 0);
  assert.equal(empty.receipt.bounds, undefined);
});

test('a degraded root may carry zero geometry tiles; a complete root may not (#604)', () => {
  const complete = artifact();
  complete.options.indexes.geometry = buildArtifactV2Index('geometry', [], { allowEmptyGeometry: true });
  assert.throws(
    () => buildArtifactV2Root(complete.options),
    (error) => error.code === 'reference-artifact-v2-invalid'
      && error.message === 'Only a degraded artifact, or one that declares why nothing is drawn, may omit geometry.',
  );

  const degraded = degrade(artifact());
  degraded.options.indexes.geometry = buildArtifactV2Index('geometry', [], { allowEmptyGeometry: true });
  const result = buildArtifactV2Root(degraded.options);
  assert.equal(result.manifest.completeness, 'degraded');
  assert.equal(result.manifest.indexes.geometry.itemCount, 0);
  assert.equal(result.manifest.objects.some((entry) => entry.logicalKind === 'geometry-tile'), false);
  assert.deepEqual(result.manifest.objects.map((entry) => entry.logicalPath), [
    'effective-source.json', 'entities.index.json', 'geometry.index.json',
    'metadata/entities-000000.json', 'properties.index.json', 'relationships.index.json',
  ]);

  // A degraded root with geometry is unchanged by the exception.
  const drawn = buildArtifactV2Root(degrade(artifact()).options);
  assert.equal(drawn.manifest.indexes.geometry.itemCount, 7);
});

// A properties-only model (#869): a complete root with an empty geometry index is admitted only
// beside a geometry-state label that accounts for every entity. The fixture has two entities.
function propertiesOnlyRoot(geometryState, entityCount = 2) {
  const value = artifact();
  value.options.indexes.geometry = buildArtifactV2Index('geometry', [], { allowEmptyGeometry: true });
  if (entityCount !== 2) {
    value.options.indexes.entities = buildArtifactV2Index('entities', [
      payload('entities', 0, '{"entities":[]}', entityCount, { idRange: { first: 'entity:1', last: 'entity:9' } }),
    ]);
  }
  value.options.geometryState = geometryState ?? {
    state: 'nothing-drawable-yet', notDrawn: 1, noGeometry: 1, reasons: [{ reason: 'shape.unsupported', count: 1 }],
  };
  return value;
}

test('a complete root without geometry is admitted only with a geometry-state label (#869)', () => {
  const result = buildArtifactV2Root(propertiesOnlyRoot().options);
  assert.equal(result.manifest.completeness, 'complete');
  assert.deepEqual(result.manifest.geometryState, {
    state: 'nothing-drawable-yet', notDrawn: 1, noGeometry: 1, reasons: [{ reason: 'shape.unsupported', count: 1 }],
  });
  assert.equal(result.manifest.indexes.geometry.itemCount, 0);
  assert.equal(result.bytes.includes(Buffer.from('"geometryState":{"noGeometry":1,"notDrawn":1,'
    + '"reasons":[{"count":1,"reason":"shape.unsupported"}],"state":"nothing-drawable-yet"}')), true);

  const unlabelled = propertiesOnlyRoot();
  delete unlabelled.options.geometryState;
  assert.throws(() => buildArtifactV2Root(unlabelled.options), (error) => error.code === 'reference-artifact-v2-invalid'
    && error.message === 'Only a degraded artifact, or one that declares why nothing is drawn, may omit geometry.');

  // A drawn root never gains the field, and may not carry the label.
  assert.equal(Object.hasOwn(buildArtifactV2Root(artifact().options).manifest, 'geometryState'), false);
  const drawn = artifact();
  drawn.options.geometryState = propertiesOnlyRoot().options.geometryState;
  assert.throws(() => buildArtifactV2Root(drawn.options), (error) => error.code === 'reference-artifact-v2-invalid'
    && error.message === 'A geometry state label is admitted only without geometry.');
});

test('the root refuses a geometry-state label that does not tally with its entities (#869)', () => {
  const label = (notDrawn, noGeometry, reasons, extra = {}) => ({
    state: 'nothing-drawable-yet', notDrawn, noGeometry, reasons, ...extra,
  });
  const reason = (name, count) => ({ reason: name, count });
  const cases = [
    ['fewer entities than the index', label(1, 0, [reason('a', 1)])],
    ['more entities than the index', label(2, 1, [reason('a', 2)])],
    ['nothing undrawn', label(0, 2, [])],
    ['no reasons', label(2, 0, [])],
    ['reasons that do not sum to notDrawn', label(2, 0, [reason('a', 1)])],
    ['reasons out of byte order', label(2, 0, [reason('b', 1), reason('a', 1)])],
    ['locale order instead of byte order', label(2, 0, [reason('a', 1), reason('B', 1)])],
    ['a repeated reason', label(2, 0, [reason('a', 1), reason('a', 1)])],
    ['a zero count', label(1, 1, [reason('a', 1), reason('b', 0)])],
    ['fractional counts', label(2, 0, [reason('a', 1.5), reason('b', 0.5)])],
    ['a fractional notDrawn', label(1.5, 0.5, [reason('a', 1), reason('b', 0.5)])],
    ['a negative noGeometry', label(3, -1, [reason('a', 3)])],
    ['a reason that is not an opaque code', label(1, 1, [reason('not drawn', 1)])],
    ['an unknown label', { ...label(1, 1, [reason('a', 1)]), state: 'nothing-drawn' }],
    ['an extra field', label(1, 1, [reason('a', 1)], { message: 'free text' })],
    ['an extra reason field', label(1, 1, [{ ...reason('a', 1), message: 'x' }])],
    ['a sparse reasons array', label(1, 1, new Array(1))],
    ['a missing field', { state: 'nothing-drawable-yet', notDrawn: 1, reasons: [reason('a', 1)] }],
  ];
  for (const [name, geometryState] of cases) {
    assert.throws(() => buildArtifactV2Root(propertiesOnlyRoot(geometryState).options),
      (error) => error.code === 'reference-artifact-v2-invalid', name);
  }
  // At most 64 distinct reasons.
  const many = (count) => Array.from({ length: count }, (_, index) => reason(`r${String(index).padStart(2, '0')}`, 1));
  assert.equal(buildArtifactV2Root(propertiesOnlyRoot(label(64, 0, many(64)), 64).options)
    .manifest.geometryState.reasons.length, 64);
  assert.throws(() => buildArtifactV2Root(propertiesOnlyRoot(label(65, 0, many(65)), 65).options),
    (error) => error.code === 'reference-artifact-v2-invalid');
});
