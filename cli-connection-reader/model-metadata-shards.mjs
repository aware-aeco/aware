import { canonicalJsonBytes, ModelReaderError } from './model-contract.mjs';
import { artifactV2Receipt, buildArtifactV2Index } from './model-artifact-v2.mjs';

const SCHEMA = 'aware.model-metadata-shard/v2';
const FAMILIES = new Set(['entities', 'properties', 'relationships']);
const DEFAULT_LIMITS = Object.freeze({ shardBytes: 16 * 1024 * 1024, shardRecords: 5_000_000 });
const HARD_LIMITS = Object.freeze({ shardBytes: 32 * 1024 * 1024, shardRecords: 10_000_000 });
const MAX_FAMILY_RECORDS = 10_000_000;

function shardError(code, message, details = undefined) {
  throw new ModelReaderError(code, 'canonical-artifact', false, message, details);
}

function limits(overrides = {}) {
  let entries; let prototype;
  try {
    prototype = overrides && typeof overrides === 'object' && !Array.isArray(overrides)
      ? Object.getPrototypeOf(overrides) : undefined;
    entries = prototype === Object.prototype || prototype === null ? Object.entries(overrides) : [];
  } catch (error) {
    shardError('reference-artifact-v2-limit-invalid', 'Metadata shard limits are invalid.', error);
  }
  if (prototype !== Object.prototype && prototype !== null) {
    shardError('reference-artifact-v2-limit-invalid', 'Metadata shard limits are invalid.');
  }
  const result = { ...DEFAULT_LIMITS };
  for (const [name, value] of entries) {
    if (!Object.hasOwn(result, name) || !Number.isSafeInteger(value) || value <= 0
        || value > HARD_LIMITS[name]) {
      shardError('reference-artifact-v2-limit-invalid', 'Metadata shard limits are invalid.');
    }
    result[name] = value;
  }
  return result;
}

export function canonicalMetadataRecord(entry) {
  let entryKeys; let hasKey; let hasRecord; let key; let record; let recordPrototype;
  try {
    entryKeys = entry && typeof entry === 'object' && !Array.isArray(entry) ? Object.keys(entry) : [];
    hasKey = entryKeys.length > 0 && Object.hasOwn(entry, 'key');
    hasRecord = entryKeys.length > 0 && Object.hasOwn(entry, 'record');
    key = hasKey ? entry.key : undefined;
    record = hasRecord ? entry.record : undefined;
    recordPrototype = record && typeof record === 'object' && !Array.isArray(record)
      ? Object.getPrototypeOf(record) : undefined;
  } catch (error) {
    shardError('reference-artifact-v2-invalid', 'A metadata record is invalid.', error);
  }
  if (entryKeys.length !== 2 || !hasKey || !hasRecord || typeof key !== 'string' || !key
      || (recordPrototype !== Object.prototype && recordPrototype !== null)) {
    shardError('reference-artifact-v2-invalid', 'A metadata record is invalid.');
  }
  let keyBytes; let recordBytes;
  try {
    canonicalJsonBytes(key);
    keyBytes = Buffer.from(key);
    recordBytes = canonicalJsonBytes(record);
  } catch (error) {
    shardError('reference-artifact-v2-invalid', 'A metadata record is not canonical JSON data.', error);
  }
  return { key, keyBytes, record: JSON.parse(recordBytes.toString('utf8')), recordBytes };
}

function frame(family, recordBytes) {
  const prefix = Buffer.from(`{"family":${JSON.stringify(family)},"records":[`);
  const suffix = Buffer.from(`],"schemaVersion":${JSON.stringify(SCHEMA)}}`);
  const pieces = [prefix];
  recordBytes.forEach((bytes, index) => {
    if (index) pieces.push(Buffer.from(','));
    pieces.push(bytes);
  });
  pieces.push(suffix);
  return Buffer.concat(pieces);
}

function logicalName(family, ordinal) {
  return `metadata/${family}-${String(ordinal).padStart(6, '0')}.json`;
}

export function partitionMetadataRecords(family, orderedRecords, options = {}) {
  let optionKeys; let recordCount;
  try {
    optionKeys = options && typeof options === 'object' && !Array.isArray(options)
      ? Object.keys(options) : [];
    recordCount = Array.isArray(orderedRecords) ? orderedRecords.length : -1;
  } catch (error) {
    shardError('reference-artifact-v2-invalid', 'Metadata shard input is invalid.', error);
  }
  if (!FAMILIES.has(family) || recordCount < 0 || optionKeys.some((key) => key !== 'limits')
      || (!options || typeof options !== 'object' || Array.isArray(options))
      || recordCount > MAX_FAMILY_RECORDS) {
    shardError('reference-artifact-v2-invalid', 'Metadata shard input is invalid.');
  }
  for (let index = 0; index < recordCount; index += 1) {
    let present;
    try { present = Object.hasOwn(orderedRecords, index); }
    catch (error) { shardError('reference-artifact-v2-invalid', 'Metadata shard input is invalid.', error); }
    if (!present) {
      shardError('reference-artifact-v2-invalid', 'Metadata shard input is invalid.');
    }
  }
  let limitInput;
  try { limitInput = options.limits; }
  catch (error) { shardError('reference-artifact-v2-limit-invalid', 'Metadata shard limits are invalid.', error); }
  const enforced = limits(limitInput);
  if (recordCount === 0) {
    if (family === 'entities') {
      shardError('reference-artifact-v2-invalid', 'A canonical model requires at least one entity.');
    }
    return { shards: [], index: buildArtifactV2Index(family, []) };
  }
  const emptyBytes = frame(family, []);
  if (emptyBytes.length > enforced.shardBytes) {
    shardError('reference-artifact-v2-limit', 'The metadata shard envelope exceeds its byte limit.');
  }
  const records = [];
  let previous;
  for (let index = 0; index < recordCount; index += 1) {
    let input;
    try { input = orderedRecords[index]; }
    catch (error) { shardError('reference-artifact-v2-invalid', 'A metadata record is invalid.', error); }
    const record = canonicalMetadataRecord(input);
    const comparison = previous ? Buffer.compare(previous.keyBytes, record.keyBytes) : -1;
    if (comparison >= 0) {
      shardError(
        comparison === 0 ? 'reference-artifact-v2-duplicate' : 'reference-artifact-v2-order-invalid',
        comparison === 0 ? 'Metadata record identities must be unique.' : 'Metadata records are not globally ordered.',
      );
    }
    records.push(record);
    previous = record;
  }

  const groups = [];
  let current = []; let currentBytes = emptyBytes.length;
  for (const record of records) {
    const addedBytes = record.recordBytes.length + (current.length ? 1 : 0);
    if (emptyBytes.length + record.recordBytes.length > enforced.shardBytes) {
      shardError('reference-artifact-v2-limit', 'One metadata record exceeds the shard byte limit.');
    }
    if (current.length && (current.length >= enforced.shardRecords
        || currentBytes + addedBytes > enforced.shardBytes)) {
      groups.push(current); current = []; currentBytes = emptyBytes.length;
    }
    current.push(record); currentBytes += record.recordBytes.length + (current.length > 1 ? 1 : 0);
  }
  if (current.length) groups.push(current);
  if (groups.length > 256) {
    shardError('reference-artifact-v2-limit', 'The metadata family exceeds its shard count limit.');
  }

  const shards = groups.map((group, ordinal) => {
    const bytes = frame(family, group.map((entry) => entry.recordBytes));
    const value = { family, records: group.map((entry) => entry.record), schemaVersion: SCHEMA };
    const receipt = artifactV2Receipt({
      logicalPath: logicalName(family, ordinal), logicalKind: `${family}-shard`, ordinal,
      mediaType: 'application/json', content: bytes, itemCount: group.length,
      idRange: { first: group[0].key, last: group.at(-1).key },
    });
    return { value, bytes, receipt };
  });
  return { shards, index: buildArtifactV2Index(family, shards.map((shard) => shard.receipt)) };
}
