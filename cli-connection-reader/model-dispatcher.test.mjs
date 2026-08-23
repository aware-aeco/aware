import assert from 'node:assert/strict';
import { spawn } from 'node:child_process';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';
import { serializeModelResult } from './model-dispatcher.mjs';

const here = path.dirname(fileURLToPath(import.meta.url));
const dispatcher = path.join(here, 'model-dispatcher.mjs');
const ifcEntry = path.join(here, 'index.mjs');

function run(entry, command, input, environment = {}) {
  return new Promise((resolve, reject) => {
    const child = spawn(process.execPath, [entry, command, '--json-stdin'], { cwd: here, env: { ...process.env, ...environment }, windowsHide: true, shell: false, stdio: ['pipe', 'pipe', 'pipe'] });
    const stdout = []; const stderr = [];
    child.stdout.on('data', (chunk) => stdout.push(chunk)); child.stderr.on('data', (chunk) => stderr.push(chunk));
    child.on('error', reject); child.on('close', (exitCode) => resolve({ exitCode, stdout: Buffer.concat(stdout), stderr: Buffer.concat(stderr) }));
    child.stdin.end(JSON.stringify(input));
  });
}

test('RVT preflight failures are bounded structured JSON without loading IFC requirements', async () => {
  const result = await run(dispatcher, 'preflight', {}, { AWARE_MODEL_REFERENCE_PROVIDER: '', AWARE_MODEL_REFERENCE_SIGNING_KEY: '' });
  assert.notEqual(result.exitCode, 0);
  assert.equal(result.stdout.length, 0);
  const error = JSON.parse(result.stderr.toString('utf8'));
  assert.equal(error.code, 'reference-provider-unavailable');
  assert.equal(error.phase, 'preflight');
  assert.match(error.diagnosticId, /^[0-9a-f-]{36}$/);
  assert.equal(result.stderr.length < 2048, true);
});

test('mixed IFC and RVT paths are refused instead of guessed', async () => {
  const result = await run(dispatcher, 'probe', { 'ifc-path': 'a.ifc', 'rvt-path': 'b.rvt' });
  assert.notEqual(result.exitCode, 0);
  assert.match(result.stderr.toString('utf8'), /mixed-model-paths/);
});

test('documented IFC probe bytes remain identical through the lazy dispatcher', async () => {
  const input = { 'ifc-path': path.join(here, 'test-fixtures', 'baseplate-bp1.ifc') };
  const before = await run(ifcEntry, 'probe', input);
  const after = await run(dispatcher, 'probe', input);
  assert.equal(before.exitCode, 0); assert.equal(after.exitCode, 0);
  assert.deepEqual(after.stdout, before.stdout);
  assert.deepEqual(after.stderr, before.stderr);
});

test('RVT command responses enforce the declared byte ceiling before stdout', () => {
  assert.throws(
    () => serializeModelResult({ coverage: { unclaimedGeometryNodes: ['x'.repeat(200)] } }, { maxCommandResponseBytes: 128 }),
    (error) => error.code === 'reference-response-too-large',
  );
});
