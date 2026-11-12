// Tests for clash in `check-snapshots` (model-change-set 0.3.0, plan 2026-11-12-change-review-s3-clash-PLAN.md §3.7).
// Synthetic closed solids are built here from numbers (boxes, an I-section extrusion, a bracket) and written to a real
// GLB tile; every expected number below is arithmetic on those numbers, not a snapshot of the code's own output.
import test from 'node:test';
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { canonicalJson, SnapshotCompareError } from './snapshot-compare.mjs';
import { PROFILE } from './snapshot-compare-fixtures.mjs';
import { checkSnapshots } from './snapshot-check.mjs';
import { loadClashEngine, readGeometryDir } from './snapshot-clash.mjs';
import { checkMesh, readTileMeshes, weld } from './snapshot-clash-geometry.mjs';
import {
  box, clashChecks, extrude, glb, sha, iSection, legacyScenario, openCheck, SOLID_TYPES, tileOf, translate, withFlippedTriangle,
  withoutTriangle, world,
} from './snapshot-clash-fixtures.mjs';

const engine = await loadClashEngine();
const id = (guid) => `lineage-A:${guid}`;
const OVERLAP = 'clash.interpenetration';
const CONTACT = 'clash.unjoined-contact';
const BOLTED = 'clash.bolted-plies-apart';

const plate = (guid, z0, z1, extra = {}) => ({ guid, type: 'Contour plate', solid: box([0, 0, z0], [100, 100, z1]), ...extra });
const bolt = (guid, plies, extra = {}) => ({ guid, type: 'Bolt group', bolted: plies, ...extra });

/** Run a check on two lists of parts. `hosts`/`contains` apply to both sides unless a side-specific list is given. */
function run(baseParts, revisedParts, { checks = clashChecks(), focus = null, hosts = [], contains = [], baseHosts = hosts, revisedHosts = hosts, shuffle = false, noEngine = false, qualificationFile = false } = {}) {
  const b = world(baseParts, { hosts: baseHosts, contains, qualificationFile });
  const r = world(revisedParts, { hosts: revisedHosts, contains, qualificationFile });
  const sh = (docs) => (shuffle ? {
    entities: { entities: [...docs.entities.entities].reverse() },
    properties: { properties: [...docs.properties.properties].reverse() },
    relationships: { relationships: [...docs.relationships.relationships].reverse() },
  } : docs);
  return checkSnapshots({
    base: sh(b.docs), revised: sh(r.docs), profile: PROFILE, checkProfile: checks, focus,
    geometry: { base: b.tiles, revised: r.tiles }, engine: noEngine ? null : engine,
  });
}
const of = (out, ruleId) => out.findings.filter((f) => f.ruleId === ruleId);
const idsOf = (finding) => finding.participants.map((p) => p.identity);
const row = (out, a, b, ruleId) => out.focusSets.find((s) => s.identities[0] === id(a) && s.identities[1] === id(b) && s.ruleId === ruleId);
const refuses = (fn, code, pattern) => assert.throws(fn, (error) => {
  assert.ok(error instanceof SnapshotCompareError, `expected a SnapshotCompareError, got ${error}`);
  assert.equal(error.code, code, error.message);
  if (pattern) assert.match(error.message, pattern);
  return true;
});

// ─── a profile without clash rules is untouched ─────────────────────────────────────────────────────

test('a profile with no clash rule gives the bytes it gave before clash existed, and carries no clash fields', () => {
  const { base, revised, checkProfile, focus } = legacyScenario();
  const out = checkSnapshots({ base, revised, profile: PROFILE, checkProfile, focus });
  const bytes = canonicalJson(out);
  // Captured from the pre-clash snapshot-check.mjs (git HEAD of this branch's parent) on this exact scenario.
  assert.equal(createHash('sha256').update(bytes).digest('hex'), '1ae02fdd4a305d2924785ff05698ade82e88c30acdbdb59006e0fc82bbe0e8d9');
  assert.equal(out.focusSets, undefined);
  assert.equal(out.inputs.geometry, undefined);
  assert.equal(out.truncated, null);
});

// ─── interpenetration: the measured number ──────────────────────────────────────────────────────────

test('a plate pushed 4.76 mm into another measures 4.76, whichever part is the one that moved', () => {
  const scenarios = [
    // B drops onto A from above
    [[plate('A', 0, 10), plate('B', 20, 30)], [plate('A', 0, 10), plate('B', 5.24, 15.24)]],
    // A rises into B from below
    [[plate('B', 0, 10), plate('A', -20, -10)], [plate('B', 0, 10), plate('A', -5.24, 4.76)]],
  ];
  for (const [base, revised] of scenarios) {
    const out = run(base, revised);
    const [f] = of(out, OVERLAP);
    assert.equal(of(out, OVERLAP).length, 1, JSON.stringify(out.findings.map((x) => x.ruleId)));
    assert.deepEqual(idsOf(f), [id('A'), id('B')]);
    assert.equal(f.evidence.overlapThicknessMm, 4.76);
    assert.equal(f.evidence.kind, 'overlap');
    assert.deepEqual(f.evidence.overlapExtentMm, [100, 100, 4.76]);
    assert.equal(f.evidence.regions, 1);
    assert.equal(f.evidence.overlapVolumeMm3, 47600);
    assert.equal(f.evidence.thicknessMethod, 'across-part-faces');
    assert.equal(f.evidence.approximate, false);
    assert.equal(f.presence, 'introduced');
    assert.equal(f.severity, 'error');
    assert.deepEqual(f.locus, {});
  }
});

test('an I-section through a 20 mm plate, no vertex of either inside the other, is 20.00', () => {
  const plateSolid = box([-100, -100, 0], [200, 200, 20]);
  const beam = iSection(50, 50, 100, 200, 10, 8, -100, 120);
  const out = run(
    [{ guid: 'P', type: 'Contour plate', solid: plateSolid }, { guid: 'IB', type: 'Beam', solid: translate(beam, [0, 0, -500]) }],
    [{ guid: 'P', type: 'Contour plate', solid: plateSolid }, { guid: 'IB', type: 'Beam', solid: beam }],
  );
  const [f] = of(out, OVERLAP);
  assert.equal(of(out, OVERLAP).length, 1);
  assert.equal(f.evidence.overlapThicknessMm, 20);
  assert.deepEqual(f.evidence.overlapExtentMm, [100, 200, 20]);
  assert.deepEqual(f.evidence.locationMm, [50, 50, 10]);
  assert.equal(f.evidence.kind, 'overlap');
});

test('identical solids are a coincident clash of their full thickness; a thinner one sharing a face is contained', () => {
  const same = run([plate('A', 0, 10)], [plate('A', 0, 10), { guid: 'B', type: 'Contour plate', solid: box([0, 0, 0], [100, 100, 10]) }]);
  const [f] = of(same, OVERLAP);
  assert.equal(f.evidence.kind, 'coincident');
  assert.equal(f.evidence.overlapThicknessMm, 10);
  assert.equal(f.evidence.overlapVolumeMm3, 100000);
  const half = run([plate('A', 0, 10)], [plate('A', 0, 10), { guid: 'B', type: 'Contour plate', solid: box([0, 0, 0], [100, 100, 5]) }]);
  const [g] = of(half, OVERLAP);
  assert.equal(g.evidence.kind, 'contained');
  assert.equal(g.evidence.overlapThicknessMm, 5);
});

test('a small solid entirely inside a bigger one is contained, as thick as its thinnest dimension', () => {
  const out = run([plate('A', 0, 100)], [plate('A', 0, 100), { guid: 'B', type: 'Beam', solid: box([30, 30, 30], [50, 60, 70]) }]);
  const [f] = of(out, OVERLAP);
  assert.equal(f.evidence.kind, 'contained');
  assert.equal(f.evidence.overlapThicknessMm, 20);
  assert.deepEqual(f.evidence.overlapExtentMm, [20, 30, 40]);
});

test('a plate stepped 2, 4, 6 mm into another measures 2, 4, 6', () => {
  const seen = [2, 4, 6].map((depth) => {
    const out = run([plate('A', 0, 10)], [plate('A', 0, 10), plate('B', 10 - depth, 20 - depth)]);
    return of(out, OVERLAP)[0].evidence.overlapThicknessMm;
  });
  assert.deepEqual(seen, [2, 4, 6]);
});

test('0.09 mm of overlap is contact, not a clash; 0.11 mm is a clash', () => {
  const touching = run([plate('A', 0, 10)], [plate('A', 0, 10), plate('B', 9.91, 19.91)]);
  assert.equal(of(touching, OVERLAP).length, 0);
  const [c] = of(touching, CONTACT);
  assert.equal(c.evidence.gapMm, 0);
  const clash = run([plate('A', 0, 10)], [plate('A', 0, 10), plate('B', 9.89, 19.89)]);
  assert.equal(of(clash, OVERLAP)[0].evidence.overlapThicknessMm, 0.11);
});

// ─── contact and gap ────────────────────────────────────────────────────────────────────────────────

test('two plates face to face touch (0.00); 0.05 mm apart still touch; 0.5 mm apart do not', () => {
  const faceToFace = run([plate('A', 0, 10)], [plate('A', 0, 10), plate('B', 10, 20)]);
  assert.equal(of(faceToFace, OVERLAP).length, 0);
  assert.equal(of(faceToFace, CONTACT)[0].evidence.gapMm, 0);
  assert.equal(of(faceToFace, CONTACT)[0].evidence.locationMm[2], 10);
  assert.equal(of(faceToFace, CONTACT)[0].severity, 'warning');
  const near = run([plate('A', 0, 10)], [plate('A', 0, 10), plate('B', 10.05, 20.05)]);
  assert.equal(of(near, CONTACT)[0].evidence.gapMm, 0.05);
  const apart = run([plate('A', 0, 10)], [plate('A', 0, 10), plate('B', 10.5, 20.5)]);
  assert.deepEqual(apart.findings, []);
  assert.equal(apart.scope.byRule[CONTACT].evaluated, 0, 'beyond the tolerance the pair is not even a candidate');
  assert.equal(apart.scope.notEvaluated.pairs, 0);
});

test('two boxes sharing one face with the rest outside touch at 0.00, with the location on the shared face', () => {
  const out = run([plate('A', 0, 10)], [plate('A', 0, 10), { guid: 'B', type: 'Contour plate', solid: box([100, 40, 0], [200, 60, 10]) }]);
  assert.equal(of(out, OVERLAP).length, 0);
  const [c] = of(out, CONTACT);
  assert.equal(c.evidence.gapMm, 0);
  assert.equal(c.evidence.locationMm[0], 100);
});

test('the gap between two solids is the exact surface distance, corner to corner as well as face to face', () => {
  const out = run(
    [plate('A', 0, 10), { guid: 'B', type: 'Contour plate', solid: box([200, 200, 200], [210, 210, 210]) }, bolt('BG', ['A', 'B'])],
    [plate('A', 0, 10), { guid: 'B', type: 'Contour plate', solid: box([101, 102, 12], [110, 110, 20]) }, bolt('BG', ['A', 'B'])],
  );
  // A's nearest corner is (100, 100, 10); B's is (101, 102, 12): 1, 2, 2 apart is a distance of 3.
  const [f] = of(out, BOLTED);
  assert.deepEqual(idsOf(f), [id('BG'), id('B')]);
  assert.equal(f.evidence.gapMm, 3);
  assert.deepEqual(f.evidence.locationMm, [100.5, 101, 11]);
});

test('two separate 0.05 mm overlaps far apart are not a clash (each region is measured alone)', () => {
  const bar = box([0, 0, 0], [1000, 100, 100]);
  const bracket = (tabStart) => extrude([[100, 99.95], [150, 99.95], [150, 110], [1050, 110], [1050, 60], [tabStart, 60], [tabStart, 40], [1100, 40], [1100, 130], [100, 130]], [0, 2, 1], 0, 100);
  const out = run([{ guid: 'BAR', type: 'Beam', solid: bar }], [{ guid: 'BAR', type: 'Beam', solid: bar }, { guid: 'BR', type: 'Item', solid: bracket(999.95) }]);
  assert.equal(of(out, OVERLAP).length, 0, JSON.stringify(of(out, OVERLAP)));
  assert.equal(of(out, CONTACT).length, 1);   // both lobes are contact
  assert.equal(out.scope.notEvaluated.byReason.interpenetrates, undefined);
});

test('one 0.05 mm overlap and one 3 mm overlap: one finding, 3.00, one region over the tolerance', () => {
  const bar = box([0, 0, 0], [1000, 100, 100]);
  const bracket = extrude([[100, 99.95], [150, 99.95], [150, 110], [1050, 110], [1050, 60], [997, 60], [997, 40], [1100, 40], [1100, 130], [100, 130]], [0, 2, 1], 0, 100);
  const out = run([{ guid: 'BAR', type: 'Beam', solid: bar }], [{ guid: 'BAR', type: 'Beam', solid: bar }, { guid: 'BR', type: 'Item', solid: bracket }]);
  const [f] = of(out, OVERLAP);
  assert.equal(of(out, OVERLAP).length, 1);
  assert.equal(f.evidence.overlapThicknessMm, 3);
  assert.equal(f.evidence.regions, 1);
  assert.deepEqual(f.evidence.overlapExtentMm, [3, 100, 20]);
  assert.equal(f.evidence.overlapVolumeMm3, 6000);
});

// ─── the quality of the evidence (G1) ───────────────────────────────────────────────────────────────

test('an open mesh and a flipped triangle are approximate with the reason named; a clean result is not-evaluated, never passed', () => {
  const closed = plate('B', 20, 30);
  for (const [label, broken, reason] of [['open', withoutTriangle(box([0, 0, 0], [100, 100, 10]), 3), 'mesh-open'], ['flipped', withFlippedTriangle(box([0, 0, 0], [100, 100, 10]), 3), 'mesh-orientation']]) {
    const out = run([], [{ guid: 'A', type: 'Contour plate', solid: broken }, closed]);
    assert.deepEqual(out.findings, [], label);
    assert.equal(out.scope.solids.approximate, 1, label);
    assert.deepEqual(out.scope.solids.byReason, { [reason]: 1 }, label);
    assert.equal(out.scope.notEvaluated.byReason['mesh-not-closed'] ?? 0, 0, 'far apart, so not even a candidate pair');
    // overlapping with the broken one: no boolean is possible, so the pair is not-evaluated and says why
    const overlapping = run([], [{ guid: 'A', type: 'Contour plate', solid: broken }, plate('B', 5, 15)]);
    assert.deepEqual(overlapping.findings, [], label);
    assert.deepEqual(overlapping.scope.notEvaluated.byReason, { 'mesh-not-closed': 1 }, label);
    assert.equal(overlapping.scope.byRule[OVERLAP].notEvaluated, 1);
    assert.equal(overlapping.scope.byRule[OVERLAP].evaluated, 0);
    assert.match(overlapping.unchecked.join('\n'), /could not be checked: mesh-not-closed: 1/);
  }
});

test('an open coverage check the profile names makes the solid approximate: findings say so, a clean result is not-evaluated', () => {
  const cuts = openCheck('operations-evaluated', 'cuts-not-applied');
  const clash = run([plate('A', 0, 10)], [plate('A', 0, 10), plate('B', 5, 15, { coverage: cuts })]);
  const [f] = of(clash, OVERLAP);
  assert.equal(f.evidence.approximate, true);
  assert.equal(f.evidence.overlapThicknessMm, 5);
  assert.deepEqual(clash.scope.solids.byReason, { 'coverage:operations-evaluated:cuts-not-applied': 1 });
  // 0.08 mm off in x and in y: the boxes are within the tolerance but the solids are 0.113 mm apart, so no contact
  const diagonal = (extra) => [plate('A', 0, 10), { guid: 'B', type: 'Contour plate', solid: box([100.08, 100.08, 0], [200, 200, 10]), ...extra }];
  const exact = run([], diagonal({}));
  assert.deepEqual(exact.findings, []);
  assert.equal(exact.scope.byRule[OVERLAP].evaluated, 1, 'on exact geometry the clean result is a pass');
  const clean = run([], diagonal({ coverage: cuts }));
  assert.deepEqual(clean.findings, []);
  assert.deepEqual(clean.scope.notEvaluated.byReason, { approximate: 1 });
  assert.equal(clean.scope.byRule[OVERLAP].evaluated, 0);
  assert.equal(clean.scope.byRule[CONTACT].evaluated, 0);
  const noCoverage = run([], [plate('A', 0, 10), plate('B', 10.5, 20.5, { coverage: null })]);
  assert.deepEqual(noCoverage.scope.solids.byReason, { 'coverage:missing': 1 });
});

test('a part with no node is no-geometry with the coverage code as the reason, and is listed as not checked', () => {
  const missing = { guid: 'C', type: 'Beam', coverage: openCheck('geometry-reconstruction', 'part-geometry-missing') };
  const out = run([plate('A', 0, 10)], [plate('A', 0, 10), missing, plate('B', 5, 15)]);
  assert.equal(out.scope.solids.noGeometry, 1);
  assert.equal(out.scope.solids.byReason['part-geometry-missing'], 1);
  assert.match(out.unchecked.join('\n'), /1 changed part\(s\) could not be checked for clash because their solid could not be built \(part-geometry-missing: 1\)/);
  assert.equal(of(out, OVERLAP).length, 1, 'the parts that do have solids are still checked');
});

test('a coverage check outside the pinned list leaves the solid evaluated; one inside it makes it approximate', () => {
  const uda = [...openCheck('operations-evaluated', 'ok', 'pass'), { name: 'uda-text-complete', state: 'open', code: 'uda-missing' }];
  const out = run([plate('A', 0, 10)], [plate('A', 0, 10), plate('B', 5, 15, { coverage: uda })]);
  assert.equal(out.scope.solids.approximate, 0);
  assert.equal(out.scope.solids.evaluated, 1);
  assert.equal(of(out, OVERLAP)[0].evidence.approximate, false);
  const placement = run([plate('A', 0, 10)], [plate('A', 0, 10), plate('B', 5, 15, { coverage: openCheck('placement-confirmed', 'placement-guessed') })]);
  assert.equal(placement.scope.solids.byReason['coverage:placement-confirmed:placement-guessed'], 1);
});

test('a row that lacks a whenPresent check still qualifies its solid; a present one must pass; a required one must be there', () => {
  const kindShape = (...names) => names.map((name) => ({ name, state: 'pass', code: 'ok' }));
  const part = kindShape('geometry-reconstruction', 'operations-evaluated');   // a part carries no recipes-evaluated or placement-confirmed
  const out = run([plate('A', 0, 10)], [plate('A', 0, 10), plate('B', 5, 15, { coverage: part })]);
  assert.equal(out.scope.solids.approximate, 0);
  assert.equal(of(out, OVERLAP)[0].evidence.approximate, false);
  const open = [{ name: 'geometry-reconstruction', state: 'pass', code: 'ok' }, { name: 'recipes-evaluated', state: 'open', code: 'recipes-skipped' }];
  const approx = run([plate('A', 0, 10)], [plate('A', 0, 10), plate('B', 5, 15, { coverage: open })]);
  assert.deepEqual(approx.scope.solids.byReason, { 'coverage:recipes-evaluated:recipes-skipped': 1 });
  assert.equal(of(approx, OVERLAP)[0].evidence.approximate, true);
  const noRequired = run([plate('A', 0, 10)], [plate('A', 0, 10), plate('B', 5, 15, { coverage: kindShape('operations-evaluated') })]);
  assert.deepEqual(noRequired.scope.solids.byReason, { 'coverage:geometry-reconstruction:missing': 1 });
});

// ─── what joins two parts ───────────────────────────────────────────────────────────────────────────

test('contact joined by a bolt group is expected; the same contact with nothing joining it is a warning', () => {
  const bolted = run([], [plate('P1', 0, 10), plate('P2', 10, 20), bolt('BG', ['P1', 'P2'])]);
  assert.deepEqual(of(bolted, CONTACT), []);
  assert.deepEqual(of(bolted, BOLTED), []);
  assert.equal(bolted.scope.byRule[BOLTED].evaluated, 1, 'a bolt group with no solid still anchors its plies');
  const loose = run([], [plate('P1', 0, 10), plate('P2', 10, 20)]);
  const [c] = of(loose, CONTACT);
  assert.deepEqual(idsOf(c), [id('P1'), id('P2')]);
  assert.equal(c.evidence.sameAssembly, false);
  assert.equal(c.evidence.sameComponent, false);
  assert.equal(c.evidence.approximate, false);
  assert.equal(c.presence, 'introduced');
});

test('the same assembly is not proof of a joint: it is shown in the evidence only', () => {
  const asm = { guid: 'ASM', type: 'Assembly' };
  const out = run([], [asm, plate('P1', 0, 10), plate('P2', 10, 20)], { hosts: [['ASM', 'P1'], ['ASM', 'P2']] });
  const [c] = of(out, CONTACT);
  assert.equal(c.evidence.sameAssembly, true);
  assert.equal(c.evidence.sameComponent, false);
  const comp = run([], [asm, plate('P1', 0, 10), plate('P2', 10, 20)], { contains: [['ASM', 'P1'], ['ASM', 'P2']] });
  assert.equal(of(comp, CONTACT)[0].evidence.sameComponent, true);
});

test('contact inside an assembly that holds a weld with unknown parts is not-evaluated (never a finding, never passed)', () => {
  const asm = { guid: 'ASM', type: 'Assembly' };
  const weld = { guid: 'W', type: 'Physical', solid: box([500, 500, 500], [510, 510, 510]) };   // no welded-parts property
  const out = run([], [asm, plate('P1', 0, 10), plate('P2', 10, 20), weld], { hosts: [['ASM', 'P1'], ['ASM', 'P2'], ['ASM', 'W']] });
  assert.deepEqual(of(out, CONTACT), []);
  assert.deepEqual(out.scope.notEvaluated.byReason, { 'weld-parts-unknown': 1 });
  assert.match(out.unchecked.join('\n'), /weld-parts-unknown: 1/);
  const known = run([], [asm, plate('P1', 0, 10), plate('P2', 10, 20), { ...weld, welded: ['P1', 'P2'] }], { hosts: [['ASM', 'P1'], ['ASM', 'P2'], ['ASM', 'W']] });
  assert.deepEqual(of(known, CONTACT), [], 'a weld that names both joins them');
  assert.deepEqual(known.scope.notEvaluated.byReason, {});
});

test('a weld overlapping a part it names is not-evaluated; the same weld overlapping another part is a clash', () => {
  const weld = { guid: 'W', type: 'Physical', solid: box([40, 40, 8], [60, 60, 12]), welded: ['P1'] };
  const out = run([], [plate('P1', 0, 10), plate('P3', 10, 20), weld]);
  const overlaps = of(out, OVERLAP);
  assert.equal(overlaps.length, 1);
  assert.deepEqual(idsOf(overlaps[0]), [id('P3'), id('W')]);
  assert.equal(overlaps[0].evidence.overlapThicknessMm, 2);
  assert.equal(out.scope.notEvaluated.byReason['own-joint'], 1);
  assert.match(out.unchecked.join('\n'), /a weld sits in the joint it makes; the joint is not checked/);
});

test('a bolt hardware solid against its own ply is not-evaluated; against another part it is checked', () => {
  const shaft = { guid: 'BG', type: 'Bolt group', solid: box([45, 45, 2], [55, 55, 8]), bolted: ['P1', 'P2'] };
  const out = run([], [plate('P1', 0, 10), plate('P2', 10, 20), shaft, { guid: 'P3', type: 'Contour plate', solid: box([50, 50, 0], [120, 120, 5]) }]);
  const overlaps = of(out, OVERLAP);
  const pairs = overlaps.map((f) => idsOf(f).join('+')).sort();
  assert.deepEqual(pairs, [`${id('BG')}+${id('P3')}`, `${id('P1')}+${id('P3')}`].sort());
  assert.equal(out.scope.notEvaluated.byReason['own-joint'], 1, 'BG against P1 (BG starts inside P1); P2 is not even a candidate');
  assert.match(out.unchecked.join('\n'), /a bolt passes through the parts it joins; the hole is not checked/);
});

// ─── bolted stacks ──────────────────────────────────────────────────────────────────────────────────

test('a bolted ply 3 mm off its mate is a finding with gapMm 3.00 between the bolt group and the detached ply', () => {
  const out = run([], [plate('P1', 0, 10), plate('P2', 13, 23), bolt('BG', ['P1', 'P2'])]);
  const [f] = of(out, BOLTED);
  assert.equal(of(out, BOLTED).length, 1);
  assert.deepEqual(idsOf(f), [id('BG'), id('P2')]);
  assert.equal(f.evidence.gapMm, 3);
  assert.equal(f.evidence.locationMm[2], 11.5);
  assert.equal(f.evidence.method, 'whole-ply-nearest');
  assert.equal(f.evidence.gapAtLeastMm, undefined);
  assert.equal(f.severity, 'warning');
  assert.deepEqual(f.locus, {});
});

test('in a three-ply stack with the third ply 2 mm off, only that ply is a finding even though the other two touch', () => {
  const out = run([], [plate('P1', 0, 10), plate('P2', 10, 20), plate('P3', 22, 32), bolt('BG', ['P1', 'P2', 'P3'])]);
  const found = of(out, BOLTED);
  assert.equal(found.length, 1);
  assert.deepEqual(idsOf(found[0]), [id('BG'), id('P3')]);
  assert.equal(found[0].evidence.gapMm, 2);
});

test('a detached group of three plies gives three findings, one per ply', () => {
  const out = run([], [plate('A1', 0, 10), plate('A2', 10, 20), plate('A3', 20, 30), plate('B1', 100, 110), plate('B2', 110, 120), bolt('BG', ['A1', 'A2', 'A3', 'B1', 'B2'])]);
  // main group = A1..A3 (3 plies); B1 and B2 are outside it, 70 mm away (inside the 50 mm search? no: beyond it)
  const found = of(out, BOLTED);
  assert.deepEqual(found.map((f) => idsOf(f)[1]).sort(), [id('B1'), id('B2')]);
  const b1 = found.find((f) => idsOf(f)[1] === id('B1'));
  assert.deepEqual(b1.evidence, { gapMm: null, gapAtLeastMm: 50, locationMm: null, method: 'whole-ply-nearest' });
});

test('a gap beyond the search is capped: capped to capped is pre-existing, exact to capped is changed, exact to exact moves', () => {
  const stack = (z) => [plate('P1', 0, 10), plate('P2', z, z + 10), bolt('BG', ['P1', 'P2'])];
  const withName = (parts, name) => parts.map((p) => (p.guid === 'P2' ? { ...p, name } : p));
  const cappedToCapped = run(withName(stack(90), 'x'), withName(stack(90), 'y'));
  assert.equal(of(cappedToCapped, BOLTED)[0].presence, 'pre-existing');
  assert.equal(of(cappedToCapped, BOLTED)[0].evidence.gapMm, null);
  const exactToCapped = run(stack(30), stack(90));
  assert.equal(of(exactToCapped, BOLTED)[0].presence, 'pre-existing-changed');
  const exactWorse = run(stack(30), stack(40));
  assert.equal(of(exactWorse, BOLTED)[0].presence, 'pre-existing-worsened');
  assert.equal(of(exactWorse, BOLTED)[0].evidence.gapMm, 30);
  const exactBetter = run(stack(40), stack(30));
  assert.equal(of(exactBetter, BOLTED)[0].presence, 'pre-existing-improved');
});

test('a stack with one ply that is not evaluated is not-evaluated as a whole, with the reason', () => {
  const missing = { guid: 'P3', type: 'Contour plate', coverage: openCheck('geometry-reconstruction', 'part-geometry-missing') };
  const out = run([], [plate('P1', 0, 10), plate('P2', 13, 23), missing, bolt('BG', ['P1', 'P2', 'P3'])]);
  assert.deepEqual(of(out, BOLTED), [], 'a missing solid could be the ply that closes the gap');
  assert.deepEqual(out.scope.notEvaluated.byReason, { 'no-geometry': 1 });
  assert.equal(out.scope.notEvaluated.stacks, 1);
  assert.equal(out.scope.byRule[BOLTED].notEvaluated, 1);
});

test('a ply token that matches no part makes the stack uncertain and not-evaluated', () => {
  const out = run([], [plate('P1', 0, 10), plate('P2', 13, 23), bolt('BG', ['P1', 'P2', 'ZZZ'])]);
  assert.deepEqual(of(out, BOLTED), []);
  assert.deepEqual(out.scope.notEvaluated.byReason, { 'join-uncertain': 1 });
});

test('plies named in a different letter case still match the identity', () => {
  const out = run([], [plate('P1', 0, 10), plate('P2', 13, 23), bolt('BG', ['p1', 'p2'])]);
  assert.equal(of(out, BOLTED)[0].evidence.gapMm, 3);
});

test('a contact where a bolt or weld names a part that is not in the saved state is not-evaluated, not a finding', () => {
  const out = run([], [plate('P1', 0, 10), plate('P2', 10, 20), plate('P4', 100, 110), bolt('BG', ['P1', 'P4', 'ZZZ'])]);
  assert.deepEqual(of(out, CONTACT), []);
  assert.deepEqual(out.scope.notEvaluated.byReason['join-uncertain'] >= 1, true);
});

// ─── the dependency closure ─────────────────────────────────────────────────────────────────────────

test('a pair of two unchanged parts is not checked, and the result says so', () => {
  const still = [plate('P1', 0, 10), plate('P2', 5, 15), plate('P9', 500, 510)];
  const out = run(still, [...still.slice(0, 2), { ...still[2], name: 'renamed' }]);
  assert.deepEqual(out.findings, []);
  assert.match(out.unchecked.join('\n'), /pairs of two unchanged parts were not checked/);
});

test('adding a bolt group puts its unmoved plies in scope: their 3 mm gap is found although neither ply changed', () => {
  const plies = [plate('P1', 0, 10), plate('P2', 13, 23)];
  const out = run(plies, [...plies, bolt('BG', ['P1', 'P2'])]);
  const [f] = of(out, BOLTED);
  assert.equal(f.evidence.gapMm, 3);
  assert.equal(f.presence, 'introduced');
  assert.equal(out.scope.added, 1);
  assert.equal(out.scope.changed, 0);
});

test('removing a bolt group puts its plies in scope: their contact is now unjoined', () => {
  const plies = [plate('P1', 0, 10), plate('P2', 10, 20)];
  const out = run([...plies, bolt('BG', ['P1', 'P2'])], plies);
  const [c] = of(out, CONTACT);
  assert.deepEqual(idsOf(c), [id('P1'), id('P2')]);
  assert.equal(c.presence, 'introduced', 'the pair was joined before, so the contact is new');
});

test('a bolt group whose plies are unchanged is re-checked when one ply moves', () => {
  const base = [plate('P1', 0, 10), plate('P2', 10, 20), plate('P3', 20, 30), bolt('BG', ['P1', 'P2', 'P3'])];
  const revised = [plate('P1', 0, 10), plate('P2', 10, 20), plate('P3', 24, 34), bolt('BG', ['P1', 'P2', 'P3'])];
  const out = run(base, revised);
  const [f] = of(out, BOLTED);
  assert.deepEqual(idsOf(f), [id('BG'), id('P3')]);
  assert.equal(f.evidence.gapMm, 4);
  assert.equal(f.presence, 'introduced');
  assert.equal(out.scope.changed, 1);
});

test('a pair that overlapped only in the earlier state is evaluated and listed as resolved', () => {
  const out = run([plate('A', 0, 10), plate('B', 5, 15)], [plate('A', 0, 10), plate('B', 60, 70)]);
  assert.deepEqual(out.findings, []);
  assert.equal(out.resolvedInChange.length, 1);
  assert.equal(out.resolvedInChange[0].ruleId, OVERLAP);
  assert.equal(out.resolvedInChange[0].evidence.overlapThicknessMm, 5);
  assert.deepEqual(out.resolvedInChange[0].participants.map((p) => p.identity), [id('A'), id('B')]);
  assert.equal(out.summary.resolvedInChange, 1);
});

// ─── presence ───────────────────────────────────────────────────────────────────────────────────────

test('a clash pushed deeper is worsened, pulled back is improved, and one that stays put is pre-existing', () => {
  const stage = (depth, nameB = 'b') => [plate('A', 0, 10), plate('B', 10 - depth, 20 - depth, { name: nameB })];
  assert.equal(of(run(stage(2), stage(5)), OVERLAP)[0].presence, 'pre-existing-worsened');
  assert.equal(of(run(stage(5), stage(2)), OVERLAP)[0].presence, 'pre-existing-improved');
  assert.equal(of(run(stage(5), stage(5, 'renamed')), OVERLAP)[0].presence, 'pre-existing');
  const cleared = run(stage(5), stage(-30));
  assert.deepEqual(cleared.findings, []);
  assert.equal(cleared.resolvedInChange.length, 1);
});

test('the same clash keeps one correspondence key as it moves: one finding per pair and rule', () => {
  const stage = (depth) => [plate('A', 0, 10), plate('B', 10 - depth, 20 - depth)];
  const a = of(run([], stage(2)), OVERLAP)[0];
  const b = of(run([], stage(6)), OVERLAP)[0];
  assert.equal(a.correspondenceKey, b.correspondenceKey);
  assert.notEqual(a.evidenceFingerprint, b.evidenceFingerprint);
});

// ─── re-checks for pairs ────────────────────────────────────────────────────────────────────────────

test('focusSets answers each state, and passed only for a definitive clean pair', () => {
  const missing = { guid: 'N', type: 'Beam', coverage: openCheck('geometry-reconstruction', 'part-geometry-missing') };
  const base = [plate('A', 0, 10), plate('B', 5, 15), plate('C', 200, 210), plate('D', 5, 15), plate('GONE', 400, 410), missing];
  const revised = [plate('A', 0, 10), plate('B', 5, 15), plate('C', 200, 210), plate('D', 300, 310), missing];
  const out = run(base, revised, { focus: { identities: [id('A'), id('B'), id('C'), id('D'), id('N')], sets: [[id('A'), id('B')], [id('A'), id('C')], [id('A'), id('D')], [id('A'), id('N')], [id('A'), id('GONE')], [id('B'), id('A')]].slice(0, 5) } });
  assert.equal(row(out, 'A', 'B', OVERLAP).state, 'finding');
  assert.equal(row(out, 'A', 'C', OVERLAP).state, 'passed');
  assert.equal(row(out, 'A', 'C', CONTACT).state, 'passed');
  assert.equal(row(out, 'A', 'D', OVERLAP).state, 'passed');
  assert.equal(row(out, 'A', 'N', OVERLAP).state, 'not-evaluated');
  assert.match(row(out, 'A', 'N', OVERLAP).reason, /part-geometry-missing/);
  assert.equal(row(out, 'A', 'GONE', OVERLAP).state, 'absent');
  assert.equal(row(out, 'A', 'B', BOLTED).state, 'not-applicable');
  assert.equal(out.focusSets.length, 5 * 3);
  assert.deepEqual(out.focusSets.map((s) => s.identities), out.focusSets.map((s) => [...s.identities].sort()));
  const withApprox = run([plate('A', 0, 10)], [plate('A', 0, 10), plate('Q', 400, 410, { coverage: openCheck('recipes-evaluated', 'recipe-skipped') })], { focus: { identities: [], sets: [[id('A'), id('Q')]] } });
  assert.equal(row(withApprox, 'A', 'Q', OVERLAP).state, 'not-evaluated', 'a clean result on approximate geometry is not passed');
});

test('an identity the later state cannot vouch for is excluded, not absent', () => {
  const revised = world([plate('A', 0, 10)]);
  revised.docs.entities.entities.push({ id: 'element:99', type: 'Beam', name: 'no guid' });   // blank identity
  const base = world([plate('A', 0, 10), plate('GONE', 400, 410)]);
  const out = checkSnapshots({
    base: base.docs, revised: revised.docs, profile: PROFILE, checkProfile: clashChecks(), geometry: { base: base.tiles, revised: revised.tiles }, engine,
    focus: { identities: [], sets: [[id('A'), id('GONE')]] },
  });
  assert.equal(row(out, 'A', 'GONE', OVERLAP).state, 'excluded');
});

test('a bolted stack is passed only when the whole contact graph is connected, and a change of main group never reads as fixed', () => {
  // Earlier: A1-A2 touch (the main group); A3 stands off alone. Later: two more plies join A3, so {A3, A4, A5} is the
  // bigger group and becomes the main one. A3 is no longer detached - but the stack is still split.
  const plies = (extra) => [plate('A1', 0, 10), plate('A2', 10, 20), plate('A3', 200, 210), ...extra];
  const names = ['A1', 'A2', 'A3'];
  const base = [...plies([]), bolt('BG', names)];
  const revised = [...plies([plate('A4', 210, 220), plate('A5', 220, 230)]), bolt('BG', [...names, 'A4', 'A5'])];
  const out = run(base, revised, { focus: { identities: [], sets: [[id('A3'), id('BG')], [id('A1'), id('BG')], [id('A4'), id('BG')]] } });
  assert.deepEqual(of(out, BOLTED).map((f) => idsOf(f)[1]).sort(), [id('A1'), id('A2')], 'the old main group is the detached one now');
  assert.equal(out.resolvedInChange.filter((r) => r.ruleId === BOLTED).length, 0, 'the earlier finding for A3 is not resolved');
  const a3 = row(out, 'A3', 'BG', BOLTED);
  assert.equal(a3.state, 'not-evaluated');
  assert.match(a3.reason, /the stack is still split, so this ply cannot be shown to be fixed/);
  assert.equal(row(out, 'A1', 'BG', BOLTED).state, 'finding');
  // once everything touches, the earlier finding is resolved and the pair re-check says passed
  const fixed = [...plies([]), bolt('BG', names)].map((p) => (p.guid === 'A3' ? plate('A3', 20, 30, { name: 'moved' }) : p));
  const done = run(base, fixed, { focus: { identities: [], sets: [[id('A3'), id('BG')]] } });
  assert.equal(row(done, 'A3', 'BG', BOLTED).state, 'passed');
  assert.equal(done.resolvedInChange.filter((r) => r.ruleId === BOLTED).length, 1);
  assert.equal(done.findings.length, 0);
});

// ─── round 3 review: a deleted ply, a bolt type the rule did not cover before, a bolted-only profile ─────────

test('deleting a ply that was detached, and dropping it from the bolt group, neither throws nor reports a finding for it', () => {
  const base = [plate('A1', 0, 10), plate('A2', 10, 20), plate('A3', 200, 210), bolt('BG', ['A1', 'A2', 'A3'])];
  const revised = [plate('A1', 0, 10), plate('A2', 10, 20), bolt('BG', ['A1', 'A2'])];
  const out = run(base, revised);
  assert.deepEqual(of(out, BOLTED), []);
});

test('a bolt group whose type the bolted rule did not cover before is not claimed pre-existing: its earlier state was not checked', () => {
  const checks = clashChecks({ geometry: { joins: { bolt: { types: ['Bolt group', 'Hardware'], plies: { property: 'native:bolted-parts', format: 'identity-key-list' } }, weld: { types: ['Physical'], parts: { property: 'native:welded-parts', format: 'identity-key-list' }, unknownWhen: { property: 'native:welded-parts', absent: true } } } } });
  const plies = [plate('P1', 0, 10), plate('P2', 13, 23)];
  const out = run([...plies, bolt('BG', ['P1', 'P2'], { type: 'Hardware' })], [...plies, bolt('BG', ['P1', 'P2'])], { checks });
  const [f] = of(out, BOLTED);
  assert.ok(f, 'the gap is still reported');
  assert.equal(f.presence, 'unknown');
});

test('a profile with only the bolted rule still counts the plies it inspected in scope.solids, and says when one has no solid', () => {
  const checks = clashChecks({ rules: ['bolted'] });
  const out = run([], [plate('P1', 0, 10), plate('P2', 13, 23), plate('P3', 30, 40, { coverage: null }), bolt('BG', ['P1', 'P2', 'P3'])], { checks });
  assert.equal(out.scope.solids.total, 3);
  assert.equal(out.scope.solids.evaluated, 2);
  assert.equal(out.scope.solids.approximate, 1, 'the ply without coverage is counted as unchecked');
});

// ─── round 4 review: a move below the comparison tolerance, and the pair budget ─────────────────────────────

test('a part moved less than the comparison tolerance but into another part is still checked for clash', () => {
  // PROFILE compares bounds at 1 mm; the clash contact tolerance is 0.1 mm. P2 moves 0.5 mm into P1 (a 0.3 mm overlap).
  const out = run([plate('P1', 0, 10), plate('P2', 10.2, 20)], [plate('P1', 0, 10), plate('P2', 9.7, 20)]);
  const [f] = of(out, OVERLAP);
  assert.ok(f, 'the interpenetration is found');
  assert.equal(f.evidence.overlapThicknessMm, 0.3);
});

test('a pair that no clash rule covers does not use up the pair budget', () => {
  const checks = clashChecks({ geometry: { budgets: { trianglesPerObject: 200000, trianglePairsPerPair: 4000000, gapSearchMm: 50, pairs: 1, focusSets: 20000 } } });
  checks.rules = [{ ...checks.rules[0], appliesTo: { types: ['Beam'] } }, { ...checks.rules[1], appliesTo: { types: ['Contour plate'] } }];
  const same = (guid, type) => ({ guid, type, solid: box([0, 0, 0], [100, 100, 10]) });
  const out = run([], [same('A', 'Beam'), same('B', 'Contour plate'), same('C', 'Beam')], { checks });
  assert.equal(of(out, OVERLAP).length, 1, 'A-C is evaluated and found, not marked over budget');
  assert.equal(out.scope.notEvaluated.byReason['over-budget'], undefined);
});

// ─── determinism, budgets, refusals ─────────────────────────────────────────────────────────────────

const busy = () => {
  const base = [plate('A', 0, 10), plate('B', 5, 15), plate('C', 40, 50), bolt('BG', ['A', 'C'])];
  const revised = [plate('A', 0, 10), plate('B', 2, 12), plate('C', 10, 20), plate('D', 20, 30), bolt('BG', ['A', 'C', 'D'])];
  return { base, revised, opts: { focus: { identities: [id('B')], sets: [[id('A'), id('B')], [id('A'), id('D')]] } } };
};

test('the output is byte-identical when run twice, and when every document and the tile are in another order', () => {
  const { base, revised, opts } = busy();
  const first = canonicalJson(run(base, revised, opts));
  assert.equal(canonicalJson(run(base, revised, opts)), first);
  const reordered = canonicalJson(run(base, revised, { ...opts, shuffle: true }));
  assert.equal(reordered, first);
  assert.ok(JSON.parse(first).findings.length >= 2);
});

test('the geometry digests are echoed, and a tile that does not match its digest is refused', () => {
  const { base, revised } = busy();
  const out = run(base, revised);
  assert.equal(out.inputs.geometry.revised.tiles.length, 1);
  assert.match(out.inputs.geometry.revised.tiles[0].sha256, /^[0-9a-f]{64}$/);
  const b = world(base); const r = world(revised);
  r.tiles.tiles[0] = { ...r.tiles.tiles[0], sha256: 'f'.repeat(64) };
  refuses(() => checkSnapshots({ base: b.docs, revised: r.docs, profile: PROFILE, checkProfile: clashChecks(), geometry: { base: b.tiles, revised: r.tiles }, engine }), 'geometry-digest-mismatch');
});

test('pairs beyond the budget are not-evaluated and counted apart from the lists; a pair that is too big is too', () => {
  const parts = [plate('A', 0, 10), plate('B', 5, 15), plate('C', 8, 18), plate('D', 12, 22)];
  const open = run([], parts);
  assert.equal(of(open, OVERLAP).length, 5, 'A-D are 2 mm apart: not a candidate');
  const limited = run([], parts, { checks: clashChecks({ geometry: { budgets: { trianglesPerObject: 200000, trianglePairsPerPair: 4000000, gapSearchMm: 50, pairs: 2, focusSets: 20000 } } }) });
  assert.equal(of(limited, OVERLAP).length, 2);
  assert.equal(limited.scope.notEvaluated.byReason['over-budget'], 3);
  assert.equal(limited.complete, true, 'over-budget evaluations are not omitted list rows');
  assert.equal(limited.truncated, null);
  assert.match(limited.unchecked.join('\n'), /3 pair\(s\) were too large to check within the limits/);
  const tooBig = run([], parts, { checks: clashChecks({ geometry: { budgets: { trianglesPerObject: 200000, trianglePairsPerPair: 100, gapSearchMm: 50, pairs: 200000, focusSets: 20000 } } }) });
  assert.deepEqual(tooBig.findings, []);
  assert.equal(tooBig.scope.notEvaluated.byReason['over-budget'], 5);
  const tiny = run([], parts, { checks: clashChecks({ geometry: { budgets: { trianglesPerObject: 10, trianglePairsPerPair: 4000000, gapSearchMm: 50, pairs: 200000, focusSets: 20000 } } }) });
  assert.equal(tiny.scope.solids.overBudget, 4);
});

test('the focusSets budget caps the list and says so; complete is false', () => {
  const parts = [plate('A', 0, 10), plate('B', 5, 15)];
  const checks = clashChecks({ geometry: { budgets: { trianglesPerObject: 200000, trianglePairsPerPair: 4000000, gapSearchMm: 50, pairs: 200000, focusSets: 2 } } });
  const out = run([], parts, { checks, focus: { identities: [], sets: [[id('A'), id('B')]] } });
  assert.equal(out.focusSets.length, 2);
  assert.equal(out.truncated.omitted.focusSets, 1);
  assert.equal(out.complete, false);
  assert.match(out.unchecked.join('\n'), /1 pair re-check row\(s\)/);
  const full = run([], parts, { focus: { identities: [], sets: [[id('A'), id('B')]] } });
  assert.equal(full.complete, true);
  assert.equal(full.focusSets.length, 3);
});

test('every refusal is an error, never a guess', () => {
  const parts = [plate('A', 0, 10), plate('B', 5, 15)];
  const b = world([]); const r = world(parts);
  const go = (checkProfile, extra = {}) => () => checkSnapshots({ base: b.docs, revised: r.docs, profile: PROFILE, checkProfile, geometry: { base: b.tiles, revised: r.tiles }, engine, ...extra });
  const withGeometry = (patch) => ({ ...clashChecks(), geometry: { ...clashChecks().geometry, ...patch } });
  refuses(go(clashChecks(), { geometry: null }), 'input-invalid', /geometry input are required/);
  refuses(go(clashChecks(), { engine: null }), 'engine-unavailable');
  const noBlock = clashChecks(); delete noBlock.geometry;
  refuses(go(noBlock), 'profile-invalid', /geometry is required/);
  refuses(go(withGeometry({ extra: 1 })), 'profile-invalid', /unknown key 'extra'/);
  refuses(go(withGeometry({ units: 'in' })), 'profile-invalid', /units/);
  refuses(go(withGeometry({ contactTolerance: 0 })), 'profile-invalid', /contactTolerance/);
  refuses(go(withGeometry({ weldTolerance: 5 })), 'profile-invalid', /weldTolerance/);
  refuses(go(withGeometry({ node: { entityIdPattern: '(', nodeName: 'x' } })), 'profile-invalid', /not a valid regular expression/);
  refuses(go(withGeometry({ node: { entityIdPattern: '^element:([0-9]+)$', nodeName: 'object-$2' } })), 'profile-invalid', /capture \$2/);
  refuses(go(withGeometry({ groups: [{ kind: 'depends-on', name: 'x' }] })), 'profile-invalid', /vocabulary/);
  refuses(go(withGeometry({ groups: [{ kind: 'hosts', name: 'a b' }, { kind: 'contains', name: 'A-B' }] })), 'profile-invalid', /used twice/);
  refuses(go(withGeometry({ qualification: { propertyKeyPattern: '^x', propertyField: 'id', json: 'checks', checks: [], whenPresent: [], passStates: ['pass'] } })), 'profile-invalid', /non-empty array/);
  const q = clashChecks().geometry.qualification;
  const { whenPresent: _dropped, ...noWhenPresent } = q;
  refuses(go(withGeometry({ qualification: noWhenPresent })), 'profile-invalid', /whenPresent/);
  refuses(go(withGeometry({ qualification: { ...q, whenPresent: ['geometry-reconstruction'] } })), 'profile-invalid', /already requires/);
  refuses(go(withGeometry({ qualification: { ...q, whenPresent: ['placement-confirmed', 'placement-confirmed'] } })), 'profile-invalid', /twice/);
  refuses(go(withGeometry({ qualification: { ...q, whenPresent: 'placement-confirmed' } })), 'profile-invalid', /whenPresent/);
  refuses(go(withGeometry({ joins: { ...clashChecks().geometry.joins, weld: { ...clashChecks().geometry.joins.weld, unknownWhen: { property: 'p', absent: false } } } })), 'profile-invalid', /absent must be true/);
  refuses(go(withGeometry({ budgets: { ...clashChecks().geometry.budgets, gapSearchMm: 0.01 } })), 'profile-invalid', /gapSearchMm/);
  refuses(go(withGeometry({ budgets: { ...clashChecks().geometry.budgets, pairs: 0 } })), 'profile-invalid', /pairs/);
  const wrongTarget = clashChecks(); wrongTarget.rules = [{ ...wrongTarget.rules[0], target: 'entity.name' }];
  refuses(go(wrongTarget), 'profile-invalid', /target must be 'solid'/);
  const blankOnSolid = clashChecks({ rules: [] }); blankOnSolid.rules = [{ id: 'x', version: '1', severity: 'warning', predicate: 'blank', target: 'solid', appliesTo: { types: ['Beam'] }, worsening: null }];
  refuses(go(blankOnSolid), 'profile-invalid', /entity.name/);
  const twice = clashChecks(); twice.rules = [twice.rules[0], { ...twice.rules[0], id: 'other' }];
  refuses(go(twice), 'profile-invalid', /used by two rules/);
  const badMetric = clashChecks(); badMetric.rules = [{ ...badMetric.rules[0], worsening: { path: 'gapMm', direction: 'higher-is-worse' } }];
  refuses(go(badMetric), 'profile-invalid', /worsening.path/);
  const contactMetric = clashChecks({ rules: ['contact'] }); contactMetric.rules = [{ ...contactMetric.rules[0], worsening: { path: 'gapMm', direction: 'higher-is-worse' } }];
  refuses(go(contactMetric), 'profile-invalid', /worsening.path/);
  for (const bad of [{ identities: [], sets: 3 }, { identities: [], sets: [['a']] }, { identities: [], sets: [['a', 'a']] }, { identities: [], sets: [['a', 'b'], ['b', 'a']] }, { identities: [], extra: 1 }]) {
    refuses(go(clashChecks(), { focus: bad }), 'focus-invalid');
  }
});

// ─── reading the tile ───────────────────────────────────────────────────────────────────────────────

test('an indexed mesh with 16-bit indices and a node transform reads the same solid, mirrored nodes keep it outward', () => {
  const solid = box([0, 0, 0], [10, 20, 30]);
  const moved = readTileMeshes(glb([{ name: 'object-1', solid, indexed: true, matrix: [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 100, 200, 300, 1] }]), 'tile').get('object-1');
  assert.equal(moved.tris.length / 3, 12);
  assert.equal(moved.positions.length / 3, 8);
  assert.deepEqual([moved.positions[0], moved.positions[1], moved.positions[2]].map(Number).length, 3);
  const xs = []; for (let i = 0; i < moved.positions.length; i += 3) xs.push(moved.positions[i]);
  assert.deepEqual([Math.min(...xs), Math.max(...xs)], [100, 110]);
  // a mirrored node (scale -1 in x) flips the winding back to outward-facing
  const mirrored = readTileMeshes(glb([{ name: 'object-1', solid, matrix: [-1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1] }]), 'tile').get('object-1');
  const out = run([], [{ guid: 'A', type: 'Contour plate', solid: box([-10, 0, 0], [0, 20, 30]) }, { guid: 'B', type: 'Contour plate', solid: box([-5, 0, 0], [5, 20, 30]) }]);
  assert.equal(of(out, OVERLAP)[0].evidence.overlapThicknessMm, 5);
  assert.equal(mirrored.tris.length, 36);
  const welded = weld(mirrored.positions, mirrored.tris, 0.001);
  const verdict = checkMesh(welded.verts, welded.tris);
  assert.equal(verdict.reason, null, 'the winding was flipped back, so the mesh is still a closed outward-facing solid');
  assert.equal(verdict.volume, 6000);
});

test('a geometry directory is read with every digest verified; a file name that leaves the directory is refused', async () => {
  const { mkdtempSync, writeFileSync, rmSync } = await import('node:fs');
  const { tmpdir } = await import('node:os');
  const { join } = await import('node:path');
  const dir = mkdtempSync(join(tmpdir(), 'clash-geometry-'));
  try {
    const bytes = glb([{ name: 'object-1', solid: box([0, 0, 0], [1, 1, 1]) }]);
    writeFileSync(join(dir, 't.glb'), bytes);
    const tile = tileOf(bytes, 't.glb');
    writeFileSync(join(dir, 'geometry.json'), JSON.stringify({ tiles: [{ ordinal: 0, file: 't.glb', sha256: tile.sha256 }] }));
    assert.equal(readGeometryDir(dir, 'dir').tiles[0].bytes.length, bytes.length);
    writeFileSync(join(dir, 'geometry.json'), JSON.stringify({ tiles: [{ ordinal: 0, file: '../t.glb', sha256: tile.sha256 }] }));
    refuses(() => readGeometryDir(dir, 'dir'), 'document-invalid', /plain file name/);
    writeFileSync(join(dir, 'geometry.json'), JSON.stringify({ tiles: [{ ordinal: 0, file: 'gone.glb', sha256: tile.sha256 }] }));
    refuses(() => readGeometryDir(dir, 'dir'), 'input-missing');
    refuses(() => readGeometryDir(join(dir, 'nope'), 'dir'), 'input-missing');
  } finally { rmSync(dir, { recursive: true, force: true }); }
  refuses(() => readTileMeshes(Buffer.from('not a glb at all, but long enough to read'), 'tile'), 'geometry-invalid');
  assert.deepEqual(SOLID_TYPES.includes('Bolt group'), true);
});

// ─── review fixes ───────────────────────────────────────────────────────────────────────────────────

test('a pair re-check gets the same budgeted verdict as the main pass: over-budget is not-evaluated, not passed', () => {
  const checks = clashChecks({ geometry: { budgets: { trianglesPerObject: 200000, trianglePairsPerPair: 4000000, gapSearchMm: 50, pairs: 1, focusSets: 20000 } } });
  // A/B overlap and come first; A/C are within the tolerance by box but 0.113 mm apart by surface, and fall beyond the budget
  const parts = [plate('A', 0, 10), plate('B', 5, 15), { guid: 'C', type: 'Contour plate', solid: box([100.08, 100.08, 0], [200, 200, 10]) }];
  const out = run([], parts, { checks, focus: { identities: [], sets: [[id('A'), id('C')]] } });
  assert.equal(out.scope.notEvaluated.byReason['over-budget'] >= 1, true);
  for (const rule of [OVERLAP, CONTACT]) {
    assert.equal(row(out, 'A', 'C', rule).state, 'not-evaluated', rule);
    assert.match(row(out, 'A', 'C', rule).reason, /beyond the number of pairs/);
  }
});

test('contact joined by a bolt, or an anchor against its named part, is not-evaluated when a solid is approximate', () => {
  const open = openCheck('operations-evaluated', 'cuts-not-applied');
  const stack = run([], [plate('P1', 0, 10), plate('P2', 10, 20, { coverage: open }), bolt('BG', ['P1', 'P2'])], { focus: { identities: [], sets: [[id('P1'), id('P2')]] } });
  assert.equal(row(stack, 'P1', 'P2', CONTACT).state, 'not-evaluated');
  assert.deepEqual(stack.scope.notEvaluated.byReason, { approximate: 1, 'ply-not-evaluated': 1 }, 'the pair and (separately) the bolt stack');
  assert.equal(stack.scope.byRule[CONTACT].evaluated, 0);
  const shaft = { guid: 'BG', type: 'Bolt group', solid: box([40, 40, 10], [60, 60, 20]), bolted: ['P1'] };
  const own = run([], [plate('P1', 0, 10, { coverage: open }), shaft], { focus: { identities: [], sets: [[id('BG'), id('P1')]] } });
  assert.equal(row(own, 'BG', 'P1', CONTACT).state, 'not-evaluated');
  assert.equal(row(own, 'BG', 'P1', OVERLAP).state, 'not-evaluated');
  const exact = run([], [plate('P1', 0, 10), plate('P2', 10, 20), bolt('BG', ['P1', 'P2'])], { focus: { identities: [], sets: [[id('P1'), id('P2')]] } });
  assert.equal(row(exact, 'P1', 'P2', CONTACT).state, 'passed', 'on exact geometry a joined contact still passes');
});

test('a 1000 x 1000 x 1 plate overlapping an identical one by 80% is an overlap, not coincident; identical plates still are', () => {
  const thin = (x0) => ({ guid: x0 === 0 ? 'A' : 'B', type: 'Contour plate', solid: box([x0, 0, 0], [x0 + 1000, 1000, 1]) });
  const shifted = run([], [thin(0), thin(200)]);
  const [f] = of(shifted, OVERLAP);
  assert.equal(f.evidence.kind, 'overlap');
  assert.equal(f.evidence.overlapThicknessMm, 1);
  assert.equal(f.evidence.overlapVolumeMm3, 800000);
  const same = run([], [thin(0), { ...thin(0), guid: 'B' }]);
  assert.equal(of(same, OVERLAP)[0].evidence.kind, 'coincident');
  const inner = run([], [plate('A', 0, 100), { guid: 'B', type: 'Beam', solid: box([30, 30, 30], [50, 60, 70]) }]);
  assert.equal(of(inner, OVERLAP)[0].evidence.kind, 'contained');
});

// ─── review fixes, round 2 ──────────────────────────────────────────────────────────────────────────

test('without the interpenetration rule, an overlapping pair is counted as not-evaluated for contact, never dropped', () => {
  const out = run([], [plate('A', 0, 10), plate('B', 5, 15)], { checks: clashChecks({ rules: ['contact'] }) });
  assert.deepEqual(out.findings, []);
  assert.deepEqual(out.scope.notEvaluated.byReason, { interpenetrates: 1 });
  assert.equal(out.scope.notEvaluated.pairs, 1);
  assert.equal(out.scope.byRule[CONTACT].notEvaluated, 1);
  assert.equal(out.scope.byRule[CONTACT].evaluated, 0);
  assert.match(out.unchecked.join('\n'), /could not be checked: interpenetrates: 1/);
  // with the rule running, the finding answers it and nothing is counted as unchecked
  const both = run([], [plate('A', 0, 10), plate('B', 5, 15)]);
  assert.deepEqual(both.scope.notEvaluated.byReason, {});
});

test('a pair the rule did not apply to in the earlier state has unknown presence, not pre-existing', () => {
  const checks = clashChecks();
  checks.rules[0] = { ...checks.rules[0], appliesTo: { types: ['Contour plate'] } };
  const asBeam = { guid: 'B', type: 'Beam', solid: box([0, 0, 5], [100, 100, 15]) };
  const out = run([plate('A', 0, 10), asBeam], [plate('A', 0, 10), plate('B', 5, 15)], { checks });
  const [f] = of(out, OVERLAP);
  assert.equal(f.presence, 'unknown');
  assert.equal(f.baseEvidenceFingerprint, null);
  assert.equal(out.summary.unknownPresence, 1);
});

test('a pair the earlier state could not evaluate has unknown presence; one it could and found clean is introduced', () => {
  const noSolid = { guid: 'B', type: 'Contour plate', coverage: openCheck('geometry-reconstruction', 'part-geometry-missing') };
  const unknown = run([plate('A', 0, 10), noSolid], [plate('A', 0, 10), plate('B', 5, 15)]);
  const [f] = of(unknown, OVERLAP);
  assert.equal(f.presence, 'unknown');
  assert.equal(f.baseEvidenceFingerprint, null);
  assert.deepEqual(unknown.summary, { findings: 1, introduced: 0, preExisting: 0, resolvedInChange: 0, byRule: { [OVERLAP]: 1, [CONTACT]: 0, [BOLTED]: 0 }, unknownPresence: 1 });
  const clean = run([plate('A', 0, 10), plate('B', 50, 60)], [plate('A', 0, 10), plate('B', 5, 15)]);
  assert.equal(of(clean, OVERLAP)[0].presence, 'introduced');
  assert.equal(clean.summary.unknownPresence, 0);
  const open = openCheck('operations-evaluated', 'cuts-not-applied');
  const approx = run([plate('A', 0, 10), plate('B', 50, 60, { coverage: open })], [plate('A', 0, 10), plate('B', 5, 15)]);
  assert.equal(of(approx, OVERLAP)[0].presence, 'unknown', 'a clean result on approximate geometry proves nothing');
  const wasContact = run([plate('A', 0, 10), plate('B', 10, 20)], [plate('A', 0, 10), plate('B', 7, 17)]);
  assert.equal(of(wasContact, OVERLAP)[0].presence, 'introduced');
});

test('the coverage rows are found by the row field the profile names: the app writes an id and no sourceKey', () => {
  const parts = [plate('A', 0, 10), plate('B', 5, 15)];
  const ok = run([], parts);
  assert.equal(ok.scope.solids.approximate, 0, 'propertyField id finds the rows the app writes');
  assert.equal(ok.scope.solids.evaluated, 2);
  const bySourceKey = clashChecks();
  bySourceKey.geometry.qualification.propertyField = 'sourceKey';
  const miss = run([], parts, { checks: bySourceKey });
  assert.deepEqual(miss.scope.solids.byReason, { 'coverage:missing': 2 }, 'the field is really used');
  const bad = clashChecks();
  bad.geometry.qualification.propertyField = 'name';
  refuses(() => run([], parts, { checks: bad }), 'profile-invalid', /propertyField/);
  const missingKey = clashChecks();
  delete missingKey.geometry.qualification.propertyField;
  refuses(() => run([], parts, { checks: missingKey }), 'profile-invalid', /missing 'propertyField'/);
});

// ─── review fixes, round 3: coverage rows handed over beside the tiles ──────────────────────────────

test('coverage rows that live only in qualification.json qualify the solids; the same rows absent read as coverage:missing', () => {
  const parts = [plate('A', 0, 10), plate('B', 5, 15)];
  const given = run([], parts, { qualificationFile: true });
  assert.equal(given.scope.solids.evaluated, 2);
  assert.equal(given.scope.solids.approximate, 0);
  assert.equal(of(given, OVERLAP)[0].evidence.approximate, false);
  assert.match(given.inputs.geometry.revised.qualification.sha256, /^[0-9a-f]{64}$/);
  assert.equal(given.inputs.geometry.revised.qualification.file, 'qualification.json');
  // the snapshot's own properties hold no coverage and no qualification file is given: nothing to qualify with
  const b = world([], { qualificationFile: true }); const r = world(parts, { qualificationFile: true });
  delete r.tiles.qualification;
  const missing = checkSnapshots({ base: b.docs, revised: r.docs, profile: PROFILE, checkProfile: clashChecks(), geometry: { base: b.tiles, revised: r.tiles }, engine });
  assert.deepEqual(missing.scope.solids.byReason, { 'coverage:missing': 2 });
  assert.equal(missing.inputs.geometry.revised.qualification, null);
  // when a qualification file is given, rows in properties.json are not consulted
  const open = openCheck('operations-evaluated', 'cuts-not-applied');
  const both = world([plate('A', 0, 10), plate('B', 5, 15)], { qualificationFile: true, qualificationRows: [] });
  const none = checkSnapshots({ base: b.docs, revised: both.docs, profile: PROFILE, checkProfile: clashChecks(), geometry: { base: b.tiles, revised: both.tiles }, engine });
  assert.deepEqual(none.scope.solids.byReason, { 'coverage:missing': 2 });
  const withOpen = run([], [plate('A', 0, 10), plate('B', 5, 15, { coverage: open })], { qualificationFile: true });
  assert.deepEqual(withOpen.scope.solids.byReason, { 'coverage:operations-evaluated:cuts-not-applied': 1 });
});

test('a qualification file that does not match its digest is refused; a bad shape is geometry-invalid', () => {
  const b = world([]); const r = world([plate('A', 0, 10)], { qualificationFile: true });
  const go = (q) => () => checkSnapshots({ base: b.docs, revised: r.docs, profile: PROFILE, checkProfile: clashChecks(), geometry: { base: b.tiles, revised: { ...r.tiles, qualification: q } }, engine });
  refuses(go({ ...r.tiles.qualification, sha256: 'f'.repeat(64) }), 'geometry-digest-mismatch', /qualification/);
  const shaped = (text) => { const bytes = Buffer.from(text); return { file: 'qualification.json', sha256: sha(bytes), bytes }; };
  refuses(go(shaped('{"properties":{}}')), 'geometry-invalid', /properties/);
  refuses(go(shaped('{"properties":[],"extra":1}')), 'geometry-invalid');
  refuses(go(shaped('{"properties":[{"id":"x"}]}')), 'geometry-invalid', /entityId/);
  refuses(go(shaped('not json')), 'geometry-invalid');
});

test('qualification rows never make an unchanged part changed, whatever they say', () => {
  const parts = [plate('A', 0, 10), plate('B', 50, 60)];
  const b = world(parts, { qualificationFile: true });
  const open = openCheck('operations-evaluated', 'cuts-not-applied');
  const r = world(parts.map((p) => ({ ...p, coverage: open })), { qualificationFile: true });
  const out = checkSnapshots({ base: b.docs, revised: r.docs, profile: PROFILE, checkProfile: clashChecks(), geometry: { base: b.tiles, revised: r.tiles }, engine });
  assert.equal(out.scope.added, 0);
  assert.equal(out.scope.changed, 0);
  assert.deepEqual(out.findings, []);
  assert.notEqual(out.inputs.geometry.base.qualification.sha256, out.inputs.geometry.revised.qualification.sha256);
});

test('a qualification file in a geometry directory is read, digest-checked and echoed', async () => {
  const { mkdtempSync, writeFileSync, rmSync } = await import('node:fs');
  const { tmpdir } = await import('node:os');
  const { join } = await import('node:path');
  const dir = mkdtempSync(join(tmpdir(), 'clash-qual-'));
  try {
    const w = world([plate('A', 0, 10)], { qualificationFile: true });
    const t = w.tiles.tiles[0]; const q = w.tiles.qualification;
    writeFileSync(join(dir, t.file), t.bytes); writeFileSync(join(dir, q.file), q.bytes);
    writeFileSync(join(dir, 'geometry.json'), JSON.stringify({ tiles: [{ ordinal: 0, file: t.file, sha256: t.sha256 }], qualification: { file: q.file, sha256: q.sha256 } }));
    const read = readGeometryDir(dir, 'dir');
    assert.equal(read.qualification.bytes.length, q.bytes.length);
    writeFileSync(join(dir, 'geometry.json'), JSON.stringify({ tiles: [], qualification: { file: '../q.json', sha256: q.sha256 } }));
    refuses(() => readGeometryDir(dir, 'dir'), 'document-invalid', /plain file name/);
    writeFileSync(join(dir, 'geometry.json'), JSON.stringify({ tiles: [], qualification: { file: 'gone.json', sha256: q.sha256 } }));
    refuses(() => readGeometryDir(dir, 'dir'), 'input-missing');
  } finally { rmSync(dir, { recursive: true, force: true }); }
});
