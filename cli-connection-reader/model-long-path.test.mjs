import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';

import { mkdtempBeyondMaxPath } from './model-long-path.mjs';

function recordingFs() {
  const calls = [];
  return { calls, mkdtemp: async (prefix) => { calls.push(prefix); return `${prefix}AbC123`; } };
}

test('Windows prefixes are namespaced for the call and returned plain', async () => {
  const disk = recordingFs();
  assert.equal(await mkdtempBeyondMaxPath(disk, 'C:\\deep\\run-', 'win32'), 'C:\\deep\\run-AbC123');
  assert.deepEqual(disk.calls, ['\\\\?\\C:\\deep\\run-']);

  const unc = recordingFs();
  assert.equal(await mkdtempBeyondMaxPath(unc, '\\\\server\\share\\run-', 'win32'), '\\\\server\\share\\run-AbC123');
  assert.deepEqual(unc.calls, ['\\\\?\\UNC\\server\\share\\run-']);

  const verbatim = recordingFs();
  assert.equal(await mkdtempBeyondMaxPath(verbatim, '\\\\?\\C:\\x\\run-', 'win32'), '\\\\?\\C:\\x\\run-AbC123');
  assert.deepEqual(verbatim.calls, ['\\\\?\\C:\\x\\run-']);
});

test('other platforms pass the prefix through untouched', async () => {
  const posix = recordingFs();
  assert.equal(await mkdtempBeyondMaxPath(posix, '/deep/run-', 'linux'), '/deep/run-AbC123');
  assert.deepEqual(posix.calls, ['/deep/run-']);
});

test('a scratch directory is created past MAX_PATH and usable by plain fs calls', async (t) => {
  const base = await fs.mkdtemp(path.join(await fs.realpath(os.tmpdir()), 'aware-long-path-'));
  t.after(() => fs.rm(base, { recursive: true, force: true }));
  const parent = path.join(base, 'a'.repeat(120), 'b'.repeat(120));
  await fs.mkdir(parent, { recursive: true });
  const created = await mkdtempBeyondMaxPath(fs, path.join(parent, 'aware-model-sort-'));
  assert.ok(created.length > 260, `expected a path beyond MAX_PATH, got ${created.length}`);
  assert.ok(created.startsWith(parent), 'the caller keeps the plain path it joined from');
  const file = path.join(created, 'run-000000.jsonl');
  await fs.writeFile(file, 'x\n', { flag: 'wx' });
  assert.equal(await fs.readFile(file, 'utf8'), 'x\n');
});
