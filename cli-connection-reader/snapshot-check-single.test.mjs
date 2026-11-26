// Tests for `check-snapshots` single mode (model-change-set 0.5.0; clash on one state is snapshot-clash-single.test.mjs, 0.6.0): ONE saved state judged on its own.
// Synthetic documents in an invented vocabulary; see snapshot-check-single-fixtures.mjs.
import test from 'node:test';
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { canonicalJson, SnapshotCompareError } from './snapshot-compare.mjs';
import {
  checkSnapshots, checkSnapshotsFromFiles, CHECK_PROFILE_SCHEMA, EVALUATOR_VERSION, FINDINGS_SCHEMA_V2,
} from './snapshot-check.mjs';
import {
  ALL_RULES, coverageOf, grade, GRADE, GUID_KEY, id, LENGTH, PROFILE, richObjects, richRelationships, rule, side, V2, writeSide,
} from './snapshot-check-single-fixtures.mjs';

const here = new URL('.', import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, '$1');
const sha = (text) => createHash('sha256').update(text, 'utf8').digest('hex');
const single = (revised, checkProfile, extra = {}) => checkSnapshots({ base: null, revised, profile: PROFILE, checkProfile, mode: 'single', ...extra });
const pair = (base, revised, checkProfile, extra = {}) => checkSnapshots({ base, revised, profile: PROFILE, checkProfile, ...extra });
const refusesWith = (fn, pattern, code) => assert.throws(
  fn, (error) => error instanceof SnapshotCompareError && (code === undefined || error.code === code) && pattern.test(error.message), String(pattern),
);

// What a finding says, without what only a comparison can add.
const stripped = (finding) => {
  const { presence: _p, baseEvidenceFingerprint: _b, participants, ...rest } = finding;
  return { ...rest, participants: participants.map(({ entityIdBase: _e, ...p }) => p) };
};

// ─── the same verdict as pair mode's later state ────────────────────────────────────────────────────

const CASES = [
  ['blank', rule('blank', undefined), (o) => o, {}],
  ['property-missing', rule('property-missing', { key: GRADE }), (o) => o, {}],
  ['property-not-in exact', rule('property-not-in', { key: GRADE, values: ['S355', 'S275'], match: 'exact' }), (o) => o, {}],
  ['property-not-in nfkc-casefold', rule('property-not-in', { key: GRADE, values: ['s355'], match: 'nfkc-casefold' }), (o) => o, {}],
  ['property-out-of-range', rule('property-out-of-range', { key: LENGTH, min: 100, max: 500, unit: 'mm' }), (o) => o, {}],
  ['relationship-missing to', rule('relationship-missing', { kind: 'hosts', end: 'to' }), (o) => o, {}],
  ['relationship-missing from', rule('relationship-missing', { kind: 'hosts', end: 'from' }, { appliesTo: { types: ['Beam', 'Host'] } }), (o) => o, {}],
  ['property-missing, source did not read the key', rule('property-missing', { key: GRADE }), (o) => o, { read: ['native:length'] }],
  ['property-out-of-range, unit and value problems', rule('property-out-of-range', { key: LENGTH, min: 100, unit: 'mm' }), (o) => o, {}],
  ['relationship-missing, kind not complete', rule('relationship-missing', { kind: 'hosts', end: 'to' }), (o) => o, { kinds: [] }],
];

for (const [name, r, shape, coverageOpts] of CASES) {
  test(`single mode judges every object as pair mode's later state does: ${name}`, () => {
    const objects = shape(richObjects());
    const revised = side(objects, richRelationships());
    const coverage = { base: null, revised: coverageOf(objects, coverageOpts) };
    const checkProfile = V2([r]);
    const one = single(revised, checkProfile, { coverage });
    // Pair mode against an EMPTY earlier state: every object is added, and only the later side is judged.
    const emptyBase = pair(side([]), revised, checkProfile, { coverage: { base: coverageOf([]), revised: coverage.revised } });
    assert.deepEqual(one.findings.map(stripped), emptyBase.findings.map(stripped), 'same findings, same evidence, same fingerprints');
    assert.deepEqual(one.scope.byRule[r.id].notEvaluated, emptyBase.scope.byRule[r.id].notEvaluated, 'same notEvaluated reasons');
    for (const key of ['evaluated', 'findings', 'notApplicable', 'evaluatedScope']) assert.equal(one.scope.byRule[r.id][key], emptyBase.scope.byRule[r.id][key], key);
    assert.equal(one.scope.byRule[r.id].coverage.revisedComplete, emptyBase.scope.byRule[r.id].coverage.revisedComplete);
    assert.equal(one.scope.byRule[r.id].coverage.baseComplete, null);
    assert.equal(one.scope.evaluated, emptyBase.scope.evaluated);
    assert.equal(one.scope.notApplicable, emptyBase.scope.notApplicable);
    assert.equal(one.scope.excluded, emptyBase.scope.excluded);
    assert.deepEqual(one.unchecked.filter((line) => line.startsWith('Rule ')), emptyBase.unchecked.filter((line) => line.startsWith('Rule ')));
    // Pair mode against the same state as the earlier one: the same findings, all pre-existing.
    const same = pair(revised, revised, checkProfile, { coverage: { base: coverage.revised, revised: coverage.revised } });
    assert.deepEqual(same.findings.map(stripped), one.findings.map(stripped));
    assert.ok(same.findings.every((f) => f.presence === 'pre-existing'), 'identical states in pair mode stay pre-existing');
    assert.equal(same.summary.introduced, 0);
    assert.equal(same.mode, undefined, 'pair mode never carries a mode field');
  });
}

test('the whole rich profile: every predicate fires or is labelled, in both modes alike', () => {
  const objects = richObjects();
  const revised = side(objects, richRelationships());
  const coverage = coverageOf(objects);
  const one = single(revised, V2(ALL_RULES()), { coverage: { base: null, revised: coverage } });
  const two = pair(side([]), revised, V2(ALL_RULES()), { coverage: { base: coverageOf([]), revised: coverage } });
  assert.deepEqual(one.findings.map(stripped), two.findings.map(stripped));
  const byRule = one.summary.byRule;
  for (const ruleId of ['firm.blank', 'firm.property-missing', 'firm.property-not-in', 'firm.property-out-of-range', 'firm.relationship-missing']) {
    assert.ok(byRule[ruleId] >= 1, `${ruleId} found something`);
  }
  assert.deepEqual(one.scope.byRule['firm.property-out-of-range'].notEvaluated, { 'unit-mismatch': 1, 'value-not-numeric': 1, 'value-missing': 1 });
});

test('a property rule on the identity key is not evaluated in single mode, as in pair mode', () => {
  const profile = { ...PROFILE, keyRules: [...PROFILE.keyRules, { match: '^native:guid$', key: 'identity:{name|nfkc-lower}' }] };
  const objects = [{ guid: 'G1', guidName: 'Stable Id', props: [] }, { guid: 'G2', guidName: 'Stable Id', props: [grade('S355')] }];
  const r = rule('property-missing', { key: 'identity:stable id' });
  const coverage = coverageOf(objects, { read: ['native:*', 'identity:*'] });
  const one = checkSnapshots({ base: null, revised: side(objects), profile, checkProfile: V2([r]), coverage: { base: null, revised: coverage }, mode: 'single' });
  assert.deepEqual(one.scope.byRule[r.id].notEvaluated, { 'key-is-identity': 2 });
  assert.equal(one.findings.length, 0);
  const two = checkSnapshots({ base: side([]), revised: side(objects), profile, checkProfile: V2([r]), coverage: { base: coverageOf([]), revised: coverage } });
  assert.deepEqual(two.scope.byRule[r.id].notEvaluated, { 'key-is-identity': 2 });
});

test('a key that was an identity key only in the earlier state is judged on the later state as single mode judges it', () => {
  // Each state is judged with its own identity keys, so a version gets the same verdict on its own and as a pair's later state.
  const profile = { ...PROFILE, keyRules: [...PROFILE.keyRules, { match: '^native:guid$', key: 'identity:{name|nfkc-lower}' }] };
  const before = [{ guid: 'G1', guidName: 'Old Id', props: [] }];
  const after = [{ guid: 'G1', guidName: 'New Id', props: [] }];
  const r = rule('property-missing', { key: 'identity:old id' });
  const read = { read: ['native:*', 'identity:*'] };
  const one = checkSnapshots({ base: null, revised: side(after), profile, checkProfile: V2([r]), coverage: { base: null, revised: coverageOf(after, read) }, mode: 'single' });
  assert.equal(one.findings.length, 1, 'the later state has no identity:old id, so the key is judged and missing');
  const two = checkSnapshots({ base: side(before), revised: side(after), profile, checkProfile: V2([r]), coverage: { base: coverageOf(before, read), revised: coverageOf(after, read) } });
  assert.deepEqual(two.findings.map(stripped), one.findings.map(stripped), 'the pair judges its later state as single mode does');
  assert.equal(two.scope.byRule[r.id].notEvaluated['key-is-identity'] ?? 0, 0, 'an identity key of the earlier state does not stop the later state being judged');
});

test('an identity used twice or blank in the version is excluded and counted, never judged', () => {
  const objects = [{ guid: 'G1', props: [] }, { guid: 'D1', props: [] }, { guid: 'D1', props: [grade('S355')] }, { props: [] }];
  const r = rule('property-missing', { key: GRADE });
  const out = single(side(objects), V2([r]), { coverage: { base: null, revised: coverageOf(objects) } });
  assert.equal(out.scope.excluded, 3);
  assert.deepEqual(out.scope.byRule[r.id].notEvaluated, { 'identity-unusable': 3 });
  assert.deepEqual(out.findings.map((f) => f.participants[0].identity), [id('G1')]);
  assert.equal(out.scope.byRule[r.id].coverage.revisedComplete, false);
  assert.ok(out.unchecked.some((line) => /3 object\(s\) in this version were not checked/.test(line)));
});

// ─── the shape of the answer ────────────────────────────────────────────────────────────────────────

test('single mode output: the documented shape, with nothing that only a comparison could say', () => {
  const objects = [{ guid: 'G1', props: [] }, { guid: 'G2', props: [grade('S355')] }];
  const r = rule('property-missing', { key: GRADE });
  const checkProfile = V2([r]);
  const coverage = coverageOf(objects);
  const out = single(side(objects), checkProfile, { coverage: { base: null, revised: coverage }, digests: { revised: { entities: 'e', properties: 'p', relationships: 'r' } } });
  assert.equal(out.schema, FINDINGS_SCHEMA_V2);
  assert.equal(out.mode, 'single');
  assert.equal(out.inputs.base, null);
  assert.deepEqual(out.inputs.revised, { entities: 'e', properties: 'p', relationships: 'r' });
  assert.equal(out.inputs.focusDigest, null);
  assert.deepEqual(out.coverageSha256, { base: null, revised: null });
  assert.deepEqual([out.scope.added, out.scope.changed, out.scope.focus, out.scope.mode, out.scope.objects], [0, 0, 0, 'all', 2]);
  assert.equal(out.scope.byRule[r.id].coverage.baseComplete, null);
  assert.deepEqual(out.resolvedInChange, []);
  assert.deepEqual(out.focusResults, []);
  assert.deepEqual(out.summary, { findings: 1, present: 1, byRule: { [r.id]: 1 } });
  const [finding] = out.findings;
  assert.equal(finding.presence, 'present');
  assert.equal(finding.baseEvidenceFingerprint, null);
  assert.equal(finding.participants[0].entityIdBase, null);
  assert.equal(finding.participants[0].entityIdRevised, 'element:1');
  assert.equal(out.truncated, null);
  assert.equal(out.complete, true);
  assert.deepEqual(out.evaluator, { agent: 'model-change-set', version: EVALUATOR_VERSION, profileSha256: sha(canonicalJson(checkProfile)) });
  assert.equal(EVALUATOR_VERSION, '0.7.0');
  assert.ok(out.unchecked[0].startsWith('Only this model version was checked'));
  assert.ok(out.unchecked.some((line) => /Clash checks were not run/.test(line)));
  assert.equal(out.unchecked.some((line) => /unchanged|added and changed/.test(line)), false, 'no pair-mode scope sentence');
  for (const gone of ['introduced', 'preExisting', 'unknown', 'resolvedInChange', 'unknownPresence']) assert.equal(gone in out.summary, false, gone);
  assert.equal(canonicalJson(JSON.parse(canonicalJson(out))), canonicalJson(out));
});

test('truncation: counts are complete, the list is capped, and present === findings === listed + omitted', () => {
  const objects = Array.from({ length: 7 }, (_, i) => ({ guid: `G${i + 1}`, props: [] }));
  const r = rule('property-missing', { key: GRADE });
  const out = single(side(objects), V2([r], { budgets: { findings: 3, focusResults: 5 } }), { coverage: { base: null, revised: coverageOf(objects) } });
  assert.equal(out.findings.length, 3);
  assert.equal(out.summary.findings, 7);
  assert.equal(out.summary.present, 7);
  assert.equal(out.summary.byRule[r.id], 7);
  assert.equal(out.scope.byRule[r.id].findings, 7);
  assert.deepEqual(out.truncated, { omitted: { findings: 4, resolvedInChange: 0, focusResults: 0 } });
  assert.equal(out.summary.findings, out.findings.length + out.truncated.omitted.findings);
  assert.equal(out.complete, false);
  assert.ok(out.unchecked.some((line) => /Partial list: 4 finding\(s\) are not listed/.test(line)));
  // The pair-mode lists are capped the same way (the listed ones are the same first three, in the pinned order).
  const pairOut = pair(side([]), side(objects), V2([r], { budgets: { findings: 3, focusResults: 5 } }), { coverage: { base: coverageOf([]), revised: coverageOf(objects) } });
  assert.deepEqual(pairOut.findings.map(stripped), out.findings.map(stripped));
  assert.equal(pairOut.summary.findings, out.summary.findings);
});

test('a rule that is not evaluated anywhere is not a pass: no findings, incomplete coverage, a reason', () => {
  const objects = [{ guid: 'G1', props: [] }];
  const r = rule('property-missing', { key: GRADE });
  const out = single(side(objects), V2([r]));   // no coverage document at all
  assert.equal(out.findings.length, 0);
  assert.deepEqual(out.scope.byRule[r.id].notEvaluated, { 'coverage-not-supplied': 1 });
  assert.equal(out.scope.byRule[r.id].evaluated, 0);
  assert.equal(out.scope.byRule[r.id].coverage.revisedComplete, false);
  assert.equal(out.coverageSha256.revised, null);
});

// ─── what single mode refuses ───────────────────────────────────────────────────────────────────────

test('single mode refuses a profile it cannot answer from one state', () => {
  const objects = [{ guid: 'G1', props: [] }];
  const revised = side(objects);
  const good = [rule('property-missing', { key: GRADE })];
  const v1 = { schema: CHECK_PROFILE_SCHEMA, id: 'a', version: '1', budgets: { findings: 1, focusResults: 1 }, rules: [] };
  refusesWith(() => single(revised, v1), /single mode needs a 'aware\.model-check-profile\/v2' check profile/, 'profile-invalid');
  refusesWith(() => single(revised, V2(good, { scope: 'changed' })), /scope 'all'/, 'profile-invalid');
  const { scope: _s, ...noScope } = V2(good);
  refusesWith(() => single(revised, noScope), /scope 'all'/, 'profile-invalid');
  for (const predicate of ['clash-overlap', 'clash-contact', 'clash-bolted-plies']) {
    const clash = { ...rule('blank', undefined), id: 'firm.clash', predicate, target: 'solid' };
    // 0.6.0 runs clash on one state, but only with that state's geometry: without it the rule is refused as clash, never
    // skipped (and not as a missing geometry block).
    refusesWith(() => single(revised, V2([...good, clash])), new RegExp(`can run the '${predicate}' predicate of rule 'firm.clash' only with the later state's geometry`), 'input-invalid');
  }
});

test('single mode refuses earlier-state inputs at the function boundary', () => {
  const objects = [{ guid: 'G1', props: [] }];
  const checks = V2([rule('property-missing', { key: GRADE })]);
  refusesWith(() => checkSnapshots({ base: side([]), revised: side(objects), profile: PROFILE, checkProfile: checks, mode: 'single' }), /only the later state/, 'input-invalid');
  // A focus and geometry narrow and feed the clash check (0.6.0): without a clash rule they would be ignored, so they are refused.
  refusesWith(() => single(side(objects), checks, { focus: { identities: [] } }), /takes a focus only for a check profile with a clash rule/, 'input-invalid');
  refusesWith(() => single(side(objects), checks, { geometry: {} }), /takes geometry only for a check profile with a clash rule/, 'input-invalid');
  refusesWith(() => single(side(objects), checks, { geometry: { base: { tiles: [] }, revised: { tiles: [] } } }), /only the later state's geometry/, 'input-invalid');
  refusesWith(() => single(side(objects), checks, { coverage: { base: coverageOf(objects), revised: null } }), /no base coverage/, 'input-invalid');
  refusesWith(() => checkSnapshots({ base: null, revised: side(objects), profile: PROFILE, checkProfile: checks, mode: 'both' }), /mode must be one of/, 'input-invalid');
});

// ─── pair mode did not move ─────────────────────────────────────────────────────────────────────────

test('pair mode output is byte-identical to the 0.4.0 golden, except evaluator.version', async () => {
  const dir = join(here, 'test-fixtures', 'check-v2-pair-golden');
  const request = {
    'base-dir': join(dir, 'base'), 'revised-dir': join(dir, 'revised'), 'profile-path': join(dir, 'profile.json'),
    'check-profile-path': join(dir, 'checks.json'), 'coverage-base-path': join(dir, 'coverage-base.json'), 'coverage-revised-path': join(dir, 'coverage-revised.json'),
  };
  const golden = JSON.parse(readFileSync(join(dir, 'golden.json'), 'utf8'));
  assert.equal(golden.evaluator.version, '0.4.0', 'the golden was generated by the 0.4.0 code');
  for (const explicit of [undefined, 'pair']) {
    const out = await checkSnapshotsFromFiles(explicit ? { ...request, mode: explicit } : request);
    assert.equal(out.evaluator.version, EVALUATOR_VERSION);
    assert.equal('mode' in out, false);
    out.evaluator.version = '0.4.0';   // the one intended difference
    assert.equal(canonicalJson(out), canonicalJson(golden));
  }
  assert.ok(golden.summary.introduced >= 1 && golden.summary.preExisting >= 1 && golden.scope.excluded >= 1, 'the golden exercises introduced, pre-existing and excluded');
});

// ─── the process boundary ───────────────────────────────────────────────────────────────────────────

const spawn = (entry, input) => spawnSync(process.execPath, [join(here, entry), 'check-snapshots'], { input: JSON.stringify(input), encoding: 'utf8', cwd: here });

for (const entry of ['index.mjs', 'model-dispatcher.mjs']) {
  test(`${entry} check-snapshots mode single: files in, canonical single-mode document out; every refused input exits non-zero with nothing on stdout`, () => {
    const root = mkdtempSync(join(tmpdir(), 'snapshot-check-single-'));
    try {
      const objects = richObjects();
      writeSide(join(root, 'revised'), side(objects, richRelationships()));
      mkdirSync(join(root, 'base'), { recursive: true });
      const checks = V2(ALL_RULES());
      writeFileSync(join(root, 'profile.json'), JSON.stringify(PROFILE));
      writeFileSync(join(root, 'checks.json'), JSON.stringify(checks));
      writeFileSync(join(root, 'coverage-revised.json'), JSON.stringify(coverageOf(objects), null, 2));
      writeFileSync(join(root, 'coverage-base.json'), JSON.stringify(coverageOf([]), null, 2));
      writeFileSync(join(root, 'focus.json'), JSON.stringify({ identities: [] }));
      const request = {
        mode: 'single', 'revised-dir': join(root, 'revised'), 'profile-path': join(root, 'profile.json'),
        'check-profile-path': join(root, 'checks.json'), 'coverage-revised-path': join(root, 'coverage-revised.json'),
      };
      const r = spawn(entry, request);
      assert.equal(r.status, 0, r.stderr);
      const out = JSON.parse(r.stdout);
      assert.equal(r.stdout, canonicalJson(out), 'canonical bytes on stdout, nothing else');
      assert.equal(spawn(entry, request).stdout, r.stdout, 'second run is byte-identical');
      assert.equal(out.mode, 'single');
      assert.equal(out.inputs.base, null);
      assert.equal(out.coverageSha256.base, null);
      assert.equal(out.coverageSha256.revised, sha(readFileSync(join(root, 'coverage-revised.json'), 'utf8')));
      assert.equal(out.inputs.revised.entities, sha(readFileSync(join(root, 'revised', 'entities.json'), 'utf8')));
      assert.equal(out.evaluator.profileSha256, sha(canonicalJson(checks)));
      assert.equal(out.evaluator.version, EVALUATOR_VERSION);
      assert.ok(out.findings.length > 0 && out.findings.every((f) => f.presence === 'present'));
      assert.equal(out.summary.present, out.summary.findings);
      // Without a coverage document the property rules are labelled, not passed, and the digest is null.
      const { 'coverage-revised-path': _drop, ...noCoverage } = request;
      const bare = JSON.parse(spawn(entry, noCoverage).stdout);
      assert.equal(bare.coverageSha256.revised, null);
      assert.ok(bare.scope.byRule['firm.property-missing'].notEvaluated['coverage-not-supplied'] > 0);

      const refused = (patch, pattern) => {
        const bad = spawn(entry, { ...request, ...patch });
        assert.notEqual(bad.status, 0, JSON.stringify(patch));
        assert.equal(bad.stdout, '', JSON.stringify(patch));
        assert.match(bad.stderr, pattern, JSON.stringify(patch));
      };
      refused({ 'base-dir': join(root, 'base') }, /`base-dir` is not accepted in single mode/);
      refused({ 'coverage-base-path': join(root, 'coverage-base.json') }, /`coverage-base-path` is not accepted in single mode/);
      refused({ 'focus-path': join(root, 'focus.json') }, /`focus-path` is accepted in single mode only for a check profile with a clash rule/);
      refused({ 'base-geometry-dir': root }, /`base-geometry-dir` is not accepted in single mode/);
      refused({ 'revised-geometry-dir': root }, /`revised-geometry-dir` is accepted in single mode only for a check profile with a clash rule/);
      refused({ mode: 'both' }, /`mode` must be one of pair, single/);
      for (const key of ['revised-dir', 'profile-path', 'check-profile-path']) {
        const { [key]: _gone, ...missing } = request;
        const bad = spawn(entry, missing);
        assert.notEqual(bad.status, 0, key);
        assert.equal(bad.stdout, '', key);
        assert.match(bad.stderr, new RegExp(`\`${key}\` is required`), key);
      }
      writeFileSync(join(root, 'v1.json'), JSON.stringify({ schema: CHECK_PROFILE_SCHEMA, id: 'a', version: '1', budgets: { findings: 1, focusResults: 1 }, rules: [] }));
      refused({ 'check-profile-path': join(root, 'v1.json') }, /single mode needs a 'aware\.model-check-profile\/v2'/);
      writeFileSync(join(root, 'changed.json'), JSON.stringify(V2(ALL_RULES(), { scope: 'changed' })));
      refused({ 'check-profile-path': join(root, 'changed.json') }, /scope 'all'/);
      writeFileSync(join(root, 'noscope.json'), JSON.stringify((({ scope: _s, ...rest }) => rest)(V2(ALL_RULES()))));
      refused({ 'check-profile-path': join(root, 'noscope.json') }, /scope 'all'/);
      // A clash predicate without the later state's geometry is refused (0.6.0 runs it only with `revised-geometry-dir`).
      const clashRule = {
        id: 'firm.overlap', version: '1.0.0', severity: 'error', predicate: 'clash-overlap', target: 'solid',
        appliesTo: { types: ['Beam'] }, worsening: null,
      };
      writeFileSync(join(root, 'clash.json'), JSON.stringify(V2([...ALL_RULES(), clashRule])));
      const clash = spawn(entry, { ...request, 'check-profile-path': join(root, 'clash.json') });
      assert.notEqual(clash.status, 0);
      assert.equal(clash.stdout, '');
      assert.match(clash.stderr, /`revised-geometry-dir` is required in single mode \(rule 'firm.overlap' uses 'clash-overlap'\)/);
    } finally { rmSync(root, { recursive: true, force: true }); }
  });

  test(`${entry} check-snapshots --validate-only with mode single applies the single-mode profile rules and lists the modes`, () => {
    const root = mkdtempSync(join(tmpdir(), 'snapshot-check-single-validate-'));
    try {
      writeFileSync(join(root, 'checks.json'), JSON.stringify(V2(ALL_RULES())));
      writeFileSync(join(root, 'changed.json'), JSON.stringify(V2(ALL_RULES(), { scope: 'changed' })));
      writeFileSync(join(root, 'v1.json'), JSON.stringify({ schema: CHECK_PROFILE_SCHEMA, id: 'a', version: '1', budgets: { findings: 1, focusResults: 1 }, rules: [] }));
      writeFileSync(join(root, 'profile.json'), JSON.stringify(PROFILE));
      const ok = spawn(entry, { 'validate-only': true, mode: 'single', 'check-profile-path': join(root, 'checks.json'), 'profile-path': join(root, 'profile.json') });
      assert.equal(ok.status, 0, ok.stderr);
      const out = JSON.parse(ok.stdout);
      assert.equal(out.valid, true);
      assert.equal(out.evaluator.version, EVALUATOR_VERSION);
      assert.deepEqual(out.supports.modes, ['pair', 'single']);
      for (const file of ['changed.json', 'v1.json']) {
        const bad = spawn(entry, { 'validate-only': true, mode: 'single', 'check-profile-path': join(root, file) });
        assert.notEqual(bad.status, 0, file);
        assert.equal(bad.stdout, '', file);
      }
      // The same profiles still validate without mode single (pair mode accepts them).
      for (const file of ['changed.json', 'v1.json']) assert.equal(spawn(entry, { 'validate-only': true, 'check-profile-path': join(root, file) }).status, 0, file);
      assert.notEqual(spawn(entry, { 'validate-only': true, mode: 'both', 'check-profile-path': join(root, 'checks.json') }).status, 0);
    } finally { rmSync(root, { recursive: true, force: true }); }
  });
}

test('a pair-mode call through the file wrapper still needs both directories (default mode is pair)', async () => {
  await assert.rejects(
    () => checkSnapshotsFromFiles({ 'revised-dir': 'x', 'profile-path': 'y', 'check-profile-path': 'z' }),
    (error) => error instanceof SnapshotCompareError && /`base-dir` is required/.test(error.message),
  );
});
