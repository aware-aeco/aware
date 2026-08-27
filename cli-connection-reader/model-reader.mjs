import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import {
  assertSha256, buildCanonicalRequest, ModelReaderError, providerFingerprintSha256,
  lowerableLimits, READER_SCHEMA_VERSION_V1, READER_SCHEMA_VERSION_V2, requestSha256, sha256,
} from './model-contract.mjs';
import { normalizeRevitGlb } from './revit-glb.mjs';
import { normalizeRevitMetadata } from './revit-metadata.mjs';
import { describeAndConvert, describeProvider, hashRegularFile } from './model-provider.mjs';
import {
  acquireCacheOwner, cacheFencePath, cacheKeySha256, cacheMaintenanceFencePath, loadAwareSigningKey, publishCacheEntry, readCacheEntry,
  releaseCacheOwner, signerFingerprintSha256,
} from './model-cache.mjs';
import { createModelHostClient } from './model-host-client.mjs';
import { buildAndPublishSnapshot } from './model-snapshot.mjs';

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

async function newRunRoot(parent) {
  await fs.mkdir(parent, { recursive: true, mode: 0o700 });
  return await fs.mkdtemp(path.join(parent, 'run-'));
}

function emit(deps, phase, extra = {}) {
  if (typeof deps.progress === 'function') deps.progress({ phase, ...extra });
}

function requestedReaderSchemaVersion(args) {
  const version = args['reader-schema-version'] ?? READER_SCHEMA_VERSION_V1;
  if (![READER_SCHEMA_VERSION_V1, READER_SCHEMA_VERSION_V2].includes(version)) {
    readerError('reference-request-invalid', 'request', 'The requested model-reader schema version is unsupported.');
  }
  return version;
}

async function providerReadiness(args, deps, config, expectedProviderSha256) {
  const signingKey = await loadAwareSigningKey(config.secretPath, config.publicPath);
  const signerSha256 = signerFingerprintSha256(signingKey.publicKeyBytes);
  const expectedSignerSha256 = args['expected-signer-sha256'];
  if (expectedSignerSha256 !== undefined) {
    assertSha256(expectedSignerSha256, 'expected-signer-sha256');
    if (expectedSignerSha256 !== signerSha256) readerError('reference-signer-pin-mismatch', 'preflight', 'The model-reader signing key does not match the expected fingerprint.');
  }
  const runRoot = await newRunRoot(config.privateRoot);
  try {
    const provider = await describeProvider({
      executable: config.executable, privateRoot: path.join(runRoot, 'describe'),
      hostRun: deps.hostRun, environment: config.environment, limits: deps.limits,
      signal: deps.signal,
      expectedProviderSha256,
      expectedProtocolVersion: args['expected-provider-protocol'] ?? '1',
      expectedDestination: args['expected-provider-destination'],
      readerSchemaVersion: requestedReaderSchemaVersion(args),
    });
    return {
      signingKey, provider,
      signerFingerprintSha256: signerSha256,
      providerFingerprintSha256: providerFingerprintSha256(provider.fingerprint),
    };
  } finally {
    await fs.rm(runRoot, { recursive: true, force: true });
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

async function convertAndCache(args, deps, config, readiness) {
  const sourcePath = sourcePathFrom(args);
  const initial = await hashSource(sourcePath, deps.limits);
  const sourceSha256 = exactExpectedSource(args, initial.sha256);
  const readerSchemaVersion = requestedReaderSchemaVersion(args);
  const propertyExpansionLimits = args['property-expansion-limits'] ?? {};
  const canonicalRequest = buildCanonicalRequest({
    limits: deps.limits,
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
    expectedPublicKey: readiness.signingKey.publicKeyBytes, withMaintenanceFence,
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
        expectedProtocolVersion: args['expected-provider-protocol'] ?? '1',
        expectedDestination: args['expected-provider-destination'],
        authorityStorePath: args['authority-store-path'],
        readerSchemaVersion,
        propertyExpansionLimits,
      });
      emit(deps, 'normalize');
      const geometry = normalizeRevitGlb(conversion.outputs.geometry.bytes, { limits: deps.limits });
      const metadata = normalizeRevitMetadata(conversion.outputs.metadata.bytes, geometry.parts, {
        limits: deps.limits,
        propertyExpansionLimits: canonicalRequest.propertyExpansionLimits,
      });
      const expectedMetadataSchema = readerSchemaVersion === READER_SCHEMA_VERSION_V2 ? '2' : '1';
      if (JSON.parse(metadata.propertiesBytes.toString('utf8')).schemaVersion !== expectedMetadataSchema) {
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
        cacheLimits: deps.cacheLimits,
        withMaintenanceFence,
      });
    } finally {
      await fs.rm(runRoot, { recursive: true, force: true });
    }
    return { hit: false, key, cache: await read() };
  } finally {
    if (owner) await releaseCacheOwner(owner);
    if (fence !== null) await deps.hostReleaseLock(fence);
  }
}

function summary(result) {
  const coverage = result.cache.manifest.coverage;
  const entityDocument = JSON.parse(result.cache.artifacts['entities.json'].toString('utf8'));
  const bounds = entityDocument.entities.reduce((box, entity) => {
    if (!entity.bounds) return box;
    if (!box) return structuredClone(entity.bounds);
    for (let axis = 0; axis < 3; axis += 1) {
      box.min[axis] = Math.min(box.min[axis], entity.bounds.min[axis]);
      box.max[axis] = Math.max(box.max[axis], entity.bounds.max[axis]);
    }
    return box;
  }, null);
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
  const config = configuration(args, deps);
  const ownedHost = deps.hostRun ? null : await createModelHostClient(config.environment.AWARE_MODEL_READER_HOST, { environment: config.environment });
  const executionDeps = ownedHost ? {
    ...deps, hostRun: ownedHost.run, hostAcquireLock: ownedHost.acquireLock, hostReleaseLock: ownedHost.releaseLock,
  } : deps;
  try {
    const pin = args['expected-provider-sha256'];
    if (command !== 'preflight') {
      if (!['probe', 'read-model', 'read-snapshot'].includes(command)) readerError('reference-command-invalid', 'request', 'Unknown model-reader command.');
      if (typeof pin !== 'string') readerError('reference-provider-pin-required', 'preflight', 'The expected provider fingerprint is required.');
      if (typeof args['expected-signer-sha256'] !== 'string') readerError('reference-signer-pin-required', 'preflight', 'The expected signer fingerprint is required.');
    }
    emit(executionDeps, 'preflight');
    const readiness = await providerReadiness(args, executionDeps, config, pin);
    if (command === 'preflight') {
      return {
        schemaVersion: requestedReaderSchemaVersion(args), ready: true, execution: readiness.provider.describe.execution,
        provider: readiness.provider.describe, providerFingerprint: readiness.provider.fingerprint,
        providerFingerprintSha256: readiness.providerFingerprintSha256,
        signerFingerprintSha256: readiness.signerFingerprintSha256,
        signerPublicKeyBase64: readiness.signingKey.publicKeyBytes.toString('base64'),
        secretProvisioning: 'provider-local; AWARE generic secrets unavailable (#448)',
      };
    }
    const result = await convertAndCache(args, executionDeps, config, readiness);
    const out = summary(result);
    if (command === 'probe') return out;
    emit(executionDeps, 'artifacts');
    if (command === 'read-snapshot') {
      return {
        ...out,
        ...await buildAndPublishSnapshot(result, readiness.signingKey, config.artifactDirectory, { limits: deps.limits }),
      };
    }
    return { ...out, artifacts: await publishRunArtifacts(result, config.artifactDirectory) };
  } finally {
    await ownedHost?.close();
  }
}
