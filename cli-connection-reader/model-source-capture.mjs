import { createHash } from 'node:crypto';
import { constants as fsConstants } from 'node:fs';
import fs from 'node:fs/promises';
import path from 'node:path';

import { canonicalJsonBytes, ModelReaderError, sha256 } from './model-contract.mjs';

const SOURCE_CAPTURE_LIMIT_RANGES = Object.freeze({
  maxNamespaces: { default: 8, hard: 16 },
  maxFiles: { default: 25_000, hard: 100_000 },
  maxDirectories: { default: 25_000, hard: 100_000 },
  maxPathBytes: { default: 1_024, hard: 4_096 },
  maxSegmentBytes: { default: 255, hard: 255 },
  maxPathDepth: { default: 64, hard: 128 },
  maxFileBytes: { default: 2 * 1024 * 1024 * 1024, hard: 4 * 1024 * 1024 * 1024 },
  maxAggregateBytes: { default: 8 * 1024 * 1024 * 1024, hard: 16 * 1024 * 1024 * 1024 },
});

export const SOURCE_CAPTURE_LIMITS = Object.freeze(Object.fromEntries(
  Object.entries(SOURCE_CAPTURE_LIMIT_RANGES).map(([name, range]) => [name, range.default]),
));

function captureError(code, message, retryable = false, details = undefined) {
  throw new ModelReaderError(code, 'capture', retryable, message, details);
}

function samePath(left, right) {
  const normalize = (value) => process.platform === 'win32' ? path.resolve(value).toLowerCase() : path.resolve(value);
  return normalize(left) === normalize(right);
}

function isWithinRoot(root, candidate) {
  const relative = path.relative(root, candidate);
  return relative === '' || (!path.isAbsolute(relative) && relative !== '..'
    && !relative.startsWith(`..${path.sep}`));
}

function selectedLimits(overrides = {}) {
  if (!overrides || typeof overrides !== 'object' || Array.isArray(overrides)) {
    throw new TypeError('capture limits must be an object');
  }
  const result = { ...SOURCE_CAPTURE_LIMITS };
  for (const [name, value] of Object.entries(overrides)) {
    if (!Object.hasOwn(result, name)) throw new TypeError(`unknown capture limit '${name}'`);
    if (!Number.isSafeInteger(value) || value <= 0 || value > SOURCE_CAPTURE_LIMIT_RANGES[name].hard) {
      throw new TypeError(`${name} exceeds its hard ceiling`);
    }
    result[name] = value;
  }
  return result;
}

function namespaceId(value) {
  if (typeof value !== 'string' || !/^[A-Za-z0-9._-]{1,128}$/.test(value)
      || value === '.' || value === '..' || /[. ]$/.test(value)
      || /^(?:con|prn|aux|nul|com[1-9]|lpt[1-9])(?:\.|$)/i.test(value)) {
    captureError('reference-source-namespace-invalid', 'Source namespace ID is invalid.');
  }
  return value;
}

function ensureNotAborted(signal) {
  if (signal?.aborted) captureError('reference-source-cancelled', 'Source capture was cancelled.');
}

function portableRelativePath(parts, limits) {
  if (parts.length === 0 || parts.length > limits.maxPathDepth) {
    captureError('reference-source-path-invalid', 'A source path exceeds the admitted depth.');
  }
  for (const part of parts) {
    if (!part || part === '.' || part === '..' || part.includes('\0') || part.includes('/')
        || part.includes('\\') || part.includes(':') || /[. ]$/.test(part)
        || /[\u0000-\u001f\u007f]/.test(part) || part !== part.normalize('NFC')
        || /^(?:con|prn|aux|nul|com[1-9]|lpt[1-9])(?:\.|$)/i.test(part)
        || Buffer.byteLength(part, 'utf8') > limits.maxSegmentBytes) {
      captureError('reference-source-path-invalid', 'A source path cannot be represented safely.');
    }
  }
  const relativePath = parts.join('/');
  if (Buffer.byteLength(relativePath, 'utf8') > limits.maxPathBytes) {
    captureError('reference-source-path-invalid', 'A source path exceeds the admitted byte length.');
  }
  return relativePath;
}

function identity(stat) {
  return {
    dev: stat.dev.toString(), ino: stat.ino.toString(), size: Number(stat.size),
    mtimeNs: stat.mtimeNs.toString(),
  };
}

function sameIdentity(left, right) {
  return left.dev === right.dev && left.ino === right.ino && left.size === right.size
    && left.mtimeNs === right.mtimeNs;
}

function sameOpenedIdentity(inventoried, opened) {
  // Node's Windows path stat reports dev=0 while FileHandle.stat reports the
  // volume serial number. The file ID (ino) remains stable across both APIs.
  const sameDevice = process.platform === 'win32' || inventoried.dev === opened.dev;
  return sameDevice && inventoried.ino === opened.ino && inventoried.size === opened.size
    && inventoried.mtimeNs === opened.mtimeNs;
}

async function safeDirectoryRoot(root, label) {
  if (typeof root !== 'string' || !path.isAbsolute(root)) {
    captureError('reference-source-root-invalid', `${label} must be an absolute directory.`);
  }
  let stat; let real;
  try {
    stat = await fs.lstat(root, { bigint: true });
    real = await fs.realpath(root);
  } catch (error) {
    captureError('reference-source-root-unavailable', `${label} is unavailable.`, true, error);
  }
  if (!stat.isDirectory() || stat.isSymbolicLink() || !samePath(real, root)) {
    captureError('reference-source-root-unsafe', `${label} cannot be a link or reparse point.`);
  }
  return { root: path.resolve(root), identity: identity(stat) };
}

async function safeStagingParent(stagingRoot) {
  const parent = path.dirname(stagingRoot);
  let stat; let real;
  try {
    stat = await fs.lstat(parent);
    real = await fs.realpath(parent);
  } catch (error) {
    captureError('reference-source-staging-invalid', 'Source staging parent is unavailable.', false, error);
  }
  if (!stat.isDirectory() || stat.isSymbolicLink() || !samePath(real, parent)) {
    captureError('reference-source-staging-invalid', 'Source staging parent is unsafe.');
  }
}

async function inventoryNamespace(namespace, limits, totals, signal, readDirectory) {
  const files = [];
  const folded = new Set();
  async function validateDirectory(directory, expectedIdentity) {
    let stat; let real;
    try {
      stat = await fs.lstat(directory, { bigint: true });
      real = await fs.realpath(directory);
    } catch (error) {
      captureError('reference-source-changed', 'A source directory changed during inventory.', true, error);
    }
    if (!stat.isDirectory() || stat.isSymbolicLink() || !sameIdentity(identity(stat), expectedIdentity)
        || !isWithinRoot(namespace.root, real)) {
      captureError('reference-source-changed', 'A source directory changed during inventory.', true);
    }
  }
  async function walk(directory, parts, expectedIdentity) {
    ensureNotAborted(signal);
    await validateDirectory(directory, expectedIdentity);
    totals.directories += 1;
    if (totals.directories > limits.maxDirectories) {
      captureError('reference-source-package-too-large', 'The source package exceeds its admitted limits.');
    }
    let entries;
    try { entries = await readDirectory(directory, { withFileTypes: true }); }
    catch (error) { captureError('reference-source-changed', 'A source namespace changed during inventory.', true, error); }
    await validateDirectory(directory, expectedIdentity);
    entries.sort((left, right) => left.name < right.name ? -1 : left.name > right.name ? 1 : 0);
    for (const entry of entries) {
      ensureNotAborted(signal);
      const childParts = [...parts, entry.name];
      const relativePath = portableRelativePath(childParts, limits);
      const absolutePath = path.join(directory, entry.name);
      let stat;
      try { stat = await fs.lstat(absolutePath, { bigint: true }); }
      catch (error) { captureError('reference-source-changed', 'A source namespace changed during inventory.', true, error); }
      if (stat.isSymbolicLink()) captureError('reference-source-link', 'Source namespaces cannot contain links or reparse points.');
      const foldedPath = relativePath.normalize('NFC').toLowerCase();
      if (folded.has(foldedPath)) captureError('reference-source-path-collision', 'A source namespace contains case-colliding paths.');
      folded.add(foldedPath);
      if (stat.isDirectory()) {
        await walk(absolutePath, childParts, identity(stat));
        continue;
      }
      if (!stat.isFile()) captureError('reference-source-entry-invalid', 'Source namespaces may contain only regular files and directories.');
      let real;
      try { real = await fs.realpath(absolutePath); }
      catch (error) { captureError('reference-source-changed', 'A source file changed during inventory.', true, error); }
      if (!isWithinRoot(namespace.root, real)) {
        captureError('reference-source-changed', 'A source file escaped its admitted namespace.', true);
      }
      const size = Number(stat.size);
      if (!Number.isSafeInteger(size) || size > limits.maxFileBytes) {
        captureError('reference-source-file-too-large', 'A source file exceeds the admitted byte limit.');
      }
      totals.files += 1; totals.bytes += size;
      if (totals.files > limits.maxFiles || totals.bytes > limits.maxAggregateBytes) {
        captureError('reference-source-package-too-large', 'The source package exceeds its admitted limits.');
      }
      files.push({ relativePath, absolutePath, size, identity: identity(stat) });
    }
    await validateDirectory(directory, expectedIdentity);
  }
  await walk(namespace.root, [], namespace.identity);
  return files;
}

const OPEN_SOURCE_FLAGS = fsConstants.O_RDONLY | (fsConstants.O_NOFOLLOW ?? 0);
const COPY_BUFFER_BYTES = 1024 * 1024;

async function openVerifiedSource(source, openSource) {
  let handle;
  try {
    handle = await openSource(source.absolutePath, OPEN_SOURCE_FLAGS);
    const openedStat = await handle.stat({ bigint: true });
    if (!openedStat.isFile() || !sameOpenedIdentity(source.identity, identity(openedStat))) {
      captureError('reference-source-changed', 'A source file changed before it could be opened safely.', true);
    }
    return handle;
  } catch (error) {
    if (handle) await handle.close().catch(() => {});
    if (error instanceof ModelReaderError) throw error;
    captureError('reference-source-changed', 'A source file changed before it could be opened safely.', true, error);
  }
}

async function readAndHash(handle, source, limits, signal, onChunk) {
  const hash = createHash('sha256');
  const buffer = Buffer.allocUnsafe(COPY_BUFFER_BYTES);
  let bytes = 0;
  for (;;) {
    ensureNotAborted(signal);
    const { bytesRead } = await handle.read(buffer, 0, buffer.length, null);
    if (bytesRead === 0) break;
    bytes += bytesRead;
    if (bytes > limits.maxFileBytes || bytes > source.size) {
      captureError('reference-source-changed', 'A source file changed while it was read.', true);
    }
    const chunk = buffer.subarray(0, bytesRead);
    hash.update(chunk);
    if (onChunk) await onChunk(chunk);
  }
  if (bytes !== source.size) captureError('reference-source-changed', 'A source file changed while it was read.', true);
  return { bytes, sha256: hash.digest('hex') };
}

async function copyAndHash(source, destination, limits, signal, openSource) {
  await fs.mkdir(path.dirname(destination), { recursive: true, mode: 0o700 });
  const output = await fs.open(destination, 'wx', 0o600);
  let input;
  try {
    input = await openVerifiedSource(source, openSource);
    const receipt = await readAndHash(input, source, limits, signal, (chunk) => output.writeFile(chunk));
    await output.sync();
    await fs.chmod(destination, 0o400);
    return receipt;
  } catch (error) {
    if (error instanceof ModelReaderError) throw error;
    captureError('reference-source-changed', 'A source file changed while it was copied.', true, error);
  } finally {
    if (input) await input.close().catch(() => {});
    await output.close();
  }
}

async function hashOriginal(file, limits, signal, openSource) {
  let input;
  try {
    input = await openVerifiedSource(file, openSource);
    return (await readAndHash(input, file, limits, signal)).sha256;
  } catch (error) {
    if (error instanceof ModelReaderError) throw error;
    captureError('reference-source-changed', 'A source file changed during verification.', true, error);
  } finally {
    if (input) await input.close().catch(() => {});
  }
}

export async function captureSourceNamespaces(namespaces, stagingRoot, options = {}) {
  const limits = selectedLimits(options.limits);
  if (!Array.isArray(namespaces) || namespaces.length === 0 || namespaces.length > limits.maxNamespaces) {
    captureError('reference-source-namespaces-invalid', 'Source capture requires an admitted number of namespaces.');
  }
  if (typeof stagingRoot !== 'string' || !path.isAbsolute(stagingRoot)) {
    captureError('reference-source-staging-invalid', 'Source staging path must be absolute.');
  }
  await safeStagingParent(stagingRoot);
  const ids = new Set();
  const admitted = [];
  for (const value of namespaces) {
    if (!value || typeof value !== 'object' || Array.isArray(value)
        || Object.keys(value).sort().join(',') !== 'id,root') {
      captureError('reference-source-namespace-invalid', 'Each source namespace must contain only id and root.');
    }
    const id = namespaceId(value.id);
    if (ids.has(id.toLowerCase())) captureError('reference-source-namespace-invalid', 'Source namespace IDs must be unique.');
    ids.add(id.toLowerCase());
    admitted.push({ id, ...await safeDirectoryRoot(value.root, `Source namespace '${id}'`) });
  }
  const resolvedStagingRoot = path.resolve(stagingRoot);
  if (admitted.some((namespace) => isWithinRoot(namespace.root, resolvedStagingRoot)
      || isWithinRoot(resolvedStagingRoot, namespace.root))) {
    captureError('reference-source-staging-invalid', 'Source staging cannot overlap a source namespace.');
  }

  const openSource = options.openSource ?? fs.open;
  const readDirectory = options.readDirectory ?? fs.readdir;
  let ownsStaging = false;
  try {
    await fs.mkdir(stagingRoot, { recursive: false, mode: 0o700 });
    ownsStaging = true;
    const stageStat = await fs.lstat(stagingRoot);
    const stageReal = await fs.realpath(stagingRoot);
    if (!stageStat.isDirectory() || stageStat.isSymbolicLink() || !samePath(stageReal, stagingRoot)) {
      captureError('reference-source-staging-invalid', 'Source staging directory is unsafe.');
    }
    const totals = { files: 0, directories: 0, bytes: 0 };
    const first = [];
    for (const namespace of admitted) {
      first.push({ namespace, files: await inventoryNamespace(namespace, limits, totals, options.signal, readDirectory) });
    }
    const captured = [];
    for (const entry of first) {
      const targetRoot = path.join(stagingRoot, 'namespaces', entry.namespace.id);
      await fs.mkdir(targetRoot, { recursive: true, mode: 0o700 });
      const files = [];
      for (const source of entry.files) {
        const target = path.join(targetRoot, ...source.relativePath.split('/'));
        const receipt = await copyAndHash(source, target, limits, options.signal, openSource);
        files.push({ path: source.relativePath, bytes: receipt.bytes, sha256: receipt.sha256 });
      }
      captured.push({ namespaceId: entry.namespace.id, files });
    }

    const secondTotals = { files: 0, directories: 0, bytes: 0 };
    for (let index = 0; index < first.length; index += 1) {
      const prior = first[index];
      const second = await inventoryNamespace(prior.namespace, limits, secondTotals, options.signal, readDirectory);
      if (second.length !== prior.files.length) captureError('reference-source-changed', 'A source namespace changed during capture.', true);
      for (let fileIndex = 0; fileIndex < second.length; fileIndex += 1) {
        const before = prior.files[fileIndex]; const after = second[fileIndex];
        if (before.relativePath !== after.relativePath || !sameIdentity(before.identity, after.identity)) {
          captureError('reference-source-changed', 'A source namespace changed during capture.', true);
        }
        const digest = await hashOriginal(after, limits, options.signal, openSource);
        if (digest !== captured[index].files[fileIndex].sha256) {
          captureError('reference-source-changed', 'A source file changed during capture.', true);
        }
      }
    }
    if (secondTotals.files !== totals.files || secondTotals.directories !== totals.directories
        || secondTotals.bytes !== totals.bytes) {
      captureError('reference-source-changed', 'A source namespace changed during capture.', true);
    }
    const manifest = { schemaVersion: 'aware.model-source-capture/v1', namespaces: captured };
    const manifestBytes = canonicalJsonBytes(manifest);
    const manifestHandle = await fs.open(path.join(stagingRoot, 'capture.json'), 'wx', 0o400);
    try {
      await manifestHandle.writeFile(manifestBytes);
      await manifestHandle.sync();
    } finally {
      await manifestHandle.close();
    }
    return {
      stagingRoot, manifest, manifestSha256: sha256(manifestBytes),
      files: totals.files, bytes: totals.bytes,
    };
  } catch (error) {
    if (ownsStaging) await fs.rm(stagingRoot, { recursive: true, force: true });
    throw error;
  }
}

export async function verifyCapturedSource(capture, options = {}) {
  const limits = selectedLimits(options.limits);
  if (!capture || typeof capture !== 'object' || Array.isArray(capture)
      || typeof capture.stagingRoot !== 'string' || !path.isAbsolute(capture.stagingRoot)
      || !capture.manifest || typeof capture.manifest !== 'object'
      || !Array.isArray(capture.manifest.namespaces)) {
    captureError('reference-source-changed', 'The staged source capture is invalid.', true);
  }
  ensureNotAborted(options.signal);
  const expectedManifestBytes = canonicalJsonBytes(capture.manifest);
  if (sha256(expectedManifestBytes) !== capture.manifestSha256) {
    captureError('reference-source-changed', 'The staged source capture manifest identity changed.', true);
  }
  const root = await safeDirectoryRoot(capture.stagingRoot, 'Source capture staging');
  let rootEntries;
  try { rootEntries = await fs.readdir(root.root, { withFileTypes: true }); }
  catch (error) { captureError('reference-source-changed', 'The staged source capture changed.', true, error); }
  const rootNames = rootEntries.map((entry) => entry.name).sort();
  if (JSON.stringify(rootNames) !== JSON.stringify(['capture.json', 'namespaces'])) {
    captureError('reference-source-changed', 'The staged source capture contains unexpected entries.', true);
  }
  const manifestPath = path.join(root.root, 'capture.json');
  let manifestStat; let manifestBytes;
  try { manifestStat = await fs.lstat(manifestPath); manifestBytes = await fs.readFile(manifestPath); }
  catch (error) { captureError('reference-source-changed', 'The staged source capture manifest changed.', true, error); }
  if (!manifestStat.isFile() || manifestStat.isSymbolicLink() || manifestBytes.length !== manifestStat.size
      || !manifestBytes.equals(expectedManifestBytes)) {
    captureError('reference-source-changed', 'The staged source capture manifest changed.', true);
  }
  const namespaceContainer = await safeDirectoryRoot(path.join(root.root, 'namespaces'), 'Source capture namespaces');
  let namespaceEntries;
  try { namespaceEntries = await fs.readdir(namespaceContainer.root, { withFileTypes: true }); }
  catch (error) { captureError('reference-source-changed', 'The staged source namespaces changed.', true, error); }
  const expectedIds = capture.manifest.namespaces.map((entry) => entry.namespaceId).sort();
  const actualIds = namespaceEntries.map((entry) => entry.name).sort();
  if (JSON.stringify(actualIds) !== JSON.stringify(expectedIds)
      || namespaceEntries.some((entry) => !entry.isDirectory() || entry.isSymbolicLink())) {
    captureError('reference-source-changed', 'The staged source namespaces changed.', true);
  }
  const totals = { files: 0, directories: 0, bytes: 0 };
  for (const expected of capture.manifest.namespaces) {
    ensureNotAborted(options.signal);
    const namespace = await safeDirectoryRoot(
      path.join(namespaceContainer.root, expected.namespaceId),
      `Source capture namespace '${expected.namespaceId}'`,
    );
    const files = await inventoryNamespace(namespace, limits, totals, options.signal, fs.readdir);
    if (files.length !== expected.files.length) {
      captureError('reference-source-changed', 'The staged source capture membership changed.', true);
    }
    for (let index = 0; index < files.length; index += 1) {
      const actual = files[index]; const receipt = expected.files[index];
      if (actual.relativePath !== receipt.path || actual.size !== receipt.bytes
          || await hashOriginal(actual, limits, options.signal, fs.open) !== receipt.sha256) {
        captureError('reference-source-changed', 'The staged source capture bytes changed.', true);
      }
    }
  }
  return true;
}
