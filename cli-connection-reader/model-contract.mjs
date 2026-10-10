import { createHash, randomUUID } from 'node:crypto';
import { isUtf8 } from 'node:buffer';

export const READER_SCHEMA_VERSION_V1 = 'model-reference-reader/v1';
export const READER_SCHEMA_VERSION_V2 = 'model-reference-reader/v2';
// Kept as v1 for callers that have not opted into the additive v2 contract.
export const READER_SCHEMA_VERSION = READER_SCHEMA_VERSION_V1;
const SHA256 = /^[0-9a-f]{64}$/;
const PLAIN = Object.getPrototypeOf({});

export const MODEL_LIMITS = Object.freeze({
  providerRequestBytes: { default: 256 * 1024, hard: 1024 * 1024 },
  providerStdoutBytes: { default: 256 * 1024, hard: 1024 * 1024 },
  providerStderrBytes: { default: 64 * 1024, hard: 256 * 1024 },
  conversionMs: { default: 10 * 60_000, hard: 30 * 60_000 },
  maxSourceBytes: { default: 150 * 1024 * 1024, hard: 4 * 1024 * 1024 * 1024 },
  maxInputGlbBytes: { default: 128 * 1024 * 1024, hard: 512 * 1024 * 1024 },
  maxMetadataBytes: { default: 16 * 1024 * 1024, hard: 128 * 1024 * 1024 },
  maxProviderOutputBytes: { default: 144 * 1024 * 1024, hard: 576 * 1024 * 1024 },
  // Bounds the JSON chunk of the PROVIDER's GLB. It has to admit what maxInputGlbBytes admits: a
  // 42 MB authenticated GLB carrying a 23.5 MB JSON chunk is inside every other declared limit, and
  // a 16 MiB sub-limit refused it (#517). Kept below maxInputGlbBytes, which the chunk is part of.
  maxGlbJsonBytes: { default: 64 * 1024 * 1024, hard: 128 * 1024 * 1024 },
  // Bounds the JSON chunk this reader EMITS. A separate budget from the input's on purpose: the two
  // measure different documents, and folding them into one knob means raising the input bound to
  // admit a real model silently raises the output bound past what the structural count limits can
  // still police, retiring the canonical-output guard.
  maxCanonicalGlbJsonBytes: { default: 16 * 1024 * 1024, hard: 128 * 1024 * 1024 },
  maxJsonDepth: { default: 64, hard: 128 },
  maxScenes: { default: 8, hard: 32 },
  maxNodes: { default: 100_000, hard: 250_000 },
  maxNodeDepth: { default: 128, hard: 256 },
  maxMeshes: { default: 100_000, hard: 250_000 },
  maxPrimitives: { default: 200_000, hard: 500_000 },
  maxAccessors: { default: 250_000, hard: 1_000_000 },
  maxBufferViews: { default: 250_000, hard: 1_000_000 },
  maxVertices: { default: 5_000_000, hard: 10_000_000 },
  maxIndices: { default: 15_000_000, hard: 40_000_000 },
  maxEntities: { default: 250_000, hard: 1_000_000 },
  maxParameters: { default: 2_000_000, hard: 5_000_000 },
  maxRelationships: { default: 1_000_000, hard: 2_000_000 },
  maxComponentJsonBytes: { default: 32 * 1024 * 1024, hard: 128 * 1024 * 1024 },
  maxCanonicalGlbBytes: { default: 256 * 1024 * 1024, hard: 512 * 1024 * 1024 },
  // Budget for the conservative worst-case working-set estimate the v1 normalizer reserves before it
  // builds canonical object graphs. This was a bare module constant pinned at 1 GiB, which made it
  // invisible here, unoverridable, and a silent override of the declared count limits above: at
  // 1024 estimated bytes per vertex a 1 GiB budget admits 1,048,576 vertices, so maxVertices' own
  // 5,000,000 default was unreachable — 21% of it — and no published limit said so (#517).
  // The estimate is a worst case, not a resident-memory reading: measured against real conversions it
  // overstates RSS by roughly 2-3.4x, so this budget corresponds to ~1.2-2 GiB actually resident.
  maxCanonicalWorkBytes: { default: 4 * 1024 * 1024 * 1024, hard: 16 * 1024 * 1024 * 1024 },
  maxCommandResponseBytes: { default: 1024 * 1024, hard: 1024 * 1024 },
});

export const PROPERTY_EXPANSION_LIMITS = Object.freeze({
  // The effective default inherits maxParameters from the same request. This exported default is
  // the ordinary profile; production can still opt into the 5M hard cap explicitly.
  maxExpandedPropertyRows: { default: MODEL_LIMITS.maxParameters.default, hard: 5_000_000 },
  // The effective default inherits maxComponentJsonBytes from the same request, so a caller-lowered
  // component budget also stops expansion before the property document is materialized.
  maxCanonicalPropertyBytes: {
    default: MODEL_LIMITS.maxComponentJsonBytes.default,
    hard: MODEL_LIMITS.maxComponentJsonBytes.hard,
  },
});

// Reader-v2 publishes larger property shards by default. Keep that versioned default beside the
// base limit table so every caller and external verifier derives the same signed request; an
// explicitly supplied lower value still wins.
export const READER_SCHEMA_LIMIT_DEFAULTS = Object.freeze({
  [READER_SCHEMA_VERSION_V1]: Object.freeze({}),
  [READER_SCHEMA_VERSION_V2]: Object.freeze({
    maxSourceBytes: 256 * 1024 * 1024,
    maxMetadataBytes: 32 * 1024 * 1024,
    maxCanonicalGlbJsonBytes: 32 * 1024 * 1024,
    maxIndices: 20_000_000,
    maxComponentJsonBytes: 128 * 1024 * 1024,
    maxCanonicalWorkBytes: 8 * 1024 * 1024 * 1024,
  }),
});

export class ModelReaderError extends Error {
  constructor(code, phase, retryable, message, unsafeDetails = undefined, providerCode = undefined) {
    super(message);
    this.name = 'ModelReaderError';
    this.code = code;
    this.phase = phase;
    this.retryable = retryable;
    this.diagnosticId = randomUUID();
    if (typeof providerCode === 'string' && /^xeorvt-[a-z0-9-]{1,90}$/.test(providerCode)) this.providerCode = providerCode;
    Object.defineProperty(this, 'unsafeDetails', { value: unsafeDetails, enumerable: false });
  }
}

export function safeErrorEnvelope(error) {
  if (error instanceof ModelReaderError) {
    return {
      code: error.code,
      phase: error.phase,
      retryable: error.retryable,
      message: boundedMessage(error.message),
      diagnosticId: error.diagnosticId,
      ...(error.providerCode ? { providerCode: error.providerCode } : {}),
    };
  }
  return {
    code: 'reference-internal-error',
    phase: 'internal',
    retryable: false,
    message: 'The model reader failed.',
    diagnosticId: randomUUID(),
  };
}

function boundedMessage(message) {
  const clean = typeof message === 'string' ? message.replace(/[\r\n\t]+/g, ' ').trim() : 'The model reader failed.';
  return clean.slice(0, 240) || 'The model reader failed.';
}

export function sha256(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

export function assertSha256(value, label = 'digest') {
  if (typeof value !== 'string' || !SHA256.test(value)) {
    throw new ModelReaderError('reference-contract-invalid', 'admission', false, `${label} must be a lowercase SHA-256 digest`);
  }
  return value;
}

const LONE_SURROGATE_CANDIDATE = /[\ud800-\udfff]/;

function assertUnicodeScalars(value) {
  // Only a string holding a surrogate code unit can violate the rule, and almost none do: let the
  // regex engine rule the common case out before the per-unit walk (#679).
  if (!LONE_SURROGATE_CANDIDATE.test(value)) return;
  for (let i = 0; i < value.length; i += 1) {
    const unit = value.charCodeAt(i);
    if (unit >= 0xd800 && unit <= 0xdbff) {
      const next = value.charCodeAt(i + 1);
      if (!(next >= 0xdc00 && next <= 0xdfff)) throw new TypeError('string must contain Unicode scalar values');
      i += 1;
    } else if (unit >= 0xdc00 && unit <= 0xdfff) {
      throw new TypeError('string must contain Unicode scalar values');
    }
  }
}

function defineJsonProperty(target, key, value) {
  Object.defineProperty(target, key, { value, enumerable: true, configurable: true, writable: true });
}

// Same result as defineJsonProperty on a fresh plain object, minus the descriptor allocation that
// dominated parse time (#679). A key that anything on Object.prototype could intercept
// (`__proto__`, `constructor`, a polluted accessor, a frozen prototype's read-only members) still
// takes the defineProperty path.
function setJsonProperty(target, key, value) {
  if (key in PLAIN) defineJsonProperty(target, key, value);
  else target[key] = value;
}

/**
 * Why canonical JSON cannot carry `value`, as a message tail, or `null` when it can.
 *
 * Finiteness alone is NOT the rule. An integral double outside the safe-integer range — `1e20`, or a
 * float32 coordinate that rounds to there — serializes to digits that no reader can read back as the
 * same value, so canonicalization refuses it. Every admission check upstream of `canonicalJsonBytes`
 * must apply this same predicate: one that asks only `Number.isFinite` admits such a value, and the
 * refusal then surfaces as a bare `TypeError` out of the canonicalizer, which the dispatch masks as
 * `reference-internal-error` — with no mention of the field that carried it (#519).
 *
 * This is the single definition of that rule. `normalizeJson` is one of its callers, not a second copy;
 * a validator that reimplements the test is how the two drifted apart in the first place.
 */
export function canonicalNumberViolation(value) {
  if (typeof value !== 'number') return 'must be a number';
  if (!Number.isFinite(value)) return 'must be finite';
  if (Number.isInteger(value) && !Number.isSafeInteger(value)) return 'must be a safe integer';
  return null;
}

const ARRAY_INDEX_KEY = /^(?:0|[1-9]\d*)$/;
const MAX_ARRAY_INDEX = 4294967294;

function isArrayIndexKey(key) {
  return ARRAY_INDEX_KEY.test(key) && Number(key) <= MAX_ARRAY_INDEX;
}

// The order in which JSON.stringify emits `keys` (already sorted by UTF-16 code unit) once they have
// been assigned to a fresh object in that order: the engine lists array-index keys first, ascending
// numerically, then everything else in insertion order. Canonical output has always been defined as
// `JSON.stringify(<sorted-key copy>)`, so that quirk is part of the byte contract and is kept.
function engineKeyOrder(keys) {
  let digitLead = false;
  for (const key of keys) {
    const lead = key.charCodeAt(0);
    if (lead >= 0x30 && lead <= 0x39) { digitLead = true; break; }
  }
  if (!digitLead) return keys;
  const indexKeys = keys.filter(isArrayIndexKey).sort((left, right) => Number(left) - Number(right));
  if (!indexKeys.length) return keys;
  return indexKeys.concat(keys.filter((key) => !isArrayIndexKey(key)));
}

// Serializes `value` straight to canonical JSON text: strict data only, object keys sorted, -0 as 0.
// Byte-for-byte what `JSON.stringify` of a sorted-key normalized copy of `value` yields (the former
// two-step normalizeJson + stringify), without building that copy (#679).
function canonicalText(value, seen) {
  if (value === null) return 'null';
  if (typeof value === 'boolean') return value ? 'true' : 'false';
  if (typeof value === 'string') {
    assertUnicodeScalars(value);
    return JSON.stringify(value);
  }
  if (typeof value === 'number') {
    const violation = canonicalNumberViolation(value);
    if (violation) throw new TypeError(`JSON number ${violation}`);
    return JSON.stringify(value);
  }
  if (!value || typeof value !== 'object') throw new TypeError('value is not JSON data');
  if (seen.has(value)) throw new TypeError('JSON value must be acyclic');
  seen.add(value);
  try {
    if (Array.isArray(value)) {
      for (let index = 0; index < value.length; index += 1) {
        if (!Object.hasOwn(value, index)) throw new TypeError('sparse array is not JSON data');
      }
      let text = '[';
      for (let index = 0; index < value.length; index += 1) {
        if (index) text += ',';
        text += canonicalText(value[index], seen);
      }
      return `${text}]`;
    }
    if (Object.getPrototypeOf(value) !== PLAIN && Object.getPrototypeOf(value) !== null) throw new TypeError('JSON object must be plain');
    // Members are validated in sorted-key order (so the first error reported does not depend on the
    // engine's emission order) and emitted in engine order.
    const keys = Object.keys(value).sort();
    const emitted = new Map();
    for (const key of keys) {
      assertUnicodeScalars(key);
      const child = value[key];
      if (child === undefined) throw new TypeError('undefined is not JSON data');
      emitted.set(key, `${JSON.stringify(key)}:${canonicalText(child, seen)}`);
    }
    const order = engineKeyOrder(keys);
    let text = '{';
    for (let index = 0; index < order.length; index += 1) {
      if (index) text += ',';
      text += emitted.get(order[index]);
    }
    return `${text}}`;
  } finally {
    seen.delete(value);
  }
}

export function canonicalJsonBytes(value) {
  return Buffer.from(canonicalText(value, new Set()), 'utf8');
}

const JSON_NUMBER = /-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?/y;

export function parseJsonStrict(input, options = {}) {
  let text;
  if (Buffer.isBuffer(input) || input instanceof Uint8Array) {
    const bytes = Buffer.from(input);
    if (!isUtf8(bytes)) throw new SyntaxError('JSON input is not valid UTF-8');
    text = bytes.toString('utf8');
  } else {
    text = String(input);
  }
  const maxBytes = options.maxBytes ?? 16 * 1024 * 1024;
  const maxDepth = options.maxDepth ?? 128;
  if (Buffer.byteLength(text, 'utf8') > maxBytes) throw new SyntaxError('JSON input exceeds its byte limit');
  const length = text.length;
  let cursor = 0;
  // RFC 8259 permits exactly space, tab, carriage return, and line feed between tokens.
  // JavaScript's `\s` also accepts BOM, NBSP, and Unicode separators.
  const white = () => {
    while (cursor < length) {
      const code = text.charCodeAt(cursor);
      if (code !== 0x20 && code !== 0x09 && code !== 0x0d && code !== 0x0a) return;
      cursor += 1;
    }
  };
  const fail = (message) => { throw new SyntaxError(`${message} at byte ${cursor}`); };
  const stringValue = () => {
    if (text.charCodeAt(cursor) !== 0x22) fail('expected JSON string');
    const start = cursor;
    cursor += 1;
    let escaped = false;
    let escapes = false;
    for (; cursor < length; cursor += 1) {
      const code = text.charCodeAt(cursor);
      if (escaped) { escaped = false; continue; }
      if (code === 0x5c) { escaped = true; escapes = true; continue; }
      if (code === 0x22) {
        cursor += 1;
        // Without a backslash the literal's value is exactly the text between its quotes, so the
        // native parse (which validates the escapes) is only needed when there is one (#679).
        const value = escapes ? JSON.parse(text.slice(start, cursor)) : text.slice(start + 1, cursor - 1);
        assertUnicodeScalars(value);
        return value;
      }
      if (code < 0x20) fail('unescaped control character');
    }
    fail('unterminated JSON string');
  };
  const value = (depth) => {
    if (depth > maxDepth) fail('JSON nesting exceeds its limit');
    white();
    const code = text.charCodeAt(cursor);
    if (code === 0x22) return stringValue();
    if (code === 0x7b) {
      cursor += 1;
      const out = {};
      white();
      if (text.charCodeAt(cursor) === 0x7d) { cursor += 1; return out; }
      for (;;) {
        white();
        const key = stringValue();
        // Every member is added to `out` as an own property, so a repeat is exactly an own hit.
        if (Object.hasOwn(out, key)) fail(`duplicate JSON key '${key}'`);
        white();
        if (text.charCodeAt(cursor) !== 0x3a) fail('expected colon');
        cursor += 1;
        setJsonProperty(out, key, value(depth + 1));
        white();
        const next = text.charCodeAt(cursor);
        if (next === 0x7d) { cursor += 1; return out; }
        if (next !== 0x2c) fail('expected comma');
        cursor += 1;
      }
    }
    if (code === 0x5b) {
      cursor += 1;
      const out = [];
      white();
      if (text.charCodeAt(cursor) === 0x5d) { cursor += 1; return out; }
      for (;;) {
        out.push(value(depth + 1));
        white();
        const next = text.charCodeAt(cursor);
        if (next === 0x5d) { cursor += 1; return out; }
        if (next !== 0x2c) fail('expected comma');
        cursor += 1;
      }
    }
    if (code === 0x74 && text.startsWith('true', cursor)) { cursor += 4; return true; }
    if (code === 0x66 && text.startsWith('false', cursor)) { cursor += 5; return false; }
    if (code === 0x6e && text.startsWith('null', cursor)) { cursor += 4; return null; }
    JSON_NUMBER.lastIndex = cursor;
    const match = JSON_NUMBER.exec(text);
    if (!match) fail('invalid JSON value');
    cursor += match[0].length;
    const number = Number(match[0]);
    if (!Number.isFinite(number)) fail('JSON number must be finite');
    if (!/[.eE]/.test(match[0]) && !Number.isSafeInteger(number)) fail('JSON integer must be a safe integer');
    return Object.is(number, -0) ? 0 : number;
  };
  const parsed = value(0);
  white();
  if (cursor !== text.length) fail('trailing JSON data');
  return parsed;
}

export function assertClosedObject(value, required, optional = [], label = 'object') {
  if (!value || typeof value !== 'object' || Array.isArray(value)) throw new TypeError(`${label} must be an object`);
  const allowed = new Set([...required, ...optional]);
  for (const key of Object.keys(value)) if (!allowed.has(key)) throw new TypeError(`${label} has unknown property '${key}'`);
  for (const key of required) if (!Object.hasOwn(value, key)) throw new TypeError(`${label} is missing '${key}'`);
  return value;
}

export function lowerableLimits(overrides = {}, readerSchemaVersion = READER_SCHEMA_VERSION_V1) {
  if (!Object.hasOwn(READER_SCHEMA_LIMIT_DEFAULTS, readerSchemaVersion)) {
    throw new TypeError('readerSchemaVersion is unsupported');
  }
  assertClosedObject(overrides, [], Object.keys(MODEL_LIMITS), 'limits');
  const result = {};
  for (const [name, range] of Object.entries(MODEL_LIMITS)) {
    const selected = Object.hasOwn(overrides, name)
      ? overrides[name]
      : (READER_SCHEMA_LIMIT_DEFAULTS[readerSchemaVersion][name] ?? range.default);
    if (!Number.isSafeInteger(selected) || selected <= 0 || selected > range.hard) throw new TypeError(`${name} exceeds its hard ceiling`);
    result[name] = selected;
  }
  return result;
}

export function lowerablePropertyExpansionLimits(overrides = {}, effectiveModelLimits = undefined) {
  assertClosedObject(overrides, [], Object.keys(PROPERTY_EXPANSION_LIMITS), 'propertyExpansionLimits');
  const modelLimits = lowerableLimits(effectiveModelLimits);
  const result = {};
  for (const [name, range] of Object.entries(PROPERTY_EXPANSION_LIMITS)) {
    const inherited = name === 'maxExpandedPropertyRows'
      ? modelLimits.maxParameters
      : modelLimits.maxComponentJsonBytes;
    let selected = Object.hasOwn(overrides, name) ? overrides[name] : inherited;
    if (!Number.isSafeInteger(selected) || selected <= 0 || selected > range.hard) {
      throw new TypeError(`${name} exceeds its hard ceiling`);
    }
    // A property shard can never exceed the enclosing component budget. Record the effective value
    // in the canonical request instead of allocating to a larger, unusable caller override.
    if (name === 'maxCanonicalPropertyBytes') selected = Math.min(selected, modelLimits.maxComponentJsonBytes);
    result[name] = selected;
  }
  return result;
}

/**
 * Budgets for re-reading an artifact this reader EMITTED, rather than one a provider supplied.
 *
 * The canonical GLB is a different document from the input GLB and carries its own declared
 * ceilings, so measuring it against the input's is a category error: normalization publishes an
 * artifact up to maxCanonicalGlbBytes / maxCanonicalGlbJsonBytes, and re-reading that same artifact
 * under maxInputGlbBytes / maxGlbJsonBytes can refuse what was just legitimately written — after the
 * cache entry exists. The two pairs are independently configurable and the defaults already differ
 * (canonical bytes 256 MiB against input 128 MiB), so this cannot be left to callers agreeing.
 */
export function canonicalArtifactLimits(limits) {
  return {
    ...limits,
    maxInputGlbBytes: limits.maxCanonicalGlbBytes,
    maxGlbJsonBytes: limits.maxCanonicalGlbJsonBytes,
  };
}

export function buildCanonicalRequest(options = {}) {
  const protocolVersion = options.protocolVersion ?? '1';
  if (!['1', '2'].includes(protocolVersion)) throw new TypeError('protocolVersion must be 1 or 2');
  const readerSchemaVersion = options.readerSchemaVersion ?? READER_SCHEMA_VERSION_V1;
  if (![READER_SCHEMA_VERSION_V1, READER_SCHEMA_VERSION_V2].includes(readerSchemaVersion)) {
    throw new TypeError('readerSchemaVersion is unsupported');
  }
  const limits = lowerableLimits(options.limits, readerSchemaVersion);
  // Validate this v2-only field even on a v1 request so malformed or out-of-range caller input can
  // never disappear. Non-empty overrides require an explicit v2 request instead of looking
  // successful while the v1 normalizer continues to use maxParameters.
  const propertyExpansionLimits = lowerablePropertyExpansionLimits(options.propertyExpansionLimits, limits);
  const request = {
    schemaVersion: '1',
    protocolVersion,
    readerSchemaVersion,
    format: 'rvt',
    documentKind: 'revit-project',
    activeScenePolicy: 'declared-active-scene-only',
    selection: { mode: 'full-model' },
    geometry: {
      primitives: ['TRIANGLES', 'TRIANGLE_STRIP', 'TRIANGLE_FAN'],
      positions: 'VEC3/FLOAT',
      targetUnits: 'mm',
      targetUp: 'z',
      targetHandedness: 'right',
      winding: 'front-face-preserving',
      colors: 'vertex-times-base-color',
      topology: 'canonical-byte-tuples-first',
    },
    metadata: { identity: 'revit-element-id-decimal-string', joins: 'explicit-appearance-name', inference: 'none' },
    canonicalJson: 'RFC8785-JCS',
    canonicalGlb: 'model-reference-reader-glb/v1',
    conversionSettings: options.conversionSettings ?? {},
    limits,
  };
  if (readerSchemaVersion === READER_SCHEMA_VERSION_V1) {
    if (Object.keys(options.propertyExpansionLimits ?? {}).length > 0) {
      throw new TypeError('propertyExpansionLimits requires readerSchemaVersion model-reference-reader/v2');
    }
    return request;
  }
  return {
    ...request,
    schemaVersion: '2',
    metadata: {
      ...request.metadata,
      propertyValues: ['source-storage', 'provider-display'],
      providerDisplayIdentity: 'excluded',
    },
    propertyExpansionLimits,
  };
}

export function requestSha256(request) {
  return sha256(canonicalJsonBytes(request));
}

export function buildProviderFingerprint(describe) {
  const v2 = describe?.protocolVersion === '2';
  assertClosedObject(describe,
    v2
      ? ['protocolVersion', 'provider', 'engine', 'engineVersion', 'adapterBuildId', 'adapterExecutableSha256', 'execution', 'destination']
      : ['protocolVersion', 'provider', 'engine', 'engineVersion', 'adapterBuildId', 'adapterExecutableSha256'],
    ['readerSchemaVersion'], 'provider fingerprint');
  assertSha256(describe.adapterExecutableSha256, 'adapterExecutableSha256');
  for (const key of ['protocolVersion', 'provider', 'engine', 'engineVersion', 'adapterBuildId']) {
    if (typeof describe[key] !== 'string' || !describe[key]) throw new TypeError(`${key} must be a non-empty string`);
  }
  const fingerprint = {
    protocolVersion: describe.protocolVersion,
    provider: describe.provider,
    engine: describe.engine,
    engineVersion: describe.engineVersion,
    adapterBuildId: describe.adapterBuildId,
    adapterExecutableSha256: describe.adapterExecutableSha256,
    readerSchemaVersion: describe.readerSchemaVersion ?? READER_SCHEMA_VERSION,
  };
  if (v2) {
    if (describe.execution !== 'managed-cloud') throw new TypeError('protocol v2 execution must be managed-cloud');
    if (typeof describe.destination !== 'string' || !describe.destination) throw new TypeError('protocol v2 destination is required');
    fingerprint.execution = describe.execution;
    fingerprint.destination = describe.destination;
  }
  return fingerprint;
}

export function providerFingerprintSha256(fingerprint) {
  return sha256(canonicalJsonBytes(buildProviderFingerprint(fingerprint)));
}
