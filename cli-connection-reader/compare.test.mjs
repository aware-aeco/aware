// Unit tests for `compare` — the identity rules and the refusal, which are the two things that decide
// whether this feature tells the truth. They live here rather than in the CLI suite because `compare.mjs`
// is deliberately pure: plain objects in, plain objects out, no WASM parser and no sample file. The
// two-file CLI path is covered separately in `compare.cli.test.mjs`.
import test from 'node:test';
import assert from 'node:assert/strict';
import { comparableFrom } from './compare.mjs';

// A unit cube, 1000 mm on a side, centred on (500, 500, 500). Two triangles per face is not needed —
// the extents and centroid come off the vertices, and the triangle count off the indices.
const CUBE = {
  globalId: 'ABC', id: 'ABC', name: 'Beam 1', ifcType: 'IFCBEAM', storey: 'L1', profile: 'W10x33', material: 'S355',
  positions: [0,0,0, 1000,0,0, 1000,1000,0, 0,1000,0, 0,0,1000, 1000,0,1000, 1000,1000,1000, 0,1000,1000],
  indices: [0,1,2, 0,2,3],
  propertySets: [{ name: 'Pset_BeamCommon', properties: [{ name: 'Reference', value: 'B1' }] }],
};

test('comparableFrom keys on globalId and IGNORES id, which may be a substituted expressID', () => {
  const c = comparableFrom({ ...CUBE, globalId: null, id: '4711' });
  assert.equal(c.globalId, null, 'a file with no GlobalId yields no identity — never the expressID');
  const real = comparableFrom(CUBE);
  assert.equal(real.globalId, 'ABC');
});

test('comparableFrom keeps identity, attributes, centroid, extents and triangle count', () => {
  const c = comparableFrom(CUBE);
  assert.equal(c.globalId, 'ABC');
  assert.equal(c.ifcType, 'IFCBEAM');
  assert.equal(c.profile, 'W10x33');
  assert.deepEqual(c.centroid, [500, 500, 500]);
  assert.deepEqual(c.extents, [1000, 1000, 1000]); // sorted ascending
  assert.equal(c.triangles, 2);
});

test('comparableFrom retains NO geometry — holding two models of triangles is the memory bill this avoids', () => {
  const c = comparableFrom(CUBE);
  assert.equal(c.positions, undefined);
  assert.equal(c.indices, undefined);
});

test('comparableFrom sorts extents, so a 90-degree turn about an axis is not a shape change', () => {
  const upright = comparableFrom({ ...CUBE, positions: [0,0,0, 100,0,0, 100,200,0, 0,200,0, 0,0,300, 100,0,300, 100,200,300, 0,200,300] });
  const onItsSide = comparableFrom({ ...CUBE, positions: [0,0,0, 300,0,0, 300,100,0, 0,100,0, 0,0,200, 300,0,200, 300,100,200, 0,100,200] });
  assert.deepEqual(upright.extents, onItsSide.extents);
});

test('comparableFrom flattens property sets to set.name -> value, so a diff can name the field', () => {
  const c = comparableFrom(CUBE);
  assert.equal(c.properties['Pset_BeamCommon.Reference'], 'B1');
});

test('comparableFrom tolerates an object with no drawable geometry rather than dividing by zero', () => {
  const c = comparableFrom({ ...CUBE, positions: [], indices: [] });
  assert.equal(c.centroid, null);
  assert.equal(c.extents, null);
  assert.equal(c.triangles, 0);
});

// --- usable ids ----------------------------------------------------------------------------------
import { partitionByUsableId } from './compare.mjs';

// `obj(globalId, …)` — the first argument is the FILE'S GlobalId. `null` means the file records none.
const obj = (globalId, over = {}) => ({ globalId, name: null, ifcType: 'IFCWALL', storey: 'L1', profile: null, material: null, centroid: [0,0,0], extents: [1,1,1], triangles: 1, properties: {}, ...over });

test('a null globalId is not usable — a file recording none gives us nothing to match on', () => {
  const { usable, uncomparable } = partitionByUsableId([obj(null), obj('A')]);
  assert.deepEqual([...usable.keys()], ['A']);
  assert.equal(uncomparable.count, 1);
  assert.equal(uncomparable.blank, 1);
  assert.equal(uncomparable.duplicated, 0);
});

test('an EMPTY-STRING globalId is not usable either — belt and braces on the reader\u2019s normalisation', () => {
  const { usable } = partitionByUsableId([obj(''), obj('A')]);
  assert.deepEqual([...usable.keys()], ['A']);
});

test('a DUPLICATED id is not usable either, and BOTH copies are excluded', () => {
  const { usable, uncomparable } = partitionByUsableId([obj('D'), obj('D'), obj('A')]);
  assert.deepEqual([...usable.keys()], ['A']);
  assert.equal(uncomparable.count, 2, 'both copies go, not just the second — nothing says which is which');
  assert.equal(uncomparable.duplicated, 2);
});

test('uncomparable objects are broken down by type and storey, so the UI can say WHICH ones', () => {
  const { uncomparable } = partitionByUsableId([
    obj(null, { ifcType: 'IFCBUILDINGELEMENTPROXY', storey: 'L2' }),
    obj(null, { ifcType: 'IFCBUILDINGELEMENTPROXY', storey: 'L2' }),
    obj(null, { ifcType: 'IFCWALL', storey: 'L1' }),
  ]);
  assert.equal(uncomparable.byType.IFCBUILDINGELEMENTPROXY, 2);
  assert.equal(uncomparable.byStorey.L2, 2);
});

// --- what counts as a change ---------------------------------------------------------------------
import { CRITERIA, changedBy } from './compare.mjs';

const base = obj('A', { name: 'B1', profile: 'W10x33', material: 'S355', centroid: [0,0,0], extents: [100,200,300], triangles: 12, properties: { 'Pset.Ref': 'B1' } });
const at = (over) => ({ ...base, ...over });

test('the comparison set is a NAMED list, echoed so a stored change list cannot silently change meaning', () => {
  assert.deepEqual(CRITERIA, ['location', 'geometry', 'ifcType', 'name', 'profile', 'material', 'properties']);
});

test('nothing fires when nothing changed', () => {
  assert.deepEqual(changedBy(base, at({}), 1), []);
});

test('a move beyond tolerance fires location, and only location', () => {
  assert.deepEqual(changedBy(base, at({ centroid: [0, 0, 250] }), 1), ['location']);
});

test('a move WITHIN tolerance fires nothing — this is what stops float noise flooding the list', () => {
  assert.deepEqual(changedBy(base, at({ centroid: [0, 0, 0.4] }), 1), []);
});

test('a different shape fires geometry', () => {
  assert.deepEqual(changedBy(base, at({ extents: [100, 200, 900] }), 1), ['geometry']);
  assert.deepEqual(changedBy(base, at({ triangles: 24 }), 1), ['geometry']);
});

test('each remaining criterion fires ALONE — a criterion that cannot be isolated is one nobody can trust a row about', () => {
  assert.deepEqual(changedBy(base, at({ ifcType: 'IFCCOLUMN' }), 1), ['ifcType']);
  assert.deepEqual(changedBy(base, at({ name: 'B2' }), 1), ['name']);
  assert.deepEqual(changedBy(base, at({ profile: 'W12x40' }), 1), ['profile']);
  assert.deepEqual(changedBy(base, at({ material: 'S275' }), 1), ['material']);
  assert.deepEqual(changedBy(base, at({ properties: { 'Pset.Ref': 'B9' } }), 1), ['properties']);
});

test('a REMOVED property fires properties — deleting a value is a change, and the easiest one to miss', () => {
  assert.deepEqual(changedBy(base, at({ properties: {} }), 1), ['properties']);
});

test('an object with no geometry compares on attributes alone rather than reporting a phantom move', () => {
  const ghostly = at({ centroid: null, extents: null, triangles: 0 });
  assert.deepEqual(changedBy(ghostly, { ...ghostly }, 1), []);
});
