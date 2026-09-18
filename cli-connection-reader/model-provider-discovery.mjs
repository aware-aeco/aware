import fs from 'node:fs/promises';
import { constants as fsConstants } from 'node:fs';
import path from 'node:path';

import {
  canonicalJsonBytes, lowerableLimits, ModelReaderError, parseJsonStrict, sha256,
} from './model-contract.mjs';
import { buildEffectiveSource, validateDependencyPolicy } from './model-effective-source.mjs';
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

export async function loadAdmittedDependencyPolicy(home, providerFingerprintSha256) {
  const pathname = path.join(home, 'providers', 'policies', `${providerFingerprintSha256}.json`);
  let stat; let bytes; let handle;
  try {
    stat = await fs.lstat(pathname, { bigint: true });
    if (!stat.isFile() || stat.isSymbolicLink() || stat.size > BigInt(1024 * 1024)) throw new Error('policy is unsafe or too large');
    handle = await fs.open(pathname, fsConstants.O_RDONLY | (fsConstants.O_NOFOLLOW ?? 0));
    const opened = await handle.stat({ bigint: true });
    const sameDevice = process.platform === 'win32' || stat.dev === opened.dev;
    if (!opened.isFile() || !sameDevice || stat.ino !== opened.ino || stat.size !== opened.size
        || stat.mtimeNs !== opened.mtimeNs) throw new Error('policy changed before open');
    bytes = await handle.readFile();
    const after = await handle.stat({ bigint: true });
    if (after.size !== opened.size || after.mtimeNs !== opened.mtimeNs
        || bytes.length !== Number(opened.size) || bytes.length > 1024 * 1024) {
      throw new Error('policy changed while read');
    }
  } catch (error) {
    discoveryError('reference-dependency-policy-unavailable', 'The admitted dependency policy is unavailable.', false, error);
  } finally {
    await handle?.close().catch(() => {});
  }
  let policy;
  try { policy = parseJsonStrict(bytes, { maxBytes: 1024 * 1024, maxDepth: 32 }); }
  catch (error) { discoveryError('reference-dependency-policy-invalid', 'The admitted dependency policy is invalid.', false, error); }
  let canonical;
  try { canonical = canonicalJsonBytes(policy); }
  catch (error) { discoveryError('reference-dependency-policy-invalid', 'The admitted dependency policy is invalid.', false, error); }
  return { policy, sha256: sha256(canonical) };
}

async function invokeDiscover(options, capture) {
  const limits = lowerableLimits(options.limits);
  const loadOptions = {
    home: options.home, formatId: options.formatId, capabilityId: options.capabilityId,
    manifestSha256: options.manifestSha256, environment: options.environment,
  };
  const before = await loadEnrolledProviderPackage(loadOptions);
  const provider = packageProviderIdentity(before, options.manifestSha256);
  const admittedPolicy = await loadAdmittedDependencyPolicy(options.home, provider.sha256);
  validateDependencyPolicy(admittedPolicy.policy, options.capabilityId, provider.sha256);
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
  const afterPolicy = await loadAdmittedDependencyPolicy(options.home, provider.sha256);
  if (admittedPolicy.sha256 !== afterPolicy.sha256) {
    discoveryError('reference-dependency-policy-changed', 'The admitted dependency policy changed during discovery.');
  }
  const effective = buildEffectiveSource({
    captureManifest: capture.manifest, captureManifestSha256: capture.manifestSha256,
    dependencyReport, policy: admittedPolicy.policy, formatId: options.formatId,
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
