import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';

import { externalSortMetadataRecords } from './model-metadata-sort.mjs';

async function temporary(t) {
  const root = await fs.mkdtemp(path.join(await fs.realpath(os.tmpdir()), 'aware-sort-test-'));
  t.after(() => fs.rm(root, { recursive: true, force: true }));
  return root;
}

const values = [
  { key: 'entity:003', record: { id: 'entity:003', value: 3 } },
  { key: 'entity:001', record: { id: 'entity:001', value: 1 } },
  { key: 'entity:004', record: { id: 'entity:004', value: 4 } },
  { key: 'entity:002', record: { id: 'entity:002', value: 2 } },
];

test('external sort is deterministic across input and run order', async (t) => {
  const tempParent = await temporary(t);
  const first = await externalSortMetadataRecords(values, { tempParent, limits: { runBytes: 100, fanIn: 2 } });
  const second = await externalSortMetadataRecords([...values].reverse(), { tempParent, limits: { runBytes: 100, fanIn: 2 } });
  assert.equal(first.count, 4);
  assert.deepEqual(await fs.readFile(first.pathname), await fs.readFile(second.pathname));
  assert.equal((await fs.readFile(first.pathname, 'utf8')).split('\n').filter(Boolean).map((line) => JSON.parse(line))
    .map((entry) => entry.key).join(','), 'entity:001,entity:002,entity:003,entity:004');
});

test('async input and empty input produce canonical deterministic files', async (t) => {
  const tempParent = await temporary(t);
  async function* shuffled() { yield values[2]; yield values[0]; yield values[3]; yield values[1]; }
  const sorted = await externalSortMetadataRecords(shuffled(), { tempParent, limits: { runBytes: 100, fanIn: 2 } });
  const empty = await externalSortMetadataRecords([], { tempParent });
  assert.equal(sorted.count, 4);
  assert.equal(empty.count, 0);
  assert.equal((await fs.readFile(empty.pathname)).length, 0);
  assert.equal((await fs.readFile(sorted.pathname, 'utf8')).split('\n').filter(Boolean)
    .map((line) => JSON.parse(line).key).join(','), 'entity:001,entity:002,entity:003,entity:004');
});

test('duplicates across separate runs fail and clean the owned root', async (t) => {
  const tempParent = await temporary(t);
  const input = [values[0], values[1], { ...values[0] }];
  const before = new Set(await fs.readdir(tempParent));
  await assert.rejects(
    () => externalSortMetadataRecords(input, { tempParent, limits: { runBytes: 100, fanIn: 2 } }),
    (error) => error.code === 'reference-artifact-v2-duplicate',
  );
  assert.deepEqual(new Set(await fs.readdir(tempParent)), before);
});

test('cancellation fails cleanly without leaving a sort root', async (t) => {
  const tempParent = await temporary(t);
  const controller = new AbortController(); controller.abort();
  await assert.rejects(
    () => externalSortMetadataRecords(values, { tempParent, signal: controller.signal }),
    (error) => error.code === 'reference-cancelled',
  );
  assert.deepEqual(await fs.readdir(tempParent), []);
});

test('cancellation during final run publication removes the owned root', async (t) => {
  const tempParent = await temporary(t);
  let reads = 0;
  const signal = { get aborted() { reads += 1; return reads >= 4; } };
  await assert.rejects(
    () => externalSortMetadataRecords([values[0]], { tempParent, signal }),
    (error) => error.code === 'reference-cancelled',
  );
  assert.deepEqual(await fs.readdir(tempParent), []);
});

test('late cancellation before publication removes an empty output', async (t) => {
  const tempParent = await temporary(t);
  let reads = 0;
  const signal = { get aborted() { reads += 1; return reads >= 2; } };
  await assert.rejects(
    () => externalSortMetadataRecords([], { tempParent, signal }),
    (error) => error.code === 'reference-cancelled',
  );
  assert.deepEqual(await fs.readdir(tempParent), []);
});

test('fan-in must make progress', async (t) => {
  const tempParent = await temporary(t);
  await assert.rejects(
    () => externalSortMetadataRecords(values, { tempParent, limits: { fanIn: 1 } }),
    (error) => error.code === 'reference-artifact-v2-limit-invalid',
  );
});

test('aggregate input bytes are bounded before further run writes', async (t) => {
  const tempParent = await temporary(t);
  const before = new Set(await fs.readdir(tempParent));
  await assert.rejects(
    () => externalSortMetadataRecords(values.slice(0, 2), { tempParent, limits: { totalBytes: 100 } }),
    (error) => error.code === 'reference-artifact-v2-limit',
  );
  assert.deepEqual(new Set(await fs.readdir(tempParent)), before);
});

test('record bytes have a separate ceiling from run bytes', async (t) => {
  const tempParent = await temporary(t);
  await assert.rejects(
    () => externalSortMetadataRecords([values[0]], {
      tempParent,
      limits: { recordBytes: 32, runBytes: 1024 },
    }),
    (error) => error.code === 'reference-artifact-v2-limit',
  );
  assert.deepEqual(await fs.readdir(tempParent), []);
});

test('throwing option and iterable accessors become stable reader errors', async (t) => {
  const tempParent = await temporary(t);
  const options = Object.defineProperty({}, 'tempParent', { get() { throw new Error('unreadable'); } });
  await assert.rejects(
    () => externalSortMetadataRecords(values, options),
    (error) => error.code === 'reference-artifact-v2-invalid',
  );
  const input = Object.defineProperty({}, Symbol.iterator, { get() { throw new Error('unreadable'); } });
  await assert.rejects(
    () => externalSortMetadataRecords(input, { tempParent }),
    (error) => error.code === 'reference-artifact-v2-invalid',
  );
  const broken = { [Symbol.asyncIterator]() { return { next() { throw new Error('unreadable'); } }; } };
  await assert.rejects(
    () => externalSortMetadataRecords(broken, { tempParent }),
    (error) => error.code === 'reference-artifact-v2-invalid',
  );
});
