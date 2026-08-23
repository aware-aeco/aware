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
  if (!stat.isFile()) providerError(`reference-${label}-unsafe`, `${label} must be a regular file.`);
  if (stat.isSymbolicLink() || !samePath(real, filePath)) providerError(`reference-${label}-unsafe`, `${label} cannot be a link or reparse point.`);
  return stat;
}

async function fileHash(filePath, limit, label) {
  const stat = await regularNonLink(filePath, label);
  if (stat.size > limit) providerError(`reference-${label}-too-large`, `${label} exceeds its byte limit.`);
  const bytes = await fs.readFile(filePath);
  return { stat, bytes, sha256: sha256(bytes) };
}

export async function validateProviderExecutable(executable) {
  const stat = await regularNonLink(executable, 'provider');
  return { path: executable, size: stat.size, sha256: sha256(await fs.readFile(executable)) };
}

export function minimalProviderEnvironment(source = process.env, platform = process.platform) {
  const allowed = platform === 'win32' ? ['SYSTEMROOT', 'WINDIR', 'COMSPEC', 'TEMP', 'TMP'] : ['HOME', 'TMPDIR'];
  const result = {};
  for (const key of allowed) if (typeof source[key] === 'string' && source[key]) result[key] = source[key];
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
  const before = await fileHash(sourcePath, limits.maxSourceBytes, 'source');
  if (before.sha256 !== expectedSourceSha256) providerError('reference-source-changed', 'The model source changed before staging.');
  await privateDirectory(stagingRoot);
  const stagedPath = path.join(stagingRoot, 'source.rvt');
  const handle = await fs.open(stagedPath, 'wx', 0o600);
  try { await handle.writeFile(before.bytes); await handle.sync(); }
  finally { await handle.close(); }
  const staged = await fileHash(stagedPath, limits.maxSourceBytes, 'source');
  const after = await fileHash(sourcePath, limits.maxSourceBytes, 'source');
  if (staged.sha256 !== expectedSourceSha256 || after.sha256 !== expectedSourceSha256) {
    providerError('reference-source-changed', 'The model source changed while staging.');
  }
  await fs.chmod(stagedPath, 0o400);
  return { path: stagedPath, sourceSha256: staged.sha256, size: staged.stat.size };
}

function boundedString(value, label, maximum = 256) {
  if (typeof value !== 'string' || !value || Buffer.byteLength(value) > maximum) providerError('reference-provider-protocol', `Provider returned an invalid ${label}.`);
  return value;
}

function validateDescribe(value) {
  try { assertClosedObject(value, ['protocolVersion', 'provider', 'engine', 'engineVersion', 'adapterBuildId', 'formats', 'execution', 'destination'], [], 'provider description'); }
  catch (error) { providerError('reference-provider-protocol', 'Provider description does not match protocol v1.', false, error); }
  if (value.protocolVersion !== '1' || value.execution !== 'local' || value.destination !== null || !Array.isArray(value.formats) || value.formats.length !== 1 || value.formats[0] !== 'rvt') {
    providerError('reference-provider-protocol', 'Provider description does not match the local RVT protocol.');
  }
  for (const key of ['provider', 'engine', 'engineVersion', 'adapterBuildId']) boundedString(value[key], key);
  return value;
}

function validateReceipt(value, describe, sourceSha256) {
  try { assertClosedObject(value, ['protocolVersion', 'provider', 'engine', 'engineVersion', 'adapterBuildId', 'formats', 'execution', 'destination', 'documentKind', 'sourceSha256', 'geometryPath', 'metadataPath'], [], 'provider receipt'); }
  catch (error) { providerError('reference-provider-protocol', 'Provider receipt does not match protocol v1.', false, error); }
  validateDescribe(Object.fromEntries(['protocolVersion', 'provider', 'engine', 'engineVersion', 'adapterBuildId', 'formats', 'execution', 'destination'].map((key) => [key, value[key]])));
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
  catch (error) { providerError('reference-provider-failed', 'The local model provider failed.', true, error); }
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
  const stdin = canonicalJsonBytes({ protocolVersion: '1', limits });
  const stdout = await callProvider(options.hostRun, {
    executable: initialExecutable.path, operation: 'describe', stdin, stdinLength: stdin.length,
    cwd, environment, timeoutMs: limits.conversionMs,
    stdoutLimit: limits.providerStdoutBytes, stderrLimit: limits.providerStderrBytes,
  }, limits);
  const afterDescribe = await validateProviderExecutable(options.executable);
  if (afterDescribe.sha256 !== initialExecutable.sha256) providerError('reference-provider-changed', 'Provider executable changed during description.');
  const describe = validateDescribe(parseProviderJson(stdout, limits, 'description'));
  const fingerprint = buildProviderFingerprint({
    protocolVersion: describe.protocolVersion, provider: describe.provider, engine: describe.engine,
    engineVersion: describe.engineVersion, adapterBuildId: describe.adapterBuildId,
    adapterExecutableSha256: initialExecutable.sha256,
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
  const describeRequest = canonicalJsonBytes({ protocolVersion: '1', limits });
  const describeBytes = await callProvider(options.hostRun, {
    executable: initialExecutable.path, operation: 'describe', stdin: describeRequest,
    stdinLength: describeRequest.length, cwd: describeCwd, environment,
    timeoutMs: limits.conversionMs, stdoutLimit: limits.providerStdoutBytes, stderrLimit: limits.providerStderrBytes,
  }, limits);
  const afterDescribe = await validateProviderExecutable(options.executable);
  if (afterDescribe.sha256 !== initialExecutable.sha256) providerError('reference-provider-changed', 'Provider executable changed during description.');
  const describe = validateDescribe(parseProviderJson(describeBytes, limits, 'description'));
  const describedFingerprint = buildProviderFingerprint({
    protocolVersion: describe.protocolVersion, provider: describe.provider, engine: describe.engine,
    engineVersion: describe.engineVersion, adapterBuildId: describe.adapterBuildId,
    adapterExecutableSha256: initialExecutable.sha256,
  });
  if (options.expectedProviderSha256 !== undefined) {
    assertSha256(options.expectedProviderSha256, 'expectedProviderSha256');
    if (providerFingerprintSha256(describedFingerprint) !== options.expectedProviderSha256) providerError('reference-provider-pin-mismatch', 'The local provider does not match the expected fingerprint.');
  }
  const canonicalRequest = buildCanonicalRequest({ limits, conversionSettings: options.conversionSettings ?? {} });
  const outputDirectory = await privateDirectory(path.join(options.privateRoot, 'output'));
  const beforeConvert = await validateProviderExecutable(options.executable);
  if (beforeConvert.sha256 !== initialExecutable.sha256) providerError('reference-provider-changed', 'Provider executable changed before conversion.');
  const convertRequest = canonicalJsonBytes({
    protocolVersion: '1', sourcePath: staging.path, outputDirectory,
    sourceSha256: staging.sourceSha256, canonicalRequest, limits,
  });
  const receiptBytes = await callProvider(options.hostRun, {
    executable: initialExecutable.path, operation: 'convert', stdin: convertRequest,
    stdinLength: convertRequest.length, cwd: outputDirectory, environment,
    timeoutMs: limits.conversionMs, stdoutLimit: limits.providerStdoutBytes, stderrLimit: limits.providerStderrBytes,
  }, limits);
  const afterConvert = await validateProviderExecutable(options.executable);
  if (afterConvert.sha256 !== initialExecutable.sha256) providerError('reference-provider-changed', 'Provider executable changed during conversion.');
  const receipt = validateReceipt(parseProviderJson(receiptBytes, limits, 'receipt'), describe, staging.sourceSha256);
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
