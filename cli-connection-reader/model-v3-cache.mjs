import fs from 'node:fs/promises';
import path from 'node:path';
import { createPublicKey, sign, verify } from 'node:crypto';

import { canonicalJsonBytes, ModelReaderError, parseJsonStrict, sha256 } from './model-contract.mjs';

const CACHE_SCHEMA = 'aware.model-reference-cache/v3';
const CACHE_SIGNATURE_DOMAIN = Buffer.from('AWARE\0model-reference-reader\0v3-cache\0v1\0', 'ascii');
const PUBLIC_PREFIX = Buffer.from('302a300506032b6570032100', 'hex');

function cacheError(code, message, details = undefined) {
  throw new ModelReaderError(code, 'cache', false, message, details);
}

export function v3CacheKey(identity) {
  return sha256(canonicalJsonBytes({ schemaVersion: CACHE_SCHEMA, ...identity }));
}

function entryRoot(root, key) {
  if (typeof root !== 'string' || !path.isAbsolute(root) || !/^[0-9a-f]{64}$/.test(key)) {
    cacheError('reference-cache-invalid', 'Protocol-v3 cache identity is invalid.');
  }
  return path.join(root, 'v3', key);
}

async function copyVerified(source, target, receipt) {
  const bytes = await fs.readFile(source);
  if (bytes.length !== receipt.bytes || sha256(bytes) !== receipt.sha256) {
    cacheError('reference-cache-invalid', 'Canonical artifact changed before cache publication.');
  }
  await fs.mkdir(path.dirname(target), { recursive: true, mode: 0o700 });
  await fs.writeFile(target, bytes, { flag: 'wx', mode: 0o400 });
}

export async function publishV3Cache(root, key, identity, canonical, signingKey) {
  if (!signingKey?.privateKey || !Buffer.isBuffer(signingKey.publicKeyBytes)
      || signingKey.publicKeyBytes.length !== 32) {
    cacheError('reference-signing-key-invalid', 'Protocol-v3 cache publication requires the AWARE signing key.');
  }
  const target = entryRoot(root, key);
  await fs.mkdir(path.dirname(target), { recursive: true, mode: 0o700 });
  const staging = await fs.mkdtemp(path.join(path.dirname(target), '.tmp-'));
  try {
    for (const object of canonical.objects) {
      await copyVerified(object.pathname, path.join(staging, ...object.receipt.logicalPath.split('/')), object.receipt);
    }
    for (const index of Object.values(canonical.indexes)) {
      const pathname = path.join(staging, ...index.receipt.logicalPath.split('/'));
      await fs.writeFile(pathname, index.bytes, { flag: 'wx', mode: 0o400 });
    }
    await fs.writeFile(path.join(staging, 'model-reference-manifest.json'), canonical.root.bytes,
      { flag: 'wx', mode: 0o400 });
    const record = {
      schemaVersion: CACHE_SCHEMA, key, identity,
      artifactRootSha256: canonical.root.sha256,
      objects: canonical.root.manifest.objects,
    };
    const recordBytes = canonicalJsonBytes(record);
    const signature = sign(null, Buffer.concat([CACHE_SIGNATURE_DOMAIN, recordBytes]), signingKey.privateKey);
    await fs.writeFile(path.join(staging, 'cache.json'), recordBytes, { flag: 'wx', mode: 0o400 });
    await fs.writeFile(path.join(staging, 'cache.sig'), signature, { flag: 'wx', mode: 0o400 });
    try { await fs.rename(staging, target); }
    catch (error) {
      if (error?.code !== 'EEXIST' && error?.code !== 'ENOTEMPTY') throw error;
    }
  } finally {
    await fs.rm(staging, { recursive: true, force: true });
  }
  return await readV3Cache(root, key, identity, signingKey.publicKeyBytes);
}

export async function readV3Cache(root, key, identity, expectedPublicKey) {
  if (!Buffer.isBuffer(expectedPublicKey) || expectedPublicKey.length !== 32) {
    cacheError('reference-signing-key-invalid', 'Protocol-v3 cache verification requires the enrolled AWARE public key.');
  }
  const directory = entryRoot(root, key);
  let record; let rootBytes;
  try {
    const recordBytes = await fs.readFile(path.join(directory, 'cache.json'));
    const signature = await fs.readFile(path.join(directory, 'cache.sig'));
    const publicKey = createPublicKey({
      key: Buffer.concat([PUBLIC_PREFIX, expectedPublicKey]), format: 'der', type: 'spki',
    });
    if (signature.length !== 64
        || !verify(null, Buffer.concat([CACHE_SIGNATURE_DOMAIN, recordBytes]), publicKey, signature)) {
      throw new Error('cache signature mismatch');
    }
    record = parseJsonStrict(recordBytes, { maxBytes: 16 * 1024 * 1024, maxDepth: 32 });
    if (!recordBytes.equals(canonicalJsonBytes(record)) || record.schemaVersion !== CACHE_SCHEMA
        || record.key !== key || !canonicalJsonBytes(record.identity).equals(canonicalJsonBytes(identity))
        || !Array.isArray(record.objects)) throw new Error('cache record mismatch');
    rootBytes = await fs.readFile(path.join(directory, 'model-reference-manifest.json'));
  } catch (error) {
    if (error?.code === 'ENOENT') cacheError('reference-cache-miss', 'Protocol-v3 cache entry is absent.');
    cacheError('reference-cache-invalid', 'Protocol-v3 cache record is invalid.', error);
  }
  if (sha256(rootBytes) !== record.artifactRootSha256) {
    cacheError('reference-cache-invalid', 'Protocol-v3 artifact root failed cache verification.');
  }
  const manifest = parseJsonStrict(rootBytes, { maxBytes: 16 * 1024 * 1024, maxDepth: 64 });
  if (!rootBytes.equals(canonicalJsonBytes(manifest))
      || !canonicalJsonBytes(manifest.objects).equals(canonicalJsonBytes(record.objects))) {
    cacheError('reference-cache-invalid', 'Protocol-v3 artifact root does not match its cache record.');
  }
  const objects = []; const indexes = {};
  for (const receipt of record.objects) {
    const pathname = path.join(directory, ...receipt.logicalPath.split('/'));
    const bytes = await fs.readFile(pathname);
    if (bytes.length !== receipt.bytes || sha256(bytes) !== receipt.sha256) {
      cacheError('reference-cache-invalid', 'Protocol-v3 cached object failed digest verification.');
    }
    if (receipt.logicalKind.endsWith('-index')) {
      const family = receipt.logicalKind.slice(0, -'-index'.length);
      indexes[family] = { index: parseJsonStrict(bytes), bytes, receipt };
    } else objects.push({ pathname, receipt });
  }
  if (!['geometry', 'entities', 'properties', 'relationships'].every((family) => indexes[family])) {
    cacheError('reference-cache-invalid', 'Protocol-v3 cache is missing an artifact index.');
  }
  return { root: { manifest, bytes: rootBytes, sha256: record.artifactRootSha256 }, indexes, objects };
}
