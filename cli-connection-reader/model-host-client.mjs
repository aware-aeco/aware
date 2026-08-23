import { spawn } from 'node:child_process';
import fs from 'node:fs/promises';
import path from 'node:path';
import { canonicalJsonBytes, ModelReaderError } from './model-contract.mjs';

const HEADER_BYTES = 50;
const MAX_PAYLOAD_BYTES = 1024 * 1024;
const ZERO_HANDLE = Buffer.alloc(32);

function hostError(code, message, details = undefined) {
  throw new ModelReaderError(code, 'provider-host', false, message, details);
}

export async function waitForCacheLock(acquire, options = {}) {
  const timeoutMs = options.timeoutMs ?? 30_000;
  const now = options.now ?? Date.now;
  const delay = options.delay ?? ((milliseconds) => new Promise((resolve) => setTimeout(resolve, milliseconds)));
  const started = now();
  while (true) {
    if (options.signal?.aborted) throw new ModelReaderError('reference-cancelled', 'cache', false, 'Cache lock waiting was cancelled.');
    const response = await acquire();
    if (response?.body?.status === 'acquired' && Buffer.isBuffer(response.handle) && !response.handle.equals(ZERO_HANDLE)) return response.handle;
    if (response?.body?.status !== 'busy') hostError('reference-provider-host-protocol', 'The managed cache fence returned an invalid response.');
    if (now() - started >= timeoutMs) throw new ModelReaderError('reference-cache-owned', 'cache', true, 'Timed out waiting for another model conversion.');
    await delay(25);
  }
}

export function encodeHostFrame({ kind, requestId, runHandle, sequence, final, payload }) {
  if (![1, 2, 3, 4].includes(kind)) throw new TypeError('unknown host frame kind');
  if (typeof requestId !== 'bigint' || requestId <= 0n || requestId > 0xffffffffffffffffn) throw new TypeError('invalid host frame request id');
  if (!Buffer.isBuffer(runHandle) || runHandle.length !== 32) throw new TypeError('invalid host frame run handle');
  if (!Number.isSafeInteger(sequence) || sequence < 0 || sequence > 0xffffffff) throw new TypeError('invalid host frame sequence');
  if (!Buffer.isBuffer(payload) || payload.length > MAX_PAYLOAD_BYTES) throw new TypeError('host frame payload exceeds limit');
  const bytes = Buffer.alloc(HEADER_BYTES + payload.length);
  bytes[0] = kind; bytes.writeBigUInt64BE(requestId, 1); runHandle.copy(bytes, 9);
  bytes.writeUInt32BE(sequence, 41); bytes[45] = final ? 1 : 0; bytes.writeUInt32BE(payload.length, 46);
  payload.copy(bytes, HEADER_BYTES); return bytes;
}

export class HostFrameDecoder {
  #buffer = Buffer.alloc(0);
  push(chunk) {
    if (!Buffer.isBuffer(chunk)) chunk = Buffer.from(chunk);
    this.#buffer = Buffer.concat([this.#buffer, chunk]);
    const frames = [];
    while (this.#buffer.length >= HEADER_BYTES) {
      const kind = this.#buffer[0];
      if (![1, 2, 3, 4].includes(kind)) throw new Error('unknown host frame kind');
      const length = this.#buffer.readUInt32BE(46);
      if (length > MAX_PAYLOAD_BYTES) throw new Error('host frame payload exceeds limit');
      if (this.#buffer.length < HEADER_BYTES + length) break;
      frames.push({
        kind, requestId: this.#buffer.readBigUInt64BE(1), runHandle: Buffer.from(this.#buffer.subarray(9, 41)),
        sequence: this.#buffer.readUInt32BE(41), final: (this.#buffer[45] & 1) !== 0,
        payload: Buffer.from(this.#buffer.subarray(HEADER_BYTES, HEADER_BYTES + length)),
      });
      this.#buffer = this.#buffer.subarray(HEADER_BYTES + length);
    }
    return frames;
  }
}

export class ModelHostClient {
  constructor(child) {
    this.child = child; this.nextRequestId = 1n; this.pending = new Map(); this.decoder = new HostFrameDecoder(); this.closed = false;
    this.stderr = Buffer.alloc(0);
    child.stdout.on('data', (chunk) => {
      try { for (const frame of this.decoder.push(chunk)) this.onFrame(frame); }
      catch (error) { this.failAll(error); }
    });
    child.stderr.on('data', (chunk) => { this.stderr = Buffer.concat([this.stderr, chunk]).subarray(-8192); });
    child.on('error', (error) => this.failAll(error));
    child.on('exit', (code) => { if (!this.closed) this.failAll(new Error(`model-reader host exited ${code}`)); });
  }

  write(frame) {
    if (!this.child.stdin.write(encodeHostFrame(frame))) {
      this.child.stdin.once('drain', () => {});
    }
  }

  control(body, runHandle = ZERO_HANDLE) {
    const requestId = this.nextRequestId++;
    return { requestId, frame: { kind: 1, requestId, runHandle, sequence: 0, final: true, payload: canonicalJsonBytes(body) } };
  }

  simple(body, runHandle = ZERO_HANDLE, captureHandle = false) {
    const { requestId, frame } = this.control(body, runHandle);
    const promise = new Promise((resolve, reject) => this.pending.set(requestId.toString(), { type: 'simple', resolve, reject, captureHandle }));
    this.write(frame); return promise;
  }

  acquireLock = async (lockPath, options = {}) => await waitForCacheLock(
    async () => await this.simple({ op: 'lock-acquire', path: lockPath }, ZERO_HANDLE, true),
    options,
  );

  releaseLock = async (handle) => {
    const response = await this.simple({ op: 'lock-release' }, handle);
    if (response.status !== 'released') hostError('reference-cache-owner-token', 'The cache fence could not be released.');
  };

  async ready() {
    const hello = await this.simple({ op: 'hello' });
    if (hello.protocol !== 'model-reader-host/v1' || typeof hello.build !== 'string') hostError('reference-provider-host-protocol', 'The managed provider host has an incompatible protocol.');
  }

  run = async (request) => {
    const { requestId, frame } = this.control({
      op: 'provider-run', executable: request.executable, operation: request.operation,
      cwd: request.cwd, environment: request.environment, stdinLength: request.stdin.length,
      timeoutMs: request.timeoutMs, stdoutLimit: request.stdoutLimit, stderrLimit: request.stderrLimit,
    });
    const promise = new Promise((resolve, reject) => this.pending.set(requestId.toString(), {
      type: 'run', request, resolve, reject, handle: null, cancelRequested: false, abortHandler: null,
      stdout: [], stderr: [], stdoutSequence: 0, stderrSequence: 0, stdoutFinal: false, stderrFinal: false,
    }));
    this.write(frame);
    if (request.signal) {
      const cancel = () => {
        const state = this.pending.get(requestId.toString());
        if (state) state.cancelRequested = true;
        if (state?.handle) void this.simple({ op: 'provider-cancel' }, state.handle).catch(() => {});
      };
      const state = this.pending.get(requestId.toString());
      if (state) state.abortHandler = cancel;
      if (request.signal.aborted) cancel(); else request.signal.addEventListener('abort', cancel, { once: true });
    }
    return await promise;
  };

  onFrame(frame) {
    const state = this.pending.get(frame.requestId.toString());
    if (!state) throw new Error('uncorrelated model-reader host frame');
    if (state.type === 'simple') {
      if (frame.kind !== 1 || !frame.final || frame.sequence !== 0) throw new Error('invalid model-reader host control response');
      this.pending.delete(frame.requestId.toString());
      const body = JSON.parse(frame.payload.toString('utf8'));
      state.resolve(state.captureHandle ? { body, handle: frame.runHandle } : body); return;
    }
    if (frame.kind === 1) {
      const control = JSON.parse(frame.payload.toString('utf8'));
      if (control.status === 'accepted') {
        if (state.handle || frame.runHandle.equals(ZERO_HANDLE)) throw new Error('invalid model-reader host run acceptance');
        state.handle = frame.runHandle;
        this.write({ kind: 4, requestId: frame.requestId, runHandle: state.handle, sequence: 0, final: true, payload: state.request.stdin });
        if (state.cancelRequested) void this.simple({ op: 'provider-cancel' }, state.handle).catch(() => {});
        return;
      }
      if (control.status === 'complete') {
        if (!state.handle || !frame.runHandle.equals(state.handle) || !state.stdoutFinal || !state.stderrFinal) throw new Error('model-reader host completed before correlated streams');
        this.pending.delete(frame.requestId.toString());
        if (state.abortHandler) state.request.signal?.removeEventListener('abort', state.abortHandler);
        if (control.hostErrorCode) {
          state.reject(new ModelReaderError(control.hostErrorCode, 'provider-host', false, 'The managed provider stream failed its bounded read.'));
          return;
        }
        state.resolve({ exitCode: control.exitCode, stdout: Buffer.concat(state.stdout), stderr: Buffer.concat(state.stderr) }); return;
      }
      throw new Error('unknown model-reader host run control');
    }
    if (!state.handle || !frame.runHandle.equals(state.handle) || !frame.final) throw new Error('uncorrelated or unterminated model-reader host stream');
    if (frame.kind === 2) {
      if (frame.sequence !== state.stdoutSequence++) throw new Error('model-reader host stdout sequence mismatch');
      state.stdout.push(frame.payload); state.stdoutFinal = true;
    } else if (frame.kind === 3) {
      if (frame.sequence !== state.stderrSequence++) throw new Error('model-reader host stderr sequence mismatch');
      state.stderr.push(frame.payload); state.stderrFinal = true;
    } else throw new Error('unexpected model-reader host frame');
  }

  failAll(error) {
    for (const state of this.pending.values()) {
      if (state.abortHandler) state.request.signal?.removeEventListener('abort', state.abortHandler);
      state.reject(new ModelReaderError('reference-provider-host-failed', 'provider-host', true, 'The managed provider host failed.', error));
    }
    this.pending.clear();
  }

  async close() {
    if (this.closed) return;
    try { await this.simple({ op: 'shutdown' }); } catch { /* host death already rejects callers */ }
    this.closed = true; this.child.stdin.end();
  }

  async terminate(options = {}) {
    if (this.closed) return;
    this.closed = true;
    if (this.child.exitCode !== null && this.child.exitCode !== undefined) return;
    const exited = new Promise((resolve) => {
      const finish = () => resolve();
      this.child.once('exit', finish); this.child.once('close', finish);
    });
    this.child.stdin.destroy?.();
    this.child.kill?.('SIGTERM');
    let gracefulTimer;
    const graceful = await Promise.race([
      exited.then(() => true),
      new Promise((resolve) => { gracefulTimer = setTimeout(() => resolve(false), options.gracefulTimeoutMs ?? 1000); }),
    ]);
    clearTimeout(gracefulTimer);
    if (!graceful && (this.child.exitCode === null || this.child.exitCode === undefined)) this.child.kill?.('SIGKILL');
    await exited;
    this.child.stdout.destroy?.(); this.child.stderr.destroy?.();
  }
}

export async function requireReadyClient(client) {
  try { await client.ready(); return client; }
  catch (error) { await client.terminate(); throw error; }
}

export async function createModelHostClient(hostPath, options = {}) {
  if (typeof hostPath !== 'string' || !path.isAbsolute(hostPath)) hostError('reference-provider-host-unavailable', 'The managed provider host path is unavailable.');
  let stat; let real;
  try { stat = await fs.lstat(hostPath); real = await fs.realpath(hostPath); }
  catch (error) { hostError('reference-provider-host-unavailable', 'The managed provider host is unavailable.', error); }
  if (!stat.isFile() || stat.isSymbolicLink() || path.resolve(real).toLowerCase() !== path.resolve(hostPath).toLowerCase()) hostError('reference-provider-host-unavailable', 'The managed provider host path is unsafe.');
  const child = spawn(hostPath, ['__model-reader-host'], { shell: false, windowsHide: true, stdio: ['pipe', 'pipe', 'pipe'], env: options.environment ?? process.env });
  return await requireReadyClient(new ModelHostClient(child));
}
