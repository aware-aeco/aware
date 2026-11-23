import assert from 'node:assert/strict';
import { generateKeyPairSync } from 'node:crypto';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';

import { canonicalJsonBytes, sha256 } from './model-contract.mjs';
import { publishV3Cache, readV3Cache, v3CacheKey } from './model-v3-cache.mjs';
import { canonicalizeProviderOutput } from './model-canonical-v2.mjs';
import { makeGlbFixture } from './model-fixtures.mjs';
import { validateGlbTile } from './model-glb-tile.mjs';

test('protocol-v3 cache revalidates every object before a warm reuse', async (t) => {
  const root = await fs.mkdtemp(path.join(await fs.realpath(os.tmpdir()), 'aware-v3-cache-'));
  t.after(() => fs.rm(root, { recursive: true, force: true }));
  const payload = Buffer.from('payload'); const source = path.join(root, 'source.bin');
  await fs.writeFile(source, payload);
  const payloadReceipt = { logicalPath: 'effective-source.json', logicalKind: 'effective-source', ordinal: 0,
    mediaType: 'application/json', bytes: payload.length, sha256: sha256(payload), itemCount: 1 };
  const indexes = {};
  for (const family of ['geometry', 'entities', 'properties', 'relationships']) {
    const index = { schemaVersion: 'aware.model-artifact-index/v2', family, itemCount: 0, objects: [] };
    const bytes = canonicalJsonBytes(index);
    const receipt = { logicalPath: `${family}.index.json`, logicalKind: `${family}-index`, ordinal: 0,
      mediaType: 'application/json', bytes: bytes.length, sha256: sha256(bytes), itemCount: 0 };
    indexes[family] = { index, bytes, receipt };
  }
  const objects = [payloadReceipt, ...Object.values(indexes).map((entry) => entry.receipt)];
  const manifest = { schemaVersion: 'model-reference-manifest/v2', completeness: 'degraded', objects };
  const rootBytes = canonicalJsonBytes(manifest); const identity = { source: 'a'.repeat(64) };
  const key = v3CacheKey(identity);
  const generated = generateKeyPairSync('ed25519');
  const signingKey = {
    privateKey: generated.privateKey,
    publicKeyBytes: generated.publicKey.export({ format: 'der', type: 'spki' }).subarray(-32),
  };
  const cached = await publishV3Cache(root, key, identity, {
    root: { manifest, bytes: rootBytes, sha256: sha256(rootBytes) }, indexes,
    objects: [{ pathname: source, receipt: payloadReceipt }],
  }, signingKey);
  assert.equal(cached.root.sha256, sha256(rootBytes));
  const cachedBytes = await fs.readFile(cached.objects[0].pathname);
  await fs.rm(cached.objects[0].pathname);
  await assert.rejects(() => readV3Cache(root, key, identity, signingKey.publicKeyBytes),
    (error) => error.code === 'reference-cache-invalid');
  await fs.writeFile(cached.objects[0].pathname, cachedBytes, { mode: 0o400 });
  await fs.chmod(cached.objects[0].pathname, 0o600);
  await fs.writeFile(cached.objects[0].pathname, 'changed');
  await assert.rejects(() => readV3Cache(root, key, identity, signingKey.publicKeyBytes),
    (error) => error.code === 'reference-cache-invalid');
});

test('protocol-v3 cache refuses a coherently rewritten record and root', async (t) => {
  const root = await fs.mkdtemp(path.join(await fs.realpath(os.tmpdir()), 'aware-v3-cache-signature-'));
  t.after(() => fs.rm(root, { recursive: true, force: true }));
  const generated = generateKeyPairSync('ed25519');
  const signingKey = {
    privateKey: generated.privateKey,
    publicKeyBytes: generated.publicKey.export({ format: 'der', type: 'spki' }).subarray(-32),
  };
  const identity = { source: 'a'.repeat(64) }; const key = v3CacheKey(identity);
  const source = path.join(root, 'source.bin'); await fs.writeFile(source, 'payload');
  const payload = await fs.readFile(source);
  const payloadReceipt = { logicalPath: 'effective-source.json', logicalKind: 'effective-source', ordinal: 0,
    mediaType: 'application/json', bytes: payload.length, sha256: sha256(payload), itemCount: 1 };
  const indexes = {};
  for (const family of ['geometry', 'entities', 'properties', 'relationships']) {
    const index = { schemaVersion: 'aware.model-artifact-index/v2', family, itemCount: 0, objects: [] };
    const bytes = canonicalJsonBytes(index);
    const receipt = { logicalPath: `${family}.index.json`, logicalKind: `${family}-index`, ordinal: 0,
      mediaType: 'application/json', bytes: bytes.length, sha256: sha256(bytes), itemCount: 0 };
    indexes[family] = { index, bytes, receipt };
  }
  const objects = [payloadReceipt, ...Object.values(indexes).map((entry) => entry.receipt)];
  const manifest = { schemaVersion: 'model-reference-manifest/v2', completeness: 'degraded', objects };
  const rootBytes = canonicalJsonBytes(manifest);
  await publishV3Cache(root, key, identity, {
    root: { manifest, bytes: rootBytes, sha256: sha256(rootBytes) }, indexes,
    objects: [{ pathname: source, receipt: payloadReceipt }],
  }, signingKey);
  const directory = path.join(root, 'v3', key);
  const recordPath = path.join(directory, 'cache.json');
  const record = JSON.parse(await fs.readFile(recordPath, 'utf8'));
  record.artifactRootSha256 = 'f'.repeat(64);
  await fs.chmod(recordPath, 0o600); await fs.writeFile(recordPath, canonicalJsonBytes(record));
  await assert.rejects(() => readV3Cache(root, key, identity, signingKey.publicKeyBytes),
    (error) => error.code === 'reference-cache-invalid');
});

test('protocol-v3 cache refuses a validly signed non-degraded root without geometry (#604)', async (t) => {
  const root = await fs.mkdtemp(path.join(await fs.realpath(os.tmpdir()), 'aware-v3-cache-empty-'));
  t.after(() => fs.rm(root, { recursive: true, force: true }));
  const generated = generateKeyPairSync('ed25519');
  const signingKey = {
    privateKey: generated.privateKey,
    publicKeyBytes: generated.publicKey.export({ format: 'der', type: 'spki' }).subarray(-32),
  };
  const source = path.join(root, 'source.bin'); await fs.writeFile(source, 'payload');
  const payload = await fs.readFile(source);
  const payloadReceipt = { logicalPath: 'effective-source.json', logicalKind: 'effective-source', ordinal: 0,
    mediaType: 'application/json', bytes: payload.length, sha256: sha256(payload), itemCount: 1 };
  const indexes = {};
  for (const family of ['geometry', 'entities', 'properties', 'relationships']) {
    const index = { schemaVersion: 'aware.model-artifact-index/v2', family, itemCount: 0, objects: [] };
    const bytes = canonicalJsonBytes(index);
    const receipt = { logicalPath: `${family}.index.json`, logicalKind: `${family}-index`, ordinal: 0,
      mediaType: 'application/json', bytes: bytes.length, sha256: sha256(bytes), itemCount: 0 };
    indexes[family] = { index, bytes, receipt };
  }
  const objects = [payloadReceipt, ...Object.values(indexes).map((entry) => entry.receipt)];
  for (const completeness of ['complete', 'degraded']) {
    const identity = { source: completeness.padEnd(64, 'a') }; const key = v3CacheKey(identity);
    const manifest = { schemaVersion: 'model-reference-manifest/v2', completeness, objects };
    const rootBytes = canonicalJsonBytes(manifest);
    const publish = () => publishV3Cache(root, key, identity, {
      root: { manifest, bytes: rootBytes, sha256: sha256(rootBytes) }, indexes,
      objects: [{ pathname: source, receipt: payloadReceipt }],
    }, signingKey);
    if (completeness === 'complete') {
      await assert.rejects(publish, (error) => error.code === 'reference-cache-invalid'
        && error.message === 'Protocol-v3 cache holds a non-degraded root without geometry.');
      await assert.rejects(() => readV3Cache(root, key, identity, signingKey.publicKeyBytes),
        (error) => error.code === 'reference-cache-invalid');
    } else {
      const cached = await publish();
      assert.equal(cached.root.manifest.completeness, 'degraded');
      assert.deepEqual(cached.indexes.geometry.index.objects, []);
    }
  }
});

// A properties-only model (#869), produced by the real canonicalizer so its root is genuine.
async function propertiesOnlyCanonical(t, { drawn = false } = {}) {
  const root = await fs.mkdtemp(path.join(await fs.realpath(os.tmpdir()), 'aware-v3-cache-properties-only-'));
  t.after(() => fs.rm(root, { recursive: true, force: true }));
  const outputRoot = path.join(root, 'output'); const workRoot = path.join(root, 'work');
  await fs.mkdir(path.join(outputRoot, 'geometry'), { recursive: true });
  await fs.mkdir(path.join(outputRoot, 'metadata'), { recursive: true });
  await fs.mkdir(workRoot);
  const files = [];
  const jsonl = async (relative, kind, records) => {
    const bytes = Buffer.from(records.map((record) => `${canonicalJsonBytes(record).toString('utf8')}\n`).join(''));
    await fs.writeFile(path.join(outputRoot, ...relative.split('/')), bytes);
    files.push({ path: relative, kind, ordinal: 0, mediaType: 'application/x-ndjson',
      bytes: bytes.length, sha256: sha256(bytes), count: records.length });
  };
  if (drawn) {
    const tile = makeGlbFixture();
    const facts = validateGlbTile(tile);
    await fs.writeFile(path.join(outputRoot, 'geometry', '000000.glb'), tile);
    files.push({ path: 'geometry/000000.glb', kind: 'geometry', ordinal: 0, mediaType: 'model/gltf-binary',
      bytes: tile.length, sha256: sha256(tile), count: facts.primitiveCount, bounds: facts.bounds });
  }
  await jsonl('metadata/entities-000000.jsonl', 'entities', [
    { id: 'entity:a', type: 'member', name: 'A', geometry: [] },
    { id: 'entity:b', type: 'assembly', name: 'B', geometry: [] },
  ]);
  if (!drawn) {
    await jsonl('metadata/geometry-state-000000.jsonl', 'geometry-state', [
      { entityId: 'entity:a', reason: 'shape.unsupported', state: 'not-drawn' },
      { entityId: 'entity:b', state: 'no-geometry' },
    ]);
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
  const canonical = await canonicalizeProviderOutput({
    output: { root: outputRoot, files }, workRoot, effectiveSource, providerPackageManifestSha256,
    formatId: 'format.synthetic', capabilityId: 'capability.synthetic',
    conversionRequestSha256: sha256(Buffer.from('request')),
  });
  return { canonical, cacheRoot: path.join(root, 'cache') };
}

// A coherent rewrite: the index bytes, their receipts and the root all agree, so the cache record
// signs cleanly and only the properties-only checks can refuse it.
function rewrite(canonical, { geometryIndex, geometryState, completeness } = {}) {
  const manifest = structuredClone(canonical.root.manifest);
  const indexes = { ...canonical.indexes };
  if (geometryIndex) {
    const bytes = canonicalJsonBytes(geometryIndex);
    const receipt = { ...canonical.indexes.geometry.receipt, bytes: bytes.length, sha256: sha256(bytes),
      itemCount: geometryIndex.itemCount };
    indexes.geometry = { index: geometryIndex, bytes, receipt };
    manifest.indexes.geometry = receipt;
    manifest.objects = manifest.objects.map((entry) => (entry.logicalPath === 'geometry.index.json' ? receipt : entry));
  }
  if (geometryState === null) delete manifest.geometryState;
  else if (geometryState) manifest.geometryState = geometryState;
  if (completeness) manifest.completeness = completeness;
  const bytes = canonicalJsonBytes(manifest);
  return { ...canonical, root: { manifest, bytes, sha256: sha256(bytes) }, indexes };
}

function testSigningKey() {
  const generated = generateKeyPairSync('ed25519');
  return {
    privateKey: generated.privateKey,
    publicKeyBytes: generated.publicKey.export({ format: 'der', type: 'spki' }).subarray(-32),
  };
}

test('protocol-v3 cache round-trips a properties-only root and rebuilds it on every warm read (#869)', async (t) => {
  const { canonical, cacheRoot } = await propertiesOnlyCanonical(t);
  assert.deepEqual(canonical.root.manifest.geometryState, {
    state: 'nothing-drawable-yet', notDrawn: 1, noGeometry: 1, reasons: [{ reason: 'shape.unsupported', count: 1 }],
  });
  const signingKey = testSigningKey(); const identity = { source: 'b'.repeat(64) }; const key = v3CacheKey(identity);
  const published = await publishV3Cache(cacheRoot, key, identity, canonical, signingKey);
  const warm = await readV3Cache(cacheRoot, key, identity, signingKey.publicKeyBytes);
  for (const cached of [published, warm]) {
    assert.equal(cached.root.sha256, canonical.root.sha256);
    assert.deepEqual(cached.root.manifest.geometryState, canonical.root.manifest.geometryState);
    assert.deepEqual(cached.indexes.geometry.index.objects, []);
  }
});

test('protocol-v3 cache refuses a coherently signed properties-only root that breaks its invariants (#869)', async (t) => {
  const { canonical, cacheRoot } = await propertiesOnlyCanonical(t);
  const { canonical: drawn } = await propertiesOnlyCanonical(t, { drawn: true });
  const state = canonical.root.manifest.geometryState;
  const emptyIndex = canonical.indexes.geometry.index;
  const cases = [
    ['a complete empty root with its label removed', rewrite(canonical, { geometryState: null }),
      'Protocol-v3 cache holds a non-degraded root without geometry.'],
    ['an empty geometry index that claims geometry',
      rewrite(canonical, { geometryIndex: { ...emptyIndex, itemCount: 1 } }),
      'Protocol-v3 cache holds an empty geometry index that claims geometry.'],
    ['a label that does not tally with the entities',
      rewrite(canonical, { geometryState: { ...state, notDrawn: 2, reasons: [{ reason: 'shape.unsupported', count: 2 }] } }),
      'Protocol-v3 cache holds an invalid properties-only root.'],
    ['a label root whose completeness is not its effective source', rewrite(canonical, { completeness: 'degraded' }),
      'Protocol-v3 cache holds an invalid properties-only root.'],
    ['a label beside drawn geometry', rewrite(drawn, { geometryState: { ...state, noGeometry: 1 } }),
      'Protocol-v3 cache holds an invalid properties-only root.'],
  ];
  for (const [index, [label, tampered, message]] of cases.entries()) {
    const signingKey = testSigningKey();
    const identity = { source: String(index).padStart(64, 'c') }; const key = v3CacheKey(identity);
    const refused = (error) => error.code === 'reference-cache-invalid' && error.message === message;
    await assert.rejects(() => publishV3Cache(cacheRoot, key, identity, tampered, signingKey), refused, label);
    await assert.rejects(() => readV3Cache(cacheRoot, key, identity, signingKey.publicKeyBytes), refused, label);
  }
});
