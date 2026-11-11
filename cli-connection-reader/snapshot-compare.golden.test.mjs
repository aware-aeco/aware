// compare-snapshots must not change when its pairing is shared with other commands. The files in
// test-fixtures/compare-golden/ are the canonical output of the ORIGINAL snapshot-compare.mjs (origin/main
// 1a8bcf000, before the pairing was exported for check-snapshots), for inputs in snapshot-compare-golden-inputs.mjs:
// a plain change set with a witness, a capped one, a byte-budgeted one, ambiguous fields, blank/duplicate
// identities on both sides, a discontinuity and placement fields with tolerances. The current code must write
// exactly those bytes.
import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync, readdirSync } from 'node:fs';
import { canonicalJson, compareSnapshots } from './snapshot-compare.mjs';
import { CASES } from './snapshot-compare-golden-inputs.mjs';

const dir = new URL('./test-fixtures/compare-golden/', import.meta.url);

test('every case has a golden file and every golden file has a case', () => {
  assert.deepEqual(readdirSync(dir).map((f) => f.replace(/\.json$/, '')).sort(), Object.keys(CASES).sort());
});

for (const [name, c] of Object.entries(CASES)) {
  test(`compare-snapshots output is byte-identical to the pre-refactor golden: ${name}`, () => {
    const out = compareSnapshots({ base: c.base, revised: c.revised, profile: c.profile, witness: c.witness ?? null, digests: c.digests ?? {} });
    assert.equal(canonicalJson(out), readFileSync(new URL(`${name}.json`, dir), 'utf8'));
  });
}

test('the goldens exercise what they claim to (a capped list, ambiguity, exclusions, a witness)', () => {
  const golden = (name) => JSON.parse(readFileSync(new URL(`${name}.json`, dir), 'utf8'));
  assert.ok(golden('capped').truncated.omitted.added > 0 && golden('capped').witness.entries === 3);
  assert.ok(golden('bytebudget').truncated.omitted.changed > 0);
  assert.equal(golden('ambiguous').ambiguousFields.count, 1);
  const ids = golden('identities').identity.excluded;
  assert.ok(ids.blank.revised.count > 0 && ids.duplicate.base.count > 0 && ids.duplicate.revised.count > 0);
  assert.equal(golden('discontinuity').identity.discontinuity, true);
  assert.equal(golden('basic').witness.agrees.count > 0, true);
});
