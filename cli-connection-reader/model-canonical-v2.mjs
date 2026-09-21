import { createReadStream } from 'node:fs';
import fs from 'node:fs/promises';
import path from 'node:path';
import readline from 'node:readline';

import {
  assertClosedObject, canonicalJsonBytes, ModelReaderError, parseJsonStrict, sha256,
} from './model-contract.mjs';
import { artifactV2Receipt, buildArtifactV2Index, buildArtifactV2Root } from './model-artifact-v2.mjs';
import { signArtifactPreimage } from './model-artifact-auth.mjs';
import { externalSortMetadataRecords } from './model-metadata-sort.mjs';
import { validateGlbTile } from './model-glb-tile.mjs';

const FAMILIES = ['entities', 'properties', 'relationships'];
const DEFAULT_LIMITS = Object.freeze({
  shardBytes: 16 * 1024 * 1024,
  shardRecords: 5_000_000,
  recordBytes: 1024 * 1024,
  entityIdentityBytes: 128 * 1024 * 1024,
});

function canonicalError(code, message, details = undefined) {
  throw new ModelReaderError(code, 'canonical-artifact', false, message, details);
}

function closed(value, required, optional, label) {
  try { return assertClosedObject(value, required, optional, label); }
  catch (error) { canonicalError('reference-metadata-semantic-invalid', `${label} is invalid.`, error); }
}

function text(value, label, allowNull = false) {
  if (allowNull && value === null) return value;
  if (typeof value !== 'string' || !value || Buffer.byteLength(value) > 4096
      || /[\u0000-\u001f\u007f]/.test(value)) {
    canonicalError('reference-metadata-semantic-invalid', `${label} is invalid.`);
  }
  return value;
}

function dense(value, maximum, label) {
  if (!Array.isArray(value) || value.length > maximum
      || Array.from({ length: value.length }, (_, index) => Object.hasOwn(value, index)).some((entry) => !entry)) {
    canonicalError('reference-metadata-semantic-invalid', `${label} is invalid.`);
  }
  return value;
}

function bounds(value, label) {
  dense(value, 6, label);
  if (value.length !== 6 || value.some((entry) => typeof entry !== 'number' || !Number.isFinite(entry))
      || value[0] > value[3] || value[1] > value[4] || value[2] > value[5]) {
    canonicalError('reference-metadata-semantic-invalid', `${label} is invalid.`);
  }
  return value;
}

function normalizeEntity(value, tiles) {
  closed(value, ['id', 'type', 'name', 'geometry'], [], 'entity record');
  const id = text(value.id, 'entity id');
  const geometry = dense(value.geometry, 256, 'entity geometry').map((entry) => {
    closed(entry, ['tileOrdinal', 'bounds'], [], 'entity geometry ownership');
    if (!Number.isSafeInteger(entry.tileOrdinal) || entry.tileOrdinal < 0 || entry.tileOrdinal >= tiles.length) {
      canonicalError('reference-metadata-semantic-invalid', 'Entity geometry references a missing tile.');
    }
    const ownedBounds = bounds(entry.bounds, 'entity geometry bounds');
    const tileBounds = tiles[entry.tileOrdinal].bounds;
    if (ownedBounds.some((coordinate, index) => index < 3
      ? coordinate < tileBounds[index] : coordinate > tileBounds[index])) {
      canonicalError('reference-metadata-semantic-invalid', 'Entity geometry bounds escape their owning tile.');
    }
    return { tileOrdinal: entry.tileOrdinal, bounds: ownedBounds };
  });
  const ordinals = geometry.map((entry) => entry.tileOrdinal);
  if (new Set(ordinals).size !== ordinals.length
      || ordinals.some((ordinal, index) => index > 0 && ordinal <= ordinals[index - 1])) {
    canonicalError('reference-metadata-semantic-invalid', 'Entity geometry ownership must be unique and ordered.');
  }
  return { id, type: text(value.type, 'entity type'), name: text(value.name, 'entity name', true), geometry };
}

function normalizeProperty(value) {
  closed(value, [
    'id', 'entityId', 'name', 'normalizedName', 'value', 'valueType', 'unit', 'source',
    'status', 'provenance',
  ], [], 'property record');
  const id = text(value.id, 'property id');
  const entityId = text(value.entityId, 'property entity id');
  const name = text(value.name, 'property name');
  const normalizedName = text(value.normalizedName, 'normalized property name');
  if (normalizedName !== name.normalize('NFKC').toLocaleLowerCase('en-US')) {
    canonicalError('reference-metadata-semantic-invalid', 'Normalized property name does not match its name.');
  }
  if (!['string', 'number', 'boolean', 'null'].includes(value.valueType)
      || (value.valueType === 'null' ? value.value !== null : typeof value.value !== value.valueType)
      || (typeof value.value === 'number' && (!Number.isFinite(value.value)
        || (Number.isInteger(value.value) && !Number.isSafeInteger(value.value))))) {
    canonicalError('reference-metadata-semantic-invalid', 'Property value does not match its declared type.');
  }
  const record = {
    id, entityId, name, normalizedName, value: value.value, valueType: value.valueType,
    unit: text(value.unit, 'property unit', true), source: text(value.source, 'property source'),
    status: text(value.status, 'property status'), provenance: text(value.provenance, 'property provenance'),
  };
  canonicalJsonBytes(record);
  return record;
}

function normalizeRelationship(value) {
  closed(value, ['id', 'kind', 'from', 'to'], [], 'relationship record');
  return {
    id: text(value.id, 'relationship id'), kind: text(value.kind, 'relationship kind'),
    from: text(value.from, 'relationship source'), to: text(value.to, 'relationship target'),
  };
}

async function* jsonlRecords(root, files, limits, signal) {
  for (const file of files) {
    if (signal?.aborted) canonicalError('reference-cancelled', 'Canonicalization was cancelled.');
    const input = createReadStream(path.join(root, ...file.path.split('/')), { encoding: 'utf8' });
    const lines = readline.createInterface({ input, crlfDelay: Infinity });
    try {
      for await (const line of lines) {
        if (signal?.aborted) canonicalError('reference-cancelled', 'Canonicalization was cancelled.');
        const bytes = Buffer.byteLength(line);
        if (!line || bytes > limits.recordBytes) {
          canonicalError('reference-metadata-semantic-invalid', 'Provider metadata contains an invalid record.');
        }
        yield parseJsonStrict(Buffer.from(line), { maxBytes: limits.recordBytes, maxDepth: 64 });
      }
    } finally {
      lines.close(); input.destroy();
    }
  }
}

function admitIdentity(values, value, limits, label) {
  if (values.has(value)) canonicalError('reference-metadata-semantic-duplicate', `${label} IDs must be unique.`);
  values.add(value);
  values.identityBytes = (values.identityBytes ?? 0) + Buffer.byteLength(value);
  if (values.identityBytes > limits.entityIdentityBytes) {
    canonicalError('reference-artifact-v2-limit', 'Metadata identity indexes exceed their memory limit.');
  }
}

async function* semanticRecords(family, output, tiles, entities, propertyIds, relationshipIds, limits, signal) {
  const files = output.files.filter((entry) => entry.kind === family)
    .sort((left, right) => left.ordinal - right.ordinal);
  for await (const input of jsonlRecords(output.root, files, limits, signal)) {
    let record; let key;
    if (family === 'entities') {
      record = normalizeEntity(input, tiles); key = record.id;
      admitIdentity(entities, record.id, limits, 'Entity');
    } else if (family === 'properties') {
      record = normalizeProperty(input);
      if (!entities.has(record.entityId)) canonicalError('reference-metadata-semantic-dangling', 'Property references a missing entity.');
      admitIdentity(propertyIds, record.id, limits, 'Property');
      key = canonicalJsonBytes([record.entityId, record.normalizedName, record.id]).toString('utf8');
    } else {
      record = normalizeRelationship(input);
      if (!entities.has(record.from) || !entities.has(record.to)) {
        canonicalError('reference-metadata-semantic-dangling', 'Relationship references a missing entity.');
      }
      admitIdentity(relationshipIds, record.id, limits, 'Relationship');
      key = canonicalJsonBytes([record.kind, record.from, record.to, record.id]).toString('utf8');
    }
    yield { key, record };
  }
}

function frame(family, records) {
  return canonicalJsonBytes({ family, records, schemaVersion: 'aware.model-metadata-shard/v2' });
}

async function partitionSorted(family, sorted, canonicalRoot, limits, signal) {
  const emptyFrameBytes = frame(family, []).length;
  const shards = []; let records = []; let recordsBytes = 0; let firstKey; let lastKey; let seen = 0;
  const flush = async () => {
    if (!records.length) return;
    const ordinal = shards.length;
    const bytes = frame(family, records);
    const logicalPath = `metadata/${family}-${String(ordinal).padStart(6, '0')}.json`;
    const pathname = path.join(canonicalRoot, ...logicalPath.split('/'));
    await fs.mkdir(path.dirname(pathname), { recursive: true });
    await fs.writeFile(pathname, bytes, { flag: 'wx', mode: 0o400 });
    const receipt = artifactV2Receipt({
      logicalPath, logicalKind: `${family}-shard`, ordinal, mediaType: 'application/json',
      content: bytes, itemCount: records.length,
      idRange: { first: firstKey, last: lastKey },
    });
    shards.push({ pathname, receipt });
    records = []; recordsBytes = 0; firstKey = undefined; lastKey = undefined;
  };
  const input = createReadStream(sorted.pathname, { encoding: 'utf8' });
  const lines = readline.createInterface({ input, crlfDelay: Infinity });
  try {
    for await (const line of lines) {
      if (signal?.aborted) canonicalError('reference-cancelled', 'Canonicalization was cancelled.');
      const entry = parseJsonStrict(Buffer.from(line), { maxBytes: limits.recordBytes, maxDepth: 65 });
      const recordBytes = canonicalJsonBytes(entry.record).length;
      const candidateBytes = emptyFrameBytes + recordsBytes + recordBytes + records.length;
      if (records.length
          && (records.length >= limits.shardRecords || candidateBytes > limits.shardBytes)) {
        await flush();
      }
      records.push(entry.record); recordsBytes += recordBytes;
      firstKey ??= entry.key; lastKey = entry.key; seen += 1;
      if (emptyFrameBytes + recordsBytes + Math.max(0, records.length - 1) > limits.shardBytes) {
        canonicalError('reference-artifact-v2-limit', 'One metadata record exceeds its shard byte limit.');
      }
    }
    await flush();
  } finally {
    lines.close(); input.destroy();
  }
  if (family === 'entities' && seen === 0) {
    canonicalError('reference-metadata-semantic-invalid', 'A canonical model requires at least one entity.');
  }
  return { shards, index: buildArtifactV2Index(family, shards.map((entry) => entry.receipt)) };
}

export async function canonicalizeProviderOutput(options) {
  const limits = { ...DEFAULT_LIMITS, ...(options.limits ?? {}) };
  const canonicalRoot = path.join(options.workRoot, 'canonical');
  await fs.mkdir(canonicalRoot, { mode: 0o700 });
  const geometryFiles = options.output.files.filter((entry) => entry.kind === 'geometry')
    .sort((left, right) => left.ordinal - right.ordinal);
  const geometry = [];
  for (const entry of geometryFiles) {
    if (!Array.isArray(entry.bounds)) canonicalError('reference-metadata-semantic-invalid', 'Geometry tile receipt requires bounds.');
    const content = await fs.readFile(path.join(options.output.root, ...entry.path.split('/')));
    const admittedBounds = bounds(entry.bounds, 'tile bounds');
    const derivedBounds = validateGlbTile(content, { limits: options.limits });
    if (derivedBounds.some((coordinate, index) => coordinate !== admittedBounds[index])) {
      canonicalError('reference-geometry-invalid', 'Geometry tile bounds do not match its POSITION data.');
    }
    const receipt = artifactV2Receipt({
      logicalPath: entry.path, logicalKind: 'geometry-tile', ordinal: entry.ordinal,
      mediaType: 'model/gltf-binary', content, itemCount: entry.count, bounds: admittedBounds,
    });
    geometry.push({ pathname: path.join(options.output.root, ...entry.path.split('/')), receipt });
  }
  const indexes = { geometry: buildArtifactV2Index('geometry', geometry.map((entry) => entry.receipt)) };
  const entities = new Set(); const propertyIds = new Set(); const relationshipIds = new Set(); const metadata = {};
  for (const family of FAMILIES) {
    const sorted = await externalSortMetadataRecords(
      semanticRecords(family, options.output, geometry.map((entry) => entry.receipt), entities,
        propertyIds, relationshipIds, limits, options.signal),
      { tempParent: options.workRoot, signal: options.signal },
    );
    try {
      metadata[family] = await partitionSorted(family, sorted, canonicalRoot, limits, options.signal);
      indexes[family] = metadata[family].index;
    } finally {
      await fs.rm(sorted.root, { recursive: true, force: true });
    }
  }
  const sourceBytes = canonicalJsonBytes(options.effectiveSource);
  const sourcePath = path.join(canonicalRoot, 'effective-source.json');
  await fs.writeFile(sourcePath, sourceBytes, { flag: 'wx', mode: 0o400 });
  const effectiveSource = {
    manifest: options.effectiveSource, bytes: sourceBytes,
    receipt: artifactV2Receipt({ logicalPath: 'effective-source.json', logicalKind: 'effective-source',
      ordinal: 0, mediaType: 'application/json', content: sourceBytes, itemCount: 1 }),
  };
  const root = buildArtifactV2Root({
    formatId: options.formatId, capabilityId: options.capabilityId,
    providerPackageManifestSha256: options.providerPackageManifestSha256,
    effectiveSourceSha256: sha256(sourceBytes), conversionRequestSha256: options.conversionRequestSha256,
    effectiveSource, indexes,
  });
  return {
    root, effectiveSource: { pathname: sourcePath, receipt: effectiveSource.receipt }, indexes,
    objects: [{ pathname: sourcePath, receipt: effectiveSource.receipt }, ...geometry,
      ...FAMILIES.flatMap((family) => metadata[family].shards)],
  };
}

async function publishOne(directory, logicalPath, bytes, digest) {
  const extension = path.extname(logicalPath);
  const id = `model-v2-${digest}${extension}`;
  const target = path.join(directory, id);
  try { await fs.writeFile(target, bytes, { flag: 'wx', mode: 0o600 }); }
  catch (error) {
    if (error?.code !== 'EEXIST' || sha256(await fs.readFile(target)) !== digest) {
      canonicalError('reference-artifact-collision', 'A canonical artifact collided with different bytes.', error);
    }
  }
  return { id, sha256: digest, bytes: bytes.length };
}

export async function publishCanonicalArtifact(canonical, signingKey, directory) {
  if (typeof directory !== 'string' || !path.isAbsolute(directory)) {
    canonicalError('reference-artifact-directory-missing', 'A run-owned artifact directory is required.');
  }
  await fs.mkdir(directory, { recursive: true, mode: 0o700 });
  const objects = {};
  for (const object of canonical.objects) {
    const bytes = await fs.readFile(object.pathname);
    if (bytes.length !== object.receipt.bytes || sha256(bytes) !== object.receipt.sha256) {
      canonicalError('reference-artifact-v2-invalid', 'A canonical artifact changed before publication.');
    }
    objects[object.receipt.logicalPath] = {
      ...await publishOne(directory, object.receipt.logicalPath, bytes, object.receipt.sha256),
      mediaType: object.receipt.mediaType, itemCount: object.receipt.itemCount,
    };
  }
  for (const index of Object.values(canonical.indexes)) {
    objects[index.receipt.logicalPath] = {
      ...await publishOne(directory, index.receipt.logicalPath, index.bytes, index.receipt.sha256),
      mediaType: index.receipt.mediaType, itemCount: index.receipt.itemCount,
    };
  }
  const manifest = await publishOne(
    directory, 'model-reference-manifest.json', canonical.root.bytes, canonical.root.sha256,
  );
  const preimage = {
    schemaVersion: 'model-reference-root-signature-preimage/v2',
    manifestSha256: canonical.root.sha256,
    objectReceipts: canonical.root.manifest.objects,
  };
  const envelope = signArtifactPreimage(
    'AWARE\0model-reference-reader\0artifact-root\0v2\0', preimage, signingKey,
  );
  const envelopeBytes = canonicalJsonBytes(envelope);
  const envelopeDescriptor = await publishOne(
    directory, 'model-reference-manifest.sig.json', envelopeBytes, sha256(envelopeBytes),
  );
  return {
    artifactRoot: { ...manifest, mediaType: 'application/json' },
    artifactRootEnvelope: { ...envelopeDescriptor, mediaType: 'application/json' },
    artifactRootSha256: canonical.root.sha256, artifactRootSignature: envelope, objects,
  };
}
