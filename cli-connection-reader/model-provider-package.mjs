import { createPublicKey, verify } from 'node:crypto';
import { isUtf8 } from 'node:buffer';
import fs from 'node:fs/promises';
import path from 'node:path';

import {
  assertClosedObject, assertSha256, canonicalJsonBytes, lowerableLimits, ModelReaderError,
  parseJsonStrict, sha256,
} from './model-contract.mjs';
import { hashRegularFile, minimalProviderEnvironment, validateProviderExecutable } from './model-provider.mjs';

const PUBLIC_PREFIX = Buffer.from('302a300506032b6570032100', 'hex');
const MANIFEST_SCHEMA = 'aware.model-provider-package/v1';
const PACKAGE_SCHEMA = 'aware.model-provider-enrollment/v1';
const PUBLISHER_SCHEMA = 'aware.model-provider-publisher/v1';
const SELECTION_SCHEMA = 'aware.model-provider-selection/v1';

function packageError(code, phase, message, unsafeDetails = undefined) {
  throw new ModelReaderError(code, phase, false, message, unsafeDetails);
}

function opaqueId(value, label) {
  if (typeof value !== 'string' || !/^[A-Za-z0-9._-]{1,128}$/.test(value)) {
    packageError('reference-provider-package-invalid', 'preflight', `${label} is not an opaque identifier.`);
  }
  return value;
}

function authorization(value) {
  if (typeof value !== 'string' || !value || Buffer.byteLength(value, 'utf8') > 64 * 1024
      || /[\u0000-\u001f\u007f]/.test(value)) {
    packageError('reference-provider-authorization-invalid', 'preflight', 'Provider authorization is required and must be opaque.');
  }
  return value;
}

function normalizedRelativePath(value) {
  if (typeof value !== 'string' || value.length === 0 || value.length > 512 || value.includes('\\')
      || value.includes(':') || value.endsWith('.') || value.endsWith(' ')
      || path.posix.isAbsolute(value) || value.split('/').some((part) => !part || part === '.' || part === '..')) {
    packageError('reference-provider-package-invalid', 'preflight', 'Provider package contains an unsafe path.');
  }
  return value;
}

function semver(value, label) {
  if (typeof value !== 'string' || value.length > 128) {
    packageError('reference-provider-package-invalid', 'preflight', `${label} is not semantic versioning.`);
  }
  const match = /^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?(?:\+[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*)?$/.exec(value);
  if (!match) packageError('reference-provider-package-invalid', 'preflight', `${label} is not semantic versioning.`);
  const prerelease = match[4]?.split('.') ?? [];
  if (prerelease.some((part) => /^\d+$/.test(part) && part.length > 1 && part.startsWith('0'))) {
    packageError('reference-provider-package-invalid', 'preflight', `${label} is not semantic versioning.`);
  }
  return { core: match.slice(1, 4).map(BigInt), prerelease };
}

function compareSemver(left, right) {
  for (let index = 0; index < 3; index += 1) {
    if (left.core[index] !== right.core[index]) return left.core[index] < right.core[index] ? -1 : 1;
  }
  if (left.prerelease.length === 0 || right.prerelease.length === 0) {
    return left.prerelease.length === right.prerelease.length ? 0 : left.prerelease.length === 0 ? 1 : -1;
  }
  for (let index = 0; index < Math.max(left.prerelease.length, right.prerelease.length); index += 1) {
    const l = left.prerelease[index]; const r = right.prerelease[index];
    if (l === undefined || r === undefined) return l === undefined ? -1 : 1;
    if (l === r) continue;
    const lNumeric = /^\d+$/.test(l); const rNumeric = /^\d+$/.test(r);
    if (lNumeric && rNumeric) return BigInt(l) < BigInt(r) ? -1 : 1;
    if (lNumeric !== rNumeric) return lNumeric ? -1 : 1;
    return l < r ? -1 : 1;
  }
  return 0;
}

function verifyRuntimeCompatibility(manifest, environment) {
  const current = semver(environment?.AWARE_RUNTIME_VERSION, 'AWARE runtime version');
  const minimum = semver(manifest.minimumAwareVersion, 'minimum AWARE version');
  const maximum = manifest.maximumAwareVersion === null
    ? null : semver(manifest.maximumAwareVersion, 'maximum AWARE version');
  if (compareSemver(current, minimum) < 0 || (maximum && compareSemver(current, maximum) > 0)) {
    packageError('reference-provider-package-incompatible', 'preflight', 'Provider package is incompatible with this AWARE version.');
  }
}

async function readJson(pathname, label) {
  let bytes;
  try {
    const stat = await fs.lstat(pathname);
    if (!stat.isFile() || stat.isSymbolicLink() || stat.size > 1024 * 1024) throw new Error('control file is unsafe or too large');
    bytes = await fs.readFile(pathname);
    if (bytes.length !== stat.size || bytes.length > 1024 * 1024) throw new Error('control file changed while read');
  }
  catch (error) { packageError('reference-provider-package-unavailable', 'preflight', `${label} is unavailable.`, error); }
  try { return { bytes, value: parseJsonStrict(bytes, { maxBytes: 1024 * 1024, maxDepth: 32 }) }; }
  catch (error) { packageError('reference-provider-package-invalid', 'preflight', `${label} is invalid.`, error); }
}

function validateCapability(value) {
  assertClosedObject(value, [
    'capabilityId', 'protocolVersion', 'sourceCaptureMode', 'requestSchema', 'resultSchema',
    'artifactRootVersion', 'cacheNamespaceVersion',
  ], [], 'provider capability');
  for (const key of [
    'capabilityId', 'sourceCaptureMode', 'requestSchema', 'resultSchema',
    'artifactRootVersion', 'cacheNamespaceVersion',
  ]) opaqueId(value[key], key);
  if (value.protocolVersion !== '3') packageError('reference-provider-package-invalid', 'preflight', 'Provider capability is not protocol v3.');
  return value;
}

function validateManifest(value) {
  assertClosedObject(value, [
    'schemaVersion', 'packageId', 'packageVersion', 'formatId', 'launcher',
    'minimumAwareVersion', 'maximumAwareVersion', 'publisherFingerprintSha256',
    'capabilities', 'files',
  ], [], 'provider package manifest');
  if (value.schemaVersion !== MANIFEST_SCHEMA) packageError('reference-provider-package-invalid', 'preflight', 'Provider package manifest schema is unsupported.');
  opaqueId(value.packageId, 'packageId');
  opaqueId(value.formatId, 'formatId');
  semver(value.packageVersion, 'provider package version');
  semver(value.minimumAwareVersion, 'minimum AWARE version');
  if (value.maximumAwareVersion !== null) semver(value.maximumAwareVersion, 'maximum AWARE version');
  normalizedRelativePath(value.launcher);
  assertSha256(value.publisherFingerprintSha256, 'publisherFingerprintSha256');
  if (!Array.isArray(value.capabilities) || value.capabilities.length === 0) packageError('reference-provider-package-invalid', 'preflight', 'Provider package has no capabilities.');
  if (!Array.isArray(value.files) || value.files.length === 0) packageError('reference-provider-package-invalid', 'preflight', 'Provider package has no files.');
  const capabilities = value.capabilities.map(validateCapability);
  const capabilityIds = new Set(capabilities.map((entry) => entry.capabilityId));
  if (capabilityIds.size !== capabilities.length) packageError('reference-provider-package-invalid', 'preflight', 'Provider package repeats a capability.');
  const paths = new Set();
  for (const file of value.files) {
    assertClosedObject(file, ['path', 'bytes', 'sha256'], [], 'provider package file');
    normalizedRelativePath(file.path);
    assertSha256(file.sha256, 'package file sha256');
    if (!Number.isSafeInteger(file.bytes) || file.bytes < 0 || paths.has(file.path)) packageError('reference-provider-package-invalid', 'preflight', 'Provider package file receipt is invalid.');
    paths.add(file.path);
  }
  if (!paths.has(value.launcher)) packageError('reference-provider-package-invalid', 'preflight', 'Provider launcher is not receipted.');
  return value;
}

function validatePackageRecord(value, expectedDigest) {
  assertClosedObject(value, [
    'schemaVersion', 'manifestSha256', 'packageRoot', 'publisherFingerprintSha256',
    'manifest', 'enrolled', 'revoked',
  ], [], 'provider enrollment');
  if (value.schemaVersion !== PACKAGE_SCHEMA || value.manifestSha256 !== expectedDigest
      || value.enrolled !== true || value.revoked !== false || !path.isAbsolute(value.packageRoot)) {
    packageError('reference-provider-package-invalid', 'preflight', 'Provider enrollment is not active and closed.');
  }
  assertSha256(value.publisherFingerprintSha256, 'publisher fingerprint');
  validateManifest(value.manifest);
  if (value.manifest.publisherFingerprintSha256 !== value.publisherFingerprintSha256) packageError('reference-provider-package-invalid', 'preflight', 'Provider enrollment publisher does not match its manifest.');
  return value;
}

function parseSignature(text, manifestDigest, publicKeyBase64) {
  const lines = text.split(/\r?\n/).filter(Boolean);
  if (lines.shift() !== 'ed25519-signature-v1') packageError('reference-provider-package-invalid', 'preflight', 'Provider package signature schema is unsupported.');
  const fields = new Map();
  for (const line of lines) {
    const colon = line.indexOf(':');
    if (colon < 1) packageError('reference-provider-package-invalid', 'preflight', 'Provider package signature is malformed.');
    const key = line.slice(0, colon).trim(); const value = line.slice(colon + 1).trim();
    if (fields.has(key)) packageError('reference-provider-package-invalid', 'preflight', 'Provider package signature repeats a field.');
    fields.set(key, value);
  }
  const keys = [...fields.keys()].sort();
  if (JSON.stringify(keys) !== JSON.stringify(['over-sha256-of', 'public-key', 'sha256', 'signature'])
      || fields.get('over-sha256-of') !== 'provider-package.json'
      || fields.get('sha256') !== manifestDigest || fields.get('public-key') !== publicKeyBase64) {
    packageError('reference-provider-package-invalid', 'preflight', 'Provider package signature does not bind its enrolled manifest.');
  }
  return fields.get('signature');
}

async function inventory(root, directory = root, found = new Map()) {
  let entries;
  try { entries = await fs.readdir(directory, { withFileTypes: true }); }
  catch (error) { packageError('reference-provider-package-unavailable', 'preflight', 'Provider package inventory is unavailable.', error); }
  for (const entry of entries) {
    const pathname = path.join(directory, entry.name);
    let stat;
    try { stat = await fs.lstat(pathname); }
    catch (error) { packageError('reference-provider-package-changed', 'preflight', 'Provider package changed while it was inspected.', error); }
    if (stat.isSymbolicLink()) packageError('reference-provider-package-invalid', 'preflight', 'Provider package contains a link.');
    if (stat.isDirectory()) await inventory(root, pathname, found);
    else if (stat.isFile()) found.set(path.relative(root, pathname).split(path.sep).join('/'), stat.size);
    else packageError('reference-provider-package-invalid', 'preflight', 'Provider package contains a non-file entry.');
  }
  return found;
}

async function verifyPackageFiles(root, manifest) {
  const expected = new Map(manifest.files.map((file) => [file.path, file]));
  expected.set('provider-package.json', null);
  expected.set('provider-package.sig', null);
  const actual = await inventory(root);
  if (JSON.stringify([...actual.keys()].sort()) !== JSON.stringify([...expected.keys()].sort())) packageError('reference-provider-package-changed', 'preflight', 'Provider package no longer matches its closed allowlist.');
  for (const file of manifest.files) {
    let receipt;
    try { receipt = await hashRegularFile(path.join(root, ...file.path.split('/')), Number.MAX_SAFE_INTEGER, 'provider-package-file'); }
    catch (error) { packageError('reference-provider-package-changed', 'preflight', 'Provider package file is unavailable.', error); }
    if (receipt.stat.size !== file.bytes || receipt.sha256 !== file.sha256) packageError('reference-provider-package-changed', 'preflight', 'Provider package file no longer matches its receipt.');
  }
}

async function loadEnrolledProviderPackageInternal({ home, formatId, capabilityId, manifestSha256, environment }) {
  opaqueId(formatId, 'provider-format');
  opaqueId(capabilityId, 'provider-capability');
  assertSha256(manifestSha256, 'provider-package-sha256');
  const selection = (await readJson(path.join(home, 'providers', 'selections', `${formatId}.json`), 'Provider selection')).value;
  assertClosedObject(selection, ['schemaVersion', 'formatId', 'generation', 'activeManifestSha256', 'previousManifestSha256'], [], 'provider selection');
  const history = selection.previousManifestSha256;
  const validHistory = Array.isArray(history) && history.length <= 8
    && history.every((digest) => typeof digest === 'string' && /^[a-f0-9]{64}$/.test(digest))
    && new Set(history).size === history.length && !history.includes(selection.activeManifestSha256);
  if (selection.schemaVersion !== SELECTION_SCHEMA || selection.formatId !== formatId
      || selection.activeManifestSha256 !== manifestSha256 || !Number.isSafeInteger(selection.generation)
      || selection.generation < 1 || !validHistory) {
    packageError('reference-provider-package-pin-mismatch', 'preflight', 'Selected provider package does not match the expected manifest.');
  }
  const packagePath = path.join(home, 'providers', 'packages', `${manifestSha256}.json`);
  const packageRecord = validatePackageRecord((await readJson(packagePath, 'Provider enrollment')).value, manifestSha256);
  verifyRuntimeCompatibility(packageRecord.manifest, environment);
  if (packageRecord.manifest.formatId !== formatId) packageError('reference-provider-package-pin-mismatch', 'preflight', 'Provider package format does not match the request.');
  const capability = packageRecord.manifest.capabilities.find((entry) => entry.capabilityId === capabilityId);
  if (!capability) packageError('reference-provider-capability-unavailable', 'preflight', 'Provider package does not expose the requested capability.');
  const publisherPath = path.join(home, 'providers', 'publishers', `${packageRecord.publisherFingerprintSha256}.json`);
  const publisher = (await readJson(publisherPath, 'Provider publisher')).value;
  assertClosedObject(publisher, ['schemaVersion', 'publisherId', 'keyFingerprintSha256', 'publicKeyBase64', 'trusted'], [], 'provider publisher');
  const publicBytes = Buffer.from(publisher.publicKeyBase64, 'base64');
  if (publisher.schemaVersion !== PUBLISHER_SCHEMA || publisher.trusted !== true
      || publisher.keyFingerprintSha256 !== packageRecord.publisherFingerprintSha256
      || publicBytes.length !== 32 || sha256(publicBytes) !== publisher.keyFingerprintSha256) {
    packageError('reference-provider-publisher-untrusted', 'preflight', 'Provider package publisher is not trusted.');
  }
  const root = packageRecord.packageRoot;
  let rootStat; let rootReal;
  try { rootStat = await fs.lstat(root); rootReal = await fs.realpath(root); }
  catch (error) { packageError('reference-provider-package-unavailable', 'preflight', 'Provider package root is unavailable.', error); }
  const sameRoot = process.platform === 'win32'
    ? path.resolve(rootReal).toLowerCase() === path.resolve(root).toLowerCase()
    : path.resolve(rootReal) === path.resolve(root);
  if (!rootStat.isDirectory() || rootStat.isSymbolicLink() || !sameRoot) packageError('reference-provider-package-changed', 'preflight', 'Provider package root is no longer a regular enrolled directory.');
  const manifestPath = path.join(root, 'provider-package.json');
  const manifestStat = await fs.lstat(manifestPath);
  if (!manifestStat.isFile() || manifestStat.isSymbolicLink() || manifestStat.size > 1024 * 1024) packageError('reference-provider-package-invalid', 'preflight', 'Provider package manifest is unsafe or too large.');
  const manifestBytes = await fs.readFile(manifestPath);
  if (manifestBytes.length !== manifestStat.size) packageError('reference-provider-package-changed', 'preflight', 'Provider package manifest changed while it was read.');
  if (sha256(manifestBytes) !== manifestSha256) packageError('reference-provider-package-changed', 'preflight', 'Provider package manifest changed after enrollment.');
  const diskManifest = validateManifest(parseJsonStrict(manifestBytes, { maxBytes: 1024 * 1024, maxDepth: 32 }));
  if (!manifestBytes.equals(canonicalJsonBytes(diskManifest))) packageError('reference-provider-package-invalid', 'preflight', 'Provider package manifest is not canonical JSON.');
  if (!canonicalJsonBytes(diskManifest).equals(canonicalJsonBytes(packageRecord.manifest))) packageError('reference-provider-package-changed', 'preflight', 'Provider package manifest differs from its enrollment.');
  const signaturePath = path.join(root, 'provider-package.sig');
  const signatureStat = await fs.lstat(signaturePath);
  if (!signatureStat.isFile() || signatureStat.isSymbolicLink() || signatureStat.size > 64 * 1024) packageError('reference-provider-package-invalid', 'preflight', 'Provider package signature is unsafe or too large.');
  const signatureBytes = await fs.readFile(signaturePath);
  if (signatureBytes.length !== signatureStat.size) packageError('reference-provider-package-changed', 'preflight', 'Provider package signature changed while it was read.');
  if (!isUtf8(signatureBytes)) packageError('reference-provider-package-invalid', 'preflight', 'Provider package signature is not UTF-8.');
  const signatureText = signatureBytes.toString('utf8');
  const signature = Buffer.from(parseSignature(signatureText, manifestSha256, publisher.publicKeyBase64), 'base64');
  const publicKey = createPublicKey({ key: Buffer.concat([PUBLIC_PREFIX, publicBytes]), format: 'der', type: 'spki' });
  if (signature.length !== 64 || !verify(null, Buffer.from(manifestSha256, 'hex'), publicKey, signature)) packageError('reference-provider-package-signature-invalid', 'preflight', 'Provider package signature does not verify.');
  await verifyPackageFiles(root, diskManifest);
  const launcher = path.join(root, ...diskManifest.launcher.split('/'));
  const executable = await validateProviderExecutable(launcher);
  return { packageRecord, capability, publisher, executable };
}

export async function loadEnrolledProviderPackage(options) {
  try { return await loadEnrolledProviderPackageInternal(options); }
  catch (error) {
    if (error instanceof ModelReaderError) throw error;
    packageError('reference-provider-package-invalid', 'preflight', 'Provider enrollment is invalid.', error);
  }
}

export async function preflightEnrolledProviderPackage(options) {
  const limits = lowerableLimits(options.limits);
  const before = await loadEnrolledProviderPackage(options);
  const request = canonicalJsonBytes({
    operation: 'describe', protocolVersion: '3', formatId: options.formatId,
    capabilityId: options.capabilityId, packageManifestSha256: options.manifestSha256,
    authorization: authorization(options.authorization),
  });
  let result;
  try {
    result = await options.hostRun({
      executable: before.executable.path, executableSha256: before.executable.sha256,
      operation: 'describe', stdin: request, stdinLength: request.length,
      cwd: before.packageRecord.packageRoot,
      environment: minimalProviderEnvironment(options.environment), timeoutMs: limits.conversionMs,
      stdoutLimit: limits.providerStdoutBytes, stderrLimit: limits.providerStderrBytes, signal: options.signal,
    });
  } catch (error) { packageError('reference-provider-failed', 'preflight', 'The enrolled provider failed during description.', error); }
  if (!result || result.exitCode !== 0 || !Buffer.isBuffer(result.stdout) || !Buffer.isBuffer(result.stderr)
      || result.stdout.length > limits.providerStdoutBytes || result.stderr.length > limits.providerStderrBytes) {
    packageError('reference-provider-failed', 'preflight', 'The enrolled provider failed during description.', result);
  }
  let description;
  try { description = parseJsonStrict(result.stdout, { maxBytes: limits.providerStdoutBytes, maxDepth: limits.maxJsonDepth }); }
  catch (error) { packageError('reference-provider-protocol', 'preflight', 'Provider description is not valid closed JSON.', error); }
  try { assertClosedObject(description, ['protocolVersion', 'capabilityId', 'ready'], [], 'provider description'); }
  catch (error) { packageError('reference-provider-protocol', 'preflight', 'Provider description is not valid closed JSON.', error); }
  if (description.protocolVersion !== '3' || description.capabilityId !== options.capabilityId || description.ready !== true) {
    packageError('reference-provider-protocol', 'preflight', 'Provider description does not match the enrolled capability.');
  }
  const after = await loadEnrolledProviderPackage(options);
  if (before.executable.sha256 !== after.executable.sha256) packageError('reference-provider-package-changed', 'preflight', 'Provider package changed during description.');
  return {
    schemaVersion: 'model-reference-reader-provider-package/v1', ready: true,
    execution: 'enrolled-local', providerPackageManifestSha256: options.manifestSha256,
    package: {
      packageId: after.packageRecord.manifest.packageId,
      packageVersion: after.packageRecord.manifest.packageVersion,
      formatId: after.packageRecord.manifest.formatId,
      publisherFingerprintSha256: after.packageRecord.publisherFingerprintSha256,
    },
    capability: after.capability,
  };
}
