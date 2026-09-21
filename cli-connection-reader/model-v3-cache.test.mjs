import assert from 'node:assert/strict';
import { generateKeyPairSync } from 'node:crypto';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';

import { canonicalJsonBytes, sha256 } from './model-contract.mjs';
import { publishV3Cache, readV3Cache, v3CacheKey } from './model-v3-cache.mjs';

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
  const manifest = { schemaVersion: 'model-reference-manifest/v2', objects };
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
  const manifest = { schemaVersion: 'model-reference-manifest/v2', objects };
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
