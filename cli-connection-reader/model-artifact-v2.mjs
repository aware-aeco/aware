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

function denseArray(value) {
  return Array.isArray(value)
    && Array.from({ length: value.length }, (_, index) => Object.hasOwn(value, index)).every(Boolean);
}

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
  if (!denseArray(value) || value.length !== 6
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
  if (!contract || !denseArray(payloadReceipts) || payloadReceipts.length > 256
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
  validateReceipt(value.receipt, `${family} index receipt`);
  let suppliedIndexBytes;
  try { suppliedIndexBytes = canonicalJsonBytes(value.index); }
  catch (error) { artifactError('reference-artifact-v2-invalid', `The ${family} index is not canonical JSON data.`, error); }
  if (!Buffer.isBuffer(value.bytes) || !value.bytes.equals(suppliedIndexBytes)
      || value.index.schemaVersion !== INDEX_SCHEMA || value.index.family !== family
      || value.receipt.sha256 !== sha256(value.bytes) || value.receipt.bytes !== value.bytes.length
      || value.receipt.logicalPath !== `${family}.index.json`
      || value.receipt.logicalKind !== `${family}-index` || value.receipt.mediaType !== 'application/json'
      || value.receipt.itemCount !== value.index.itemCount) {
    artifactError('reference-artifact-v2-invalid', `The ${family} index package is invalid.`);
  }
  const rebuilt = buildArtifactV2Index(family, value.index.objects);
  if (!rebuilt.bytes.equals(value.bytes)
      || !canonicalJsonBytes(rebuilt.receipt).equals(canonicalJsonBytes(value.receipt))) {
    artifactError('reference-artifact-v2-invalid', `The ${family} index package is not canonical.`);
  }
  return rebuilt;
}

function validateEffectiveSource(value, expected) {
  closed(value, ['manifest', 'bytes', 'receipt'], [], 'effective-source package');
  closed(value.manifest, [
    'schemaVersion', 'formatId', 'protocolVersion', 'capabilityId', 'providerFingerprintSha256',
    'providerPackageManifestSha256', 'discoveryPolicy', 'completeness', 'primary', 'consumed',
    'absent', 'unsupportedExternal', 'authentication', 'crossFileEvidence',
  ], [], 'effective-source manifest');
  closed(value.manifest.discoveryPolicy, ['policyId', 'sha256'], [], 'effective-source discovery policy');
  opaque(value.manifest.discoveryPolicy.policyId, 'effective-source policyId');
  digest(value.manifest.discoveryPolicy.sha256, 'effective-source policy sha256');
  let canonicalBytes;
  try { canonicalBytes = canonicalJsonBytes(value.manifest); }
  catch (error) { artifactError('reference-artifact-v2-invalid', 'The effective-source manifest is not canonical JSON data.', error); }
  if (!Buffer.isBuffer(value.bytes) || !value.bytes.equals(canonicalBytes)
      || sha256(value.bytes) !== expected.sha256
      || value.manifest.schemaVersion !== 'model-effective-source/v2'
      || value.manifest.formatId !== expected.formatId
      || value.manifest.capabilityId !== expected.capabilityId
      || value.manifest.providerPackageManifestSha256 !== expected.providerPackageManifestSha256
      || !['complete', 'degraded'].includes(value.manifest.completeness)
      || !denseArray(value.manifest.absent) || value.manifest.absent.length > 10_000) {
    artifactError('reference-artifact-v2-invalid', 'The effective-source package is invalid.');
  }
  const absent = value.manifest.absent.map((entry) => {
    closed(entry, ['role', 'classification', 'affectedDomains'], [], 'effective-source absence');
    opaque(entry.role, 'effective-source absent role');
    if (!['optional', 'degraded'].includes(entry.classification)
        || !denseArray(entry.affectedDomains) || entry.affectedDomains.length > 64
        || entry.affectedDomains.some((domain) => typeof domain !== 'string' || !OPAQUE_ID.test(domain))
        || (entry.classification === 'degraded') !== (entry.affectedDomains.length > 0)) {
      artifactError('reference-artifact-v2-invalid', 'The effective-source absence is invalid.');
    }
    return {
      role: entry.role, classification: entry.classification,
      affectedDomains: [...entry.affectedDomains].sort(),
    };
  }).sort((left, right) => left.role.localeCompare(right.role, 'en'));
  if (new Set(absent.map((entry) => entry.role)).size !== absent.length
      || absent.some((entry, index) => !canonicalJsonBytes(entry).equals(canonicalJsonBytes(value.manifest.absent[index])))
      || (absent.some((entry) => entry.classification === 'degraded') ? 'degraded' : 'complete')
        !== value.manifest.completeness) {
    artifactError('reference-artifact-v2-invalid', 'The effective-source coverage is invalid.');
  }
  const receipt = validateReceipt(value.receipt, 'effective-source receipt');
  if (receipt.logicalPath !== 'effective-source.json'
      || receipt.logicalKind !== 'effective-source' || receipt.ordinal !== 0
      || receipt.mediaType !== 'application/json' || receipt.itemCount !== 1
      || receipt.sha256 !== expected.sha256 || receipt.bytes !== value.bytes.length) {
    artifactError('reference-artifact-v2-invalid', 'The effective-source receipt is invalid.');
  }
  return {
    receipt,
    completeness: value.manifest.completeness,
    discoveryPolicy: { ...value.manifest.discoveryPolicy },
    absent,
  };
}

export function buildArtifactV2Root(options) {
  if (!options || typeof options !== 'object' || Array.isArray(options)) {
    artifactError('reference-artifact-v2-invalid', 'Artifact root input is invalid.');
  }
  closed(options, [
    'formatId', 'capabilityId', 'providerPackageManifestSha256', 'effectiveSourceSha256',
    'conversionRequestSha256', 'effectiveSource', 'indexes',
  ], [], 'artifact root input');
  const formatId = opaque(options.formatId, 'formatId');
  const capabilityId = opaque(options.capabilityId, 'capabilityId');
  const providerPackageManifestSha256 = digest(
    options.providerPackageManifestSha256, 'providerPackageManifestSha256',
  );
  const effectiveSourceSha256 = digest(options.effectiveSourceSha256, 'effectiveSourceSha256');
  const conversionRequestSha256 = digest(options.conversionRequestSha256, 'conversionRequestSha256');
  const source = validateEffectiveSource(options.effectiveSource, {
    sha256: effectiveSourceSha256, formatId, capabilityId, providerPackageManifestSha256,
  });
  const effectiveSource = source.receipt;
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
    discoveryPolicy: source.discoveryPolicy, completeness: source.completeness,
    absent: source.absent, indexes, objects,
  };
  const bytes = canonicalJsonBytes(manifest);
  return { manifest, bytes, sha256: sha256(bytes) };
}
