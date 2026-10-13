import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';
import { fileURLToPath } from 'node:url';
import { DIAGNOSTIC_LIMITS, diagnosticRecord, persistModelDiagnostic } from './model-diagnostics.mjs';
import { ModelReaderError, safeErrorEnvelope } from './model-contract.mjs';

async function fixture(t) {
  // Windows %TEMP% may deliberately grant sandbox principals DeleteChild. Use the checked-out
  // tree's safe operator-owned parent without changing shared ancestor permissions.
  const parent = process.platform === 'win32' ? fileURLToPath(new URL('../.tmp/', import.meta.url)) : await fs.realpath(os.tmpdir());
  await fs.mkdir(parent, { recursive: true, mode: 0o700 });
  const home = await fs.mkdtemp(path.join(parent, 'aware-private-diagnostics-'));
  t.after(() => fs.rm(home, { recursive: true, force: true }));
  const error = new ModelReaderError('reference-provider-failed', 'conversion', true,
    'The enrolled provider failed during conversion.', {
      exitCode: 1, stderr: Buffer.from("ValueError: could not convert string to float: ''\nPRIVATE-608"),
      stdout: Buffer.from([0xff, 0x00]), authorization: 'DO-NOT-RETAIN-AUTHORIZATION',
    });
  return { home, error, directory: path.join(home, 'model-reader-diagnostics') };
}

test('retains exact binary provider diagnostics privately under the public diagnostic ID', async (t) => {
  const { home, error, directory } = await fixture(t);
  assert.equal(await persistModelDiagnostic(error, 'read-snapshot', { AWARE_HOME: home }), true);
  const bytes = await fs.readFile(path.join(directory, `${error.diagnosticId}.json`));
  const record = JSON.parse(bytes);
  assert.equal(record.error.diagnosticId, safeErrorEnvelope(error).diagnosticId);
  assert.equal(record.command, 'read-snapshot');
  assert.equal(record.evidence.exitCode, 1);
  assert.deepEqual(Buffer.from(record.evidence.stderr.data, 'base64'), error.unsafeDetails.stderr);
  assert.deepEqual(Buffer.from(record.evidence.stdout.data, 'base64'), error.unsafeDetails.stdout);
  assert.equal(record.evidence.stderr.truncated, false);
  assert.equal(bytes.includes(Buffer.from('DO-NOT-RETAIN')), false);
  assert.doesNotMatch(JSON.stringify(safeErrorEnvelope(error)), /PRIVATE-608|ValueError/);
  if (process.platform === 'win32') {
    const script = `$sid = [System.Security.Principal.WindowsIdentity]::GetCurrent().User.Value
$acl = [System.IO.File]::GetAccessControl($env:AWARE_DIAGNOSTIC_TEST_FILE)
if ($acl.Access.Count -ne 1 -or $acl.Access[0].IdentityReference.Translate([System.Security.Principal.SecurityIdentifier]).Value -ne $sid) { exit 1 }`;
    execFileSync(path.join(process.env.SystemRoot, 'System32', 'WindowsPowerShell', 'v1.0', 'powershell.exe'),
      ['-NoProfile', '-NonInteractive', '-EncodedCommand', Buffer.from(script, 'utf16le').toString('base64')],
      { env: { ...process.env, AWARE_DIAGNOSTIC_TEST_FILE: path.join(directory, `${error.diagnosticId}.json`) }, windowsHide: true });
  } else {
    assert.equal((await fs.stat(directory)).mode & 0o777, 0o700);
    assert.equal((await fs.stat(path.join(directory, `${error.diagnosticId}.json`))).mode & 0o777, 0o600);
  }
});

test('bounds streams and documents and survives cyclic nested failure evidence', async (t) => {
  const { error } = await fixture(t);
  error.unsafeDetails.stderr = Buffer.alloc(DIAGNOSTIC_LIMITS.streamBytes + 99, 0xff);
  error.unsafeDetails.cause = error.unsafeDetails;
  const bytes = diagnosticRecord(error, 'read-snapshot');
  assert.ok(bytes.length <= DIAGNOSTIC_LIMITS.recordBytes);
  const record = JSON.parse(bytes);
  assert.equal(record.evidence.stderr.bytes, DIAGNOSTIC_LIMITS.streamBytes + 99);
  assert.equal(record.evidence.stderr.retainedBytes, DIAGNOSTIC_LIMITS.streamBytes);
  assert.equal(record.evidence.stderr.truncated, true);
});

test('retention evicts the oldest record before publication and preserves unrelated files', async (t) => {
  const { home, error, directory } = await fixture(t);
  assert.equal(await persistModelDiagnostic(error, 'read-snapshot', { AWARE_HOME: home }), true);
  await fs.unlink(path.join(directory, `${error.diagnosticId}.json`));
  let oldest;
  for (let index = 0; index < DIAGNOSTIC_LIMITS.records; index += 1) {
    const prior = new ModelReaderError('prior', 'conversion', true, 'Prior failure');
    const pathname = path.join(directory, `${prior.diagnosticId}.json`);
    await fs.writeFile(pathname, '{}', { mode: 0o600 });
    await fs.utimes(pathname, new Date(index * 1000), new Date(index * 1000));
    oldest ??= pathname;
  }
  await fs.writeFile(path.join(directory, 'operator-note.txt'), 'keep');
  assert.equal(await persistModelDiagnostic(error, 'read-snapshot', { AWARE_HOME: home }), true);
  assert.equal((await fs.readdir(directory)).filter((name) => name.endsWith('.json')).length, DIAGNOSTIC_LIMITS.records);
  await assert.rejects(fs.stat(oldest), { code: 'ENOENT' });
  assert.equal(await fs.readFile(path.join(directory, 'operator-note.txt'), 'utf8'), 'keep');
});

test('private storage failure and an occupied fence preserve the original error without overwrite', async (t) => {
  const { home, error, directory } = await fixture(t);
  await fs.writeFile(directory, 'not a directory');
  assert.equal(await persistModelDiagnostic(error, 'read-snapshot', { AWARE_HOME: home }), false);
  assert.equal(await fs.readFile(directory, 'utf8'), 'not a directory');
  await fs.unlink(directory);
  await fs.mkdir(directory, { mode: 0o700 });
  const lock = path.join(directory, '.lock');
  await fs.writeFile(lock, '999999999');
  const before = safeErrorEnvelope(error);
  assert.equal(await persistModelDiagnostic(error, 'read-snapshot', { AWARE_HOME: home }), false);
  assert.equal(await fs.readFile(lock, 'utf8'), '999999999');
  assert.deepEqual(safeErrorEnvelope(error), before);
  await fs.unlink(lock);
  const target = path.join(directory, `${error.diagnosticId}.json`);
  await fs.writeFile(target, 'existing');
  assert.equal(await persistModelDiagnostic(error, 'read-snapshot', { AWARE_HOME: home }), false);
  assert.equal(await fs.readFile(target, 'utf8'), 'existing');
});

test('refuses linked diagnostic stores without writing or pruning their targets', async (t) => {
  const { home, error, directory } = await fixture(t);
  const target = path.join(home, 'elsewhere');
  await fs.mkdir(target);
  await fs.symlink(target, directory, process.platform === 'win32' ? 'junction' : 'dir');
  assert.equal(await persistModelDiagnostic(error, 'read-snapshot', { AWARE_HOME: home }), false);
  assert.deepEqual(await fs.readdir(target), []);
});

test('never invokes evidence getters and bounds allocation before encoding nested evidence', async (t) => {
  const { error } = await fixture(t);
  Object.defineProperty(error.unsafeDetails, 'message', { get() { assert.fail('private evidence invoked a getter'); } });
  error.unsafeDetails.cause = new Error('\u0001'.repeat(1024 * 1024));
  const bytes = diagnosticRecord(error, 'read-snapshot');
  assert.ok(bytes.length <= DIAGNOSTIC_LIMITS.recordBytes);
  const record = JSON.parse(bytes);
  assert.equal(record.evidence.message, undefined);
  assert.match(Buffer.from(record.evidence.stderr.data, 'base64').toString('utf8'), /PRIVATE-608/);
});

test('retains bounded cleanup evidence without replacing the original diagnostic identity', async (t) => {
  const { error } = await fixture(t);
  Object.defineProperty(error, 'cleanupError', { value: new Error('PRIVATE-608 cleanup failure') });
  const record = JSON.parse(diagnosticRecord(error, 'read-snapshot'));
  assert.equal(record.error.diagnosticId, error.diagnosticId);
  assert.equal(record.cleanupFailed, true);
  assert.equal(record.cleanupError.message, 'PRIVATE-608 cleanup failure');
});

test('refuses hard-linked records without changing or deleting the unrelated target', async (t) => {
  const { home, error, directory } = await fixture(t);
  await fs.mkdir(directory, { mode: 0o700 });
  const target = path.join(home, 'unrelated.txt');
  await fs.writeFile(target, 'keep', { mode: 0o644 });
  const mode = (await fs.stat(target)).mode;
  await fs.link(target, path.join(directory, `${error.diagnosticId}.json`));
  assert.equal(await persistModelDiagnostic(error, 'read-snapshot', { AWARE_HOME: home }), false);
  assert.equal(await fs.readFile(target, 'utf8'), 'keep');
  assert.equal((await fs.stat(target)).mode, mode);
});
