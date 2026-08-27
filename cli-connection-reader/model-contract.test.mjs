import assert from 'node:assert/strict';
import test from 'node:test';
import {
  MODEL_LIMITS,
  PROPERTY_EXPANSION_LIMITS,
  ModelReaderError,
  buildCanonicalRequest,
  buildProviderFingerprint,
  canonicalJsonBytes,
  parseJsonStrict,
  providerFingerprintSha256,
  requestSha256,
  safeErrorEnvelope,
} from './model-contract.mjs';

test('JCS bytes are stable across key order and pin number/string edge cases', () => {
  const a = canonicalJsonBytes({ z: '\u20ac', a: [3, -0, 1e-7], nested: { b: true, a: null } });
  const b = canonicalJsonBytes({ nested: { a: null, b: true }, a: [3, 0, 0.0000001], z: '\u20ac' });
  assert.deepEqual(a, b);
  assert.equal(a.toString('utf8'), '{"a":[3,0,1e-7],"nested":{"a":null,"b":true},"z":"€"}');
  assert.throws(() => canonicalJsonBytes({ unsafe: Number.MAX_SAFE_INTEGER + 1 }), /safe integer/);
  assert.throws(() => canonicalJsonBytes({ bad: Number.NaN }), /finite/);
  assert.throws(() => canonicalJsonBytes({ bad: '\ud800' }), /Unicode scalar/);
});

test('strict JSON parsing rejects duplicate keys, trailing bytes, and unsafe integers', () => {
  assert.deepEqual(parseJsonStrict('{"a":1,"nested":{"b":2}}'), { a: 1, nested: { b: 2 } });
  assert.throws(() => parseJsonStrict('{"a":1,"a":2}'), /duplicate JSON key/);
  assert.throws(() => parseJsonStrict('{"a":1}x'), /trailing JSON data/);
  assert.throws(() => parseJsonStrict('{"a":9007199254740992}'), /safe integer/);
  assert.throws(() => parseJsonStrict(Buffer.from([0x22, 0x80, 0x22])), /valid UTF-8/);
  assert.deepEqual(parseJsonStrict(' \t\r\n{"a":1} \t\r\n'), { a: 1 });
  for (const whitespace of ['\u00a0', '\ufeff', '\u2028', '\u2029']) {
    assert.throws(() => parseJsonStrict(`{"a":1}${whitespace}`), /trailing JSON data/);
  }
});

test('legal __proto__ keys remain own JSON data through strict parsing and canonicalization', () => {
  const parsed = parseJsonStrict('{"__proto__":{"polluted":true},"stable":1}');
  assert.equal(Object.hasOwn(parsed, '__proto__'), true);
  assert.equal(Object.getPrototypeOf(parsed), Object.prototype);
  assert.equal(Object.getPrototypeOf(parsed).polluted, undefined);
  assert.equal(canonicalJsonBytes(parsed).toString('utf8'), '{"__proto__":{"polluted":true},"stable":1}');
});

test('every canonical request leaf affects the cache/request preimage', () => {
  const request = buildCanonicalRequest();
  const baseline = requestSha256(request);
  const mutations = [
    { ...request, readerSchemaVersion: 'model-reference-reader/v2' },
    { ...request, activeScenePolicy: 'all-scenes' },
    { ...request, selection: { ...request.selection, mode: 'subset' } },
    { ...request, limits: { ...request.limits, maxEntities: request.limits.maxEntities - 1 } },
    { ...request, geometry: { ...request.geometry, targetUnits: 'm' } },
  ];
  for (const mutation of mutations) assert.notEqual(requestSha256(mutation), baseline);
  assert.equal(request.limits.maxInputGlbBytes, MODEL_LIMITS.maxInputGlbBytes.default);
  assert.equal(buildCanonicalRequest({ protocolVersion: '2' }).protocolVersion, '2');
  assert.throws(() => buildCanonicalRequest({ protocolVersion: '3' }), /protocolVersion/);
});

test('provider fingerprint is the exact seven-field JCS tuple', () => {
  const fingerprint = buildProviderFingerprint({
    protocolVersion: '1', provider: 'fixture', engine: 'fixture-engine', engineVersion: '1.2.3',
    adapterBuildId: 'fixture-build', adapterExecutableSha256: 'a'.repeat(64),
  });
  assert.deepEqual(Object.keys(fingerprint).sort(), [
    'adapterBuildId', 'adapterExecutableSha256', 'engine', 'engineVersion', 'protocolVersion',
    'provider', 'readerSchemaVersion',
  ]);
  const baseline = providerFingerprintSha256(fingerprint);
  for (const key of Object.keys(fingerprint)) {
    const value = fingerprint[key];
    const mutation = { ...fingerprint, [key]: key === 'adapterExecutableSha256' ? 'b'.repeat(64) : `${value}-changed` };
    assert.notEqual(providerFingerprintSha256(mutation), baseline, key);
  }
});

test('reader v2 binds closed effective expansion limits while v1 canonical bytes remain unchanged', () => {
  const v1 = buildCanonicalRequest();
  assert.equal(v1.schemaVersion, '1');
  assert.equal('propertyExpansionLimits' in v1, false);
  const v2 = buildCanonicalRequest({
    readerSchemaVersion: 'model-reference-reader/v2',
    propertyExpansionLimits: { maxExpandedPropertyRows: 12, maxCanonicalPropertyBytes: 4096 },
  });
  assert.equal(v2.schemaVersion, '2');
  assert.deepEqual(v2.propertyExpansionLimits, { maxExpandedPropertyRows: 12, maxCanonicalPropertyBytes: 4096 });
  assert.deepEqual(v2.metadata.propertyValues, ['source-storage', 'provider-display']);
  assert.equal(v2.metadata.providerDisplayIdentity, 'excluded');
  assert.equal(PROPERTY_EXPANSION_LIMITS.maxExpandedPropertyRows.hard, 2_000_000);
  assert.equal(PROPERTY_EXPANSION_LIMITS.maxCanonicalPropertyBytes.hard, 32 * 1024 * 1024);
  assert.throws(() => buildCanonicalRequest({
    readerSchemaVersion: 'model-reference-reader/v2',
    propertyExpansionLimits: { maxExpandedPropertyRows: 2_000_001 },
  }), /hard ceiling/);
  assert.throws(() => buildCanonicalRequest({ readerSchemaVersion: 'model-reference-reader/v3' }), /unsupported/);
});

test('managed-cloud provider fingerprint binds execution and exact destination', () => {
  const fingerprint = buildProviderFingerprint({
    protocolVersion: '2', provider: 'fixture', engine: 'fixture-engine', engineVersion: '1.2.3',
    adapterBuildId: 'fixture-build', adapterExecutableSha256: 'a'.repeat(64),
    execution: 'managed-cloud', destination: 'https://api.stage.floless.io',
  });
  assert.deepEqual(Object.keys(fingerprint).sort(), [
    'adapterBuildId', 'adapterExecutableSha256', 'destination', 'engine', 'engineVersion', 'execution',
    'protocolVersion', 'provider', 'readerSchemaVersion',
  ]);
  const baseline = providerFingerprintSha256(fingerprint);
  assert.notEqual(providerFingerprintSha256({ ...fingerprint, destination: 'https://api.floless.io' }), baseline);
  assert.throws(() => buildProviderFingerprint({ ...fingerprint, execution: 'local' }), /managed-cloud/);
});

test('structured errors expose bounded safe fields and never paths or provider output', () => {
  const error = new ModelReaderError('reference-provider-failed', 'convert', false, 'provider failed', {
    sourcePath: 'C:\\private\\Residential.rvt', stderr: 'secret-provider-output', count: 3,
  });
  const envelope = safeErrorEnvelope(error);
  assert.deepEqual(Object.keys(envelope), ['code', 'phase', 'retryable', 'message', 'diagnosticId']);
  assert.equal(envelope.code, 'reference-provider-failed');
  assert.match(envelope.diagnosticId, /^[0-9a-f-]{36}$/);
  assert.doesNotMatch(JSON.stringify(envelope), /Residential|secret-provider-output|private/);
});
