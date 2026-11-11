// Unit tests for `check-snapshots`. Synthetic facade documents (format-blind, like the compare tests): the
// Tekla-looking keys come in through the compare PROFILE, the checks through the CHECK PROFILE; the code knows
// neither. Every expected key and fingerprint below is recomputed here from the documented formula, so a change
// to either formula, to the pairing or to a presence rule fails a test by name.
import test from 'node:test';
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { canonicalJson, compareSnapshots, SnapshotCompareError } from './snapshot-compare.mjs';
import { GUID_KEY, PROFILE } from './snapshot-compare-fixtures.mjs';
import {
  checkSnapshots, CHECK_PROFILE_SCHEMA, FINDINGS_SCHEMA, presenceOfChanged, validateCheckProfile,
} from './snapshot-check.mjs';

const sha = (text) => createHash('sha256').update(text, 'utf8').digest('hex');
const id = (guid) => `lineage-A:${guid}`;
const RULE = 'integrity.part-name-blank';

/** objects: { guid, type, name (key `name` absent = no name key), profile, extraGuids } -> the three facade documents. */
function side(objects, { relationships = [] } = {}) {
  const entities = [];
  const properties = [];
  const entityIdOf = new Map();
  objects.forEach((o, index) => {
    const entityId = `element:${index + 1}`;
    if (o.guid) entityIdOf.set(o.guid, entityId);
    const entity = { id: entityId, type: o.type ?? 'Beam' };
    if (!('name' in o)) entity.name = `part ${o.guid}`;
    else if (o.name !== undefined) entity.name = o.name;
    entities.push(entity);
    let n = 0;
    const push = (sourceKey, value) => {
      properties.push({ id: `prop:${entityId}:${n}`, entityId, groupName: 'g', groupOrdinal: 0, name: sourceKey, value, valueType: 'string', unit: null, propertyOrdinal: n, semantic: 'x', dimension: 'none', isUda: false, sourceKey });
      n += 1;
    };
    if (o.guid) push(GUID_KEY, o.guid);
    for (const guid of o.extraGuids ?? []) push(GUID_KEY, guid);
    push('native:profile', o.profile ?? 'HEA200');
  });
  const rels = relationships.map(([kind, from, to], i) => ({ id: `rel:${i}`, kind, from: entityIdOf.get(from), to: entityIdOf.get(to) }));
  return { entities: { entities }, properties: { properties }, relationships: { relationships: rels } };
}

const rule = (patch = {}) => ({
  id: RULE, version: '1.0.0', severity: 'warning', predicate: 'blank', target: 'entity.name',
  appliesTo: { types: ['Beam', 'Column'] }, worsening: null, ...patch,
});
const CHECKS = (patch = {}) => ({
  schema: CHECK_PROFILE_SCHEMA, id: 'floless.change-checks', version: '1.0.0',
  budgets: { findings: 2000, focusResults: 20000 }, rules: [rule()], ...patch,
});
const check = (base, revised, extra = {}) => checkSnapshots({ base, revised, profile: PROFILE, checkProfile: CHECKS(), ...extra });
const findingOf = (out, guid, ruleId = RULE) => out.findings.find((f) => f.ruleId === ruleId && f.participants[0].identity === id(guid));
const stateOf = (out, guid, ruleId = RULE) => out.focusResults.find((r) => r.identity === id(guid) && r.ruleId === ruleId)?.state;

// A scenario that exercises every presence value a `blank` rule can produce.
function scenario() {
  const base = side([
    { guid: 'G2', name: '' },                       // blank before and after, something else changes -> pre-existing
    { guid: 'G3', name: '' },                       // blank before; blank, but differently, after -> pre-existing-changed
    { guid: 'G4', name: 'Fine' },                   // named before, blank after -> introduced
    { guid: 'G5', name: '' },                       // blank before, named after -> resolved
    { guid: 'G6', name: '' },                       // blank before and after, nothing changes -> only checked when named
    { guid: 'G7', name: 'Fine' },                   // untouched, fine
  ]);
  const revised = side([
    { guid: 'G1', name: '' },                       // added, blank -> introduced (no base object)
    { guid: 'G2', name: '', profile: 'HEB300' },
    { guid: 'G3', name: ' ' },
    { guid: 'G4', name: '' },
    { guid: 'G5', name: 'Now named' },
    { guid: 'G6', name: '' },
    { guid: 'G7', name: 'Fine' },
  ]);
  return { base, revised };
}

test('keys and fingerprints are exactly the documented formulas, recomputed independently', () => {
  const { base, revised } = scenario();
  const out = check(base, revised);
  assert.equal(out.schema, FINDINGS_SCHEMA);
  const g1 = findingOf(out, 'G1');
  const expectedKey = sha(`{"locus":{},"participants":["lineage-A:G1"],"ruleId":"${RULE}"}`);
  assert.equal(g1.correspondenceKey, expectedKey);
  assert.equal(g1.evidenceFingerprint, sha(`{"correspondenceKey":"${expectedKey}","evidence":{"name":""},"ruleVersion":"1.0.0"}`));
  // Same object, same rule, different evidence: same correspondence key, different fingerprint.
  const g3 = findingOf(out, 'G3');
  assert.equal(g3.correspondenceKey, sha(`{"locus":{},"participants":["lineage-A:G3"],"ruleId":"${RULE}"}`));
  assert.equal(g3.evidenceFingerprint, sha(`{"correspondenceKey":"${g3.correspondenceKey}","evidence":{"name":" "},"ruleVersion":"1.0.0"}`));
  assert.equal(g3.baseEvidenceFingerprint, sha(`{"correspondenceKey":"${g3.correspondenceKey}","evidence":{"name":""},"ruleVersion":"1.0.0"}`));
  assert.notEqual(g3.evidenceFingerprint, g3.baseEvidenceFingerprint);
  // A rule version bump changes the fingerprint but not the key.
  const bumped = check(base, revised, { checkProfile: CHECKS({ rules: [rule({ version: '1.0.1' })] }) });
  assert.equal(findingOf(bumped, 'G1').correspondenceKey, expectedKey);
  assert.equal(findingOf(bumped, 'G1').evidenceFingerprint, sha(`{"correspondenceKey":"${expectedKey}","evidence":{"name":""},"ruleVersion":"1.0.1"}`));
  assert.equal(out.inputs.checkProfileDigest, sha(canonicalJson(CHECKS())));
  assert.equal(out.inputs.profileDigest, sha(canonicalJson(PROFILE)));
  assert.equal(out.inputs.focusDigest, null);
  assert.deepEqual(out.checkProfile, { id: 'floless.change-checks', version: '1.0.0' });
});

test('presence: introduced, pre-existing, pre-existing-changed and resolvedInChange, and which objects are in scope', () => {
  const { base, revised } = scenario();
  const out = check(base, revised);
  const presence = Object.fromEntries(out.findings.map((f) => [f.participants[0].identity, f.presence]));
  assert.deepEqual(presence, {
    [id('G1')]: 'introduced', [id('G2')]: 'pre-existing', [id('G3')]: 'pre-existing-changed', [id('G4')]: 'introduced',
  });
  assert.equal(findingOf(out, 'G1').baseEvidenceFingerprint, null);
  assert.equal(findingOf(out, 'G4').baseEvidenceFingerprint, null, 'no base finding for G4: it was named before');
  assert.equal(findingOf(out, 'G2').baseEvidenceFingerprint, findingOf(out, 'G2').evidenceFingerprint);
  assert.deepEqual(findingOf(out, 'G1').participants[0], {
    identity: id('G1'), entityIdBase: null, entityIdRevised: 'element:1', type: 'Beam', name: '',
  });
  assert.equal(findingOf(out, 'G2').participants[0].entityIdBase, 'element:1');
  // G5 was blank and is not any more.
  assert.equal(out.resolvedInChange.length, 1);
  const resolved = out.resolvedInChange[0];
  assert.equal(resolved.participants[0].identity, id('G5'));
  assert.deepEqual(resolved.evidence, { name: '' });
  assert.equal(resolved.evidenceFingerprint, sha(`{"correspondenceKey":"${resolved.correspondenceKey}","evidence":{"name":""},"ruleVersion":"1.0.0"}`));
  // G6 (blank, unchanged, not named) and G7 are NOT in scope: no finding for G6.
  assert.equal(findingOf(out, 'G6'), undefined);
  assert.deepEqual(out.summary, { findings: 4, introduced: 2, preExisting: 2, resolvedInChange: 1, byRule: { [RULE]: 4 } });
  assert.deepEqual(out.scope, {
    added: 1, changed: 4, focus: 0, evaluated: 5, notApplicable: 0, excluded: 0,
    byRule: { [RULE]: { evaluated: 5, findings: 4, notApplicable: 0 } },
  });
  assert.equal(out.complete, true);
  assert.equal(out.truncated, null);
  assert.equal(out.unchecked[0], 'Checks ran on added and changed objects, and on objects named for a re-check, only; unchanged objects were not checked.');
  const keys = out.findings.map((f) => f.correspondenceKey);
  assert.deepEqual(keys, [...keys].sort(), 'sorted by correspondenceKey within one rule');
});

test('focus brings an unchanged object into scope, and a finding there is pre-existing', () => {
  const { base, revised } = scenario();
  const out = check(base, revised, { focus: { identities: [id('G6'), id('G7')] } });
  assert.equal(findingOf(out, 'G6').presence, 'pre-existing');
  assert.equal(findingOf(out, 'G7'), undefined);
  assert.equal(out.scope.focus, 2);
  assert.equal(out.scope.evaluated, 7);
  assert.equal(out.scope.changed, 4);
  assert.equal(out.inputs.focusDigest, sha(canonicalJson({ identities: [id('G6'), id('G7')] })));
  assert.equal(stateOf(out, 'G6'), 'finding');
  assert.equal(stateOf(out, 'G7'), 'passed');
  // The same object named while it also changed is still one evaluation, not two.
  const both = check(base, revised, { focus: { identities: [id('G2')] } });
  assert.equal(both.scope.evaluated, 5);
  assert.equal(both.scope.focus, 1);
});

test('every focusResults state: finding, passed, not-applicable, excluded (both ways), absent', () => {
  const base = side([{ guid: 'A', name: 'x' }]);
  const revised = side([
    { guid: 'A', name: '' },                       // finding
    { guid: 'B', name: 'ok' },                     // passed
    { guid: 'C', name: '', type: 'Bolt' },         // not applicable (type not listed)
    { guid: null, name: 'no identity' },           // blank identity -> an exclusion in revised
    { guid: 'D', name: 'x' }, { guid: 'D', name: 'y' },   // duplicate identity -> excluded
  ]);
  const focus = { identities: [id('A'), id('B'), id('C'), id('D'), id('MISSING')] };
  const out = check(base, revised, { focus });
  assert.equal(stateOf(out, 'A'), 'finding');
  assert.equal(stateOf(out, 'B'), 'passed');
  assert.equal(stateOf(out, 'C'), 'not-applicable');
  assert.equal(stateOf(out, 'D'), 'excluded', 'identity excluded as a duplicate');
  assert.equal(stateOf(out, 'MISSING'), 'excluded', 'absence is not provable while revised has an excluded object');
  assert.deepEqual(out.focusResults.map((r) => r.identity), [id('A'), id('B'), id('C'), id('D'), id('MISSING')], 'sorted by identity');
  assert.equal(out.scope.excluded, 3, 'one blank + two duplicate objects');
  assert.match(out.unchecked.join('\n'), /3 object\(s\) in the revised state were not checked because their identity was blank or duplicated \(1 blank, 2 duplicate\)/);

  // With no blank or duplicate identity in revised, a missing identity is a confirmed absence.
  const clean = check(base, side([{ guid: 'A', name: 'x' }]), { focus: { identities: [id('A'), id('GONE')] } });
  assert.equal(stateOf(clean, 'GONE'), 'absent');
  assert.equal(stateOf(clean, 'A'), 'passed');
  assert.equal(clean.scope.excluded, 0);
});

test('an identity the base state holds twice is excluded in revised too, never reported absent or checked', () => {
  const base = side([{ guid: 'D', name: 'a' }, { guid: 'D', name: 'b' }]);
  const revised = side([{ guid: 'D', name: '' }]);
  const out = check(base, revised, { focus: { identities: [id('D')] } });
  assert.equal(stateOf(out, 'D'), 'excluded');
  assert.deepEqual(out.findings, []);
  assert.equal(out.scope.added, 0, 'compare-snapshots does not call it added either');
  assert.equal(compareSnapshots({ base, revised, profile: PROFILE }).summary.added, 0);
});

test('applicability is by the revised entity type; a rule that stops applying never resolves its base finding', () => {
  const checks = CHECKS({ rules: [rule(), rule({ id: 'integrity.plate-name-blank', appliesTo: { types: ['Contour plate'] } })] });
  const base = side([{ guid: 'P', name: '', type: 'Contour plate' }, { guid: 'B', name: '', type: 'Beam' }]);
  const revised = side([
    { guid: 'P', name: '', type: 'Contour plate', profile: 'PL12' },
    { guid: 'B', name: '', type: 'Bolt' },          // re-typed: the beam rule no longer applies; that is not a fix
    { guid: 'N', name: '', type: 'Beam' },
  ]);
  const out = check(base, revised, { checkProfile: checks });
  assert.deepEqual(out.findings.map((f) => [f.ruleId, f.participants[0].identity, f.presence]), [
    ['integrity.part-name-blank', id('N'), 'introduced'],
    ['integrity.plate-name-blank', id('P'), 'pre-existing'],
  ]);
  assert.deepEqual(out.scope.byRule, {
    'integrity.part-name-blank': { evaluated: 1, findings: 1, notApplicable: 2 },
    'integrity.plate-name-blank': { evaluated: 1, findings: 1, notApplicable: 2 },
  });
  assert.equal(out.scope.evaluated + out.scope.notApplicable, 6, 'three objects x two rules');
  assert.deepEqual(out.resolvedInChange, [], 'a rule that no longer applies proves nothing was fixed');
});

test('blank means missing, null, not a string, or empty after NFKC + trim; evidence carries the raw value', () => {
  const names = [
    ['EMPTY', '', { name: '' }], ['SPACES', '   ', { name: '   ' }], ['NBSP', '  ', { name: '  ' }],
    ['IDEOGRAPHIC', '　', { name: '　' }], ['NULL', null, { name: null }], ['NUMBER', 5, { name: 5 }],
    ['MISSING', undefined, { name: null }],
  ];
  const fine = [['LIG', 'ﬁ'], ['PLAIN', 'B-1'], ['PADDED', ' B-1 ']];
  const revised = side([...names.map(([guid, name]) => ({ guid, name })), ...fine.map(([guid, name]) => ({ guid, name }))]);
  const out = check(side([]), revised);
  for (const [guid, , evidence] of names) assert.deepEqual(findingOf(out, guid)?.evidence, evidence, `${guid} is blank`);
  for (const [guid] of fine) assert.equal(findingOf(out, guid), undefined, `${guid} is not blank`);
  assert.equal(findingOf(out, 'MISSING').participants[0].name, null);
});

test('presence for a rule with a numeric metric: worsened, improved, and equal-is-changed (classification unit)', () => {
  const higher = { worsening: { path: 'gap', direction: 'higher-is-worse' } };
  const lower = { worsening: { path: 'gap', direction: 'lower-is-worse' } };
  assert.equal(presenceOfChanged(higher, { gap: 2 }, { gap: 5 }), 'pre-existing-worsened');
  assert.equal(presenceOfChanged(higher, { gap: 5 }, { gap: 2 }), 'pre-existing-improved');
  assert.equal(presenceOfChanged(lower, { gap: 5 }, { gap: 2 }), 'pre-existing-worsened');
  assert.equal(presenceOfChanged(lower, { gap: 2 }, { gap: 5 }), 'pre-existing-improved');
  assert.equal(presenceOfChanged(higher, { gap: 3 }, { gap: 3 }), 'pre-existing-changed');
  assert.equal(presenceOfChanged(higher, { gap: 'a' }, { gap: 5 }), 'pre-existing-changed');
  assert.equal(presenceOfChanged({ worsening: null }, { gap: 2 }, { gap: 5 }), 'pre-existing-changed');
  // The schema accepts a worsening block on a rule; only `blank` refuses one (it has no number to move).
  assert.throws(() => validateCheckProfile(CHECKS({ rules: [rule(higher)] })), /worsening must be null for the 'blank' predicate/);
});

test('scope is the full pairing, not the capped change rows: compare-snapshots with a tiny cap agrees', () => {
  const objects = Array.from({ length: 6 }, (_, i) => ({ guid: `G${i}`, name: '' }));
  const base = side(objects.slice(0, 4));
  const revised = side([...objects.slice(0, 3).map((o) => ({ ...o, profile: 'HEB300' })), objects[3], objects[4], objects[5]],
    { relationships: [['connects', 'G3', 'G4']] });
  const tiny = { ...PROFILE, budgets: { ...PROFILE.budgets, changes: 1 } };
  const compared = compareSnapshots({ base, revised, profile: tiny });
  assert.equal(compared.changes.length, 1, 'precondition: compare-snapshots listed only one row');
  const out = checkSnapshots({ base, revised, profile: tiny, checkProfile: CHECKS() });
  assert.equal(out.scope.added, compared.summary.added);
  assert.equal(out.scope.changed, compared.summary.changed);
  assert.deepEqual([out.scope.added, out.scope.changed], [2, 3]);
  assert.equal(out.findings.length, 5, 'G0-G2 (changed) and G4-G5 (added) are checked; unchanged G3 is not');
});

test('relationship-only changes put both endpoints in scope, exactly as compare-snapshots reports them', () => {
  const base = side([{ guid: 'X', name: '' }, { guid: 'Y', name: '' }, { guid: 'Z', name: '' }]);
  const revised = side([{ guid: 'X', name: '' }, { guid: 'Y', name: '' }, { guid: 'Z', name: '' }], { relationships: [['welded', 'X', 'Y']] });
  const compared = compareSnapshots({ base, revised, profile: PROFILE });
  const out = check(base, revised);
  assert.deepEqual(compared.changes.map((c) => c.identity), [id('X'), id('Y')]);
  assert.deepEqual(out.findings.map((f) => f.participants[0].identity).sort(), [id('X'), id('Y')]);
  assert.equal(out.scope.changed, 2);
  assert.ok(out.findings.every((f) => f.presence === 'pre-existing'));
});

test('budgets cap findings, resolvedInChange and focusResults in sorted order; counts stay complete', () => {
  const base = side(['R1', 'R2', 'R3'].map((guid) => ({ guid, name: '' })));
  const revised = side([
    ...['F1', 'F2', 'F3'].map((guid) => ({ guid, name: '' })),
    ...['R1', 'R2', 'R3'].map((guid) => ({ guid, name: 'named' })),
  ]);
  const focus = { identities: [id('F1'), id('F2'), id('F3')] };
  const full = check(base, revised, { focus });
  assert.equal(full.complete, true);
  assert.equal(full.truncated, null);
  const out = check(base, revised, { focus, checkProfile: CHECKS({ budgets: { findings: 2, focusResults: 1 } }) });
  assert.equal(out.summary.findings, 3, 'summary counts everything');
  assert.equal(out.summary.resolvedInChange, 3);
  assert.equal(out.scope.byRule[RULE].findings, 3);
  assert.equal(out.findings.length, 2);
  assert.equal(out.resolvedInChange.length, 2);
  assert.equal(out.focusResults.length, 1);
  assert.deepEqual(out.findings, full.findings.slice(0, 2));
  assert.deepEqual(out.resolvedInChange, full.resolvedInChange.slice(0, 2));
  assert.deepEqual(out.focusResults, full.focusResults.slice(0, 1));
  assert.deepEqual(out.truncated, { omitted: { findings: 1, resolvedInChange: 1, focusResults: 2 } });
  assert.equal(out.complete, false);
  assert.match(out.unchecked.join('\n'), /Partial list: 1 finding\(s\), 1 resolved finding\(s\) and 2 re-check row\(s\) are not listed/);
  // A cap of exactly the number of rows omits nothing.
  const exact = check(base, revised, { focus, checkProfile: CHECKS({ budgets: { findings: 3, focusResults: 3 } }) });
  assert.equal(exact.complete, true);
  assert.equal(exact.truncated, null);
});

test('deterministic: two runs, and the same documents in another row order, give byte-identical output', () => {
  const { base, revised } = scenario();
  const focus = { identities: [id('G7'), id('G6')] };
  const first = canonicalJson(check(base, revised, { focus }));
  assert.equal(canonicalJson(check(base, revised, { focus })), first);
  const reordered = { ...revised, properties: { properties: [...revised.properties.properties].reverse() } };
  assert.equal(canonicalJson(check(base, reordered, { focus })), first);
});

test('refusals: every unknown or missing key, predicate, target, bad focus and non-facade document', () => {
  const { base, revised } = scenario();
  const refuses = (checkProfile, pattern) => assert.throws(
    () => check(base, revised, { checkProfile }),
    (error) => error instanceof SnapshotCompareError && pattern.test(error.message),
    String(pattern),
  );
  refuses({ ...CHECKS(), surprise: 1 }, /unknown key 'surprise'/);
  const { rules: _rules, ...noRules } = CHECKS();
  refuses(noRules, /missing 'rules'/);
  refuses(CHECKS({ schema: 'aware.model-check-profile/v2' }), /schema must be/);
  refuses(CHECKS({ budgets: { findings: 1 } }), /budgets is missing 'focusResults'/);
  refuses(CHECKS({ budgets: { findings: -1, focusResults: 1 } }), /budgets.findings must be a non-negative integer/);
  refuses(CHECKS({ rules: [rule({ extra: true })] }), /unknown key 'extra'/);
  refuses(CHECKS({ rules: [rule({ predicate: 'in-range' })] }), /predicate 'in-range' is not known/);
  refuses(CHECKS({ rules: [rule({ target: 'entity.type' })] }), /target 'entity.type' is not known/);
  refuses(CHECKS({ rules: [rule({ severity: 'fatal' })] }), /severity must be one of/);
  refuses(CHECKS({ rules: [rule({ appliesTo: { types: [] } })] }), /types must be a non-empty array/);
  refuses(CHECKS({ rules: [rule({ appliesTo: { types: ['Beam'], more: 1 } })] }), /unknown key 'more'/);
  refuses(CHECKS({ rules: [rule(), rule()] }), /used by two rules/);
  refuses(CHECKS({ rules: [rule({ worsening: { path: 'gap' } })] }), /worsening is missing 'direction'/);
  refuses(CHECKS({ rules: [rule({ worsening: { path: 'gap', direction: 'sideways' } })] }), /direction must be one of/);
  for (const bad of [{ identities: [id('G1')], more: 1 }, { identities: 'G1' }, { identities: [5] }, { identities: [''] }, { identities: [id('G1'), id('G1')] }, []]) {
    assert.throws(() => check(base, revised, { focus: bad }), (error) => error instanceof SnapshotCompareError && error.code === 'focus-invalid', JSON.stringify(bad));
  }
  assert.throws(() => check(base, { ...revised, entities: { entities: [], extra: [] } }), (error) => error.code === 'document-invalid');
  assert.throws(() => checkSnapshots({ base, revised, profile: { ...PROFILE, surprise: 1 }, checkProfile: CHECKS() }), /unknown key 'surprise'/);
});
