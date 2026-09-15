import { createReadStream } from 'node:fs';
import fs from 'node:fs/promises';
import path from 'node:path';
import { createInterface } from 'node:readline';

import { canonicalJsonBytes, ModelReaderError, parseJsonStrict } from './model-contract.mjs';
import { canonicalMetadataRecord, MAX_METADATA_RECORD_DEPTH } from './model-metadata-shards.mjs';

const DEFAULT_LIMITS = Object.freeze({
  runBytes: 64 * 1024 * 1024,
  totalBytes: 1.5 * 1024 * 1024 * 1024,
  fanIn: 32,
  records: 10_000_000,
});
const HARD_LIMITS = Object.freeze({
  runBytes: 64 * 1024 * 1024,
  totalBytes: 2 * 1024 * 1024 * 1024,
  fanIn: 32,
  records: 10_000_000,
});

function sortError(code, message, details = undefined) {
  throw new ModelReaderError(code, 'canonical-artifact', false, message, details);
}

function checkCancellation(signal) {
  let aborted;
  try { aborted = signal?.aborted; }
  catch (error) { sortError('reference-artifact-v2-invalid', 'Metadata sort input is invalid.', error); }
  if (aborted) sortError('reference-cancelled', 'Metadata sorting was cancelled.');
}

function enforcedLimits(overrides = {}) {
  let prototype; let entries;
  try {
    prototype = overrides && typeof overrides === 'object' && !Array.isArray(overrides)
      ? Object.getPrototypeOf(overrides) : undefined;
    entries = prototype === Object.prototype || prototype === null ? Object.entries(overrides) : [];
  } catch (error) {
    sortError('reference-artifact-v2-limit-invalid', 'Metadata sort limits are invalid.', error);
  }
  if (prototype !== Object.prototype && prototype !== null) {
    sortError('reference-artifact-v2-limit-invalid', 'Metadata sort limits are invalid.');
  }
  const result = { ...DEFAULT_LIMITS };
  for (const [name, value] of entries) {
    if (!Object.hasOwn(result, name) || !Number.isSafeInteger(value) || value <= 0
        || (name === 'fanIn' && value < 2)
        || value > HARD_LIMITS[name]) {
      sortError('reference-artifact-v2-limit-invalid', 'Metadata sort limits are invalid.');
    }
    result[name] = value;
  }
  return result;
}

function lineFor(record) {
  return Buffer.concat([canonicalJsonBytes({ key: record.key, record: record.record }), Buffer.from('\n')]);
}

async function writeAll(handle, bytes, signal) {
  let offset = 0;
  while (offset < bytes.length) {
    checkCancellation(signal);
    const length = Math.min(bytes.length - offset, 1024 * 1024);
    const result = await handle.write(bytes, offset, length, null);
    if (result.bytesWritten <= 0) sortError('reference-artifact-v2-io', 'A metadata sort run could not be written.');
    offset += result.bytesWritten;
  }
  checkCancellation(signal);
}

async function writeRun(records, pathname, signal) {
  checkCancellation(signal);
  records.sort((left, right) => Buffer.compare(left.keyBytes, right.keyBytes));
  checkCancellation(signal);
  for (let index = 1; index < records.length; index += 1) {
    checkCancellation(signal);
    if (Buffer.compare(records[index - 1].keyBytes, records[index].keyBytes) === 0) {
      sortError('reference-artifact-v2-duplicate', 'Metadata record identities must be unique.');
    }
  }
  const handle = await fs.open(pathname, 'wx', 0o600);
  try {
    for (const record of records) await writeAll(handle, lineFor(record), signal);
    await handle.sync();
    checkCancellation(signal);
  } finally {
    await handle.close();
  }
}

async function closeRun(state) {
  try {
    const returned = state.iterator.return?.();
    if (returned) await returned;
  } catch {
    // Cleanup is best-effort and must not replace the primary failure.
  }
  state.lines.close();
  state.stream.destroy();
}

async function openRun(pathname, recordBytes) {
  const stream = createReadStream(pathname, { highWaterMark: 1024 * 1024 });
  const lines = createInterface({ input: stream, crlfDelay: Infinity });
  const iterator = lines[Symbol.asyncIterator]();
  const state = { stream, lines, iterator, head: undefined, previous: undefined };
  state.advance = async () => {
    const next = await iterator.next();
    if (next.done) { state.head = undefined; return; }
    const bytes = Buffer.from(next.value);
    if (bytes.length === 0 || bytes.length > recordBytes) {
      sortError('reference-artifact-v2-invalid', 'A metadata sort run contains an invalid record.');
    }
    let value;
    try { value = parseJsonStrict(bytes, { maxBytes: recordBytes, maxDepth: MAX_METADATA_RECORD_DEPTH + 1 }); }
    catch (error) { sortError('reference-artifact-v2-invalid', 'A metadata sort run contains invalid JSON.', error); }
    const record = canonicalMetadataRecord(value);
    if (!canonicalJsonBytes({ key: record.key, record: record.record }).equals(bytes)) {
      sortError('reference-artifact-v2-invalid', 'A metadata sort run is not canonical.');
    }
    if (state.previous && Buffer.compare(state.previous, record.keyBytes) >= 0) {
      sortError('reference-artifact-v2-invalid', 'A metadata sort run is not strictly ordered.');
    }
    state.previous = record.keyBytes;
    state.head = record;
  };
  try {
    await state.advance();
    return state;
  } catch (error) {
    await closeRun(state);
    throw error;
  }
}

async function mergeRuns(inputs, output, limits, signal) {
  const states = [];
  let handle;
  try {
    for (const pathname of inputs) states.push(await openRun(pathname, limits.runBytes));
    handle = await fs.open(output, 'wx', 0o600);
    let previous;
    for (;;) {
      checkCancellation(signal);
      let selected;
      for (const state of states) {
        if (!state.head) continue;
        if (!selected || Buffer.compare(state.head.keyBytes, selected.head.keyBytes) < 0) selected = state;
      }
      if (!selected) break;
      if (previous && Buffer.compare(previous, selected.head.keyBytes) === 0) {
        sortError('reference-artifact-v2-duplicate', 'Metadata record identities must be unique.');
      }
      await writeAll(handle, lineFor(selected.head), signal);
      previous = selected.head.keyBytes;
      await selected.advance();
    }
    await handle.sync();
  } finally {
    await handle?.close().catch(() => {});
    await Promise.all(states.map(closeRun));
  }
}

async function* metadataInputs(records, syncIterator, asyncIterator) {
  let iterator;
  try { iterator = (asyncIterator ?? syncIterator).call(records); }
  catch (error) { sortError('reference-artifact-v2-invalid', 'Metadata sort input is invalid.', error); }
  if (!iterator || typeof iterator.next !== 'function') {
    sortError('reference-artifact-v2-invalid', 'Metadata sort input is invalid.');
  }
  try {
    for (;;) {
      let next; let done; let value;
      try {
        next = await iterator.next();
        done = next?.done;
        value = next?.value;
      } catch (error) {
        sortError('reference-artifact-v2-invalid', 'Metadata sort input is invalid.', error);
      }
      if (!next || typeof next !== 'object') {
        sortError('reference-artifact-v2-invalid', 'Metadata sort input is invalid.');
      }
      if (done) return;
      yield value;
    }
  } finally {
    try {
      const returned = iterator.return?.();
      if (returned) await returned;
    } catch {
      // The primary validation, cancellation, or I/O error remains authoritative.
    }
  }
}

export async function externalSortMetadataRecords(records, options = {}) {
  let optionKeys; let optionPrototype; let tempParent; let limitInput; let signal;
  let syncIterator; let asyncIterator;
  try {
    optionPrototype = options && typeof options === 'object' && !Array.isArray(options)
      ? Object.getPrototypeOf(options) : undefined;
    optionKeys = optionPrototype === Object.prototype || optionPrototype === null ? Object.keys(options) : [];
    tempParent = options?.tempParent;
    limitInput = options?.limits;
    signal = options?.signal;
    syncIterator = records?.[Symbol.iterator];
    asyncIterator = records?.[Symbol.asyncIterator];
  } catch (error) {
    sortError('reference-artifact-v2-invalid', 'Metadata sort input is invalid.', error);
  }
  if ((optionPrototype !== Object.prototype && optionPrototype !== null)
      || optionKeys.some((key) => !['tempParent', 'limits', 'signal'].includes(key))
      || typeof tempParent !== 'string' || !path.isAbsolute(tempParent)
      || (typeof syncIterator !== 'function' && typeof asyncIterator !== 'function')) {
    sortError('reference-artifact-v2-invalid', 'Metadata sort input is invalid.');
  }
  const limits = enforcedLimits(limitInput);
  checkCancellation(signal);
  let root;
  try {
    const realTempParent = await fs.realpath(tempParent);
    root = await fs.mkdtemp(path.join(realTempParent, 'aware-model-sort-'));
    let buffered = []; let bufferedBytes = 0; let totalBytes = 0; let count = 0; let ordinal = 0;
    const runs = [];
    for await (const input of metadataInputs(records, syncIterator, asyncIterator)) {
      checkCancellation(signal);
      const record = canonicalMetadataRecord(input);
      const bytes = lineFor(record).length;
      if (bytes > limits.runBytes) sortError('reference-artifact-v2-limit', 'One metadata record exceeds the sort run limit.');
      if (bytes > limits.totalBytes - totalBytes) {
        sortError('reference-artifact-v2-limit', 'The metadata family exceeds its aggregate byte limit.');
      }
      if (buffered.length && bufferedBytes + bytes > limits.runBytes) {
        const pathname = path.join(root, `run-${String(ordinal).padStart(6, '0')}.jsonl`);
        await writeRun(buffered, pathname, signal); runs.push(pathname); ordinal += 1;
        buffered = []; bufferedBytes = 0;
      }
      buffered.push(record); bufferedBytes += bytes; totalBytes += bytes; count += 1;
      if (count > limits.records) sortError('reference-artifact-v2-limit', 'The metadata family exceeds its record limit.');
    }
    if (buffered.length) {
      const pathname = path.join(root, `run-${String(ordinal).padStart(6, '0')}.jsonl`);
      await writeRun(buffered, pathname, signal); runs.push(pathname);
    }
    let pass = 0; let active = runs;
    while (active.length > 1) {
      const merged = [];
      for (let start = 0; start < active.length; start += limits.fanIn) {
        const batch = active.slice(start, start + limits.fanIn);
        const pathname = path.join(root, `merge-${String(pass).padStart(3, '0')}-${String(merged.length).padStart(6, '0')}.jsonl`);
        await mergeRuns(batch, pathname, limits, signal);
        await Promise.all(batch.map((input) => fs.rm(input, { force: true })));
        merged.push(pathname);
      }
      active = merged; pass += 1;
    }
    const pathname = active[0] ?? path.join(root, 'empty.jsonl');
    if (!active.length) await fs.writeFile(pathname, Buffer.alloc(0), { flag: 'wx', mode: 0o600 });
    checkCancellation(signal);
    return { root, pathname, count };
  } catch (error) {
    if (root) await fs.rm(root, { recursive: true, force: true }).catch(() => {});
    if (error instanceof ModelReaderError) throw error;
    sortError('reference-artifact-v2-io', 'Metadata sorting failed.', error);
  }
}
