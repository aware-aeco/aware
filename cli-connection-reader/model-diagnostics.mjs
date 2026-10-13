import { execFile } from 'node:child_process';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { promisify } from 'node:util';
import { isNativeError } from 'node:util/types';
import { ModelReaderError, safeErrorEnvelope } from './model-contract.mjs';

export const DIAGNOSTIC_LIMITS = Object.freeze({ records: 64, recordBytes: 256 * 1024, streamBytes: 64 * 1024 });
const UUID = /^[0-9a-f]{8}-[0-9a-f]{4}-[1-8][0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i;
const execute = promisify(execFile);
const nativeStackGetter = Object.getOwnPropertyDescriptor(new Error(), 'stack')?.get;

function boundedText(value, bytes = 8192) {
  return typeof value === 'string' ? Buffer.from(value.slice(0, bytes)).subarray(0, bytes).toString('utf8') : undefined;
}

function ownData(value, field) {
  const descriptor = Object.getOwnPropertyDescriptor(value, field);
  if (field === 'stack' && isNativeError(value) && nativeStackGetter
      && descriptor?.get === nativeStackGetter) {
    // Native stack formatting also reads the error name/message. Refuse accessor-backed
    // formatting inputs so the trusted stack getter cannot indirectly invoke custom getters.
    for (const input of ['name', 'message']) {
      let cursor = value;
      while (cursor) {
        const fieldDescriptor = Object.getOwnPropertyDescriptor(cursor, input);
        if (fieldDescriptor) {
          if (!Object.hasOwn(fieldDescriptor, 'value')) return undefined;
          break;
        }
        cursor = Object.getPrototypeOf(cursor);
      }
    }
    return nativeStackGetter.call(value);
  }
  return descriptor && Object.hasOwn(descriptor, 'value') ? descriptor.value : undefined;
}

// Do not serialize arbitrary objects: a host request can contain authorization, environments,
// entire source closures or cyclic objects. Preserve only bounded failure and stream evidence.
function privateEvidence(value, budget, seen = new Set(), depth = 0) {
  if (!value || typeof value !== 'object' || seen.has(value) || depth > 3) return undefined;
  seen.add(value);
  const result = {};
  for (const field of ['name', 'message', 'stack', 'code', 'reason']) {
    // Account for worst-case JSON escaping before constructing text or base64 buffers.
    const text = boundedText(ownData(value, field), Math.min(8192, Math.max(0, Math.floor((budget.remaining - 128) / 6))));
    if (text !== undefined && budget.remaining > 128) {
      result[field] = text;
      budget.remaining -= Buffer.byteLength(JSON.stringify(text)) + 128;
    }
  }
  const exitCode = ownData(value, 'exitCode');
  if (Number.isSafeInteger(exitCode) && budget.remaining > 128) { result.exitCode = exitCode; budget.remaining -= 128; }
  for (const field of ['stderr', 'stdout']) {
    const bytes = ownData(value, field);
    if (!Buffer.isBuffer(bytes) || budget.remaining < 256) continue;
    const retained = bytes.subarray(0, Math.min(DIAGNOSTIC_LIMITS.streamBytes, Math.floor((budget.remaining - 256) / 4) * 3));
    result[field] = { encoding: 'base64', bytes: bytes.length, retainedBytes: retained.length,
      truncated: retained.length !== bytes.length, data: retained.toString('base64') };
    budget.remaining -= Math.ceil(retained.length / 3) * 4 + 256;
  }
  for (const field of ['unsafeDetails', 'cause', 'conversionError', 'cleanupError']) {
    if (budget.remaining < 128) break;
    budget.remaining -= 128;
    const child = privateEvidence(ownData(value, field), budget, seen, depth + 1);
    if (child) result[field] = child;
  }
  return Object.keys(result).length ? result : undefined;
}

export function diagnosticRecord(error, command) {
  if (!(error instanceof ModelReaderError) || !UUID.test(error.diagnosticId)) return null;
  const budget = { remaining: DIAGNOSTIC_LIMITS.recordBytes - 4096 };
  const record = { schemaVersion: 'aware.model-reader-diagnostic/v1', createdAt: new Date().toISOString(),
    command: boundedText(command, 128), error: safeErrorEnvelope(error),
    evidence: privateEvidence(error.unsafeDetails, budget) };
  const cleanup = ownData(error, 'cleanupError');
  if (cleanup) { record.cleanupFailed = true; record.cleanupError = privateEvidence(cleanup, budget); }
  let bytes = Buffer.from(JSON.stringify(record));
  if (bytes.length > DIAGNOSTIC_LIMITS.recordBytes) {
    // Multiple nested host errors can exceed the document budget despite per-field bounds.
    // Keep the primary evidence, including its raw provider streams, before subsidiary causes.
    const evidence = record.evidence;
    if (evidence) for (const field of ['unsafeDetails', 'cause', 'conversionError', 'cleanupError']) delete evidence[field];
    record.truncated = true;
    bytes = Buffer.from(JSON.stringify(record));
  }
  return bytes.length <= DIAGNOSTIC_LIMITS.recordBytes ? bytes : null;
}

async function realDirectory(directory) {
  const stat = await fs.lstat(directory);
  const real = await fs.realpath(directory);
  const normalize = (value) => process.platform === 'win32' ? path.resolve(value).toLowerCase() : path.resolve(value);
  if (!stat.isDirectory() || stat.isSymbolicLink() || normalize(real) !== normalize(directory)) throw new Error('Unsafe diagnostic directory');
  return stat;
}

export async function ensureDiagnosticHomeAncestors(home) {
  const absolute = path.resolve(home);
  // Check every existing ancestor before creating anything, including junctions on Windows.
  let cursor = path.parse(absolute).root;
  for (const component of absolute.slice(cursor.length).split(path.sep).filter(Boolean)) {
    cursor = path.join(cursor, component);
    let ancestor;
    try { ancestor = await realDirectory(cursor); }
    catch (error) {
      if (error.code !== 'ENOENT') throw error;
      try { await fs.mkdir(cursor, { mode: 0o700 }); }
      catch (created) { if (created.code !== 'EEXIST') throw created; }
      ancestor = await realDirectory(cursor);
    }
    // Validate the actual directory admitted after creation too: EEXIST can mean a different
    // user won the creation race beneath a shared sticky parent.
    if (process.platform !== 'win32') {
      if (ancestor.uid !== 0 && ancestor.uid !== process.getuid()) throw new Error('Untrusted diagnostic ancestor');
      if ((ancestor.mode & 0o022) && !(ancestor.mode & 0o1000)) throw new Error('Replaceable diagnostic ancestor');
    }
  }
  return absolute;
}

async function diagnosticDirectory(home) {
  const absolute = await ensureDiagnosticHomeAncestors(home);
  const directory = path.join(absolute, 'model-reader-diagnostics');
  try { await fs.mkdir(directory, { mode: 0o700 }); }
  catch (error) { if (error.code !== 'EEXIST') throw error; }
  const stat = await realDirectory(directory);
  for (const name of await fs.readdir(directory)) {
    if (!UUID.test(name.replace(/\.json$/, '')) || !name.endsWith('.json')) continue;
    const record = await fs.lstat(path.join(directory, name));
    if (!record.isFile() || record.isSymbolicLink() || record.nlink !== 1 || record.size > DIAGNOSTIC_LIMITS.recordBytes) throw new Error('Unsafe diagnostic record');
    if (process.platform !== 'win32' && (record.uid !== process.getuid() || (record.mode & 0o777) !== 0o600)) throw new Error('Unsafe diagnostic permissions');
  }
  if (process.platform === 'win32') {
    // chmod does not protect Windows files. Secure the EMPTY directory before raw bytes are
    // created; records then inherit only the current operator's access. Refuse another owner.
    const script = `$ErrorActionPreference = 'Stop'
$root = $env:AWARE_PRIVATE_DIAGNOSTIC_ROOT
$sid = [System.Security.Principal.WindowsIdentity]::GetCurrent().User
$trusted = @($sid.Value, 'S-1-5-18', 'S-1-5-32-544', 'S-1-5-80-956008885-3418522649-1831038044-1853292631-2271478464')
$cursor = [System.IO.DirectoryInfo]::new($root).Parent
while ($null -ne $cursor) {
  $parentAcl = [System.IO.Directory]::GetAccessControl($cursor.FullName)
  if ($parentAcl.GetOwner([System.Security.Principal.SecurityIdentifier]).Value -notin $trusted) { throw 'Untrusted diagnostic ancestor' }
  foreach ($entry in $parentAcl.Access) {
    $entrySid = $entry.IdentityReference.Translate([System.Security.Principal.SecurityIdentifier]).Value
    $dangerous = if ($null -eq $cursor.Parent) { 786496 } else { 852032 }
    if ($entry.AccessControlType -eq 'Allow' -and -not ($entry.PropagationFlags -band [System.Security.AccessControl.PropagationFlags]::InheritOnly) -and $entrySid -notin $trusted -and ($entry.FileSystemRights -band $dangerous)) { throw 'Replaceable diagnostic ancestor' }
  }
  $cursor = $cursor.Parent
}
$existing = [System.IO.Directory]::GetAccessControl($root)
if ($existing.GetOwner([System.Security.Principal.SecurityIdentifier]) -ne $sid) { throw 'Diagnostic owner mismatch' }
$acl = [System.Security.AccessControl.DirectorySecurity]::new()
$acl.SetOwner($sid)
$acl.SetAccessRuleProtection($true, $false)
$rule = [System.Security.AccessControl.FileSystemAccessRule]::new($sid, 'FullControl', 'ContainerInherit,ObjectInherit', 'None', 'Allow')
$acl.AddAccessRule($rule)
[System.IO.Directory]::SetAccessControl($root, $acl)
foreach ($file in [System.IO.Directory]::GetFileSystemEntries($root)) {
  if ([System.IO.Path]::GetFileName($file) -notmatch '^[0-9a-f-]{36}\\.json$') { continue }
  if ([System.IO.File]::GetAttributes($file) -band ([System.IO.FileAttributes]::ReparsePoint -bor [System.IO.FileAttributes]::Directory)) { throw 'Unsafe diagnostic record' }
  $old = [System.IO.File]::GetAccessControl($file)
  if ($old.GetOwner([System.Security.Principal.SecurityIdentifier]) -ne $sid) { throw 'Diagnostic owner mismatch' }
  if ($old.Access.Count -ne 1 -or $old.Access[0].IdentityReference.Translate([System.Security.Principal.SecurityIdentifier]) -ne $sid -or $old.Access[0].AccessControlType -ne 'Allow') { throw 'Unsafe diagnostic permissions' }
}`;
    await execute(path.join(process.env.SystemRoot ?? 'C:\\Windows', 'System32', 'WindowsPowerShell', 'v1.0', 'powershell.exe'),
      ['-NoProfile', '-NonInteractive', '-EncodedCommand', Buffer.from(script, 'utf16le').toString('base64')],
      { env: { ...process.env, AWARE_PRIVATE_DIAGNOSTIC_ROOT: directory }, windowsHide: true, timeout: 10_000, maxBuffer: 4096 });
  } else {
    if (stat.uid !== process.getuid()) throw new Error('Diagnostic owner mismatch');
    await fs.chmod(directory, 0o700);
  }
  await realDirectory(directory);
  return directory;
}

async function acquireLock(directory) {
  const lock = path.join(directory, '.lock');
  for (let attempt = 0; attempt < 20; attempt += 1) {
    try {
      const handle = await fs.open(lock, 'wx', 0o600);
      try {
        await handle.writeFile(String(process.pid));
        return { handle, lock };
      } catch (error) {
        await handle.close().catch(() => {});
        await fs.unlink(lock).catch(() => {});
        throw error;
      }
    } catch (error) {
      if (error.code !== 'EEXIST') throw error;
      // Never reclaim a fence by pathname: even an inode check followed by unlink can delete
      // a replacement writer's lock. A crashed fence requires operator removal; diagnostics
      // remain best-effort and preserve the original failure while the fence is unavailable.
      const stat = await fs.lstat(lock);
      if (!stat.isFile() || stat.isSymbolicLink() || stat.size > 32) throw new Error('Unsafe diagnostic lock');
      await new Promise((resolve) => setTimeout(resolve, 25));
    }
  }
  return null;
}

export async function persistModelDiagnostic(error, command, environment = process.env) {
  // Diagnostics must never mask the original failure or fall back to public stderr.
  let fence; let target; let committed = false; let ownsTarget = false;
  try {
    const bytes = diagnosticRecord(error, command);
    if (!bytes) return false;
    const directory = await diagnosticDirectory(environment.AWARE_HOME || path.join(os.homedir(), '.aware'));
    fence = await acquireLock(directory);
    if (!fence) return false;
    const entries = [];
    for (const name of await fs.readdir(directory)) {
      if (!UUID.test(name.replace(/\.json$/, '')) || !name.endsWith('.json')) continue;
      const pathname = path.join(directory, name);
      const stat = await fs.lstat(pathname);
      if (!stat.isFile() || stat.isSymbolicLink() || stat.nlink !== 1 || stat.size > DIAGNOSTIC_LIMITS.recordBytes) throw new Error('Unsafe diagnostic record');
      if (process.platform !== 'win32') {
        if (stat.uid !== process.getuid() || (stat.mode & 0o777) !== 0o600) throw new Error('Unsafe diagnostic permissions');
      }
      entries.push({ pathname, created: stat.mtimeMs });
    }
    entries.sort((left, right) => left.created - right.created || left.pathname.localeCompare(right.pathname));
    while (entries.length >= DIAGNOSTIC_LIMITS.records) await fs.unlink(entries.shift().pathname);
    target = path.join(directory, `${error.diagnosticId}.json`);
    const handle = await fs.open(target, 'wx', 0o600);
    ownsTarget = true;
    try { await handle.writeFile(bytes); await handle.sync(); }
    finally { await handle.close(); }
    if (process.platform !== 'win32') {
      const parent = await fs.open(directory, 'r');
      try { await parent.sync(); } finally { await parent.close(); }
    }
    committed = true;
    return true;
  } catch { return false; }
  finally {
    if (ownsTarget && !committed) await fs.unlink(target).catch(() => {});
    if (fence) {
      await fence.handle.close().catch(() => {});
      await fs.unlink(fence.lock).catch(() => {});
    }
  }
}
