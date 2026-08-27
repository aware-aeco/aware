import { createPrivateKey, createPublicKey, randomUUID, sign, verify } from 'node:crypto';
import fs from 'node:fs/promises';
import path from 'node:path';
import {
  assertClosedObject, assertSha256, canonicalJsonBytes, lowerableLimits, ModelReaderError, parseJsonStrict,
  providerFingerprintSha256, sha256,
} from './model-contract.mjs';

const SECRET_PREFIX = Buffer.from('302e020100300506032b657004220420', 'hex');
const PUBLIC_PREFIX = Buffer.from('302a300506032b6570032100', 'hex');
const REQUIRED_ARTIFACTS = ['geometry.glb', 'entities.json', 'properties.json', 'relationships.json'];
const ENTRY_FILES = ['receipt.json', 'receipt.sig'];
const DEFAULT_CACHE_LIMITS = Object.freeze({ maxEntries: 64, maxBytes: 1024 * 1024 * 1024, staleStagingMs: 60 * 60_000 });
const MAX_RECEIPT_BYTES = 64 * 1024;
const MAX_SIGNATURE_BYTES = 4 * 1024;
const MAX_ACCESS_JOURNAL_BYTES = 256 * 1024;
const CACHE_READ_CHUNK_BYTES = 1024 * 1024;

function cacheError(code, message, details = undefined) {
  throw new ModelReaderError(code, 'cache', false, message, details);
}

function throwIfCancelled(signal) {
  if (signal?.aborted) throw new ModelReaderError('reference-cancelled', 'cache', false, 'Cache reading was cancelled.');
}

async function readBoundedFile(file, maxBytes, signal = undefined) {
  throwIfCancelled(signal);
  const handle = await fs.open(file, 'r');
  try {
    const before = await handle.stat();
    if (!before.isFile() || !Number.isSafeInteger(before.size) || before.size > maxBytes) throw new TypeError('cache file exceeds its byte limit');
    const bytes = Buffer.allocUnsafe(before.size);
    let offset = 0;
    while (offset < bytes.length) {
      throwIfCancelled(signal);
      const length = Math.min(bytes.length - offset, CACHE_READ_CHUNK_BYTES);
      const { bytesRead } = await handle.read(bytes, offset, length, offset);
      if (bytesRead === 0) throw new TypeError('cache file changed while it was read');
      offset += bytesRead;
    }
    const after = await handle.stat();
    if (after.size !== before.size) throw new TypeError('cache file changed while it was read');
    return bytes;
  } finally { await handle.close(); }
}

function parseKeyFile(text, marker, size, label) {
  const parts = text.trim().split(/\s+/);
  if (parts.length !== 2 || parts[0] !== marker) cacheError('reference-signing-key-invalid', `${label} key is not in AWARE format.`);
  let bytes;
  try { bytes = Buffer.from(parts[1], 'base64'); }
  catch (error) { cacheError('reference-signing-key-invalid', `${label} key is invalid.`, error); }
  if (bytes.length !== size || bytes.toString('base64') !== parts[1]) cacheError('reference-signing-key-invalid', `${label} key is invalid.`);
  return bytes;
}

export async function loadAwareSigningKey(secretPath, publicPath) {
  let secretText; let publicText;
  try { [secretText, publicText] = await Promise.all([fs.readFile(secretPath, 'utf8'), fs.readFile(publicPath, 'utf8')]); }
  catch (error) { cacheError('reference-signing-key-missing', 'The model-reader signing key is unavailable.', error); }
  const secretKeyBytes = parseKeyFile(secretText, 'ed25519-secret-key-v1', 32, 'secret');
  const publicKeyBytes = parseKeyFile(publicText, 'ed25519-public-key-v1', 32, 'public');
  const privateKey = createPrivateKey({ key: Buffer.concat([SECRET_PREFIX, secretKeyBytes]), format: 'der', type: 'pkcs8' });
  const derived = createPublicKey(privateKey).export({ format: 'der', type: 'spki' }).subarray(-32);
  if (!derived.equals(publicKeyBytes)) cacheError('reference-signing-key-invalid', 'The model-reader keypair does not match.');
  return { privateKey, publicKeyBytes };
}

export function signerFingerprintSha256(publicKeyBytes) {
  if (!Buffer.isBuffer(publicKeyBytes) || publicKeyBytes.length !== 32) throw new TypeError('public key must be 32 bytes');
  return sha256(publicKeyBytes);
}

function validateIdentity(identity) {
  try { assertClosedObject(identity, ['sourceSha256', 'canonicalRequest', 'providerFingerprint', 'signerFingerprintSha256'], [], 'cache identity'); }
  catch (error) { cacheError('reference-cache-identity-invalid', 'Cache identity is not closed.', error); }
  assertSha256(identity.sourceSha256, 'sourceSha256');
  assertSha256(identity.signerFingerprintSha256, 'signerFingerprintSha256');
  return identity;
}

export function cacheKeySha256(identity) {
  return sha256(canonicalJsonBytes(validateIdentity(identity)));
}

async function ensureLayout(root) {
  await fs.mkdir(path.join(root, 'blobs'), { recursive: true, mode: 0o700 });
  await fs.mkdir(path.join(root, 'entries'), { recursive: true, mode: 0o700 });
  await fs.mkdir(path.join(root, 'staging'), { recursive: true, mode: 0o700 });
  await fs.mkdir(path.join(root, 'locks'), { recursive: true, mode: 0o700 });
  await fs.mkdir(path.join(root, 'kernel-locks'), { recursive: true, mode: 0o700 });
}

async function accessRecency(root) {
  const journal = path.join(root, 'access.log');
  let bytes;
  try { bytes = await readBoundedFile(journal, MAX_ACCESS_JOURNAL_BYTES); }
  catch (error) {
    if (error?.code === 'ENOENT') return new Map();
    cacheError('reference-cache-invalid', 'The cache access journal is invalid or exceeds its byte limit.', error);
  }
  const text = bytes.toString('utf8');
  const lastNewline = text.lastIndexOf('\n');
  const complete = lastNewline < 0 ? '' : text.slice(0, lastNewline);
  if (!complete) return new Map();
  const latest = new Map();
  for (const [sequence, key] of complete.split('\n').entries()) {
    try { assertSha256(key, 'cache access key'); }
    catch (error) { cacheError('reference-cache-invalid', 'The cache access journal contains an invalid record.', error); }
    latest.set(key, sequence);
  }
  return latest;
}

async function recordCacheAccess(root, key) {
  const journal = path.join(root, 'access.log');
  const line = `${key}\n`;
  let size = 0;
  try { size = (await fs.stat(journal)).size; }
  catch (error) { if (error?.code !== 'ENOENT') throw error; }
  let hasTruncatedTail = false;
  if (size > 0) {
    const handle = await fs.open(journal, 'r');
    try {
      const tail = Buffer.allocUnsafe(1);
      const { bytesRead } = await handle.read(tail, 0, 1, size - 1);
      hasTruncatedTail = bytesRead !== 1 || tail[0] !== 0x0a;
    } finally { await handle.close(); }
  }
  if (hasTruncatedTail || size + Buffer.byteLength(line) > MAX_ACCESS_JOURNAL_BYTES) {
    const latest = await accessRecency(root);
    latest.delete(key);
    const compacted = [...latest.entries()]
      .sort((a, b) => a[1] - b[1] || Buffer.compare(Buffer.from(a[0]), Buffer.from(b[0])))
      .map(([entryKey]) => `${entryKey}\n`).join('') + line;
    const temporary = path.join(root, `.access-${randomUUID()}.tmp`);
    const handle = await fs.open(temporary, 'wx', 0o600);
    try { await handle.writeFile(compacted); await handle.sync(); }
    finally { await handle.close(); }
    try { await fs.rename(temporary, journal); }
    finally { await fs.rm(temporary, { force: true }); }
    return;
  }
  const handle = await fs.open(journal, 'a', 0o600);
  try { await handle.writeFile(line); await handle.sync(); }
  finally { await handle.close(); }
}

function boundedCacheLimits(overrides = {}) {
  const limits = { ...DEFAULT_CACHE_LIMITS, ...overrides };
  for (const name of Object.keys(DEFAULT_CACHE_LIMITS)) {
    if (!Number.isSafeInteger(limits[name]) || limits[name] <= 0 || limits[name] > DEFAULT_CACHE_LIMITS[name]) {
      cacheError('reference-cache-limits-invalid', 'Cache limits are invalid.');
    }
  }
  return limits;
}

async function receiptDigests(entry) {
  try {
    const receipt = parseJsonStrict(await readBoundedFile(path.join(entry, 'receipt.json'), MAX_RECEIPT_BYTES));
    assertClosedObject(receipt, ['schemaVersion', 'key', 'identitySha256', 'blobs'], [], 'cache receipt');
    assertSha256(receipt.key, 'cache key');
    assertSha256(receipt.identitySha256, 'cache identity');
    if (receipt.schemaVersion !== 'model-reference-cache-receipt/v1' || receipt.key !== path.basename(entry)) throw new TypeError('cache receipt identity mismatch');
    const expectedNames = [...REQUIRED_ARTIFACTS, 'manifest.json'].sort();
    if (JSON.stringify(Object.keys(receipt.blobs).sort()) !== JSON.stringify(expectedNames)) throw new TypeError('cache receipt artifact set mismatch');
    return expectedNames.map((name) => {
      const record = receipt.blobs[name];
      assertClosedObject(record, ['sha256', 'bytes'], [], `cache artifact ${name}`);
      assertSha256(record.sha256);
      if (!Number.isSafeInteger(record.bytes) || record.bytes < 0) throw new TypeError('cache receipt byte count is invalid');
      return record.sha256;
    });
  } catch (error) {
    cacheError('reference-cache-invalid', 'Cache maintenance found an unreadable visible receipt.', error);
  }
}

async function maintainCache(root, key, blobs, overrides) {
  const limits = boundedCacheLimits(overrides);
  const recency = await accessRecency(root);
  const now = Date.now();
  for (const name of await fs.readdir(path.join(root, 'staging'))) {
    const candidate = path.join(root, 'staging', name);
    try {
      const stat = await fs.stat(candidate);
      if (now - stat.mtimeMs > limits.staleStagingMs) await fs.rm(candidate, { recursive: true, force: true });
    } catch { /* a concurrent publisher moved or removed it */ }
  }
  const entries = [];
  for (const name of await fs.readdir(path.join(root, 'entries'))) {
    const entry = path.join(root, 'entries', name);
    try {
      const stat = await fs.stat(entry);
      if (stat.isDirectory()) entries.push({ name, entry, mtimeMs: stat.mtimeMs, accessSequence: recency.get(name) });
    } catch { /* concurrent eviction */ }
  }
  entries.sort((a, b) => {
    if (a.accessSequence !== undefined || b.accessSequence !== undefined) {
      if (a.accessSequence === undefined) return -1;
      if (b.accessSequence === undefined) return 1;
      if (a.accessSequence !== b.accessSequence) return a.accessSequence - b.accessSequence;
    } else if (a.mtimeMs !== b.mtimeMs) return a.mtimeMs - b.mtimeMs;
    return Buffer.compare(Buffer.from(a.name), Buffer.from(b.name));
  });
  const blobDirectory = path.join(root, 'blobs');
  let blobBytes = 0;
  const existing = new Set();
  for (const name of await fs.readdir(blobDirectory)) {
    try { const stat = await fs.stat(path.join(blobDirectory, name)); if (stat.isFile()) { blobBytes += stat.size; existing.add(name); } }
    catch { /* concurrent publication */ }
  }
  const initiallyReachable = new Set();
  for (const entry of entries) {
    for (const digest of await receiptDigests(entry.entry)) initiallyReachable.add(digest);
  }
  for (const digest of [...existing].sort()) {
    if (initiallyReachable.has(digest)) continue;
    try {
      const stat = await fs.stat(path.join(blobDirectory, digest));
      await fs.rm(path.join(blobDirectory, digest), { force: true });
      blobBytes -= stat.size;
      existing.delete(digest);
    } catch { /* crash cleanup is best-effort under the held maintenance fence */ }
  }
  const incomingRecords = new Map(Object.values(blobs).map((record) => [record.sha256, record.bytes]));
  let incoming = [...incomingRecords].filter(([digest]) => !existing.has(digest)).reduce((sum, [, bytes]) => sum + bytes, 0);
  const targetExists = entries.some((entry) => entry.name === key);
  while ((!targetExists && entries.length >= limits.maxEntries) || blobBytes + incoming > limits.maxBytes) {
    const victim = entries.find((entry) => entry.name !== key);
    if (!victim) cacheError('reference-cache-full', 'The bounded model-reader cache is full.');
    await fs.rm(victim.entry, { recursive: true, force: true });
    entries.splice(entries.indexOf(victim), 1);
    const reachable = new Set();
    for (const entry of entries) for (const digest of await receiptDigests(entry.entry)) reachable.add(digest);
    for (const digest of [...existing].sort()) {
      if (reachable.has(digest)) continue;
      try {
        const stat = await fs.stat(path.join(blobDirectory, digest));
        await fs.rm(path.join(blobDirectory, digest), { force: true });
        blobBytes -= stat.size;
        if (incomingRecords.has(digest)) incoming += incomingRecords.get(digest);
        existing.delete(digest);
      }
      catch { /* concurrent publication or cleanup */ }
    }
  }
  if (blobBytes + incoming > limits.maxBytes) cacheError('reference-cache-full', 'The bounded model-reader cache is full.');
}

export async function cacheFencePath(root, key) {
  assertSha256(key, 'cache key');
  await ensureLayout(root);
  return path.resolve(root, 'kernel-locks', `${key}.lock`);
}

export async function cacheMaintenanceFencePath(root) {
  await ensureLayout(root);
  return path.resolve(root, 'kernel-locks', 'maintenance.lock');
}

async function writeBlob(root, digest, bytes) {
  const target = path.join(root, 'blobs', digest);
  const temporary = path.join(root, 'blobs', `.${digest}-${randomUUID()}.tmp`);
  try {
    const handle = await fs.open(temporary, 'wx', 0o600);
    try { await handle.writeFile(bytes); await handle.sync(); }
    finally { await handle.close(); }
    try { await fs.link(temporary, target); }
    catch (error) { if (error?.code !== 'EEXIST') throw error; }
  } catch (error) {
    if (error?.code !== 'EEXIST') throw error;
  } finally {
    await fs.rm(temporary, { force: true });
  }
  let stored;
  try { stored = await readBoundedFile(target, bytes.length); }
  catch (error) { cacheError('reference-cache-tampered', 'A content-addressed cache blob is invalid or exceeds its byte limit.', error); }
  if (sha256(stored) !== digest) cacheError('reference-cache-tampered', 'A content-addressed cache blob has an invalid digest.');
  return target;
}

function signatureText(receiptBytes, signingKey) {
  const signature = sign(null, Buffer.from(sha256(receiptBytes), 'hex'), signingKey.privateKey).toString('base64');
  return Buffer.from(`ed25519-signature-v1\nover-sha256-of: receipt.json\nsignature: ${signature}\npublic-key: ${signingKey.publicKeyBytes.toString('base64')}\n`);
}

function verifySignature(receiptBytes, signatureBytes, expectedPublicKey) {
  const text = signatureBytes.toString('utf8');
  const lines = new Map(text.trim().split(/\r?\n/).slice(1).map((line) => {
    const split = line.indexOf(':'); return [line.slice(0, split), line.slice(split + 1).trim()];
  }));
  if (!text.startsWith('ed25519-signature-v1\n') || lines.get('over-sha256-of') !== 'receipt.json') cacheError('reference-cache-signature-invalid', 'Cache receipt signature is invalid.');
  const publicBytes = Buffer.from(lines.get('public-key') ?? '', 'base64');
  if (!publicBytes.equals(expectedPublicKey)) cacheError('reference-cache-signer-mismatch', 'Cache receipt was signed by an unexpected signer.');
  const signature = Buffer.from(lines.get('signature') ?? '', 'base64');
  const publicKey = createPublicKey({ key: Buffer.concat([PUBLIC_PREFIX, publicBytes]), format: 'der', type: 'spki' });
  if (signature.length !== 64 || !verify(null, Buffer.from(sha256(receiptBytes), 'hex'), publicKey, signature)) cacheError('reference-cache-signature-invalid', 'Cache receipt signature does not verify.');
}

function artifactManifest(identity, artifacts, details = {}) {
  try { assertClosedObject(details, [], ['coverage', 'frame', 'canonicalRequestSha256', 'providerFingerprintSha256'], 'manifest details'); }
  catch (error) { cacheError('reference-cache-artifacts-invalid', 'Manifest details are not closed.', error); }
  const names = Object.keys(artifacts).sort();
  if (names.length !== REQUIRED_ARTIFACTS.length || !REQUIRED_ARTIFACTS.every((name) => names.includes(name))) cacheError('reference-cache-artifacts-invalid', 'A conversion must publish exactly four component artifacts.');
  const records = {};
  for (const name of REQUIRED_ARTIFACTS) {
    if (!Buffer.isBuffer(artifacts[name])) cacheError('reference-cache-artifacts-invalid', `Artifact ${name} is not binary-safe bytes.`);
    records[name] = { sha256: sha256(artifacts[name]), bytes: artifacts[name].length };
  }
  return { schemaVersion: 'model-reference-manifest/v1', identity, ...details, artifacts: records };
}

export async function publishCacheEntry({ root, identity, artifacts, signingKey, details = {}, limits: limitOverrides = {}, cacheLimits = undefined, withMaintenanceFence }) {
  await ensureLayout(root);
  let limits;
  try { limits = lowerableLimits(limitOverrides); }
  catch (error) { cacheError('reference-cache-limits-invalid', 'Cache publication limits are invalid.', error); }
  const key = cacheKeySha256(identity);
  const manifest = artifactManifest(identity, artifacts, details);
  const manifestBytes = canonicalJsonBytes(manifest);
  if (manifestBytes.length > limits.maxComponentJsonBytes) cacheError('reference-cache-artifacts-invalid', 'The cache manifest exceeds its component byte limit.');
  const all = { ...artifacts, 'manifest.json': manifestBytes };
  const blobs = {};
  for (const [name, bytes] of Object.entries(all)) blobs[name] = { sha256: sha256(bytes), bytes: bytes.length };
  if (typeof withMaintenanceFence !== 'function') cacheError('reference-cache-fence-missing', 'Cache publication requires a process-wide maintenance fence.');
  return await withMaintenanceFence(async () => {
    await maintainCache(root, key, blobs, cacheLimits);
    for (const [name, bytes] of Object.entries(all)) {
      await writeBlob(root, blobs[name].sha256, bytes);
    }
    const receipt = { schemaVersion: 'model-reference-cache-receipt/v1', key, identitySha256: sha256(canonicalJsonBytes(identity)), blobs };
    const receiptBytes = canonicalJsonBytes(receipt);
    const signatureBytes = signatureText(receiptBytes, signingKey);
    const staging = path.join(root, 'staging', `${key}-${randomUUID()}`);
    await fs.mkdir(staging, { mode: 0o700 });
    await fs.writeFile(path.join(staging, 'receipt.json'), receiptBytes, { mode: 0o600, flag: 'wx' });
    await fs.writeFile(path.join(staging, 'receipt.sig'), signatureBytes, { mode: 0o600, flag: 'wx' });
    const target = path.join(root, 'entries', key);
    try { await fs.rename(staging, target); }
    catch (error) {
      if (!['EEXIST', 'ENOTEMPTY', 'EPERM'].includes(error?.code)) throw error;
      await fs.rm(staging, { recursive: true, force: true });
    }
    await recordCacheAccess(root, key);
    return { key, manifest, receipt };
  });
}

async function readCacheEntryUnsafe({
  root, key, expectedIdentity = undefined, expectedPublicKey, limits: limitOverrides = {},
  identityFilter = undefined, signal = undefined,
}) {
  assertSha256(key, 'cache key');
  let limits;
  try { limits = lowerableLimits(limitOverrides); }
  catch (error) { cacheError('reference-cache-limits-invalid', 'Cache read limits are invalid.', error); }
  const entry = path.join(root, 'entries', key);
  let names;
  try { names = (await fs.readdir(entry)).sort(); }
  catch (error) { cacheError('reference-cache-miss', 'The model conversion is not cached.', error); }
  if (JSON.stringify(names) !== JSON.stringify(ENTRY_FILES)) cacheError('reference-cache-entry-open', 'Cache entry is not closed; missing or extra files were found.');
  let receiptBytes; let signatureBytes;
  try {
    [receiptBytes, signatureBytes] = await Promise.all([
      readBoundedFile(path.join(entry, 'receipt.json'), MAX_RECEIPT_BYTES, signal),
      readBoundedFile(path.join(entry, 'receipt.sig'), MAX_SIGNATURE_BYTES, signal),
    ]);
  } catch (error) {
    if (error?.code === 'reference-cancelled') throw error;
    cacheError('reference-cache-entry-invalid', 'Cache entry control files are invalid or exceed their byte limits.', error);
  }
  verifySignature(receiptBytes, signatureBytes, expectedPublicKey);
  let receipt;
  try { receipt = parseJsonStrict(receiptBytes); assertClosedObject(receipt, ['schemaVersion', 'key', 'identitySha256', 'blobs'], [], 'cache receipt'); }
  catch (error) { cacheError('reference-cache-entry-invalid', 'Cache receipt is invalid.', error); }
  if (receipt.schemaVersion !== 'model-reference-cache-receipt/v1' || receipt.key !== key) cacheError('reference-cache-entry-invalid', 'Cache receipt identity is invalid.');
  const blobNames = Object.keys(receipt.blobs).sort();
  const expectedNames = [...REQUIRED_ARTIFACTS, 'manifest.json'].sort();
  if (JSON.stringify(blobNames) !== JSON.stringify(expectedNames)) cacheError('reference-cache-entry-open', 'Cache receipt has missing or extra artifacts.');
  for (const name of expectedNames) {
    const record = receipt.blobs[name];
    const artifactLimit = name === 'geometry.glb' ? limits.maxCanonicalGlbBytes : limits.maxComponentJsonBytes;
    try {
      assertClosedObject(record, ['sha256', 'bytes'], [], `cache artifact ${name}`);
      assertSha256(record.sha256);
      if (!Number.isSafeInteger(record.bytes) || record.bytes < 0 || record.bytes > artifactLimit) throw new TypeError('cache artifact exceeds its byte limit');
    }
    catch (error) { cacheError('reference-cache-entry-invalid', 'Cache artifact receipt is invalid.', error); }
  }
  // The signed manifest is the small authenticated identity index. Validate it and apply
  // caller pins before touching any potentially large component blob.
  const manifestRecord = receipt.blobs['manifest.json'];
  let manifestBytes;
  try { manifestBytes = await readBoundedFile(path.join(root, 'blobs', manifestRecord.sha256), manifestRecord.bytes, signal); }
  catch (error) {
    if (error?.code === 'reference-cancelled') throw error;
    cacheError('reference-cache-tampered', 'Cache manifest is invalid or exceeds its byte limit.', error);
  }
  if (manifestBytes.length !== manifestRecord.bytes || sha256(manifestBytes) !== manifestRecord.sha256) cacheError('reference-cache-tampered', 'Cache manifest digest is invalid or tampered.');
  let manifest;
  try { manifest = parseJsonStrict(manifestBytes); }
  catch (error) { cacheError('reference-cache-entry-invalid', 'Cached manifest is invalid.', error); }
  let identityBytes;
  try { identityBytes = canonicalJsonBytes(validateIdentity(manifest.identity)); }
  catch (error) { cacheError('reference-cache-entry-invalid', 'Cached manifest identity is invalid.', error); }
  if (receipt.identitySha256 !== sha256(identityBytes) || key !== cacheKeySha256(manifest.identity)) {
    cacheError('reference-cache-identity-mismatch', 'Cached manifest identity does not match its signed receipt.');
  }
  if (expectedIdentity !== undefined
      && !identityBytes.equals(canonicalJsonBytes(validateIdentity(expectedIdentity)))) {
    cacheError('reference-cache-identity-mismatch', 'Cache identity does not match the request.');
  }
  if (typeof identityFilter === 'function' && !identityFilter(manifest.identity)) {
    cacheError('reference-cache-identity-mismatch', 'Cache identity does not match the pinned lookup fields.');
  }
  for (const name of REQUIRED_ARTIFACTS) {
    if (manifest.artifacts?.[name]?.sha256 !== receipt.blobs[name].sha256 || manifest.artifacts?.[name]?.bytes !== receipt.blobs[name].bytes) cacheError('reference-cache-entry-invalid', 'Cached manifest does not reconcile with its blobs.');
  }
  const artifacts = { 'manifest.json': manifestBytes };
  for (const name of REQUIRED_ARTIFACTS) {
    const record = receipt.blobs[name];
    let bytes;
    try { bytes = await readBoundedFile(path.join(root, 'blobs', record.sha256), record.bytes, signal); }
    catch (error) {
      if (error?.code === 'reference-cancelled') throw error;
      cacheError('reference-cache-tampered', 'Cache artifact is invalid or exceeds its byte limit.', error);
    }
    if (bytes.length !== record.bytes || sha256(bytes) !== record.sha256) cacheError('reference-cache-tampered', 'Cache artifact digest is invalid or tampered.');
    artifacts[name] = bytes;
  }
  return { manifest, receipt, receiptSha256: sha256(receiptBytes), artifacts };
}

export async function readCacheEntry(options) {
  if (typeof options?.withMaintenanceFence !== 'function') cacheError('reference-cache-fence-missing', 'Cache reads require the process-wide maintenance fence.');
  return await options.withMaintenanceFence(async () => {
    const result = await readCacheEntryUnsafe(options);
    await recordCacheAccess(options.root, options.key);
    return result;
  });
}

/// Locate an authenticated entry using only caller-pinned identity fields. The provider's full
/// fingerprint is intentionally learned from the signed cached manifest, so a warm hit does not
/// need to launch the commercial adapter merely to repeat `describe`.
export async function findCacheEntry(options) {
  if (typeof options?.withMaintenanceFence !== 'function') cacheError('reference-cache-fence-missing', 'Cache reads require the process-wide maintenance fence.');
  assertSha256(options.expectedSourceSha256, 'expected source sha256');
  assertSha256(options.expectedProviderFingerprintSha256, 'expected provider fingerprint sha256');
  assertSha256(options.expectedSignerFingerprintSha256, 'expected signer fingerprint sha256');
  const expectedRequestBytes = canonicalJsonBytes(options.expectedCanonicalRequest);
  return await options.withMaintenanceFence(async () => {
    await ensureLayout(options.root);
    const names = (await fs.readdir(path.join(options.root, 'entries'))).sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b)));
    for (const key of names) {
      if (!/^[0-9a-f]{64}$/.test(key)) continue;
      let result;
      try {
        result = await readCacheEntryUnsafe({
          ...options,
          key,
          expectedIdentity: undefined,
          identityFilter: (identity) => identity.sourceSha256 === options.expectedSourceSha256
            && identity.signerFingerprintSha256 === options.expectedSignerFingerprintSha256
            && providerFingerprintSha256(identity.providerFingerprint) === options.expectedProviderFingerprintSha256
            && canonicalJsonBytes(identity.canonicalRequest).equals(expectedRequestBytes),
        });
      } catch (error) {
        if (error?.code === 'reference-cancelled') throw error;
        continue;
      }
      await recordCacheAccess(options.root, key);
      return { key, ...result };
    }
    cacheError('reference-cache-miss', 'The model conversion is not cached.');
  });
}

function defaultProcessIdentity() { return { pid: process.pid, start: process.uptime().toFixed(6) }; }
async function defaultIsAlive(owner) {
  try { process.kill(owner.pid, 0); return true; } catch { return null; }
}

function parseCacheOwner(bytes) {
  const owner = parseJsonStrict(bytes);
  assertClosedObject(owner, ['token', 'pid', 'processStart', 'heartbeatAt'], [], 'cache owner');
  if (typeof owner.token !== 'string' || owner.token.length === 0) throw new TypeError('cache owner token must be a non-empty string');
  if (!Number.isSafeInteger(owner.pid) || owner.pid <= 0) throw new TypeError('cache owner pid must be a positive safe integer');
  if (typeof owner.processStart !== 'string' || owner.processStart.length === 0) throw new TypeError('cache owner processStart must be a non-empty string');
  if (typeof owner.heartbeatAt !== 'string' || !Number.isFinite(Date.parse(owner.heartbeatAt))) throw new TypeError('cache owner heartbeatAt must be an ISO timestamp');
  return owner;
}

export async function acquireCacheOwner({ root, key, processIdentity = defaultProcessIdentity(), isOwnerAlive = defaultIsAlive, staleMs = 30_000, now = () => Date.now(), fenced = false, removeLockDirectory = fs.rm }) {
  assertSha256(key, 'cache key'); await ensureLayout(root);
  const lockDirectory = path.join(root, 'locks', key);
  const ownerPath = path.join(lockDirectory, 'owner.json');
  const create = async () => {
    await fs.mkdir(lockDirectory, { mode: 0o700 });
    const lease = { token: randomUUID(), pid: processIdentity.pid, processStart: processIdentity.start, heartbeatAt: new Date(now()).toISOString() };
    await fs.writeFile(ownerPath, canonicalJsonBytes(lease), { mode: 0o600, flag: 'wx' });
    return { root, key, lockDirectory, ownerPath, token: lease.token };
  };
  try { return await create(); }
  catch (error) {
    if (error?.code !== 'EEXIST') throw error;
  }
  let owner;
  try { owner = parseCacheOwner(await fs.readFile(ownerPath)); }
  catch (error) {
    if (!fenced) cacheError('reference-cache-owned', 'The cache key has an unverifiable owner.', error);
    try { await removeLockDirectory(lockDirectory, { recursive: true, force: true }); }
    catch (removeError) { cacheError('reference-cache-owned', 'Fenced cache-owner debris could not be reclaimed.', removeError); }
    try { return await create(); }
    catch (replacementError) { cacheError('reference-cache-owned', 'Another conversion won cache ownership.', replacementError); }
  }
  const age = now() - Date.parse(owner.heartbeatAt);
  const alive = fenced ? false : await isOwnerAlive(owner);
  if (!fenced && (!Number.isFinite(age) || age <= staleMs || alive !== false)) cacheError('reference-cache-owned', 'The cache key is owned by another live or unverifiable conversion.');
  try { await removeLockDirectory(lockDirectory, { recursive: true, force: true }); }
  catch (error) { cacheError('reference-cache-owned', 'Stale cache-owner debris could not be reclaimed.', error); }
  try { return await create(); }
  catch (error) { cacheError('reference-cache-owned', 'Another conversion won cache ownership.', error); }
}

export async function releaseCacheOwner(lease, options = {}) {
  let owner;
  try { owner = parseCacheOwner(await fs.readFile(lease.ownerPath)); }
  catch (error) { cacheError('reference-cache-owner-token', 'Cache ownership cannot be verified.', error); }
  if (owner.token !== lease.token) cacheError('reference-cache-owner-token', 'Cache ownership token no longer matches.');
  try { await (options.removeLockDirectory ?? fs.rm)(lease.lockDirectory, { recursive: true, force: true }); }
  catch (error) { cacheError('reference-cache-owner-release', 'Cache ownership could not be released.', error); }
}
