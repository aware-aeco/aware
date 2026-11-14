// Tests for the v2 single-object rules of `check-snapshots` (property-missing, property-not-in, property-out-of-range,
// relationship-missing), the read-coverage input, scope 'all', unknown presence and the v1 byte-identity lock.
// Synthetic facade documents in an invented vocabulary: AWARE has no format knowledge, so the keys here are just data.
import test from 'node:test';
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { canonicalJson, SnapshotCompareError } from './snapshot-compare.mjs';
import { GUID_KEY, PROFILE } from './snapshot-compare-fixtures.mjs';
import {
  checkSnapshots, checkSnapshotsFromFiles, CHECK_PROFILE_SCHEMA, CHECK_PROFILE_SCHEMA_V2, COVERAGE_SCHEMA, EVALUATOR_VERSION,
  FINDINGS_SCHEMA, FINDINGS_SCHEMA_V2, validateCheckProfile,
} from './snapshot-check.mjs';

const here = new URL('.', import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, '$1');
const sha = (text) => createHash('sha256').update(text, 'utf8').digest('hex');
const id = (guid) => `lineage-A:${guid}`;

/** objects: { guid, type, name, props: [{ key, value, unit?, name? }] }; rels: [kind, fromGuid|'element:n', toGuid|'element:n']. */
function side(objects, rels = []) {
  const entities = [];
  const properties = [];
  const entityIdOf = new Map();
  objects.forEach((o, index) => {
    const entityId = `element:${index + 1}`;
    if (o.guid) entityIdOf.set(o.guid, entityId);
    entities.push({ id: entityId, type: o.type ?? 'Beam', name: o.name ?? `part ${o.guid}` });
    const push = (n, p) => properties.push({
      id: `prop:${entityId}:${n}`, entityId, groupName: 'g', groupOrdinal: 0, name: p.name ?? p.key, value: p.value,
      valueType: typeof p.value === 'number' ? 'number' : 'string', unit: p.unit ?? null, propertyOrdinal: n,
      semantic: 'x', dimension: 'none', isUda: false, sourceKey: p.key,
    });
    let n = 0;
    if (o.guid) push(n++, { key: GUID_KEY, value: o.guid });
    for (const p of o.props ?? []) push(n++, p);
  });
  const ref = (token) => (token.startsWith('element:') ? token : entityIdOf.get(token));
  const relationships = rels.map(([kind, from, to], i) => ({ id: `rel:${i}`, kind, from: ref(from), to: ref(to) }));
  return { entities: { entities }, properties: { properties }, relationships: { relationships } };
}

/** A coverage document for a side: every entity reads `read`; `perObject[guid]` overrides it; `omit` leaves entities out. */
function coverageOf(objects, { read = ['native:*', 'uda:*'], kinds = ['hosts'], perObject = {}, omit = [] } = {}) {
  const entities = {};
  objects.forEach((o, index) => {
    if (omit.includes(o.guid)) return;
    entities[`element:${index + 1}`] = { properties: perObject[o.guid] ?? read };
  });
  return { schema: COVERAGE_SCHEMA, entities, relationships: { complete: kinds } };
}

const rule = (predicate, parameters, patch = {}) => ({
  id: `firm.${predicate}`, version: '1.0.0', severity: 'warning', predicate,
  target: predicate === 'relationship-missing' ? 'relationship' : 'property',
  appliesTo: { types: ['Beam'] }, worsening: null, parameters, ...patch,
});
const V2 = (rules, patch = {}) => ({
  schema: CHECK_PROFILE_SCHEMA_V2, id: 'firm.standards', version: '1.0.0', scope: 'all',
  budgets: { findings: 2000, focusResults: 20000 }, rules, ...patch,
});
const run = (base, revised, checkProfile, extra = {}) => checkSnapshots({ base, revised, profile: PROFILE, checkProfile, ...extra });
const both = (baseObjects, revisedObjects, opts) => ({ base: coverageOf(baseObjects, opts), revised: coverageOf(revisedObjects, opts) });
const presences = (out) => Object.fromEntries(out.findings.map((f) => [f.participants[0].identity, f.presence]));
const stat = (out, rule_) => out.scope.byRule[rule_.id];
const refuses = (checkProfile, pattern, code = 'profile-invalid') => assert.throws(
  () => validateCheckProfile(checkProfile),
  (error) => error instanceof SnapshotCompareError && error.code === code && pattern.test(error.message),
  String(pattern),
);

const GRADE = 'native:grade';
const grade = (value) => ({ key: GRADE, value });

// ─── the lock: a v1 profile gets today's v1 document, byte for byte ─────────────────────────────────

test('v1 profiles: the findings document is byte-identical to the FloLess S2 goldens (generated before this change)', async () => {
  const dir = join(here, 'test-fixtures', 'check-v1-golden');
  const request = (revised, focus) => ({
    'base-dir': join(dir, 'A'), 'revised-dir': join(dir, revised), 'profile-path': join(dir, 'profile.json'),
    'check-profile-path': join(dir, 'check-profile.json'), ...(focus ? { 'focus-path': join(dir, focus) } : {}),
  });
  const cases = [['B', null], ['C', 'focus-C.json'], ['D', 'focus-D.json']];
  for (const [revised, focus] of cases) {
    const golden = JSON.parse(readFileSync(join(dir, `golden-${revised}.json`), 'utf8'));
    const out = await checkSnapshotsFromFiles(request(revised, focus));
    assert.equal(out.schema, FINDINGS_SCHEMA);
    assert.equal(canonicalJson(out), canonicalJson(golden), `state ${revised}`);
    for (const absent of ['evaluator', 'coverageSha256']) assert.equal(absent in out, false);
  }
  assert.equal(readdirSync(dir).length >= 10, true);
});

test('a v1 profile cannot use anything v2 added: the new predicates, targets, parameters and scope are all rejected', () => {
  const v1 = (patch) => ({ ...V2([rule('property-missing', { key: GRADE })]), schema: CHECK_PROFILE_SCHEMA, ...patch });
  refuses(v1({}), /unknown key 'scope'/);
  const { scope: _scope, ...noScope } = v1({});
  refuses(noScope, /unknown key 'parameters'/);
  const { parameters: _parameters, ...noParameters } = noScope.rules[0];
  refuses({ ...noScope, rules: [noParameters] }, /predicate 'property-missing' is not known/);
  const blankWithParameters = { ...noScope, rules: [{ ...rule('blank', {}), target: 'entity.name' }] };
  refuses(blankWithParameters, /unknown key 'parameters'/);
  refuses({ ...noScope, rules: [{ ...noParameters, predicate: 'blank', target: 'property' }] }, /target 'property' is not known/);
});

// ─── property-missing ───────────────────────────────────────────────────────────────────────────────

test('property-missing: red on an absent or blank value, green once fixed; presence is introduced / pre-existing', () => {
  const r = rule('property-missing', { key: GRADE });
  const baseObjects = [
    { guid: 'G1', props: [grade('S355')] }, { guid: 'G2', props: [grade('S355')] },
    { guid: 'G3', props: [] }, { guid: 'G4', props: [grade('S355')] },
  ];
  const revisedObjects = [
    { guid: 'G1', props: [grade('S355')] },        // fine
    { guid: 'G2', props: [] },                     // row gone -> introduced
    { guid: 'G3', props: [] },                     // missing before and after -> pre-existing
    { guid: 'G4', props: [grade('  ')] },          // blank after (NFKC + trim) -> introduced
    { guid: 'G5', props: [] },                     // new and missing -> introduced
  ];
  const base = side(baseObjects);
  const revised = side(revisedObjects);
  const out = run(base, revised, V2([r]), { coverage: both(baseObjects, revisedObjects) });
  assert.equal(out.schema, FINDINGS_SCHEMA_V2);
  assert.deepEqual(presences(out), { [id('G2')]: 'introduced', [id('G3')]: 'pre-existing', [id('G4')]: 'introduced', [id('G5')]: 'introduced' });
  const g4 = out.findings.find((f) => f.participants[0].identity === id('G4'));
  assert.deepEqual(g4.evidence, { key: GRADE, present: true, value: '  ' });
  const g2 = out.findings.find((f) => f.participants[0].identity === id('G2'));
  assert.deepEqual(g2.evidence, { key: GRADE, present: false, value: null });
  assert.equal(g2.baseEvidenceFingerprint, null);
  assert.deepEqual(stat(out, r), { evaluated: 5, findings: 4, notApplicable: 0, notEvaluated: {}, coverage: { revisedComplete: true, baseComplete: true }, evaluatedScope: 'all' });
  // Fixed: every object carries a grade -> no findings, and a base finding that is gone is resolved.
  const fixedObjects = revisedObjects.map((o) => ({ ...o, props: [grade('S355')] }));
  const fixed = run(base, side(fixedObjects), V2([r]), { coverage: both(baseObjects, fixedObjects) });
  assert.equal(fixed.findings.length, 0);
  assert.deepEqual(fixed.resolvedInChange.map((x) => x.participants[0].identity), [id('G3')]);
  assert.equal(fixed.summary.introduced + fixed.summary.preExisting + fixed.summary.unknown, fixed.summary.findings);
});

test('a property is found by its CANONICAL key (after the comparison profile keyRules), not by its renumbered definition id', () => {
  const r = rule('property-missing', { key: 'uda:text:name:fire rating' });
  const revisedObjects = [
    { guid: 'G1', props: [{ key: 'uda:text:17', name: 'Fire Rating', value: 'R60' }] },
    { guid: 'G2', props: [{ key: 'uda:text:99', name: 'Other', value: 'x' }] },
  ];
  const revised = side(revisedObjects);
  const base = side([]);
  const out = run(base, revised, V2([r]), { coverage: { base: coverageOf([]), revised: coverageOf(revisedObjects) } });
  assert.deepEqual(presences(out), { [id('G2')]: 'introduced' });
});

// ─── property-not-in ────────────────────────────────────────────────────────────────────────────────

test('property-not-in: exact vs nfkc-casefold, and a missing value is not evaluated (never a pass, never a finding)', () => {
  const objects = [
    { guid: 'G1', props: [grade('HEA200')] },       // in the list
    { guid: 'G2', props: [grade('hea200')] },       // case differs
    { guid: 'G3', props: [grade('XYZ')] },          // not in the list
    { guid: 'G4', props: [] },                      // no value
    { guid: 'G5', props: [grade('')] },             // blank value
    { guid: 'G6', props: [grade('ＨＥＡ２００')] },  // full-width letters: NFKC folds them
  ];
  const revised = side(objects);
  const base = side([]);
  const coverage = { base: coverageOf([]), revised: coverageOf(objects) };
  const exact = rule('property-not-in', { key: GRADE, values: ['HEA200', 'IPE300'], match: 'exact' });
  const out = run(base, revised, V2([exact]), { coverage });
  assert.deepEqual(Object.keys(presences(out)).sort(), [id('G2'), id('G3'), id('G6')].sort());
  assert.deepEqual(stat(out, exact).notEvaluated, { 'value-missing': 2 });
  assert.deepEqual(stat(out, exact).coverage, { revisedComplete: false, baseComplete: true });
  assert.deepEqual(out.findings.find((f) => f.participants[0].identity === id('G3')).evidence, { key: GRADE, value: 'XYZ', match: 'exact' });
  const folded = rule('property-not-in', { key: GRADE, values: ['HEA200', 'IPE300'], match: 'nfkc-casefold' });
  const out2 = run(base, revised, V2([folded]), { coverage });
  assert.deepEqual(Object.keys(presences(out2)), [id('G3')]);
  // The value is the object's: replacing the XYZ with a listed one clears the finding.
  const fixed = objects.map((o) => (o.guid === 'G3' ? { ...o, props: [grade('IPE300')] } : o));
  assert.equal(run(base, side(fixed), V2([folded]), { coverage: { base: coverageOf([]), revised: coverageOf(fixed) } }).findings.length, 0);
});

// ─── property-out-of-range ──────────────────────────────────────────────────────────────────────────

test('property-out-of-range: below / above / on the bound, the distance is the worsening metric, and worse/better is told apart', () => {
  const r = rule('property-out-of-range', { key: 'native:length', min: 100, max: 500, unit: 'mm' });
  const len = (value, unit = 'mm') => ({ key: 'native:length', value, unit });
  const baseObjects = [
    { guid: 'G1', props: [len(450)] }, { guid: 'G2', props: [len(600)] }, { guid: 'G3', props: [len(600)] },
    { guid: 'G4', props: [len(50)] }, { guid: 'G5', props: [len(300)] },
  ];
  const revisedObjects = [
    { guid: 'G1', props: [len(99.5)] },   // fell below min: introduced, distance 0.5
    { guid: 'G2', props: [len(700)] },    // was 100 over, now 200 over: worsened
    { guid: 'G3', props: [len(550)] },    // was 100 over, now 50 over: improved
    { guid: 'G4', props: [len(50)] },     // unchanged -> pre-existing (scope all)
    { guid: 'G5', props: [len(500)] },    // on the bound -> passed
    { guid: 'G6', props: [len(100)] },    // on the other bound -> passed
  ];
  const out = run(side(baseObjects), side(revisedObjects), V2([r]), { coverage: both(baseObjects, revisedObjects) });
  assert.deepEqual(presences(out), {
    [id('G1')]: 'introduced', [id('G2')]: 'pre-existing-worsened', [id('G3')]: 'pre-existing-improved', [id('G4')]: 'pre-existing',
  });
  assert.deepEqual(out.findings.find((f) => f.participants[0].identity === id('G1')).evidence, {
    key: 'native:length', value: 99.5, unit: 'mm', min: 100, max: 500, distanceOutside: 0.5,
  });
  assert.deepEqual(stat(out, r).notEvaluated, {});
  // Only one bound given: the other is null in the evidence.
  const upper = rule('property-out-of-range', { key: 'native:length', max: 500, unit: 'mm' });
  const out2 = run(side(baseObjects), side(revisedObjects), V2([upper]), { coverage: both(baseObjects, revisedObjects) });
  assert.equal(out2.findings.find((f) => f.participants[0].identity === id('G2')).evidence.min, null);
  assert.equal(out2.findings.some((f) => f.participants[0].identity === id('G1')), false);
});

test('property-out-of-range: a missing value, a non-number and another unit are each not evaluated with their own reason', () => {
  const r = rule('property-out-of-range', { key: 'native:length', min: 100, unit: 'mm' });
  const objects = [
    { guid: 'G1', props: [] },
    { guid: 'G2', props: [{ key: 'native:length', value: 'long', unit: 'mm' }] },
    { guid: 'G3', props: [{ key: 'native:length', value: 3, unit: 'in' }] },
    { guid: 'G4', props: [{ key: 'native:length', value: 3, unit: null }] },
    { guid: 'G5', props: [{ key: 'native:length', value: 50, unit: 'mm' }] },
    { guid: 'G6', props: [{ key: 'native:length', value: null, unit: 'mm' }] },
  ];
  const out = run(side([]), side(objects), V2([r]), { coverage: { base: coverageOf([]), revised: coverageOf(objects) } });
  assert.deepEqual(stat(out, r).notEvaluated, { 'value-missing': 2, 'value-not-numeric': 1, 'unit-mismatch': 2 });
  assert.deepEqual(Object.keys(presences(out)), [id('G5')], 'only the judged object is reported');
  assert.equal(stat(out, r).coverage.revisedComplete, false);
  assert.ok(out.unchecked.some((line) => line.includes('unit-mismatch')), 'the document says what it could not judge');
});

// ─── relationship-missing ───────────────────────────────────────────────────────────────────────────

test('relationship-missing: the object is the `to` (or `from`) end of no row of that kind', () => {
  const objects = [{ guid: 'H1', type: 'Host' }, { guid: 'G1' }, { guid: 'G2' }, { guid: 'G3' }];
  const rels = [['hosts', 'H1', 'G1'], ['contains', 'H1', 'G3']];
  const revised = side(objects, rels);
  const base = side([]);
  const coverage = { base: coverageOf([]), revised: coverageOf(objects, { kinds: ['hosts', 'contains'] }) };
  const toRule = rule('relationship-missing', { kind: 'hosts', end: 'to' });
  const out = run(base, revised, V2([toRule]), { coverage });
  assert.deepEqual(Object.keys(presences(out)).sort(), [id('G2'), id('G3')]);   // G1 is hosted; Host is another type
  assert.deepEqual(out.findings[0].evidence, { kind: 'hosts', end: 'to' });
  const fromRule = rule('relationship-missing', { kind: 'hosts', end: 'from' }, { appliesTo: { types: ['Host', 'Beam'] } });
  const out2 = run(base, revised, V2([fromRule]), { coverage });
  assert.deepEqual(Object.keys(presences(out2)).sort(), [id('G1'), id('G2'), id('G3')]);   // only H1 hosts something
  // Fixed: G2 and G3 get a host.
  const fixedRels = [...rels, ['hosts', 'H1', 'G2'], ['hosts', 'H1', 'G3']];
  assert.equal(run(base, side(objects, fixedRels), V2([toRule]), { coverage }).findings.length, 0);
});

// ─── not evaluated: one fixture per reason ──────────────────────────────────────────────────────────

test('coverage-not-supplied: no coverage document for the side means nothing absent can be called missing', () => {
  const r = rule('property-missing', { key: GRADE });
  const objects = [{ guid: 'G1', props: [] }];
  const out = run(side([]), side(objects), V2([r]));
  assert.equal(out.findings.length, 0);
  assert.deepEqual(stat(out, r).notEvaluated, { 'coverage-not-supplied': 1 });
  assert.equal(stat(out, r).coverage.revisedComplete, false);
  assert.deepEqual(out.coverageSha256, { base: null, revised: null });
  const rel = rule('relationship-missing', { kind: 'hosts', end: 'to' }, { id: 'firm.rel' });
  assert.deepEqual(stat(run(side([]), side(objects), V2([rel])), rel).notEvaluated, { 'coverage-not-supplied': 1 });
});

test('source-did-not-read: a document that does not cover the key, the entity or the relationship kind', () => {
  const r = rule('property-missing', { key: 'uda:text:name:fire rating' });
  const objects = [{ guid: 'G1', props: [] }, { guid: 'G2', props: [] }, { guid: 'G3', props: [] }];
  const coverage = (revised) => ({ base: coverageOf([]), revised });
  // G1 read native:* only; G2 is absent from the document; G3 read the whole uda family.
  const doc = coverageOf(objects, { perObject: { G1: ['native:*'], G3: ['uda:*'] }, omit: ['G2'] });
  const out = run(side([]), side(objects), V2([r]), { coverage: coverage(doc) });
  assert.deepEqual(Object.keys(presences(out)), [id('G3')], 'only the entity whose family was read can be called missing');
  assert.deepEqual(stat(out, r).notEvaluated, { 'source-did-not-read': 2 });
  // An exact key and a family both cover; a family does not cover a prefix without the colon boundary.
  const exactDoc = coverageOf(objects, { read: ['uda:text:name:fire rating'] });
  assert.equal(run(side([]), side(objects), V2([r]), { coverage: coverage(exactDoc) }).findings.length, 3);
  const nearMiss = coverageOf(objects, { read: ['uda:text:name:fire:*'] });
  assert.deepEqual(stat(run(side([]), side(objects), V2([r]), { coverage: coverage(nearMiss) }), r).notEvaluated, { 'source-did-not-read': 3 });
  // A relationship kind not listed complete.
  const rel = rule('relationship-missing', { kind: 'hosts', end: 'to' });
  const relOut = run(side([]), side(objects), V2([rel]), { coverage: coverage(coverageOf(objects, { kinds: ['contains'] })) });
  assert.deepEqual(stat(relOut, rel).notEvaluated, { 'source-did-not-read': 3 });
});

test('key-ambiguous: two rows with one canonical key are never guessed between', () => {
  const r = rule('property-missing', { key: 'uda:text:name:grade' });
  const objects = [
    { guid: 'G1', props: [{ key: 'uda:text:1', name: 'Grade', value: 'a' }, { key: 'uda:text:2', name: 'GRADE', value: 'b' }] },
    { guid: 'G2', props: [] },
  ];
  const out = run(side([]), side(objects), V2([r]), { coverage: { base: coverageOf([]), revised: coverageOf(objects) } });
  assert.deepEqual(stat(out, r).notEvaluated, { 'key-ambiguous': 1 });
  assert.deepEqual(Object.keys(presences(out)), [id('G2')]);
});

test('identity-unusable: objects excluded for a blank or duplicate identity count against completeness, by rule type', () => {
  const r = rule('property-missing', { key: GRADE });
  const other = rule('property-missing', { key: GRADE }, { id: 'firm.other', appliesTo: { types: ['Plate'] } });
  const objects = [
    { guid: 'G1', props: [grade('x')] },
    { guid: 'G2', props: [grade('x')] },        // same GUID as G3 -> both excluded
    { guid: 'G2', props: [grade('x')] },
    { guid: ' ', props: [grade('x')] },         // blank identity -> excluded
    { guid: 'G9', type: 'Plate', props: [grade('x')] },
  ];
  const out = run(side([]), side(objects), V2([r, other]), { coverage: { base: coverageOf([]), revised: coverageOf(objects) } });
  assert.deepEqual(stat(out, r).notEvaluated, { 'identity-unusable': 3 });
  assert.equal(stat(out, r).coverage.revisedComplete, false);
  assert.deepEqual(stat(out, other).notEvaluated, {}, 'a rule for another type is not affected');
  assert.equal(stat(out, other).coverage.revisedComplete, true);
});

test('relationship-endpoint-unusable: a row of the kind that ended here but was discarded for its other end is not a "missing"', () => {
  const objects = [{ guid: 'G1' }, { guid: 'G2' }, { guid: 'G3' }, { guid: 'DUP' }, { guid: 'DUP' }];
  // G1 is hosted by a duplicate-identity object (row discarded); G2 by an entity that does not exist; G3 by nothing.
  const rels = [['hosts', 'element:4', 'G1'], ['hosts', 'element:999', 'G2']];
  const r = rule('relationship-missing', { kind: 'hosts', end: 'to' });
  const out = run(side([]), side(objects, rels), V2([r]), { coverage: { base: coverageOf([]), revised: coverageOf(objects) } });
  assert.deepEqual(stat(out, r).notEvaluated, { 'identity-unusable': 2, 'relationship-endpoint-unusable': 2 });
  assert.deepEqual(Object.keys(presences(out)), [id('G3')]);
});

// ─── side-specific results and unknown presence ─────────────────────────────────────────────────────

test('unknown presence: a base side that cannot be judged never makes a finding introduced, and never resolves one', () => {
  const r = rule('property-missing', { key: GRADE });
  const baseObjects = [{ guid: 'G1', props: [] }, { guid: 'G2', props: [] }, { guid: 'G3', props: [grade('x')] }];
  const revisedObjects = [{ guid: 'G1', props: [] }, { guid: 'G2', props: [grade('x')] }, { guid: 'G3', props: [] }];
  const revisedCoverage = coverageOf(revisedObjects);
  // No base coverage at all: G1 and G3 are findings in the revised state, G2 was possibly a finding before.
  let out = run(side(baseObjects), side(revisedObjects), V2([r]), { coverage: { base: null, revised: revisedCoverage } });
  assert.deepEqual(presences(out), { [id('G1')]: 'unknown', [id('G3')]: 'unknown' });
  assert.equal(out.summary.unknown, 2);
  assert.equal(out.summary.introduced + out.summary.preExisting + out.summary.unknown, out.summary.findings);
  assert.equal(out.resolvedInChange.length, 0, 'a base finding that could not be established is never resolved');
  assert.deepEqual(stat(out, r).coverage, { revisedComplete: true, baseComplete: false });
  // A base document that did not read the key behaves the same.
  out = run(side(baseObjects), side(revisedObjects), V2([r]), { coverage: { base: coverageOf(baseObjects, { read: ['uda:*'] }), revised: revisedCoverage } });
  assert.deepEqual(presences(out), { [id('G1')]: 'unknown', [id('G3')]: 'unknown' });
  // With the base covered: G1 pre-existing, G3 introduced, G2 resolved.
  out = run(side(baseObjects), side(revisedObjects), V2([r]), { coverage: { base: coverageOf(baseObjects), revised: revisedCoverage } });
  assert.deepEqual(presences(out), { [id('G1')]: 'pre-existing', [id('G3')]: 'introduced' });
  assert.deepEqual(out.resolvedInChange.map((x) => x.participants[0].identity), [id('G2')]);
  assert.equal(out.summary.unknown, 0);
});

test('a revised object that cannot be judged is never resolved, even if the base had a finding', () => {
  const r = rule('property-missing', { key: GRADE });
  const baseObjects = [{ guid: 'G1', props: [] }];
  const revisedObjects = [{ guid: 'G1', props: [grade('x')] }];
  const out = run(side(baseObjects), side(revisedObjects), V2([r]), { coverage: { base: coverageOf(baseObjects), revised: coverageOf(revisedObjects, { read: ['uda:*'] }) } });
  assert.equal(out.resolvedInChange.length, 0);
  assert.equal(out.findings.length, 0);
  assert.deepEqual(stat(out, r).notEvaluated, { 'source-did-not-read': 1 });
});

test('an object whose base type the rule does not cover is introduced, not unknown (the rule did not apply before)', () => {
  const r = rule('property-missing', { key: GRADE });
  const baseObjects = [{ guid: 'G1', type: 'Plate', props: [] }];
  const revisedObjects = [{ guid: 'G1', type: 'Beam', props: [] }];
  const out = run(side(baseObjects), side(revisedObjects), V2([r]), { coverage: { base: null, revised: coverageOf(revisedObjects) } });
  assert.deepEqual(presences(out), { [id('G1')]: 'introduced' });
});

// ─── scope ──────────────────────────────────────────────────────────────────────────────────────────

test("scope 'all' judges every applicable object (an old violation is pre-existing); 'changed' judges only what changed", () => {
  const r = rule('property-missing', { key: GRADE });
  const baseObjects = [{ guid: 'G1', props: [] }, { guid: 'G2', props: [grade('x')] }, { guid: 'G3', props: [] }];
  const revisedObjects = [{ guid: 'G1', props: [] }, { guid: 'G2', props: [grade('x')] }, { guid: 'G3', props: [], profile: 'changed' }];
  const revisedWithChange = side(revisedObjects.map((o) => (o.guid === 'G3' ? { ...o, props: [{ key: 'native:profile', value: 'HEB300' }] } : o)));
  const baseWithProfile = side(baseObjects.map((o) => (o.guid === 'G3' ? { ...o, props: [{ key: 'native:profile', value: 'HEA200' }] } : o)));
  const coverage = both(baseObjects, revisedObjects);
  const all = run(baseWithProfile, revisedWithChange, V2([r], { scope: 'all' }), { coverage });
  assert.deepEqual(presences(all), { [id('G1')]: 'pre-existing', [id('G3')]: 'pre-existing' });
  assert.equal(all.scope.mode, 'all');
  assert.equal(all.scope.objects, 3);
  assert.equal(stat(all, r).evaluated, 3);
  const changed = run(baseWithProfile, revisedWithChange, V2([r], { scope: 'changed' }), { coverage });
  assert.deepEqual(presences(changed), { [id('G3')]: 'pre-existing' });
  assert.equal(changed.scope.mode, 'changed');
  assert.equal(stat(changed, r).evaluated, 1);
  const omitted = V2([r]);
  delete omitted.scope;
  assert.equal(run(baseWithProfile, revisedWithChange, omitted, { coverage }).scope.mode, 'changed', 'scope defaults to changed');
  // Identical states with an existing violation: pre-existing, never introduced.
  const same = run(side(baseObjects), side(baseObjects), V2([r]), { coverage: both(baseObjects, baseObjects) });
  assert.deepEqual(presences(same), { [id('G1')]: 'pre-existing', [id('G3')]: 'pre-existing' });
});

// ─── echoes, determinism ────────────────────────────────────────────────────────────────────────────

test('v2 echoes the evaluator and the coverage digests; the same inputs give the same bytes', () => {
  const r = rule('property-missing', { key: GRADE });
  const objects = [{ guid: 'G1', props: [] }];
  const profile = V2([r]);
  const digests = { coverage: { base: 'a'.repeat(64), revised: 'b'.repeat(64) } };
  const out = run(side(objects), side(objects), profile, { coverage: both(objects, objects), digests });
  assert.deepEqual(out.evaluator, { agent: 'model-change-set', version: EVALUATOR_VERSION, profileSha256: sha(canonicalJson(profile)) });
  assert.equal(out.inputs.checkProfileDigest, out.evaluator.profileSha256);
  assert.deepEqual(out.coverageSha256, digests.coverage);
  const again = run(side(objects), side(objects), profile, { coverage: both(objects, objects), digests });
  assert.equal(canonicalJson(again), canonicalJson(out));
  const reordered = { ...side(objects), properties: { properties: [...side(objects).properties.properties].reverse() } };
  assert.equal(canonicalJson(run(side(objects), reordered, profile, { coverage: both(objects, objects), digests })), canonicalJson(out));
});

test('EVALUATOR_VERSION is the version the agent manifest declares', () => {
  const manifest = readFileSync(join(here, '..', '20-agents', 'aeco', 'engineering', 'model-change-set', 'manifest.yaml'), 'utf8');
  assert.equal(/^version: (\S+)/m.exec(manifest)[1], EVALUATOR_VERSION);
});

test('blank and the v2 rules can share a profile; blank behaves as in v1 and is never held back by coverage', () => {
  const blank = { id: 'firm.blank', version: '1.0.0', severity: 'warning', predicate: 'blank', target: 'entity.name', appliesTo: { types: ['Beam'] }, worsening: null };
  const objects = [{ guid: 'G1', name: '', props: [] }];
  const out = run(side([]), side(objects), V2([blank, rule('property-missing', { key: GRADE })]));
  assert.deepEqual(out.findings.map((f) => f.ruleId), ['firm.blank']);
  assert.deepEqual(stat(out, blank), { evaluated: 1, findings: 1, notApplicable: 0, notEvaluated: {}, coverage: { revisedComplete: true, baseComplete: true }, evaluatedScope: 'all' });
});

// ─── strict validation: negative controls ───────────────────────────────────────────────────────────

test('strict validation: every malformed v2 rule is rejected for the reason it names', () => {
  const ok = (r, patch) => V2([r], patch);
  validateCheckProfile(ok(rule('property-missing', { key: GRADE })));   // the control: the baseline is valid
  refuses(ok(rule('property-missing', { key: GRADE, extra: 1 })), /parameters has unknown key 'extra'/);
  refuses(ok(rule('property-missing', undefined)), /parameters is required/);
  refuses(ok(rule('property-missing', {})), /parameters is missing 'key'/);
  refuses(ok(rule('property-missing', { key: '' })), /key must be a non-empty string/);
  refuses(ok(rule('property-missing', { key: 'uda:*' })), /no '\*'/);
  refuses(ok(rule('property-missing', { key: GRADE }, { target: 'entity.name' })), /target must be 'property'/);
  refuses(ok(rule('property-missing', { key: GRADE }, { worsening: { path: 'x', direction: 'higher-is-worse' } })), /worsening must be null/);
  refuses(ok(rule('property-missing', { key: GRADE }, { scope: 'x' })), /unknown key 'scope'/);
  refuses(ok(rule('property-missing', { key: GRADE }), { scope: 'sometimes' }), /scope must be one of/);
  const notIn = (p) => ok(rule('property-not-in', { key: GRADE, values: ['a'], match: 'exact', ...p }));
  refuses(notIn({ match: 'regex' }), /match must be one of/);
  const { match: _match, ...noMatch } = { key: GRADE, values: ['a'], match: 'exact' };
  refuses(ok(rule('property-not-in', noMatch)), /parameters is missing 'match'/);
  refuses(notIn({ values: [] }), /values must be an array of 1 to 5000/);
  refuses(notIn({ values: ['a', ''] }), /values\[1\] must be a non-empty string/);
  refuses(notIn({ values: ['a', 5] }), /values\[1\] must be a non-empty string/);
  refuses(notIn({ values: ['x'.repeat(65)] }), /at most 64 characters/);
  refuses(notIn({ values: Array.from({ length: 5001 }, (_, i) => `v${i}`) }), /1 to 5000/);
  validateCheckProfile(notIn({ values: Array.from({ length: 5000 }, (_, i) => `v${i}`) }));   // the bound itself is allowed
  const range = (p) => ok(rule('property-out-of-range', { key: 'k', min: 1, max: 2, unit: 'mm', ...p }));
  refuses(ok(rule('property-out-of-range', { key: 'k', unit: 'mm' })), /needs at least one of min and max/);
  refuses(range({ min: '1' }), /min must be a finite number/);
  refuses(range({ max: Number.POSITIVE_INFINITY }), /max must be a finite number/);
  refuses(range({ min: 3 }), /min must not be greater than max/);
  refuses(ok(rule('property-out-of-range', { key: 'k', min: 1 })), /parameters is missing 'unit'/);
  refuses(range({ unit: '' }), /unit must be a non-empty string/);
  refuses(ok(rule('property-out-of-range', { key: 'k', min: 1, unit: 'mm' }, { worsening: { path: 'distanceOutside', direction: 'lower-is-worse' } })), /worsening must be null or/);
  validateCheckProfile(ok(rule('property-out-of-range', { key: 'k', min: 1, unit: 'mm' }, { worsening: { path: 'distanceOutside', direction: 'higher-is-worse' } })));
  const rel = (p) => ok(rule('relationship-missing', { kind: 'hosts', end: 'to', ...p }));
  refuses(rel({ end: 'both' }), /end must be one of to, from/);
  refuses(rel({ kind: '' }), /kind must be a non-empty string/);
  refuses(ok(rule('relationship-missing', { end: 'to' })), /parameters is missing 'kind'/);
  refuses(ok(rule('relationship-missing', { kind: 'hosts', end: 'to' }, { target: 'property' })), /target must be 'relationship'/);
  refuses(ok(rule('in-range', {})), /predicate 'in-range' is not known/);
  refuses(ok(rule('property-missing', { key: GRADE }), { surprise: 1 }), /unknown key 'surprise'/);
  // The compiled worsening of out-of-range is always the distance, even if the rule says null.
  assert.deepEqual(validateCheckProfile(ok(rule('property-out-of-range', { key: 'k', min: 1, unit: 'mm' }))).rules[0].worsening, { path: 'distanceOutside', direction: 'higher-is-worse' });
});

test('a rule that reads the comparison profile identity property is rejected (it cannot be judged)', () => {
  assert.throws(
    () => run(side([]), side([{ guid: 'G1' }]), V2([rule('property-missing', { key: GUID_KEY })])),
    (error) => error instanceof SnapshotCompareError && error.code === 'profile-invalid' && /identity property/.test(error.message),
  );
});

test('strict validation of a coverage document: unknown keys, a wrong schema, a bare * and malformed families are rejected', () => {
  const objects = [{ guid: 'G1', props: [] }];
  const r = V2([rule('property-missing', { key: GRADE })]);
  const good = coverageOf(objects);
  const bad = (coverage, pattern) => assert.throws(
    () => run(side([]), side(objects), r, { coverage: { base: null, revised: coverage } }),
    (error) => error instanceof SnapshotCompareError && error.code === 'coverage-invalid' && pattern.test(error.message),
    String(pattern),
  );
  run(side([]), side(objects), r, { coverage: { base: null, revised: good } });   // the control
  bad({ ...good, extra: 1 }, /unknown key 'extra'/);
  bad({ ...good, schema: 'aware.model-read-coverage/v0' }, /schema must be/);
  bad({ ...good, entities: { 'element:1': { properties: ['*'] } } }, /not a family|invalid/);
  bad({ ...good, entities: { 'element:1': { properties: ['a:*:b'] } } }, /invalid/);
  bad({ ...good, entities: { 'element:1': { properties: [''] } } }, /non-empty strings/);
  bad({ ...good, entities: { 'element:1': { read: [] } } }, /unknown key 'read'/);
  bad({ ...good, entities: { 'element:1': {} } }, /missing 'properties'/);
  bad({ ...good, relationships: { complete: [''] } }, /relationship kinds/);
  bad({ ...good, relationships: { done: [] } }, /unknown key 'done'/);
  bad({ schema: COVERAGE_SCHEMA, entities: {} }, /missing 'relationships'/);
});

// ─── the process boundary: coverage files, digests, --validate-only ─────────────────────────────────

function writeFiles(root, objectsBase, objectsRevised, checkProfile, coverageBase, coverageRevised) {
  for (const [name, objects] of [['base', objectsBase], ['revised', objectsRevised]]) {
    const dir = join(root, name);
    mkdirSync(dir, { recursive: true });
    const doc = side(objects);
    writeFileSync(join(dir, 'entities.json'), canonicalJson(doc.entities));
    writeFileSync(join(dir, 'properties.json'), canonicalJson(doc.properties));
    writeFileSync(join(dir, 'relationships.json'), canonicalJson(doc.relationships));
  }
  writeFileSync(join(root, 'profile.json'), JSON.stringify(PROFILE));
  writeFileSync(join(root, 'checks.json'), JSON.stringify(checkProfile));
  if (coverageBase) writeFileSync(join(root, 'coverage-base.json'), JSON.stringify(coverageBase, null, 2));
  if (coverageRevised) writeFileSync(join(root, 'coverage-revised.json'), JSON.stringify(coverageRevised, null, 2));
}
const spawn = (entry, input) => spawnSync(process.execPath, [join(here, entry), 'check-snapshots'], { input: JSON.stringify(input), encoding: 'utf8', cwd: here });

for (const entry of ['index.mjs', 'model-dispatcher.mjs']) {
  test(`${entry} check-snapshots v2: coverage files in, v2 document out with the digests of the exact file bytes`, () => {
    const root = mkdtempSync(join(tmpdir(), 'snapshot-check-v2-'));
    try {
      const baseObjects = [{ guid: 'G1', props: [grade('x')] }];
      const revisedObjects = [{ guid: 'G1', props: [] }];
      const checks = V2([rule('property-missing', { key: GRADE })]);
      writeFiles(root, baseObjects, revisedObjects, checks, coverageOf(baseObjects), coverageOf(revisedObjects));
      const request = {
        'base-dir': join(root, 'base'), 'revised-dir': join(root, 'revised'), 'profile-path': join(root, 'profile.json'),
        'check-profile-path': join(root, 'checks.json'), 'coverage-base-path': join(root, 'coverage-base.json'), 'coverage-revised-path': join(root, 'coverage-revised.json'),
      };
      const r = spawn(entry, request);
      assert.equal(r.status, 0, r.stderr);
      const out = JSON.parse(r.stdout);
      assert.equal(r.stdout, canonicalJson(out), 'canonical bytes on stdout, nothing else');
      assert.equal(out.schema, FINDINGS_SCHEMA_V2);
      assert.deepEqual(out.coverageSha256, {
        base: sha(readFileSync(join(root, 'coverage-base.json'), 'utf8')), revised: sha(readFileSync(join(root, 'coverage-revised.json'), 'utf8')),
      });
      assert.equal(out.evaluator.profileSha256, sha(canonicalJson(checks)));
      assert.deepEqual(out.findings.map((f) => f.presence), ['introduced']);
      assert.equal(spawn(entry, request).stdout, r.stdout, 'second run is byte-identical');
      // Only the revised side's document: the base cannot be judged, so presence is unknown and its digest is null.
      const { 'coverage-base-path': _drop, ...revisedOnly } = request;
      const partial = JSON.parse(spawn(entry, revisedOnly).stdout);
      assert.equal(partial.coverageSha256.base, null);
      assert.deepEqual(partial.findings.map((f) => f.presence), ['unknown']);
      // A bad coverage file fails with a message on stderr and nothing on stdout.
      writeFileSync(join(root, 'coverage-revised.json'), JSON.stringify({ schema: COVERAGE_SCHEMA, entities: {}, relationships: { complete: [] }, extra: 1 }));
      const bad = spawn(entry, request);
      assert.notEqual(bad.status, 0);
      assert.equal(bad.stdout, '');
      assert.match(bad.stderr, /unknown key 'extra'/);
    } finally { rmSync(root, { recursive: true, force: true }); }
  });

  test(`${entry} check-snapshots --validate-only: validates the profile and echoes the evaluator without reading a snapshot`, () => {
    const root = mkdtempSync(join(tmpdir(), 'snapshot-check-validate-'));
    try {
      const checks = V2([rule('property-missing', { key: GRADE })]);
      writeFileSync(join(root, 'checks.json'), JSON.stringify(checks));
      writeFileSync(join(root, 'profile.json'), JSON.stringify(PROFILE));
      // No base-dir, no revised-dir, and the named directories do not exist: nothing is read.
      const r = spawn(entry, { 'validate-only': true, 'check-profile-path': join(root, 'checks.json'), 'base-dir': join(root, 'nope') });
      assert.equal(r.status, 0, r.stderr);
      const out = JSON.parse(r.stdout);
      assert.equal(r.stdout, canonicalJson(out));
      assert.equal(out.schema, 'aware.model-check-validation/v1');
      assert.equal(out.valid, true);
      assert.deepEqual(out.evaluator, { agent: 'model-change-set', version: EVALUATOR_VERSION, profileSha256: sha(canonicalJson(checks)) });
      assert.equal(out.comparisonProfileChecked, false);
      assert.ok(out.supports.predicates.includes('relationship-missing'));
      assert.deepEqual(out.supports.checkProfileSchemas, [CHECK_PROFILE_SCHEMA, CHECK_PROFILE_SCHEMA_V2]);
      // A v1 profile validates too; with the comparison profile given the identity-key rule is caught.
      const v1 = { schema: CHECK_PROFILE_SCHEMA, id: 'a', version: '1', budgets: { findings: 1, focusResults: 1 }, rules: [] };
      writeFileSync(join(root, 'v1.json'), JSON.stringify(v1));
      assert.equal(JSON.parse(spawn(entry, { 'validate-only': true, 'check-profile-path': join(root, 'v1.json') }).stdout).evaluator.profileSha256, sha(canonicalJson(v1)));
      assert.equal(JSON.parse(spawn(entry, { 'validate-only': true, 'check-profile-path': join(root, 'checks.json'), 'profile-path': join(root, 'profile.json') }).stdout).comparisonProfileChecked, true);
      writeFileSync(join(root, 'identity.json'), JSON.stringify(V2([rule('property-missing', { key: GUID_KEY })])));
      const identity = spawn(entry, { 'validate-only': true, 'check-profile-path': join(root, 'identity.json'), 'profile-path': join(root, 'profile.json') });
      assert.notEqual(identity.status, 0);
      assert.equal(identity.stdout, '');
      // An invalid profile: non-zero, message on stderr, nothing on stdout.
      writeFileSync(join(root, 'invalid.json'), JSON.stringify(V2([rule('property-missing', { key: GRADE, extra: 1 })])));
      const bad = spawn(entry, { 'validate-only': true, 'check-profile-path': join(root, 'invalid.json') });
      assert.notEqual(bad.status, 0);
      assert.equal(bad.stdout, '');
      assert.match(bad.stderr, /unknown key 'extra'/);
      assert.notEqual(spawn(entry, { 'validate-only': 'yes', 'check-profile-path': join(root, 'checks.json') }).status, 0);
      assert.notEqual(spawn(entry, { 'validate-only': true }).status, 0);
    } finally { rmSync(root, { recursive: true, force: true }); }
  });
}

test('coverage files are ignored by a v1 profile, which has nowhere to echo them', async () => {
  const root = mkdtempSync(join(tmpdir(), 'snapshot-check-v1cov-'));
  try {
    const objects = [{ guid: 'G1', name: '' }];
    const v1 = { schema: CHECK_PROFILE_SCHEMA, id: 'a', version: '1', budgets: { findings: 10, focusResults: 10 }, rules: [{ id: 'r', version: '1', severity: 'warning', predicate: 'blank', target: 'entity.name', appliesTo: { types: ['Beam'] }, worsening: null }] };
    writeFiles(root, objects, objects, v1, null, null);
    const request = { 'base-dir': join(root, 'base'), 'revised-dir': join(root, 'revised'), 'profile-path': join(root, 'profile.json'), 'check-profile-path': join(root, 'checks.json') };
    const plain = await checkSnapshotsFromFiles(request);
    const withPath = await checkSnapshotsFromFiles({ ...request, 'coverage-base-path': join(root, 'does-not-exist.json') });
    assert.equal(canonicalJson(withPath), canonicalJson(plain));
  } finally { rmSync(root, { recursive: true, force: true }); }
});

test('nfkc-casefold folds case, not just lower-cases: sharp s, final sigma and full-width letters match; exact stays exact', () => {
  const objects = [
    { guid: 'G1', props: [grade('STRASSE')] }, { guid: 'G2', props: [grade('ΟΔΟΣ')] }, { guid: 'G3', props: [grade('ＨＥＡ')] },
    { guid: 'G4', props: [grade('strasse2')] }, { guid: 'G5', props: [grade('STRAẞE')] },   // capital sharp s
    { guid: 'G6', props: [grade('ı')] },   // dotless i is not an i
  ];
  const coverage = { base: coverageOf([]), revised: coverageOf(objects) };
  const folded = rule('property-not-in', { key: GRADE, values: ['Straße', 'οδος', 'hea', 'i'], match: 'nfkc-casefold' });
  assert.deepEqual(Object.keys(presences(run(side([]), side(objects), V2([folded]), { coverage }))).sort(), [id('G4'), id('G6')]);
  const exact = rule('property-not-in', { key: GRADE, values: ['Straße', 'οδος', 'hea'], match: 'exact' });
  assert.equal(run(side([]), side(objects), V2([exact]), { coverage }).findings.length, 6);
});

test('a rule naming the identity property by its CANONICAL (renamed) key is rejected too', () => {
  const renamed = { ...PROFILE, keyRules: [...PROFILE.keyRules, { match: '^native:guid$', key: 'native:identity' }] };
  const objects = [{ guid: 'G1', props: [] }];
  assert.throws(
    () => checkSnapshots({ base: side([]), revised: side(objects), profile: renamed, checkProfile: V2([rule('property-missing', { key: 'native:identity' })]), coverage: { base: null, revised: coverageOf(objects) } }),
    (error) => error instanceof SnapshotCompareError && error.code === 'profile-invalid' && /identity property/.test(error.message),
  );
});

test("scope 'changed': relationship-missing judges every applicable object (and says so); the other rules judge what changed", () => {
  const baseObjects = [{ guid: 'H1', type: 'Host' }, { guid: 'G1' }, { guid: 'G2' }, { guid: 'G3', props: [] }];
  const revisedObjects = [{ guid: 'G1' }, { guid: 'G2' }, { guid: 'G3', props: [] }];
  const baseRels = [['hosts', 'H1', 'G1'], ['hosts', 'H1', 'G2']];
  const revisedRels = [];   // the host is gone: G1 and G2 lost it, G3 never had one
  const hosted = rule('relationship-missing', { kind: 'hosts', end: 'to' });
  const named = rule('property-missing', { key: GRADE }, { id: 'firm.grade' });
  const coverage = { base: coverageOf(baseObjects, { kinds: ['hosts'] }), revised: coverageOf(revisedObjects, { kinds: ['hosts'] }) };
  const out = run(side(baseObjects, baseRels), side(revisedObjects, revisedRels), V2([hosted, named], { scope: 'changed' }), { coverage });
  // G1 and G2 lost their host: introduced. G3 never had one: pre-existing, never introduced.
  assert.deepEqual(out.findings.filter((f) => f.ruleId === hosted.id).map((f) => [f.participants[0].identity, f.presence]).sort(),
    [[id('G1'), 'introduced'], [id('G2'), 'introduced'], [id('G3'), 'pre-existing']]);
  assert.equal(stat(out, hosted).evaluatedScope, 'all');
  assert.equal(stat(out, named).evaluatedScope, 'changed');
  // The grade rule has no reason to look at G1 or G2 (nothing about them changed), so they are not judged for it.
  assert.equal(out.findings.filter((f) => f.ruleId === named.id).length, 0);
  assert.equal(stat(out, named).evaluated, 0);
  assert.equal(stat(out, hosted).evaluated, 3);
  // 'all' sees G3 as well.
  const all = run(side(baseObjects, baseRels), side(revisedObjects, revisedRels), V2([hosted], { scope: 'all' }), { coverage });
  assert.deepEqual(all.findings.map((f) => [f.participants[0].identity, f.presence]).sort(), [[id('G1'), 'introduced'], [id('G2'), 'introduced'], [id('G3'), 'pre-existing']]);
});

test("scope 'changed': a deleted host-to-beam row whose host has no usable identity is still judged and reported (unknown presence)", () => {
  // The host has a blank identity, so compare-snapshots cannot attribute the removed row to any beam and the beam is not
  // "changed". The rule must still look at the beam: it is no longer hosted, and the earlier state cannot say it was.
  const objects = [{ guid: ' ', type: 'Host' }, { guid: 'G1' }];
  const hosted = rule('relationship-missing', { kind: 'hosts', end: 'to' });
  const coverage = { base: coverageOf(objects, { kinds: ['hosts'] }), revised: coverageOf(objects, { kinds: ['hosts'] }) };
  for (const scope of ['changed', 'all']) {
    const out = run(side(objects, [['hosts', 'element:1', 'G1']]), side(objects, []), V2([hosted], { scope }), { coverage });
    assert.deepEqual(presences(out), { [id('G1')]: 'unknown' }, scope);
    assert.equal(stat(out, hosted).evaluated, 1, scope);
    assert.equal(stat(out, hosted).evaluatedScope, 'all', scope);
    assert.deepEqual(stat(out, hosted).coverage, { revisedComplete: true, baseComplete: false }, scope);   // the earlier state could not judge the beam: its row was lost with the host
  }
});

test('evaluatedScope: every single-object rule says which objects it judged; blank follows the profile scope', () => {
  const blank = { id: 'firm.blank', version: '1.0.0', severity: 'warning', predicate: 'blank', target: 'entity.name', appliesTo: { types: ['Beam'] }, worsening: null };
  const objects = [{ guid: 'G1' }];
  const changed = run(side(objects), side(objects), V2([blank], { scope: 'changed' }));
  assert.equal(stat(changed, blank).evaluatedScope, 'changed');
  assert.equal(stat(run(side(objects), side(objects), V2([blank], { scope: 'all' })), blank).evaluatedScope, 'all');
});
