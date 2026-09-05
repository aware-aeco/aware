import assert from 'node:assert/strict';
import { mkdirSync, mkdtempSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import test from 'node:test';
import { resolve } from 'node:path';
import { forbiddenEncodings, verifyReproducibleOutputs } from './verify-reproducible-builds.mjs';

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

test('a root that is BOTH case-folded and JSON-escaped is a needle on every platform', () => {
  // Derived from a SYNTHETIC Windows root, not from tmpdir(): this suite runs on
  // ubuntu in CI, where a real temp path has no backslashes, so
  // JSON.stringify(root) is the identity and the needle collapses to
  // root.toLowerCase() -- which the unfixed code already emitted. The literal
  // backslashes below survive resolve() on POSIX as ordinary characters, so the
  // fold-then-escape combination is genuinely absent before the fix everywhere.
  const root = 'C:\\Users\\Alice\\src\\aware';
  const labels = new Set(forbiddenEncodings(root).map((entry) => entry.label));
  const escapedFolded = JSON.stringify(resolve(root).toLowerCase()).slice(1, -1);
  assert.ok(labels.has(escapedFolded), `missing lowercased JSON-escaped needle: ${escapedFolded}`);
  // The separately-generated forms must still be there.
  assert.ok(labels.has(resolve(root).toLowerCase()), 'missing folded needle');
  assert.ok(labels.has(JSON.stringify(resolve(root)).slice(1, -1)), 'missing escaped needle');
});
