import fs from 'node:fs/promises';

import {
  canonicalJsonBytes, lowerableLimits, ModelReaderError, parseJsonStrict, sha256,
} from './model-contract.mjs';
import { buildEffectiveSource } from './model-effective-source.mjs';
import { loadEnrolledProviderPackage } from './model-provider-package.mjs';
import { minimalProviderEnvironment } from './model-provider.mjs';
import { captureSourceNamespaces, verifyCapturedSource } from './model-source-capture.mjs';

function discoveryError(code, message, retryable = false, details = undefined) {
  throw new ModelReaderError(code, 'discovery', retryable, message, details);
}

export function packageProviderIdentity(loaded, manifestSha256) {
  const identity = {
    schemaVersion: 'aware.enrolled-model-provider-fingerprint/v1',
    execution: 'enrolled-local',
    packageManifestSha256: manifestSha256,
    launcherSha256: loaded.executable.sha256,
    publisherFingerprintSha256: loaded.packageRecord.publisherFingerprintSha256,
    formatId: loaded.packageRecord.manifest.formatId,
    capability: loaded.capability,
  };
  return { identity, sha256: sha256(canonicalJsonBytes(identity)) };
}

function authorization(value) {
  if (typeof value !== 'string' || !value || Buffer.byteLength(value, 'utf8') > 64 * 1024
      || /[\u0000-\u001f\u007f]/.test(value)) {
    discoveryError('reference-provider-authorization-invalid', 'Provider authorization is invalid.');
  }
  return value;
}

async function invokeDiscover(options, capture) {
  const limits = lowerableLimits(options.limits);
  const loadOptions = {
    home: options.home, formatId: options.formatId, capabilityId: options.capabilityId,
    manifestSha256: options.manifestSha256, environment: options.environment,
  };
  const before = await loadEnrolledProviderPackage(loadOptions);
  const provider = packageProviderIdentity(before, options.manifestSha256);
  const request = canonicalJsonBytes({
    operation: 'discover', protocolVersion: '3', formatId: options.formatId,
    capabilityId: options.capabilityId, packageManifestSha256: options.manifestSha256,
    authorization: authorization(options.authorization),
    capture: {
      schemaVersion: capture.manifest.schemaVersion,
      manifestSha256: capture.manifestSha256,
      root: capture.stagingRoot,
      manifestPath: 'capture.json',
      namespaceRoot: 'namespaces',
    },
  });
  if (request.length > limits.providerRequestBytes) {
    discoveryError('reference-provider-request-too-large', 'Provider discovery request exceeds its byte limit.');
  }
  let result;
  try {
    result = await options.hostRun({
      executable: before.executable.path, executableSha256: before.executable.sha256,
      operation: 'discover', stdin: request, stdinLength: request.length,
      cwd: before.packageRecord.packageRoot,
      environment: minimalProviderEnvironment(options.environment), timeoutMs: limits.conversionMs,
      stdoutLimit: limits.providerStdoutBytes, stderrLimit: limits.providerStderrBytes,
      signal: options.signal,
    });
  } catch (error) {
    if (options.signal?.aborted || (error instanceof ModelReaderError && error.code === 'reference-cancelled')) {
      discoveryError('reference-cancelled', 'Provider dependency discovery was cancelled.', false, error);
    }
    if (error instanceof ModelReaderError && [
      'reference-provider-executable-mismatch', 'reference-provider-host-failed',
      'reference-provider-output-limit', 'reference-provider-timeout',
    ].includes(error.code)) throw error;
    discoveryError('reference-provider-failed', 'The enrolled provider failed during dependency discovery.', true, error);
  }
  if (options.signal?.aborted) discoveryError('reference-cancelled', 'Provider dependency discovery was cancelled.');
  if (!result || result.exitCode !== 0 || !Buffer.isBuffer(result.stdout) || !Buffer.isBuffer(result.stderr)
      || result.stdout.length > limits.providerStdoutBytes || result.stderr.length > limits.providerStderrBytes) {
    discoveryError('reference-provider-failed', 'The enrolled provider failed during dependency discovery.', true, result);
  }
  await verifyCapturedSource(capture, { limits: options.captureLimits, signal: options.signal });
  let dependencyReport;
  try {
    dependencyReport = parseJsonStrict(result.stdout, {
      maxBytes: limits.providerStdoutBytes, maxDepth: limits.maxJsonDepth,
    });
  } catch (error) {
    discoveryError('reference-provider-protocol', 'Provider dependency discovery did not return valid JSON.', false, error);
  }
  const after = await loadEnrolledProviderPackage(loadOptions);
  const afterProvider = packageProviderIdentity(after, options.manifestSha256);
  if (provider.sha256 !== afterProvider.sha256) {
    discoveryError('reference-provider-package-changed', 'Provider package identity changed during dependency discovery.');
  }
  const effective = buildEffectiveSource({
    captureManifest: capture.manifest, captureManifestSha256: capture.manifestSha256,
    dependencyReport, policy: options.policy, formatId: options.formatId,
    capabilityId: options.capabilityId, providerFingerprintSha256: provider.sha256,
    providerPackageManifestSha256: options.manifestSha256,
    degradedMode: options.degradedMode,
  });
  return { ...effective, providerIdentity: provider.identity, capture };
}

export async function discoverCapturedSource(options, capture) {
  if (!options || typeof options !== 'object' || Array.isArray(options)
      || typeof options.hostRun !== 'function') {
    discoveryError('reference-provider-host-unavailable', 'Provider dependency discovery requires a managed host.');
  }
  return await invokeDiscover(options, capture);
}

export async function fingerprintSource(options) {
  if (!options || typeof options !== 'object' || Array.isArray(options)
      || typeof options.stagingRoot !== 'string') {
    discoveryError('reference-source-staging-invalid', 'Source fingerprinting requires a private staging path.');
  }
  let capture;
  try {
    capture = await captureSourceNamespaces(options.namespaces, options.stagingRoot, {
      limits: options.captureLimits, signal: options.signal,
    });
    const discovered = await discoverCapturedSource(options, capture);
    return {
      effectiveSource: discovered.effectiveSource, bytes: discovered.bytes,
      sha256: discovered.sha256, dependencyPolicySha256: discovered.dependencyPolicySha256,
      providerIdentity: discovered.providerIdentity,
    };
  } finally {
    if (capture) {
      try { await fs.rm(capture.stagingRoot, { recursive: true, force: true }); }
      catch (error) { discoveryError('reference-provider-run-cleanup-failed', 'Source fingerprint staging could not be removed.', false, error); }
    }
  }
}
