import assert from 'node:assert/strict';
import { createHash, createPublicKey, generateKeyPairSync, verify } from 'node:crypto';
import { spawn } from 'node:child_process';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';
import { sha256 } from './model-contract.mjs';
import { runModelCommand } from './model-reader.mjs';

const fixture = fileURLToPath(new URL('./test-fixtures/model-provider-fixture.mjs', import.meta.url));
const PUBLIC_PREFIX = Buffer.from('302a300506032b6570032100', 'hex');

function uint64be(value) {
  const bytes = Buffer.alloc(8);
  bytes.writeBigUInt64BE(BigInt(value));
  return bytes;
}

function canonicalFixture(value) {
  if (Array.isArray(value)) return `[${value.map(canonicalFixture).join(',')}]`;
  if (value && typeof value === 'object') {
    return `{${Object.keys(value).sort().map((key) => `${JSON.stringify(key)}:${canonicalFixture(value[key])}`).join(',')}}`;
  }
  return JSON.stringify(value);
}

function verifyEnvelope(domain, preimage, envelope) {
  const preimageBytes = Buffer.from(canonicalFixture(preimage), 'utf8');
  const digest = createHash('sha256').update(preimageBytes).digest('hex');
  assert.equal(envelope.preimageSha256, digest);
  const signatureInput = createHash('sha256')
    .update(Buffer.from(domain, 'ascii'))
    .update(uint64be(preimageBytes.length))
    .update(preimageBytes)
    .digest();
  const publicKey = createPublicKey({
    key: Buffer.concat([PUBLIC_PREFIX, Buffer.from(envelope.publicKeyBase64, 'base64')]),
    format: 'der', type: 'spki',
  });
  assert.equal(verify(null, signatureInput, publicKey, Buffer.from(envelope.signatureBase64, 'base64')), true);
}

async function setup(t) {
  const root = await fs.mkdtemp(path.join(await fs.realpath(os.tmpdir()), 'aware-model-reader-'));
  t.after(() => fs.rm(root, { recursive: true, force: true }));
  const sourcePath = path.join(root, 'fixture.rvt');
  const executable = path.join(root, 'provider.exe');
  await fs.writeFile(sourcePath, 'fixture-rvt');
  await fs.writeFile(executable, 'fixture-provider-binary');
  const { privateKey, publicKey } = generateKeyPairSync('ed25519');
  const secretPath = path.join(root, 'reader.sec'); const publicPath = path.join(root, 'reader.pub');
  await fs.writeFile(secretPath, `ed25519-secret-key-v1 ${privateKey.export({ format: 'der', type: 'pkcs8' }).subarray(-32).toString('base64')}\n`);
  await fs.writeFile(publicPath, `ed25519-public-key-v1 ${publicKey.export({ format: 'der', type: 'spki' }).subarray(-32).toString('base64')}\n`);
  const calls = [];
  const hostRequests = [];
  const lockCalls = [];
  let nextLock = 0;
  const hostRun = async (request) => {
    calls.push(request.operation);
    hostRequests.push(request);
    return await new Promise((resolve, reject) => {
      const child = spawn(process.execPath, [fixture, request.operation], { cwd: request.cwd, env: request.environment, windowsHide: true, shell: false, stdio: ['pipe', 'pipe', 'pipe'] });
      const stdout = []; const stderr = [];
      child.stdout.on('data', (chunk) => stdout.push(chunk)); child.stderr.on('data', (chunk) => stderr.push(chunk));
      child.on('error', reject); child.on('close', (exitCode) => resolve({ exitCode, stdout: Buffer.concat(stdout), stderr: Buffer.concat(stderr) }));
      child.stdin.end(request.stdin);
    });
  };
  return {
    root, sourcePath, executable, secretPath, publicPath, calls, hostRequests, lockCalls,
    deps: {
      hostRun,
      hostAcquireLock: async (lockPath, options) => {
        lockCalls.push({ lockPath, options });
        return Buffer.from(`lock-${nextLock += 1}`);
      },
      hostReleaseLock: async () => {},
      cacheRoot: path.join(root, 'cache'), privateRoot: path.join(root, 'runs'), artifactDirectory: path.join(root, 'artifacts'),
    },
    args: { 'rvt-path': sourcePath, 'source-sha256': sha256(Buffer.from('fixture-rvt')), 'provider-path': executable, 'signing-secret-path': secretPath, 'signing-public-path': publicPath },
  };
}

test('request-only failures precede provider configuration and managed host setup', async () => {
  const unconfigured = { environment: {} };
  await assert.rejects(
    () => runModelCommand('preflight', { limits: null }, unconfigured),
    (error) => error.code === 'reference-limits-invalid',
  );
  await assert.rejects(
    () => runModelCommand('preflight', { 'expected-provider-protocol': '9' }, unconfigured),
    (error) => error.code === 'reference-request-invalid',
  );
  await assert.rejects(
    () => runModelCommand('read-model', {
      'rvt-path': path.resolve('admission-only.rvt'),
      'source-sha256': '0'.repeat(64),
    }, unconfigured),
    (error) => error.code === 'reference-provider-pin-required',
  );
});

test('preflight describes provider and key readiness without conversion or source access', async (t) => {
  const state = await setup(t);
  const out = await runModelCommand('preflight', {
    'provider-path': state.executable, 'signing-secret-path': state.secretPath, 'signing-public-path': state.publicPath,
  }, state.deps);
  assert.equal(out.ready, true);
  assert.equal(out.execution, 'local');
  assert.match(out.providerFingerprintSha256, /^[0-9a-f]{64}$/);
  assert.equal(Buffer.from(out.signerPublicKeyBase64, 'base64').length, 32);
  assert.equal(sha256(Buffer.from(out.signerPublicKeyBase64, 'base64')), out.signerFingerprintSha256);
  assert.deepEqual(state.calls, ['describe']);
});
test('preflight enforces the managed authority-store contract before provider launch', async (t) => {
  const state = await setup(t);
  const base = {
    'provider-path': state.executable,
    'signing-secret-path': state.secretPath,
    'signing-public-path': state.publicPath,
  };
  await assert.rejects(() => runModelCommand('preflight', {
    ...base,
    'expected-provider-protocol': '2',
    'expected-provider-destination': 'https://api.example.test',
  }, state.deps), (error) => error.code === 'reference-provider-protocol');
  await assert.rejects(() => runModelCommand('preflight', {
    ...base,
    'expected-provider-protocol': '1',
    'authority-store-path': path.join(state.root, 'authority'),
  }, state.deps), (error) => error.code === 'reference-provider-protocol');
  assert.deepEqual(state.calls, []);
});

test('request limits are validated once and propagated through provider and package boundaries', async (t) => {
  const state = await setup(t);
  const limits = { maxSourceBytes: 32, maxCanonicalGlbBytes: 1024 * 1024 };
  const preflight = await runModelCommand('preflight', {
    'provider-path': state.executable, 'signing-secret-path': state.secretPath,
    'signing-public-path': state.publicPath, limits,
  }, state.deps);
  const out = await runModelCommand('read-snapshot', {
    ...state.args, limits,
    'expected-provider-sha256': preflight.providerFingerprintSha256,
    'expected-signer-sha256': preflight.signerFingerprintSha256,
  }, state.deps);
  assert.equal(out.packageConfiguration.maximumTileBytes, limits.maxCanonicalGlbBytes);
  for (const request of state.hostRequests) {
    const body = JSON.parse(request.stdin.toString('utf8'));
    assert.equal(body.limits.maxSourceBytes, limits.maxSourceBytes);
    assert.equal(body.limits.maxCanonicalGlbBytes, limits.maxCanonicalGlbBytes);
  }
  await assert.rejects(
    () => runModelCommand('preflight', {
      'provider-path': state.executable, 'signing-secret-path': state.secretPath,
      'signing-public-path': state.publicPath, limits: { unknownLimit: 1 },
    }, state.deps),
    (error) => error.code === 'reference-limits-invalid',
  );
  await assert.rejects(
    () => runModelCommand('preflight', {
      'provider-path': state.executable, 'signing-secret-path': state.secretPath,
      'signing-public-path': state.publicPath, limits: null,
    }, state.deps),
    (error) => error.code === 'reference-limits-invalid',
  );
  await assert.rejects(
    () => runModelCommand('preflight', {
      'provider-path': state.executable, 'signing-secret-path': state.secretPath,
      'signing-public-path': state.publicPath, limits: { maxSourceBytes: null },
    }, state.deps),
    (error) => error.code === 'reference-limits-invalid',
  );
});

test('starting a model command reclaims only stale abandoned provider run directories', async (t) => {
  const state = await setup(t);
  const stale = path.join(state.deps.privateRoot, 'run-abandoned');
  const active = path.join(state.deps.privateRoot, 'run-active');
  const current = path.join(state.deps.privateRoot, 'run-current');
  await fs.mkdir(stale, { recursive: true });
  await fs.writeFile(path.join(stale, 'sensitive.rvt'), 'staged-model');
  await fs.writeFile(path.join(stale, '.active'), '');
  await fs.mkdir(active, { recursive: true });
  await fs.writeFile(path.join(active, '.active'), '');
  await fs.mkdir(current, { recursive: true });
  const old = new Date(Date.now() - (2 * 60 * 60_000));
  await fs.utimes(stale, old, old);
  await fs.utimes(path.join(stale, '.active'), old, old);
  await fs.utimes(active, old, old);
  await runModelCommand('preflight', {
    'provider-path': state.executable, 'signing-secret-path': state.secretPath,
    'signing-public-path': state.publicPath,
  }, state.deps);
  await assert.rejects(() => fs.access(stale), (error) => error.code === 'ENOENT');
  await fs.access(active);
  await fs.access(current);
});

test('read-model publishes five binary-safe artifacts with reconciled coverage and provenance', async (t) => {
  const state = await setup(t);
  const preflight = await runModelCommand('preflight', {
    'provider-path': state.executable, 'signing-secret-path': state.secretPath, 'signing-public-path': state.publicPath,
  }, state.deps);
  const out = await runModelCommand('read-model', {
    ...state.args, 'expected-provider-sha256': preflight.providerFingerprintSha256,
    'expected-signer-sha256': preflight.signerFingerprintSha256,
  }, state.deps);
  assert.equal(out.schemaVersion, 'model-reference-reader/v1');
  assert.equal(out.frame.units, 'mm'); assert.equal(out.frame.up, 'z');
  assert.equal(out.coverage.discoveredEntities, 1);
  assert.equal(Object.keys(out.artifacts).length, 5);
  for (const descriptor of Object.values(out.artifacts)) {
    assert.deepEqual(Object.keys(descriptor).sort(), ['bytes', 'id', 'mediaType', 'sha256']);
    assert.match(descriptor.id, /^[a-z0-9.-]+$/);
    const bytes = await fs.readFile(path.join(state.deps.artifactDirectory, descriptor.id));
    assert.equal(sha256(bytes), descriptor.sha256);
    assert.equal(bytes.length, descriptor.bytes);
  }
  const geometry = await fs.readFile(path.join(state.deps.artifactDirectory, out.artifacts.geometry.id));
  assert.equal(geometry.readUInt32LE(0), 0x46546c67);
  assert.equal('sourceArtifactEnvelope' in out, false);
  assert.equal('sourceArtifactPreimage' in out, false);
});

test('reader v2 binds expansion limits and publishes tagged provider-display property artifacts', async (t) => {
  const state = await setup(t);
  const versionArgs = {
    'reader-schema-version': 'model-reference-reader/v2',
    'property-expansion-limits': { maxExpandedPropertyRows: 100, maxCanonicalPropertyBytes: 4096 },
  };
  const preflight = await runModelCommand('preflight', {
    'provider-path': state.executable, 'signing-secret-path': state.secretPath,
    'signing-public-path': state.publicPath, ...versionArgs,
  }, state.deps);
  assert.equal(preflight.schemaVersion, 'model-reference-reader/v2');
  const out = await runModelCommand('read-snapshot', {
    ...state.args, ...versionArgs,
    'expected-provider-sha256': preflight.providerFingerprintSha256,
    'expected-signer-sha256': preflight.signerFingerprintSha256,
  }, state.deps);
  assert.equal(out.schemaVersion, 'model-reference-reader/v2');
  assert.equal(out.coverage.expandedProperties, 1);
  assert.deepEqual(out.coverage.effectivePropertyLimits, versionArgs['property-expansion-limits']);
  assert.equal(out.packageConfiguration.schemaVersion, 'model-reference-package-configuration/v2');
  const properties = JSON.parse(await fs.readFile(path.join(state.deps.artifactDirectory, out.artifacts.properties.id), 'utf8'));
  assert.equal(properties.schemaVersion, '2');
  assert.deepEqual(properties.properties[0], {
    entityId: 'element:1001', groupId: 'parameter-group:1', groupName: 'Identity Data', groupOrdinal: 0,
    parameterId: 'parameter:1', parameterOrdinal: 0, name: 'Display Mark', unit: null,
    valueEncoding: 'provider-display', valueType: 'string', value: 'A-1',
  });
  assert.equal(JSON.stringify(out).includes('A-1'), false, 'summary and receipts must not leak property values');
});

test('read-snapshot derives public source and package envelopes after private cache verification', async (t) => {
  const state = await setup(t);
  const preflight = await runModelCommand('preflight', {
    'provider-path': state.executable, 'signing-secret-path': state.secretPath, 'signing-public-path': state.publicPath,
  }, state.deps);
  const args = {
    ...state.args, 'expected-provider-sha256': preflight.providerFingerprintSha256,
    'expected-signer-sha256': preflight.signerFingerprintSha256,
  };
  const out = await runModelCommand('read-snapshot', args, state.deps);

  assert.equal(out.schemaVersion, 'model-reference-reader/v1');
  assert.equal(out.sourceArtifactPreimage.outputs.length, 5);
  assert.deepEqual(out.sourceArtifactPreimage.outputs.map((item) => item.logicalName),
    ['geometry', 'entities', 'properties', 'relationships', 'manifest']);
  verifyEnvelope('AWARE\0model-reference-reader\0source-artifact-set\0v1\0', out.sourceArtifactPreimage, out.sourceArtifactEnvelope);

  assert.equal(out.packagePreimage.outputs.length, 6);
  assert.deepEqual(out.packagePreimage.outputs.map((item) => item.logicalName),
    ['manifest', 'tile-000000', 'entities-000000', 'properties-000000', 'relationships-000000', 'index']);
  verifyEnvelope('AWARE\0model-reference-reader\0package-set\0v1\0', out.packagePreimage, out.packageArtifactEnvelope);
  assert.equal(out.packageConfiguration.maximumTileTriangles, 15_000_000);
  assert.equal(out.packageConfiguration.maximumAggregateBytes, (256 * 1024 * 1024) + (32 * 1024 * 1024 * 5));
  assert.equal(out.packagePreimage.source.sourceArtifactPreimageSha256, out.sourceArtifactEnvelope.preimageSha256);

  for (const [logicalName, descriptor] of Object.entries({ ...out.artifacts, ...out.packageArtifacts })) {
    const bytes = await fs.readFile(path.join(state.deps.artifactDirectory, descriptor.id));
    assert.equal(bytes.length, descriptor.bytes, logicalName);
    assert.equal(sha256(bytes), descriptor.sha256, logicalName);
  }
  assert.equal(JSON.stringify(out).includes('receipt.sig'), false);
  assert.equal(JSON.stringify(out).includes('receipt.json'), false);
  assert.equal(state.calls.filter((operation) => operation === 'convert').length, 1);

  const providerCalls = [...state.calls];
  const warm = await runModelCommand('read-snapshot', args, state.deps);
  assert.equal(warm.cache, 'hit');
  assert.deepEqual(state.calls, providerCalls);
  assert.equal(state.calls.filter((operation) => operation === 'convert').length, 1);
  assert.deepEqual(warm.sourceArtifactPreimage, out.sourceArtifactPreimage);
  assert.deepEqual(warm.packagePreimage, out.packagePreimage);
});

test('read-snapshot refuses a tampered private cache receipt before public artifact publication', async (t) => {
  const state = await setup(t);
  const preflight = await runModelCommand('preflight', {
    'provider-path': state.executable, 'signing-secret-path': state.secretPath, 'signing-public-path': state.publicPath,
  }, state.deps);
  const args = {
    ...state.args, 'expected-provider-sha256': preflight.providerFingerprintSha256,
    'expected-signer-sha256': preflight.signerFingerprintSha256,
  };
  await runModelCommand('read-model', args, state.deps);
  const entries = await fs.readdir(path.join(state.deps.cacheRoot, 'entries'));
  assert.equal(entries.length, 1);
  const receiptPath = path.join(state.deps.cacheRoot, 'entries', entries[0], 'receipt.json');
  const receipt = JSON.parse(await fs.readFile(receiptPath, 'utf8'));
  receipt.identitySha256 = '0'.repeat(64);
  await fs.writeFile(receiptPath, JSON.stringify(receipt));

  await assert.rejects(
    () => runModelCommand('read-snapshot', args, state.deps),
    (error) => error.code === 'reference-cache-signature-invalid',
  );
  const published = await fs.readdir(state.deps.artifactDirectory);
  assert.equal(published.some((name) => name.startsWith('snapshot-')), false);
});

test('probe is bounded, cache-aware, and two cold conversions produce identical artifact hashes', async (t) => {
  const first = await setup(t);
  const preflight = await runModelCommand('preflight', { 'provider-path': first.executable, 'signing-secret-path': first.secretPath, 'signing-public-path': first.publicPath }, first.deps);
  const args = {
    ...first.args, 'expected-provider-sha256': preflight.providerFingerprintSha256,
    'expected-signer-sha256': preflight.signerFingerprintSha256,
  };
  const cold = await runModelCommand('read-model', args, first.deps);
  const probe = await runModelCommand('probe', args, first.deps);
  assert.equal(probe.cache, 'hit');
  assert.equal(probe.entities, 1);

  const unclaimed = await setup(t);
  const unclaimedPreflight = await runModelCommand('preflight', {
    'provider-path': unclaimed.executable,
    'signing-secret-path': unclaimed.secretPath,
    'signing-public-path': unclaimed.publicPath,
  }, unclaimed.deps);
  const unclaimedProbe = await runModelCommand('probe', {
    ...unclaimed.args,
    'conversion-settings': { fixtureUnclaimedOffset: true },
    'expected-provider-sha256': unclaimedPreflight.providerFingerprintSha256,
    'expected-signer-sha256': unclaimedPreflight.signerFingerprintSha256,
  }, unclaimed.deps);
  assert.deepEqual(unclaimedProbe.coverage.unclaimedGeometryNodes, ['unclaimed-offset']);
  assert.deepEqual(unclaimedProbe.bounds, { min: [0, 0, 0], max: [11000, 0, 1000] });

  const secondCache = path.join(first.root, 'second-cache');
  const secondArtifacts = path.join(first.root, 'second-artifacts');
  const second = await runModelCommand('read-model', args, { ...first.deps, cacheRoot: secondCache, artifactDirectory: secondArtifacts, privateRoot: path.join(first.root, 'second-runs') });
  assert.deepEqual(Object.fromEntries(Object.entries(cold.artifacts).map(([name, value]) => [name, value.sha256])), Object.fromEntries(Object.entries(second.artifacts).map(([name, value]) => [name, value.sha256])));
});

test('every production cache lock wait receives the command cancellation signal', async (t) => {
  const state = await setup(t);
  const controller = new AbortController();
  state.deps.signal = controller.signal;
  const preflight = await runModelCommand('preflight', {
    'provider-path': state.executable, 'signing-secret-path': state.secretPath, 'signing-public-path': state.publicPath,
  }, state.deps);
  await runModelCommand('read-model', {
    ...state.args,
    'expected-provider-sha256': preflight.providerFingerprintSha256,
    'expected-signer-sha256': preflight.signerFingerprintSha256,
  }, state.deps);
  assert.equal(state.lockCalls.length > 0, true);
  assert.equal(state.lockCalls.every((call) => call.options?.signal === controller.signal), true);
  assert.equal(state.hostRequests.every((request) => request.signal === controller.signal), true);
});

test('a wrong provider pin refuses before convert and errors never disclose paths', async (t) => {
  const state = await setup(t);
  const preflight = await runModelCommand('preflight', {
    'provider-path': state.executable, 'signing-secret-path': state.secretPath, 'signing-public-path': state.publicPath,
  }, state.deps);
  state.calls.length = 0;
  await assert.rejects(() => runModelCommand('read-model', {
    ...state.args, 'expected-provider-sha256': '0'.repeat(64),
    'expected-signer-sha256': preflight.signerFingerprintSha256,
  }, state.deps), (error) => {
    assert.equal(error.code, 'reference-provider-pin-mismatch');
    assert.equal(error.message.includes(state.sourcePath), false);
    return true;
  });
  assert.deepEqual(state.calls, ['describe']);
});

test('converting commands require the caller-pinned signer and refuse rotation before conversion', async (t) => {
  const state = await setup(t);
  const preflight = await runModelCommand('preflight', {
    'provider-path': state.executable, 'signing-secret-path': state.secretPath, 'signing-public-path': state.publicPath,
  }, state.deps);
  const providerPin = preflight.providerFingerprintSha256;
  await assert.rejects(
    () => runModelCommand('read-model', { ...state.args, 'expected-provider-sha256': providerPin }, state.deps),
    (error) => error.code === 'reference-signer-pin-required',
  );
  await assert.rejects(
    () => runModelCommand('read-model', {
      ...state.args, 'expected-provider-sha256': providerPin, 'expected-signer-sha256': '0'.repeat(64),
    }, state.deps),
    (error) => error.code === 'reference-signer-pin-mismatch',
  );
  assert.deepEqual(state.calls, ['describe']);
});
