import {
  assertClosedObject, assertSha256, canonicalJsonBytes, ModelReaderError, sha256,
} from './model-contract.mjs';

const ROOT_SCHEMA = 'model-reference-manifest/v2';
const INDEX_SCHEMA = 'aware.model-artifact-index/v2';
const FAMILIES = Object.freeze({
  geometry: {
    kind: 'geometry-tile', mediaType: 'model/gltf-binary', pattern: /^geometry\/(\d{6})\.glb$/,
  },
  entities: {
    kind: 'entities-shard', mediaType: 'application/json', pattern: /^metadata\/entities-(\d{6})\.json$/,
  },
  properties: {
    kind: 'properties-shard', mediaType: 'application/json', pattern: /^metadata\/properties-(\d{6})\.json$/,
  },
  relationships: {
    kind: 'relationships-shard', mediaType: 'application/json', pattern: /^metadata\/relationships-(\d{6})\.json$/,
  },
});
const OPAQUE_ID = /^[A-Za-z0-9._-]{1,128}$/;
const LOGICAL_PATH = /^(?:[A-Za-z0-9._-]+\/)*[A-Za-z0-9._-]+$/;

function artifactError(code, message, details = undefined) {
  throw new ModelReaderError(code, 'canonical-artifact', false, message, details);
}

function closed(value, required, optional, label) {
  try { return assertClosedObject(value, required, optional, label); }
  catch (error) { artifactError('reference-artifact-v2-invalid', `${label} is invalid.`, error); }
}

function digest(value, label) {
  try { return assertSha256(value, label); }
  catch (error) { artifactError('reference-artifact-v2-invalid', `${label} is invalid.`, error); }
}

function opaque(value, label) {
  if (typeof value !== 'string' || !OPAQUE_ID.test(value)) {
    artifactError('reference-artifact-v2-invalid', `${label} is invalid.`);
  }
  return value;
}

function integer(value, label, maximum = Number.MAX_SAFE_INTEGER) {
  if (!Number.isSafeInteger(value) || value < 0 || value > maximum) {
    artifactError('reference-artifact-v2-invalid', `${label} is invalid.`);
  }
  return value;
}

function bounds(value) {
  if (!Array.isArray(value) || value.length !== 6
      || value.some((entry) => typeof entry !== 'number' || !Number.isFinite(entry))
      || value[0] > value[3] || value[1] > value[4] || value[2] > value[5]) {
    artifactError('reference-artifact-v2-invalid', 'artifact bounds are invalid.');
  }
  try { canonicalJsonBytes(value); }
  catch (error) { artifactError('reference-artifact-v2-invalid', 'artifact bounds are invalid.', error); }
  return value;
}

function idRange(value) {
  closed(value, ['first', 'last'], [], 'artifact ID range');
  if (typeof value.first !== 'string' || !value.first || typeof value.last !== 'string' || !value.last
      || Buffer.compare(Buffer.from(value.first), Buffer.from(value.last)) > 0) {
    artifactError('reference-artifact-v2-invalid', 'artifact ID range is invalid.');
  }
  return value;
}

function validateReceipt(value, label = 'artifact receipt') {
  closed(value, [
    'logicalPath', 'logicalKind', 'ordinal', 'mediaType', 'bytes', 'sha256', 'itemCount',
  ], ['bounds', 'idRange'], label);
  if (typeof value.logicalPath !== 'string' || !LOGICAL_PATH.test(value.logicalPath)
      || typeof value.logicalKind !== 'string' || !value.logicalKind
      || typeof value.mediaType !== 'string' || !value.mediaType) {
    artifactError('reference-artifact-v2-invalid', `${label} names are invalid.`);
  }
  integer(value.ordinal, `${label} ordinal`, 255);
  integer(value.bytes, `${label} bytes`);
  integer(value.itemCount, `${label} itemCount`);
  digest(value.sha256, `${label} sha256`);
  if (value.bounds !== undefined) bounds(value.bounds);
  if (value.idRange !== undefined) idRange(value.idRange);
  try { canonicalJsonBytes(value); }
  catch (error) { artifactError('reference-artifact-v2-invalid', `${label} is not canonical JSON data.`, error); }
  return value;
}

export function artifactV2Receipt(options) {
  if (!options || typeof options !== 'object' || Array.isArray(options)
      || !(Buffer.isBuffer(options.content) || options.content instanceof Uint8Array)) {
    artifactError('reference-artifact-v2-invalid', 'Artifact receipt input is invalid.');
  }
  const content = Buffer.from(options.content);
  return validateReceipt({
    logicalPath: options.logicalPath,
    logicalKind: options.logicalKind,
    ordinal: options.ordinal,
    mediaType: options.mediaType,
    bytes: content.length,
    sha256: sha256(content),
    itemCount: options.itemCount,
    ...(options.bounds === undefined ? {} : { bounds: options.bounds }),
    ...(options.idRange === undefined ? {} : { idRange: options.idRange }),
  });
}

export function buildArtifactV2Index(family, payloadReceipts) {
  const contract = typeof family === 'string' && Object.hasOwn(FAMILIES, family)
    ? FAMILIES[family] : undefined;
  if (!contract || !Array.isArray(payloadReceipts) || payloadReceipts.length > 256
      || ((family === 'geometry' || family === 'entities') && payloadReceipts.length === 0)) {
    artifactError('reference-artifact-v2-invalid', 'Artifact index input is invalid.');
  }
  const objects = payloadReceipts.map((receipt) => ({ ...validateReceipt(receipt) }))
    .sort((left, right) => left.ordinal - right.ordinal);
  let itemCount = 0;
  for (let ordinal = 0; ordinal < objects.length; ordinal += 1) {
    const object = objects[ordinal];
    const match = contract.pattern.exec(object.logicalPath);
    if (!match || Number(match[1]) !== ordinal || object.ordinal !== ordinal
        || object.logicalKind !== contract.kind || object.mediaType !== contract.mediaType) {
      artifactError('reference-artifact-v2-invalid', `The ${family} artifact sequence is invalid.`);
    }
    itemCount += object.itemCount;
    if (!Number.isSafeInteger(itemCount)) {
      artifactError('reference-artifact-v2-invalid', `The ${family} item count is invalid.`);
    }
  }
  const index = { schemaVersion: INDEX_SCHEMA, family, itemCount, objects };
  const bytes = canonicalJsonBytes(index);
  const receipt = artifactV2Receipt({
    logicalPath: `${family}.index.json`, logicalKind: `${family}-index`, ordinal: 0,
    mediaType: 'application/json', content: bytes, itemCount,
  });
  return { index, bytes, receipt };
}

function validateIndex(value, family) {
  closed(value, ['index', 'bytes', 'receipt'], [], `${family} index package`);
  closed(value.index, ['schemaVersion', 'family', 'itemCount', 'objects'], [], `${family} index`);
  if (!Buffer.isBuffer(value.bytes) || !value.bytes.equals(canonicalJsonBytes(value.index))
      || value.index.schemaVersion !== INDEX_SCHEMA || value.index.family !== family
      || value.receipt.sha256 !== sha256(value.bytes) || value.receipt.bytes !== value.bytes.length
      || value.receipt.logicalPath !== `${family}.index.json`
      || value.receipt.logicalKind !== `${family}-index` || value.receipt.mediaType !== 'application/json'
      || value.receipt.itemCount !== value.index.itemCount) {
    artifactError('reference-artifact-v2-invalid', `The ${family} index package is invalid.`);
  }
  validateReceipt(value.receipt, `${family} index receipt`);
  const rebuilt = buildArtifactV2Index(family, value.index.objects);
  if (!rebuilt.bytes.equals(value.bytes)
      || !canonicalJsonBytes(rebuilt.receipt).equals(canonicalJsonBytes(value.receipt))) {
    artifactError('reference-artifact-v2-invalid', `The ${family} index package is not canonical.`);
  }
  return rebuilt;
}

export function buildArtifactV2Root(options) {
  if (!options || typeof options !== 'object' || Array.isArray(options)) {
    artifactError('reference-artifact-v2-invalid', 'Artifact root input is invalid.');
  }
  const formatId = opaque(options.formatId, 'formatId');
  const capabilityId = opaque(options.capabilityId, 'capabilityId');
  const providerPackageManifestSha256 = digest(
    options.providerPackageManifestSha256, 'providerPackageManifestSha256',
  );
  const effectiveSourceSha256 = digest(options.effectiveSourceSha256, 'effectiveSourceSha256');
  const conversionRequestSha256 = digest(options.conversionRequestSha256, 'conversionRequestSha256');
  if (!['complete', 'degraded'].includes(options.completeness)) {
    artifactError('reference-artifact-v2-invalid', 'Artifact completeness is invalid.');
  }
  const effectiveSource = validateReceipt(options.effectiveSource, 'effective-source receipt');
  if (effectiveSource.logicalPath !== 'effective-source.json'
      || effectiveSource.logicalKind !== 'effective-source' || effectiveSource.ordinal !== 0
      || effectiveSource.mediaType !== 'application/json' || effectiveSource.itemCount !== 1
      || effectiveSource.sha256 !== effectiveSourceSha256) {
    artifactError('reference-artifact-v2-invalid', 'The effective-source receipt is invalid.');
  }
  const indexes = {};
  const objects = [effectiveSource];
  for (const family of Object.keys(FAMILIES)) {
    const rebuilt = validateIndex(options.indexes?.[family], family);
    indexes[family] = rebuilt.receipt;
    objects.push(rebuilt.receipt, ...rebuilt.index.objects);
  }
  const paths = new Set(); const receipts = new Set();
  for (const object of objects) {
    const receiptKey = `${object.sha256}\0${object.bytes}\0${object.mediaType}\0${object.logicalPath}`;
    if (paths.has(object.logicalPath) || receipts.has(receiptKey)) {
      artifactError('reference-artifact-v2-invalid', 'Canonical artifact objects must be unique.');
    }
    paths.add(object.logicalPath); receipts.add(receiptKey);
  }
  objects.sort((left, right) => Buffer.compare(Buffer.from(left.logicalPath), Buffer.from(right.logicalPath)));
  const manifest = {
    schemaVersion: ROOT_SCHEMA, artifactVersion: '2', formatId, capabilityId,
    providerPackageManifestSha256, effectiveSourceSha256, conversionRequestSha256,
    completeness: options.completeness, indexes, objects,
  };
  const bytes = canonicalJsonBytes(manifest);
  return { manifest, bytes, sha256: sha256(bytes) };
}
