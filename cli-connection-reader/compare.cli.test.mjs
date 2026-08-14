// The two-file CLI path for `compare`, against real IFCs.
//
// `compare.test.mjs` covers the identity rules and the refusal with plain objects; this file covers the
// things only the process boundary can prove — that the dispatch accepts a command taking two files
// neither of which is called `ifc-path`, that the filter reaches `selectExpressIds` under the key it
// actually reads, and that the stdout guard still leaves the protocol channel clean on an error.
import test from 'node:test';
import assert from 'node:assert/strict';
import { execFileSync, spawnSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { join } from 'node:path';

const DOWNLOADS = join(process.env.USERPROFILE ?? process.env.HOME ?? '', 'Downloads');
const sample = (n) => (existsSync(join(DOWNLOADS, n)) ? join(DOWNLOADS, n) : null);
const run = (command, args) =>
  JSON.parse(execFileSync(process.execPath, ['index.mjs', command], { input: JSON.stringify(args), encoding: 'utf8' }));

test('compare of a file with ITSELF reports every object unchanged and refuses nothing', (t) => {
  const f = sample('example-steel-framing.ifc');
  if (!f) return t.skip('sample file not present — see the reference-objects design doc §8');
  const r = run('compare', { 'base-ifc-path': f, 'revised-ifc-path': f });
  assert.equal(r.identity.refused, undefined);
  assert.equal(r.summary.added, 0);
  assert.equal(r.summary.removed, 0);
  assert.equal(r.summary.changed, 0);
  assert.ok(r.summary.unchanged > 0);
  assert.equal(r.frame, 'z-up', 'the frame is STATED, never inferred from a version number');
  assert.deepEqual(r.criteria, ['location', 'geometry', 'ifcType', 'name', 'profile', 'material', 'properties']);
});

test('compare of two UNRELATED files refuses rather than reporting a wholesale replacement', (t) => {
  const a = sample('example-steel-framing.ifc'), b = sample('Building-Architecture.ifc');
  if (!a || !b) return t.skip('sample files not present — see the reference-objects design doc §8');
  const r = run('compare', { 'base-ifc-path': a, 'revised-ifc-path': b });
  assert.ok(r.identity.refused);
  assert.equal(r.changes, undefined);
});

test('a REAL revision pair pairs everything and reports exactly what changed', (t) => {
  // The Phase 0 pair: `Take 5.ifc` (Tekla 2025 SP6.1) with one beam relocated 750 mm and one column's
  // profile changed W14X43 -> W14X53. Ground truth is known exactly, which is what makes this the one
  // test here that can assert counts rather than only shapes. See the tolerance-measurement doc.
  const a = sample('reference-revision-pair/base.ifc'), b = sample('reference-revision-pair/revised.ifc');
  if (!a || !b) return t.skip('the Phase 0 revision pair is not present — see 2026-08-14-reference-tolerance-measurement.md §3.1');
  const r = run('compare', { 'base-ifc-path': a, 'revised-ifc-path': b });
  assert.equal(r.identity.refused, undefined);
  assert.equal(r.summary.added, 0, 'a revision of one model adds nothing when nothing was added');
  assert.equal(r.summary.removed, 0);
  assert.equal(r.summary.uncomparable, 0, 'not one blank or duplicated id in 3,232 objects');
  assert.equal(r.summary.changed, 5, 'one relocated beam + the FOUR columns sharing the edited profile');
  const moved = r.changes.filter((c) => c.changedBy?.includes('location'));
  assert.equal(moved.length, 1, 'exactly one object moved');
  assert.ok(Math.abs(moved[0].distance - 750) < 0.001, `moved 750 mm, got ${moved[0].distance}`);
  // The section change moved those centroids by 0.0588 mm — UNDER the 1 mm tolerance — so `location`
  // must stay quiet for them while `profile` and `geometry` fire. This is the discrimination the
  // tolerance exists for, and the pair is what makes it assertable.
  const reprofiled = r.changes.filter((c) => c.changedBy?.includes('profile'));
  assert.equal(reprofiled.length, 4);
  for (const c of reprofiled) {
    assert.ok(c.changedBy.includes('geometry'), 'a new section is a new shape');
    assert.ok(!c.changedBy.includes('location'), 'but it did not MOVE — 0.0588 mm is under tolerance');
  }
});

test('a filter applies to BOTH sides, and its receipt comes back per side', (t) => {
  const f = sample('Building-Architecture.ifc');
  if (!f) return t.skip('sample file not present');
  const r = run('compare', { 'base-ifc-path': f, 'revised-ifc-path': f, 'ifc-types': ['IFCWALL', 'IFCNONSENSE'] });
  assert.ok(r.selected.base.unmatched.some((u) => u.ifcType === 'IFCNONSENSE'));
  assert.ok(r.selected.revised.unmatched.some((u) => u.ifcType === 'IFCNONSENSE'));
});

test('the type filter is HONOURED, not silently ignored — a wrong opts key reads the whole building', (t) => {
  // The failure this pins: `selectExpressIds` reads `opts.ifcTypes`. Wire `'ifc-types'` straight through
  // and `given()` sees no filter at all, so a caller asking for the walls gets the entire model and a
  // success exit code. Asserting `selected.candidates` is what makes the difference visible — a count
  // alone would pass against the bug on a model that happens to be mostly walls.
  const f = sample('Building-Architecture.ifc');
  if (!f) return t.skip('sample file not present');
  const all = run('compare', { 'base-ifc-path': f, 'revised-ifc-path': f });
  const walls = run('compare', { 'base-ifc-path': f, 'revised-ifc-path': f, 'ifc-types': ['IFCWALL'] });
  // `candidates`, NOT `applied`. `selectExpressIds` returns `{ ids, applied, report }` and only `report`
  // reaches the output, so `selected.base.applied` is undefined — and `undefined !== false` is true, so
  // an assertion on it passes against the very bug it was written to catch.
  assert.ok(Number.isFinite(walls.selected.base.candidates), 'the receipt is present at all');
  assert.ok(walls.selected.base.candidates < all.identity.base.objects, 'and it genuinely narrowed the read');
  assert.ok(walls.summary.unchanged < all.summary.unchanged);
});

test('per-side filters let a NOT-like-for-like comparison run at all', (t) => {
  const f = sample('Building-Architecture.ifc');
  if (!f) return t.skip('sample file not present');
  const r = run('compare', { 'base-ifc-path': f, 'revised-ifc-path': f, 'base-ifc-types': ['IFCWALL'], 'revised-ifc-types': ['IFCWALL', 'IFCSLAB'] });
  assert.notDeepEqual(r.selected.base.ifcTypes, r.selected.revised.ifcTypes, 'the receipts differ, so a caller can BANNER the scope change');
});

test('a missing path is refused by name, not by a stack trace', () => {
  assert.throws(() => run('compare', { 'base-ifc-path': 'a.ifc' }), /revised-ifc-path/);
});

test('the OTHER commands still refuse a missing ifc-path — the guard moved, it did not go', () => {
  assert.throws(() => run('probe', {}), /ifc-path/);
});

test('an error path writes NOTHING to stdout and reports on stderr', () => {
  // WHAT THIS DOES AND DOES NOT PROVE: a subprocess exits immediately after throwing, so this cannot
  // observe whether `process.stdout.write` was restored — only where the bytes went. That is still the
  // property the protocol depends on (a consumer parses stdout as pure JSON and a half-written error
  // there is unparseable), so the test earns its place; it just is not a guard-restoration test.
  const out = spawnSync(process.execPath, ['index.mjs', 'probe'], { input: '{"ifc-path":"/nope/missing.ifc"}', encoding: 'utf8' });
  assert.notEqual(out.status, 0);
  assert.match(out.stderr, /missing\.ifc|ENOENT|could not/i, 'the error is reported');
  assert.equal(out.stdout.trim(), '', 'and stdout carries no half-written protocol output');
});

test('a non-string ifc-path is refused before the stdout guard goes up', () => {
  const out = spawnSync(process.execPath, ['index.mjs', 'probe'], { input: '{"ifc-path":42}', encoding: 'utf8' });
  assert.notEqual(out.status, 0);
  assert.equal(out.stdout.trim(), '');
});

// EVERY test above skips when its sample file is absent, and a file of skipped tests reports success.
// This is the tripwire: if NOTHING ran, say so loudly rather than shipping a green run that proved
// nothing about a two-file command.
test('the fixtures this suite needs are present at all', () => {
  assert.ok(sample('example-steel-framing.ifc'), 'no sample IFC found — every test above skipped, so this suite proved NOTHING. See the reference-objects design doc §8.');
});
