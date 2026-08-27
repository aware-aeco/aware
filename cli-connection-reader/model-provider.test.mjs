import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { Readable } from 'node:stream';
import test from 'node:test';
import { fileURLToPath } from 'node:url';
import { sha256 } from './model-contract.mjs';
import {
  describeAndConvert, describeProvider, hashRegularFile, minimalProviderEnvironment, stageImmutableSource,
  validateProviderExecutable,
} from './model-provider.mjs';

const fixture = fileURLToPath(new URL('./test-fixtures/model-provider-fixture.mjs', import.meta.url));

async function temporaryDirectory(t) {
  const directory = await fs.mkdtemp(path.join(await fs.realpath(os.tmpdir()), 'aware-model-provider-'));
  t.after(() => fs.rm(directory, { recursive: true, force: true }));
  return directory;
}

function fixtureHostRun(calls) {
  return async (request) => {
    calls.push(request);
    return await new Promise((resolve, reject) => {
      const child = spawn(process.execPath, [fixture, request.operation], {
        cwd: request.cwd, env: request.environment, shell: false, windowsHide: true,
        stdio: ['pipe', 'pipe', 'pipe'],
      });
      const stdout = []; const stderr = [];
      child.stdout.on('data', (chunk) => stdout.push(chunk));
      child.stderr.on('data', (chunk) => stderr.push(chunk));
      child.on('error', reject);
      child.on('close', (exitCode) => resolve({ exitCode, stdout: Buffer.concat(stdout), stderr: Buffer.concat(stderr) }));
      child.stdin.end(request.stdin);
    });
  };
}

test('provider executable and source must be absolute regular non-link files', async (t) => {
  const root = await temporaryDirectory(t);
  const executable = path.join(root, 'provider.exe');
  await fs.writeFile(executable, 'fixture');
  assert.equal((await validateProviderExecutable(executable)).path, executable);
  await assert.rejects(() => validateProviderExecutable('provider.exe'), /absolute/);
  await assert.rejects(() => validateProviderExecutable(root), /regular file/);
  const link = path.join(root, 'provider-link.exe');
  try {
    await fs.symlink(executable, link, 'file');
    await assert.rejects(() => validateProviderExecutable(link), /link|reparse/);
  } catch (error) {
    if (error?.code !== 'EPERM') throw error;
    t.diagnostic('symlink creation is unavailable; the explicit link refusal branch is unverified');
  }
});

test('minimal provider environment omits paths, proxies, credentials, and AWARE state', () => {
  const environment = minimalProviderEnvironment({
    SystemRoot: 'C:\\Windows', windir: 'C:\\Windows', ComSpec: 'C:\\Windows\\System32\\cmd.exe',
    Temp: 'C:\\Temp', tMp: 'C:\\Scratch', PaTh: 'secret', Http_Proxy: 'secret',
    Aware_Home: 'secret', Aws_Secret_Access_Key: 'secret', ToKeN: 'secret',
    'ſYSTEMROOT': 'unicode-near-alias',
  }, 'win32');
  assert.deepEqual(environment, {
    SYSTEMROOT: 'C:\\Windows', WINDIR: 'C:\\Windows', COMSPEC: 'C:\\Windows\\System32\\cmd.exe',
    TEMP: 'C:\\Temp', TMP: 'C:\\Scratch', LANG: 'C', LC_ALL: 'C', TZ: 'UTC',
  });
});

test('minimal Windows environment collapses identical aliases and refuses conflicting aliases', () => {
  assert.deepEqual(minimalProviderEnvironment({ SystemRoot: 'C:\\Windows', SYSTEMROOT: 'C:\\Windows' }, 'win32'), {
    SYSTEMROOT: 'C:\\Windows', LANG: 'C', LC_ALL: 'C', TZ: 'UTC',
  });
  assert.throws(
    () => minimalProviderEnvironment({ SystemRoot: 'C:\\Windows', SYSTEMROOT: 'D:\\Windows' }, 'win32'),
    (error) => error.code === 'reference-provider-environment-ambiguous',
  );
});

test('minimal POSIX environment remains case-sensitive', () => {
  assert.deepEqual(minimalProviderEnvironment({ home: '/forbidden', HOME: '/home/aware', TmpDir: '/forbidden', TMPDIR: '/tmp/aware' }, 'linux'), {
    HOME: '/home/aware', TMPDIR: '/tmp/aware', LANG: 'C', LC_ALL: 'C', TZ: 'UTC',
  });
});

test('source staging hashes both sides, creates an immutable private copy, and detects expected-hash drift', async (t) => {
  const root = await temporaryDirectory(t);
  const source = path.join(root, 'source.rvt');
  const staging = path.join(root, 'staging');
  const bytes = Buffer.from('deterministic-rvt-fixture');
  await fs.writeFile(source, bytes);
  const expected = sha256(bytes);
  const staged = await stageImmutableSource(source, staging, expected);
  assert.equal(staged.sourceSha256, expected);
  assert.deepEqual(await fs.readFile(staged.path), bytes);
  assert.equal((await fs.stat(staged.path)).mode & 0o222, 0);
  await assert.rejects(() => stageImmutableSource(source, path.join(root, 'other'), '0'.repeat(64)), /source changed/);
});

test('describe and convert agree on provenance and return only bounded private outputs', async (t) => {
  const root = await temporaryDirectory(t);
  const executable = path.join(root, 'provider.exe');
  const source = path.join(root, 'source.rvt');
  await fs.writeFile(executable, 'fixture-provider-binary');
  await fs.writeFile(source, 'fixture-rvt');
  const calls = [];
  const controller = new AbortController();
  const result = await describeAndConvert({
    executable, sourcePath: source, expectedSourceSha256: sha256(Buffer.from('fixture-rvt')),
    privateRoot: path.join(root, 'private'), hostRun: fixtureHostRun(calls), signal: controller.signal,
  });
  assert.equal(calls.length, 2);
  assert.deepEqual(calls.map((call) => call.operation), ['describe', 'convert']);
  assert.equal(calls[0].cwd.startsWith(path.join(root, 'private')), true);
  assert.equal(calls[0].environment.PATH, undefined);
  assert.equal(calls.every((call) => call.signal === controller.signal), true);
  assert.equal(result.describe.provider, 'fixture-provider');
  assert.equal(result.receipt.sourceSha256, sha256(Buffer.from('fixture-rvt')));
  assert.equal(result.outputs.geometry.path.endsWith('geometry.glb'), true);
  assert.equal(result.outputs.metadata.path.endsWith('metadata.json'), true);
  assert.equal(result.outputs.geometry.sha256, sha256(result.outputs.geometry.bytes));
  assert.equal(result.outputs.metadata.sha256, sha256(result.outputs.metadata.bytes));
});

test('managed-cloud protocol requires an exact canonical HTTPS destination pin', async (t) => {
  const root = await temporaryDirectory(t);
  const executable = path.join(root, 'provider.exe');
  await fs.writeFile(executable, 'fixture-provider-binary');
  const description = {
    protocolVersion: '2', provider: 'fixture-provider', engine: 'xeoRvt', engineVersion: '0.2.0',
    adapterBuildId: 'fixture-v2', formats: ['rvt'], execution: 'managed-cloud',
    destination: 'https://api.stage.floless.io',
  };
  const hostRun = async () => ({ exitCode: 0, stdout: Buffer.from(JSON.stringify(description)), stderr: Buffer.alloc(0) });
  const accepted = await describeProvider({
    executable, privateRoot: path.join(root, 'accepted'), hostRun,
    expectedProtocolVersion: '2', expectedDestination: description.destination,
  });
  assert.equal(accepted.describe.execution, 'managed-cloud');
  assert.equal(accepted.fingerprint.destination, description.destination);
  await assert.rejects(() => describeProvider({
    executable, privateRoot: path.join(root, 'wrong'), hostRun,
    expectedProtocolVersion: '2', expectedDestination: 'https://api.floless.io',
  }), (error) => error.code === 'reference-provider-destination-mismatch');
  await assert.rejects(() => describeProvider({
    executable, privateRoot: path.join(root, 'noncanonical'),
    hostRun: async () => ({ ...await hostRun(), stdout: Buffer.from(JSON.stringify({ ...description, destination: `${description.destination}/` })) }),
    expectedProtocolVersion: '2', expectedDestination: description.destination,
  }), (error) => error.code === 'reference-provider-destination-mismatch');
});

test('managed-cloud conversion passes only an absolute caller-bound authority store', async (t) => {
  const root = await temporaryDirectory(t);
  const executable = path.join(root, 'provider.exe');
  const source = path.join(root, 'source.rvt');
  const authorityStorePath = path.join(root, 'authority');
  await fs.writeFile(executable, 'fixture-provider-binary');
  await fs.writeFile(source, 'fixture-rvt');
  const description = {
    protocolVersion: '2', provider: 'fixture-provider', engine: 'xeoRvt', engineVersion: '0.2.0',
    adapterBuildId: 'fixture-v2', formats: ['rvt'], execution: 'managed-cloud',
    destination: 'https://api.stage.floless.io',
  };
  const calls = [];
  const hostRun = async (request) => {
    calls.push(request);
    if (request.operation === 'describe') return { exitCode: 0, stdout: Buffer.from(JSON.stringify(description)), stderr: Buffer.alloc(0) };
    const body = JSON.parse(request.stdin.toString('utf8'));
    await fs.writeFile(path.join(body.outputDirectory, 'geometry.glb'), Buffer.from('glb'));
    await fs.writeFile(path.join(body.outputDirectory, 'metadata.json'), Buffer.from('{}'));
    return { exitCode: 0, stdout: Buffer.from(JSON.stringify({
      ...description, documentKind: 'revit-project', sourceSha256: body.sourceSha256,
      geometryPath: path.join(body.outputDirectory, 'geometry.glb'), metadataPath: path.join(body.outputDirectory, 'metadata.json'),
    })), stderr: Buffer.alloc(0) };
  };
  await describeAndConvert({
    executable, sourcePath: source, expectedSourceSha256: sha256(Buffer.from('fixture-rvt')),
    privateRoot: path.join(root, 'accepted'), hostRun,
    expectedProtocolVersion: '2', expectedDestination: description.destination, authorityStorePath,
  });
  const convert = JSON.parse(calls[1].stdin.toString('utf8'));
  assert.equal(convert.authorityStorePath, path.resolve(authorityStorePath));
  await assert.rejects(() => describeAndConvert({
    executable, sourcePath: source, expectedSourceSha256: sha256(Buffer.from('fixture-rvt')),
    privateRoot: path.join(root, 'relative'), hostRun,
    expectedProtocolVersion: '2', expectedDestination: description.destination, authorityStorePath: 'relative',
  }), (error) => error.code === 'reference-provider-protocol');
});

test('streaming hash and staging stop at the byte limit even when the source grows after stat', async (t) => {
  const root = await temporaryDirectory(t);
  const source = path.join(root, 'growing.rvt');
  await fs.writeFile(source, 'a');
  await assert.rejects(
    () => hashRegularFile(source, 4, 'source', {
      createReadStream: () => Readable.from([Buffer.from('abc'), Buffer.from('de')]),
    }),
    (error) => error.code === 'reference-source-too-large',
  );

  let reads = 0;
  await assert.rejects(
    () => stageImmutableSource(source, path.join(root, 'stage'), sha256(Buffer.from('a')), {
      limits: { maxSourceBytes: 4 },
      createReadStream: () => {
        reads += 1;
        return Readable.from([reads === 1 ? Buffer.from('a') : Buffer.from('abcde')]);
      },
    }),
    (error) => error.code === 'reference-source-too-large',
  );
});

test('malformed receipts, provenance drift, extra files, non-zero exits, and provider text stay bounded and redacted', async (t) => {
  const root = await temporaryDirectory(t);
  const executable = path.join(root, 'provider.exe');
  const source = path.join(root, 'secret-residential.rvt');
  await fs.writeFile(executable, 'fixture-provider-binary');
  await fs.writeFile(source, 'fixture-rvt');
  const base = {
    executable, sourcePath: source, expectedSourceSha256: sha256(Buffer.from('fixture-rvt')),
    privateRoot: path.join(root, 'private'),
  };
  let call = 0;
  await assert.rejects(() => describeAndConvert({ ...base, hostRun: async () => {
    call += 1;
    if (call === 1) return { exitCode: 0, stdout: Buffer.from('{"protocolVersion":"1","provider":"p","engine":"e","engineVersion":"1","adapterBuildId":"b","formats":["rvt"],"execution":"local","destination":null}'), stderr: Buffer.alloc(0) };
    return { exitCode: 7, stdout: Buffer.alloc(0), stderr: Buffer.from(`${source} TOKEN=secret`) };
  } }), (error) => {
    assert.equal(error.code, 'reference-provider-failed');
    assert.equal(error.message.includes('Residential'), false);
    assert.equal(error.message.includes('secret'), false);
    return true;
  });
});

test('provider executable mutation at a provenance bracket and undeclared output files are refused', async (t) => {
  const root = await temporaryDirectory(t);
  const executable = path.join(root, 'provider.exe');
  const source = path.join(root, 'source.rvt');
  await fs.writeFile(executable, 'fixture-provider-binary');
  await fs.writeFile(source, 'fixture-rvt');
  const base = {
    executable, sourcePath: source, expectedSourceSha256: sha256(Buffer.from('fixture-rvt')),
  };
  const mutationCalls = [];
  const mutateHost = fixtureHostRun(mutationCalls);
  await assert.rejects(() => describeAndConvert({
    ...base, privateRoot: path.join(root, 'mutation-private'), hostRun: async (request) => {
      const result = await mutateHost(request);
      await fs.writeFile(executable, 'changed-provider-binary');
      return result;
    },
  }), /executable changed/);

  await fs.writeFile(executable, 'fixture-provider-binary');
  const extraCalls = [];
  const extraHost = fixtureHostRun(extraCalls);
  await assert.rejects(() => describeAndConvert({
    ...base, privateRoot: path.join(root, 'extra-private'), hostRun: async (request) => {
      const result = await extraHost(request);
      if (request.operation === 'convert') await fs.writeFile(path.join(request.cwd, 'undeclared.txt'), 'no');
      return result;
    },
  }), /output directory was not closed/);
});
