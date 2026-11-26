import assert from 'node:assert/strict';
import { generateKeyPairSync, sign } from 'node:crypto';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';

import { canonicalJsonBytes, ModelReaderError, safeErrorEnvelope, sha256 } from './model-contract.mjs';
import { fingerprintSource } from './model-provider-discovery.mjs';

async function fixture(t) {
  const root = await fs.mkdtemp(path.join(await fs.realpath(os.tmpdir()), 'aware-discovery-'));
  t.after(() => fs.rm(root, { recursive: true, force: true }));
  const home = path.join(root, 'home'); const packageRoot = path.join(root, 'package');
  const source = path.join(root, 'source'); const stagingRoot = path.join(root, 'capture');
  await fs.mkdir(packageRoot, { recursive: true }); await fs.mkdir(source);
  await fs.writeFile(path.join(source, 'main.synthetic'), 'main');
  await fs.writeFile(path.join(source, 'notes.txt'), 'notes');
  await fs.writeFile(path.join(source, 'support.bin'), 'support');
  const launcherBytes = Buffer.from('synthetic executable image');
  const launcherSha256 = sha256(launcherBytes);
  await fs.writeFile(path.join(packageRoot, 'provider.bin'), launcherBytes);
  const { privateKey, publicKey } = generateKeyPairSync('ed25519');
  const publicBytes = publicKey.export({ format: 'der', type: 'spki' }).subarray(-32);
  const publisherFingerprintSha256 = sha256(publicBytes);
  const capability = {
    capabilityId: 'capability.synthetic', protocolVersion: '3', sourceCaptureMode: 'capture.synthetic',
    requestSchema: 'request.synthetic.v1', resultSchema: 'result.synthetic.v1',
    artifactRootVersion: 'artifact.synthetic.v1', cacheNamespaceVersion: 'cache.synthetic.v1',
  };
  const manifest = {
    schemaVersion: 'aware.model-provider-package/v1', packageId: 'package.synthetic', packageVersion: '1.2.3',
    formatId: 'format.synthetic', launcher: 'provider.bin', minimumAwareVersion: '0.137.0', maximumAwareVersion: null,
    publisherFingerprintSha256, capabilities: [capability],
    files: [{ path: 'provider.bin', bytes: launcherBytes.length, sha256: launcherSha256 }],
  };
  const manifestBytes = canonicalJsonBytes(manifest); const manifestSha256 = sha256(manifestBytes);
  const signature = sign(null, Buffer.from(manifestSha256, 'hex'), privateKey).toString('base64');
  await fs.writeFile(path.join(packageRoot, 'provider-package.json'), manifestBytes);
  await fs.writeFile(path.join(packageRoot, 'provider-package.sig'), `ed25519-signature-v1\nover-sha256-of: provider-package.json\nsha256: ${manifestSha256}\nsignature: ${signature}\npublic-key: ${publicBytes.toString('base64')}\n`);
  const records = [
    [path.join(home, 'providers', 'publishers', `${publisherFingerprintSha256}.json`), {
      schemaVersion: 'aware.model-provider-publisher/v1', publisherId: 'publisher.synthetic',
      keyFingerprintSha256: publisherFingerprintSha256, publicKeyBase64: publicBytes.toString('base64'), trusted: true,
    }],
    [path.join(home, 'providers', 'packages', `${manifestSha256}.json`), {
      schemaVersion: 'aware.model-provider-enrollment/v1', manifestSha256, packageRoot,
      publisherFingerprintSha256, manifest, enrolled: true, revoked: false,
    }],
    [path.join(home, 'providers', 'selections', 'format.synthetic.json'), {
      schemaVersion: 'aware.model-provider-selection/v1', formatId: 'format.synthetic', generation: 1,
      activeManifestSha256: manifestSha256, previousManifestSha256: [],
    }],
  ];
  for (const [pathname, value] of records) {
    await fs.mkdir(path.dirname(pathname), { recursive: true });
    await fs.writeFile(pathname, canonicalJsonBytes(value));
  }
  const providerIdentity = {
    schemaVersion: 'aware.enrolled-model-provider-fingerprint/v1', execution: 'enrolled-local',
    packageManifestSha256: manifestSha256, launcherSha256, publisherFingerprintSha256,
    formatId: manifest.formatId, capability,
  };
  const policy = {
    schemaVersion: 'aware.model-dependency-policy/v1', policyId: 'synthetic-policy-v1',
    capabilityId: capability.capabilityId, providerFingerprintSha256: sha256(canonicalJsonBytes(providerIdentity)),
    roles: [
      { role: 'primary', classification: 'mandatory', affectedDomains: [] },
      { role: 'support', classification: 'optional', affectedDomains: [] },
    ],
  };
  await fs.mkdir(path.join(home, 'providers', 'policies'), { recursive: true });
  await fs.writeFile(
    path.join(home, 'providers', 'policies', `${policy.providerFingerprintSha256}.json`),
    canonicalJsonBytes(policy),
  );
  return {
    root, home, packageRoot, source, stagingRoot, manifestSha256, capability, policy,
    base: {
      home, formatId: manifest.formatId, capabilityId: capability.capabilityId, manifestSha256,
      environment: { AWARE_RUNTIME_VERSION: '0.138.0' }, authorization: 'synthetic-capability',
      namespaces: [{ id: 'model', root: source }], stagingRoot,
    },
  };
}

function discoveryReport(request, capture) {
  return {
    schemaVersion: 'aware.model-provider-dependency-report/v1', protocolVersion: '3',
    capabilityId: request.capabilityId, captureManifestSha256: request.capture.manifestSha256,
    primary: { namespaceId: 'model', path: 'main.synthetic', role: 'primary' },
    files: capture.namespaces[0].files.map((file) => ({
      namespaceId: 'model', path: file.path,
      role: file.path === 'main.synthetic' ? 'primary' : file.path === 'support.bin' ? 'support' : null,
      disposition: file.path === 'notes.txt' ? 'ignored' : 'consumed',
    })),
    absent: [], unsupportedExternal: [],
    authentication: [{ kind: 'framing', subject: 'primary', evidence: 'recognized' }],
    crossFileEvidence: [{ kind: 'identity-match', subject: 'primary-support', evidence: 'matched' }],
  };
}

test('fingerprint captures privately, discovers only from staging, and removes staging', async (t) => {
  const value = await fixture(t); let request;
  const result = await fingerprintSource({
    ...value.base,
    hostRun: async (call) => {
      request = JSON.parse(call.stdin);
      assert.notEqual(request.capture.root, value.source);
      assert.equal(request.capture.root, value.stagingRoot);
      const capture = JSON.parse(await fs.readFile(path.join(request.capture.root, request.capture.manifestPath)));
      return { exitCode: 0, stdout: canonicalJsonBytes(discoveryReport(request, capture)), stderr: Buffer.alloc(0) };
    },
  });
  assert.equal(request.operation, 'discover');
  assert.equal(request.authorization, 'synthetic-capability');
  assert.deepEqual(result.effectiveSource.consumed.map((file) => file.path), ['main.synthetic', 'support.bin']);
  assert.equal(result.effectiveSource.providerPackageManifestSha256, value.manifestSha256);
  await assert.rejects(fs.stat(value.stagingRoot), (error) => error.code === 'ENOENT');
});

test('fingerprint brackets discovery with complete package verification', async (t) => {
  const value = await fixture(t);
  await assert.rejects(() => fingerprintSource({
    ...value.base,
    hostRun: async (call) => {
      const request = JSON.parse(call.stdin);
      const capture = JSON.parse(await fs.readFile(path.join(request.capture.root, 'capture.json')));
      await fs.writeFile(path.join(value.packageRoot, 'provider.bin'), 'mutated');
      return { exitCode: 0, stdout: canonicalJsonBytes(discoveryReport(request, capture)), stderr: Buffer.alloc(0) };
    },
  }), (error) => error.code === 'reference-provider-package-changed');
  await assert.rejects(fs.stat(value.stagingRoot), (error) => error.code === 'ENOENT');
});

test('fingerprint refuses missing authorization before launch and still cleans capture', async (t) => {
  const value = await fixture(t); let launches = 0;
  await assert.rejects(() => fingerprintSource({
    ...value.base, authorization: undefined,
    hostRun: async () => { launches += 1; return null; },
  }), (error) => error.code === 'reference-provider-authorization-invalid');
  assert.equal(launches, 0);
  await assert.rejects(fs.stat(value.stagingRoot), (error) => error.code === 'ENOENT');
});

test('fingerprint rejects staged bytes changed by the provider', async (t) => {
  const value = await fixture(t);
  await assert.rejects(() => fingerprintSource({
    ...value.base,
    hostRun: async (call) => {
      const request = JSON.parse(call.stdin);
      const capture = JSON.parse(await fs.readFile(path.join(request.capture.root, 'capture.json')));
      const stagedPrimary = path.join(request.capture.root, 'namespaces', 'model', 'main.synthetic');
      await fs.chmod(stagedPrimary, 0o600);
      await fs.writeFile(stagedPrimary, 'evil');
      return { exitCode: 0, stdout: canonicalJsonBytes(discoveryReport(request, capture)), stderr: Buffer.alloc(0) };
    },
  }), (error) => error.code === 'reference-source-changed');
  await assert.rejects(fs.stat(value.stagingRoot), (error) => error.code === 'ENOENT');
});

test('fingerprint preserves managed-host timeout and cancellation semantics', async (t) => {
  const timeoutValue = await fixture(t);
  const timeout = new ModelReaderError('reference-provider-timeout', 'provider-host', true, 'timed out');
  await assert.rejects(() => fingerprintSource({
    ...timeoutValue.base, hostRun: async () => { throw timeout; },
  }), (error) => error === timeout);

  const cancelledValue = await fixture(t); const controller = new AbortController();
  await assert.rejects(() => fingerprintSource({
    ...cancelledValue.base, signal: controller.signal,
    hostRun: async () => { controller.abort(); throw new Error('terminated'); },
  }), (error) => error.code === 'reference-cancelled' && error.retryable === false);
});

test('fingerprint refuses an admitted policy changed during discovery', async (t) => {
  const value = await fixture(t);
  await assert.rejects(() => fingerprintSource({
    ...value.base,
    hostRun: async (call) => {
      const request = JSON.parse(call.stdin);
      const capture = JSON.parse(await fs.readFile(path.join(request.capture.root, 'capture.json')));
      const policyPath = path.join(
        value.home, 'providers', 'policies', `${value.policy.providerFingerprintSha256}.json`,
      );
      await fs.writeFile(policyPath, canonicalJsonBytes({ ...value.policy, policyId: 'synthetic-policy-v2' }));
      return { exitCode: 0, stdout: canonicalJsonBytes(discoveryReport(request, capture)), stderr: Buffer.alloc(0) };
    },
  }), (error) => error.code === 'reference-dependency-policy-changed');
});

test('fingerprint refuses a malformed admitted policy before provider launch', async (t) => {
  const value = await fixture(t); let launches = 0;
  const policyPath = path.join(
    value.home, 'providers', 'policies', `${value.policy.providerFingerprintSha256}.json`,
  );
  await fs.writeFile(policyPath, canonicalJsonBytes({ ...value.policy, capabilityId: 'capability.other' }));
  await assert.rejects(() => fingerprintSource({
    ...value.base, hostRun: async () => { launches += 1; return null; },
  }), (error) => error.code === 'reference-dependency-policy-invalid');
  assert.equal(launches, 0);
});

// #883: discovery used to flatten every provider refusal into a retryable failure.
const PATH_PROVIDER_ID = '123e4567-e89b-42d3-a456-426614174000';
const LONG_LOGICAL = `Shapes/${'s'.repeat(200)}/profile.dat`;

async function withLongSource(value) {
  await fs.mkdir(path.join(value.source, 'Shapes', 's'.repeat(200)), { recursive: true });
  await fs.writeFile(path.join(value.source, ...LONG_LOGICAL.split('/')), 'shape');
  return value;
}

function pathRefusal(request, logical = LONG_LOGICAL) {
  const length = `${request.capture.root}\\namespaces\\model\\${logical.replaceAll('/', '\\')}`.length;
  return canonicalJsonBytes({
    code: 'reference-source-path-too-long', phase: 'discovery', retryable: false,
    message: `path-length-v1:${length},259`, diagnosticId: PATH_PROVIDER_ID,
    source: { namespaceId: 'model', path: logical },
  });
}

function refuse(call) {
  return { exitCode: 2, stdout: Buffer.alloc(0), stderr: pathRefusal(JSON.parse(call.stdin)) };
}

test('discovery admits a corroborated source-path refusal as non-retryable', async (t) => {
  const value = await withLongSource(await fixture(t));
  await assert.rejects(() => fingerprintSource({ ...value.base, platform: 'win32', hostRun: async (call) => refuse(call) }),
    (error) => {
      const safe = safeErrorEnvelope(error);
      assert.equal(safe.code, 'reference-source-path-too-long');
      assert.equal(safe.phase, 'discovery');
      assert.equal(safe.retryable, false);
      assert.notEqual(safe.diagnosticId, PATH_PROVIDER_ID);
      assert.equal(safe.details.sourcePath, LONG_LOGICAL);
      assert.equal(safe.details.pathLimit, '259');
      return true;
    });
  await assert.rejects(fs.stat(value.stagingRoot), (error) => error.code === 'ENOENT');
});

test('control: discovery keeps the generic retryable failure for a refusal in the wrong phase', async (t) => {
  const value = await withLongSource(await fixture(t));
  await assert.rejects(() => fingerprintSource({
    ...value.base, platform: 'win32',
    hostRun: async (call) => {
      const stderr = pathRefusal(JSON.parse(call.stdin)).toString('utf8').replace('"discovery"', '"conversion"');
      return { exitCode: 2, stdout: Buffer.alloc(0), stderr: Buffer.from(stderr) };
    },
  }), (error) => error.code === 'reference-provider-failed' && error.retryable === true);
});

test('discovery integrity failures supersede a source-path refusal', async (t) => {
  const cases = {
    'reference-source-changed': async (_value, request) => {
      const staged = path.join(request.capture.root, 'namespaces', 'model', 'main.synthetic');
      await fs.chmod(staged, 0o600); await fs.writeFile(staged, 'evil');
    },
    'reference-provider-package-changed': async (value) => {
      await fs.writeFile(path.join(value.packageRoot, 'provider.bin'), 'mutated');
    },
    'reference-dependency-policy-changed': async (value) => {
      await fs.writeFile(path.join(value.home, 'providers', 'policies', `${value.policy.providerFingerprintSha256}.json`),
        canonicalJsonBytes({ ...value.policy, policyId: 'synthetic-policy-v2' }));
    },
  };
  for (const [expected, mutate] of Object.entries(cases)) {
    const value = await withLongSource(await fixture(t));
    await assert.rejects(() => fingerprintSource({
      ...value.base, platform: 'win32',
      hostRun: async (call) => { await mutate(value, JSON.parse(call.stdin)); return refuse(call); },
    }), (error) => { assert.equal(error.code, expected); return true; });
  }
});

test('a staging cleanup failure cannot replace the discovery refusal', async (t) => {
  const value = await withLongSource(await fixture(t));
  const cleanup = new Error('EBUSY: staging held open');
  const removeStaging = async (...args) => { await fs.rm(...args); throw cleanup; };
  await assert.rejects(() => fingerprintSource({
    ...value.base, platform: 'win32', removeStaging, hostRun: async (call) => refuse(call),
  }), (error) => {
    assert.equal(error.code, 'reference-source-path-too-long');
    assert.equal(error.cleanupError, cleanup);
    return true;
  });
  // Control: with no primary failure the cleanup failure still surfaces.
  const clean = await fixture(t);
  await assert.rejects(() => fingerprintSource({
    ...clean.base, removeStaging,
    hostRun: async (call) => {
      const request = JSON.parse(call.stdin);
      const capture = JSON.parse(await fs.readFile(path.join(request.capture.root, 'capture.json')));
      return { exitCode: 0, stdout: canonicalJsonBytes(discoveryReport(request, capture)), stderr: Buffer.alloc(0) };
    },
  }), (error) => error.code === 'reference-provider-run-cleanup-failed');
});
