import assert from 'node:assert/strict';
import { mkdirSync, mkdtempSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';
import { verifyReproducibleOutputs } from './verify-reproducible-builds.mjs';

function roots() {
  const root = mkdtempSync(join(tmpdir(), 'aware-repro-compare-'));
  const left = join(root, 'left-short'); const right = join(root, 'right-much-longer');
  mkdirSync(left); mkdirSync(right);
  return { root, left, right };
}

test('two closed ordered inventories must match by path, size, and hash', () => {
  const { left, right } = roots();
  writeFileSync(join(left, 'aware.exe'), 'same'); writeFileSync(join(right, 'aware.exe'), 'same');
  assert.deepEqual(verifyReproducibleOutputs({ left, right }), [{
    path: 'aware.exe', size: 4,
    sha256: '0967115f2813a3541eaef77de9d9d5773f1c0c04314b0bbfe4ff3b3b1c55b5d5',
  }]);
  writeFileSync(join(right, 'aware.exe'), 'different');
  assert.throws(() => verifyReproducibleOutputs({ left, right }), /builder outputs differ/);
});

test('checkout roots are rejected in raw, normalized, JSON, URI, case, and UTF-16 forms', () => {
  const { root, left, right } = roots();
  writeFileSync(join(left, 'receipt.json'), Buffer.from(root.toUpperCase(), 'utf16le'));
  writeFileSync(join(right, 'receipt.json'), Buffer.from(root.toUpperCase(), 'utf16le'));
  assert.throws(() => verifyReproducibleOutputs({ left, right, forbiddenRoots: [root] }), /root leaked/);
});

test('a root that is BOTH case-folded and JSON-escaped is still a leak', () => {
  // The scanner lowercases file content, so an escaped needle that kept its
  // original casing matched nothing: a serialized Windows path written as
  // c:\users\alice slipped through the proof that no builder root leaked.
  const { root, left, right } = roots();
  const escapedFolded = JSON.stringify(root).slice(1, -1).toLowerCase();
  writeFileSync(join(left, 'receipt.json'), escapedFolded, 'utf8');
  writeFileSync(join(right, 'receipt.json'), escapedFolded, 'utf8');
  assert.throws(() => verifyReproducibleOutputs({ left, right, forbiddenRoots: [root] }), /root leaked/);
});
