import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import {
  assertSha256, buildCanonicalRequest, canonicalArtifactLimits, ModelReaderError,
  providerFingerprintSha256, lowerableLimits, READER_SCHEMA_VERSION_V1, READER_SCHEMA_VERSION_V2,
  requestSha256, sha256,
} from './model-contract.mjs';
import { normalizeRevitGlb, parseGlb } from './revit-glb.mjs';
import { normalizeRevitMetadata } from './revit-metadata.mjs';
import { describeAndConvert, describeProvider, hashRegularFile } from './model-provider.mjs';
import {
  acquireCacheOwner, cacheFencePath, cacheKeySha256, cacheMaintenanceFencePath, findCacheEntry,
  loadAwareSigningKey, publishCacheEntry, readCacheEntry,
  releaseCacheOwner, signerFingerprintSha256,
} from './model-cache.mjs';
import { createModelHostClient } from './model-host-client.mjs';
import { buildAndPublishSnapshot } from './model-snapshot.mjs';
import { preflightEnrolledProviderPackage } from './model-provider-package.mjs';
import { fingerprintSource } from './model-provider-discovery.mjs';
import { sourceCaptureLimits } from './model-source-capture.mjs';
import { buildProviderConversionRequest, convertProviderSource } from './model-provider-conversion.mjs';
import { canonicalizeProviderOutput, publishCanonicalArtifact } from './model-canonical-v2.mjs';
import { publishV3Cache, readV3Cache, v3CacheKey } from './model-v3-cache.mjs';

const STALE_PROVIDER_RUN_MS = 60 * 60_000;
const ACTIVE_RUN_MARKER = '.active';
const PROVIDER_RUN_HEARTBEAT_MS = 60_000;
const providerRunHeartbeats = new Map();
const CONVERSION_ATTEMPT_ID = /^[0-9a-f]{8}-[0-9a-f]{4}-[1-5][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;

function readerError(code, phase, message, retryable = false, details = undefined) {
  throw new ModelReaderError(code, phase, retryable, message, details);
}

function awareHome(environment) {
  return environment.AWARE_HOME || path.join(os.homedir(), '.aware');
}

function configuration(args, deps) {
  const environment = deps.environment ?? process.env;
  const home = awareHome(environment);
  const executable = args['provider-path'] ?? environment.AWARE_MODEL_REFERENCE_PROVIDER;
  if (typeof executable !== 'string' || !executable) readerError('reference-provider-unavailable', 'preflight', 'A local model provider is not configured.');
  const secretPath = args['signing-secret-path'] ?? environment.AWARE_MODEL_REFERENCE_SIGNING_KEY ?? path.join(home, 'keys', 'model-reference-reader.sec');
  const publicPath = args['signing-public-path'] ?? environment.AWARE_MODEL_REFERENCE_PUBLIC_KEY ?? secretPath.replace(/\.sec$/i, '.pub');
  return {
    executable, secretPath, publicPath, environment,
    cacheRoot: deps.cacheRoot ?? path.join(home, 'cache', 'model-reference-reader'),
    privateRoot: deps.privateRoot ?? path.join(home, 'cache', 'model-reference-reader', 'provider-runs'),
    artifactDirectory: deps.artifactDirectory ?? environment.AWARE_ARTIFACT_DIR,
  };
}

function packageMode(args) {
  return ['provider-format', 'provider-capability', 'provider-package-sha256']
    .some((field) => Object.hasOwn(args, field));
}

function packageSigningConfiguration(args, deps) {
  const environment = deps.environment ?? process.env;
  const home = awareHome(environment);
  const secretPath = args['signing-secret-path'] ?? environment.AWARE_MODEL_REFERENCE_SIGNING_KEY ?? path.join(home, 'keys', 'model-reference-reader.sec');
  const publicPath = args['signing-public-path'] ?? environment.AWARE_MODEL_REFERENCE_PUBLIC_KEY ?? secretPath.replace(/\.sec$/i, '.pub');
  return {
    environment, home, secretPath, publicPath,
    cacheRoot: deps.cacheRoot ?? path.join(home, 'cache', 'model-reference-reader'),
    privateRoot: deps.privateRoot ?? path.join(home, 'cache', 'model-reference-reader', 'provider-runs'),
  };
}

function validatePackageRequest(command, args, deps) {
  const limits = requestLimits(args, deps);
  let captureLimits;
  try { captureLimits = sourceCaptureLimits(args['source-capture-limits']); }
  catch (error) { readerError('reference-limits-invalid', 'request', 'Source capture limits are invalid.', false, error); }
  for (const field of ['provider-format', 'provider-capability', 'provider-package-sha256']) {
    if (typeof args[field] !== 'string' || !args[field]) readerError('reference-provider-package-request-invalid', 'request', `Package preflight requires ${field}.`);
  }
  if (args['expected-provider-protocol'] !== '3') {
    readerError('reference-provider-package-request-invalid', 'request', 'Enrolled provider packages require protocol v3.');
  }
  for (const field of ['provider-path', 'expected-provider-sha256', 'expected-provider-destination', 'authority-store-path']) {
    if (args[field] !== undefined) readerError('reference-provider-package-request-invalid', 'request', `Package preflight cannot mix ${field} with enrollment selection.`);
  }
  if (typeof args['provider-authorization'] !== 'string' || !args['provider-authorization']
      || Buffer.byteLength(args['provider-authorization']) > 64 * 1024
      || /[\u0000-\u001f\u007f]/.test(args['provider-authorization'])) {
    readerError('reference-provider-authorization-invalid', 'request', 'Enrolled provider execution requires host authorization.');
  }
  if (args['reader-schema-version'] !== 'model-reference-reader/v3') {
    readerError('reference-provider-package-request-invalid', 'request', 'Enrolled provider reads require reader schema v3.');
  }
  if (command !== 'preflight') {
    if (!Array.isArray(args['source-namespaces']) || args['source-namespaces'].length === 0) {
      readerError('reference-source-namespaces-invalid', 'request', 'Enrolled provider execution requires captured namespace roots.');
    }
    if (args['degraded-mode'] !== undefined && !['refuse', 'allow'].includes(args['degraded-mode'])) {
      readerError('reference-request-invalid', 'request', 'degraded-mode must be refuse or allow.');
    }
  }
  if (args['expected-signer-sha256'] !== undefined) assertSha256(args['expected-signer-sha256'], 'expected-signer-sha256');
  return { limits, captureLimits };
}

async function hasFreshHeartbeat(candidate, now) {
  const marker = path.join(candidate, ACTIVE_RUN_MARKER);
  try {
    const stat = await fs.lstat(marker);
    if (!stat.isFile() || stat.isSymbolicLink()) return false;
    const age = now - stat.mtimeMs;
    return Number.isFinite(age) && age >= -PROVIDER_RUN_HEARTBEAT_MS && age <= STALE_PROVIDER_RUN_MS;
  } catch (error) {
    if (error?.code === 'ENOENT') return false;
    throw error;
  }
}

async function sweepAbandonedRunRoots(parent, now = Date.now()) {
  let names;
  try {
    await fs.mkdir(parent, { recursive: true, mode: 0o700 });
    names = await fs.readdir(parent);
  } catch (error) {
    readerError('reference-provider-run-cleanup-failed', 'cleanup', 'Provider staging could not be inspected.', false, error);
  }
  for (const name of names) {
    if (!name.startsWith('run-')) continue;
    const candidate = path.join(parent, name);
    try {
      const stat = await fs.lstat(candidate);
      if (stat.isDirectory() && !stat.isSymbolicLink() && now - stat.mtimeMs > STALE_PROVIDER_RUN_MS && !await hasFreshHeartbeat(candidate, now)) {
        await fs.rm(candidate, { recursive: true, force: true });
      }
    } catch (error) {
      if (error?.code !== 'ENOENT') {
        readerError('reference-provider-run-cleanup-failed', 'cleanup', 'Abandoned provider staging could not be reclaimed.', false, error);
      }
    }
  }
}

async function newRunRoot(parent) {
  await sweepAbandonedRunRoots(parent);
  let runRoot;
  try { runRoot = await fs.mkdtemp(path.join(parent, 'run-')); }
  catch (error) { readerError('reference-provider-run-cleanup-failed', 'cleanup', 'Provider staging could not be allocated.', false, error); }
  try {
    const marker = path.join(runRoot, ACTIVE_RUN_MARKER);
    await fs.writeFile(marker, '', { flag: 'wx', mode: 0o600 });
    const heartbeat = setInterval(() => {
      const current = new Date();
      void fs.utimes(marker, current, current).catch(() => {});
    }, PROVIDER_RUN_HEARTBEAT_MS);
    heartbeat.unref();
    providerRunHeartbeats.set(runRoot, heartbeat);
    return runRoot;
  } catch (error) {
    try { await fs.rm(runRoot, { recursive: true, force: true }); }
    catch (removeError) { readerError('reference-provider-run-cleanup-failed', 'cleanup', 'Unowned provider staging could not be removed.', false, removeError); }
    readerError('reference-provider-run-cleanup-failed', 'cleanup', 'Provider staging ownership could not be recorded.', false, error);
  }
}

async function removeRunRoot(runRoot) {
  clearInterval(providerRunHeartbeats.get(runRoot));
  providerRunHeartbeats.delete(runRoot);
  try { await fs.rm(runRoot, { recursive: true, force: true }); }
  catch (error) { readerError('reference-provider-run-cleanup-failed', 'cleanup', 'Provider staging could not be removed.', false, error); }
}

function requestLimits(args, deps, readerSchemaVersion) {
  const configured = Object.hasOwn(args, 'limits') ? args.limits : deps.limits;
  try {
    return lowerableLimits(configured, readerSchemaVersion);
  }
  catch (error) { readerError('reference-limits-invalid', 'request', 'Model reader limits are invalid.', false, error); }
}

function requestedReaderSchemaVersion(args) {
  const version = args['reader-schema-version'] ?? READER_SCHEMA_VERSION_V1;
  if (![READER_SCHEMA_VERSION_V1, READER_SCHEMA_VERSION_V2].includes(version)) {
    readerError('reference-request-invalid', 'request', 'The requested model-reader schema version is unsupported.');
  }
  return version;
}

function validateRequest(command, args, deps) {
  const readerSchemaVersion = requestedReaderSchemaVersion(args);
  const limits = requestLimits(args, deps, readerSchemaVersion);
  const protocolVersion = args['expected-provider-protocol'] ?? '1';
  // A managed conversion has a delivery-attempt identity which must be part of
  // the independently versioned package signature. That binding was introduced
  // with reader schema v2, so accepting protocol 2 with the v1 package schema
  // would validate an attempt that the delivered artifact cannot authenticate.
  if (protocolVersion === '2' && readerSchemaVersion !== READER_SCHEMA_VERSION_V2) {
    readerError('reference-request-invalid', 'request', 'Managed Revit conversion requires model-reader schema version v2.');
  }
  // This constructs and canonicalizes the complete request-only contract without touching the
  // provider, signing key, cache, or source filesystem. It validates protocol, conversion
  // settings, and all lowerable limits before an environment-dependent error can mask them.
  try {
    requestSha256(buildCanonicalRequest({
      limits,
      protocolVersion,
      conversionSettings: args['conversion-settings'] ?? {},
      readerSchemaVersion,
      propertyExpansionLimits: args['property-expansion-limits'] ?? {},
    }));
  } catch (error) {
    if (error instanceof ModelReaderError) throw error;
    readerError('reference-request-invalid', 'request', 'The model reader request is invalid.', false, error);
  }
  if (command !== 'preflight') {
    sourcePathFrom(args);
    assertSha256(args['source-sha256'], 'source-sha256');
    if (typeof args['expected-provider-sha256'] !== 'string') readerError('reference-provider-pin-required', 'preflight', 'The expected provider fingerprint is required.');
    if (typeof args['expected-signer-sha256'] !== 'string') readerError('reference-signer-pin-required', 'preflight', 'The expected signer fingerprint is required.');
  }
  for (const [field, label] of [
    ['expected-provider-sha256', 'expected-provider-sha256'],
    ['expected-signer-sha256', 'expected-signer-sha256'],
  ]) {
    if (args[field] !== undefined) assertSha256(args[field], label);
  }
  return { limits, pin: args['expected-provider-sha256'] };
}

function conversionAttemptId(args, protocolVersion) {
  if (protocolVersion !== '2') return undefined;
  const attemptId = args['conversion-attempt-id'];
  if (typeof attemptId !== 'string' || !CONVERSION_ATTEMPT_ID.test(attemptId)) {
    readerError('reference-provider-request-invalid', 'request', 'The managed Revit conversion attempt identity is invalid.');
  }
  return attemptId;
}

function emit(deps, phase, extra = {}) {
  if (typeof deps.progress === 'function') deps.progress({ phase, ...extra });
}

async function signingReadiness(args, config) {
  const signingKey = await loadAwareSigningKey(config.secretPath, config.publicPath);
  const signerSha256 = signerFingerprintSha256(signingKey.publicKeyBytes);
  const expectedSignerSha256 = args['expected-signer-sha256'];
  if (expectedSignerSha256 !== undefined) {
    assertSha256(expectedSignerSha256, 'expected-signer-sha256');
    if (expectedSignerSha256 !== signerSha256) readerError('reference-signer-pin-mismatch', 'preflight', 'The model-reader signing key does not match the expected fingerprint.');
  }
  return { signingKey, signerFingerprintSha256: signerSha256 };
}

async function providerReadiness(args, deps, config, expectedProviderSha256, signing) {
  const runRoot = await newRunRoot(config.privateRoot);
  try {
    const provider = await describeProvider({
      executable: config.executable, privateRoot: path.join(runRoot, 'describe'),
      hostRun: deps.hostRun, environment: config.environment, limits: deps.limits,
      signal: deps.signal,
      expectedProviderSha256,
      expectedProtocolVersion: args['expected-provider-protocol'] ?? '1',
      expectedDestination: args['expected-provider-destination'],
      authorityStorePath: args['authority-store-path'],
      readerSchemaVersion: requestedReaderSchemaVersion(args),
    });
    return {
      ...signing, provider,
      providerFingerprintSha256: providerFingerprintSha256(provider.fingerprint),
    };
  } finally {
    await removeRunRoot(runRoot);
  }
}

function sourcePathFrom(args) {
  const values = [args['rvt-path'], args.rvtPath, args['model-path']].filter((value) => value !== undefined);
  if (values.length !== 1 || typeof values[0] !== 'string' || !values[0]) readerError('reference-source-invalid', 'source', 'Exactly one absolute RVT source path is required.');
  if (!path.isAbsolute(values[0]) || !/\.rvt$/i.test(values[0])) readerError('reference-source-invalid', 'source', 'The source must be an absolute .rvt path.');
  return values[0];
}

async function hashSource(sourcePath, limits) {
  try { return await hashRegularFile(sourcePath, lowerableLimits(limits).maxSourceBytes, 'source'); }
  catch (error) {
    if (error instanceof ModelReaderError) throw error;
    readerError('reference-source-unavailable', 'source', 'The RVT source is unavailable.', false, error);
  }
}

function exactExpectedSource(args, actual) {
  const expected = args['source-sha256'];
  assertSha256(expected, 'source-sha256');
  if (expected !== actual) readerError('reference-source-changed', 'source', 'The RVT source does not match the expected digest.');
  return expected;
}

function manifestDetails(geometry, metadata, canonicalRequestSha256, fingerprintSha256) {
  return {
    frame: { units: 'mm', up: 'z', handedness: 'right', axes: ['x', 'y', 'z'] },
    canonicalRequestSha256,
    providerFingerprintSha256: fingerprintSha256,
    coverage: { ...metadata.coverage, geometry: geometry.coverage },
  };
}

async function convertAndCache(args, deps, config, readiness, attemptId) {
  const sourcePath = sourcePathFrom(args);
  const initial = await hashSource(sourcePath, deps.limits);
  const sourceSha256 = exactExpectedSource(args, initial.sha256);
  const expectedProtocolVersion = args['expected-provider-protocol'] ?? '1';
  const readerSchemaVersion = requestedReaderSchemaVersion(args);
  const propertyExpansionLimits = args['property-expansion-limits'] ?? {};
  const canonicalRequest = buildCanonicalRequest({
    limits: deps.limits,
    protocolVersion: expectedProtocolVersion,
    conversionSettings: args['conversion-settings'] ?? {},
    readerSchemaVersion,
    propertyExpansionLimits,
  });
  const identity = {
    sourceSha256, canonicalRequest, providerFingerprint: readiness.provider.fingerprint,
    signerFingerprintSha256: readiness.signerFingerprintSha256,
  };
  const key = cacheKeySha256(identity);
  const withMaintenanceFence = async (work) => {
    if (!deps.hostAcquireLock || !deps.hostReleaseLock) readerError('reference-provider-host-unavailable', 'cache', 'The managed cache fence is unavailable.');
    const maintenanceFence = await deps.hostAcquireLock(
      await cacheMaintenanceFencePath(config.cacheRoot),
      { signal: deps.signal },
    );
    try { return await work(); }
    finally { await deps.hostReleaseLock(maintenanceFence); }
  };
  const read = async () => await readCacheEntry({
    root: config.cacheRoot, key, expectedIdentity: identity,
    expectedPublicKey: readiness.signingKey.publicKeyBytes, withMaintenanceFence, limits: deps.limits,
  });
  try { return { hit: true, key, cache: await read() }; }
  catch (error) { if (error?.code !== 'reference-cache-miss') throw error; }
  const fence = deps.hostAcquireLock
    ? await deps.hostAcquireLock(await cacheFencePath(config.cacheRoot, key), { signal: deps.signal })
    : null;
  let owner;
  try {
    owner = await acquireCacheOwner({ root: config.cacheRoot, key, fenced: fence !== null });
    try { return { hit: true, key, cache: await read() }; }
    catch (error) { if (error?.code !== 'reference-cache-miss') throw error; }
    emit(deps, 'convert');
    const runRoot = await newRunRoot(config.privateRoot);
    let conversion;
    try {
      conversion = await describeAndConvert({
        executable: config.executable, sourcePath, expectedSourceSha256: sourceSha256,
        expectedProviderSha256: readiness.providerFingerprintSha256,
        privateRoot: path.join(runRoot, 'conversion'), hostRun: deps.hostRun,
        environment: config.environment, limits: deps.limits,
        signal: deps.signal,
        conversionSettings: args['conversion-settings'] ?? {},
        expectedProtocolVersion,
        expectedDestination: args['expected-provider-destination'],
        authorityStorePath: args['authority-store-path'],
        conversionAttemptId: attemptId,
        readerSchemaVersion,
        propertyExpansionLimits,
      });
      emit(deps, 'normalize');
      const geometry = normalizeRevitGlb(conversion.outputs.geometry.bytes, { limits: deps.limits });
      const expectedMetadataSchema = readerSchemaVersion === READER_SCHEMA_VERSION_V2 ? '2' : '1';
      const metadata = normalizeRevitMetadata(conversion.outputs.metadata.bytes, geometry.parts, {
        limits: deps.limits,
        propertyExpansionLimits: canonicalRequest.propertyExpansionLimits,
        expectedSchemaVersion: expectedMetadataSchema,
      });
      // The normalizer refuses a mismatch up front, so this is defence in depth
      // against that guard being weakened. Assert on the coverage the normalizer
      // already returned rather than re-parsing propertiesBytes, which can reach
      // maxCanonicalPropertyBytes (128 MiB default and hard cap) on every read.
      if ((metadata.coverage.metadataSchemaVersion ?? '1') !== expectedMetadataSchema) {
        readerError('reference-metadata-invalid', 'normalize-metadata', 'Provider metadata does not match the requested reader schema version.');
      }
      const finalSource = await hashSource(sourcePath, deps.limits);
      if (finalSource.sha256 !== sourceSha256) readerError('reference-source-changed', 'source', 'The RVT source changed during conversion.');
      const artifacts = {
        'geometry.glb': geometry.glb,
        'entities.json': metadata.entitiesBytes,
        'properties.json': metadata.propertiesBytes,
        'relationships.json': metadata.relationshipsBytes,
      };
      const details = manifestDetails(geometry, metadata, requestSha256(canonicalRequest), readiness.providerFingerprintSha256);
      emit(deps, 'publish');
      await publishCacheEntry({
        root: config.cacheRoot, identity, artifacts, signingKey: readiness.signingKey, details,
        limits: deps.limits,
        cacheLimits: deps.cacheLimits,
        withMaintenanceFence,
      });
    } finally {
      await removeRunRoot(runRoot);
    }
    return { hit: false, key, cache: await read() };
  } finally {
    if (owner) await releaseCacheOwner(owner);
    if (fence !== null) await deps.hostReleaseLock(fence);
  }
}

function canonicalGeometryBounds(bytes, limits) {
  // `bytes` is this reader's own canonical artifact, so it is measured against the canonical
  // ceilings rather than the input ones it was never bound by.
  const document = parseGlb(bytes, { limits: canonicalArtifactLimits(limits) }).json;
  const positionAccessors = new Set();
  for (const mesh of document.meshes ?? []) {
    for (const primitive of mesh?.primitives ?? []) {
      const index = primitive?.attributes?.POSITION;
      if (Number.isSafeInteger(index)) positionAccessors.add(index);
    }
  }
  let bounds = null;
  for (const index of positionAccessors) {
    const accessor = document.accessors?.[index];
    if (!accessor || !Array.isArray(accessor.min) || !Array.isArray(accessor.max)
        || accessor.min.length !== 3 || accessor.max.length !== 3
        || [...accessor.min, ...accessor.max].some((value) => typeof value !== 'number' || !Number.isFinite(value))) {
      readerError('reference-cache-entry-invalid', 'cache', 'Cached canonical geometry bounds are invalid.');
    }
    if (!bounds) bounds = { min: [...accessor.min], max: [...accessor.max] };
    else for (let axis = 0; axis < 3; axis += 1) {
      bounds.min[axis] = Math.min(bounds.min[axis], accessor.min[axis]);
      bounds.max[axis] = Math.max(bounds.max[axis], accessor.max[axis]);
    }
  }
  return bounds;
}

async function findCachedConversion(args, deps, config, signing, expectedProviderSha256) {
  const sourcePath = sourcePathFrom(args);
  const initial = await hashSource(sourcePath, deps.limits);
  const sourceSha256 = exactExpectedSource(args, initial.sha256);
  const canonicalRequest = buildCanonicalRequest({
    limits: deps.limits,
    protocolVersion: args['expected-provider-protocol'] ?? '1',
    conversionSettings: args['conversion-settings'] ?? {},
    readerSchemaVersion: requestedReaderSchemaVersion(args),
    propertyExpansionLimits: args['property-expansion-limits'] ?? {},
  });
  if (!deps.hostAcquireLock || !deps.hostReleaseLock) readerError('reference-provider-host-unavailable', 'cache', 'The managed cache fence is unavailable.');
  const withMaintenanceFence = async (work) => {
    const maintenanceFence = await deps.hostAcquireLock(
      await cacheMaintenanceFencePath(config.cacheRoot),
      { signal: deps.signal },
    );
    try { return await work(); }
    finally { await deps.hostReleaseLock(maintenanceFence); }
  };
  try {
    const cache = await findCacheEntry({
      root: config.cacheRoot,
      expectedSourceSha256: sourceSha256,
      expectedCanonicalRequest: canonicalRequest,
      expectedProviderFingerprintSha256: expectedProviderSha256,
      expectedSignerFingerprintSha256: signing.signerFingerprintSha256,
      expectedPublicKey: signing.signingKey.publicKeyBytes,
      withMaintenanceFence,
      limits: deps.limits,
      signal: deps.signal,
    });
    return { hit: true, key: cache.key, cache };
  } catch (error) {
    if (error?.code === 'reference-cache-miss') return null;
    throw error;
  }
}

function summary(result, limits) {
  const coverage = result.cache.manifest.coverage;
  const bounds = canonicalGeometryBounds(result.cache.artifacts['geometry.glb'], limits);
  return {
    schemaVersion: result.cache.manifest.identity.canonicalRequest.readerSchemaVersion, cache: result.hit ? 'hit' : 'miss',
    sourceSha256: result.cache.manifest.identity.sourceSha256,
    canonicalRequestSha256: result.cache.manifest.canonicalRequestSha256,
    providerFingerprint: result.cache.manifest.identity.providerFingerprint,
    providerFingerprintSha256: result.cache.manifest.providerFingerprintSha256,
    signerFingerprintSha256: result.cache.manifest.identity.signerFingerprintSha256,
    frame: result.cache.manifest.frame, coverage, bounds,
    entities: coverage.indexedEntities, geometryNodes: coverage.geometryNodes,
    properties: coverage.properties, relationships: coverage.relationships,
  };
}

async function publishRunArtifacts(result, directory) {
  if (typeof directory !== 'string' || !path.isAbsolute(directory)) readerError('reference-artifact-directory-missing', 'publish', 'A run-owned artifact directory is required.');
  await fs.mkdir(directory, { recursive: true, mode: 0o700 });
  const logical = { geometry: 'geometry.glb', entities: 'entities.json', properties: 'properties.json', relationships: 'relationships.json', manifest: 'manifest.json' };
  const descriptors = {};
  for (const [name, cacheName] of Object.entries(logical)) {
    const bytes = result.cache.artifacts[cacheName];
    const digest = sha256(bytes);
    const id = `model-${result.key.slice(0, 16)}-${cacheName}`;
    const target = path.join(directory, id);
    try { await fs.writeFile(target, bytes, { flag: 'wx', mode: 0o600 }); }
    catch (error) {
      if (error?.code !== 'EEXIST' || sha256(await fs.readFile(target)) !== digest) readerError('reference-artifact-collision', 'publish', 'A run artifact id collided with different bytes.', false, error);
    }
    descriptors[name] = { id, mediaType: cacheName.endsWith('.glb') ? 'model/gltf-binary' : 'application/json', bytes: bytes.length, sha256: digest };
  }
  return descriptors;
}

export async function runModelCommand(command, args = {}, deps = {}) {
  if (!args || typeof args !== 'object' || Array.isArray(args)) readerError('reference-request-invalid', 'request', 'Command input must be a JSON object.');
  if (!['preflight', 'fingerprint-source', 'probe', 'read-model', 'read-snapshot'].includes(command)) {
    readerError('reference-command-invalid', 'request', 'Unknown model-reader command.');
  }
  if (packageMode(args) || command === 'fingerprint-source') {
    const { limits, captureLimits } = validatePackageRequest(command, args, deps);
    const config = packageSigningConfiguration(args, deps);
    const signing = await signingReadiness(args, config);
    const ownedHost = deps.hostRun ? null : await createModelHostClient(config.environment.AWARE_MODEL_READER_HOST, { environment: config.environment });
    try {
      if (command === 'preflight') {
        const result = await preflightEnrolledProviderPackage({
          home: config.home, formatId: args['provider-format'], capabilityId: args['provider-capability'],
          manifestSha256: args['provider-package-sha256'], limits, environment: config.environment,
          hostRun: deps.hostRun ?? ownedHost?.run, signal: deps.signal,
          authorization: args['provider-authorization'],
        });
        return {
          ...result,
          signerFingerprintSha256: signing.signerFingerprintSha256,
          signerPublicKeyBase64: signing.signingKey.publicKeyBytes.toString('base64'),
        };
      }
      const runRoot = await newRunRoot(config.privateRoot);
      try {
        const fingerprint = await (deps.fingerprintSource ?? fingerprintSource)({
          home: config.home, formatId: args['provider-format'], capabilityId: args['provider-capability'],
          manifestSha256: args['provider-package-sha256'], limits, environment: config.environment,
          hostRun: deps.hostRun ?? ownedHost?.run, signal: deps.signal,
          authorization: args['provider-authorization'], namespaces: args['source-namespaces'],
          stagingRoot: path.join(runRoot, 'source'), degradedMode: args['degraded-mode'] ?? 'refuse',
          captureLimits,
        });
        if (command === 'fingerprint-source') {
          return {
            schemaVersion: 'model-reference-reader-source-fingerprint/v1',
            effectiveSource: fingerprint.effectiveSource,
            effectiveSourceSha256: fingerprint.sha256,
            dependencyPolicySha256: fingerprint.dependencyPolicySha256,
            providerIdentity: fingerprint.providerIdentity,
            signerFingerprintSha256: signing.signerFingerprintSha256,
          };
        }
        const conversionIdentity = buildProviderConversionRequest({
          formatId: args['provider-format'], capabilityId: args['provider-capability'],
          providerPackageManifestSha256: args['provider-package-sha256'],
          effectiveSourceSha256: fingerprint.sha256, degradedMode: args['degraded-mode'] ?? 'refuse',
          conversionSettings: args['conversion-settings'] ?? {}, limits,
          outputLimits: args['provider-output-limits'],
        });
        const cacheIdentity = {
          effectiveSourceSha256: fingerprint.sha256,
          conversionRequestSha256: conversionIdentity.sha256,
          providerPackageManifestSha256: args['provider-package-sha256'],
          signerFingerprintSha256: signing.signerFingerprintSha256,
        };
        const cacheKey = v3CacheKey(cacheIdentity);
        let canonical; let cache = 'hit';
        try {
          canonical = await (deps.readV3Cache ?? readV3Cache)(
            config.cacheRoot, cacheKey, cacheIdentity, signing.signingKey.publicKeyBytes,
          );
        } catch (error) {
          if (error?.code !== 'reference-cache-miss') throw error;
          cache = 'miss';
        }
        if (!canonical) {
          const conversion = await (deps.convertProviderSource ?? convertProviderSource)({
            home: config.home, formatId: args['provider-format'], capabilityId: args['provider-capability'],
            manifestSha256: args['provider-package-sha256'], authorization: args['provider-authorization'],
            namespaces: args['source-namespaces'], effectiveSource: fingerprint.effectiveSource,
            effectiveSourceSha256: fingerprint.sha256, degradedMode: args['degraded-mode'] ?? 'refuse',
            conversionSettings: args['conversion-settings'] ?? {}, limits,
            outputLimits: args['provider-output-limits'], captureLimits, environment: config.environment,
            hostRun: deps.hostRun ?? ownedHost?.run, signal: deps.signal,
            stagingRoot: path.join(runRoot, 'conversion'),
          });
          if (conversion.conversionRequestSha256 !== conversionIdentity.sha256) {
            readerError('reference-provider-output-invalid', 'provider-output',
              'Provider conversion identity changed during execution.');
          }
          canonical = await (deps.canonicalizeProviderOutput ?? canonicalizeProviderOutput)({
            output: conversion.output, effectiveSource: fingerprint.effectiveSource,
            formatId: args['provider-format'], capabilityId: args['provider-capability'],
            providerPackageManifestSha256: args['provider-package-sha256'],
            conversionRequestSha256: conversion.conversionRequestSha256,
            workRoot: runRoot, signal: deps.signal,
          });
          canonical = await (deps.publishV3Cache ?? publishV3Cache)(
            config.cacheRoot, cacheKey, cacheIdentity, canonical, signing.signingKey,
          );
        }
        const result = {
          schemaVersion: 'model-reference-reader/v3', cache,
          effectiveSourceSha256: fingerprint.sha256,
          providerPackageManifestSha256: args['provider-package-sha256'],
          providerFingerprintSha256: fingerprint.effectiveSource.providerFingerprintSha256,
          signerFingerprintSha256: signing.signerFingerprintSha256,
          artifactRootSha256: canonical.root.sha256,
          completeness: canonical.root.manifest.completeness,
          counts: Object.fromEntries(Object.entries(canonical.indexes)
            .map(([family, index]) => [family, index.index.itemCount])),
        };
        if (command === 'probe') return result;
        return {
          ...result,
          ...await (deps.publishCanonicalArtifact ?? publishCanonicalArtifact)(
            canonical, signing.signingKey, deps.artifactDirectory ?? config.environment.AWARE_ARTIFACT_DIR,
          ),
        };
      } finally {
        await removeRunRoot(runRoot);
      }
    } finally {
      if (ownedHost) await ownedHost.close();
    }
  }
  // Validate the complete request-only contract before resolving credentials, starting a host,
  // or probing a provider. Malformed calls retain stable errors on an unconfigured machine.
  const { limits, pin } = validateRequest(command, args, deps);
  const config = configuration(args, deps);
  const ownedHost = deps.hostRun ? null : await createModelHostClient(config.environment.AWARE_MODEL_READER_HOST, { environment: config.environment });
  const executionDeps = {
    ...deps, limits,
    ...(ownedHost ? { hostRun: ownedHost.run, hostAcquireLock: ownedHost.acquireLock, hostReleaseLock: ownedHost.releaseLock } : {}),
  };
  try {
    emit(executionDeps, 'preflight');
    const signing = await signingReadiness(args, config);
    if (command === 'preflight') {
      const readiness = await providerReadiness(args, executionDeps, config, pin, signing);
      return {
        schemaVersion: requestedReaderSchemaVersion(args), ready: true, execution: readiness.provider.describe.execution,
        provider: readiness.provider.describe, providerFingerprint: readiness.provider.fingerprint,
        providerFingerprintSha256: readiness.providerFingerprintSha256,
        signerFingerprintSha256: readiness.signerFingerprintSha256,
        signerPublicKeyBase64: readiness.signingKey.publicKeyBytes.toString('base64'),
        secretProvisioning: 'provider-local; AWARE generic secrets unavailable (#448)',
      };
    }
    let result = await findCachedConversion(args, executionDeps, config, signing, pin);
    let attemptId = null;
    let readiness = signing;
    if (!result) {
      // A durable attempt identity binds a managed conversion request to its receipt. Validate it
      // after the authenticated warm-cache path, but before any provider I/O for a cold request.
      attemptId = conversionAttemptId(args, args['expected-provider-protocol'] ?? '1');
      readiness = await providerReadiness(args, executionDeps, config, pin, signing);
      result = await convertAndCache(args, executionDeps, config, readiness, attemptId);
    } else if (command === 'read-snapshot' && (args['expected-provider-protocol'] ?? '1') === '2') {
      // A warm cache avoids provider I/O, but the package is freshly signed for this delivery.
      // Bind that signature to the caller's current attempt so an older package cannot be replayed.
      attemptId = conversionAttemptId(args, '2');
    }
    const out = summary(result, limits);
    if (command === 'probe') return out;
    emit(executionDeps, 'artifacts');
    if (command === 'read-snapshot') {
      return {
        ...out,
        ...await buildAndPublishSnapshot(result, readiness.signingKey, config.artifactDirectory, {
          limits, conversionAttemptId: attemptId,
        }),
      };
    }
    return { ...out, artifacts: await publishRunArtifacts(result, config.artifactDirectory) };
  } finally {
    await ownedHost?.close();
  }
}
