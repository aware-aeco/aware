import assert from 'node:assert/strict';
import { mkdtempSync, mkdirSync, rmSync, symlinkSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { tmpdir } from 'node:os';
import test from 'node:test';
import { READER_BUILD_SETTINGS } from '../cli-connection-reader/repro-settings.mjs';
import { createWindowsBuilderRecords } from './create-windows-internal-repro-inputs.mjs';
import { compilerFixture } from './windows-compiler-fixture.mjs';

function fixture() {
  const result = compilerFixture();
  writeFileSync(join(result.input.closures['npm-cache'], 'z'), 'z');
  writeFileSync(join(result.input.closures['npm-cache'], 'a'), 'a');
  return result;
}

test('builder input compiler keeps local paths out of the canonical manifest', () => {
  const f = fixture();
  try {
    const first = createWindowsBuilderRecords(f.input);
    const second = createWindowsBuilderRecords(f.input);
    assert.equal(first.manifestText, second.manifestText);
    assert.equal(first.buildId, second.buildId);
    assert.equal(first.manifest.closures['npm-cache'].files.map((record) => record.path).join(','), 'a,cache,z');
    assert.deepEqual(first.manifest.settings, READER_BUILD_SETTINGS);
    assert.ok(!first.manifestText.includes(f.root));
    assert.equal(JSON.parse(first.locatorText).sourceBundle, f.input.source.bundle);
  } finally { rmSync(f.root, { recursive: true, force: true }); }
});

test('builder input compiler rejects symlinks inside offline closures', { skip: process.platform === 'win32' }, () => {
  const f = fixture();
  try {
    symlinkSync(join(f.root, 'source.bundle'), join(f.input.closures['npm-cache'], 'link'));
    assert.throws(() => createWindowsBuilderRecords(f.input), /path-redirection/);
  } finally { rmSync(f.root, { recursive: true, force: true }); }
});
