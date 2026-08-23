import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { copyFileSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { build } from 'esbuild';

if (process.platform !== 'win32') {
  process.stdout.write('model Windows harness: skipped (requires Windows process semantics)\n');
  process.exit(0);
}

const here = path.dirname(fileURLToPath(import.meta.url));
const root = path.dirname(here);
const temporary = mkdtempSync(path.join(os.tmpdir(), 'aware-rvt-harness-'));
const FUSE = 'NODE_SEA_FUSE_fce680ab2cc467b6e072b8b5df1996b2';
const aware = process.env.AWARE_SOURCE_BUILT || path.join(root, 'cli', 'target', 'debug', 'aware.exe');

function sha256(bytes) { return createHash('sha256').update(bytes).digest('hex'); }

async function buildSea(entry, output) {
  const buildDirectory = path.join(temporary, `sea-${path.basename(output, '.exe')}`);
  mkdirSync(buildDirectory, { recursive: true });
  const bundle = path.join(buildDirectory, 'bundle.cjs');
  await build({ entryPoints: [entry], bundle: true, platform: 'node', format: 'cjs', target: 'node20', outfile: bundle });
  const config = path.join(buildDirectory, 'sea-config.json');
  const blob = path.join(buildDirectory, 'sea.blob');
  writeFileSync(config, JSON.stringify({ main: bundle, output: blob, disableExperimentalSEAWarning: true }));
  execFileSync(process.execPath, ['--experimental-sea-config', config], { stdio: 'pipe' });
  copyFileSync(process.execPath, output);
  execFileSync(process.execPath, [path.join(here, 'node_modules', 'postject', 'dist', 'cli.js'), output, 'NODE_SEA_BLOB', blob, '--sentinel-fuse', FUSE], { stdio: 'pipe' });
}

function run(entry, command, input, environment, cwd, timeout = 30_000) {
  const stdout = execFileSync(entry, [command, '--json-stdin'], { input: JSON.stringify(input), encoding: 'utf8', env: environment, cwd, windowsHide: true, timeout, maxBuffer: 4 * 1024 * 1024 });
  return JSON.parse(stdout);
}

function findArtifactSet(value) {
  if (!value || typeof value !== 'object') return null;
  if (value.artifacts && typeof value.artifacts === 'object' && Object.keys(value.artifacts).length === 5) {
    return value.artifacts;
  }
  for (const child of Object.values(value)) {
    const found = findArtifactSet(child);
    if (found) return found;
  }
  return null;
}

try {
  assert.equal(path.isAbsolute(aware), true, 'AWARE_SOURCE_BUILT must be absolute');
  const providerDirectory = path.join(temporary, 'authorized-provider'); mkdirSync(providerDirectory);
  const provider = path.join(providerDirectory, 'fixture-model-provider.exe');
  await buildSea(path.join(here, 'test-fixtures', 'model-provider-fixture.mjs'), provider);
  const descendantProvider = path.join(providerDirectory, 'descendant-model-provider.exe');
  await buildSea(path.join(here, 'test-fixtures', 'model-provider-descendant-fixture.mjs'), descendantProvider);

  execFileSync(process.execPath, [path.join(here, 'build.mjs')], { cwd: here, stdio: 'inherit' });
  const stage = path.join(temporary, 'clean-stage'); mkdirSync(stage);
  const packaged = path.join(stage, 'aware-connection-reader.exe');
  copyFileSync(path.join(here, 'dist', 'aware-connection-reader.exe'), packaged);
  copyFileSync(path.join(here, 'dist', 'web-ifc-node.wasm'), path.join(stage, 'web-ifc-node.wasm'));
  assert.deepEqual(readdirSync(stage).sort(), ['aware-connection-reader.exe', 'web-ifc-node.wasm']);

  const home = path.join(temporary, 'aware-home');
  execFileSync(aware, ['key', 'generate', 'model-reference-reader'], { env: { ...process.env, AWARE_HOME: home }, stdio: 'pipe', windowsHide: true });
  const inputDirectory = path.join(temporary, 'authorized-input'); mkdirSync(inputDirectory);
  const source = path.join(inputDirectory, 'fixture.rvt'); writeFileSync(source, 'fixture-rvt');
  const artifacts = path.join(temporary, 'artifacts'); mkdirSync(artifacts);
  const unrelatedCwd = path.join(temporary, 'unrelated-cwd'); mkdirSync(unrelatedCwd);
  const environment = {
    ...process.env, AWARE_HOME: home, AWARE_MODEL_READER_HOST: aware,
    AWARE_MODEL_REFERENCE_PROVIDER: provider,
    AWARE_MODEL_REFERENCE_SIGNING_KEY: path.join(home, 'keys', 'model-reference-reader.sec'),
    AWARE_MODEL_REFERENCE_PUBLIC_KEY: path.join(home, 'keys', 'model-reference-reader.pub'),
    AWARE_ARTIFACT_DIR: artifacts,
  };
  const bridges = path.join(home, 'bridges'); mkdirSync(bridges, { recursive: true });
  copyFileSync(packaged, path.join(bridges, 'aware-connection-reader.exe'));
  copyFileSync(path.join(stage, 'web-ifc-node.wasm'), path.join(bridges, 'web-ifc-node.wasm'));
  const awareVersion = execFileSync(aware, ['--version'], { encoding: 'utf8', windowsHide: true }).trim().split(/\s+/).at(-1);
  writeFileSync(path.join(bridges, 'aware-connection-reader.version'), awareVersion);
  const preflight = run(packaged, 'preflight', {}, environment, unrelatedCwd);
  assert.equal(preflight.ready, true); assert.equal(preflight.execution, 'local');
  const descendantPreflight = run(packaged, 'preflight', {}, {
    ...environment, AWARE_MODEL_REFERENCE_PROVIDER: descendantProvider,
  }, unrelatedCwd, 15_000);
  assert.equal(descendantPreflight.ready, true, 'successful provider descendants must be killed before inherited pipes can hang');
  const request = {
    'rvt-path': source, 'source-sha256': sha256(readFileSync(source)),
    'expected-provider-sha256': preflight.providerFingerprintSha256,
    'expected-signer-sha256': preflight.signerFingerprintSha256,
  };
  const probe = run(packaged, 'probe', request, environment, unrelatedCwd);
  assert.equal(probe.entities, 1); assert.equal(probe.frame.up, 'z');
  const model = run(packaged, 'read-model', request, environment, unrelatedCwd);
  assert.equal(Object.keys(model.artifacts).length, 5);
  const geometry = readFileSync(path.join(artifacts, model.artifacts.geometry.id));
  assert.equal(geometry.readUInt32LE(0), 0x46546c67); assert.equal(sha256(geometry), model.artifacts.geometry.sha256);
  for (const descriptor of Object.values(model.artifacts)) assert.equal(sha256(readFileSync(path.join(artifacts, descriptor.id))), descriptor.sha256);

  const ifc = run(packaged, 'probe', { 'ifc-path': path.join(here, 'test-fixtures', 'baseplate-bp1.ifc') }, environment, unrelatedCwd);
  assert.equal(ifc.schema, 'IFC4'); assert.equal(ifc.frame, 'z-up');

  execFileSync(aware, ['agent', 'install', path.join(root, '20-agents', 'aeco', 'engineering', 'model-reference-reader')], {
    env: environment, stdio: 'pipe', windowsHide: true,
  });
  const appDirectory = path.join(temporary, 'rvt-reader-e2e'); mkdirSync(appDirectory);
  const appSource = path.join(appDirectory, 'rvt-reader-e2e.flo');
  writeFileSync(appSource, `app: rvt-reader-e2e
version: 0.1.0
display-name: RVT Reader E2E
description: Exercise the authenticated local RVT reader through a real one-shot AWARE app.
exposes-as-agent: false
requires:
  - model-reference-reader@0.1.x
requires-permissions:
  filesystem:
    - read: '*.rvt'
layout: linear
nodes:
  - id: read-reference
    agent: model-reference-reader
    command: read-model
    inputs:
      rvt-path: '{{ inputs.rvt-path }}'
      source-sha256: '{{ inputs.source-sha256 }}'
      expected-provider-sha256: '{{ inputs.expected-provider-sha256 }}'
      expected-signer-sha256: '{{ inputs.expected-signer-sha256 }}'
`);
  execFileSync(aware, ['app', 'install', appDirectory], { env: environment, stdio: 'pipe', windowsHide: true });
  const appStdout = execFileSync(aware, [
    'app', 'run', 'rvt-reader-e2e',
    '--input', `rvt-path=${source}`,
    '--input', `source-sha256=${request['source-sha256']}`,
    '--input', `expected-provider-sha256=${preflight.providerFingerprintSha256}`,
    '--input', `expected-signer-sha256=${preflight.signerFingerprintSha256}`,
  ], { env: environment, encoding: 'utf8', windowsHide: true, maxBuffer: 4 * 1024 * 1024 });
  const runId = appStdout.match(/run-id ([0-9a-f-]{36})/)?.[1];
  assert.ok(runId, 'real aware app run must report its run id');
  const trace = execFileSync(aware, ['app', 'logs', 'rvt-reader-e2e', '--run-id', runId], {
    env: environment, encoding: 'utf8', windowsHide: true, maxBuffer: 4 * 1024 * 1024,
  }).trim().split(/\r?\n/).filter(Boolean).map((line) => JSON.parse(line));
  const appArtifacts = findArtifactSet(trace);
  assert.ok(appArtifacts, 'real aware app run must return all five artifact descriptors');
  for (const [kind, descriptor] of Object.entries(appArtifacts)) {
    const retrieved = path.join(temporary, `retrieved-${kind}${kind === 'geometry' ? '.glb' : '.json'}`);
    execFileSync(aware, ['app', 'artifact', 'rvt-reader-e2e', descriptor.id, '--run-id', runId, '--output', retrieved], {
      env: environment, stdio: 'pipe', windowsHide: true,
    });
    assert.equal(sha256(readFileSync(retrieved)), descriptor.sha256);
  }
  assert.equal(readdirSync(stage).some((name) => /\.(?:mjs|json)$/i.test(name)), false);
  process.stdout.write(`model Windows harness: PASS (${model.sourceSha256}, ${model.providerFingerprintSha256})\n`);
} finally {
  rmSync(temporary, { recursive: true, force: true });
}
