import fs from 'node:fs/promises';
import path from 'node:path';

import { canonicalJsonBytes, ModelReaderError, parseJsonStrict } from './model-contract.mjs';
import { canonicalMetadataRecord } from './model-metadata-shards.mjs';

const DEFAULT_LIMITS = Object.freeze({
  runBytes: 64 * 1024 * 1024,
  recordBytes: 1024 * 1024,
  totalBytes: 1.5 * 1024 * 1024 * 1024,
  fanIn: 32,
  records: 10_000_000,
  initialRuns: 65_536,
  mergeRuns: 65_536,
});
const HARD_LIMITS = Object.freeze({
  runBytes: 64 * 1024 * 1024,
  recordBytes: 4 * 1024 * 1024,
  totalBytes: 2 * 1024 * 1024 * 1024,
  fanIn: 32,
  records: 10_000_000,
  initialRuns: 65_536,
  mergeRuns: 65_536,
});
const MAX_METADATA_RECORD_DEPTH = 128;
const NEWLINE = Buffer.from('\n');
const ENVELOPE_BYTES = Buffer.byteLength('{"key":,"record":}\n');
const READ_CHUNK_BYTES = 1024 * 1024;
const ITERATOR_FINALIZE_MS = 100;

function readerError(code, message, details = undefined) {
  return new ModelReaderError(code, 'canonical-artifact', false, message, details);
}

function sortError(code, message, details = undefined) {
  throw readerError(code, message, details);
}

function checkCancellation(signal) {
  let aborted;
  try { aborted = signal?.aborted; }
  catch (error) { sortError('reference-artifact-v2-invalid', 'Metadata sort input is invalid.', error); }
  if (aborted) throw readerError('reference-cancelled', 'Metadata sorting was cancelled.');
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

function jsonStringBytes(value) {
  let bytes = 2;
  for (let index = 0; index < value.length; index += 1) {
    const unit = value.charCodeAt(index);
    if (unit === 0x22 || unit === 0x5c || unit === 0x08 || unit === 0x09
        || unit === 0x0a || unit === 0x0c || unit === 0x0d) {
      bytes += 2;
    } else if (unit < 0x20) {
      bytes += 6;
    } else if (unit <= 0x7f) {
      bytes += 1;
    } else if (unit <= 0x7ff) {
      bytes += 2;
    } else if (unit >= 0xd800 && unit <= 0xdbff) {
      const low = value.charCodeAt(index + 1);
      if (low < 0xdc00 || low > 0xdfff) throw new TypeError('string must contain Unicode scalar values');
      bytes += 4; index += 1;
    } else if (unit >= 0xdc00 && unit <= 0xdfff) {
      throw new TypeError('string must contain Unicode scalar values');
    } else {
      bytes += 3;
    }
  }
  return bytes;
}

function boundedSnapshot(value, tracker, depth = 0, seen = new Set()) {
  if (depth > MAX_METADATA_RECORD_DEPTH) {
    sortError('reference-artifact-v2-limit', 'One metadata record exceeds its nesting limit.');
  }
  if (value === null) { tracker.add(4); return null; }
  if (typeof value === 'boolean') { tracker.add(value ? 4 : 5); return value; }
  if (typeof value === 'string') { tracker.add(jsonStringBytes(value)); return value; }
  if (typeof value === 'number') {
    if (!Number.isFinite(value) || (Number.isInteger(value) && !Number.isSafeInteger(value))) {
      throw new TypeError('number is not canonical JSON data');
    }
    const normalized = Object.is(value, -0) ? 0 : value;
    tracker.add(Buffer.byteLength(JSON.stringify(normalized)));
    return normalized;
  }
  if (!value || typeof value !== 'object') throw new TypeError('value is not JSON data');
  if (seen.has(value)) throw new TypeError('JSON value must be acyclic');
  seen.add(value);
  try {
    if (Array.isArray(value)) {
      tracker.add(2);
      const copy = [];
      for (let index = 0; index < value.length; index += 1) {
        if (!Object.hasOwn(value, index)) throw new TypeError('sparse array is not JSON data');
        if (index) tracker.add(1);
        copy.push(boundedSnapshot(value[index], tracker, depth + 1, seen));
      }
      return copy;
    }
    const prototype = Object.getPrototypeOf(value);
    if (prototype !== Object.prototype && prototype !== null) throw new TypeError('JSON object must be plain');
    const copy = {};
    let propertyCount = 0;
    tracker.add(2);
    for (const key in value) {
      if (!Object.hasOwn(value, key)) continue;
      if (propertyCount) tracker.add(1);
      tracker.add(jsonStringBytes(key) + 1);
      const child = boundedSnapshot(value[key], tracker, depth + 1, seen);
      Object.defineProperty(copy, key, { value: child, enumerable: true, configurable: true, writable: true });
      propertyCount += 1;
    }
    return copy;
  } finally {
    seen.delete(value);
  }
}

function encodedRecord(input, recordByteLimit) {
  try {
    const keys = input && typeof input === 'object' && !Array.isArray(input) ? Object.keys(input) : [];
    const prototype = input && typeof input === 'object' && !Array.isArray(input)
      ? Object.getPrototypeOf(input) : undefined;
    if (prototype !== Object.prototype && prototype !== null
        || keys.length !== 2 || !keys.includes('key') || !keys.includes('record')) {
      sortError('reference-artifact-v2-invalid', 'A metadata record is invalid.');
    }
    const key = input.key;
    const record = input.record;
    if (typeof key !== 'string' || !key) sortError('reference-artifact-v2-invalid', 'A metadata record is invalid.');
    const tracker = {
      bytes: ENVELOPE_BYTES,
      add(bytes) {
        if (bytes > recordByteLimit - this.bytes) {
          sortError('reference-artifact-v2-limit', 'One metadata record exceeds its byte limit.');
        }
        this.bytes += bytes;
      },
    };
    tracker.add(jsonStringBytes(key));
    const snapshot = { key, record: boundedSnapshot(record, tracker) };
    const canonical = canonicalMetadataRecord(snapshot);
    const bytes = canonicalJsonBytes({ key: canonical.key, record: canonical.record });
    if (bytes.length + NEWLINE.length !== tracker.bytes) {
      sortError('reference-artifact-v2-invalid', 'A metadata record is not canonical JSON data.');
    }
    return { keyBytes: canonical.keyBytes, bytes };
  } catch (error) {
    if (error instanceof ModelReaderError) throw error;
    sortError('reference-artifact-v2-invalid', 'A metadata record is not canonical JSON data.', error);
  }
}

async function awaitAbortable(value, signal) {
  const pending = Promise.resolve(value);
  pending.catch(() => undefined);
  checkCancellation(signal);
  let add; let remove;
  try {
    add = signal?.addEventListener;
    remove = signal?.removeEventListener;
  } catch (error) {
    sortError('reference-artifact-v2-invalid', 'Metadata sort input is invalid.', error);
  }
  if (typeof add !== 'function' || typeof remove !== 'function') {
    const result = await pending;
    checkCancellation(signal);
    return result;
  }
  return new Promise((resolve, reject) => {
    let settled = false;
    const aborted = () => {
      if (settled) return;
      settled = true;
      remove.call(signal, 'abort', aborted);
      reject(readerError('reference-cancelled', 'Metadata sorting was cancelled.'));
    };
    add.call(signal, 'abort', aborted, { once: true });
    try { if (signal.aborted) aborted(); }
    catch (error) {
      remove.call(signal, 'abort', aborted);
      reject(readerError('reference-artifact-v2-invalid', 'Metadata sort input is invalid.', error));
      return;
    }
    pending.then(
      (result) => {
        if (settled) return;
        settled = true; remove.call(signal, 'abort', aborted); resolve(result);
      },
      (error) => {
        if (settled) return;
        settled = true; remove.call(signal, 'abort', aborted); reject(error);
      },
    );
  });
}

async function writeAll(handle, bytes, signal) {
  let offset = 0;
  while (offset < bytes.length) {
    checkCancellation(signal);
    const length = Math.min(bytes.length - offset, READ_CHUNK_BYTES);
    const result = await handle.write(bytes, offset, length, null);
    if (result.bytesWritten <= 0) sortError('reference-artifact-v2-io', 'A metadata sort run could not be written.');
    offset += result.bytesWritten;
  }
  checkCancellation(signal);
}

async function writeRun(records, pathname, signal, io) {
  checkCancellation(signal);
  records.sort((left, right) => Buffer.compare(left.keyBytes, right.keyBytes));
  checkCancellation(signal);
  for (let index = 1; index < records.length; index += 1) {
    checkCancellation(signal);
    if (Buffer.compare(records[index - 1].keyBytes, records[index].keyBytes) === 0) {
      sortError('reference-artifact-v2-duplicate', 'Metadata record identities must be unique.');
    }
  }
  const handle = await io.open(pathname, 'wx', 0o600);
  let primary;
  try {
    for (const record of records) {
      await writeAll(handle, record.bytes, signal);
      await writeAll(handle, NEWLINE, signal);
    }
    await handle.sync();
    checkCancellation(signal);
  } catch (error) {
    primary = error;
  }
  let closeFailure;
  try { await handle.close(); }
  catch (error) { closeFailure = error; }
  if (primary) {
    if (closeFailure) throw primaryWithSecondary(primary, 'closeError', closeFailure);
    throw primary;
  }
  if (closeFailure) {
    sortError('reference-artifact-v2-io', 'A metadata sort run could not be closed.', closeFailure);
  }
}

async function readRawLine(state, limits, signal) {
  const pieces = [];
  let bytes = 0;
  for (;;) {
    checkCancellation(signal);
    if (state.offset < state.chunk.length) {
      const newline = state.chunk.indexOf(0x0a, state.offset);
      const end = newline < 0 ? state.chunk.length : newline;
      const piece = state.chunk.subarray(state.offset, end);
      if (piece.length) {
        if (piece.length + bytes + NEWLINE.length > limits.recordBytes) {
          sortError('reference-artifact-v2-invalid', 'A metadata sort run contains an oversized record.');
        }
        pieces.push(Buffer.from(piece)); bytes += piece.length;
      }
      state.offset = newline < 0 ? state.chunk.length : newline + 1;
      if (newline >= 0) {
        if (bytes === 0 || pieces.at(-1)?.at(-1) === 0x0d) {
          sortError('reference-artifact-v2-invalid', 'A metadata sort run contains invalid line framing.');
        }
        return pieces.length === 1 ? pieces[0] : Buffer.concat(pieces, bytes);
      }
    }
    if (state.done) {
      if (bytes) sortError('reference-artifact-v2-invalid', 'A metadata sort run is missing its final newline.');
      return undefined;
    }
    const result = await awaitAbortable(state.handle.read(state.buffer, 0, state.buffer.length, null), signal);
    if (result.bytesRead === 0) {
      state.done = true; state.chunk = Buffer.alloc(0); state.offset = 0;
    } else {
      state.chunk = Buffer.from(state.buffer.subarray(0, result.bytesRead)); state.offset = 0;
    }
  }
}

async function closeRun(state) {
  if (state.closed) return;
  state.closed = true;
  await state.handle.close();
}

async function openRun(pathname, limits, signal, io) {
  const handle = await io.open(pathname, 'r');
  const state = {
    handle,
    buffer: Buffer.alloc(Math.max(1, Math.min(READ_CHUNK_BYTES, limits.recordBytes))),
    chunk: Buffer.alloc(0),
    offset: 0,
    done: false,
    closed: false,
    head: undefined,
    previous: undefined,
  };
  state.advance = async () => {
    const bytes = await readRawLine(state, limits, signal);
    if (!bytes) { state.head = undefined; return; }
    let value;
    try {
      value = parseJsonStrict(bytes, { maxBytes: limits.recordBytes, maxDepth: MAX_METADATA_RECORD_DEPTH + 1 });
    } catch (error) {
      sortError('reference-artifact-v2-invalid', 'A metadata sort run contains invalid JSON.', error);
    }
    const record = canonicalMetadataRecord(value);
    if (!canonicalJsonBytes({ key: record.key, record: record.record }).equals(bytes)) {
      sortError('reference-artifact-v2-invalid', 'A metadata sort run is not canonical.');
    }
    if (state.previous && Buffer.compare(state.previous, record.keyBytes) >= 0) {
      sortError('reference-artifact-v2-invalid', 'A metadata sort run is not strictly ordered.');
    }
    state.previous = record.keyBytes;
    state.head = { bytes, keyBytes: record.keyBytes };
  };
  try {
    await state.advance();
    return state;
  } catch (error) {
    let closeFailure;
    try { await closeRun(state); }
    catch (failure) { closeFailure = failure; }
    if (closeFailure) throw primaryWithSecondary(error, 'closeError', closeFailure);
    throw error;
  }
}

async function mergeRuns(inputs, output, limits, signal, io) {
  const states = [];
  let handle;
  let primary;
  try {
    for (const pathname of inputs) states.push(await openRun(pathname, limits, signal, io));
    handle = await io.open(output, 'wx', 0o600);
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
      await writeAll(handle, selected.head.bytes, signal);
      await writeAll(handle, NEWLINE, signal);
      previous = selected.head.keyBytes;
      await selected.advance();
    }
    await handle.sync();
  } catch (error) {
    primary = error;
  }
  const closeResults = await Promise.allSettled([
    ...(handle ? [handle.close()] : []),
    ...states.map(closeRun),
  ]);
  const closeFailures = closeResults
    .filter((result) => result.status === 'rejected')
    .map((result) => result.reason);
  const closeFailure = closeFailures.length > 1
    ? new AggregateError(closeFailures, 'Metadata sort handles could not be closed.')
    : closeFailures[0];
  if (primary) {
    if (closeFailure) throw primaryWithSecondary(primary, 'closeError', closeFailure);
    throw primary;
  }
  if (closeFailure) {
    sortError('reference-artifact-v2-io', 'A metadata sort run could not be closed.', closeFailure);
  }
}

function boundedWait(promise) {
  return new Promise((resolve) => {
    const timer = setTimeout(resolve, ITERATOR_FINALIZE_MS);
    Promise.resolve(promise).catch(() => undefined).then(() => {
      clearTimeout(timer); resolve();
    });
  });
}

async function* metadataInputs(records, syncIterator, asyncIterator, signal) {
  let iterator; let completed = false;
  try { iterator = (asyncIterator ?? syncIterator).call(records); }
  catch (error) { sortError('reference-artifact-v2-invalid', 'Metadata sort input is invalid.', error); }
  if (!iterator || typeof iterator.next !== 'function') {
    sortError('reference-artifact-v2-invalid', 'Metadata sort input is invalid.');
  }
  try {
    for (;;) {
      let next; let done; let value;
      try {
        checkCancellation(signal);
        next = await awaitAbortable(iterator.next(), signal);
        done = next?.done;
        value = next?.value;
      } catch (error) {
        if (error instanceof ModelReaderError) throw error;
        sortError('reference-artifact-v2-invalid', 'Metadata sort input is invalid.', error);
      }
      if (!next || typeof next !== 'object') {
        sortError('reference-artifact-v2-invalid', 'Metadata sort input is invalid.');
      }
      if (done) { completed = true; return; }
      yield value;
    }
  } finally {
    if (!completed) {
      try {
        const returned = iterator.return?.();
        if (returned) await boundedWait(returned);
      } catch {
        // The primary validation, cancellation, or I/O error remains authoritative.
      }
    }
  }
}

function primaryWithCleanup(error, cleanupError, root) {
  const primary = error instanceof ModelReaderError
    ? error
    : readerError('reference-artifact-v2-io', 'Metadata sorting failed.', error);
  if (!cleanupError) return primary;
  return new ModelReaderError(primary.code, primary.phase, primary.retryable, primary.message, {
    primary: primary.unsafeDetails ?? primary,
    cleanupError,
    leakedRoot: root,
  }, primary.providerCode);
}

function primaryWithSecondary(error, name, secondary) {
  const primary = error instanceof ModelReaderError
    ? error
    : readerError('reference-artifact-v2-io', 'Metadata sorting failed.', error);
  return new ModelReaderError(primary.code, primary.phase, primary.retryable, primary.message, {
    primary: primary.unsafeDetails ?? primary,
    [name]: secondary,
  }, primary.providerCode);
}

async function sortMetadataRecords(records, options, dependencies) {
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
  const io = dependencies.fs;
  let root;
  try {
    const realTempParent = await io.realpath(tempParent);
    root = await io.mkdtemp(path.join(realTempParent, 'aware-model-sort-'));
    let buffered = []; let bufferedBytes = 0; let totalBytes = 0; let count = 0; let ordinal = 0;
    const runs = [];
    const publishRun = async () => {
      if (runs.length >= limits.initialRuns) {
        sortError('reference-artifact-v2-limit', 'The metadata family exceeds its initial run limit.');
      }
      const pathname = path.join(root, `run-${String(ordinal).padStart(6, '0')}.jsonl`);
      await writeRun(buffered, pathname, signal, io);
      await dependencies.afterRunWritten?.(pathname, { kind: 'initial', ordinal });
      runs.push(pathname); ordinal += 1; buffered = []; bufferedBytes = 0;
    };
    for await (const input of metadataInputs(records, syncIterator, asyncIterator, signal)) {
      checkCancellation(signal);
      const record = encodedRecord(input, limits.recordBytes);
      const bytes = record.bytes.length + NEWLINE.length;
      if (bytes > limits.runBytes) sortError('reference-artifact-v2-limit', 'One metadata record exceeds the sort run limit.');
      if (bytes > limits.totalBytes - totalBytes) {
        sortError('reference-artifact-v2-limit', 'The metadata family exceeds its aggregate byte limit.');
      }
      if (buffered.length && bufferedBytes + bytes > limits.runBytes) await publishRun();
      buffered.push(record); bufferedBytes += bytes; totalBytes += bytes; count += 1;
      if (count > limits.records) sortError('reference-artifact-v2-limit', 'The metadata family exceeds its record limit.');
    }
    if (buffered.length) await publishRun();
    let pass = 0; let active = runs; let mergeCount = 0;
    while (active.length > 1) {
      const merged = [];
      for (let start = 0; start < active.length; start += limits.fanIn) {
        if (mergeCount >= limits.mergeRuns) {
          sortError('reference-artifact-v2-limit', 'The metadata family exceeds its merge run limit.');
        }
        const batch = active.slice(start, start + limits.fanIn);
        const pathname = path.join(root, `merge-${String(pass).padStart(3, '0')}-${String(merged.length).padStart(6, '0')}.jsonl`);
        await mergeRuns(batch, pathname, limits, signal, io);
        await dependencies.afterRunWritten?.(pathname, { kind: 'merge', ordinal: mergeCount });
        await Promise.all(batch.map((input) => io.rm(input, { force: true })));
        merged.push(pathname); mergeCount += 1;
      }
      active = merged; pass += 1;
    }
    const pathname = active[0] ?? path.join(root, 'empty.jsonl');
    if (!active.length) await io.writeFile(pathname, Buffer.alloc(0), { flag: 'wx', mode: 0o600 });
    checkCancellation(signal);
    return { root, pathname, count };
  } catch (error) {
    let cleanupError;
    if (root) {
      try { await io.rm(root, { recursive: true, force: true }); }
      catch (failure) { cleanupError = failure; }
    }
    throw primaryWithCleanup(error, cleanupError, root);
  }
}

const PRODUCTION_DEPENDENCIES = Object.freeze({ fs });

export function externalSortMetadataRecords(records, options = {}) {
  return sortMetadataRecords(records, options, PRODUCTION_DEPENDENCIES);
}

export function createExternalSortMetadataRecordsForTesting(dependencies = {}) {
  const selected = { fs: dependencies.fs ?? fs, afterRunWritten: dependencies.afterRunWritten };
  return (records, options = {}) => sortMetadataRecords(records, options, selected);
}
