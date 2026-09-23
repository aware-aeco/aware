import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';

import { canonicalJsonBytes, safeErrorEnvelope, sha256 } from './model-contract.mjs';
import { buildProviderConversionRequest, convertProviderSource } from './model-provider-conversion.mjs';
import { packageProviderIdentity } from './model-provider-discovery.mjs';

const PACKAGE_SHA = sha256(Buffer.from('package'));

function loadedPackage() {
  return {
    executable: { path: 'C:\\provider\\provider.exe', sha256: sha256(Buffer.from('launcher')) },
    packageRecord: {
      packageRoot: 'C:\\provider', publisherFingerprintSha256: sha256(Buffer.from('publisher')),
      manifest: { formatId: 'format.synthetic' },
    },
    capability: {
      capabilityId: 'capability.synthetic', protocolVersion: '3', sourceCaptureMode: 'capture.synthetic',
      requestSchema: 'request.synthetic/v1', resultSchema: 'result.synthetic/v1',
      artifactVersion: 'artifact.synthetic/v1', cacheNamespace: 'cache.synthetic/v1', limits: {},
    },
  };
}

async function scenario(t) {
  const parent = await fs.mkdtemp(path.join(await fs.realpath(os.tmpdir()), 'aware-convert-'));
  const stagingRoot = path.join(parent, 'run');
  await fs.mkdir(stagingRoot);
  t.after(() => fs.rm(parent, { recursive: true, force: true }));
  const loaded = loadedPackage();
  const provider = packageProviderIdentity(loaded, PACKAGE_SHA);
  const policy = {
    schemaVersion: 'aware.model-dependency-policy/v1', policyId: 'policy.synthetic',
    capabilityId: 'capability.synthetic', providerFingerprintSha256: provider.sha256, roles: [{
      role: 'primary', classification: 'mandatory', affectedDomains: [],
    }],
  };
  const policySha256 = sha256(canonicalJsonBytes(policy));
  const effectiveSource = {
    schemaVersion: 'model-effective-source/v2', formatId: 'format.synthetic', protocolVersion: '3',
    capabilityId: 'capability.synthetic', providerFingerprintSha256: provider.sha256,
    providerPackageManifestSha256: PACKAGE_SHA, discoveryPolicy: { policyId: 'policy.synthetic', sha256: policySha256 },
    completeness: 'complete', primary: { namespaceId: 'model', path: 'db.1', role: 'primary' },
    consumed: [], absent: [], unsupportedExternal: [], authentication: [], crossFileEvidence: [],
  };
  const bytes = canonicalJsonBytes(effectiveSource);
  const capture = {
    stagingRoot: path.join(stagingRoot, 'source'),
    manifest: { schemaVersion: 'aware.model-source-capture/v1', namespaces: [] },
    manifestSha256: sha256(Buffer.from('capture')),
  };
  const calls = [];
  const options = {
    home: parent, stagingRoot, formatId: 'format.synthetic', capabilityId: 'capability.synthetic',
    manifestSha256: PACKAGE_SHA, authorization: 'short-lived-token', namespaces: [{ id: 'model', root: 'D:\\model' }],
    effectiveSource, effectiveSourceSha256: sha256(bytes), environment: {},
    hostRun: async (request) => {
      calls.push(request);
      return {
        exitCode: 0, stderr: Buffer.alloc(0), stdout: canonicalJsonBytes({
          schemaVersion: 'aware.model-provider-conversion-response/v1', protocolVersion: '3',
          capabilityId: 'capability.synthetic', complete: true,
        }),
      };
    },
  };
  const deps = {
    capture: async () => { await fs.mkdir(capture.stagingRoot); return capture; },
    discover: async () => ({ effectiveSource, bytes, sha256: sha256(bytes) }),
    stageClosure: async (_capture, _discovered, root) => {
      await fs.mkdir(root);
      return { ...capture, stagingRoot: root };
    },
    loadPackage: async () => loaded,
    loadPolicy: async () => ({ policy, sha256: policySha256 }),
    verifyCapture: async () => true,
    verifyOutput: async (root, verifyOptions) => ({ root: verifyOptions.admittedRoot, sourceRoot: root, verifyOptions }),
  };
  return { options, deps, calls, bytes, capture, effectiveSource };
}

test('conversion identity binds source, package, settings and enforced limits', () => {
  const base = {
    formatId: 'format.synthetic', capabilityId: 'capability.synthetic',
    providerPackageManifestSha256: PACKAGE_SHA, effectiveSourceSha256: sha256(Buffer.from('source')),
    conversionSettings: { detail: 'full' },
  };
  const first = buildProviderConversionRequest(base);
  assert.equal(first.request.protocolVersion, '3');
  assert.notEqual(first.sha256, buildProviderConversionRequest({ ...base, conversionSettings: { detail: 'coarse' } }).sha256);
  assert.notEqual(first.sha256, buildProviderConversionRequest({ ...base, outputLimits: { payloadBytes: 1024 } }).sha256);
});

test('recaptures, rediscovers and invokes the exact enrolled provider before admitting output', async (t) => {
  const value = await scenario(t);
  const result = await convertProviderSource(value.options, value.deps);
  assert.equal(value.calls.length, 1);
  const control = JSON.parse(value.calls[0].stdin.toString('utf8'));
  assert.equal(control.operation, 'convert');
  assert.equal(control.effectiveSource.sha256, value.options.effectiveSourceSha256);
  assert.equal(control.capture.root, path.join(value.options.stagingRoot, 'closure'));
  assert.equal(control.conversionRequest.effectiveSourceSha256, value.options.effectiveSourceSha256);
  assert.equal(control.authorization, 'short-lived-token');
  assert.equal(value.calls[0].stdin.includes(Buffer.from('D:\\model')), false);
  assert.equal(result.output.verifyOptions.conversionRequestSha256, result.conversionRequestSha256);
  await assert.rejects(fs.stat(path.join(value.options.stagingRoot, 'provider-output')), (error) => error.code === 'ENOENT');
  assert.equal(result.output.root, path.join(value.options.stagingRoot, 'admitted-output'));
});

test('preserves only the allowlisted canonical conversion refusal without leaking provider detail', async (t) => {
  const value = await scenario(t);
  const privatePath = 'D:\\private\\customer-model\\db.1';
  const refusal = {
    code: 'reference-model-coverage-incomplete', phase: 'conversion', retryable: false,
    message: `Unsupported geometry in ${privatePath}`,
    diagnosticId: '123e4567-e89b-42d3-a456-426614174000',
  };
  value.options.hostRun = async () => ({ exitCode: 2, stdout: Buffer.alloc(0), stderr: canonicalJsonBytes(refusal) });
  await assert.rejects(() => convertProviderSource(value.options, value.deps), (error) => {
    const safe = safeErrorEnvelope(error);
    assert.equal(safe.code, refusal.code);
    assert.equal(safe.phase, 'conversion');
    assert.equal(safe.retryable, false);
    assert.notEqual(safe.diagnosticId, refusal.diagnosticId);
    assert.doesNotMatch(JSON.stringify(safe), /customer-model|db\.1|Unsupported geometry/);
    return true;
  });
  for (const version of ['6', '7', '8']) {
    const modern = { ...refusal, diagnosticId: `123e4567-e89b-${version}2d3-a456-426614174000` };
    value.options.hostRun = async () => ({ exitCode: 2, stdout: Buffer.alloc(0), stderr: canonicalJsonBytes(modern) });
    await assert.rejects(() => convertProviderSource(value.options, value.deps),
      (error) => error.code === modern.code && error.retryable === false);
  }
});

test('forged, malformed and oversized provider stderr remains a generic failure', async (t) => {
  const value = await scenario(t);
  const base = {
    code: 'reference-model-coverage-incomplete', phase: 'conversion', retryable: false,
    message: 'The model contains geometry this reader cannot safely place.',
    diagnosticId: '123e4567-e89b-42d3-a456-426614174000',
  };
  const bad = [
    { ...base, code: 'reference-provider-pin-mismatch' },
    { ...base, phase: 'preflight' },
    { ...base, retryable: true },
    { ...base, details: { path: 'D:\\private' } },
    { ...base, diagnosticId: 'not-a-uuid' },
    { ...base, message: 'a'.repeat(241) },
    { ...base, message: 'A path:\nD:\\private' },
    Buffer.from('{"code":'),
    Buffer.from('not json'),
    Buffer.from([0xff, 0xfe]),
    Buffer.alloc(4097, 0x61),
    Buffer.from(`${canonicalJsonBytes(base).toString('utf8')}\n`),
  ];
  for (const candidate of bad) {
    const stderr = Buffer.isBuffer(candidate) ? candidate : canonicalJsonBytes(candidate);
    value.options.hostRun = async () => ({ exitCode: 2, stdout: Buffer.alloc(0), stderr });
    await assert.rejects(() => convertProviderSource(value.options, value.deps), (error) => {
      const safe = safeErrorEnvelope(error);
      assert.equal(safe.code, 'reference-provider-failed');
      assert.equal(safe.retryable, true);
      assert.doesNotMatch(JSON.stringify(safe), /private|not json/);
      return true;
    });
  }
  value.options.hostRun = async () => ({ exitCode: 2, stdout: Buffer.from('partial'), stderr: canonicalJsonBytes(base) });
  await assert.rejects(() => convertProviderSource(value.options, value.deps),
    (error) => error.code === 'reference-provider-failed');
});

test('refuses conversion when the second discovery sees a different effective source', async (t) => {
  const value = await scenario(t);
  value.deps.discover = async () => ({
    effectiveSource: value.options.effectiveSource,
    bytes: canonicalJsonBytes({ changed: true }), sha256: sha256(canonicalJsonBytes({ changed: true })),
  });
  await assert.rejects(
    () => convertProviderSource(value.options, value.deps),
    (error) => error.code === 'reference-effective-source-changed' && error.retryable === true,
  );
  assert.equal(value.calls.length, 0);
});

test('rejects a provider package identity change around conversion', async (t) => {
  const value = await scenario(t);
  let loads = 0;
  value.deps.loadPackage = async () => {
    loads += 1;
    const loaded = loadedPackage();
    if (loads > 1) loaded.executable.sha256 = sha256(Buffer.from('changed-launcher'));
    return loaded;
  };
  await assert.rejects(
    () => convertProviderSource(value.options, value.deps),
    (error) => error.code === 'reference-provider-package-changed',
  );
});

test('rejects a dependency policy change before conversion', async (t) => {
  const value = await scenario(t);
  value.deps.loadPolicy = async () => ({
    policy: {
      schemaVersion: 'aware.model-dependency-policy/v1', policyId: 'policy.synthetic',
      capabilityId: 'capability.synthetic',
      providerFingerprintSha256: value.options.effectiveSource.providerFingerprintSha256,
      roles: [{ role: 'primary', classification: 'mandatory', affectedDomains: [] }],
    },
    sha256: sha256(Buffer.from('different-policy')),
  });
  await assert.rejects(
    () => convertProviderSource(value.options, value.deps),
    (error) => error.code === 'reference-dependency-policy-changed',
  );
  assert.equal(value.calls.length, 0);
});

test('rejects a dependency policy changed during conversion', async (t) => {
  const value = await scenario(t);
  const original = value.deps.loadPolicy;
  let loads = 0;
  value.deps.loadPolicy = async () => {
    loads += 1;
    const admitted = await original();
    return loads === 1 ? admitted : { ...admitted, sha256: sha256(Buffer.from('changed-policy')) };
  };
  await assert.rejects(
    () => convertProviderSource(value.options, value.deps),
    (error) => error.code === 'reference-dependency-policy-changed',
  );
  assert.equal(value.calls.length, 1);
});

test('never removes a pre-existing provider output directory', async (t) => {
  const value = await scenario(t);
  const outputRoot = path.join(value.options.stagingRoot, 'provider-output');
  await fs.mkdir(outputRoot);
  const sentinel = path.join(outputRoot, 'sentinel.txt');
  await fs.writeFile(sentinel, 'keep');
  await assert.rejects(() => convertProviderSource(value.options, value.deps), (error) => error.code === 'EEXIST');
  assert.equal(await fs.readFile(sentinel, 'utf8'), 'keep');
});

test('filtered closure preserves verified capture order instead of locale order', async (t) => {
  const value = await scenario(t);
  const sourceFiles = [
    { path: 'B', content: Buffer.from('upper') },
    { path: 'a', content: Buffer.from('lower') },
  ].map((file) => ({ ...file, bytes: file.content.length, sha256: sha256(file.content) }));
  value.capture.manifest.namespaces = [{
    namespaceId: 'model',
    files: sourceFiles.map(({ path: filePath, bytes, sha256: digest }) => ({ path: filePath, bytes, sha256: digest })),
  }];
  value.effectiveSource.consumed = [...sourceFiles].reverse().map(({ path: filePath, bytes, sha256: digest }) => ({
    namespaceId: 'model', path: filePath, role: 'primary', bytes, sha256: digest,
  }));
  const effectiveBytes = canonicalJsonBytes(value.effectiveSource);
  value.options.effectiveSourceSha256 = sha256(effectiveBytes);
  value.deps.discover = async () => ({
    effectiveSource: value.effectiveSource, bytes: effectiveBytes, sha256: sha256(effectiveBytes),
  });
  value.deps.capture = async () => {
    const root = path.join(value.capture.stagingRoot, 'namespaces', 'model');
    await fs.mkdir(root, { recursive: true });
    await Promise.all(sourceFiles.map((file) => fs.writeFile(path.join(root, file.path), file.content)));
    return value.capture;
  };
  delete value.deps.stageClosure;
  let closureManifest;
  value.options.hostRun = async (request) => {
    closureManifest = JSON.parse(await fs.readFile(path.join(JSON.parse(request.stdin).capture.root, 'capture.json'), 'utf8'));
    return {
      exitCode: 0, stderr: Buffer.alloc(0), stdout: canonicalJsonBytes({
        schemaVersion: 'aware.model-provider-conversion-response/v1', protocolVersion: '3',
        capabilityId: 'capability.synthetic', complete: true,
      }),
    };
  };
  await convertProviderSource(value.options, value.deps);
  assert.deepEqual(closureManifest.namespaces[0].files.map((file) => file.path), ['B', 'a']);
});

test('failed closure staging removes the partial run-owned directory', async (t) => {
  const value = await scenario(t);
  const missing = Buffer.from('missing');
  value.capture.manifest.namespaces = [{
    namespaceId: 'model', files: [{ path: 'missing.db', bytes: missing.length, sha256: sha256(missing) }],
  }];
  value.effectiveSource.consumed = [{
    namespaceId: 'model', path: 'missing.db', role: 'primary', bytes: missing.length, sha256: sha256(missing),
  }];
  const effectiveBytes = canonicalJsonBytes(value.effectiveSource);
  value.options.effectiveSourceSha256 = sha256(effectiveBytes);
  value.deps.discover = async () => ({
    effectiveSource: value.effectiveSource, bytes: effectiveBytes, sha256: sha256(effectiveBytes),
  });
  delete value.deps.stageClosure;
  await assert.rejects(() => convertProviderSource(value.options, value.deps), (error) => error.code === 'ENOENT');
  await assert.rejects(
    fs.stat(path.join(value.options.stagingRoot, 'closure')),
    (error) => error.code === 'ENOENT',
  );
});

test('reports cleanup failure and still attempts every owned path', async (t) => {
  const value = await scenario(t);
  const removed = [];
  value.deps.remove = async (target, options) => {
    removed.push(target);
    if (target === path.join(value.options.stagingRoot, 'provider-output')) {
      const error = new Error('provider kept an output file locked');
      error.code = 'EBUSY';
      throw error;
    }
    await fs.rm(target, options);
  };
  await assert.rejects(
    () => convertProviderSource(value.options, value.deps),
    (error) => error.code === 'reference-staging-cleanup-failed' && error.retryable === true,
  );
  assert.deepEqual(removed, [
    value.capture.stagingRoot,
    path.join(value.options.stagingRoot, 'closure'),
    path.join(value.options.stagingRoot, 'provider-output'),
    path.join(value.options.stagingRoot, 'effective-source.json'),
  ]);
});
