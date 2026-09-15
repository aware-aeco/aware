import assert from 'node:assert/strict';
import test from 'node:test';

import { canonicalJsonBytes, sha256 } from './model-contract.mjs';
import { buildEffectiveSource } from './model-effective-source.mjs';

const hash = (text) => sha256(Buffer.from(text));

function fixture() {
  const captureManifest = {
    schemaVersion: 'aware.model-source-capture/v1',
    namespaces: [{ namespaceId: 'model', files: [
      { path: 'main.synthetic', bytes: 4, sha256: hash('main') },
      { path: 'notes.txt', bytes: 5, sha256: hash('notes') },
      { path: 'support.bin', bytes: 7, sha256: hash('support') },
    ] }],
  };
  const providerFingerprintSha256 = hash('provider');
  const policy = {
    schemaVersion: 'aware.model-dependency-policy/v1', policyId: 'synthetic-policy-v1',
    capabilityId: 'synthetic.read', providerFingerprintSha256,
    roles: [
      { role: 'primary', classification: 'mandatory', affectedDomains: [] },
      { role: 'support', classification: 'optional', affectedDomains: [] },
    ],
  };
  const dependencyReport = {
    schemaVersion: 'aware.model-provider-dependency-report/v1', protocolVersion: '3',
    capabilityId: 'synthetic.read', captureManifestSha256: sha256(canonicalJsonBytes(captureManifest)),
    primary: { namespaceId: 'model', path: 'main.synthetic', role: 'primary' },
    files: [
      { namespaceId: 'model', path: 'notes.txt', role: null, disposition: 'ignored' },
      { namespaceId: 'model', path: 'support.bin', role: 'support', disposition: 'consumed' },
      { namespaceId: 'model', path: 'main.synthetic', role: 'primary', disposition: 'consumed' },
    ],
    absent: [], unsupportedExternal: [],
    authentication: [{ kind: 'framing', subject: 'primary', evidence: 'recognized' }],
    crossFileEvidence: [{ kind: 'identity-match', subject: 'primary-support', evidence: 'matched' }],
  };
  return {
    captureManifest, captureManifestSha256: dependencyReport.captureManifestSha256,
    dependencyReport, policy, formatId: 'format.synthetic', capabilityId: 'synthetic.read',
    providerFingerprintSha256, providerPackageManifestSha256: hash('package'),
  };
}

test('effective source is canonical, sorted, and excludes ignored capture members', () => {
  const input = fixture(); const result = buildEffectiveSource(input);
  assert.equal(result.bytes.equals(canonicalJsonBytes(result.effectiveSource)), true);
  assert.equal(result.sha256, sha256(result.bytes));
  assert.deepEqual(result.effectiveSource.consumed.map((file) => file.path), ['main.synthetic', 'support.bin']);
  assert.equal(JSON.stringify(result.effectiveSource).includes('notes.txt'), false);
  const reordered = fixture(); reordered.dependencyReport.files.reverse();
  assert.equal(buildEffectiveSource(reordered).sha256, result.sha256);
});

test('an added ignored file does not change effective identity', () => {
  const before = fixture(); const after = fixture();
  after.captureManifest.namespaces[0].files.push({ path: 'log.txt', bytes: 3, sha256: hash('log') });
  after.captureManifestSha256 = sha256(canonicalJsonBytes(after.captureManifest));
  after.dependencyReport.captureManifestSha256 = after.captureManifestSha256;
  after.dependencyReport.files.push({ namespaceId: 'model', path: 'log.txt', role: null, disposition: 'ignored' });
  assert.equal(buildEffectiveSource(after).sha256, buildEffectiveSource(before).sha256);
});

test('a consumed byte change changes effective identity', () => {
  const before = fixture(); const after = fixture();
  after.captureManifest.namespaces[0].files[2].sha256 = hash('changed');
  after.captureManifestSha256 = sha256(canonicalJsonBytes(after.captureManifest));
  after.dependencyReport.captureManifestSha256 = after.captureManifestSha256;
  assert.notEqual(buildEffectiveSource(after).sha256, buildEffectiveSource(before).sha256);
});

test('mandatory absence, unknown roles, unsupported externals, incomplete coverage, and omitted roles refuse', () => {
  for (const [mutate, code] of [
    [(x) => { x.policy.roles[1].classification = 'mandatory'; x.dependencyReport.absent.push({ role: 'support' }); x.dependencyReport.files[1].disposition = 'ignored'; x.dependencyReport.files[1].role = null; }, 'reference-dependency-missing'],
    [(x) => { x.dependencyReport.files[1].role = 'invented'; }, 'reference-dependency-unknown'],
    [(x) => { x.dependencyReport.unsupportedExternal.push({ kind: 'uri', reference: 'opaque' }); }, 'reference-external-dependency-unsupported'],
    [(x) => { x.dependencyReport.files.pop(); }, 'reference-dependency-report-invalid'],
    [(x) => { x.policy.roles.push({ role: 'unreported', classification: 'optional', affectedDomains: [] }); }, 'reference-dependency-report-invalid'],
  ]) {
    const input = fixture(); mutate(input);
    assert.throws(() => buildEffectiveSource(input), (error) => error.code === code);
  }
});

test('degraded absence requires explicit allowance and is bound into the result', () => {
  const input = fixture();
  input.policy.roles.push({ role: 'detail', classification: 'degraded', affectedDomains: ['geometry', 'properties'] });
  input.dependencyReport.absent.push({ role: 'detail' });
  assert.throws(() => buildEffectiveSource(input), (error) => error.code === 'reference-degraded-conversion-refused');
  const result = buildEffectiveSource({ ...input, degradedMode: 'allow' });
  assert.equal(result.effectiveSource.completeness, 'degraded');
  assert.deepEqual(result.effectiveSource.absent, [{
    role: 'detail', classification: 'degraded', affectedDomains: ['geometry', 'properties'],
  }]);
});

test('primary dependency identifiers must be strings', () => {
  for (const mutate of [
    (input) => { input.dependencyReport.primary.namespaceId = ['model']; },
    (input) => { input.dependencyReport.primary.path = ['main.synthetic']; },
    (input) => { input.dependencyReport.primary.role = ['primary']; },
  ]) {
    const input = fixture(); mutate(input);
    assert.throws(
      () => buildEffectiveSource(input),
      (error) => error.code === 'reference-dependency-report-invalid',
    );
  }
});

test('dependency policy affected domains must be string identifiers', () => {
  const input = fixture();
  input.policy.roles.push({
    role: 'detail', classification: 'degraded', affectedDomains: [['geometry']],
  });
  input.dependencyReport.absent.push({ role: 'detail' });
  assert.throws(
    () => buildEffectiveSource({ ...input, degradedMode: 'allow' }),
    (error) => error.code === 'reference-dependency-policy-invalid',
  );
});

test('malformed dependency policy structures use the policy error path', () => {
  for (const mutate of [
    (input) => { input.policy.unexpected = true; },
    (input) => { input.policy.policyId = ['synthetic-policy-v1']; },
    (input) => { input.policy.roles[0].unexpected = true; },
    (input) => { input.policy.roles[0].role = ['primary']; },
  ]) {
    const input = fixture(); mutate(input);
    assert.throws(
      () => buildEffectiveSource(input),
      (error) => error.code === 'reference-dependency-policy-invalid',
    );
  }
});

test('capture and report paths must obey the portable capture contract', () => {
  for (const invalidPath of ['', '../main.synthetic', '/main.synthetic', 'dir\\main.synthetic', 'con/file']) {
    const captureInput = fixture();
    captureInput.captureManifest.namespaces[0].files[0].path = invalidPath;
    captureInput.captureManifestSha256 = sha256(canonicalJsonBytes(captureInput.captureManifest));
    captureInput.dependencyReport.captureManifestSha256 = captureInput.captureManifestSha256;
    captureInput.dependencyReport.files[2].path = invalidPath;
    captureInput.dependencyReport.primary.path = invalidPath;
    assert.throws(
      () => buildEffectiveSource(captureInput),
      (error) => error.code === 'reference-dependency-report-invalid',
    );
  }

  const malformedUnicode = fixture();
  malformedUnicode.captureManifest.namespaces[0].files[0].path = `bad-\ud800.bin`;
  assert.throws(
    () => buildEffectiveSource(malformedUnicode),
    (error) => error.code === 'reference-dependency-report-invalid',
  );

  const reportInput = fixture();
  reportInput.dependencyReport.files[2].path = ['main.synthetic'];
  assert.throws(
    () => buildEffectiveSource(reportInput),
    (error) => error.code === 'reference-dependency-report-invalid',
  );
});

test('provider evidence rejects unpaired UTF-16 surrogates through the structured error path', () => {
  const input = fixture();
  input.dependencyReport.authentication[0].evidence = `bad-\ud800`;
  assert.throws(
    () => buildEffectiveSource(input),
    (error) => error.code === 'reference-dependency-report-invalid',
  );

  const validInput = fixture();
  validInput.dependencyReport.authentication[0].evidence = 'verified-\ud83d\udd12';
  assert.equal(buildEffectiveSource(validInput).effectiveSource.authentication[0].evidence, 'verified-\ud83d\udd12');
});

test('capture manifests cannot exceed the capture subsystem aggregate byte ceiling', () => {
  const input = fixture();
  input.captureManifest.namespaces[0].files = Array.from({ length: 5 }, (_, index) => ({
    path: `part-${index}.bin`, bytes: 4 * 1024 * 1024 * 1024, sha256: hash(`part-${index}`),
  }));
  input.captureManifestSha256 = sha256(canonicalJsonBytes(input.captureManifest));
  input.dependencyReport.captureManifestSha256 = input.captureManifestSha256;
  assert.throws(
    () => buildEffectiveSource(input),
    (error) => error.code === 'reference-dependency-report-invalid',
  );
});

test('capture namespace IDs cannot collide by case', () => {
  const input = fixture();
  input.captureManifest.namespaces.push({
    namespaceId: 'Model', files: [{ path: 'other.bin', bytes: 1, sha256: hash('other') }],
  });
  input.captureManifestSha256 = sha256(canonicalJsonBytes(input.captureManifest));
  input.dependencyReport.captureManifestSha256 = input.captureManifestSha256;
  assert.throws(
    () => buildEffectiveSource(input),
    (error) => error.code === 'reference-dependency-report-invalid',
  );
});
