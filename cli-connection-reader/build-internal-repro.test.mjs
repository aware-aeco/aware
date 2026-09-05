import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { mkdtempSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';
import { buildConnectionReader, canonicalJson, READER_BUILD_SETTINGS } from './build.mjs';
import { rejectedAmbientKeys, verifyInternalInputs } from './build-internal-repro.mjs';

const sha = (bytes) => createHash('sha256').update(bytes).digest('hex');

test('SEA configuration is path-independent and every injection operand is relative to output cwd', () => {
  const text = readFileSync(new URL('./build.mjs', import.meta.url), 'utf8');
  assert.match(text, /main: 'bundle\.cjs'/);
  assert.match(text, /output: 'sea-prep\.blob'/);
  assert.match(text, /cwd: outputDir/);
  assert.match(text, /EXE_NAME, READER_BUILD_SETTINGS\.sea\.section, 'sea-prep\.blob'/);
  assert.match(text, /'--sentinel-fuse', READER_BUILD_SETTINGS\.sea\.sentinelFuse/);
  assert.doesNotMatch(text, /main:\s*join\(outputDir/);
  assert.doesNotMatch(text, /output:\s*join\(outputDir/);
});

test('canonical receipt JSON is independent of insertion order', () => {
  assert.equal(canonicalJson({ z: 1, a: { y: 2, x: 3 } }), canonicalJson({ a: { x: 3, y: 2 }, z: 1 }));
});

test('build cleanup refuses to own the reader source root', async () => {
  const readerRoot = new URL('.', import.meta.url).pathname.replace(/^\/(?:[A-Za-z]:)/, (value) => value.slice(1));
  await assert.rejects(() => buildConnectionReader({ outputDir: readerRoot }), /must not be the reader source root/);
});

test('ambient compiler, Node, npm, dotnet, and credential authority is rejected', () => {
  assert.deepEqual(rejectedAmbientKeys({ Path: 'ok', RUSTFLAGS: 'poison', npm_config_cache: 'x', DOTNET_ROOT: 'x' }),
    ['DOTNET_ROOT', 'RUSTFLAGS', 'npm_config_cache']);
  assert.deepEqual(rejectedAmbientKeys({ NODE_OPTIONS: '--require evil', AWARE_GOOGLE_CLIENT_SECRET: 'secret' }),
    ['AWARE_GOOGLE_CLIENT_SECRET', 'NODE_OPTIONS']);
});

test('ambient authority is rejected by folded case and by case-only collisions', () => {
  // Windows resolves process.env.ESBUILD_BINARY_PATH from an ambient
  // `esbuild_binary_path`, so a case-sensitive exact match let esbuild run an
  // undeclared binary inside a supposedly closed build.
  assert.deepEqual(rejectedAmbientKeys({ esbuild_binary_path: 'C:\evil.exe' }), ['esbuild_binary_path']);
  assert.deepEqual(rejectedAmbientKeys({ NoDe_OpTiOnS: '--require evil' }), ['NoDe_OpTiOnS']);
  // Two spellings of one name: which one a child reads is not ours to decide.
  assert.deepEqual(rejectedAmbientKeys({ Path: 'a', PATH: 'b' }), ['PATH', 'Path']);
  // A single ordinary Path is still authority the reader needs.
  assert.deepEqual(rejectedAmbientKeys({ Path: 'ok' }), []);
});

test('tool verification goes red when a declared tool byte changes', () => {
  const root = mkdtempSync(join(tmpdir(), 'aware-repro-test-'));
  const tool = join(root, 'tool.bin'); writeFileSync(tool, 'one');
  const manifest = {
    schema: 'aware-windows-repro-builder/v1', platform: 'win32', arch: 'x64', nodeVersion: '24.14.0',
    settings: READER_BUILD_SETTINGS,
    source: { bundleSha256: 'a'.repeat(64), commit: 'b'.repeat(40), tree: 'c'.repeat(40) },
    tools: {
      node: { id: 'node', sha256: sha('one') },
      postject: { id: 'postject', sha256: sha('one') },
      'web-ifc-wasm': { id: 'web-ifc-wasm', sha256: sha('one') },
    },
    inputs: { 'reader-package-lock': 'd'.repeat(64), 'aware-cargo-lock': 'e'.repeat(64) },
  };
  const locator = { schema: 'aware-windows-repro-locator/v1', tools: { node: tool, postject: tool, 'web-ifc-wasm': tool } };
  writeFileSync(tool, 'two');
  assert.throws(() => verifyInternalInputs({ manifest, locator, env: {} }), /tool digest mismatch/);
});

test('controlled reader rejects omitted or changed byte-affecting settings', () => {
  const root = mkdtempSync(join(tmpdir(), 'aware-repro-settings-'));
  const tool = join(root, 'tool.bin'); writeFileSync(tool, 'one');
  const manifest = {
    schema: 'aware-windows-repro-builder/v1', platform: 'win32', arch: 'x64', nodeVersion: '24.14.0',
    source: { bundleSha256: 'a'.repeat(64), commit: 'b'.repeat(40), tree: 'c'.repeat(40) },
    tools: Object.fromEntries(['node', 'postject', 'web-ifc-wasm'].map((id) => [id, { id, sha256: sha('one') }])),
    inputs: { 'reader-package-lock': 'd'.repeat(64), 'aware-cargo-lock': 'e'.repeat(64) },
  };
  const locator = { schema: 'aware-windows-repro-locator/v1', tools: { node: tool, postject: tool, 'web-ifc-wasm': tool } };
  assert.throws(() => verifyInternalInputs({ manifest, locator, env: {} }), /reader build settings differ/);
  assert.throws(() => verifyInternalInputs({
    manifest: { ...manifest, settings: { ...READER_BUILD_SETTINGS, sea: { ...READER_BUILD_SETTINGS.sea, section: 'evil' } } },
    locator, env: {},
  }), /reader build settings differ/);
});
