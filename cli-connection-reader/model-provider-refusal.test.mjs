import assert from 'node:assert/strict';
import test from 'node:test';

import { canonicalJsonBytes, ModelReaderError, safeErrorEnvelope } from './model-contract.mjs';
import { parseProviderRefusal, providerRefusalError, SOURCE_PATH_LIMIT } from './model-provider-refusal.mjs';

const PROVIDER_ID = '123e4567-e89b-42d3-a456-426614174000';
const ROOT = 'C:\\aware-home\\cache\\model-reference-reader\\provider-runs\\run-abc123\\conversion\\closure';
const LOGICAL = `Shapes/${'n'.repeat(160)}/profile.dat`;

function capture(files = [LOGICAL]) {
  return {
    stagingRoot: ROOT,
    manifest: { schemaVersion: 'aware.model-source-capture/v1', namespaces: [{
      namespaceId: 'model', files: files.map((path) => ({ path, bytes: 1, sha256: '0'.repeat(64) })),
    }] },
  };
}

// The length a provider measures: the absolute path it opened, built from the request's
// capture.root, in UTF-16 units without the terminating NUL. Written out independently of the
// implementation's path.win32.join so the two cannot drift together.
function measured(root, logical) {
  return `${root}\\namespaces\\model\\${logical.replaceAll('/', '\\')}`.length;
}

function refusal(overrides = {}) {
  return {
    code: 'reference-source-path-too-long', phase: 'conversion', retryable: false,
    message: `path-length-v1:${measured(ROOT, LOGICAL)},${SOURCE_PATH_LIMIT}`,
    diagnosticId: PROVIDER_ID, source: { namespaceId: 'model', path: LOGICAL },
    ...overrides,
  };
}

function failed(value) {
  const stderr = Buffer.isBuffer(value) ? value : canonicalJsonBytes(value);
  return { exitCode: 2, stdout: Buffer.alloc(0), stderr };
}

function parse(value, options = {}) {
  return parseProviderRefusal(failed(value), {
    phase: 'conversion', stderrLimit: 4096, capture: capture(), platform: 'win32', ...options,
  });
}

test('admits a corroborated source-path refusal in both phases with AWARE-derived details', () => {
  const length = measured(ROOT, LOGICAL);
  assert.ok(length > SOURCE_PATH_LIMIT, 'fixture must be over the classic limit');
  for (const phase of ['discovery', 'conversion']) {
    const admitted = parse(refusal({ phase }), { phase });
    assert.deepEqual(admitted, {
      code: 'reference-source-path-too-long',
      message: `path-length-v1:${length},259`,
      details: { pathLength: String(length), pathLimit: '259', sourceNamespace: 'model', sourcePath: LOGICAL },
    });
    const envelope = safeErrorEnvelope(providerRefusalError(phase, admitted, failed(refusal({ phase }))));
    assert.equal(envelope.code, 'reference-source-path-too-long');
    assert.equal(envelope.phase, phase);
    assert.equal(envelope.retryable, false);
    assert.notEqual(envelope.diagnosticId, PROVIDER_ID, 'AWARE keeps its own diagnostic ID');
    assert.match(envelope.diagnosticId, /^[0-9a-f-]{36}$/);
    assert.deepEqual(envelope.details, admitted.details);
  }
});

test('control: an unknown refusal code with the same payload is not admitted', () => {
  assert.equal(parse(refusal({ code: 'reference-source-unreadable' })), null);
});

test('the classic MAX_PATH boundary excludes the terminating NUL', () => {
  const at = (length) => {
    const root = 'C:\\r';
    const prefix = `${root}\\namespaces\\model\\`;
    const logical = 'a'.repeat(length - prefix.length);
    const value = { ...capture([logical]), stagingRoot: root };
    return parse(refusal({ message: `path-length-v1:${length},259`, source: { namespaceId: 'model', path: logical } }),
      { capture: value });
  };
  assert.equal(at(259), null, 'a 259-unit path fits under MAX_PATH');
  assert.equal(at(260)?.code, 'reference-source-path-too-long');
  for (const limit of ['1', '258', '260', '32767', '0259']) {
    assert.equal(parse(refusal({ message: `path-length-v1:${measured(ROOT, LOGICAL)},${limit}` })), null, limit);
  }
});

test('an uncorroborated path claim falls back to the generic failure', () => {
  const length = measured(ROOT, LOGICAL);
  const cases = {
    'missing source': refusal({ source: undefined }),
    'unknown file': refusal({ source: { namespaceId: 'model', path: 'Shapes/other.dat' } }),
    'unknown namespace': refusal({ source: { namespaceId: 'other', path: LOGICAL } }),
    'length mismatch': refusal({ message: `path-length-v1:${length + 1},259` }),
    'extra source key': refusal({ source: { namespaceId: 'model', path: LOGICAL, note: 'x' } }),
    'source not an object': refusal({ source: LOGICAL }),
    'leading zero length': refusal({ message: `path-length-v1:0${length},259` }),
    'wrong version': refusal({ message: `path-length-v2:${length},259` }),
    retryable: refusal({ retryable: true }),
    'wrong phase': refusal({ phase: 'preflight' }),
  };
  for (const [label, value] of Object.entries(cases)) {
    if (value.source === undefined) delete value.source;
    assert.equal(parse(value), null, label);
  }
  assert.equal(parse(refusal(), { platform: 'linux' }), null, 'a Windows-only diagnosis on another platform');
});

test('conversion validates against the consumed closure, not the full capture', () => {
  const closureOnly = capture(['Shapes/consumed.dat']);
  const consumed = 'Shapes/consumed.dat';
  const stagedOnly = LOGICAL;
  const pad = 'p'.repeat(200);
  const root = `${ROOT}\\${pad}`;
  const value = { ...closureOnly, stagingRoot: root };
  const make = (logical) => refusal({
    message: `path-length-v1:${measured(root, logical)},259`, source: { namespaceId: 'model', path: logical },
  });
  assert.equal(parse(make(consumed), { capture: value })?.details.sourcePath, consumed);
  assert.equal(parse(make(stagedOnly), { capture: value }), null);
});

test('coverage refusals stay conversion-only and cannot carry a source', () => {
  const coverage = {
    code: 'reference-model-coverage-incomplete', phase: 'conversion', retryable: false,
    message: 'coverage-counts-v1:0,0,49,58,0,16', diagnosticId: PROVIDER_ID,
  };
  assert.equal(parse(coverage)?.message, coverage.message);
  assert.equal(parse({ ...coverage, phase: 'discovery' }, { phase: 'discovery' }), null);
  assert.equal(parse({ ...coverage, source: { namespaceId: 'model', path: LOGICAL } }), null);
});

test('provider free text is never forwarded into the public envelope', () => {
  const secret = 'SECRET-c7f1-customer-file';
  const attempts = [
    refusal({ message: `path-length-v1:${measured(ROOT, LOGICAL)},259 ${secret}` }),
    refusal({ message: `Could not open C:\\${secret}\\model.db1` }),
    refusal({ source: { namespaceId: 'model', path: `Shapes/${secret}.dat` } }),
    refusal({ source: { namespaceId: 'model', path: `../${secret}` } }),
    refusal({ source: { namespaceId: secret, path: LOGICAL } }),
    refusal({ diagnosticId: secret }),
  ];
  for (const attempt of attempts) {
    const admitted = parse(attempt);
    const envelope = admitted
      ? safeErrorEnvelope(providerRefusalError('conversion', admitted, failed(attempt)))
      : safeErrorEnvelope(new ModelReaderError('reference-provider-failed', 'conversion', true,
        'The enrolled provider failed during conversion.', failed(attempt)));
    assert.doesNotMatch(JSON.stringify(envelope), new RegExp(secret));
  }
  // Control: the assertion is not vacuous — AWARE's own manifest string does reach the envelope.
  const envelope = safeErrorEnvelope(providerRefusalError('conversion', parse(refusal()), failed(refusal())));
  assert.match(JSON.stringify(envelope), /profile\.dat/);
});

test('a long admitted path forwards only the numbers', () => {
  const logical = `Shapes/${'q'.repeat(300)}.dat`;
  const value = capture([logical]);
  const admitted = parse(refusal({
    message: `path-length-v1:${measured(ROOT, logical)},259`, source: { namespaceId: 'model', path: logical },
  }), { capture: value });
  assert.deepEqual(Object.keys(admitted.details), ['pathLength', 'pathLimit']);
});

test('public details admit only the validated keys, never private evidence', () => {
  const error = new ModelReaderError('reference-source-path-too-long', 'conversion', false, 'x', { stderr: 'raw' });
  assert.throws(() => error.setPublicDetails({ stderr: 'raw provider text' }), TypeError);
  assert.throws(() => error.setPublicDetails({ sourcePath: 'a\nb' }), TypeError);
  assert.throws(() => error.setPublicDetails({ sourcePath: 'x'.repeat(257) }), TypeError);
  for (const [key, value] of [['pathLength', 'raw provider text'], ['pathLimit', '0259'],
    ['sourcePath', '../outside.dat'], ['sourcePath', 'Shapes/./a.dat'], ['sourcePath', '/abs.dat'],
    ['sourcePath', 'C:\\private\\a.dat'], ['sourceNamespace', 'model/other']]) {
    assert.throws(() => error.setPublicDetails({ [key]: value }), TypeError, `${key}=${value}`);
  }
  assert.equal(safeErrorEnvelope(error).details, undefined);
  // Control: the shapes AWARE itself produces are admitted.
  error.setPublicDetails({ pathLength: '266', pathLimit: '259', sourceNamespace: 'model', sourcePath: 'Shapes/a b.dat' });
  assert.equal(safeErrorEnvelope(error).details.sourcePath, 'Shapes/a b.dat');
  assert.equal(Object.hasOwn(safeErrorEnvelope(new ModelReaderError('reference-x', 'x', false, 'x')), 'details'), false,
    'every other envelope is unchanged');
});
