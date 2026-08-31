import assert from 'node:assert/strict';
import { mkdtempSync, mkdirSync, rmSync, symlinkSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { tmpdir } from 'node:os';
import test from 'node:test';
import { createWindowsBuilderRecords } from './create-windows-internal-repro-inputs.mjs';

const toolIds = ['git', 'node', 'npm-cli', 'cargo', 'rustc', 'rustdoc', 'cl', 'link', 'lib', 'postject', 'web-ifc-wasm'];

function fixture() {
  const root = mkdtempSync(join(tmpdir(), 'aware-builder-inputs-'));
  const file = (name, text = name) => { const path = join(root, name); writeFileSync(path, text); return path; };
  const tools = Object.fromEntries(toolIds.map((id) => [id, file(`tool-${id}`)]));
  const npm = join(root, 'npm'); const cargo = join(root, 'cargo'); mkdirSync(npm); mkdirSync(cargo);
  writeFileSync(join(npm, 'z'), 'z'); writeFileSync(join(npm, 'a'), 'a'); writeFileSync(join(cargo, 'crate'), 'crate');
  return { root, input: {
    schema: 'aware-windows-repro-builder-inputs/v1',
    source: { commit: 'a'.repeat(40), tree: 'b'.repeat(40), bundle: file('source.bundle') },
    inputs: {
      'aware-cargo-lock': file('Cargo.lock'), 'reader-package-lock': file('package-lock.json'),
      'builder-script': file('build-windows-internal-repro.mjs'),
    },
    tools, closures: { 'npm-cache': npm, 'cargo-home': cargo },
    environment: Object.fromEntries(['PATH', 'INCLUDE', 'LIB', 'LIBPATH', 'SystemRoot', 'WINDIR', 'ComSpec', 'PATHEXT'].map((key) => [key, key])),
  } };
}

test('builder input compiler keeps local paths out of the canonical manifest', () => {
  const f = fixture();
  try {
    const first = createWindowsBuilderRecords(f.input);
    const second = createWindowsBuilderRecords(f.input);
    assert.equal(first.manifestText, second.manifestText);
    assert.equal(first.buildId, second.buildId);
    assert.equal(first.manifest.closures['npm-cache'].files.map((record) => record.path).join(','), 'a,z');
    assert.ok(!first.manifestText.includes(f.root));
    assert.equal(JSON.parse(first.locatorText).sourceBundle, f.input.source.bundle);
  } finally { rmSync(f.root, { recursive: true, force: true }); }
});

test('builder input compiler rejects symlinks inside offline closures', { skip: process.platform === 'win32' }, () => {
  const f = fixture();
  try {
    symlinkSync(join(f.root, 'source.bundle'), join(f.input.closures['npm-cache'], 'link'));
    assert.throws(() => createWindowsBuilderRecords(f.input), /symbolic link/);
  } finally { rmSync(f.root, { recursive: true, force: true }); }
});
