import { createHash } from 'node:crypto';
import { constants as fsConstants } from 'node:fs';
import fs from 'node:fs/promises';
import path from 'node:path';

import {
  assertClosedObject, assertSha256, canonicalJsonBytes, ModelReaderError, parseJsonStrict, sha256,
} from './model-contract.mjs';

const MANIFEST_SCHEMA = 'aware.model-provider-output-manifest/v1';
const COMPLETION_SCHEMA = 'aware.model-provider-output-completion/v1';
const MANIFEST_NAME = 'intermediate-manifest.json';
const COMPLETION_NAME = 'complete.json';
const OPAQUE_ID = /^[A-Za-z0-9._-]{1,128}$/;
const KINDS = Object.freeze({
  geometry: { pattern: /^geometry\/(\d{6})\.glb$/, mediaType: 'model/gltf-binary', maximum: 256 },
  entities: { pattern: /^metadata\/entities-(\d{6})\.jsonl$/, mediaType: 'application/x-ndjson', maximum: 256 },
  properties: { pattern: /^metadata\/properties-(\d{6})\.jsonl$/, mediaType: 'application/x-ndjson', maximum: 256 },
  relationships: { pattern: /^metadata\/relationships-(\d{6})\.jsonl$/, mediaType: 'application/x-ndjson', maximum: 256 },
  diagnostics: { pattern: /^metadata\/diagnostics-(\d{6})\.jsonl$/, mediaType: 'application/x-ndjson', maximum: 256 },
});
const DEFAULT_LIMITS = Object.freeze({
  manifestBytes: 16 * 1024 * 1024,
  completionBytes: 1024 * 1024,
  payloadBytes: 96 * 1024 * 1024,
  aggregateBytes: 2 * 1024 * 1024 * 1024,
  recordsPerShard: 5_000_000,
  recordBytes: 1024 * 1024,
});
const HARD_LIMITS = Object.freeze({
  manifestBytes: 32 * 1024 * 1024,
  completionBytes: 1024 * 1024,
  payloadBytes: 128 * 1024 * 1024,
  aggregateBytes: 4 * 1024 * 1024 * 1024,
  recordsPerShard: 10_000_000,
  recordBytes: 4 * 1024 * 1024,
});

function outputError(code, message, retryable = false, details = undefined) {
  throw new ModelReaderError(code, 'provider-output', retryable, message, details);
}

function checkCancellation(signal) {
  if (signal?.aborted) outputError('reference-cancelled', 'Provider output verification was cancelled.');
}

function samePath(left, right) {
  const normalize = (value) => process.platform === 'win32' ? path.resolve(value).toLowerCase() : path.resolve(value);
  return normalize(left) === normalize(right);
}

function directoryIdentity(stat) {
  return { dev: stat.dev, ino: stat.ino, mtimeNs: stat.mtimeNs };
}

function sameDirectoryIdentity(left, right) {
  const sameDevice = process.platform === 'win32' || left.dev === right.dev;
  return sameDevice && left.ino === right.ino && left.mtimeNs === right.mtimeNs;
}

export function providerOutputLimits(overrides = {}) {
  if (!overrides || typeof overrides !== 'object' || Array.isArray(overrides)) {
    outputError('reference-provider-output-limits-invalid', 'Provider output limits are invalid.');
  }
  const limits = { ...DEFAULT_LIMITS };
  for (const [name, value] of Object.entries(overrides)) {
    if (!Object.hasOwn(limits, name) || !Number.isSafeInteger(value) || value <= 0
        || value > HARD_LIMITS[name]) {
      outputError('reference-provider-output-limits-invalid', 'Provider output limits are invalid.');
    }
    limits[name] = value;
  }
  return limits;
}

function closed(value, required, optional, label) {
  try { return assertClosedObject(value, required, optional, label); }
  catch (error) { outputError('reference-provider-output-invalid', `The ${label} is invalid.`, false, error); }
}

function digest(value, label) {
  try { return assertSha256(value, label); }
  catch (error) { outputError('reference-provider-output-invalid', `${label} is invalid.`, false, error); }
}

function admittedBounds(value) {
  if (!Array.isArray(value) || value.length !== 6
      || value.some((entry) => typeof entry !== 'number' || !Number.isFinite(entry))
      || value[0] > value[3] || value[1] > value[4] || value[2] > value[5]) {
    outputError('reference-provider-output-invalid', 'A geometry tile receipt has invalid bounds.');
  }
  try { canonicalJsonBytes(value); }
  catch (error) { outputError('reference-provider-output-invalid', 'A geometry tile receipt has invalid bounds.', false, error); }
  return value;
}

function canonicalOutputJson(value, label) {
  try { return canonicalJsonBytes(value); }
  catch (error) { outputError('reference-provider-output-invalid', `The ${label} is invalid.`, false, error); }
}

async function safeDirectory(root, relative = '', rootReal = undefined) {
  const pathname = relative ? path.join(root, relative) : root;
  let stat; let real;
  try { stat = await fs.lstat(pathname, { bigint: true }); real = await fs.realpath(pathname); }
  catch (error) { outputError('reference-provider-output-incomplete', 'Provider output is incomplete.', false, error); }
  const admittedRoot = rootReal ?? real;
  const relativeToRoot = path.relative(admittedRoot, real);
  if (!stat.isDirectory() || stat.isSymbolicLink()
      || (!relative && !samePath(real, root))
      || (relative && (path.isAbsolute(relativeToRoot) || relativeToRoot === '..'
        || relativeToRoot.startsWith(`..${path.sep}`)))) {
    outputError('reference-provider-output-unsafe', 'Provider output contains an unsafe directory.');
  }
  return { pathname, real, identity: directoryIdentity(stat), rootReal: admittedRoot };
}

async function validateDirectory(record) {
  const current = await safeDirectory(
    path.dirname(record.pathname), path.basename(record.pathname), record.rootReal,
  );
  if (!samePath(current.real, record.real) || !sameDirectoryIdentity(current.identity, record.identity)) {
    outputError('reference-provider-output-changed', 'Provider output directories changed while they were verified.', true);
  }
}

function sameIdentity(before, opened, after) {
  const sameDevice = process.platform === 'win32' || (before.dev === opened.dev && opened.dev === after.dev);
  return sameDevice && before.ino === opened.ino && opened.ino === after.ino
    && before.size === opened.size && opened.size === after.size
    && before.mtimeNs === opened.mtimeNs && opened.mtimeNs === after.mtimeNs;
}

async function readStableFile(root, relative, maximumBytes, options = {}, directories) {
  const collect = options.collect !== false;
  const recordLimit = options.recordLimit;
  const pathname = path.join(root, ...relative.split('/'));
  const parentKey = path.posix.dirname(relative) === '.' ? '' : path.posix.dirname(relative);
  const parent = directories?.get(parentKey);
  let before; let opened; let after; let handle; let output;
  try {
    checkCancellation(options.signal);
    if (parent) await validateDirectory(parent);
    before = await fs.lstat(pathname, { bigint: true });
    const real = await fs.realpath(pathname);
    if (parent && !samePath(path.dirname(real), parent.real)) {
      outputError('reference-provider-output-unsafe', 'A provider output file escaped its admitted directory.');
    }
    if (!before.isFile() || before.isSymbolicLink() || before.size > BigInt(maximumBytes)) {
      outputError('reference-provider-output-unsafe', 'Provider output contains an unsafe or oversized file.');
    }
    handle = await fs.open(pathname, fsConstants.O_RDONLY | (fsConstants.O_NOFOLLOW ?? 0));
    if (options.copyTo) output = await fs.open(options.copyTo, 'wx', 0o600);
    opened = await handle.stat({ bigint: true });
    const chunks = []; const hash = createHash('sha256'); let bytes = 0;
    let records = 0; let lineBytes = 0; let lineChunks = [];
    const buffer = Buffer.allocUnsafe(1024 * 1024);
    for (;;) {
      checkCancellation(options.signal);
      const result = await handle.read(buffer, 0, buffer.length, null);
      if (result.bytesRead === 0) break;
      bytes += result.bytesRead;
      if (bytes > maximumBytes || bytes > Number(opened.size)) {
        outputError('reference-provider-output-changed', 'Provider output changed while it was verified.', true);
      }
      const chunk = buffer.subarray(0, result.bytesRead);
      hash.update(chunk);
      if (collect) chunks.push(Buffer.from(chunk));
      if (output) await output.writeFile(chunk);
      checkCancellation(options.signal);
      if (recordLimit !== undefined) {
        let segmentStart = 0;
        for (let index = 0; index < chunk.length; index += 1) {
          const byte = chunk[index];
          if (byte === 0x0d) outputError('reference-provider-output-invalid', 'Provider JSONL must use canonical LF line endings.');
          if (byte === 0x0a) {
            const segment = chunk.subarray(segmentStart, index);
            if (segment.length) lineChunks.push(Buffer.from(segment));
            lineBytes += segment.length;
            if (lineBytes === 0) outputError('reference-provider-output-invalid', 'Provider JSONL cannot contain empty records.');
            if (lineBytes > options.recordBytes) outputError('reference-provider-output-limit', 'A provider JSONL record exceeds its byte limit.');
            const line = Buffer.concat(lineChunks, lineBytes);
            let parsed;
            try { parsed = parseJsonStrict(line, { maxBytes: options.recordBytes, maxDepth: 64 }); }
            catch (error) { outputError('reference-provider-output-invalid', 'Provider JSONL contains an invalid record.', false, error); }
            if (!line.equals(canonicalOutputJson(parsed, 'provider JSONL record'))) {
              outputError('reference-provider-output-invalid', 'Provider JSONL records must be canonical JSON.');
            }
            records += 1; lineBytes = 0;
            lineChunks = []; segmentStart = index + 1;
            if (records > recordLimit) outputError('reference-provider-output-limit', 'A provider output shard exceeds its record limit.');
          }
        }
        const remainder = chunk.subarray(segmentStart);
        if (remainder.length) { lineChunks.push(Buffer.from(remainder)); lineBytes += remainder.length; }
        if (lineBytes > options.recordBytes) outputError('reference-provider-output-limit', 'A provider JSONL record exceeds its byte limit.');
      }
    }
    after = await handle.stat({ bigint: true });
    if (bytes !== Number(opened.size) || !sameIdentity(before, opened, after)) {
      outputError('reference-provider-output-changed', 'Provider output changed while it was verified.', true);
    }
    if (recordLimit !== undefined && lineBytes !== 0) {
      outputError('reference-provider-output-invalid', 'Every provider JSONL record must end with LF.');
    }
    if (output) { await output.sync(); await output.close(); output = undefined; await fs.chmod(options.copyTo, 0o400); }
    if (parent) await validateDirectory(parent);
    return {
      bytes: collect ? Buffer.concat(chunks, bytes) : undefined,
      length: bytes, sha256: hash.digest('hex'), records: recordLimit === undefined ? undefined : records,
    };
  } catch (error) {
    await output?.close().catch(() => {});
    if (options.copyTo) await fs.rm(options.copyTo, { force: true }).catch(() => {});
    if (error instanceof ModelReaderError) throw error;
    outputError('reference-provider-output-incomplete', 'Provider output could not be verified.', false, error);
  } finally {
    await handle?.close().catch(() => {});
  }
}

function overlaps(left, right) {
  const relative = path.relative(left, right);
  return relative === '' || (!path.isAbsolute(relative) && relative !== '..'
    && !relative.startsWith(`..${path.sep}`));
}

async function createAdmittedRoot(sourceRoot, admittedRoot) {
  if (typeof admittedRoot !== 'string' || !path.isAbsolute(admittedRoot)
      || overlaps(sourceRoot, admittedRoot) || overlaps(admittedRoot, sourceRoot)) {
    outputError('reference-provider-output-root-invalid', 'The admitted output root is invalid.');
  }
  const parent = path.dirname(admittedRoot);
  let stat; let real;
  try { stat = await fs.lstat(parent); real = await fs.realpath(parent); }
  catch (error) { outputError('reference-provider-output-root-invalid', 'The admitted output parent is unavailable.', false, error); }
  if (!stat.isDirectory() || stat.isSymbolicLink() || !samePath(real, parent)) {
    outputError('reference-provider-output-root-invalid', 'The admitted output parent is unsafe.');
  }
  let created = false;
  try {
    await fs.mkdir(admittedRoot, { recursive: false, mode: 0o700 });
    created = true;
    await fs.mkdir(path.join(admittedRoot, 'geometry'), { mode: 0o700 });
    await fs.mkdir(path.join(admittedRoot, 'metadata'), { mode: 0o700 });
  } catch (error) {
    if (created) await fs.rm(admittedRoot, { recursive: true, force: true }).catch(() => {});
    outputError('reference-provider-output-root-invalid', 'The admitted output root could not be created.', false, error);
  }
}

async function outputDirectories(root) {
  const rootRecord = await safeDirectory(root);
  return new Map([
    ['', rootRecord],
    ['geometry', await safeDirectory(root, 'geometry', rootRecord.real)],
    ['metadata', await safeDirectory(root, 'metadata', rootRecord.real)],
  ]);
}

async function actualFiles(directories) {
  const files = [];
  for (const directory of ['', 'geometry', 'metadata']) {
    const record = directories.get(directory);
    await validateDirectory(record);
    let entries;
    try { entries = await fs.readdir(record.pathname, { withFileTypes: true }); }
    catch (error) { outputError('reference-provider-output-incomplete', 'Provider output could not be listed.', false, error); }
    await validateDirectory(record);
    for (const entry of entries) {
      const relative = directory ? `${directory}/${entry.name}` : entry.name;
      if (!directory && ['geometry', 'metadata'].includes(entry.name) && entry.isDirectory()) continue;
      if (!entry.isFile() || entry.isSymbolicLink()) {
        outputError('reference-provider-output-unsafe', 'Provider output contains an unadmitted entry.');
      }
      files.push(relative);
    }
  }
  return files.sort();
}

function validateManifest(manifest, expected) {
  closed(manifest, [
    'schemaVersion', 'protocolVersion', 'formatId', 'capabilityId', 'providerPackageManifestSha256',
    'effectiveSourceSha256', 'conversionRequestSha256', 'files',
  ], [], 'provider output manifest');
  if (manifest.schemaVersion !== MANIFEST_SCHEMA || manifest.protocolVersion !== '3'
      || manifest.formatId !== expected.formatId || manifest.capabilityId !== expected.capabilityId
      || manifest.providerPackageManifestSha256 !== expected.providerPackageManifestSha256
      || manifest.effectiveSourceSha256 !== expected.effectiveSourceSha256
      || manifest.conversionRequestSha256 !== expected.conversionRequestSha256
      || typeof manifest.formatId !== 'string' || typeof manifest.capabilityId !== 'string'
      || !OPAQUE_ID.test(manifest.formatId) || !OPAQUE_ID.test(manifest.capabilityId)
      || !Array.isArray(manifest.files) || manifest.files.length < 1 || manifest.files.length > 1_024) {
    outputError('reference-provider-output-invalid', 'The provider output manifest does not match the conversion request.');
  }
  digest(manifest.providerPackageManifestSha256, 'providerPackageManifestSha256');
  digest(manifest.effectiveSourceSha256, 'effectiveSourceSha256');
  digest(manifest.conversionRequestSha256, 'conversionRequestSha256');
  const paths = new Set(); const ordinals = new Map();
  for (const entry of manifest.files) {
    closed(entry, ['path', 'kind', 'ordinal', 'mediaType', 'bytes', 'sha256', 'count'], ['bounds'], 'provider output file receipt');
    const contract = typeof entry.kind === 'string' && Object.hasOwn(KINDS, entry.kind)
      ? KINDS[entry.kind] : undefined;
    const match = typeof entry.path === 'string' && contract?.pattern.exec(entry.path);
    if (!match || entry.mediaType !== contract.mediaType || !Number.isSafeInteger(entry.ordinal)
        || entry.ordinal < 0 || entry.ordinal >= contract.maximum || entry.ordinal !== Number(match[1])
        || !Number.isSafeInteger(entry.bytes) || entry.bytes < 0
        || !Number.isSafeInteger(entry.count) || entry.count < 0 || paths.has(entry.path)) {
      outputError('reference-provider-output-invalid', 'The provider output contains an invalid file receipt.');
    }
    digest(entry.sha256, 'provider output file sha256');
    if (entry.kind === 'geometry') admittedBounds(entry.bounds);
    else if (entry.bounds !== undefined) {
      outputError('reference-provider-output-invalid', 'Only geometry tile receipts may carry bounds.');
    }
    paths.add(entry.path);
    const values = ordinals.get(entry.kind) ?? [];
    values.push(entry.ordinal); ordinals.set(entry.kind, values);
  }
  if (!ordinals.has('geometry') || !ordinals.has('entities')) {
    outputError('reference-provider-output-invalid', 'Provider output requires geometry and entity shards.');
  }
  for (const values of ordinals.values()) {
    values.sort((left, right) => left - right);
    if (values.some((value, index) => value !== index)) {
      outputError('reference-provider-output-invalid', 'Provider output shard ordinals must be contiguous.');
    }
  }
  return paths;
}

export async function verifyProviderOutput(root, options = {}) {
  if (typeof root !== 'string' || !path.isAbsolute(root)) {
    outputError('reference-provider-output-root-invalid', 'Provider output requires an absolute private root.');
  }
  const expected = {
    formatId: options.formatId, capabilityId: options.capabilityId,
    providerPackageManifestSha256: options.providerPackageManifestSha256,
    effectiveSourceSha256: options.effectiveSourceSha256,
    conversionRequestSha256: options.conversionRequestSha256,
  };
  const admittedRoot = options.admittedRoot;
  if (typeof expected.formatId !== 'string' || typeof expected.capabilityId !== 'string'
      || !OPAQUE_ID.test(expected.formatId) || !OPAQUE_ID.test(expected.capabilityId)) {
    outputError('reference-provider-output-request-invalid', 'Provider output verification requires format and capability IDs.');
  }
  for (const [name, value] of Object.entries(expected).slice(2)) {
    try { assertSha256(value, name); }
    catch (error) { outputError('reference-provider-output-request-invalid', 'Provider output verification requires request digests.', false, error); }
  }
  await createAdmittedRoot(root, admittedRoot);
  let succeeded = false;
  try {
  checkCancellation(options.signal);
  const limits = providerOutputLimits(options.limits);
  const directories = await outputDirectories(root);
  const completionFile = await readStableFile(root, COMPLETION_NAME, limits.completionBytes, {
    copyTo: path.join(admittedRoot, COMPLETION_NAME),
    signal: options.signal,
  }, directories);
  let completion;
  try { completion = parseJsonStrict(completionFile.bytes, { maxBytes: limits.completionBytes, maxDepth: 8 }); }
  catch (error) { outputError('reference-provider-output-invalid', 'The provider completion marker is invalid.', false, error); }
  closed(completion, ['schemaVersion', 'manifestPath', 'manifestBytes', 'manifestSha256'], [], 'provider completion marker');
  if (completion.schemaVersion !== COMPLETION_SCHEMA || completion.manifestPath !== MANIFEST_NAME
      || !Number.isSafeInteger(completion.manifestBytes) || completion.manifestBytes < 1) {
    outputError('reference-provider-output-invalid', 'The provider completion marker is unsupported.');
  }
  digest(completion.manifestSha256, 'manifestSha256');
  if (!completionFile.bytes.equals(canonicalOutputJson(completion, 'provider completion marker'))) {
    outputError('reference-provider-output-invalid', 'The provider completion marker is not canonical JSON.');
  }
  const manifestFile = await readStableFile(root, MANIFEST_NAME, limits.manifestBytes, {
    copyTo: path.join(admittedRoot, MANIFEST_NAME),
    signal: options.signal,
  }, directories);
  if (manifestFile.bytes.length !== completion.manifestBytes || manifestFile.sha256 !== completion.manifestSha256) {
    outputError('reference-provider-output-invalid', 'The provider output manifest does not match its completion marker.');
  }
  let manifest;
  try { manifest = parseJsonStrict(manifestFile.bytes, { maxBytes: limits.manifestBytes, maxDepth: 16 }); }
  catch (error) { outputError('reference-provider-output-invalid', 'The provider output manifest is invalid.', false, error); }
  if (!manifestFile.bytes.equals(canonicalOutputJson(manifest, 'provider output manifest'))) {
    outputError('reference-provider-output-invalid', 'The provider output manifest is not canonical JSON.');
  }
  const receipted = validateManifest(manifest, expected);
  const expectedFiles = [...receipted, MANIFEST_NAME, COMPLETION_NAME].sort();
  const actual = await actualFiles(directories);
  if (actual.length !== expectedFiles.length || actual.some((value, index) => value !== expectedFiles[index])) {
    outputError('reference-provider-output-invalid', 'Provider output contains missing or unreceipted files.');
  }
  let aggregateBytes = 0; const verified = [];
  for (const entry of manifest.files) {
    checkCancellation(options.signal);
    if (entry.bytes > limits.payloadBytes || entry.count > limits.recordsPerShard) {
      outputError('reference-provider-output-limit', 'A provider output shard exceeds its admitted limits.');
    }
    aggregateBytes += entry.bytes;
    if (!Number.isSafeInteger(aggregateBytes) || aggregateBytes > limits.aggregateBytes) {
      outputError('reference-provider-output-limit', 'Provider output exceeds its aggregate byte limit.');
    }
    const isJsonl = entry.mediaType === 'application/x-ndjson';
    const file = await readStableFile(root, entry.path, limits.payloadBytes, {
      collect: false, recordLimit: isJsonl ? limits.recordsPerShard : undefined,
      recordBytes: limits.recordBytes,
      copyTo: path.join(admittedRoot, ...entry.path.split('/')),
      signal: options.signal,
    }, directories);
    if (file.length !== entry.bytes || file.sha256 !== entry.sha256
        || (isJsonl && file.records !== entry.count)) {
      outputError('reference-provider-output-invalid', 'A provider output shard does not match its receipt.');
    }
    verified.push({ ...entry });
  }
  const afterActual = await actualFiles(directories);
  if (afterActual.length !== expectedFiles.length || afterActual.some((value, index) => value !== expectedFiles[index])) {
    outputError('reference-provider-output-changed', 'Provider output closure changed while it was verified.', true);
  }
  const afterManifest = await readStableFile(
    root, MANIFEST_NAME, limits.manifestBytes, { signal: options.signal }, directories,
  );
  const afterCompletion = await readStableFile(
    root, COMPLETION_NAME, limits.completionBytes, { signal: options.signal }, directories,
  );
  if (afterManifest.sha256 !== manifestFile.sha256 || afterCompletion.sha256 !== completionFile.sha256) {
    outputError('reference-provider-output-changed', 'Provider output controls changed while they were verified.', true);
  }
  const admittedDirectories = await outputDirectories(admittedRoot);
  const admittedFiles = await actualFiles(admittedDirectories);
  if (admittedFiles.length !== expectedFiles.length
      || admittedFiles.some((value, index) => value !== expectedFiles[index])) {
    outputError('reference-provider-output-invalid', 'The admitted provider output snapshot is incomplete.');
  }
  const admittedManifest = await readStableFile(
    admittedRoot, MANIFEST_NAME, limits.manifestBytes, { signal: options.signal }, admittedDirectories,
  );
  const admittedCompletion = await readStableFile(
    admittedRoot, COMPLETION_NAME, limits.completionBytes, { signal: options.signal }, admittedDirectories,
  );
  if (admittedManifest.sha256 !== manifestFile.sha256 || admittedCompletion.sha256 !== completionFile.sha256) {
    outputError('reference-provider-output-invalid', 'The admitted provider output controls failed verification.');
  }
  for (const entry of manifest.files) {
    checkCancellation(options.signal);
    const file = await readStableFile(admittedRoot, entry.path, limits.payloadBytes, {
      collect: false, recordLimit: entry.mediaType === 'application/x-ndjson' ? limits.recordsPerShard : undefined,
      recordBytes: limits.recordBytes, signal: options.signal,
    }, admittedDirectories);
    if (file.length !== entry.bytes || file.sha256 !== entry.sha256
        || (entry.mediaType === 'application/x-ndjson' && file.records !== entry.count)) {
      outputError('reference-provider-output-invalid', 'The admitted provider output snapshot failed verification.');
    }
  }
  succeeded = true;
  return {
    schemaVersion: MANIFEST_SCHEMA,
    root: admittedRoot, manifest, manifestBytes: manifestFile.bytes,
    manifestSha256: sha256(manifestFile.bytes), files: verified,
  };
  } finally {
    if (!succeeded) await fs.rm(admittedRoot, { recursive: true, force: true }).catch(() => {});
  }
}
