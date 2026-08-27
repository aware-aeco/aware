import { createHash } from 'node:crypto';
import { createReadStream } from 'node:fs';
import fs from 'node:fs/promises';
import path from 'node:path';
import {
  assertClosedObject, assertSha256, buildCanonicalRequest, buildProviderFingerprint,
  canonicalJsonBytes, lowerableLimits, ModelReaderError, parseJsonStrict, providerFingerprintSha256, sha256,
} from './model-contract.mjs';

function providerError(code, message, retryable = false, details = undefined) {
  throw new ModelReaderError(code, 'provider', retryable, message, details);
}

function samePath(left, right) {
  const normalize = (value) => process.platform === 'win32' ? path.resolve(value).toLowerCase() : path.resolve(value);
  return normalize(left) === normalize(right);
}

async function regularNonLink(filePath, label) {
  if (typeof filePath !== 'string' || !path.isAbsolute(filePath)) providerError(`reference-${label}-unsafe`, `${label} path must be absolute.`);
  let stat; let real;
  try { stat = await fs.lstat(filePath); real = await fs.realpath(filePath); }
  catch (error) { providerError(`reference-${label}-unavailable`, `${label} is unavailable.`, false, error); }
  if (stat.isSymbolicLink() || !samePath(real, filePath)) providerError(`reference-${label}-unsafe`, `${label} cannot be a link or reparse point.`);
  if (!stat.isFile()) providerError(`reference-${label}-unsafe`, `${label} must be a regular file.`);
  return stat;
}

async function fileHash(filePath, limit, label) {
  const stat = await regularNonLink(filePath, label);
  if (stat.size > limit) providerError(`reference-${label}-too-large`, `${label} exceeds its byte limit.`);
  const bytes = await fs.readFile(filePath);
  return { stat, bytes, sha256: sha256(bytes) };
}

export async function hashRegularFile(filePath, limit, label = 'source', options = {}) {
  const stat = await regularNonLink(filePath, label);
  if (stat.size > limit) providerError(`reference-${label}-too-large`, `${label} exceeds its byte limit.`);
  const hash = createHash('sha256');
  let streamedBytes = 0;
  try {
    const stream = (options.createReadStream ?? createReadStream)(filePath, { highWaterMark: 1024 * 1024 });
    for await (const chunk of stream) {
      streamedBytes += chunk.length;
      if (streamedBytes > limit) providerError(`reference-${label}-too-large`, `${label} exceeds its byte limit.`);
      hash.update(chunk);
    }
  } catch (error) {
    if (error instanceof ModelReaderError) throw error;
    providerError(`reference-${label}-unavailable`, `${label} is unavailable.`, false, error);
  }
  return { stat, sha256: hash.digest('hex') };
}

export async function validateProviderExecutable(executable, options = {}) {
  const image = await hashRegularFile(executable, Number.MAX_SAFE_INTEGER, 'provider', options);
  return { path: executable, size: image.stat.size, sha256: image.sha256 };
}

export function minimalProviderEnvironment(source = process.env, platform = process.platform) {
  const allowed = platform === 'win32' ? ['SYSTEMROOT', 'WINDIR', 'COMSPEC', 'TEMP', 'TMP'] : ['HOME', 'TMPDIR'];
  const result = {};
  if (platform === 'win32') {
    const allowedSet = new Set(allowed);
    const indexed = new Map();
    for (const [sourceKey, value] of Object.entries({ ...source })) {
      if (!/^[\x00-\x7f]+$/.test(sourceKey)) continue;
      const canonicalKey = sourceKey.replace(/[a-z]/g, (letter) => String.fromCharCode(letter.charCodeAt(0) - 32));
      if (!allowedSet.has(canonicalKey) || typeof value !== 'string' || !value) continue;
      const existing = indexed.get(canonicalKey);
      if (existing !== undefined && existing !== value) {
        providerError('reference-provider-environment-ambiguous', 'The provider environment contains conflicting Windows aliases.');
      }
      indexed.set(canonicalKey, value);
    }
    for (const key of allowed) if (indexed.has(key)) result[key] = indexed.get(key);
  } else {
    for (const key of allowed) if (typeof source[key] === 'string' && source[key]) result[key] = source[key];
  }
  result.LANG = 'C'; result.LC_ALL = 'C'; result.TZ = 'UTC';
  return result;
}

async function privateDirectory(directory) {
  await fs.mkdir(directory, { recursive: false, mode: 0o700 });
  const stat = await fs.lstat(directory);
  if (!stat.isDirectory() || stat.isSymbolicLink()) providerError('reference-output-unsafe', 'Private provider directory is unsafe.');
  return directory;
}

export async function stageImmutableSource(sourcePath, stagingRoot, expectedSourceSha256, options = {}) {
  assertSha256(expectedSourceSha256, 'expectedSourceSha256');
  const limits = lowerableLimits(options.limits);
  const before = await hashRegularFile(sourcePath, limits.maxSourceBytes, 'source', options);
  if (before.sha256 !== expectedSourceSha256) providerError('reference-source-changed', 'The model source changed before staging.');
  await privateDirectory(stagingRoot);
  const stagedPath = path.join(stagingRoot, 'source.rvt');
  const handle = await fs.open(stagedPath, 'wx', 0o600);
  const copiedHash = createHash('sha256');
  let copiedBytes = 0;
  try {
    const stream = (options.createReadStream ?? createReadStream)(sourcePath, { highWaterMark: 1024 * 1024 });
    for await (const chunk of stream) {
      copiedBytes += chunk.length;
      if (copiedBytes > limits.maxSourceBytes) providerError('reference-source-too-large', 'source exceeds its byte limit.');
      copiedHash.update(chunk);
      await handle.writeFile(chunk);
    }
    await handle.sync();
  }
  finally { await handle.close(); }
  const staged = await hashRegularFile(stagedPath, limits.maxSourceBytes, 'source');
  const after = await hashRegularFile(sourcePath, limits.maxSourceBytes, 'source', options);
  if (copiedHash.digest('hex') !== expectedSourceSha256 || staged.sha256 !== expectedSourceSha256 || after.sha256 !== expectedSourceSha256) {
    providerError('reference-source-changed', 'The model source changed while staging.');
  }
  await fs.chmod(stagedPath, 0o400);
  return { path: stagedPath, sourceSha256: staged.sha256, size: staged.stat.size };
}

function boundedString(value, label, maximum = 256) {
  if (typeof value !== 'string' || !value || Array.from(value).length > maximum) providerError('reference-provider-protocol', `Provider returned an invalid ${label}.`);
  return value;
}

function canonicalHttpsOrigin(value) {
  if (typeof value !== 'string' || value.length > 512) return null;
  try {
    const parsed = new URL(value);
    if (parsed.protocol !== 'https:' || parsed.username || parsed.password || parsed.pathname !== '/'
      || parsed.search || parsed.hash || value !== parsed.origin) return null;
    return parsed.origin;
  } catch { return null; }
}

function managedAuthorityStore(value, expectedProtocolVersion) {
  if (expectedProtocolVersion !== '2') {
    if (value !== undefined) providerError('reference-provider-protocol', 'Local providers cannot receive a managed authority store.');
    return undefined;
  }
  if (typeof value !== 'string' || !value || value.length > 4096 || value.includes('\0') || !path.isAbsolute(value)) {
    providerError('reference-provider-protocol', 'Managed providers require an absolute authority-store path.');
  }
  return path.resolve(value);
}

function validateDescribe(value, expectedProtocolVersion = '1', expectedDestination = undefined) {
  try { assertClosedObject(value, ['protocolVersion', 'provider', 'engine', 'engineVersion', 'adapterBuildId', 'formats', 'execution', 'destination'], [], 'provider description'); }
  catch (error) { providerError('reference-provider-protocol', `Provider description does not match protocol v${expectedProtocolVersion}.`, false, error); }
  if (!Array.isArray(value.formats) || value.formats.length !== 1 || value.formats[0] !== 'rvt') {
    providerError('reference-provider-protocol', 'Provider supports an invalid model format set.');
  }
  if (expectedProtocolVersion === '1') {
    if (value.protocolVersion !== '1' || value.execution !== 'local' || value.destination !== null) {
      providerError('reference-provider-protocol', 'Provider description does not match the local RVT protocol.');
    }
  } else if (expectedProtocolVersion === '2') {
    const destination = canonicalHttpsOrigin(value.destination);
    if (value.protocolVersion !== '2' || value.execution !== 'managed-cloud' || !destination
      || typeof expectedDestination !== 'string' || destination !== expectedDestination) {
      providerError('reference-provider-destination-mismatch', 'Managed-cloud provider destination does not match the exact caller pin.');
    }
  } else {
    providerError('reference-provider-protocol', 'The requested provider protocol is unsupported.');
  }
  for (const key of ['provider', 'engine', 'engineVersion']) boundedString(value[key], key, 128);
  boundedString(value.adapterBuildId, 'adapterBuildId', 256);
  return value;
}

function validateReceipt(value, describe, sourceSha256, expectedProtocolVersion, expectedDestination) {
  try { assertClosedObject(value, ['protocolVersion', 'provider', 'engine', 'engineVersion', 'adapterBuildId', 'formats', 'execution', 'destination', 'documentKind', 'sourceSha256', 'geometryPath', 'metadataPath'], [], 'provider receipt'); }
  catch (error) { providerError('reference-provider-protocol', `Provider receipt does not match protocol v${expectedProtocolVersion}.`, false, error); }
  validateDescribe(Object.fromEntries(['protocolVersion', 'provider', 'engine', 'engineVersion', 'adapterBuildId', 'formats', 'execution', 'destination'].map((key) => [key, value[key]])), expectedProtocolVersion, expectedDestination);
  for (const key of ['protocolVersion', 'provider', 'engine', 'engineVersion', 'adapterBuildId', 'execution', 'destination']) {
    if (value[key] !== describe[key]) providerError('reference-provider-changed', 'Provider provenance changed during conversion.');
  }
  if (JSON.stringify(value.formats) !== JSON.stringify(describe.formats)) providerError('reference-provider-changed', 'Provider formats changed during conversion.');
  if (value.documentKind !== 'revit-project') providerError('reference-provider-protocol', 'Provider returned the wrong document kind.');
  if (value.sourceSha256 !== sourceSha256) providerError('reference-source-changed', 'Provider did not convert the staged source.');
  assertSha256(value.sourceSha256, 'sourceSha256');
  return value;
}

function parseProviderJson(bytes, limits, label) {
  if (!Buffer.isBuffer(bytes) || bytes.length > limits.providerStdoutBytes) providerError('reference-provider-output-too-large', `Provider ${label} exceeded its output limit.`);
  try { return parseJsonStrict(bytes, { maxBytes: limits.providerStdoutBytes, maxDepth: limits.maxJsonDepth }); }
  catch (error) { providerError('reference-provider-protocol', `Provider ${label} was not valid closed JSON.`, false, error); }
}

async function callProvider(hostRun, request, limits) {
  if (typeof hostRun !== 'function') providerError('reference-provider-host-unavailable', 'The managed provider host is unavailable.');
  if (request.stdin.length > limits.providerRequestBytes) providerError('reference-provider-request-too-large', 'Provider request exceeds its byte limit.');
  let result;
  try { result = await hostRun(request); }
  catch (error) {
    if (request.signal?.aborted || (error instanceof ModelReaderError && error.code === 'reference-cancelled')) {
      providerError('reference-cancelled', 'The model provider run was cancelled.', false, error);
    }
    if (error instanceof ModelReaderError && [
      'reference-provider-executable-mismatch',
      'reference-provider-host-failed',
      'reference-provider-output-limit',
      'reference-provider-timeout',
    ].includes(error.code)) throw error;
    providerError('reference-provider-failed', 'The local model provider failed.', true, error);
  }
  if (request.signal?.aborted) providerError('reference-cancelled', 'The model provider run was cancelled.');
  if (!result || !Number.isSafeInteger(result.exitCode) || !Buffer.isBuffer(result.stdout) || !Buffer.isBuffer(result.stderr)) providerError('reference-provider-host-protocol', 'The managed provider host returned an invalid result.');
  if (result.stdout.length > limits.providerStdoutBytes || result.stderr.length > limits.providerStderrBytes) providerError('reference-provider-output-too-large', 'Provider diagnostics exceeded their byte limit.');
  if (result.exitCode !== 0) providerError('reference-provider-failed', 'The local model provider failed.', true, { exitCode: result.exitCode, stderr: result.stderr });
  return result.stdout;
}

async function validatedOutput(outputPath, expectedPath, limit, label) {
  if (typeof outputPath !== 'string' || !path.isAbsolute(outputPath) || !samePath(outputPath, expectedPath)) providerError('reference-provider-protocol', `Provider returned an invalid ${label} path.`);
  const output = await fileHash(outputPath, limit, 'output');
  return { path: outputPath, bytes: output.bytes, size: output.stat.size, sha256: output.sha256 };
}

export async function describeProvider(options) {
  const limits = lowerableLimits(options.limits);
  const initialExecutable = await validateProviderExecutable(options.executable);
  await privateDirectory(options.privateRoot);
  const cwd = await privateDirectory(path.join(options.privateRoot, 'describe'));
  const environment = minimalProviderEnvironment(options.environment);
  const expectedProtocolVersion = options.expectedProtocolVersion ?? '1';
  managedAuthorityStore(options.authorityStorePath, expectedProtocolVersion);
  const stdin = canonicalJsonBytes({ protocolVersion: expectedProtocolVersion, limits });
  const stdout = await callProvider(options.hostRun, {
    executable: initialExecutable.path, executableSha256: initialExecutable.sha256,
    operation: 'describe', stdin, stdinLength: stdin.length,
    cwd, environment, timeoutMs: limits.conversionMs,
    stdoutLimit: limits.providerStdoutBytes, stderrLimit: limits.providerStderrBytes, signal: options.signal,
  }, limits);
  const afterDescribe = await validateProviderExecutable(options.executable);
  if (afterDescribe.sha256 !== initialExecutable.sha256) providerError('reference-provider-changed', 'Provider executable changed during description.');
  const describe = validateDescribe(parseProviderJson(stdout, limits, 'description'), expectedProtocolVersion, options.expectedDestination);
  const fingerprint = buildProviderFingerprint({
    protocolVersion: describe.protocolVersion, provider: describe.provider, engine: describe.engine,
    engineVersion: describe.engineVersion, adapterBuildId: describe.adapterBuildId,
    adapterExecutableSha256: initialExecutable.sha256,
    ...(describe.protocolVersion === '2' ? { execution: describe.execution, destination: describe.destination } : {}),
  });
  if (options.expectedProviderSha256 !== undefined) {
    assertSha256(options.expectedProviderSha256, 'expectedProviderSha256');
    if (providerFingerprintSha256(fingerprint) !== options.expectedProviderSha256) providerError('reference-provider-pin-mismatch', 'The local provider does not match the expected fingerprint.');
  }
  return { describe, fingerprint, providerExecutableSha256: initialExecutable.sha256 };
}

export async function describeAndConvert(options) {
  const limits = lowerableLimits(options.limits);
  const initialExecutable = await validateProviderExecutable(options.executable);
  await privateDirectory(options.privateRoot);
  const staging = await stageImmutableSource(options.sourcePath, path.join(options.privateRoot, 'source'), options.expectedSourceSha256, { limits });
  const describeCwd = await privateDirectory(path.join(options.privateRoot, 'describe'));
  const environment = minimalProviderEnvironment(options.environment);
  const expectedProtocolVersion = options.expectedProtocolVersion ?? '1';
  const authorityStorePath = managedAuthorityStore(options.authorityStorePath, expectedProtocolVersion);
  const describeRequest = canonicalJsonBytes({ protocolVersion: expectedProtocolVersion, limits });
  const describeBytes = await callProvider(options.hostRun, {
    executable: initialExecutable.path, executableSha256: initialExecutable.sha256,
    operation: 'describe', stdin: describeRequest,
    stdinLength: describeRequest.length, cwd: describeCwd, environment,
    timeoutMs: limits.conversionMs, stdoutLimit: limits.providerStdoutBytes, stderrLimit: limits.providerStderrBytes, signal: options.signal,
  }, limits);
  const afterDescribe = await validateProviderExecutable(options.executable);
  if (afterDescribe.sha256 !== initialExecutable.sha256) providerError('reference-provider-changed', 'Provider executable changed during description.');
  const describe = validateDescribe(parseProviderJson(describeBytes, limits, 'description'), expectedProtocolVersion, options.expectedDestination);
  const describedFingerprint = buildProviderFingerprint({
    protocolVersion: describe.protocolVersion, provider: describe.provider, engine: describe.engine,
    engineVersion: describe.engineVersion, adapterBuildId: describe.adapterBuildId,
    adapterExecutableSha256: initialExecutable.sha256,
    ...(describe.protocolVersion === '2' ? { execution: describe.execution, destination: describe.destination } : {}),
  });
  if (options.expectedProviderSha256 !== undefined) {
    assertSha256(options.expectedProviderSha256, 'expectedProviderSha256');
    if (providerFingerprintSha256(describedFingerprint) !== options.expectedProviderSha256) providerError('reference-provider-pin-mismatch', 'The local provider does not match the expected fingerprint.');
  }
  const canonicalRequest = buildCanonicalRequest({
    limits,
    protocolVersion: expectedProtocolVersion,
    conversionSettings: options.conversionSettings ?? {},
  });
  const outputDirectory = await privateDirectory(path.join(options.privateRoot, 'output'));
  const beforeConvert = await validateProviderExecutable(options.executable);
  if (beforeConvert.sha256 !== initialExecutable.sha256) providerError('reference-provider-changed', 'Provider executable changed before conversion.');
  const convertRequest = canonicalJsonBytes({
    protocolVersion: expectedProtocolVersion, sourcePath: staging.path, outputDirectory,
    sourceSha256: staging.sourceSha256, canonicalRequest, limits,
    ...(authorityStorePath ? { authorityStorePath } : {}),
  });
  const receiptBytes = await callProvider(options.hostRun, {
    executable: initialExecutable.path, executableSha256: initialExecutable.sha256,
    operation: 'convert', stdin: convertRequest,
    stdinLength: convertRequest.length, cwd: outputDirectory, environment,
    timeoutMs: limits.conversionMs, stdoutLimit: limits.providerStdoutBytes, stderrLimit: limits.providerStderrBytes, signal: options.signal,
  }, limits);
  const afterConvert = await validateProviderExecutable(options.executable);
  if (afterConvert.sha256 !== initialExecutable.sha256) providerError('reference-provider-changed', 'Provider executable changed during conversion.');
  const receipt = validateReceipt(parseProviderJson(receiptBytes, limits, 'receipt'), describe, staging.sourceSha256, expectedProtocolVersion, options.expectedDestination);
  const geometryPath = path.join(outputDirectory, 'geometry.glb');
  const metadataPath = path.join(outputDirectory, 'metadata.json');
  const entries = await fs.readdir(outputDirectory);
  if (entries.length !== 2 || !entries.includes('geometry.glb') || !entries.includes('metadata.json')) providerError('reference-provider-extra-output', 'Provider output directory was not closed.');
  const geometry = await validatedOutput(receipt.geometryPath, geometryPath, limits.maxInputGlbBytes, 'geometry');
  const metadata = await validatedOutput(receipt.metadataPath, metadataPath, limits.maxMetadataBytes, 'metadata');
  if (geometry.size + metadata.size > limits.maxProviderOutputBytes) providerError('reference-provider-output-too-large', 'Provider files exceed their total byte limit.');
  return {
    describe, receipt, canonicalRequest,
    fingerprint: describedFingerprint,
    providerExecutableSha256: initialExecutable.sha256,
    stagedSource: staging, outputs: { geometry, metadata },
  };
}
