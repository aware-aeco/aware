import assert from 'node:assert/strict';
import test from 'node:test';
import { HostFrameDecoder, encodeHostFrame, waitForCacheLock } from './model-host-client.mjs';

test('host frames preserve request id, run handle, sequence, final flag, and binary payload across arbitrary chunks', () => {
  const expected = {
    kind: 2, requestId: 0x0102030405060708n, runHandle: Buffer.alloc(32, 0x7f),
    sequence: 9, final: true, payload: Buffer.from([0, 255, 1, 2]),
  };
  const bytes = encodeHostFrame(expected);
  const decoder = new HostFrameDecoder();
  assert.deepEqual(decoder.push(bytes.subarray(0, 17)), []);
  assert.deepEqual(decoder.push(bytes.subarray(17, 51)), []);
  const [actual] = decoder.push(bytes.subarray(51));
  assert.deepEqual(actual, expected);
});

test('frame decoder refuses unknown kinds and oversized payload declarations before allocation', () => {
  const unknown = encodeHostFrame({ kind: 1, requestId: 1n, runHandle: Buffer.alloc(32), sequence: 0, final: true, payload: Buffer.alloc(0) });
  unknown[0] = 9;
  assert.throws(() => new HostFrameDecoder().push(unknown), /kind/);
  const oversized = Buffer.alloc(50); oversized[0] = 1; oversized.writeUInt32BE(1024 * 1024 + 1, 46);
  assert.throws(() => new HostFrameDecoder().push(oversized), /limit/);
});

test('cache lock contention waits boundedly for the winner and honors cancellation', async () => {
  let attempts = 0;
  const handle = Buffer.alloc(32, 7);
  const acquired = await waitForCacheLock(async () => {
    attempts += 1;
    return attempts < 3 ? { body: { status: 'busy' }, handle: Buffer.alloc(32) } : { body: { status: 'acquired' }, handle };
  }, { delay: async () => {}, now: (() => { let value = 0; return () => value += 1; })(), timeoutMs: 10 });
  assert.deepEqual(acquired, handle);
  assert.equal(attempts, 3);

  const controller = new AbortController();
  controller.abort();
  await assert.rejects(
    () => waitForCacheLock(async () => ({ body: { status: 'busy' }, handle: Buffer.alloc(32) }), { signal: controller.signal }),
    (error) => error.code === 'reference-cancelled',
  );
});
