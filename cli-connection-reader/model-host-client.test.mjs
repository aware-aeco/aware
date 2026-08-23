import assert from 'node:assert/strict';
import { EventEmitter } from 'node:events';
import test from 'node:test';
import {
  HostFrameDecoder, ModelHostClient, encodeHostFrame, requireReadyClient, waitForCacheLock,
} from './model-host-client.mjs';

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

test('provider cancellation requested before acceptance is sent as soon as the run handle arrives', async () => {
  const child = new EventEmitter();
  child.stdout = new EventEmitter(); child.stderr = new EventEmitter(); child.stdin = { write: () => true, once: () => {}, end: () => {} };
  const client = new ModelHostClient(child);
  const controls = [];
  client.write = () => {};
  client.simple = async (body, handle) => { controls.push({ body, handle }); return { status: 'cancel-requested' }; };
  const controller = new AbortController();
  const request = {
    executable: 'provider', operation: 'convert', cwd: 'work', environment: {}, stdin: Buffer.alloc(0),
    timeoutMs: 1, stdoutLimit: 1, stderrLimit: 1, signal: controller.signal,
  };
  const result = client.run(request);
  controller.abort();
  const handle = Buffer.alloc(32, 7);
  client.onFrame({ kind: 1, requestId: 1n, runHandle: handle, sequence: 0, final: true, payload: Buffer.from('{"status":"accepted"}') });
  await new Promise((resolve) => setImmediate(resolve));
  assert.equal(controls.some((call) => call.body.op === 'provider-cancel' && call.handle.equals(handle)), true);
  client.onFrame({ kind: 2, requestId: 1n, runHandle: handle, sequence: 0, final: true, payload: Buffer.alloc(0) });
  client.onFrame({ kind: 3, requestId: 1n, runHandle: handle, sequence: 0, final: true, payload: Buffer.alloc(0) });
  client.onFrame({ kind: 1, requestId: 1n, runHandle: handle, sequence: 0, final: true, payload: Buffer.from('{"status":"complete","exitCode":130}') });
  await result;
});

test('a failed host handshake terminates the spawned client before rejecting', async () => {
  let terminated = false;
  const failure = new Error('incompatible');
  const client = {
    ready: async () => { throw failure; },
    terminate: async () => { terminated = true; },
  };
  await assert.rejects(() => requireReadyClient(client), failure);
  assert.equal(terminated, true);
});

test('forced host termination follows an ignored graceful signal and waits for confirmed exit', async () => {
  const child = new EventEmitter();
  child.stdout = new EventEmitter(); child.stderr = new EventEmitter();
  child.stdout.destroy = () => {}; child.stderr.destroy = () => {};
  child.stdin = { destroy: () => {} };
  child.exitCode = null;
  const signals = [];
  child.kill = (signal) => {
    signals.push(signal ?? 'SIGTERM');
    if (signal === 'SIGKILL') {
      child.exitCode = 137;
      queueMicrotask(() => child.emit('exit', child.exitCode));
    }
    return true;
  };
  const client = new ModelHostClient(child);
  await client.terminate({ gracefulTimeoutMs: 1 });
  assert.deepEqual(signals, ['SIGTERM', 'SIGKILL']);
  assert.equal(child.exitCode, 137);
});
