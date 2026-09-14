import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import {
  MODEL_LIMITS,
  PROPERTY_EXPANSION_LIMITS,
  ModelReaderError,
  buildCanonicalRequest,
  buildProviderFingerprint,
  canonicalJsonBytes,
  canonicalNumberViolation,
  lowerableLimits,
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
  assert.equal(request.limits.maxGlbJsonBytes, 64 * 1024 * 1024);
  assert.equal(MODEL_LIMITS.maxGlbJsonBytes.hard, 128 * 1024 * 1024);
  assert.equal(MODEL_LIMITS.maxInputGlbBytes.default, 128 * 1024 * 1024);
  assert.equal(MODEL_LIMITS.maxInputGlbBytes.hard, 512 * 1024 * 1024);
  assert.equal(MODEL_LIMITS.maxSourceBytes.default, 256 * 1024 * 1024);
  assert.ok(2 * 94_531_584 <= MODEL_LIMITS.maxSourceBytes.default,
    'the default source limit must admit at least twice the pinned Snowdon RVT');
  assert.equal(buildCanonicalRequest({ protocolVersion: '2' }).protocolVersion, '2');
  assert.throws(() => buildCanonicalRequest({ protocolVersion: '3' }), /protocolVersion/);
});

test('every resource bound the reader enforces is declared here, lowerable and fail-closed', () => {
  // The canonical working-set budget was a module constant inside revit-glb.mjs, so it was invisible
  // to callers, could not be overridden, and silently overrode the count limits declared here: at
  // 1024 estimated bytes per vertex its fixed 1 GiB admitted 1,048,576 vertices, making maxVertices'
  // own 5,000,000 default unreachable. A bound the reader enforces but does not declare is the bug.
  for (const name of ['maxCanonicalWorkBytes', 'maxGlbJsonBytes', 'maxCanonicalGlbJsonBytes']) {
    const range = MODEL_LIMITS[name];
    assert.ok(range, `${name} must be declared in MODEL_LIMITS`);
    assert.ok(Number.isSafeInteger(range.default) && Number.isSafeInteger(range.hard));
    // hard > default, or the knob is decorative: lowerableLimits refuses anything above hard, so a
    // limit whose default already sits at its ceiling can never be raised to admit a valid model.
    assert.ok(range.hard > range.default, `${name} cannot be raised: default already equals its hard ceiling`);
    assert.equal(lowerableLimits({ [name]: range.default - 1 })[name], range.default - 1);
    assert.equal(lowerableLimits({})[name], range.default);
    assert.throws(() => lowerableLimits({ [name]: range.hard + 1 }), /exceeds its hard ceiling/);
    assert.throws(() => lowerableLimits({ [name]: 0 }), /exceeds its hard ceiling/);
  }
  // The pinned Snowdon Towers profile from #517 — 1,528,598 vertices, 8,523,729 indices (counted once
  // as read and once as expanded) and 23,181 primitives — is the model class this reader advertises.
  // Its estimate is what the superseded 1 GiB gate refused; the declared budget must admit it.
  const snowdonWork = 1_528_598 * 1024 + 8_523_729 * 128 * 2 + 23_181 * 4096;
  assert.equal(snowdonWork, 3_842_308_352);
  assert.ok(snowdonWork <= MODEL_LIMITS.maxCanonicalWorkBytes.default,
    'the declared budget must admit the pinned sample, or the supported model class excludes it');
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
  assert.equal(PROPERTY_EXPANSION_LIMITS.maxExpandedPropertyRows.hard, 5_000_000);
  assert.equal(PROPERTY_EXPANSION_LIMITS.maxCanonicalPropertyBytes.hard, 128 * 1024 * 1024);
  assert.equal(PROPERTY_EXPANSION_LIMITS.maxExpandedPropertyRows.default, MODEL_LIMITS.maxParameters.default,
    'opting into v2 must not silently narrow the v1 property-row budget');
  assert.equal(PROPERTY_EXPANSION_LIMITS.maxCanonicalPropertyBytes.default, MODEL_LIMITS.maxComponentJsonBytes.hard,
    'the v2 property budget must be able to use the component parser headroom');
  assert.throws(() => buildCanonicalRequest({
    readerSchemaVersion: 'model-reference-reader/v2',
    propertyExpansionLimits: { maxExpandedPropertyRows: 5_000_001 },
  }), /hard ceiling/);
  assert.throws(() => buildCanonicalRequest({ readerSchemaVersion: 'model-reference-reader/v3' }), /unsupported/);
});

test('v2 schema binds each provider-display valueType to the matching JSON value type', () => {
  const schema = JSON.parse(readFileSync(new URL('./model-metadata-v2.schema.json', import.meta.url), 'utf8'));
  const providerDisplayBranches = schema.properties.parameters.items.oneOf
    .filter((branch) => branch.properties?.valueEncoding?.const === 'provider-display')
    .map((branch) => [branch.properties.valueType.const, branch.properties.value.type])
    .sort(([left], [right]) => left.localeCompare(right));

  assert.deepEqual(providerDisplayBranches, [
    ['number', 'number'],
    ['string', 'string'],
  ]);
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

// #519: the defect was two definitions of "a number canonical JSON can carry" drifting apart — the
// admission checks asked Number.isFinite while the canonicalizer additionally required safe-integrality,
// so a value cleared validation and then threw a bare TypeError mid-signing. canonicalNumberViolation is
// now the single definition, and this asserts it stays in lockstep with the canonicalizer it speaks for.
// A reviewer relaxing either one alone has to make this test red to do it.
test('canonicalNumberViolation agrees with canonicalJsonBytes on every boundary number', () => {
  const cases = [
    0, -0, 1, -1, 1.25, 1e-7, 5e-324, 1e300, 0.1 + 0.2,
    Number.MAX_SAFE_INTEGER, -Number.MAX_SAFE_INTEGER, Number.MIN_SAFE_INTEGER,
    Number.MAX_SAFE_INTEGER + 1, Number.MIN_SAFE_INTEGER - 1,
    9.223372036854776e18, -9.223372036854776e18, Math.fround(1e20), 2 ** 53, 2 ** 64, 1e21,
    Number.NaN, Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY, Number.MAX_VALUE,
  ];
  let refused = 0;
  let accepted = 0;
  for (const value of cases) {
    const violation = canonicalNumberViolation(value);
    if (violation === null) {
      accepted += 1;
      assert.doesNotThrow(() => canonicalJsonBytes({ value }), `canonicalNumberViolation accepted ${value} but the canonicalizer refused it`);
    } else {
      refused += 1;
      assert.throws(() => canonicalJsonBytes({ value }), new RegExp(violation.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
        `canonicalNumberViolation refused ${value} as "${violation}" but the canonicalizer did not agree`);
    }
  }
  // Both sides of the partition must be populated, or the loop above proves nothing.
  assert.equal(accepted, 11);
  assert.equal(refused, cases.length - 11);
});

test('canonicalNumberViolation names the reason, and refuses non-numbers rather than coercing them', () => {
  assert.equal(canonicalNumberViolation(1.5), null);
  assert.equal(canonicalNumberViolation(Number.NaN), 'must be finite');
  assert.equal(canonicalNumberViolation(Number.POSITIVE_INFINITY), 'must be finite');
  assert.equal(canonicalNumberViolation(2 ** 53), 'must be a safe integer');
  // A numeric STRING is exactly what a producer is told to send instead, so it must not be mistaken for
  // an acceptable number here: callers use the null return as permission to write the value as JSON.
  assert.equal(canonicalNumberViolation('9223372036854775807'), 'must be a number');
  assert.equal(canonicalNumberViolation(null), 'must be a number');
  assert.equal(canonicalNumberViolation(undefined), 'must be a number');
  assert.equal(canonicalNumberViolation(9007199254740993n), 'must be a number');
});
