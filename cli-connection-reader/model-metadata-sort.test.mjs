import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import test from 'node:test';

import {
  createExternalSortMetadataRecordsForTesting,
  externalSortMetadataRecords,
} from './model-metadata-sort.mjs';

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

test('bounded snapshot reads getters once and measures the complete canonical line', async (t) => {
  const tempParent = await temporary(t);
  let recordReads = 0; let valueReads = 0;
  const record = Object.defineProperty({}, 'value', {
    enumerable: true,
    get() { valueReads += 1; return valueReads === 1 ? 'kept' : 'x'.repeat(1024); },
  });
  const input = { key: 'entity:getter' };
  Object.defineProperty(input, 'record', {
    enumerable: true,
    get() { recordReads += 1; return record; },
  });
  const expected = Buffer.from('{"key":"entity:getter","record":{"value":"kept"}}\n');
  const sorted = await externalSortMetadataRecords([input], {
    tempParent,
    limits: { recordBytes: expected.length },
  });
  assert.equal(recordReads, 1);
  assert.equal(valueReads, 1);
  assert.deepEqual(await fs.readFile(sorted.pathname), expected);
  await fs.rm(sorted.root, { recursive: true, force: true });

  await assert.rejects(
    () => externalSortMetadataRecords([{ key: 'entity:getter', record: { value: 'kept' } }], {
      tempParent,
      limits: { recordBytes: expected.length - 1 },
    }),
    (error) => error.code === 'reference-artifact-v2-limit',
  );
});

test('bounded snapshot applies its byte gate before sorting a wide object', async (t) => {
  const tempParent = await temporary(t);
  const reads = [];
  const record = {};
  Object.defineProperty(record, 'z-first', {
    enumerable: true,
    get() { reads.push('z-first'); return 'x'.repeat(128); },
  });
  for (let index = 0; index < 10_000; index += 1) {
    Object.defineProperty(record, `a-${String(index).padStart(5, '0')}`, {
      enumerable: true,
      get() { reads.push('later'); return index; },
    });
  }
  await assert.rejects(
    () => externalSortMetadataRecords([{ key: 'entity:wide', record }], {
      tempParent,
      limits: { recordBytes: 64 },
    }),
    (error) => error.code === 'reference-artifact-v2-limit',
  );
  assert.deepEqual(reads, ['z-first']);
});

test('deep input is refused before canonical allocation without changing shard admission', async (t) => {
  const tempParent = await temporary(t);
  let record = 'leaf';
  for (let depth = 0; depth < 129; depth += 1) record = { value: record };
  await assert.rejects(
    () => externalSortMetadataRecords([{ key: 'entity:deep', record }], { tempParent }),
    (error) => error.code === 'reference-artifact-v2-limit',
  );
  assert.deepEqual(await fs.readdir(tempParent), []);
});

test('initial and merge run counts have independent hard stops', async (t) => {
  const tempParent = await temporary(t);
  await assert.rejects(
    () => externalSortMetadataRecords(values, {
      tempParent,
      limits: { runBytes: 70, fanIn: 2, initialRuns: 2 },
    }),
    (error) => error.code === 'reference-artifact-v2-limit',
  );
  assert.deepEqual(await fs.readdir(tempParent), []);

  await assert.rejects(
    () => externalSortMetadataRecords(values, {
      tempParent,
      limits: { runBytes: 70, fanIn: 2, mergeRuns: 1 },
    }),
    (error) => error.code === 'reference-artifact-v2-limit',
  );
  assert.deepEqual(await fs.readdir(tempParent), []);
});

for (const [name, corrupt] of [
  ['oversized', async (pathname) => fs.writeFile(pathname, Buffer.alloc(257, 0x61))],
  ['invalid UTF-8', async (pathname) => fs.writeFile(pathname, Buffer.from([0x7b, 0x22, 0xff, 0x0a]))],
  ['CRLF', async (pathname) => {
    const bytes = await fs.readFile(pathname);
    await fs.writeFile(pathname, Buffer.concat([bytes.subarray(0, -1), Buffer.from('\r\n')]));
  }],
  ['unterminated', async (pathname) => {
    const bytes = await fs.readFile(pathname);
    await fs.writeFile(pathname, bytes.subarray(0, -1));
  }],
]) {
  test(`raw run reader rejects ${name} records and cleans its root`, async (t) => {
    const tempParent = await temporary(t);
    let changed = false;
    const sort = createExternalSortMetadataRecordsForTesting({
      async afterRunWritten(pathname, run) {
        if (!changed && run.kind === 'initial') { changed = true; await corrupt(pathname); }
      },
    });
    await assert.rejects(
      () => sort(values.slice(0, 2), {
        tempParent,
        limits: { runBytes: 70, recordBytes: 256, fanIn: 2 },
      }),
      (error) => error.code === 'reference-artifact-v2-invalid',
    );
    assert.deepEqual(await fs.readdir(tempParent), []);
  });
}

test('abort interrupts a blocked merge read and closes the opened handle', async (t) => {
  const tempParent = await temporary(t);
  const controller = new AbortController();
  let readOpens = 0; let readCloses = 0;
  const io = Object.create(fs);
  io.open = async (pathname, flags, mode) => {
    const handle = await fs.open(pathname, flags, mode);
    if (flags !== 'r') return handle;
    readOpens += 1;
    return {
      read() { controller.abort(); return new Promise(() => {}); },
      async close() { readCloses += 1; await handle.close(); },
    };
  };
  const sort = createExternalSortMetadataRecordsForTesting({ fs: io });
  await assert.rejects(
    () => sort(values.slice(0, 2), {
      tempParent,
      signal: controller.signal,
      limits: { runBytes: 70, fanIn: 2 },
    }),
    (error) => error.code === 'reference-cancelled',
  );
  assert.equal(readOpens, 1);
  assert.equal(readCloses, 1);
  assert.deepEqual(await fs.readdir(tempParent), []);
});

test('abort and cleanup do not wait forever for an uncooperative input iterator', async (t) => {
  const tempParent = await temporary(t);
  const controller = new AbortController();
  const input = {
    [Symbol.asyncIterator]() {
      return {
        next() { return new Promise(() => {}); },
        return() { return new Promise(() => {}); },
      };
    },
  };
  const started = Date.now();
  const sorting = externalSortMetadataRecords(input, { tempParent, signal: controller.signal });
  setTimeout(() => controller.abort(), 20);
  await assert.rejects(sorting, (error) => error.code === 'reference-cancelled');
  assert.ok(Date.now() - started < 500);
  assert.deepEqual(await fs.readdir(tempParent), []);
});

test('synchronous abort still observes the rejected iterator result', async (t) => {
  const tempParent = await temporary(t);
  const controller = new AbortController();
  const unhandled = [];
  const onUnhandled = (error) => unhandled.push(error);
  process.on('unhandledRejection', onUnhandled);
  t.after(() => process.off('unhandledRejection', onUnhandled));
  const input = {
    [Symbol.asyncIterator]() {
      return {
        next() {
          controller.abort();
          return Promise.reject(new Error('producer aborted'));
        },
      };
    },
  };
  await assert.rejects(
    () => externalSortMetadataRecords(input, { tempParent, signal: controller.signal }),
    (error) => error.code === 'reference-cancelled',
  );
  await new Promise((resolve) => setImmediate(resolve));
  assert.deepEqual(unhandled, []);
  assert.deepEqual(await fs.readdir(tempParent), []);
});

test('merge output close failures become I/O errors and clean the owned root', async (t) => {
  const tempParent = await temporary(t);
  const io = Object.create(fs);
  let writeOpens = 0;
  io.open = async (pathname, flags, mode) => {
    const handle = await fs.open(pathname, flags, mode);
    if (flags !== 'wx' || ++writeOpens !== 3) return handle;
    return {
      write: handle.write.bind(handle),
      sync: handle.sync.bind(handle),
      async close() {
        await handle.close();
        throw new Error('injected merge close failure');
      },
    };
  };
  const sort = createExternalSortMetadataRecordsForTesting({ fs: io });
  await assert.rejects(
    () => sort(values.slice(0, 2), {
      tempParent,
      limits: { runBytes: 70, fanIn: 2 },
    }),
    (error) => error.code === 'reference-artifact-v2-io'
      && error.unsafeDetails.message === 'injected merge close failure',
  );
  assert.deepEqual(await fs.readdir(tempParent), []);
});

test('cleanup failure preserves the primary code and non-public leaked-root diagnostics', async (t) => {
  const tempParent = await temporary(t);
  const io = Object.create(fs);
  io.rm = async (pathname, options) => {
    if (options?.recursive) throw new Error('injected cleanup failure');
    return fs.rm(pathname, options);
  };
  const sort = createExternalSortMetadataRecordsForTesting({ fs: io });
  let failure;
  try {
    await sort([values[0], values[1], { ...values[0] }], {
      tempParent,
      limits: { runBytes: 70, fanIn: 2 },
    });
  } catch (error) {
    failure = error;
  }
  assert.equal(failure.code, 'reference-artifact-v2-duplicate');
  assert.equal(failure.unsafeDetails.cleanupError.message, 'injected cleanup failure');
  assert.equal(path.dirname(failure.unsafeDetails.leakedRoot), tempParent);
  assert.equal(Object.keys(failure).includes('unsafeDetails'), false);
  t.after(() => fs.rm(failure.unsafeDetails.leakedRoot, { recursive: true, force: true }));
});
