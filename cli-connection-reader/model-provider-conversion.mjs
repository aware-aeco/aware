import fs from 'node:fs/promises';
import { constants as fsConstants } from 'node:fs';
import path from 'node:path';

import {
  assertClosedObject, assertSha256, canonicalJsonBytes, lowerableLimits, ModelReaderError,
  parseJsonStrict, sha256,
} from './model-contract.mjs';
import {
  discoverCapturedSource, loadAdmittedDependencyPolicy, packageProviderIdentity,
} from './model-provider-discovery.mjs';
import { validateDependencyPolicy } from './model-effective-source.mjs';
import { providerOutputLimits, verifyProviderOutput } from './model-provider-output.mjs';
import { loadEnrolledProviderPackage } from './model-provider-package.mjs';
import { hashRegularFile, minimalProviderEnvironment } from './model-provider.mjs';
import { captureSourceNamespaces, verifyCapturedSource } from './model-source-capture.mjs';

const REQUEST_SCHEMA = 'model-reference-conversion-request/v3';
const RESPONSE_SCHEMA = 'aware.model-provider-conversion-response/v1';
const OPAQUE_ID = /^[A-Za-z0-9._-]{1,128}$/;

function conversionError(code, message, retryable = false, details = undefined) {
  throw new ModelReaderError(code, 'conversion', retryable, message, details);
}

function authorization(value) {
  if (typeof value !== 'string' || !value || Buffer.byteLength(value, 'utf8') > 64 * 1024
      || /[\u0000-\u001f\u007f]/.test(value)) {
    conversionError('reference-provider-authorization-invalid', 'Provider authorization is invalid.');
  }
  return value;
}

function opaque(value, label) {
  if (typeof value !== 'string' || !OPAQUE_ID.test(value)) {
    conversionError('reference-provider-package-request-invalid', `${label} is invalid.`);
  }
  return value;
}

function checkCancellation(signal) {
  if (signal?.aborted) conversionError('reference-cancelled', 'Provider conversion was cancelled.');
}

async function stageConsumedClosure(capture, discovered, closureRoot, signal) {
  await fs.mkdir(closureRoot, { mode: 0o700 });
  await fs.mkdir(path.join(closureRoot, 'namespaces'), { mode: 0o700 });
  const grouped = new Map();
  for (const receipt of discovered.effectiveSource.consumed) {
    checkCancellation(signal);
    const files = grouped.get(receipt.namespaceId) ?? [];
    files.push({ path: receipt.path, bytes: receipt.bytes, sha256: receipt.sha256 });
    grouped.set(receipt.namespaceId, files);
    const source = path.join(capture.stagingRoot, 'namespaces', receipt.namespaceId, ...receipt.path.split('/'));
    const target = path.join(closureRoot, 'namespaces', receipt.namespaceId, ...receipt.path.split('/'));
    await fs.mkdir(path.dirname(target), { recursive: true, mode: 0o700 });
    await fs.copyFile(source, target, fsConstants.COPYFILE_EXCL);
    await fs.chmod(target, 0o400);
    const copied = await hashRegularFile(target, receipt.bytes, 'consumed source closure');
    if (Number(copied.stat.size) !== receipt.bytes || copied.sha256 !== receipt.sha256) {
      conversionError('reference-source-changed', 'A consumed source file changed while staging conversion.', true);
    }
  }
  const namespaces = [...grouped.entries()]
    .sort(([left], [right]) => left.localeCompare(right, 'en'))
    .map(([namespaceId, files]) => ({
      namespaceId,
      files: files.sort((left, right) => left.path.localeCompare(right.path, 'en')),
    }));
  const manifest = { schemaVersion: 'aware.model-source-capture/v1', namespaces };
  const manifestBytes = canonicalJsonBytes(manifest);
  await fs.writeFile(path.join(closureRoot, 'capture.json'), manifestBytes, { flag: 'wx', mode: 0o400 });
  return { stagingRoot: closureRoot, manifest, manifestSha256: sha256(manifestBytes) };
}

export function buildProviderConversionRequest(options) {
  const formatId = opaque(options?.formatId, 'provider-format');
  const capabilityId = opaque(options?.capabilityId, 'provider-capability');
  let providerPackageManifestSha256; let effectiveSourceSha256;
  try {
    providerPackageManifestSha256 = assertSha256(
      options.providerPackageManifestSha256, 'providerPackageManifestSha256',
    );
    effectiveSourceSha256 = assertSha256(options.effectiveSourceSha256, 'effectiveSourceSha256');
  } catch (error) {
    conversionError('reference-provider-package-request-invalid', 'Provider conversion digests are invalid.', false, error);
  }
  const degradedMode = options.degradedMode ?? 'refuse';
  if (!['refuse', 'allow'].includes(degradedMode)) {
    conversionError('reference-request-invalid', 'degraded-mode must be refuse or allow.');
  }
  const conversionSettings = options.conversionSettings ?? {};
  if (!conversionSettings || typeof conversionSettings !== 'object' || Array.isArray(conversionSettings)) {
    conversionError('reference-request-invalid', 'Conversion settings must be an object.');
  }
  let limits; let outputLimits; let bytes;
  try {
    limits = lowerableLimits(options.limits);
    outputLimits = providerOutputLimits(options.outputLimits);
    bytes = canonicalJsonBytes({
      schemaVersion: REQUEST_SCHEMA, protocolVersion: '3', formatId, capabilityId,
      providerPackageManifestSha256, effectiveSourceSha256, degradedMode,
      conversionSettings, limits, outputLimits,
    });
  } catch (error) {
    if (error instanceof ModelReaderError) throw error;
    conversionError('reference-request-invalid', 'The provider conversion request is invalid.', false, error);
  }
  return { request: parseJsonStrict(bytes), bytes, sha256: sha256(bytes), limits, outputLimits };
}

function validateExpectedSource(options) {
  let bytes;
  try { bytes = canonicalJsonBytes(options.effectiveSource); }
  catch (error) { conversionError('reference-effective-source-invalid', 'The expected effective source is invalid.', false, error); }
  let expectedSha256;
  try { expectedSha256 = assertSha256(options.effectiveSourceSha256, 'effectiveSourceSha256'); }
  catch (error) { conversionError('reference-effective-source-invalid', 'The expected effective-source digest is invalid.', false, error); }
  if (sha256(bytes) !== expectedSha256) {
    conversionError('reference-effective-source-invalid', 'The expected effective source does not match its digest.');
  }
  return { bytes, sha256: expectedSha256 };
}

async function invokeConvert(options, capture, closure, discovered, identity, runRoot, ownership, deps) {
  const loadOptions = {
    home: options.home, formatId: options.formatId, capabilityId: options.capabilityId,
    manifestSha256: options.manifestSha256, environment: options.environment,
  };
  const before = await (deps.loadPackage ?? loadEnrolledProviderPackage)(loadOptions);
  const provider = packageProviderIdentity(before, options.manifestSha256);
  if (provider.sha256 !== discovered.effectiveSource.providerFingerprintSha256) {
    conversionError('reference-provider-package-changed', 'Provider identity changed before conversion.');
  }
  const admittedPolicy = await (deps.loadPolicy ?? loadAdmittedDependencyPolicy)(options.home, provider.sha256);
  const validatedPolicy = validateDependencyPolicy(admittedPolicy.policy, options.capabilityId, provider.sha256);
  if (admittedPolicy.sha256 !== validatedPolicy.sha256
      || admittedPolicy.sha256 !== discovered.effectiveSource.discoveryPolicy.sha256) {
    conversionError('reference-dependency-policy-changed', 'The admitted dependency policy changed before conversion.');
  }
  const outputRoot = path.join(runRoot, 'provider-output');
  const admittedRoot = path.join(runRoot, 'admitted-output');
  const sourceManifestPath = path.join(runRoot, 'effective-source.json');
  await fs.writeFile(sourceManifestPath, discovered.bytes, { flag: 'wx', mode: 0o400 });
  ownership.effectiveSource = true;
  await fs.mkdir(outputRoot, { mode: 0o700 });
  ownership.providerOutput = true;
  const control = canonicalJsonBytes({
    operation: 'convert', protocolVersion: '3', formatId: options.formatId,
    capabilityId: options.capabilityId, packageManifestSha256: options.manifestSha256,
    authorization: authorization(options.authorization),
    capture: {
      schemaVersion: closure.manifest.schemaVersion, manifestSha256: closure.manifestSha256,
      root: closure.stagingRoot, manifestPath: 'capture.json', namespaceRoot: 'namespaces',
    },
    effectiveSource: {
      schemaVersion: discovered.effectiveSource.schemaVersion, sha256: discovered.sha256,
      path: sourceManifestPath,
    },
    conversionRequest: identity.request,
    output: { root: outputRoot, manifestPath: 'intermediate-manifest.json', completionPath: 'complete.json' },
  });
  if (control.length > identity.limits.providerRequestBytes) {
    conversionError('reference-provider-request-too-large', 'Provider conversion request exceeds its byte limit.');
  }
  let result;
  try {
    result = await options.hostRun({
      executable: before.executable.path, executableSha256: before.executable.sha256,
      operation: 'convert', stdin: control, stdinLength: control.length,
      cwd: before.packageRecord.packageRoot,
      environment: minimalProviderEnvironment(options.environment), timeoutMs: identity.limits.conversionMs,
      stdoutLimit: identity.limits.providerStdoutBytes, stderrLimit: identity.limits.providerStderrBytes,
      signal: options.signal,
    });
  } catch (error) {
    if (options.signal?.aborted || error?.code === 'reference-cancelled') {
      conversionError('reference-cancelled', 'Provider conversion was cancelled.', false, error);
    }
    if (error instanceof ModelReaderError) throw error;
    conversionError('reference-provider-failed', 'The enrolled provider failed during conversion.', true, error);
  }
  checkCancellation(options.signal);
  if (!result || result.exitCode !== 0 || !Buffer.isBuffer(result.stdout) || !Buffer.isBuffer(result.stderr)
      || result.stdout.length > identity.limits.providerStdoutBytes
      || result.stderr.length > identity.limits.providerStderrBytes) {
    conversionError('reference-provider-failed', 'The enrolled provider failed during conversion.', true, result);
  }
  let response;
  try {
    response = parseJsonStrict(result.stdout, {
      maxBytes: identity.limits.providerStdoutBytes, maxDepth: identity.limits.maxJsonDepth,
    });
    assertClosedObject(response, ['schemaVersion', 'protocolVersion', 'capabilityId', 'complete'], [],
      'provider conversion response');
  } catch (error) {
    conversionError('reference-provider-protocol', 'Provider conversion did not return valid closed JSON.', false, error);
  }
  if (response.schemaVersion !== RESPONSE_SCHEMA || response.protocolVersion !== '3'
      || response.capabilityId !== options.capabilityId || response.complete !== true
      || !result.stdout.equals(canonicalJsonBytes(response))) {
    conversionError('reference-provider-protocol', 'Provider conversion response does not match the request.');
  }
  await (deps.verifyCapture ?? verifyCapturedSource)(capture, {
    limits: options.captureLimits, signal: options.signal,
  });
  await (deps.verifyCapture ?? verifyCapturedSource)(closure, {
    limits: options.captureLimits, signal: options.signal,
  });
  const after = await (deps.loadPackage ?? loadEnrolledProviderPackage)(loadOptions);
  if (packageProviderIdentity(after, options.manifestSha256).sha256 !== provider.sha256) {
    conversionError('reference-provider-package-changed', 'Provider identity changed during conversion.');
  }
  const afterPolicy = await (deps.loadPolicy ?? loadAdmittedDependencyPolicy)(options.home, provider.sha256);
  if (afterPolicy.sha256 !== admittedPolicy.sha256) {
    conversionError('reference-dependency-policy-changed', 'The admitted dependency policy changed during conversion.');
  }
  return await (deps.verifyOutput ?? verifyProviderOutput)(outputRoot, {
    admittedRoot, formatId: options.formatId, capabilityId: options.capabilityId,
    providerPackageManifestSha256: options.manifestSha256,
    effectiveSourceSha256: discovered.sha256, conversionRequestSha256: identity.sha256,
    limits: identity.outputLimits, signal: options.signal,
  });
}

export async function convertProviderSource(options, deps = {}) {
  if (!options || typeof options !== 'object' || Array.isArray(options)
      || typeof options.hostRun !== 'function' || typeof options.stagingRoot !== 'string'
      || !path.isAbsolute(options.stagingRoot)) {
    conversionError('reference-provider-host-unavailable', 'Provider conversion requires a managed host and private staging root.');
  }
  const expected = validateExpectedSource(options);
  const identity = buildProviderConversionRequest({
    formatId: options.formatId, capabilityId: options.capabilityId,
    providerPackageManifestSha256: options.manifestSha256,
    effectiveSourceSha256: expected.sha256, degradedMode: options.degradedMode,
    conversionSettings: options.conversionSettings, limits: options.limits,
    outputLimits: options.outputLimits,
  });
  let capture; let closure;
  const ownership = { source: false, closure: false, providerOutput: false, effectiveSource: false };
  try {
    checkCancellation(options.signal);
    capture = await (deps.capture ?? captureSourceNamespaces)(options.namespaces, path.join(options.stagingRoot, 'source'), {
      limits: options.captureLimits, signal: options.signal,
    });
    ownership.source = true;
    const discovered = await (deps.discover ?? discoverCapturedSource)(options, capture);
    if (discovered.sha256 !== expected.sha256 || !discovered.bytes.equals(expected.bytes)) {
      conversionError('reference-effective-source-changed', 'The effective model source changed before conversion.', true);
    }
    await (deps.verifyCapture ?? verifyCapturedSource)(capture, {
      limits: options.captureLimits, signal: options.signal,
    });
    closure = await (deps.stageClosure ?? stageConsumedClosure)(
      capture, discovered, path.join(options.stagingRoot, 'closure'), options.signal,
    );
    ownership.closure = true;
    await (deps.verifyCapture ?? verifyCapturedSource)(capture, {
      limits: options.captureLimits, signal: options.signal,
    });
    const output = await invokeConvert(
      options, capture, closure, discovered, identity, options.stagingRoot, ownership, deps,
    );
    return { output, conversionRequest: identity.request, conversionRequestSha256: identity.sha256 };
  } finally {
    if (ownership.source) await fs.rm(capture.stagingRoot, { recursive: true, force: true }).catch(() => {});
    if (ownership.closure) await fs.rm(closure.stagingRoot, { recursive: true, force: true }).catch(() => {});
    if (ownership.providerOutput) await fs.rm(path.join(options.stagingRoot, 'provider-output'), { recursive: true, force: true }).catch(() => {});
    if (ownership.effectiveSource) await fs.rm(path.join(options.stagingRoot, 'effective-source.json'), { force: true }).catch(() => {});
  }
}
