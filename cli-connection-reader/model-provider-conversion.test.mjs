import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';

import { canonicalJsonBytes, sha256 } from './model-contract.mjs';
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
    loadPackage: async () => loaded,
    loadPolicy: async () => ({ policy, sha256: policySha256 }),
    verifyCapture: async () => true,
    verifyOutput: async (root, verifyOptions) => ({ root: verifyOptions.admittedRoot, sourceRoot: root, verifyOptions }),
  };
  return { options, deps, calls, bytes };
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
  assert.equal(control.conversionRequest.effectiveSourceSha256, value.options.effectiveSourceSha256);
  assert.equal(control.authorization, 'short-lived-token');
  assert.equal(value.calls[0].stdin.includes(Buffer.from('D:\\model')), false);
  assert.equal(result.output.verifyOptions.conversionRequestSha256, result.conversionRequestSha256);
  await assert.rejects(fs.stat(path.join(value.options.stagingRoot, 'provider-output')), (error) => error.code === 'ENOENT');
  assert.equal(result.output.root, path.join(value.options.stagingRoot, 'admitted-output'));
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
