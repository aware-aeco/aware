import assert from 'node:assert/strict';
import test from 'node:test';
import {
  MODEL_LIMITS,
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
