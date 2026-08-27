import fs from 'node:fs/promises';
import path from 'node:path';
import {
  canonicalJsonBytes, lowerableLimits, ModelReaderError, parseJsonStrict, sha256,
} from './model-contract.mjs';
import { signArtifactPreimage } from './model-artifact-auth.mjs';

export const SOURCE_ARTIFACT_DOMAIN = 'AWARE\0model-reference-reader\0source-artifact-set\0v1\0';
export const PACKAGE_ARTIFACT_DOMAIN = 'AWARE\0model-reference-reader\0package-set\0v1\0';
export const SOURCE_ARTIFACT_DOMAIN_V2 = 'AWARE\0model-reference-reader\0source-artifact-set\0v2\0';
export const PACKAGE_ARTIFACT_DOMAIN_V2 = 'AWARE\0model-reference-reader\0package-set\0v2\0';

const SOURCE_ORDER = [
  ['geometry', 'geometry.glb', 'model/gltf-binary'],
  ['entities', 'entities.json', 'application/json'],
  ['properties', 'properties.json', 'application/json'],
  ['relationships', 'relationships.json', 'application/json'],
  ['manifest', 'manifest.json', 'application/json'],
];

const PACKAGE_ORDER = [
  ['manifest', 'snapshot-manifest.json', 'application/json'],
  ['tile-000000', 'tile-000000.glb', 'model/gltf-binary'],
  ['entities-000000', 'entities-000000.json', 'application/json'],
  ['properties-000000', 'properties-000000.json', 'application/json'],
  ['relationships-000000', 'relationships-000000.json', 'application/json'],
  ['index', 'index.json', 'application/json'],
];

function snapshotError(code, message, details = undefined) {
  throw new ModelReaderError(code, 'package', false, message, details);
}

function document(bytes, key, limits, expectedSchemaVersion) {
  let parsed;
  try { parsed = parseJsonStrict(bytes, { maxBytes: limits.maxComponentJsonBytes, maxDepth: limits.maxJsonDepth }); }
  catch (error) { snapshotError('reference-snapshot-source-invalid', `${key} source artifact is invalid.`, error); }
  if (parsed?.schemaVersion !== expectedSchemaVersion || !Array.isArray(parsed[key])) {
    snapshotError('reference-snapshot-source-invalid', `${key} source artifact has an invalid closed document.`);
  }
  return parsed;
}

function sourceItems(logicalName, parsed) {
  if (logicalName === 'entities') return parsed.entities.entities.length;
  if (logicalName === 'properties') return parsed.properties.properties.length;
  if (logicalName === 'relationships') return parsed.relationships.relationships.length;
  return 1;
}

function receipts(order, bytesByName, itemsByName) {
  return order.map(([logicalName, fileName, mediaType]) => {
    const bytes = bytesByName[fileName];
    if (!Buffer.isBuffer(bytes)) snapshotError('reference-snapshot-source-invalid', `Artifact ${logicalName} is missing.`);
    return { logicalName, mediaType, bytes: bytes.length, items: itemsByName[logicalName], sha256: sha256(bytes) };
  });
}

function packageConfiguration(limits, v2) {
  return {
    schemaVersion: `model-reference-package-configuration/v${v2 ? '2' : '1'}`,
    partitionPolicy: 'single-canonical-glb-v1',
    canonicalOrdering: 'utf8-byte-order-v1',
    tileBoundaryDuplication: 'none',
    maximumTileBytes: limits.maxCanonicalGlbBytes,
    maximumTileTriangles: limits.maxIndices,
    maximumShardBytes: limits.maxComponentJsonBytes,
    maximumShardRecords: Math.max(limits.maxEntities, limits.maxParameters, limits.maxRelationships),
    maximumPackageArtifacts: 6,
    maximumAggregateBytes: limits.maxCanonicalGlbBytes + (limits.maxComponentJsonBytes * 5),
    supportedGlb: { version: '2.0', extensions: [], componentTypes: [5121, 5123, 5125, 5126] },
    schemas: {
      manifest: `floless.model-snapshot-package/v${v2 ? '2' : '1'}`,
      entities: v2 ? '2' : '1', properties: v2 ? 'aware.model-properties/v2' : '1',
      relationships: v2 ? '2' : '1', index: `floless.model-snapshot-index/v${v2 ? '2' : '1'}`,
    },
  };
}

function packagedBytes(result, parsed, sourceArtifactEnvelope, configuration, v2) {
  const entities = parsed.entities.entities;
  const index = canonicalJsonBytes({
    schemaVersion: `floless.model-snapshot-index/v${v2 ? '2' : '1'}`,
    entities: entities.map((entity, ordinal) => ({
      id: entity.id, ordinal, tiles: Array.isArray(entity.geometry) && entity.geometry.length > 0 ? ['tile-000000'] : [],
    })),
  });
  const components = {
    'tile-000000.glb': result.cache.artifacts['geometry.glb'],
    'entities-000000.json': result.cache.artifacts['entities.json'],
    'properties-000000.json': result.cache.artifacts['properties.json'],
    'relationships-000000.json': result.cache.artifacts['relationships.json'],
    'index.json': index,
  };
  const componentReceipts = receipts(PACKAGE_ORDER.slice(1), components, {
    'tile-000000': 1,
    'entities-000000': parsed.entities.entities.length,
    'properties-000000': parsed.properties.properties.length,
    'relationships-000000': parsed.relationships.relationships.length,
    index: entities.length,
  });
  const manifest = canonicalJsonBytes({
    schemaVersion: `floless.model-snapshot-package/v${v2 ? '2' : '1'}`,
    sourceArtifactEnvelopeSha256: sha256(canonicalJsonBytes(sourceArtifactEnvelope)),
    configurationSha256: sha256(canonicalJsonBytes(configuration)),
    frame: result.cache.manifest.frame,
    coverage: result.cache.manifest.coverage,
    outputs: componentReceipts,
    conservation: {
      sourceGeometryArtifacts: 1, packagedGeometryTiles: 1,
      entities: parsed.entities.entities.length,
      properties: parsed.properties.properties.length,
      relationships: parsed.relationships.relationships.length,
      unclaimedGeometryNodes: result.cache.manifest.coverage.unclaimedGeometryNodes,
    },
  });
  for (const [name, bytes] of [['snapshot manifest', manifest], ['entity index', index]]) {
    if (bytes.length > configuration.maximumShardBytes) {
      snapshotError('reference-output-too-large', `${name} exceeds its package byte limit.`);
    }
  }
  return { 'snapshot-manifest.json': manifest, ...components };
}

async function publishArtifacts(result, directory, sourceBytes, packageBytes) {
  if (typeof directory !== 'string' || !path.isAbsolute(directory)) {
    snapshotError('reference-artifact-directory-missing', 'A run-owned artifact directory is required.');
  }
  await fs.mkdir(directory, { recursive: true, mode: 0o700 });
  const descriptors = { artifacts: {}, packageArtifacts: {} };
  for (const [target, order, bytesByName, prefix] of [
    [descriptors.artifacts, SOURCE_ORDER, sourceBytes, 'model'],
    [descriptors.packageArtifacts, PACKAGE_ORDER, packageBytes, 'snapshot'],
  ]) {
    for (const [logicalName, fileName, mediaType] of order) {
      const bytes = bytesByName[fileName];
      const digest = sha256(bytes);
      const id = `${prefix}-${result.key.slice(0, 16)}-${fileName}`;
      const output = path.join(directory, id);
      try { await fs.writeFile(output, bytes, { flag: 'wx', mode: 0o600 }); }
      catch (error) {
        if (error?.code !== 'EEXIST' || sha256(await fs.readFile(output)) !== digest) {
          snapshotError('reference-artifact-collision', 'A run artifact id collided with different bytes.', error);
        }
      }
      target[logicalName] = { id, mediaType, bytes: bytes.length, sha256: digest };
    }
  }
  return descriptors;
}

export async function buildAndPublishSnapshot(result, signingKey, artifactDirectory, options = {}) {
  if (!result.cache.receiptSha256) snapshotError('reference-cache-authentication-missing', 'The private cache receipt was not authenticated.');
  const limits = lowerableLimits(options.limits);
  const readerSchemaVersion = result.cache.manifest.identity?.canonicalRequest?.readerSchemaVersion ?? 'model-reference-reader/v1';
  const v2 = readerSchemaVersion === 'model-reference-reader/v2';
  if (!v2 && readerSchemaVersion !== 'model-reference-reader/v1') {
    snapshotError('reference-snapshot-source-invalid', 'The reader schema version is unsupported.');
  }
  const artifactSchemaVersion = v2 ? '2' : '1';
  const parsed = {
    entities: document(result.cache.artifacts['entities.json'], 'entities', limits, artifactSchemaVersion),
    properties: document(result.cache.artifacts['properties.json'], 'properties', limits, artifactSchemaVersion),
    relationships: document(result.cache.artifacts['relationships.json'], 'relationships', limits, artifactSchemaVersion),
  };
  const sourceReceipts = receipts(SOURCE_ORDER, result.cache.artifacts, {
    geometry: 1, entities: sourceItems('entities', parsed), properties: sourceItems('properties', parsed),
    relationships: sourceItems('relationships', parsed), manifest: 1,
  });
  const identity = result.cache.manifest.identity;
  const sourceArtifactPreimage = {
    schemaVersion: artifactSchemaVersion,
    source: {
      sourceSha256: identity.sourceSha256,
      canonicalRequestSha256: result.cache.manifest.canonicalRequestSha256,
      providerFingerprintSha256: result.cache.manifest.providerFingerprintSha256,
      signerFingerprintSha256: identity.signerFingerprintSha256,
      privateCacheReceiptSha256: result.cache.receiptSha256,
    },
    outputs: sourceReceipts,
  };
  const sourceArtifactEnvelope = signArtifactPreimage(v2 ? SOURCE_ARTIFACT_DOMAIN_V2 : SOURCE_ARTIFACT_DOMAIN, sourceArtifactPreimage, signingKey);
  const configuration = packageConfiguration(limits, v2);
  const configurationSha256 = sha256(canonicalJsonBytes(configuration));
  const packageBytes = packagedBytes(result, parsed, sourceArtifactEnvelope, configuration, v2);
  const aggregateBytes = Object.values(packageBytes).reduce((sum, bytes) => sum + bytes.length, 0);
  if (aggregateBytes > configuration.maximumAggregateBytes) {
    snapshotError('reference-output-too-large', 'Snapshot package exceeds its aggregate byte limit.');
  }
  const packageReceipts = receipts(PACKAGE_ORDER, packageBytes, {
    manifest: 1, 'tile-000000': 1, 'entities-000000': parsed.entities.entities.length,
    'properties-000000': parsed.properties.properties.length,
    'relationships-000000': parsed.relationships.relationships.length, index: parsed.entities.entities.length,
  });
  const packagePreimage = {
    schemaVersion: artifactSchemaVersion,
    source: {
      sourceArtifactPreimageSha256: sourceArtifactEnvelope.preimageSha256,
      sourceArtifactEnvelopeSha256: sha256(canonicalJsonBytes(sourceArtifactEnvelope)),
      receipts: sourceReceipts,
      sourceSha256: identity.sourceSha256,
      canonicalRequestSha256: result.cache.manifest.canonicalRequestSha256,
      providerFingerprintSha256: result.cache.manifest.providerFingerprintSha256,
      signerFingerprintSha256: identity.signerFingerprintSha256,
    },
    packager: {
      agent: 'model-reference-reader', version: v2 ? '0.5.0' : '0.2.0',
      bridgeBuildId: v2 ? 'aware-connection-reader@0.3.0' : 'aware-connection-reader@0.2.0', configurationSha256,
    },
    outputs: packageReceipts,
  };
  const packageArtifactEnvelope = signArtifactPreimage(v2 ? PACKAGE_ARTIFACT_DOMAIN_V2 : PACKAGE_ARTIFACT_DOMAIN, packagePreimage, signingKey);
  const descriptors = await publishArtifacts(result, artifactDirectory, result.cache.artifacts, packageBytes);
  return {
    ...descriptors, sourceArtifactPreimage, sourceArtifactEnvelope,
    packageConfiguration: configuration, packagePreimage, packageArtifactEnvelope,
  };
}
