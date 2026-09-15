import assert from 'node:assert/strict';
import { generateKeyPairSync, sign } from 'node:crypto';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';

import { canonicalJsonBytes, sha256 } from './model-contract.mjs';
import { preflightEnrolledProviderPackage } from './model-provider-package.mjs';

async function fixture(t) {
  const root = await fs.mkdtemp(path.join(await fs.realpath(os.tmpdir()), 'aware-provider-package-'));
  t.after(() => fs.rm(root, { recursive: true, force: true }));
  const home = path.join(root, 'home'); const packageRoot = path.join(root, 'package');
  await fs.mkdir(packageRoot, { recursive: true });
  const launcherBytes = Buffer.from('synthetic executable image');
  await fs.writeFile(path.join(packageRoot, 'provider.bin'), launcherBytes);
  const { privateKey, publicKey } = generateKeyPairSync('ed25519');
  const publicBytes = publicKey.export({ format: 'der', type: 'spki' }).subarray(-32);
  const publicKeyBase64 = publicBytes.toString('base64');
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
    files: [{ path: 'provider.bin', bytes: launcherBytes.length, sha256: sha256(launcherBytes) }],
  };
  const manifestBytes = canonicalJsonBytes(manifest); const manifestSha256 = sha256(manifestBytes);
  const signature = sign(null, Buffer.from(manifestSha256, 'hex'), privateKey).toString('base64');
  await fs.writeFile(path.join(packageRoot, 'provider-package.json'), manifestBytes);
  await fs.writeFile(path.join(packageRoot, 'provider-package.sig'), `ed25519-signature-v1\nover-sha256-of: provider-package.json\nsha256: ${manifestSha256}\nsignature: ${signature}\npublic-key: ${publicKeyBase64}\n`);
  const publisher = {
    schemaVersion: 'aware.model-provider-publisher/v1', publisherId: 'publisher.synthetic',
    keyFingerprintSha256: publisherFingerprintSha256, publicKeyBase64, trusted: true,
  };
  const enrollment = {
    schemaVersion: 'aware.model-provider-enrollment/v1', manifestSha256, packageRoot,
    publisherFingerprintSha256, manifest, enrolled: true, revoked: false,
  };
  const selection = {
    schemaVersion: 'aware.model-provider-selection/v1', formatId: manifest.formatId, generation: 1,
    activeManifestSha256: manifestSha256, previousManifestSha256: [],
  };
  for (const [pathname, value] of [
    [path.join(home, 'providers', 'publishers', `${publisherFingerprintSha256}.json`), publisher],
    [path.join(home, 'providers', 'packages', `${manifestSha256}.json`), enrollment],
    [path.join(home, 'providers', 'selections', `${manifest.formatId}.json`), selection],
  ]) {
    await fs.mkdir(path.dirname(pathname), { recursive: true });
    await fs.writeFile(pathname, canonicalJsonBytes(value));
  }
  return { home, packageRoot, manifest, manifestSha256, capability };
}

test('package preflight exposes only enrolled manifest identity and capability', async (t) => {
  const value = await fixture(t); const calls = [];
  const result = await preflightEnrolledProviderPackage({
    home: value.home, formatId: value.manifest.formatId, capabilityId: value.capability.capabilityId,
    manifestSha256: value.manifestSha256, environment: { AWARE_RUNTIME_VERSION: '0.137.2' },
    hostRun: async (request) => {
      calls.push(request);
      return { exitCode: 0, stdout: canonicalJsonBytes({ protocolVersion: '3', capabilityId: value.capability.capabilityId, ready: true }), stderr: Buffer.alloc(0) };
    },
  });
  assert.equal(calls.length, 1);
  assert.deepEqual(JSON.parse(calls[0].stdin), {
    capabilityId: 'capability.synthetic', formatId: 'format.synthetic', operation: 'describe',
    packageManifestSha256: value.manifestSha256, protocolVersion: '3',
  });
  assert.equal(result.providerPackageManifestSha256, value.manifestSha256);
  assert.equal(result.capability.capabilityId, 'capability.synthetic');
  assert.equal(result.package.packageId, 'package.synthetic');
  assert.equal(JSON.stringify(result).includes(value.packageRoot), false);
  assert.equal(JSON.stringify(result).includes('provider.bin'), false);
});

test('package preflight refuses mutation before provider launch', async (t) => {
  const value = await fixture(t); let launches = 0;
  await fs.writeFile(path.join(value.packageRoot, 'provider.bin'), 'changed');
  await assert.rejects(() => preflightEnrolledProviderPackage({
    home: value.home, formatId: value.manifest.formatId, capabilityId: value.capability.capabilityId,
    manifestSha256: value.manifestSha256, environment: { AWARE_RUNTIME_VERSION: '0.137.2' },
    hostRun: async () => { launches += 1; return null; },
  }), (error) => error.code === 'reference-provider-package-changed');
  assert.equal(launches, 0);
});

test('package preflight brackets provider execution with a second full verification', async (t) => {
  const value = await fixture(t);
  await assert.rejects(() => preflightEnrolledProviderPackage({
    home: value.home, formatId: value.manifest.formatId, capabilityId: value.capability.capabilityId,
    manifestSha256: value.manifestSha256, environment: { AWARE_RUNTIME_VERSION: '0.137.2' },
    hostRun: async () => {
      await fs.writeFile(path.join(value.packageRoot, 'provider.bin'), 'changed after launch');
      return { exitCode: 0, stdout: canonicalJsonBytes({ protocolVersion: '3', capabilityId: value.capability.capabilityId, ready: true }), stderr: Buffer.alloc(0) };
    },
  }), (error) => error.code === 'reference-provider-package-changed');
});

test('package preflight refuses stale selection and undeclared capability without launching', async (t) => {
  const value = await fixture(t); let launches = 0;
  for (const [manifestSha256, capabilityId, code] of [
    ['0'.repeat(64), value.capability.capabilityId, 'reference-provider-package-pin-mismatch'],
    [value.manifestSha256, 'capability.other', 'reference-provider-capability-unavailable'],
  ]) {
    await assert.rejects(() => preflightEnrolledProviderPackage({
      home: value.home, formatId: value.manifest.formatId, capabilityId, manifestSha256,
      environment: { AWARE_RUNTIME_VERSION: '0.137.2' },
      hostRun: async () => { launches += 1; return null; },
    }), (error) => error.code === code);
  }
  assert.equal(launches, 0);
});

test('package preflight refuses malformed or unbounded selection history without launching', async (t) => {
  const value = await fixture(t); let launches = 0;
  const selectionPath = path.join(value.home, 'providers', 'selections', `${value.manifest.formatId}.json`);
  const base = {
    schemaVersion: 'aware.model-provider-selection/v1', formatId: value.manifest.formatId,
    generation: 2, activeManifestSha256: value.manifestSha256,
  };
  for (const previousManifestSha256 of [
    ['invalid'],
    ['1'.repeat(64), '1'.repeat(64)],
    Array.from({ length: 9 }, (_, index) => index.toString(16).padStart(64, '0')),
    [value.manifestSha256],
  ]) {
    await fs.writeFile(selectionPath, canonicalJsonBytes({ ...base, previousManifestSha256 }));
    await assert.rejects(() => preflightEnrolledProviderPackage({
      home: value.home, formatId: value.manifest.formatId, capabilityId: value.capability.capabilityId,
      manifestSha256: value.manifestSha256, environment: { AWARE_RUNTIME_VERSION: '0.137.2' },
      hostRun: async () => { launches += 1; return null; },
    }), (error) => error.code === 'reference-provider-package-pin-mismatch');
  }
  assert.equal(launches, 0);
});

test('package preflight rechecks its AWARE compatibility range before launch', async (t) => {
  const value = await fixture(t); let launches = 0;
  await assert.rejects(() => preflightEnrolledProviderPackage({
    home: value.home, formatId: value.manifest.formatId, capabilityId: value.capability.capabilityId,
    manifestSha256: value.manifestSha256, environment: { AWARE_RUNTIME_VERSION: '0.136.9' },
    hostRun: async () => { launches += 1; return null; },
  }), (error) => error.code === 'reference-provider-package-incompatible');
  assert.equal(launches, 0);
});
