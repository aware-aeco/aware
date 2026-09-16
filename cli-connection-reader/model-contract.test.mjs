import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import {
  MODEL_LIMITS,
  PROPERTY_EXPANSION_LIMITS,
  READER_SCHEMA_LIMIT_DEFAULTS,
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
  sha256,
} from './model-contract.mjs';
import { cacheKeySha256 } from './model-cache.mjs';

test('legacy local model-reader identity bytes remain frozen across package-mode additions', () => {
  const canonicalRequest = buildCanonicalRequest();
  assert.equal(
    sha256(canonicalJsonBytes(canonicalRequest)),
    'd762a0e27b6b97f34d75a59945a537fe8a4c286757aaecc46547f75a72a0d60e',
  );
  const providerFingerprint = buildProviderFingerprint({
    protocolVersion: '1', provider: 'provider.synthetic', engine: 'engine.synthetic',
    engineVersion: '1.2.3', adapterBuildId: 'build.synthetic',
    adapterExecutableSha256: 'a'.repeat(64),
  });
  assert.equal(
    providerFingerprintSha256(providerFingerprint),
    '41b01ab9166307a60e761b4119079ab4e7c40064b73a8605ce05eea00455fe5f',
  );
  assert.equal(cacheKeySha256({
    sourceSha256: 'b'.repeat(64), canonicalRequest, providerFingerprint,
    signerFingerprintSha256: 'c'.repeat(64),
  }), '7d8513537afa897436bcbe535e1ed5387dc132b5fe5580fbeb4ce17d2ad620be');
});

test('JCS bytes are stable across key order and pin number/string edge cases', () => {
  const a = canonicalJsonBytes({ z: '\u20ac', a: [3, -0, 1e-7], nested: { b: true, a: null } });
  const b = canonicalJsonBytes({ nested: { a: null, b: true }, a: [3, 0, 0.0000001], z: '\u20ac' });
  assert.deepEqual(a, b);
  assert.equal(a.toString('utf8'), '{"a":[3,0,1e-7],"nested":{"a":null,"b":true},"z":"€"}');
  assert.throws(() => canonicalJsonBytes({ unsafe: Number.MAX_SAFE_INTEGER + 1 }), /safe integer/);
  assert.throws(() => canonicalJsonBytes({ bad: Number.NaN }), /finite/);
  assert.throws(() => canonicalJsonBytes({ bad: '\ud800' }), /Unicode scalar/);
});

test('canonical JSON refuses sparse arrays instead of changing holes to null', () => {
  assert.throws(() => canonicalJsonBytes(new Array(1)), /sparse array/);
  assert.throws(() => canonicalJsonBytes({ nested: [1, , 3] }), /sparse array/);
  assert.equal(canonicalJsonBytes([1, null, 3]).toString('utf8'), '[1,null,3]');
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
  assert.equal(buildCanonicalRequest({ protocolVersion: '2' }).protocolVersion, '2');
  assert.throws(() => buildCanonicalRequest({ protocolVersion: '3' }), /protocolVersion/);
});

test('every size and count limit admits at least twice the complete authenticated Snowdon profile', () => {
  // Captured from the successful authenticated Snowdon Towers conversion and its signed canonical
  // manifest/cache receipt (source sha256 690a69b7…00e5). Provider temporaries are deliberately
  // deleted after normalization, so metadataBytes pins the conservative upper bound proven by that
  // run: it passed under the then-signed 16 MiB limit. providerOutputBytes is consequently bounded by
  // the exact provider GLB plus that whole metadata envelope. All retained canonical values are exact.
  const snowdon = Object.freeze({
    sourceBytes: 94_531_584,
    inputGlbBytes: 41_926_848,
    metadataBytes: 16 * 1024 * 1024,
    providerOutputBytes: 41_926_848 + (16 * 1024 * 1024),
    inputGlbJsonBytes: 23_576_276,
    canonicalGlbBytes: 91_236_816,
    canonicalGlbJsonBytes: 15_246_024,
    scenes: 1,
    nodes: 5_262,
    meshes: 5_262,
    primitives: 23_181,
    accessors: 69_544,
    bufferViews: 69_544,
    vertices: 1_528_598,
    indices: 8_523_729,
    entities: 6_544,
    parameters: 17_225,
    relationships: 0,
    largestComponentJsonBytes: 48_248_289,
    expandedPropertyRows: 180_004,
    canonicalPropertyBytes: 48_248_289,
    canonicalWorkBytes: 3_842_308_352,
  });
  const modelLimitProfile = {
    maxSourceBytes: snowdon.sourceBytes,
    maxInputGlbBytes: snowdon.inputGlbBytes,
    maxMetadataBytes: snowdon.metadataBytes,
    maxProviderOutputBytes: snowdon.providerOutputBytes,
    maxGlbJsonBytes: snowdon.inputGlbJsonBytes,
    maxCanonicalGlbBytes: snowdon.canonicalGlbBytes,
    maxCanonicalGlbJsonBytes: snowdon.canonicalGlbJsonBytes,
    maxScenes: snowdon.scenes,
    maxNodes: snowdon.nodes,
    maxMeshes: snowdon.meshes,
    maxPrimitives: snowdon.primitives,
    maxAccessors: snowdon.accessors,
    maxBufferViews: snowdon.bufferViews,
    maxVertices: snowdon.vertices,
    maxIndices: snowdon.indices,
    maxEntities: snowdon.entities,
    maxParameters: snowdon.parameters,
    maxRelationships: snowdon.relationships,
    maxCanonicalWorkBytes: snowdon.canonicalWorkBytes,
  };
  for (const [limit, measured] of Object.entries(modelLimitProfile)) {
    assert.ok(measured * 2 <= MODEL_LIMITS[limit].default,
      `${limit} must admit at least twice the pinned Snowdon requirement (${measured})`);
  }

  const v2ComponentDefault = READER_SCHEMA_LIMIT_DEFAULTS['model-reference-reader/v2'].maxComponentJsonBytes;
  assert.ok(snowdon.largestComponentJsonBytes * 2 <= v2ComponentDefault,
    'the v2 component/shard limit must admit twice Snowdon properties.json');
  assert.ok(snowdon.expandedPropertyRows * 2 <= PROPERTY_EXPANSION_LIMITS.maxExpandedPropertyRows.default,
    'the property row limit must admit twice Snowdon expanded properties');
  assert.ok(snowdon.canonicalPropertyBytes * 2 <= v2ComponentDefault,
    'the v2 canonical-property limit must admit twice Snowdon property bytes');

  const canonicalAggregateBytes = 91_236_816 + 2_565_876 + 48_248_289 + 40 + 3_695;
  const v2PackageAggregateLimit = MODEL_LIMITS.maxCanonicalGlbBytes.default + (v2ComponentDefault * 5);
  assert.ok(canonicalAggregateBytes * 2 <= v2PackageAggregateLimit,
    'the signed package aggregate limit must admit twice all Snowdon canonical artifacts');
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
  assert.equal(READER_SCHEMA_LIMIT_DEFAULTS['model-reference-reader/v2'].maxComponentJsonBytes, 128 * 1024 * 1024);
  assert.equal(v2.limits.maxComponentJsonBytes, 128 * 1024 * 1024,
    'the reader-v2 shard default is published and reproduced by canonical request construction');
  assert.equal(lowerableLimits({}, 'model-reference-reader/v2').maxComponentJsonBytes, 128 * 1024 * 1024);
  assert.deepEqual(v2.propertyExpansionLimits, { maxExpandedPropertyRows: 12, maxCanonicalPropertyBytes: 4096 });
  assert.deepEqual(v2.metadata.propertyValues, ['source-storage', 'provider-display']);
  assert.equal(v2.metadata.providerDisplayIdentity, 'excluded');
  assert.equal(PROPERTY_EXPANSION_LIMITS.maxExpandedPropertyRows.hard, 5_000_000);
  assert.equal(PROPERTY_EXPANSION_LIMITS.maxCanonicalPropertyBytes.hard, 128 * 1024 * 1024);
  assert.equal(PROPERTY_EXPANSION_LIMITS.maxExpandedPropertyRows.default, MODEL_LIMITS.maxParameters.default,
    'opting into v2 must not silently narrow the v1 property-row budget');
  assert.equal(PROPERTY_EXPANSION_LIMITS.maxCanonicalPropertyBytes.default, MODEL_LIMITS.maxComponentJsonBytes.default,
    'the ordinary property-byte budget inherits the ordinary component budget');
  const lowered = buildCanonicalRequest({
    readerSchemaVersion: 'model-reference-reader/v2',
    limits: { maxParameters: 100, maxComponentJsonBytes: 64 * 1024 },
  });
  assert.deepEqual(lowered.propertyExpansionLimits,
    { maxExpandedPropertyRows: 100, maxCanonicalPropertyBytes: 64 * 1024 },
    'lowered model limits must become the default pre-allocation expansion limits');
  const explicitRows = buildCanonicalRequest({
    readerSchemaVersion: 'model-reference-reader/v2',
    limits: { maxParameters: 100, maxComponentJsonBytes: 64 * 1024 },
    propertyExpansionLimits: { maxExpandedPropertyRows: 1000, maxCanonicalPropertyBytes: 128 * 1024 },
  });
  assert.deepEqual(explicitRows.propertyExpansionLimits,
    { maxExpandedPropertyRows: 1000, maxCanonicalPropertyBytes: 64 * 1024 },
    'an explicit row expansion may exceed the table count, but bytes remain inside the enclosing shard');
  assert.throws(() => buildCanonicalRequest({
    readerSchemaVersion: 'model-reference-reader/v2',
    propertyExpansionLimits: { maxExpandedPropertyRows: 5_000_001 },
  }), /hard ceiling/);
  assert.throws(() => buildCanonicalRequest({ readerSchemaVersion: 'model-reference-reader/v3' }), /unsupported/);
});

test('v1 refuses every supplied v2 property-limit override instead of silently discarding it', () => {
  for (const propertyExpansionLimits of [
    { maxExpandedPropertyRows: 4096 },
    { maxExpandedPropertyRows: 999_999_999 },
    { maxExpandedPropertyRows: -1 },
    { maxExpandedPropertyRows: 1.5 },
    { maxExpandedPropertyRow: 4096 },
    'nonsense',
  ]) {
    assert.throws(
      () => buildCanonicalRequest({ propertyExpansionLimits }),
      /propertyExpansionLimits|maxExpandedPropertyRows/,
    );
  }
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
  const numericValue = schema.properties.parameters.items.oneOf
    .find((branch) => branch.properties?.valueType?.const === 'number').properties.value;
  assert.deepEqual(numericValue.anyOf, [
    { type: 'integer', minimum: Number.MIN_SAFE_INTEGER, maximum: Number.MAX_SAFE_INTEGER },
    { not: { type: 'integer' } },
  ], 'the public schema must reject the unsafe integral numbers refused by canonical JSON');

  const sourceStorage = schema.properties.parameters.items.oneOf
    .filter((branch) => branch.properties?.valueEncoding?.const === 'source-storage');
  assert.deepEqual(sourceStorage.map((branch) => branch.properties.storageType.const).sort(),
    ['boolean', 'double', 'element-id', 'integer', 'none', 'string']);
  for (const branch of sourceStorage) {
    const storageType = branch.properties.storageType.const;
    assert.equal(branch.properties.readable.const, storageType !== 'none', `${storageType} readability`);
  }
  assert.equal(sourceStorage.find((branch) => branch.properties.storageType.const === 'none').properties.value.type, 'null');
  assert.equal(sourceStorage.find((branch) => branch.properties.storageType.const === 'boolean').properties.value.type, 'boolean');
  assert.equal(sourceStorage.find((branch) => branch.properties.storageType.const === 'string').properties.value.type, 'string');
  const signedInt64 = schema.$defs.signedInt64Decimal;
  assert.equal(signedInt64.type, 'string');
  const signedInt64Pattern = new RegExp(signedInt64.pattern);
  for (const accepted of ['0', '1', '-1', '9223372036854775807', '-9223372036854775808']) {
    assert.equal(signedInt64Pattern.test(accepted), true, `accept signed int64 ${accepted}`);
  }
  for (const refused of ['-0', '+1', '01', '9223372036854775808', '-9223372036854775809']) {
    assert.equal(signedInt64Pattern.test(refused), false, `refuse non-int64 ${refused}`);
  }
  for (const storageType of ['integer', 'element-id']) {
    const value = sourceStorage.find((branch) => branch.properties.storageType.const === storageType).properties.value;
    assert.deepEqual(value, { $ref: '#/$defs/signedInt64Decimal' });
  }
  const doubleValue = sourceStorage.find((branch) => branch.properties.storageType.const === 'double').properties.value;
  assert.deepEqual(doubleValue.anyOf, numericValue.anyOf, 'double values obey the same signed JSON number constraint');
});

test('v2 schema publishes the closed metadata records the reader accepts', () => {
  const schema = JSON.parse(readFileSync(new URL('./model-metadata-v2.schema.json', import.meta.url), 'utf8'));
  const positiveInt64 = new RegExp(schema.$defs.positiveInt64Decimal.pattern);
  for (const accepted of ['1', '9223372036854775807']) assert.equal(positiveInt64.test(accepted), true);
  for (const refused of ['0', '-1', '01', '9223372036854775808']) assert.equal(positiveInt64.test(refused), false);

  for (const table of ['types', 'levels', 'parameterGroups', 'elements']) {
    assert.equal(schema.properties[table].items.type, 'object', `${table} has an item object`);
    assert.equal(schema.properties[table].items.additionalProperties, false, `${table} item is closed`);
  }
  assert.match(schema.properties.levels.items.properties.elevation.description, /millimetres/i);
  assert.match(schema.properties.levels.items.properties.elevation.description, /not republished/i);
  assert.deepEqual(schema.properties.elements.items.required,
    ['id', 'revitClass', 'category', 'family', 'type', 'level', 'parameterGroups', 'appearances']);
  assert.equal(schema.properties.elements.items.properties.appearances.uniqueItems, true);
  assert.equal(schema.properties.elements.items.properties.appearances.items.minLength, 1);
  const ifcGuidDescription = schema.properties.elements.items.properties.ifcGuid.description;
  assert.match(ifcGuidDescription, /must match/i);
  assert.match(ifcGuidDescription, /source-storage/i);
  assert.match(ifcGuidDescription, /provider-display values cannot establish IFC identity/i);

  const relationBranches = schema.properties.relations.items.oneOf;
  assert.equal(relationBranches.length, 2);
  assert.deepEqual(relationBranches.map((branch) => branch.properties.kind.const ?? branch.properties.kind.enum).flat().sort(),
    ['contains', 'depends-on', 'hosts', 'provider-explicit']);
  assert.deepEqual(relationBranches.find((branch) => branch.properties.kind.const === 'provider-explicit').required,
    ['id', 'kind', 'from', 'to', 'providerRelationKind']);
  assert.ok(relationBranches.every((branch) => branch.additionalProperties === false));
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
