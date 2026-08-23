import assert from 'node:assert/strict';
import { generateKeyPairSync } from 'node:crypto';
import { spawn } from 'node:child_process';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';
import { sha256 } from './model-contract.mjs';
import { runModelCommand } from './model-reader.mjs';

const fixture = fileURLToPath(new URL('./test-fixtures/model-provider-fixture.mjs', import.meta.url));

async function setup(t) {
  const root = await fs.mkdtemp(path.join(os.tmpdir(), 'aware-model-reader-'));
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
  let nextLock = 0;
  const hostRun = async (request) => {
    calls.push(request.operation);
    return await new Promise((resolve, reject) => {
      const child = spawn(process.execPath, [fixture, request.operation], { cwd: request.cwd, env: request.environment, windowsHide: true, shell: false, stdio: ['pipe', 'pipe', 'pipe'] });
      const stdout = []; const stderr = [];
      child.stdout.on('data', (chunk) => stdout.push(chunk)); child.stderr.on('data', (chunk) => stderr.push(chunk));
      child.on('error', reject); child.on('close', (exitCode) => resolve({ exitCode, stdout: Buffer.concat(stdout), stderr: Buffer.concat(stderr) }));
      child.stdin.end(request.stdin);
    });
  };
  return {
    root, sourcePath, executable, secretPath, publicPath, calls,
    deps: {
      hostRun,
      hostAcquireLock: async () => Buffer.from(`lock-${nextLock += 1}`),
      hostReleaseLock: async () => {},
      cacheRoot: path.join(root, 'cache'), privateRoot: path.join(root, 'runs'), artifactDirectory: path.join(root, 'artifacts'),
    },
    args: { 'rvt-path': sourcePath, 'source-sha256': sha256(Buffer.from('fixture-rvt')), 'provider-path': executable, 'signing-secret-path': secretPath, 'signing-public-path': publicPath },
  };
}

test('preflight describes provider and key readiness without conversion or source access', async (t) => {
  const state = await setup(t);
  const out = await runModelCommand('preflight', {
    'provider-path': state.executable, 'signing-secret-path': state.secretPath, 'signing-public-path': state.publicPath,
  }, state.deps);
  assert.equal(out.ready, true);
  assert.equal(out.execution, 'local');
  assert.match(out.providerFingerprintSha256, /^[0-9a-f]{64}$/);
  assert.deepEqual(state.calls, ['describe']);
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
    assert.match(descriptor.id, /^[a-z0-9.-]+$/);
    const bytes = await fs.readFile(path.join(state.deps.artifactDirectory, descriptor.id));
    assert.equal(sha256(bytes), descriptor.sha256);
    assert.equal(bytes.length, descriptor.bytes);
  }
  const geometry = await fs.readFile(path.join(state.deps.artifactDirectory, out.artifacts.geometry.id));
  assert.equal(geometry.readUInt32LE(0), 0x46546c67);
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

  const secondCache = path.join(first.root, 'second-cache');
  const secondArtifacts = path.join(first.root, 'second-artifacts');
  const second = await runModelCommand('read-model', args, { ...first.deps, cacheRoot: secondCache, artifactDirectory: secondArtifacts, privateRoot: path.join(first.root, 'second-runs') });
  assert.deepEqual(Object.fromEntries(Object.entries(cold.artifacts).map(([name, value]) => [name, value.sha256])), Object.fromEntries(Object.entries(second.artifacts).map(([name, value]) => [name, value.sha256])));
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
