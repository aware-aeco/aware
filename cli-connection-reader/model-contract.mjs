import { createHash, randomUUID } from 'node:crypto';

export const READER_SCHEMA_VERSION = 'model-reference-reader/v1';
const SHA256 = /^[0-9a-f]{64}$/;
const PLAIN = Object.getPrototypeOf({});

export const MODEL_LIMITS = Object.freeze({
  providerRequestBytes: { default: 256 * 1024, hard: 1024 * 1024 },
  providerStdoutBytes: { default: 256 * 1024, hard: 1024 * 1024 },
  providerStderrBytes: { default: 64 * 1024, hard: 256 * 1024 },
  conversionMs: { default: 10 * 60_000, hard: 30 * 60_000 },
  maxSourceBytes: { default: 150 * 1024 * 1024, hard: 4 * 1024 * 1024 * 1024 },
  maxInputGlbBytes: { default: 128 * 1024 * 1024, hard: 512 * 1024 * 1024 },
  maxMetadataBytes: { default: 16 * 1024 * 1024, hard: 64 * 1024 * 1024 },
  maxProviderOutputBytes: { default: 144 * 1024 * 1024, hard: 576 * 1024 * 1024 },
  maxGlbJsonBytes: { default: 4 * 1024 * 1024, hard: 16 * 1024 * 1024 },
  maxJsonDepth: { default: 64, hard: 128 },
  maxScenes: { default: 8, hard: 32 },
  maxNodes: { default: 100_000, hard: 250_000 },
  maxNodeDepth: { default: 128, hard: 256 },
  maxMeshes: { default: 100_000, hard: 250_000 },
  maxPrimitives: { default: 200_000, hard: 500_000 },
  maxAccessors: { default: 250_000, hard: 1_000_000 },
  maxBufferViews: { default: 250_000, hard: 1_000_000 },
  maxVertices: { default: 5_000_000, hard: 10_000_000 },
  maxIndices: { default: 15_000_000, hard: 30_000_000 },
  maxEntities: { default: 250_000, hard: 1_000_000 },
  maxParameters: { default: 2_000_000, hard: 5_000_000 },
  maxRelationships: { default: 1_000_000, hard: 2_000_000 },
  maxComponentJsonBytes: { default: 32 * 1024 * 1024, hard: 128 * 1024 * 1024 },
  maxCanonicalGlbBytes: { default: 256 * 1024 * 1024, hard: 512 * 1024 * 1024 },
  maxCommandResponseBytes: { default: 1024 * 1024, hard: 1024 * 1024 },
});

export class ModelReaderError extends Error {
  constructor(code, phase, retryable, message, unsafeDetails = undefined) {
    super(message);
    this.name = 'ModelReaderError';
    this.code = code;
    this.phase = phase;
    this.retryable = retryable;
    this.diagnosticId = randomUUID();
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

function assertUnicodeScalars(value) {
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

function normalizeJson(value, seen = new Set()) {
  if (value === null || typeof value === 'boolean') return value;
  if (typeof value === 'string') {
    assertUnicodeScalars(value);
    return value;
  }
  if (typeof value === 'number') {
    if (!Number.isFinite(value)) throw new TypeError('JSON number must be finite');
    if (Number.isInteger(value) && !Number.isSafeInteger(value)) throw new TypeError('JSON integer must be a safe integer');
    return Object.is(value, -0) ? 0 : value;
  }
  if (!value || typeof value !== 'object') throw new TypeError('value is not JSON data');
  if (seen.has(value)) throw new TypeError('JSON value must be acyclic');
  seen.add(value);
  try {
    if (Array.isArray(value)) return value.map((entry) => normalizeJson(entry, seen));
    if (Object.getPrototypeOf(value) !== PLAIN && Object.getPrototypeOf(value) !== null) throw new TypeError('JSON object must be plain');
    const out = {};
    for (const key of Object.keys(value).sort()) {
      assertUnicodeScalars(key);
      if (value[key] === undefined) throw new TypeError('undefined is not JSON data');
      out[key] = normalizeJson(value[key], seen);
    }
    return out;
  } finally {
    seen.delete(value);
  }
}

export function canonicalJsonBytes(value) {
  return Buffer.from(JSON.stringify(normalizeJson(value)), 'utf8');
}

export function parseJsonStrict(input, options = {}) {
  const text = Buffer.isBuffer(input) || input instanceof Uint8Array ? Buffer.from(input).toString('utf8') : String(input);
  const maxBytes = options.maxBytes ?? 16 * 1024 * 1024;
  const maxDepth = options.maxDepth ?? 128;
  if (Buffer.byteLength(text, 'utf8') > maxBytes) throw new SyntaxError('JSON input exceeds its byte limit');
  let cursor = 0;
  const white = () => { while (/\s/.test(text[cursor] ?? '')) cursor += 1; };
  const fail = (message) => { throw new SyntaxError(`${message} at byte ${cursor}`); };
  const stringValue = () => {
    if (text[cursor] !== '"') fail('expected JSON string');
    const start = cursor;
    cursor += 1;
    let escaped = false;
    for (; cursor < text.length; cursor += 1) {
      const ch = text[cursor];
      if (escaped) { escaped = false; continue; }
      if (ch === '\\') { escaped = true; continue; }
      if (ch === '"') {
        cursor += 1;
        const value = JSON.parse(text.slice(start, cursor));
        assertUnicodeScalars(value);
        return value;
      }
      if (ch.charCodeAt(0) < 0x20) fail('unescaped control character');
    }
    fail('unterminated JSON string');
  };
  const value = (depth) => {
    if (depth > maxDepth) fail('JSON nesting exceeds its limit');
    white();
    const ch = text[cursor];
    if (ch === '"') return stringValue();
    if (ch === '{') {
      cursor += 1;
      const out = {};
      const keys = new Set();
      white();
      if (text[cursor] === '}') { cursor += 1; return out; }
      for (;;) {
        white();
        const key = stringValue();
        if (keys.has(key)) fail(`duplicate JSON key '${key}'`);
        keys.add(key);
        white();
        if (text[cursor] !== ':') fail('expected colon');
        cursor += 1;
        out[key] = value(depth + 1);
        white();
        if (text[cursor] === '}') { cursor += 1; return out; }
        if (text[cursor] !== ',') fail('expected comma');
        cursor += 1;
      }
    }
    if (ch === '[') {
      cursor += 1;
      const out = [];
      white();
      if (text[cursor] === ']') { cursor += 1; return out; }
      for (;;) {
        out.push(value(depth + 1));
        white();
        if (text[cursor] === ']') { cursor += 1; return out; }
        if (text[cursor] !== ',') fail('expected comma');
        cursor += 1;
      }
    }
    for (const [token, result] of [['true', true], ['false', false], ['null', null]]) {
      if (text.startsWith(token, cursor)) { cursor += token.length; return result; }
    }
    const match = text.slice(cursor).match(/^-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?/);
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

export function lowerableLimits(overrides = {}) {
  assertClosedObject(overrides, [], Object.keys(MODEL_LIMITS), 'limits');
  const result = {};
  for (const [name, range] of Object.entries(MODEL_LIMITS)) {
    const selected = overrides[name] ?? range.default;
    if (!Number.isSafeInteger(selected) || selected <= 0 || selected > range.hard) throw new TypeError(`${name} exceeds its hard ceiling`);
    result[name] = selected;
  }
  return result;
}

export function buildCanonicalRequest(options = {}) {
  const limits = lowerableLimits(options.limits);
  return {
    schemaVersion: '1',
    protocolVersion: '1',
    readerSchemaVersion: READER_SCHEMA_VERSION,
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
}

export function requestSha256(request) {
  return sha256(canonicalJsonBytes(request));
}

export function buildProviderFingerprint(describe) {
  assertClosedObject(describe,
    ['protocolVersion', 'provider', 'engine', 'engineVersion', 'adapterBuildId', 'adapterExecutableSha256'],
    ['readerSchemaVersion'], 'provider fingerprint');
  assertSha256(describe.adapterExecutableSha256, 'adapterExecutableSha256');
  for (const key of ['protocolVersion', 'provider', 'engine', 'engineVersion', 'adapterBuildId']) {
    if (typeof describe[key] !== 'string' || !describe[key]) throw new TypeError(`${key} must be a non-empty string`);
  }
  return {
    protocolVersion: describe.protocolVersion,
    provider: describe.provider,
    engine: describe.engine,
    engineVersion: describe.engineVersion,
    adapterBuildId: describe.adapterBuildId,
    adapterExecutableSha256: describe.adapterExecutableSha256,
    readerSchemaVersion: describe.readerSchemaVersion ?? READER_SCHEMA_VERSION,
  };
}

export function providerFingerprintSha256(fingerprint) {
  return sha256(canonicalJsonBytes(buildProviderFingerprint(fingerprint)));
}
