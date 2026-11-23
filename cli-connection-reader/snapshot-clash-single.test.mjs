// Tests for clash on ONE model version (`check-snapshots` mode single, model-change-set 0.6.0), the overlap report threshold
// (`geometry.minOverlap`), per-rule type pairs (`appliesTo.pairs`), the focus closure and the bounded broad phase.
// Synthetic closed solids from numbers (snapshot-clash-fixtures.mjs); every expected number below is arithmetic on them.
// Threshold boundaries use binary fractions (0.125, 0.5, steps of 2^-7) so the float32 tile holds them exactly.
import test from 'node:test';
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { mkdtempSync, rmSync, writeFileSync, mkdirSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { canonicalJson, SnapshotCompareError } from './snapshot-compare.mjs';
import { PROFILE } from './snapshot-compare-fixtures.mjs';
import { checkSnapshots, CHECK_PROFILE_SCHEMA_V2, EVALUATOR_VERSION } from './snapshot-check.mjs';
import { loadClashEngine } from './snapshot-clash.mjs';
import { box, clashChecks, openCheck, world } from './snapshot-clash-fixtures.mjs';

const engine = await loadClashEngine();
const here = new URL('.', import.meta.url).pathname.replace(/^\/([A-Za-z]:)/, '$1');
const id = (guid) => `lineage-A:${guid}`;
const OVERLAP = 'clash.interpenetration';
const CONTACT = 'clash.unjoined-contact';
const BOLTED = 'clash.bolted-plies-apart';

/** A 100 x 100 plate from z0 to z1, its corner at (x, 0). */
const plate = (guid, z0, z1, { x = 0, ...extra } = {}) => ({ guid, type: 'Contour plate', solid: box([x, 0, z0], [x + 100, 100, z1]), ...extra });
const bolt = (guid, plies, extra = {}) => ({ guid, type: 'Bolt group', bolted: plies, ...extra });

/** A v2, scope-'all' clash profile (what single mode takes); `pairs` patches each rule's `appliesTo.pairs`. */
function singleChecks({ rules, geometry = {}, budgets, pairs = {}, findings } = {}) {
  const checks = clashChecks({ rules, geometry, budgets: findings === undefined ? budgets : { ...budgets, findings } });
  checks.schema = CHECK_PROFILE_SCHEMA_V2;
  checks.scope = 'all';
  checks.rules = checks.rules.map((rule) => (pairs[rule.id] ? { ...rule, appliesTo: { ...rule.appliesTo, pairs: pairs[rule.id] } } : rule));
  return checks;
}
const withBudgets = (patch) => ({ geometry: { budgets: { trianglesPerObject: 200000, trianglePairsPerPair: 4000000, gapSearchMm: 50, pairs: 200000, focusSets: 20000, ...patch } } });

function single(parts, { checks = singleChecks(), focus = null, hosts = [], shuffle = false } = {}) {
  const w = world(parts, { hosts });
  const docs = shuffle ? {
    entities: { entities: [...w.docs.entities.entities].reverse() },
    properties: { properties: [...w.docs.properties.properties].reverse() },
    relationships: { relationships: [...w.docs.relationships.relationships].reverse() },
  } : w.docs;
  return checkSnapshots({ base: null, revised: docs, profile: PROFILE, checkProfile: checks, focus, geometry: { revised: w.tiles }, engine, mode: 'single' });
}
const of = (out, ruleId) => out.findings.filter((f) => f.ruleId === ruleId);
const idsOf = (finding) => finding.participants.map((p) => p.identity);
const refuses = (fn, code, pattern) => assert.throws(fn, (error) => {
  assert.ok(error instanceof SnapshotCompareError, `expected a SnapshotCompareError, got ${error}`);
  assert.equal(error.code, code, error.message);
  if (pattern) assert.match(error.message, pattern);
  return true;
});

// ─── one version: overlap, touch, gap ───────────────────────────────────────────────────────────────

// A-B overlap 5 mm; C-D touch face to face; E-F are 2 mm apart (beyond the 0.1 mm tolerance: neither a touch nor a pair).
const seeded = () => [
  plate('A', 0, 10), plate('B', 5, 15),
  plate('C', 0, 10, { x: 500 }), plate('D', 10, 20, { x: 500 }),
  plate('E', 0, 10, { x: 1000 }), plate('F', 12, 22, { x: 1000 }),
];

test('one version: an overlap and a touch are findings, every one present; a 2 mm gap is neither', () => {
  const out = single(seeded());
  assert.equal(out.mode, 'single');
  assert.deepEqual(out.findings.map((f) => `${f.ruleId}:${idsOf(f).join('+')}`), [`${OVERLAP}:${id('A')}+${id('B')}`, `${CONTACT}:${id('C')}+${id('D')}`]);
  const [overlap] = of(out, OVERLAP);
  assert.equal(overlap.evidence.overlapThicknessMm, 5);
  assert.equal(overlap.evidence.kind, 'overlap');
  assert.equal(of(out, CONTACT)[0].evidence.gapMm, 0);
  for (const f of out.findings) {
    assert.equal(f.presence, 'present');
    assert.equal(f.baseEvidenceFingerprint, null);
    assert.ok(f.participants.every((p) => p.entityIdBase === null && typeof p.entityIdRevised === 'string'));
  }
  assert.deepEqual(out.summary, { findings: 2, present: 2, byRule: { [OVERLAP]: 1, [CONTACT]: 1, [BOLTED]: 0 } });
  assert.deepEqual([out.scope.added, out.scope.changed, out.scope.focus, out.scope.mode, out.scope.objects], [0, 0, 0, 'all', 6]);
  assert.deepEqual(out.scope.solids, { total: 6, evaluated: 6, approximate: 0, noGeometry: 0, overBudget: 0, byReason: {} });
  assert.deepEqual(out.scope.notEvaluated, { pairs: 0, stacks: 0, byReason: {} });
  assert.equal(out.inputs.geometry.base, null);
  assert.match(out.inputs.geometry.revised.tiles[0].sha256, /^[0-9a-f]{64}$/);
  assert.deepEqual(out.resolvedInChange, []);
  assert.deepEqual(out.focusResults, []);
  assert.equal('focusSets' in out, false, 'single mode takes no pair re-checks');
  for (const ruleId of [OVERLAP, CONTACT, BOLTED]) {
    assert.deepEqual(out.scope.byRule[ruleId].coverage, { revisedComplete: true, baseComplete: null });
    assert.equal(out.scope.byRule[ruleId].evaluatedScope, 'all');
    assert.deepEqual(out.scope.byRule[ruleId].notEvaluated, {});
  }
  assert.equal(out.scope.byRule[OVERLAP].belowThreshold, undefined, 'no minOverlap in the profile: no belowThreshold count');
  assert.equal(out.complete, true);
  assert.equal(out.unchecked.some((line) => /unchanged/.test(line)), false, 'no "unchanged parts" sentence in single mode');
  assert.equal(out.unchecked.some((line) => /Clash checks were not run/.test(line)), false);
  assert.equal(out.unchecked.some((line) => /Only pairs that involve/.test(line)), false, 'not focused');
  assert.equal(out.evaluator.version, EVALUATOR_VERSION);
  assert.deepEqual(out.evaluator.geometry, {
    contactTolerance: 0.1, minOverlap: 0.1, weldTolerance: 0.001,
    budgets: { trianglesPerObject: 200000, trianglePairsPerPair: 4000000, gapSearchMm: 50, pairs: 200000, focusSets: 20000 },
    pairs: { [OVERLAP]: null, [CONTACT]: null },
  });
});

test('one version gets the verdicts pair mode gives the same state against an empty earlier state', () => {
  const parts = [...seeded(), plate('G', 0, 10, { x: 1500 }), plate('H', 13, 23, { x: 1500 }), bolt('BG', ['G', 'H'])];
  const one = single(parts);
  const w = world(parts); const empty = world([]);
  const checks = { ...singleChecks(), scope: 'changed' };
  const pair = checkSnapshots({ base: empty.docs, revised: w.docs, profile: PROFILE, checkProfile: checks, geometry: { base: empty.tiles, revised: w.tiles }, engine });
  const strip = (f) => ({ ruleId: f.ruleId, correspondenceKey: f.correspondenceKey, evidence: f.evidence, evidenceFingerprint: f.evidenceFingerprint, ids: idsOf(f) });
  assert.deepEqual(one.findings.map(strip), pair.findings.map(strip), 'same pairs, same evidence, same keys and fingerprints');
  assert.ok(of(one, BOLTED).length === 1 && of(one, BOLTED)[0].evidence.gapMm === 3);
  assert.deepEqual(one.scope.solids, pair.scope.solids);
});

test('the output is byte-identical when run twice and when every document is in another order', () => {
  const parts = [...seeded(), plate('G', 0, 10, { x: 1500 }), plate('H', 13, 23, { x: 1500 }), bolt('BG', ['G', 'H'])];
  const first = canonicalJson(single(parts));
  assert.equal(canonicalJson(single(parts)), first);
  assert.equal(canonicalJson(single(parts, { shuffle: true })), first);
});

// ─── tolerances and the report threshold ────────────────────────────────────────────────────────────

test('raising the contact tolerance from 0.1 to 3 mm turns a 2 mm gap into a touch, and back', () => {
  const parts = [plate('A', 0, 10), plate('B', 12, 22)];
  assert.deepEqual(single(parts).findings, []);
  const wide = single(parts, { checks: singleChecks({ geometry: { contactTolerance: 3 } }) });
  const [touch] = of(wide, CONTACT);
  assert.ok(touch, 'within 3 mm the two plates touch');
  assert.equal(touch.evidence.gapMm, 2);
  assert.equal(of(wide, OVERLAP).length, 0);
  assert.equal(wide.evaluator.geometry.contactTolerance, 3);
  assert.equal(wide.evaluator.geometry.minOverlap, 3, 'minOverlap defaults to the contact tolerance');
});

test('contact tolerance boundaries (0.125 mm): a gap at, under and over it; an overlap at, under and over it', () => {
  const checks = singleChecks({ geometry: { contactTolerance: 0.125 } });
  const at = (z) => single([plate('A', 0, 10), plate('B', z, z + 10)], { checks });
  const gap = (g) => at(10 + g);
  assert.equal(of(gap(0.125), CONTACT).length, 1, 'a gap equal to the tolerance touches');
  assert.equal(of(gap(0.1171875), CONTACT).length, 1, 'just under: touches');
  assert.deepEqual(gap(0.1328125).findings, [], 'just over: apart');
  const into = (d) => at(10 - d);
  const exactly = into(0.125);
  assert.equal(of(exactly, OVERLAP).length, 0, 'an overlap as thick as the tolerance is not interpenetration');
  assert.equal(of(exactly, CONTACT)[0].evidence.gapMm, 0, 'it is contact');
  assert.equal(of(into(0.1171875), OVERLAP).length, 0);
  assert.equal(of(into(0.1171875), CONTACT).length, 1);
  const over = into(0.1328125);
  assert.equal(of(over, OVERLAP)[0].evidence.overlapThicknessMm, 0.13);
  assert.equal(of(over, CONTACT).length, 0, 'interpenetration is never also an unjoined contact');
});

test('minOverlap hides an overlap without making it a contact: counted below the threshold, at, under and over 0.5 mm', () => {
  const checks = singleChecks({ geometry: { contactTolerance: 0.125, minOverlap: 0.5 } });
  const into = (d) => single([plate('A', 0, 10), plate('B', 10 - d, 20 - d)], { checks });
  for (const [depth, listed] of [[0.5, false], [0.4921875, false], [0.25, false], [0.5078125, true], [5, true]]) {
    const out = into(depth);
    const stat = out.scope.byRule[OVERLAP];
    assert.equal(of(out, OVERLAP).length, listed ? 1 : 0, `depth ${depth}`);
    assert.equal(of(out, CONTACT).length, 0, `depth ${depth}: a penetration below the report threshold never turns into a contact finding`);
    assert.deepEqual(out.scope.byRule[CONTACT].notEvaluated, {}, `depth ${depth}: contact is answered by the overlap verdict, not left unjudged`);
    assert.equal(stat.evaluated, 1, `depth ${depth}: measured either way`);
    assert.deepEqual(stat.belowThreshold, listed ? { count: 0, maxThicknessMm: null } : { count: 1, maxThicknessMm: Math.round(depth * 100) / 100 }, `depth ${depth}`);
    assert.equal(out.unchecked.some((line) => /overlap\(s\) no thicker than 0.5 mm/.test(line)), !listed, `depth ${depth}`);
    assert.deepEqual(out.scope.byRule[OVERLAP].coverage, { revisedComplete: true, baseComplete: null }, 'below the threshold is measured, not unjudged');
  }
  assert.equal(of(into(0.5078125), OVERLAP)[0].evidence.overlapThicknessMm, 0.51);
  const two = single([plate('A', 0, 10), plate('B', 9.75, 19.75), plate('C', 0, 10, { x: 500 }), plate('D', 9.625, 19.625, { x: 500 })], { checks });
  assert.deepEqual(two.scope.byRule[OVERLAP].belowThreshold, { count: 2, maxThicknessMm: 0.38 }, 'the thickest of them: 0.375');
  assert.match(two.unchecked.join('\n'), /2 overlap\(s\) no thicker than 0\.5 mm \(the thickest 0\.38 mm\) were measured and are not listed/);
  assert.equal(two.evaluator.geometry.minOverlap, 0.5);
});

test('minOverlap: joined and unjoined parts, and a contained part, classify as they did', () => {
  const checks = singleChecks({ geometry: { contactTolerance: 0.125, minOverlap: 2 } });
  const inner = single([plate('A', 0, 100), { guid: 'B', type: 'Beam', solid: box([30, 30, 30], [31, 60, 70]) }], { checks });
  assert.equal(of(inner, OVERLAP).length, 0, 'a contained part 1 mm thin is below 2 mm');
  assert.equal(of(inner, CONTACT).length, 0);
  assert.equal(inner.scope.byRule[OVERLAP].belowThreshold.count, 1);
  const thick = single([plate('A', 0, 100), { guid: 'B', type: 'Beam', solid: box([30, 30, 30], [50, 60, 70]) }], { checks });
  assert.equal(of(thick, OVERLAP)[0].evidence.kind, 'contained');
  // A bolted pair that overlaps 1 mm: still interpenetration (counted below the threshold), never a bolted contact.
  const bolted = single([plate('P1', 0, 10), plate('P2', 9, 19), bolt('BG', ['P1', 'P2'])], { checks });
  assert.deepEqual(bolted.findings, []);
  assert.equal(bolted.scope.byRule[OVERLAP].belowThreshold.count, 1);
  const unjoined = single([plate('P1', 0, 10), plate('P2', 9, 19)], { checks });
  assert.deepEqual(unjoined.findings, [], 'unjoined and overlapping by less than the threshold: no contact finding either');
});

test('a minOverlap smaller than the contact tolerance, zero or not a number is refused', () => {
  for (const bad of [0.05, 0, -1, '1']) refuses(() => single(seeded(), { checks: singleChecks({ geometry: { minOverlap: bad } }) }), 'profile-invalid', /minOverlap/);
  assert.equal(single(seeded(), { checks: singleChecks({ geometry: { minOverlap: 0.1 } }) }).scope.byRule[OVERLAP].belowThreshold.count, 0, 'equal to the tolerance is fine');
});

// ─── per-rule type pairs ────────────────────────────────────────────────────────────────────────────

test('per-rule pairs are asymmetric: overlap checks bolts against plates only, contact checks plates against plates only', () => {
  const pairs = { [OVERLAP]: [['Contour plate', 'Bolt group'], ['Bolt group', 'Contour plate']], [CONTACT]: [['Contour plate', 'Contour plate']] };
  const checks = singleChecks({ pairs });
  const parts = [
    plate('P1', 0, 10), plate('P2', 5, 15),                                          // plate-plate overlap: neither rule reports it
    plate('P3', 0, 10, { x: 500 }), { guid: 'BX', type: 'Bolt group', solid: box([540, 40, 5], [560, 60, 30]), bolted: ['P9'] },   // a bolt through a plate it does not name
    plate('P4', 0, 10, { x: 1000 }), plate('P5', 10, 20, { x: 1000 }),               // plate-plate touch: contact reports it
    plate('P6', 0, 10, { x: 1500 }), { guid: 'BY', type: 'Bolt group', solid: box([1540, 40, 10], [1560, 60, 30]), bolted: ['P8'] },  // a bolt touching a plate
    plate('P8', 0, 10, { x: 3000 }), plate('P9', 0, 10, { x: 4000 }),
  ];
  const out = single(parts, { checks });
  assert.deepEqual(out.findings.map((f) => `${f.ruleId}:${idsOf(f).join('+')}`), [`${OVERLAP}:${id('BX')}+${id('P3')}`, `${CONTACT}:${id('P4')}+${id('P5')}`]);
  // P1-P2 and P4-P5 are plate pairs the overlap rule leaves out; BY-P6 (and BX-P3, whose contact the overlap answers) are bolt pairs contact leaves out.
  assert.deepEqual(out.scope.byRule[OVERLAP].notEvaluated, { 'pair-excluded': 2 });
  assert.equal(out.scope.byRule[OVERLAP].coverage.revisedComplete, false, 'a pair a rule does not check is not judged');
  assert.deepEqual(out.scope.byRule[CONTACT].notEvaluated, { 'pair-excluded': 2, interpenetrates: 1 }, 'P1-P2 overlap with the overlap rule not checking it: contact cannot judge them');
  assert.deepEqual(out.evaluator.geometry.pairs, { [OVERLAP]: [['Bolt group', 'Contour plate']], [CONTACT]: [['Contour plate', 'Contour plate']] }, 'normalised: each pair sorted, duplicates removed');
  const lines = out.unchecked.join('\n');
  assert.match(lines, new RegExp(`Rule '${OVERLAP}' does not check Beam with Beam, .*Contour plate with Contour plate.*: the check profile leaves these type pairs out \\(2 nearby pair\\(s\\) not checked\\)`));
  assert.match(lines, new RegExp(`Rule '${CONTACT}' does not check .*Bolt group with Contour plate`));
  assert.equal(out.scope.notEvaluated.byReason['pair-excluded'], undefined, 'a left-out type pair is the profile\'s choice, counted per rule, not a pair the check failed on');
});

test('a type pair left out of every pair rule never uses the pair budget; the bolted-plies rule ignores pair policies', () => {
  const checks = singleChecks({ ...withBudgets({ pairs: 1 }), pairs: { [OVERLAP]: [['Beam', 'Contour plate']], [CONTACT]: [['Beam', 'Contour plate']] } });
  const parts = [
    plate('P1', 0, 10), plate('P2', 13, 23), bolt('BG', ['P1', 'P2']),            // plates 3 mm apart: the stack is split
    plate('Q1', 0, 10, { x: 500 }), plate('Q2', 5, 15, { x: 500 }),                // a plate pair left out of both rules
    { guid: 'M', type: 'Beam', solid: box([1000, 0, 0], [1100, 100, 10]) }, plate('Q3', 5, 15, { x: 1000 }),
  ];
  const out = single(parts, { checks });
  assert.deepEqual(out.findings.map((f) => `${f.ruleId}:${idsOf(f).join('+')}`), [`${BOLTED}:${id('BG')}+${id('P2')}`, `${OVERLAP}:${id('M')}+${id('Q3')}`]);
  assert.equal(out.scope.notEvaluated.byReason['pair-budget'], undefined, 'Q1-Q2 did not take the one budgeted pair from M-Q3');
  assert.equal(out.scope.byRule[BOLTED].evaluated, 1);
});

test('appliesTo.pairs is validated: [] , a type outside types, a malformed pair and a non-pair rule are refused', () => {
  const go = (pairs, rule = OVERLAP) => () => single(seeded(), { checks: singleChecks({ pairs: { [rule]: pairs } }) });
  refuses(go([]), 'profile-invalid', /appliesTo.pairs must be a non-empty array/);
  refuses(go([['Contour plate', 'Wall']]), 'profile-invalid', /names 'Wall', which is not in appliesTo.types/);
  refuses(go([['Contour plate']]), 'profile-invalid', /must be two non-empty type strings/);
  refuses(go('Contour plate'), 'profile-invalid', /non-empty array/);
  refuses(go([['Bolt group', 'Bolt group']], BOLTED), 'profile-invalid', /allowed only for the clash-overlap and clash-contact predicates/);
  const blank = singleChecks();
  blank.rules = [{ id: 'b', version: '1', severity: 'warning', predicate: 'blank', target: 'entity.name', appliesTo: { types: ['Beam'], pairs: [['Beam', 'Beam']] }, worsening: null }];
  refuses(() => checkSnapshots({ base: null, revised: world([]).docs, profile: PROFILE, checkProfile: blank, mode: 'single' }), 'profile-invalid', /allowed only for the clash-overlap and clash-contact/);
});

test('per-rule pairs apply the same way in pair mode, and its re-checks call a left-out pair not applicable', () => {
  const pairs = { [OVERLAP]: [['Beam', 'Contour plate']], [CONTACT]: [['Beam', 'Contour plate']] };
  const checks = clashChecks();
  checks.rules = checks.rules.map((rule) => (pairs[rule.id] ? { ...rule, appliesTo: { ...rule.appliesTo, pairs: pairs[rule.id] } } : rule));
  const b = world([]); const r = world([plate('A', 0, 10), plate('B', 5, 15)]);
  const out = checkSnapshots({ base: b.docs, revised: r.docs, profile: PROFILE, checkProfile: checks, geometry: { base: b.tiles, revised: r.tiles }, engine, focus: { identities: [], sets: [[id('A'), id('B')]] } });
  assert.deepEqual(out.findings, []);
  assert.equal(out.scope.byRule[OVERLAP].pairExcluded, 1, 'a v1 document counts them beside the v1 counts');
  assert.equal(out.scope.byRule[OVERLAP].notEvaluated, 0);
  assert.deepEqual(out.focusSets.filter((row) => row.ruleId !== BOLTED).map((row) => row.state), ['not-applicable', 'not-applicable']);
});

// ─── the focus ──────────────────────────────────────────────────────────────────────────────────────

// A bolt group BG joins P1 and P2 (3 mm apart). P1 touches R (unjoined). Far away, Q1 and Q2 overlap.
const connection = () => [
  plate('P1', 0, 10), plate('P2', 13, 23), bolt('BG', ['P1', 'P2']),
  plate('R', 0, 10, { x: 100 }),
  plate('Q1', 0, 10, { x: 2000 }), plate('Q2', 5, 15, { x: 2000 }),
];

test('a focus on a bolt checks its plies, its stack and what touches them; nothing else, and says so', () => {
  const focus = { identities: [id('BG')] };
  const out = single(connection(), { focus });
  assert.deepEqual(out.findings.map((f) => `${f.ruleId}:${idsOf(f).join('+')}`), [`${BOLTED}:${id('BG')}+${id('P2')}`, `${CONTACT}:${id('P1')}+${id('R')}`]);
  assert.deepEqual(out.scope.focus, { requested: 1, effective: 3, unknown: [id('BG')] }, 'BG and the plies it pulls in; BG itself has no solid here, so it is reported, never silently passed');
  assert.match(out.unchecked.join('\n'), /Only pairs that involve the 1 selected object\(s\), their connection parts, or anything touching them were checked \(3 object\(s\) in all\)\./);
  assert.equal(out.inputs.focusDigest, createHash('sha256').update(canonicalJson(focus), 'utf8').digest('hex'));
  for (const ruleId of [OVERLAP, CONTACT, BOLTED]) assert.equal(out.scope.byRule[ruleId].evaluatedScope, 'focus');
  const whole = single(connection());
  assert.ok(of(whole, OVERLAP).some((f) => idsOf(f).includes(id('Q1'))), 'the whole model does find Q1-Q2');
});

test('a focus on a member pulls in the bolt group that names it and that bolt group\'s other plies', () => {
  const out = single(connection(), { focus: { identities: [id('P2')] } });
  assert.deepEqual(out.findings.map((f) => `${f.ruleId}:${idsOf(f).join('+')}`), [`${BOLTED}:${id('BG')}+${id('P2')}`, `${CONTACT}:${id('P1')}+${id('R')}`]);
  assert.deepEqual(out.scope.focus, { requested: 1, effective: 2, unknown: [] }, 'P2 and P1 (the stack is checked; the bolt group itself has no solid)');
  const r = single(connection(), { focus: { identities: [id('R')] } });
  assert.deepEqual(r.findings.map((f) => `${f.ruleId}:${idsOf(f).join('+')}`), [`${CONTACT}:${id('P1')}+${id('R')}`], 'R alone: what touches it, not the stack it is not in');
  assert.deepEqual(r.scope.focus, { requested: 1, effective: 1, unknown: [] });
});

test('a focus naming an unknown identity, or one with no solid, reports it as unknown and counts it as no-geometry, never drops it', () => {
  const parts = [...connection(), { guid: 'N', type: 'Contour plate', coverage: openCheck('geometry-reconstruction', 'part-geometry-missing') }];
  const out = single(parts, { focus: { identities: [id('NOPE'), id('N'), id('R')] } });
  assert.deepEqual(out.scope.focus, { requested: 3, effective: 2, unknown: [id('N'), id('NOPE')] });
  assert.equal(out.scope.solids.noGeometry, 2);
  assert.deepEqual(out.scope.solids.byReason, { 'not-in-snapshot': 1, 'part-geometry-missing': 1 });
  assert.equal(out.scope.solids.total, 3, 'R, N and the identity that is not there: the focus closure, not its neighbours');
  assert.match(out.unchecked.join('\n'), /2 part\(s\) could not be checked for clash because their solid could not be built \(not-in-snapshot: 1, part-geometry-missing: 1\)/);
  const empty = single(parts, { focus: { identities: [] } });
  assert.deepEqual(empty.findings, []);
  assert.deepEqual(empty.scope.focus, { requested: 0, effective: 0, unknown: [] });
  assert.match(empty.unchecked.join('\n'), /Only pairs that involve the 0 selected object\(s\)/, 'an empty focus says it checked nothing, it never reads as clean');
});

test('single mode takes a focus as { identities } only, and refuses an earlier state\'s inputs', () => {
  refuses(() => single(connection(), { focus: { identities: [id('BG')], sets: [[id('P1'), id('P2')]] } }), 'focus-invalid', /exactly \{ identities/);
  refuses(() => single(connection(), { focus: { identities: [id('BG'), id('BG')] } }), 'focus-invalid', /twice/);
  const w = world(connection());
  refuses(() => checkSnapshots({ base: null, revised: w.docs, profile: PROFILE, checkProfile: singleChecks(), geometry: { base: w.tiles, revised: w.tiles }, engine, mode: 'single' }), 'input-invalid', /only the later state's geometry/);
  refuses(() => checkSnapshots({ base: null, revised: w.docs, profile: PROFILE, checkProfile: singleChecks(), mode: 'single' }), 'input-invalid', /only with the later state's geometry/);
  refuses(() => checkSnapshots({ base: null, revised: w.docs, profile: PROFILE, checkProfile: singleChecks(), geometry: { revised: w.tiles }, mode: 'single' }), 'engine-unavailable');
  refuses(() => checkSnapshots({ base: null, revised: w.docs, profile: PROFILE, checkProfile: { ...singleChecks(), scope: 'changed' }, geometry: { revised: w.tiles }, engine, mode: 'single' }), 'profile-invalid', /scope 'all'/);
  refuses(() => checkSnapshots({ base: null, revised: w.docs, profile: PROFILE, checkProfile: clashChecks(), geometry: { revised: w.tiles }, engine, mode: 'single' }), 'profile-invalid', /v2/);
});

// ─── budgets: pairs, triangles, the list ────────────────────────────────────────────────────────────

/** `n` 10 mm plates stacked 1 mm apart in z: every pair of them overlaps, so n(n-1)/2 candidate pairs. */
const dense = (n) => Array.from({ length: n }, (_, i) => plate(`D${String(i).padStart(2, '0')}`, i, i + 10));

test('a dense model over the pair budget: the first pairs in a fixed order are checked, the rest counted as pair-budget', () => {
  const n = 24;
  const unbounded = single(dense(n));
  const candidates = of(unbounded, OVERLAP).length + of(unbounded, CONTACT).length;
  assert.ok(candidates > 100, `a dense model (${candidates} candidate pairs)`);
  const limit = 40;
  const out = single(dense(n), { checks: singleChecks(withBudgets({ pairs: limit })) });
  assert.equal(of(out, OVERLAP).length + of(out, CONTACT).length, limit, 'exactly the budget is measured');
  assert.equal(out.scope.notEvaluated.byReason['pair-budget'], candidates - limit, 'every other candidate is counted, none is lost');
  assert.equal(out.scope.notEvaluated.pairs, candidates - limit);
  for (const ruleId of [OVERLAP, CONTACT]) assert.equal(out.scope.byRule[ruleId].notEvaluated['pair-budget'], candidates - limit);
  const firstPairs = [...unbounded.findings].map(idsOf).sort((p, q) => (p.join('\u0000') < q.join('\u0000') ? -1 : 1)).slice(0, limit);
  assert.deepEqual(out.findings.map(idsOf).sort((p, q) => (p.join('\u0000') < q.join('\u0000') ? -1 : 1)), firstPairs, 'the kept pairs are the first by identity, whatever the sweep met first');
  assert.equal(out.complete, true, 'a budget is not a truncated list');
  assert.match(out.unchecked.join('\n'), new RegExp(`${candidates - limit} candidate pair\\(s\\) were not checked: the check stops at ${limit} pair\\(s\\), taken in a fixed order`));
  assert.equal(canonicalJson(single(dense(n), { checks: singleChecks(withBudgets({ pairs: limit })), shuffle: true })), canonicalJson(out), 'the same pairs in any input order');
});

test('triangle budgets: an object over its limit and a pair over its limit are triangle-budget, not pair-budget', () => {
  const object = single(seeded(), { checks: singleChecks(withBudgets({ trianglesPerObject: 11 })) });
  assert.deepEqual(object.findings, []);
  assert.equal(object.scope.solids.overBudget, 6);
  assert.deepEqual(object.scope.solids.byReason, { 'triangle-budget': 6 });
  assert.deepEqual(object.scope.notEvaluated.byReason, { 'triangle-budget': 2 }, 'A-B and C-D');
  assert.equal(object.scope.byRule[OVERLAP].notEvaluated['triangle-budget'], 2);
  const pair = single(seeded(), { checks: singleChecks(withBudgets({ trianglePairsPerPair: 143 })) });
  assert.deepEqual(pair.findings, []);
  assert.equal(pair.scope.solids.overBudget, 0);
  assert.deepEqual(pair.scope.notEvaluated.byReason, { 'triangle-budget': 2 });
  assert.match(pair.unchecked.join('\n'), /2 pair\(s\) were too large to check within the limits/);
  assert.equal(single(seeded(), { checks: singleChecks(withBudgets({ trianglePairsPerPair: 144 })) }).findings.length, 2, '12 x 12 triangles is within 144');
});

test('a list over the findings budget is truncated and says so; the pair and triangle budgets are untouched', () => {
  const out = single(dense(6), { checks: singleChecks({ findings: 2 }) });
  assert.equal(out.findings.length, 2);
  assert.equal(out.complete, false);
  assert.equal(out.truncated.omitted.findings, out.summary.findings - 2);
  assert.deepEqual(out.scope.notEvaluated.byReason, {});
  assert.match(out.unchecked.join('\n'), /Partial list: \d+ finding\(s\) are not listed/);
});

// ─── through the real entry points ──────────────────────────────────────────────────────────────────

const spawn = (entry, input) => spawnSync(process.execPath, [join(here, entry), 'check-snapshots'], { input: JSON.stringify(input), encoding: 'utf8', cwd: here });

function writeWorld(root, parts) {
  const w = world(parts);
  const revised = join(root, 'revised');
  mkdirSync(revised, { recursive: true });
  for (const [name, doc] of Object.entries(w.docs)) writeFileSync(join(revised, `${name}.json`), JSON.stringify(doc));
  const geo = join(root, 'revised-geometry');
  mkdirSync(geo, { recursive: true });
  for (const tile of w.tiles.tiles) writeFileSync(join(geo, tile.file), tile.bytes);
  writeFileSync(join(geo, 'geometry.json'), JSON.stringify({ tiles: w.tiles.tiles.map(({ ordinal, file, sha256 }) => ({ ordinal, file, sha256 })) }));
  writeFileSync(join(root, 'profile.json'), JSON.stringify(PROFILE));
  writeFileSync(join(root, 'base.json'), JSON.stringify({}));
}

test('pair mode: an overlap that shrinks below minOverlap is resolved, and its re-check passes saying it still overlaps', () => {
  const checks = clashChecks({ geometry: { contactTolerance: 0.125, minOverlap: 0.5 } });
  const b = world([plate('A', 0, 10), plate('B', 5, 15)]); const r = world([plate('A', 0, 10), plate('B', 9.75, 19.75)]);
  const out = checkSnapshots({ base: b.docs, revised: r.docs, profile: PROFILE, checkProfile: checks, geometry: { base: b.tiles, revised: r.tiles }, engine, focus: { identities: [id('B')], sets: [[id('A'), id('B')]] } });
  assert.deepEqual(out.findings, []);
  assert.deepEqual(out.resolvedInChange.map((f) => `${f.ruleId}:${f.evidence.overlapThicknessMm}`), [`${OVERLAP}:5`]);
  assert.deepEqual(out.scope.byRule[OVERLAP].belowThreshold, { count: 1, maxThicknessMm: 0.25 });
  const row = out.focusSets.find((s) => s.ruleId === OVERLAP);
  assert.equal(row.state, 'passed', 'a re-check speaks only the documented states');
  assert.equal(row.reason, 'the parts overlap by 0.25 mm, no thicker than the report threshold (minOverlap 0.5 mm)');
  assert.ok(out.focusSets.every((s) => ['finding', 'passed', 'not-evaluated', 'absent', 'excluded', 'not-applicable'].includes(s.state)));
});

test('a requested bolt group with no solid is reported unknown even when only the bolted-plies rule runs', () => {
  const out = single(connection(), { checks: singleChecks({ rules: ['bolted'] }), focus: { identities: [id('BG')] } });
  assert.deepEqual(out.scope.focus.unknown, [id('BG')]);
  assert.equal(out.scope.solids.noGeometry, 1);
  assert.equal(of(out, BOLTED).length, 1);
});

for (const entry of ['index.mjs', 'model-dispatcher.mjs']) {
  test(`${entry} check-snapshots mode single with clash: the later state's geometry and a focus in, present findings out`, () => {
    const root = mkdtempSync(join(tmpdir(), 'snapshot-clash-single-'));
    try {
      writeWorld(root, connection());
      writeFileSync(join(root, 'checks.json'), JSON.stringify(singleChecks({ geometry: { minOverlap: 0.5 } })));
      writeFileSync(join(root, 'focus.json'), JSON.stringify({ identities: [id('BG')] }));
      const request = {
        mode: 'single', 'revised-dir': join(root, 'revised'), 'profile-path': join(root, 'profile.json'),
        'check-profile-path': join(root, 'checks.json'), 'revised-geometry-dir': join(root, 'revised-geometry'),
      };
      const whole = spawn(entry, request);
      assert.equal(whole.status, 0, whole.stderr);
      assert.equal(whole.stdout, canonicalJson(JSON.parse(whole.stdout)), 'canonical bytes');
      assert.equal(spawn(entry, request).stdout, whole.stdout, 'byte-identical on a second run');
      const out = JSON.parse(whole.stdout);
      assert.equal(out.mode, 'single');
      assert.equal(out.evaluator.geometry.minOverlap, 0.5);
      assert.deepEqual(out.findings.map((f) => `${f.ruleId}:${f.presence}`).sort(), [`${BOLTED}:present`, `${OVERLAP}:present`, `${CONTACT}:present`]);
      const focused = JSON.parse(spawn(entry, { ...request, 'focus-path': join(root, 'focus.json') }).stdout);
      assert.deepEqual(focused.scope.focus, { requested: 1, effective: 3, unknown: [id('BG')] });
      assert.equal(focused.findings.some((f) => f.ruleId === OVERLAP), false);
      const refused = (patch, pattern) => {
        const bad = spawn(entry, { ...request, ...patch });
        assert.notEqual(bad.status, 0, JSON.stringify(patch));
        assert.equal(bad.stdout, '', JSON.stringify(patch));
        assert.match(bad.stderr, pattern, JSON.stringify(patch));
      };
      refused({ 'base-geometry-dir': join(root, 'revised-geometry') }, /`base-geometry-dir` is not accepted in single mode/);
      refused({ 'base-dir': join(root, 'revised') }, /`base-dir` is not accepted in single mode/);
      refused({ 'revised-geometry-dir': undefined }, /`revised-geometry-dir` is required in single mode/);
      writeFileSync(join(root, 'sets.json'), JSON.stringify({ identities: [], sets: [[id('P1'), id('P2')]] }));
      refused({ 'focus-path': join(root, 'sets.json') }, /exactly \{ identities/);
      // --validate-only in single mode accepts a clash profile and echoes the geometry it would measure with.
      const valid = spawn(entry, { 'validate-only': true, mode: 'single', 'check-profile-path': join(root, 'checks.json') });
      assert.equal(valid.status, 0, valid.stderr);
      assert.equal(JSON.parse(valid.stdout).evaluator.geometry.minOverlap, 0.5);
    } finally { rmSync(root, { recursive: true, force: true }); }
  });
}
