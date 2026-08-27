import assert from 'node:assert/strict';
import { generateKeyPairSync } from 'node:crypto';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';
import { canonicalJsonBytes, sha256 } from './model-contract.mjs';
import {
  acquireCacheOwner, cacheKeySha256, loadAwareSigningKey, publishCacheEntry as publishCacheEntryRaw,
  findCacheEntry as findCacheEntryRaw, readCacheEntry as readCacheEntryRaw, releaseCacheOwner, signerFingerprintSha256,
} from './model-cache.mjs';

function processFence() {
  let tail = Promise.resolve();
  return async (work) => {
    const previous = tail;
    let release;
    tail = new Promise((resolve) => { release = resolve; });
    await previous;
    try { return await work(); }
    finally { release(); }
  };
}

const withMaintenanceFence = processFence();
const publishCacheEntry = (options) => publishCacheEntryRaw({
  ...options,
  withMaintenanceFence: options.withMaintenanceFence ?? withMaintenanceFence,
});
const readCacheEntry = (options) => readCacheEntryRaw({
  ...options,
  withMaintenanceFence: options.withMaintenanceFence ?? withMaintenanceFence,
});
const findCacheEntry = (options) => findCacheEntryRaw({
  ...options,
  withMaintenanceFence: options.withMaintenanceFence ?? withMaintenanceFence,
});

async function temporaryDirectory(t) {
  const directory = await fs.mkdtemp(path.join(os.tmpdir(), 'aware-model-cache-'));
  t.after(() => fs.rm(directory, { recursive: true, force: true }));
  return directory;
}

async function signingFixture(root) {
  const { privateKey, publicKey } = generateKeyPairSync('ed25519');
  const privateDer = privateKey.export({ format: 'der', type: 'pkcs8' });
  const publicDer = publicKey.export({ format: 'der', type: 'spki' });
  const secretPath = path.join(root, 'model-reader.sec');
  const publicPath = path.join(root, 'model-reader.pub');
  await fs.writeFile(secretPath, `ed25519-secret-key-v1 ${privateDer.subarray(-32).toString('base64')}\n`);
  await fs.writeFile(publicPath, `ed25519-public-key-v1 ${publicDer.subarray(-32).toString('base64')}\n`);
  return { secretPath, publicPath, key: await loadAwareSigningKey(secretPath, publicPath) };
}

function identity(overrides = {}) {
  return {
    sourceSha256: '1'.repeat(64), canonicalRequest: { schemaVersion: '1', value: 'request' },
    providerFingerprint: { protocolVersion: '1', provider: 'fixture', engine: 'engine', engineVersion: '1', adapterBuildId: 'b', adapterExecutableSha256: '2'.repeat(64), readerSchemaVersion: 'model-reference-reader/v1' },
    signerFingerprintSha256: '3'.repeat(64), ...overrides,
  };
}

function artifacts() {
  return {
    'geometry.glb': Buffer.from([0x67, 0x6c, 0x54, 0x46]),
    'entities.json': canonicalJsonBytes({ schemaVersion: '1', entities: [] }),
    'properties.json': canonicalJsonBytes({ schemaVersion: '1', properties: [] }),
    'relationships.json': canonicalJsonBytes({ schemaVersion: '1', relationships: [] }),
  };
}

test('cache identity changes for source, request, every provider fingerprint leaf, and signer trust', () => {
  const base = identity();
  const first = cacheKeySha256(base);
  for (const [field, changed] of [
    ['sourceSha256', '4'.repeat(64)], ['signerFingerprintSha256', '5'.repeat(64)],
    ['canonicalRequest', { schemaVersion: '1', value: 'changed' }],
  ]) assert.notEqual(cacheKeySha256({ ...base, [field]: changed }), first);
  for (const field of Object.keys(base.providerFingerprint)) {
    const value = field === 'adapterExecutableSha256' ? '6'.repeat(64) : `${base.providerFingerprint[field]}-changed`;
    assert.notEqual(cacheKeySha256({ ...base, providerFingerprint: { ...base.providerFingerprint, [field]: value } }), first, field);
  }
});

test('AWARE-format Ed25519 keys sign complete entries and every hit verifies signatures and bytes', async (t) => {
  const root = await temporaryDirectory(t);
  const { key } = await signingFixture(root);
  const cacheRoot = path.join(root, 'cache');
  const cacheIdentity = identity({ signerFingerprintSha256: signerFingerprintSha256(key.publicKeyBytes) });
  const published = await publishCacheEntry({ root: cacheRoot, identity: cacheIdentity, artifacts: artifacts(), signingKey: key });
  const hit = await readCacheEntry({ root: cacheRoot, key: published.key, expectedIdentity: cacheIdentity, expectedPublicKey: key.publicKeyBytes });
  assert.deepEqual(Object.keys(hit.artifacts).sort(), ['entities.json', 'geometry.glb', 'manifest.json', 'properties.json', 'relationships.json']);
  assert.equal(hit.manifest.artifacts['geometry.glb'].sha256, sha256(artifacts()['geometry.glb']));
  const geometryBlob = path.join(cacheRoot, 'blobs', hit.manifest.artifacts['geometry.glb'].sha256);
  await fs.writeFile(geometryBlob, Buffer.from([1, 2, 3, 4]));
  await assert.rejects(() => readCacheEntry({ root: cacheRoot, key: published.key, expectedIdentity: cacheIdentity, expectedPublicKey: key.publicKeyBytes }), /tampered|digest|invalid/);
});

test('missing, extra, wrong signer, and identity-mismatched cache entries are never hits', async (t) => {
  const root = await temporaryDirectory(t);
  const firstKey = (await signingFixture(path.join(root, 'first').replace(/first$/, ''))).key;
  const otherRoot = path.join(root, 'other'); await fs.mkdir(otherRoot);
  const secondKey = (await signingFixture(otherRoot)).key;
  const cacheRoot = path.join(root, 'cache');
  const cacheIdentity = identity({ signerFingerprintSha256: signerFingerprintSha256(firstKey.publicKeyBytes) });
  const published = await publishCacheEntry({ root: cacheRoot, identity: cacheIdentity, artifacts: artifacts(), signingKey: firstKey });
  await assert.rejects(() => readCacheEntry({ root: cacheRoot, key: published.key, expectedIdentity: identity(), expectedPublicKey: firstKey.publicKeyBytes }), /identity/);
  await assert.rejects(() => readCacheEntry({ root: cacheRoot, key: published.key, expectedIdentity: cacheIdentity, expectedPublicKey: secondKey.publicKeyBytes }), /signer/);
  await fs.writeFile(path.join(cacheRoot, 'entries', published.key, 'extra'), 'no');
  await assert.rejects(() => readCacheEntry({ root: cacheRoot, key: published.key, expectedIdentity: cacheIdentity, expectedPublicKey: firstKey.publicKeyBytes }), /extra|closed/);
});

test('pinned cache lookup filters authenticated manifests before reading component blobs', async (t) => {
  const root = await temporaryDirectory(t);
  const { key } = await signingFixture(root);
  const cacheRoot = path.join(root, 'cache');
  const signer = signerFingerprintSha256(key.publicKeyBytes);
  const orderedIdentities = Array.from({ length: 16 }, (_, index) => identity({
    sourceSha256: index.toString(16).padStart(64, '0'),
    signerFingerprintSha256: signer,
  })).sort((left, right) => cacheKeySha256(left).localeCompare(cacheKeySha256(right)));
  const missedIdentity = orderedIdentities[0];
  const wantedIdentity = orderedIdentities.at(-1);
  const missedArtifacts = { ...artifacts(), 'geometry.glb': Buffer.from('missed-geometry') };
  const missed = await publishCacheEntry({ root: cacheRoot, identity: missedIdentity, artifacts: missedArtifacts, signingKey: key });
  const wanted = await publishCacheEntry({ root: cacheRoot, identity: wantedIdentity, artifacts: artifacts(), signingKey: key });
  const missedGeometry = missed.manifest.artifacts['geometry.glb'];
  await fs.truncate(path.join(cacheRoot, 'blobs', missedGeometry.sha256), missedGeometry.bytes + 1);

  const hit = await findCacheEntry({
    root: cacheRoot,
    expectedSourceSha256: wantedIdentity.sourceSha256,
    expectedCanonicalRequest: wantedIdentity.canonicalRequest,
    expectedProviderFingerprintSha256: sha256(canonicalJsonBytes(wantedIdentity.providerFingerprint)),
    expectedSignerFingerprintSha256: signer,
    expectedPublicKey: key.publicKeyBytes,
  });
  assert.equal(hit.key, wanted.key);
});

test('pinned cache lookup preserves cancellation instead of treating it as a miss', async (t) => {
  const root = await temporaryDirectory(t);
  const { key } = await signingFixture(root);
  const cacheRoot = path.join(root, 'cache');
  const signer = signerFingerprintSha256(key.publicKeyBytes);
  const cacheIdentity = identity({ signerFingerprintSha256: signer });
  await publishCacheEntry({ root: cacheRoot, identity: cacheIdentity, artifacts: artifacts(), signingKey: key });
  const controller = new AbortController();
  controller.abort();
  await assert.rejects(() => findCacheEntry({
    root: cacheRoot,
    expectedSourceSha256: cacheIdentity.sourceSha256,
    expectedCanonicalRequest: cacheIdentity.canonicalRequest,
    expectedProviderFingerprintSha256: sha256(canonicalJsonBytes(cacheIdentity.providerFingerprint)),
    expectedSignerFingerprintSha256: signer,
    expectedPublicKey: key.publicKeyBytes,
    signal: controller.signal,
  }), (error) => error.code === 'reference-cancelled');
});

test('cache reads require the same maintenance fence that protects eviction', async (t) => {
  const root = await temporaryDirectory(t);
  await assert.rejects(
    () => readCacheEntryRaw({ root: path.join(root, 'cache'), key: 'a'.repeat(64), expectedIdentity: identity(), expectedPublicKey: Buffer.alloc(32) }),
    (error) => error.code === 'reference-cache-fence-missing',
  );
});

test('owner leases serialize same-key publishers and stale/dead owners are fenced by token', async (t) => {
  const root = await temporaryDirectory(t);
  const first = await acquireCacheOwner({ root, key: 'a'.repeat(64), processIdentity: { pid: 101, start: 'one' } });
  await assert.rejects(() => acquireCacheOwner({ root, key: 'a'.repeat(64), processIdentity: { pid: 102, start: 'two' }, isOwnerAlive: async () => true }), /owned/);
  const lockPath = path.join(root, 'locks', 'a'.repeat(64), 'owner.json');
  const stale = JSON.parse(await fs.readFile(lockPath, 'utf8'));
  stale.heartbeatAt = '2000-01-01T00:00:00.000Z';
  await fs.writeFile(lockPath, JSON.stringify(stale));
  await assert.rejects(
    () => acquireCacheOwner({ root, key: 'a'.repeat(64), processIdentity: { pid: 102, start: 'two' }, isOwnerAlive: async () => null }),
    /unverifiable|owned/,
  );
  const second = await acquireCacheOwner({ root, key: 'a'.repeat(64), processIdentity: { pid: 102, start: 'two' }, isOwnerAlive: async () => false });
  await assert.rejects(() => releaseCacheOwner(first), /token/);
  await releaseCacheOwner(second);
});

test('publication refuses an oversized generated manifest before exposing a cache entry', async (t) => {
  const root = await temporaryDirectory(t);
  const { key } = await signingFixture(root);
  const cacheRoot = path.join(root, 'cache');
  const cacheIdentity = identity({ signerFingerprintSha256: signerFingerprintSha256(key.publicKeyBytes) });
  await assert.rejects(
    () => publishCacheEntry({
      root: cacheRoot,
      identity: cacheIdentity,
      artifacts: artifacts(),
      signingKey: key,
      details: { coverage: { padding: 'x'.repeat(512) } },
      limits: { maxComponentJsonBytes: 128 },
    }),
    (error) => error.code === 'reference-cache-artifacts-invalid',
  );
  assert.deepEqual(await fs.readdir(path.join(cacheRoot, 'entries')), []);
});

test('cache-owner release failures use a stable redacted cache error', async (t) => {
  const root = await temporaryDirectory(t);
  const lease = await acquireCacheOwner({ root, key: 'f'.repeat(64), processIdentity: { pid: 101, start: 'one' } });
  await assert.rejects(
    () => releaseCacheOwner(lease, {
      removeLockDirectory: async () => { throw new Error(`cannot remove ${lease.lockDirectory}`); },
    }),
    (error) => error.code === 'reference-cache-owner-release' && !error.message.includes(root),
  );
});

test('cache hits reject oversized control files and blobs before reading their contents', async (t) => {
  const root = await temporaryDirectory(t);
  const { key } = await signingFixture(root);
  const cacheRoot = path.join(root, 'cache');
  const cacheIdentity = identity({ signerFingerprintSha256: signerFingerprintSha256(key.publicKeyBytes) });
  const published = await publishCacheEntry({ root: cacheRoot, identity: cacheIdentity, artifacts: artifacts(), signingKey: key });
  const entry = path.join(cacheRoot, 'entries', published.key);
  const receiptPath = path.join(entry, 'receipt.json');
  const signaturePath = path.join(entry, 'receipt.sig');
  const receiptBytes = await fs.readFile(receiptPath);
  const signatureBytes = await fs.readFile(signaturePath);

  await fs.truncate(receiptPath, (64 * 1024) + 1);
  await assert.rejects(() => readCacheEntry({ root: cacheRoot, key: published.key, expectedIdentity: cacheIdentity, expectedPublicKey: key.publicKeyBytes }), /limit|invalid/);
  await fs.writeFile(receiptPath, receiptBytes);
  await fs.truncate(signaturePath, (4 * 1024) + 1);
  await assert.rejects(() => readCacheEntry({ root: cacheRoot, key: published.key, expectedIdentity: cacheIdentity, expectedPublicKey: key.publicKeyBytes }), /limit|invalid/);
  await fs.writeFile(signaturePath, signatureBytes);

  const receipt = JSON.parse(receiptBytes.toString('utf8'));
  const geometryBlob = path.join(cacheRoot, 'blobs', receipt.blobs['geometry.glb'].sha256);
  await fs.truncate(geometryBlob, 1024 * 1024);
  await assert.rejects(() => readCacheEntry({ root: cacheRoot, key: published.key, expectedIdentity: cacheIdentity, expectedPublicKey: key.publicKeyBytes }), /limit|tampered|invalid/);
});

test('authoritative kernel fences recover missing or malformed owner diagnostics', async (t) => {
  const root = await temporaryDirectory(t);
  for (const [key, owner] of [
    ['b'.repeat(64), null],
    ['c'.repeat(64), '{broken'],
    ['e'.repeat(64), 'null'],
  ]) {
    const lockDirectory = path.join(root, 'locks', key);
    await fs.mkdir(lockDirectory, { recursive: true });
    if (owner !== null) await fs.writeFile(path.join(lockDirectory, 'owner.json'), owner);
    await assert.rejects(
      () => acquireCacheOwner({ root, key, processIdentity: { pid: 201, start: 'replacement' } }),
      (error) => error.code === 'reference-cache-owned',
    );
    const replacement = await acquireCacheOwner({
      root, key, processIdentity: { pid: 201, start: 'replacement' }, fenced: true,
    });
    assert.equal(JSON.parse(await fs.readFile(replacement.ownerPath, 'utf8')).processStart, 'replacement');
    await releaseCacheOwner(replacement);
  }
  const key = 'd'.repeat(64);
  await fs.mkdir(path.join(root, 'locks', key), { recursive: true });
  await assert.rejects(
    () => acquireCacheOwner({
      root, key, fenced: true,
      removeLockDirectory: async () => { throw Object.assign(new Error('private path'), { code: 'EACCES' }); },
    }),
    (error) => error.code === 'reference-cache-owned' && !error.message.includes(root),
  );
});

test('different cache keys sharing content never observe a partially published final blob', async (t) => {
  const root = await temporaryDirectory(t);
  const { key } = await signingFixture(root);
  const cacheRoot = path.join(root, 'cache');
  const signer = signerFingerprintSha256(key.publicKeyBytes);
  const firstIdentity = identity({ sourceSha256: '1'.repeat(64), signerFingerprintSha256: signer });
  const secondIdentity = identity({ sourceSha256: '2'.repeat(64), signerFingerprintSha256: signer });
  const [first, second] = await Promise.all([
    publishCacheEntry({ root: cacheRoot, identity: firstIdentity, artifacts: artifacts(), signingKey: key }),
    publishCacheEntry({ root: cacheRoot, identity: secondIdentity, artifacts: artifacts(), signingKey: key }),
  ]);
  await Promise.all([
    readCacheEntry({ root: cacheRoot, key: first.key, expectedIdentity: firstIdentity, expectedPublicKey: key.publicKeyBytes }),
    readCacheEntry({ root: cacheRoot, key: second.key, expectedIdentity: secondIdentity, expectedPublicKey: key.publicKeyBytes }),
  ]);
});

test('different-key publication enters one cache-wide maintenance fence at a time', async (t) => {
  const root = await temporaryDirectory(t);
  const { key } = await signingFixture(root);
  const cacheRoot = path.join(root, 'cache');
  const signer = signerFingerprintSha256(key.publicKeyBytes);
  let active = 0;
  let maximum = 0;
  let tail = Promise.resolve();
  const withMaintenanceFence = async (publish) => {
    const previous = tail;
    let release;
    tail = new Promise((resolve) => { release = resolve; });
    await previous;
    active += 1;
    maximum = Math.max(maximum, active);
    try { await new Promise((resolve) => setTimeout(resolve, 20)); return await publish(); }
    finally { active -= 1; release(); }
  };
  await Promise.all([
    publishCacheEntry({
      root: cacheRoot,
      identity: identity({ sourceSha256: '3'.repeat(64), signerFingerprintSha256: signer }),
      artifacts: artifacts(), signingKey: key, withMaintenanceFence,
    }),
    publishCacheEntry({
      root: cacheRoot,
      identity: identity({ sourceSha256: '4'.repeat(64), signerFingerprintSha256: signer }),
      artifacts: artifacts(), signingKey: key, withMaintenanceFence,
    }),
  ]);
  assert.equal(maximum, 1);
});

test('concurrent publication converges on one complete winner with identical deterministic bytes', async (t) => {
  const root = await temporaryDirectory(t);
  const { key } = await signingFixture(root);
  const cacheRoot = path.join(root, 'cache');
  const cacheIdentity = identity({ signerFingerprintSha256: signerFingerprintSha256(key.publicKeyBytes) });
  const [a, b] = await Promise.all([
    publishCacheEntry({ root: cacheRoot, identity: cacheIdentity, artifacts: artifacts(), signingKey: key }),
    publishCacheEntry({ root: cacheRoot, identity: cacheIdentity, artifacts: artifacts(), signingKey: key }),
  ]);
  assert.equal(a.key, b.key);
  const hit = await readCacheEntry({ root: cacheRoot, key: a.key, expectedIdentity: cacheIdentity, expectedPublicKey: key.publicKeyBytes });
  assert.deepEqual(hit.artifacts['geometry.glb'], artifacts()['geometry.glb']);
});

test('cache maintenance evicts deterministically, sweeps stale staging, and refuses an impossible quota', async (t) => {
  const root = await temporaryDirectory(t);
  const { key } = await signingFixture(root);
  const cacheRoot = path.join(root, 'cache');
  const staleStage = path.join(cacheRoot, 'staging', 'abandoned');
  await fs.mkdir(staleStage, { recursive: true });
  await fs.utimes(staleStage, new Date(0), new Date(0));
  const firstIdentity = identity({ signerFingerprintSha256: signerFingerprintSha256(key.publicKeyBytes) });
  const first = await publishCacheEntry({
    root: cacheRoot, identity: firstIdentity, artifacts: artifacts(), signingKey: key,
    cacheLimits: { maxEntries: 1, maxBytes: 1024 * 1024, staleStagingMs: 1 },
  });
  const secondIdentity = identity({
    ...firstIdentity, sourceSha256: '9'.repeat(64),
  });
  const second = await publishCacheEntry({
    root: cacheRoot, identity: secondIdentity, artifacts: artifacts(), signingKey: key,
    cacheLimits: { maxEntries: 1, maxBytes: 1024 * 1024, staleStagingMs: 1 },
  });
  await assert.rejects(() => fs.access(path.join(cacheRoot, 'entries', first.key)));
  await fs.access(path.join(cacheRoot, 'entries', second.key));
  await assert.rejects(() => fs.access(staleStage));
  await assert.rejects(
    () => publishCacheEntry({
      root: path.join(root, 'tiny-cache'), identity: firstIdentity, artifacts: artifacts(), signingKey: key,
      cacheLimits: { maxEntries: 1, maxBytes: 1, staleStagingMs: 1 },
    }),
    (error) => error.code === 'reference-cache-full',
  );
});

test('successful reads refresh deterministic LRU recency before eviction', async (t) => {
  const root = await temporaryDirectory(t);
  const { key } = await signingFixture(root);
  const cacheRoot = path.join(root, 'cache');
  const signer = signerFingerprintSha256(key.publicKeyBytes);
  const firstIdentity = identity({ sourceSha256: '1'.repeat(64), signerFingerprintSha256: signer });
  const secondIdentity = identity({ sourceSha256: '2'.repeat(64), signerFingerprintSha256: signer });
  const thirdIdentity = identity({ sourceSha256: '3'.repeat(64), signerFingerprintSha256: signer });
  const limits = { maxEntries: 2, maxBytes: 1024 * 1024, staleStagingMs: 1 };
  const first = await publishCacheEntry({ root: cacheRoot, identity: firstIdentity, artifacts: artifacts(), signingKey: key, cacheLimits: limits });
  const second = await publishCacheEntry({ root: cacheRoot, identity: secondIdentity, artifacts: artifacts(), signingKey: key, cacheLimits: limits });
  await fs.appendFile(path.join(cacheRoot, 'access.log'), 'crash-truncated-tail');
  await readCacheEntry({ root: cacheRoot, key: first.key, expectedIdentity: firstIdentity, expectedPublicKey: key.publicKeyBytes });
  const third = await publishCacheEntry({ root: cacheRoot, identity: thirdIdentity, artifacts: artifacts(), signingKey: key, cacheLimits: limits });
  await fs.access(path.join(cacheRoot, 'entries', first.key));
  await assert.rejects(() => fs.access(path.join(cacheRoot, 'entries', second.key)));
  await fs.access(path.join(cacheRoot, 'entries', third.key));
});

test('cache maintenance removes crash-left orphan blobs before applying quota', async (t) => {
  const root = await temporaryDirectory(t);
  const { key } = await signingFixture(root);
  const cacheRoot = path.join(root, 'cache');
  const orphan = path.join(cacheRoot, 'blobs', 'f'.repeat(64));
  const temporary = path.join(cacheRoot, 'blobs', `.${'e'.repeat(64)}-00000000-0000-4000-8000-000000000000.tmp`);
  await fs.mkdir(path.dirname(orphan), { recursive: true });
  await fs.writeFile(orphan, Buffer.alloc(1024 * 1024 - 1));
  await fs.writeFile(temporary, Buffer.alloc(1024 * 1024 - 1));
  const cacheIdentity = identity({ signerFingerprintSha256: signerFingerprintSha256(key.publicKeyBytes) });
  const published = await publishCacheEntry({
    root: cacheRoot, identity: cacheIdentity, artifacts: artifacts(), signingKey: key,
    cacheLimits: { maxEntries: 1, maxBytes: 1024 * 1024, staleStagingMs: 1 },
  });
  await assert.rejects(() => fs.access(orphan));
  await assert.rejects(() => fs.access(temporary));
  await fs.access(path.join(cacheRoot, 'entries', published.key));
});

test('cache maintenance aborts before reclaiming blobs referenced by an unreadable visible receipt', async (t) => {
  const root = await temporaryDirectory(t);
  const { key } = await signingFixture(root);
  const cacheRoot = path.join(root, 'cache');
  const signer = signerFingerprintSha256(key.publicKeyBytes);
  const firstIdentity = identity({ signerFingerprintSha256: signer });
  const first = await publishCacheEntry({ root: cacheRoot, identity: firstIdentity, artifacts: artifacts(), signingKey: key });
  const receiptPath = path.join(cacheRoot, 'entries', first.key, 'receipt.json');
  const receipt = JSON.parse(await fs.readFile(receiptPath, 'utf8'));
  const protectedBlob = path.join(cacheRoot, 'blobs', receipt.blobs['geometry.glb'].sha256);
  await fs.writeFile(receiptPath, '{corrupt');

  await assert.rejects(
    () => publishCacheEntry({
      root: cacheRoot,
      identity: identity({ ...firstIdentity, sourceSha256: '8'.repeat(64) }),
      artifacts: artifacts(),
      signingKey: key,
      cacheLimits: { maxEntries: 1, maxBytes: 1024 * 1024, staleStagingMs: 1 },
    }),
    (error) => error.code === 'reference-cache-invalid',
  );
  await fs.access(protectedBlob);
  await fs.access(path.join(cacheRoot, 'entries', first.key));

  await fs.writeFile(receiptPath, '{}');
  await assert.rejects(
    () => publishCacheEntry({
      root: cacheRoot,
      identity: identity({ ...firstIdentity, sourceSha256: '7'.repeat(64) }),
      artifacts: artifacts(),
      signingKey: key,
      cacheLimits: { maxEntries: 1, maxBytes: 1024 * 1024, staleStagingMs: 1 },
    }),
    (error) => error.code === 'reference-cache-invalid',
  );
  await fs.access(protectedBlob);
});
