import assert from 'node:assert/strict';
import { EventEmitter } from 'node:events';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';
import { sha256, READER_SCHEMA_VERSION_V2 } from './model-contract.mjs';
import { describeAndConvert, describeProvider, managedProviderEnvironment, minimalProviderEnvironment } from './model-provider.mjs';
import { HostFrameDecoder, ModelHostClient } from './model-host-client.mjs';

const key = 'AWARE_MODEL_PROVIDER_AUTHORITY_STORE_DIR';
async function fixture(t) {
  const root = await fs.mkdtemp(path.join(await fs.realpath(os.tmpdir()), 'aware-managed-authority-'));
  t.after(() => fs.rm(root, { recursive: true, force: true }));
  const executable = path.join(root, 'provider.exe'), sourcePath = path.join(root, 'source.rvt');
  await fs.writeFile(executable, 'bounded-provider-fixture');
  await fs.writeFile(sourcePath, 'bounded-source-fixture');
  return { root, executable, sourcePath, authorityStorePath: path.join(root, 'authority'),
    expectedSourceSha256: sha256(Buffer.from('bounded-source-fixture')),
    expectedProtocolVersion: '2', expectedDestination: 'https://provider.example.test',
    readerSchemaVersion: READER_SCHEMA_VERSION_V2,
    conversionAttemptId: '123e4567-e89b-42d3-a456-426614174000' };
}

test('managed authority environment is independent, bounded and protocol-2 only', () => {
  const directory = 'C:\\private\\authority';
  const source = { [key]: directory, SystemRoot: 'C:\\Windows', PATH: 'foreign', TOKEN: 'foreign',
    LOCALAPPDATA: 'foreign', FLOLESS_LICENSE_DIR: 'foreign', HTTP_PROXY: 'foreign', AWARE_HOME: 'foreign' };
  assert.deepEqual(managedProviderEnvironment(source, '2', directory, 'win32'), {
    SYSTEMROOT: 'C:\\Windows', LANG: 'C', LC_ALL: 'C', TZ: 'UTC', [key]: directory,
  });
  for (const protocol of ['1', '3']) {
    assert.deepEqual(managedProviderEnvironment({ ...source, [key]: '\0invalid' }, protocol, undefined, 'win32'),
      minimalProviderEnvironment(source, 'win32'));
  }
  assert.deepEqual(managedProviderEnvironment({}, '2', directory, 'win32'), minimalProviderEnvironment({}, 'win32'));
  for (const configured of ['', undefined, 4, 'relative', 'C:\\private\\..\\authority', `${directory}\\`,
    `${directory}\0`, `C:\\${'x'.repeat(4097)}`, 'C:\\foreign\\authority']) {
    assert.throws(() => managedProviderEnvironment({ [key]: configured }, '2', directory, 'win32'),
      (error) => error.code === 'reference-provider-protocol');
  }
  assert.throws(() => managedProviderEnvironment({ [key]: directory }, '2', directory.toLowerCase(), 'win32'),
    (error) => error.code === 'reference-provider-protocol');
});

test('managed Windows aliases collapse only identical bindings; POSIX is case-sensitive', () => {
  const directory = 'C:\\private\\authority', alias = key.toLowerCase();
  assert.equal(managedProviderEnvironment({ [key]: directory, [alias]: directory }, '2', directory, 'win32')[key], directory);
  assert.throws(() => managedProviderEnvironment({ [key]: directory, [alias]: 'C:\\other' }, '2', directory, 'win32'),
    (error) => error.code === 'reference-provider-environment-ambiguous');
  assert.equal(managedProviderEnvironment({ [alias]: '/foreign' }, '2', '/private', 'linux')[key], undefined);
  assert.equal(managedProviderEnvironment({ [key]: '/private' }, '2', '/private', 'linux')[key], '/private');
});

test('actual describe/convert builders carry only the independent binding through host-client frames', async (t) => {
  const f = await fixture(t), calls = [], frames = [], decoder = new HostFrameDecoder();
  const child = new EventEmitter(); child.stdout = new EventEmitter(); child.stderr = new EventEmitter();
  child.stdin = { write: (bytes) => { frames.push(...decoder.push(bytes)); return true; }, once: () => {} };
  const client = new ModelHostClient(child);
  const description = { protocolVersion: '2', provider: 'fixture', engine: 'engine', engineVersion: '1',
    adapterBuildId: 'fixture', formats: ['rvt'], execution: 'managed-cloud', destination: f.expectedDestination,
    readerSchemaVersion: READER_SCHEMA_VERSION_V2 };
  const hostRun = async (request) => {
    calls.push(request);
    let stdout = description;
    if (request.operation === 'convert') {
      const input = JSON.parse(request.stdin);
      assert.equal(input.authorityStorePath, f.authorityStorePath);
      const geometryPath = path.join(request.cwd, 'geometry.glb'), metadataPath = path.join(request.cwd, 'metadata.json');
      await fs.writeFile(geometryPath, 'glb'); await fs.writeFile(metadataPath, '{}');
      stdout = { ...description, documentKind: 'revit-project', sourceSha256: input.sourceSha256,
        conversionAttemptId: input.conversionAttemptId, geometryPath, metadataPath };
    }
    const requestId = client.nextRequestId, handle = Buffer.alloc(32, calls.length);
    const pending = client.run(request);
    const receive = (kind, payload) => client.onFrame({ kind, requestId, runHandle: handle, sequence: 0, final: true, payload });
    receive(1, Buffer.from('{"status":"accepted"}'));
    receive(2, Buffer.from(JSON.stringify(stdout))); receive(3, Buffer.alloc(0));
    receive(1, Buffer.from('{"status":"complete","exitCode":0}'));
    return await pending;
  };
  const environment = { [key]: f.authorityStorePath, TOKEN: 'foreign', PATH: 'foreign', HTTP_PROXY: 'foreign',
    FLOLESS_LICENSE_DIR: 'foreign', LOCALAPPDATA: 'foreign' };
  await describeProvider({ ...f, privateRoot: path.join(f.root, 'preflight'), environment, hostRun });
  await describeAndConvert({ ...f, privateRoot: path.join(f.root, 'conversion'), environment, hostRun });
  assert.deepEqual(calls.map((call) => call.operation), ['describe', 'describe', 'convert']);
  const controls = frames.filter((frame) => frame.kind === 1).map((frame) => JSON.parse(frame.payload));
  assert.equal(controls.length, 3);
  for (const control of controls) {
    assert.equal(control.op, 'provider-run');
    assert.equal(control.environment[key], f.authorityStorePath);
    assert.equal(minimalProviderEnvironment(environment, process.platform)[key], undefined);
    assert.deepEqual(control.environment, { ...minimalProviderEnvironment(environment, process.platform), [key]: f.authorityStorePath });
  }
});

test('invalid configured or request authority rejects before either builder launches a provider', async (t) => {
  const f = await fixture(t); let launches = 0; let attempt = 0;
  const hostRun = async () => { launches++; assert.fail('invalid binding launched provider'); };
  const variants = [
    { environment: { [key]: 'relative' } },
    { environment: { [key]: `${f.authorityStorePath}\0` } },
    { environment: { [key]: path.join(f.root, 'foreign') } },
    { environment: { [key]: path.join(f.root, '..', path.basename(f.root), 'authority') + path.sep } },
    { environment: { [key]: f.authorityStorePath }, authorityStorePath: path.join(f.root, 'foreign') },
    { environment: { [key]: f.authorityStorePath }, authorityStorePath: `${f.authorityStorePath}${path.sep}.` },
  ];
  for (const build of [describeProvider, describeAndConvert]) {
    for (const variant of variants) {
      await assert.rejects(() => build({ ...f, ...variant, hostRun, privateRoot: path.join(f.root, `reject-${attempt++}`) }),
        (error) => error.code === 'reference-provider-protocol');
    }
  }
  assert.equal(launches, 0);
});
