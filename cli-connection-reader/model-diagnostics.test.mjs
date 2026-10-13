import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';
import { DIAGNOSTIC_LIMITS, diagnosticRecord, ensureDiagnosticHomeAncestors, persistModelDiagnostic } from './model-diagnostics.mjs';
import { ModelReaderError, safeErrorEnvelope } from './model-contract.mjs';

async function fixture(t) {
  // Hosted scratch volumes can have an untrusted owner. The fixture home is private; the
  // production writer itself must create and secure its diagnostic directory and records.
  const parent = await fs.realpath(process.platform === 'win32' ? os.homedir() : os.tmpdir());
  await fs.mkdir(parent, { recursive: true, mode: 0o700 });
  const home = await fs.mkdtemp(path.join(parent, 'aware-private-diagnostics-'));
  t.after(() => fs.rm(home, { recursive: true, force: true }));
  if (process.platform === 'win32') windowsScript(`$acl = [System.Security.AccessControl.DirectorySecurity]::new()
$acl.SetOwner($sid)
$acl.SetAccessRuleProtection($true, $false)
$acl.AddAccessRule([System.Security.AccessControl.FileSystemAccessRule]::new($sid, 'FullControl', 'ContainerInherit,ObjectInherit', 'None', 'Allow'))
[System.IO.Directory]::SetAccessControl($env:AWARE_DIAGNOSTIC_TEST_FILE, $acl)`, home);
  const error = new ModelReaderError('reference-provider-failed', 'conversion', true,
    'The enrolled provider failed during conversion.', {
      exitCode: 1, stderr: Buffer.from("ValueError: could not convert string to float: ''\nPRIVATE-608"),
      stdout: Buffer.from([0xff, 0x00]), authorization: 'DO-NOT-RETAIN-AUTHORIZATION',
    });
  return { home, error, directory: path.join(home, 'model-reader-diagnostics') };
}

function windowsScript(script, pathname) {
  return execFileSync(path.join(process.env.SystemRoot, 'System32', 'WindowsPowerShell', 'v1.0', 'powershell.exe'),
    ['-NoProfile', '-NonInteractive', '-EncodedCommand', Buffer.from(`$ErrorActionPreference = 'Stop'\n$sid = [System.Security.Principal.WindowsIdentity]::GetCurrent().User\n${script}`, 'utf16le').toString('base64')],
    { env: { ...process.env, AWARE_DIAGNOSTIC_TEST_FILE: pathname }, windowsHide: true, encoding: 'utf8' });
}

function assertWindowsPrivate(pathname, directory = false) {
  windowsScript(`$acl = [System.IO.${directory ? 'Directory' : 'File'}]::GetAccessControl($env:AWARE_DIAGNOSTIC_TEST_FILE)
if ($acl.GetOwner([System.Security.Principal.SecurityIdentifier]) -ne $sid -or -not $acl.AreAccessRulesProtected -or $acl.Access.Count -ne 1 -or $acl.Access[0].IdentityReference.Translate([System.Security.Principal.SecurityIdentifier]) -ne $sid -or $acl.Access[0].AccessControlType -ne 'Allow') { throw 'Private operator ownership or DACL mismatch' }`, pathname);
}

test('fresh Windows directory and two successive records have explicit operator ownership', { skip: process.platform !== 'win32' }, async (t) => {
  const { home, error, directory } = await fixture(t);
  await assert.rejects(fs.lstat(directory), { code: 'ENOENT' });
  assert.equal(await persistModelDiagnostic(error, 'read-snapshot', { AWARE_HOME: home }), true);
  assertWindowsPrivate(directory, true);
  assertWindowsPrivate(path.join(directory, `${error.diagnosticId}.json`));
  const second = new ModelReaderError('reference-provider-protocol', 'conversion', true, 'Invalid provider response', error.unsafeDetails);
  assert.equal(await persistModelDiagnostic(second, 'read-snapshot', { AWARE_HOME: home }), true);
  assertWindowsPrivate(path.join(directory, `${second.diagnosticId}.json`));
  assert.equal((await fs.readdir(directory)).filter((name) => name.endsWith('.json')).length, 2);
});

test('existing permissive Windows diagnostic directories are refused without ACL takeover', { skip: process.platform !== 'win32' }, async (t) => {
  const { home, error, directory } = await fixture(t);
  await fs.mkdir(directory);
  const script = `$acl = [System.IO.Directory]::GetAccessControl($env:AWARE_DIAGNOSTIC_TEST_FILE)
$acl.SetOwner($sid)
$acl.AddAccessRule([System.Security.AccessControl.FileSystemAccessRule]::new([System.Security.Principal.SecurityIdentifier]::new('S-1-1-0'), 'Read', 'Allow'))
[System.IO.Directory]::SetAccessControl($env:AWARE_DIAGNOSTIC_TEST_FILE, $acl)
[Console]::Write($acl.GetSecurityDescriptorSddlForm('All'))`;
  const before = windowsScript(script, directory);
  assert.equal(await persistModelDiagnostic(error, 'read-snapshot', { AWARE_HOME: home }), false);
  assert.equal(windowsScript(`[Console]::Write([System.IO.Directory]::GetAccessControl($env:AWARE_DIAGNOSTIC_TEST_FILE).GetSecurityDescriptorSddlForm('All'))`, directory), before);
  assert.deepEqual(await fs.readdir(directory), []);
});

test('Windows replaceable ancestors are refused before creating the diagnostic directory', { skip: process.platform !== 'win32' }, async (t) => {
  const { home, error } = await fixture(t);
  windowsScript(`$acl = [System.IO.Directory]::GetAccessControl($env:AWARE_DIAGNOSTIC_TEST_FILE)
$acl.AddAccessRule([System.Security.AccessControl.FileSystemAccessRule]::new([System.Security.Principal.SecurityIdentifier]::new('S-1-1-0'), 'DeleteSubdirectoriesAndFiles', 'Allow'))
[System.IO.Directory]::SetAccessControl($env:AWARE_DIAGNOSTIC_TEST_FILE, $acl)`, home);
  const nested = path.join(home, 'new-home');
  assert.equal(await persistModelDiagnostic(error, 'read-snapshot', { AWARE_HOME: nested }), false);
  await assert.rejects(fs.lstat(nested), { code: 'ENOENT' });
});

test('existing Windows records with inherited permissions are refused without changing them', { skip: process.platform !== 'win32' }, async (t) => {
  const { home, error, directory } = await fixture(t);
  assert.equal(await persistModelDiagnostic(error, 'read-snapshot', { AWARE_HOME: home }), true);
  const unsafeError = new ModelReaderError('prior', 'conversion', true, 'Prior failure');
  const unsafe = path.join(directory, `${unsafeError.diagnosticId}.json`);
  await fs.writeFile(unsafe, '{}');
  const before = windowsScript(`[Console]::Write([System.IO.File]::GetAccessControl($env:AWARE_DIAGNOSTIC_TEST_FILE).GetSecurityDescriptorSddlForm('All'))`, unsafe);
  const next = new ModelReaderError('next', 'conversion', true, 'Next failure');
  assert.equal(await persistModelDiagnostic(next, 'read-snapshot', { AWARE_HOME: home }), false);
  assert.equal(windowsScript(`[Console]::Write([System.IO.File]::GetAccessControl($env:AWARE_DIAGNOSTIC_TEST_FILE).GetSecurityDescriptorSddlForm('All'))`, unsafe), before);
  assert.equal(await fs.readFile(unsafe, 'utf8'), '{}');
  assert.equal((await fs.readdir(directory)).filter((name) => name.endsWith('.json')).length, 2);
});

test('elevated Windows existing Administrators-owned records remain untouched and are refused', { skip: process.platform !== 'win32' }, async (t) => {
  const { home, error, directory } = await fixture(t);
  if (windowsScript(`[Console]::Write([System.Security.Principal.WindowsIdentity]::GetCurrent().Owner.Value -eq $sid.Value)`, home) === 'True') {
    t.skip('requires an elevated token with a different default owner'); return;
  }
  assert.equal(await persistModelDiagnostic(error, 'read-snapshot', { AWARE_HOME: home }), true);
  const first = path.join(directory, `${error.diagnosticId}.json`);
  const second = new ModelReaderError('prior', 'conversion', true, 'Prior failure');
  const unsafe = path.join(directory, `${second.diagnosticId}.json`);
  windowsScript(`$acl = [System.Security.AccessControl.FileSecurity]::new()
$acl.SetOwner([System.Security.Principal.WindowsIdentity]::GetCurrent().Owner)
$acl.SetAccessRuleProtection($true, $false)
$acl.AddAccessRule([System.Security.AccessControl.FileSystemAccessRule]::new($sid, 'FullControl', 'Allow'))
$stream = [System.IO.FileStream]::new($env:AWARE_DIAGNOSTIC_TEST_FILE, [System.IO.FileMode]::CreateNew, [System.Security.AccessControl.FileSystemRights]::FullControl, [System.IO.FileShare]::None, 4096, [System.IO.FileOptions]::None, $acl)
try { $stream.WriteByte(123); $stream.WriteByte(125) } finally { $stream.Dispose() }`, unsafe);
  const before = windowsScript(`[Console]::Write([System.IO.File]::GetAccessControl($env:AWARE_DIAGNOSTIC_TEST_FILE).GetSecurityDescriptorSddlForm('All'))`, unsafe);
  const third = new ModelReaderError('new', 'conversion', true, 'New failure');
  assert.equal(await persistModelDiagnostic(third, 'read-snapshot', { AWARE_HOME: home }), false);
  assert.equal(windowsScript(`[Console]::Write([System.IO.File]::GetAccessControl($env:AWARE_DIAGNOSTIC_TEST_FILE).GetSecurityDescriptorSddlForm('All'))`, unsafe), before);
  assert.equal(await fs.readFile(unsafe, 'utf8'), '{}');
  assertWindowsPrivate(first);
  assert.equal((await fs.readdir(directory)).filter((name) => name.endsWith('.json')).length, 2);
});

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
  const priorPaths = [];
  for (let index = 0; index < DIAGNOSTIC_LIMITS.records; index += 1) {
    const prior = new ModelReaderError('prior', 'conversion', true, 'Prior failure');
    const pathname = path.join(directory, `${prior.diagnosticId}.json`);
    priorPaths.push(pathname);
    if (process.platform !== 'win32') await fs.writeFile(pathname, '{}', { mode: 0o600 });
    oldest ??= pathname;
  }
  if (process.platform === 'win32') windowsScript(`$acl = [System.Security.AccessControl.FileSecurity]::new()
$acl.SetOwner($sid)
$acl.SetAccessRuleProtection($true, $false)
$acl.AddAccessRule([System.Security.AccessControl.FileSystemAccessRule]::new($sid, 'FullControl', 'Allow'))
foreach ($name in @(${priorPaths.map((name) => `'${path.basename(name)}'`).join(',')})) {
  $stream = [System.IO.FileStream]::new([System.IO.Path]::Combine($env:AWARE_DIAGNOSTIC_TEST_FILE, $name), [System.IO.FileMode]::CreateNew, [System.Security.AccessControl.FileSystemRights]::FullControl, [System.IO.FileShare]::None, 4096, [System.IO.FileOptions]::None, $acl)
  try { $stream.WriteByte(123); $stream.WriteByte(125) } finally { $stream.Dispose() }
}`, directory);
  for (const [index, pathname] of priorPaths.entries()) await fs.utimes(pathname, new Date(index * 1000), new Date(index * 1000));
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
  const prior = new ModelReaderError('prior', 'conversion', true, 'Prior failure');
  assert.equal(await persistModelDiagnostic(prior, 'read-snapshot', { AWARE_HOME: home }), true);
  await fs.unlink(path.join(directory, `${prior.diagnosticId}.json`));
  const lock = path.join(directory, '.lock');
  await fs.writeFile(lock, '999999999');
  const before = safeErrorEnvelope(error);
  assert.equal(await persistModelDiagnostic(error, 'read-snapshot', { AWARE_HOME: home }), false);
  assert.equal(await fs.readFile(lock, 'utf8'), '999999999');
  assert.deepEqual(safeErrorEnvelope(error), before);
  await fs.unlink(lock);
  const target = path.join(directory, `${error.diagnosticId}.json`);
  assert.equal(await persistModelDiagnostic(prior, 'read-snapshot', { AWARE_HOME: home }), true);
  await fs.rename(path.join(directory, `${prior.diagnosticId}.json`), target);
  const existing = await fs.readFile(target);
  assert.equal(await persistModelDiagnostic(error, 'read-snapshot', { AWARE_HOME: home }), false);
  assert.deepEqual(await fs.readFile(target), existing);
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

test('POSIX home creation races reject the actual post-mkdir ancestor for its owner or permissions', async (t) => {
  // Model the syscall race directly so both Windows and POSIX CI execute this negative control.
  // The trusted sticky parent is admitted; another process creates the missing home before mkdir.
  const platform = Object.getOwnPropertyDescriptor(process, 'platform');
  const getuid = Object.getOwnPropertyDescriptor(process, 'getuid');
  Object.defineProperty(process, 'platform', { value: 'linux', configurable: true });
  Object.defineProperty(process, 'getuid', { value: () => 1000, configurable: true });
  t.after(() => {
    Object.defineProperty(process, 'platform', platform);
    if (getuid) Object.defineProperty(process, 'getuid', getuid); else delete process.getuid;
  });
  const home = path.resolve('diagnostic-sticky-parent', 'race-created-home');
  const parent = path.dirname(home);
  let probes = 0; let created = 0; let raceUid = 2000;
  const stat = (uid, mode) => ({ uid, mode, isDirectory: () => true, isSymbolicLink: () => false });
  t.mock.method(fs, 'realpath', async (pathname) => path.resolve(pathname));
  t.mock.method(fs, 'lstat', async (pathname) => {
    if (pathname === home) {
      probes += 1;
      if (probes === 1) throw Object.assign(new Error('missing home'), { code: 'ENOENT' });
      return stat(raceUid, 0o777);
    }
    return stat(0, pathname === parent ? 0o1777 : 0o755);
  });
  t.mock.method(fs, 'mkdir', async (pathname) => {
    assert.equal(pathname, home, 'only the initially missing home may be created');
    created += 1;
    throw Object.assign(new Error('raced directory exists'), { code: 'EEXIST' });
  });
  await assert.rejects(ensureDiagnosticHomeAncestors(home), { message: 'Untrusted diagnostic ancestor' });
  assert.equal(probes, 2, 'the actual post-mkdir directory must be inspected');
  assert.equal(created, 1);
  probes = 0; created = 0; raceUid = 1000;
  await assert.rejects(ensureDiagnosticHomeAncestors(home), { message: 'Replaceable diagnostic ancestor' });
  assert.equal(probes, 2);
  assert.equal(created, 1);
});

test('PID write failure releases the newly owned diagnostic fence and storage recovery succeeds', async (t) => {
  const { home, error, directory } = await fixture(t);
  const open = fs.open.bind(fs);
  let failed = false; let closed = false;
  t.mock.method(fs, 'open', async (pathname, ...options) => {
    const handle = await open(pathname, ...options);
    if (pathname === path.join(directory, '.lock') && !failed) {
      failed = true;
      return {
        writeFile: async () => { throw Object.assign(new Error('PID write ran out of space'), { code: 'ENOSPC' }); },
        close: async () => { closed = true; await handle.close(); },
      };
    }
    return handle;
  });
  assert.equal(await persistModelDiagnostic(error, 'read-snapshot', { AWARE_HOME: home }), false);
  assert.equal(failed, true, 'the fault must occur while initializing the newly created fence');
  assert.equal(closed, true, 'the owned file handle must be closed after PID write fails');
  await assert.rejects(fs.lstat(path.join(directory, '.lock')), { code: 'ENOENT' });
  assert.equal(await persistModelDiagnostic(error, 'read-snapshot', { AWARE_HOME: home }), true);
  const record = JSON.parse(await fs.readFile(path.join(directory, `${error.diagnosticId}.json`), 'utf8'));
  assert.match(Buffer.from(record.evidence.stderr.data, 'base64').toString('utf8'), /PRIVATE-608/);
});

test('private diagnostics capture the native error throw site and never invoke a custom stack getter', async (t) => {
  const { error } = await fixture(t);
  const throwProviderFailure = () => { throw new Error('PRIVATE-608 unexpected converter exception'); };
  let thrown;
  try { throwProviderFailure(); } catch (failure) { thrown = failure; }
  error.unsafeDetails.cause = thrown;
  const record = JSON.parse(diagnosticRecord(error, 'read-snapshot'));
  assert.match(record.evidence.cause.stack, /throwProviderFailure/);
  assert.match(record.evidence.cause.stack, /PRIVATE-608 unexpected converter exception/);
  Object.defineProperty(thrown, 'stack', { configurable: true, get() { assert.fail('custom error stack getter was invoked'); } });
  const malicious = JSON.parse(diagnosticRecord(error, 'read-snapshot'));
  assert.equal(malicious.evidence.cause.stack, undefined);
  assert.equal(malicious.evidence.cause.message, thrown.message);
  const maliciousName = new Error('PRIVATE-608 native formatting input');
  Object.defineProperty(maliciousName, 'name', { get() { assert.fail('native stack formatting invoked a custom name getter'); } });
  error.unsafeDetails.cause = maliciousName;
  assert.equal(JSON.parse(diagnosticRecord(error, 'read-snapshot')).evidence.cause.stack, undefined);
});
