// Unit tests for `compare-snapshots`. Synthetic facade documents, because `snapshot-compare.mjs` is pure and
// format-blind: every Tekla-looking key below arrives through the PROFILE, never through the code. Each
// test names the outcome it expects; the renumbering test carries its own negative control.
import test from 'node:test';
import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import { join } from 'node:path';
import {
  canonicalJson, compareSnapshots, CHANGE_SET_SCHEMA, SnapshotCompareError, validateProfile,
} from './snapshot-compare.mjs';
import { GUID_KEY, PROFILE } from './snapshot-compare-fixtures.mjs';

const profileWith = (patch) => ({ ...PROFILE, ...patch });

/**
 * Build the three documents for one side from objects.
 *   { guid, type, name, props: {sourceKey: value}, udas: [{kind, defId, name, value}], bounds }
 * `idOffset` renumbers every `element:<n>`, property id and UDA definition id, which is what a re-save does.
 */
function side(objects, { relationships = [], idOffset = 0, udaOffset = 0 } = {}) {
  const entities = [];
  const properties = [];
  const entityIdOf = new Map();
  objects.forEach((o, index) => {
    const id = `element:${index + 1 + idOffset}`;
    if (o.guid !== undefined) entityIdOf.set(o.guid, id);
    entities.push({
      id, type: o.type ?? 'BEAM', name: o.name ?? `part ${index}`,
      geometry: o.bounds ? [{ tileOrdinal: 0, bounds: o.bounds }] : [], extra: 'ignored',
    });
    let n = 0;
    const push = (sourceKey, name, value, extra = {}) => {
      properties.push({
        id: `prop:${id}:${n}`, entityId: id, groupName: 'g', groupOrdinal: 0, name, value, valueType: typeof value === 'number' ? 'number' : 'string',
        unit: null, propertyOrdinal: n, semantic: 'x', dimension: 'none', isUda: sourceKey.startsWith('uda:'), sourceKey, ...extra,
      });
      n += 1;
    };
    if (o.guid !== undefined && o.guid !== null) push(GUID_KEY, 'GUID', o.guid);
    for (const guid of o.extraGuids ?? []) push(GUID_KEY, 'GUID', guid);
    push('native:id', 'ID', 100000 + index + idOffset);                 // volatile: differs on every re-save
    for (const [key, value] of Object.entries(o.props ?? {})) push(key, key.split(':')[1], value);
    for (const uda of o.udas ?? []) push(`uda:${uda.kind}:${uda.defId + udaOffset}`, uda.name, uda.value);
    push('viewer:note', 'viewer', 'x', { id: `viewer:${id}` });          // FloLess-added row, ignored by the profile
  });
  const rels = relationships.map(([kind, from, to], i) => ({ id: `rel:${i + idOffset}`, kind, from: entityIdOf.get(from), to: entityIdOf.get(to), extra: 1 }));
  return { entities: { entities }, properties: { properties }, relationships: { relationships: rels } };
}
const compare = (base, revised, extra = {}) => compareSnapshots({ base, revised, profile: PROFILE, ...extra });
const beam = (guid, extra = {}) => ({ guid, type: 'BEAM', name: `B-${guid}`, props: { 'native:profile': 'HEA200', 'native:length': 3000, 'native:material': 'S355' }, bounds: [0, 0, 0, 3000, 200, 190], ...extra });
const rowFor = (out, identity) => out.changes.find((c) => c.identity === identity);
const id = (guid) => `lineage-A:${guid}`;

test('renumbered ids give ZERO changes (and without the masks everything would change: negative control)', () => {
  const objects = [
    beam('G1', { udas: [{ kind: 'text', defId: 5, name: 'Phase', value: 'P1' }] }),
    beam('G2', { udas: [{ kind: 'text', defId: 6, name: 'Zone', value: 'Z9' }, { kind: 'number', defId: 7, name: 'Load', value: 4 }] }),
    beam('G3'),
  ];
  const base = side(objects);
  const revised = side(objects, { idOffset: 1000, udaOffset: 40 });   // every element id, row id and UDA definition id moved
  assert.notEqual(base.entities.entities[0].id, revised.entities.entities[0].id, 'precondition: ids really were renumbered');
  const out = compare(base, revised, { witness: null });
  assert.equal(out.schema, CHANGE_SET_SCHEMA);
  assert.equal(out.identity.paired, 3);
  assert.deepEqual(out.changes, []);
  assert.equal(out.summary.changed, 0);
  assert.equal(out.summary.unchanged, 3);

  // Negative control: the same data WITHOUT the key rule (and without ignoring native:id) changes everything.
  const naive = compareSnapshots({ base, revised, profile: profileWith({ keyRules: [], volatile: [] }) });
  assert.ok(naive.summary.changed >= 2, 'with the masks removed, renumbering looks like a change to every object that has a UDA or an id');
  assert.ok(naive.changes.some((c) => c.fields.some((f) => f.key.startsWith('uda:text:') && f.op === 'removed')));
  assert.ok(naive.changes.some((c) => c.fields.some((f) => f.key === 'native:id')));
});

test('copy: a new GUID is one added object; a copied ORPHAN connection is one added Connection, not a change to the original', () => {
  const conn = { guid: 'C1', type: 'CONNECTION', name: 'Gusset', props: { 'native:profile': 'PL10' }, bounds: [10, 10, 10, 20, 20, 20] };
  const base = side([beam('G1'), conn]);
  const revised = side([beam('G1'), conn, beam('G9'), { ...conn, guid: 'C2' }]);
  const out = compare(base, revised);
  assert.equal(out.summary.added, 2);
  assert.equal(out.summary.removed, 0);
  assert.equal(out.summary.changed, 0);
  assert.deepEqual(out.changes.map((c) => c.status + ':' + c.identity), ['added:' + id('C2'), 'added:' + id('G9')]);
  assert.equal(rowFor(out, id('C2')).type, 'CONNECTION');
  assert.equal(rowFor(out, id('C1')), undefined, 'the original connection is untouched');
});

test('split: the original keeps its GUID and is changed (shape-fields: length); the new half is added', () => {
  const base = side([beam('G1')]);
  const revised = side([beam('G1', { props: { 'native:profile': 'HEA200', 'native:length': 1500, 'native:material': 'S355' }, bounds: [0, 0, 0, 1500, 200, 190] }), beam('G1b', { props: { 'native:profile': 'HEA200', 'native:length': 1500, 'native:material': 'S355' }, bounds: [1500, 0, 0, 3000, 200, 190] })]);
  const out = compare(base, revised);
  const changed = rowFor(out, id('G1'));
  assert.equal(changed.status, 'changed');
  assert.ok(changed.kinds.includes('shape-fields'));
  const length = changed.fields.find((f) => f.key === 'native:length');
  assert.deepEqual([length.kind, length.before, length.after], ['shape-fields', 3000, 1500]);
  assert.equal(rowFor(out, id('G1b')).status, 'added');
  assert.equal(out.summary.byKind['shape-fields'], 1);
});

test('combine: the first part is kept and changed, the second is removed', () => {
  const half = (guid, x0) => beam(guid, { props: { 'native:profile': 'HEA200', 'native:length': 1500, 'native:material': 'S355' }, bounds: [x0, 0, 0, x0 + 1500, 200, 190] });
  const out = compare(side([half('G1', 0), half('G1b', 1500)]), side([beam('G1')]));
  assert.equal(rowFor(out, id('G1')).status, 'changed');
  assert.equal(rowFor(out, id('G1b')).status, 'removed');
  assert.equal(out.summary.added, 0);
});

test('delete-and-recreate: one removed and one added, both in the SAME unpaired group by type and name', () => {
  const out = compare(side([beam('OLD', { name: 'Column 7' })]), side([beam('NEW', { name: 'Column 7' })]));
  assert.equal(out.summary.removed, 1);
  assert.equal(out.summary.added, 1);
  assert.equal(out.summary.changed, 0, 'grouping is a presentation, never a pairing');
  assert.equal(out.unpaired.groupCount, 1);
  const [group] = out.unpaired.groups;
  assert.deepEqual([group.type, group.name, group.removed.count, group.added.count], ['BEAM', 'Column 7', 1, 1]);
  assert.deepEqual(group.removed.entityIds, ['element:1']);
});

test('component regeneration keeps the GUIDs and the child count: no changes', () => {
  const parts = [{ guid: 'K', type: 'CONNECTION', name: 'Knee', props: { 'native:children': 3 } }, beam('K1'), beam('K2'), beam('K3')];
  const out = compare(side(parts), side(parts, { idOffset: 500 }));
  assert.equal(out.summary.changed + out.summary.added + out.summary.removed, 0);
});

test('a component whose child count changes: the connection property is changed and the children are added or removed in its group', () => {
  const conn = (children) => ({ guid: 'K', type: 'CONNECTION', name: 'Knee', props: { 'native:children': children } });
  const base = side([conn(3), beam('K1', { name: 'Knee plate' }), beam('K2', { name: 'Knee plate' }), beam('K3', { name: 'Knee plate' })]);
  const revised = side([conn(2), beam('K1', { name: 'Knee plate' }), beam('K4', { name: 'Knee plate' })]);
  const out = compare(base, revised);
  const k = rowFor(out, id('K'));
  assert.equal(k.status, 'changed');
  assert.deepEqual(k.fields.map((f) => [f.key, f.kind, f.before, f.after]), [['native:children', 'properties', 3, 2]]);
  assert.equal(out.unpaired.groups.length, 1);
  assert.equal(out.unpaired.groups[0].removed.count, 2);
  assert.equal(out.unpaired.groups[0].added.count, 1);
});

test('discontinuity: 27 of 3095 paired is flagged, with the grouped unpaired listing and the first-class outcome', () => {
  const mk = (prefix, n, name) => Array.from({ length: n }, (_, i) => beam(`${prefix}${i}`, { name }));
  const shared = Array.from({ length: 27 }, (_, i) => beam(`S${i}`));
  const base = side([...shared, ...mk('B', 3068, 'Old beam')]);
  const revised = side([...shared, ...mk('R', 3000, 'New beam')]);
  const out = compare(base, revised);
  assert.equal(out.identity.base, 3095);
  assert.equal(out.identity.paired, 27);
  assert.equal(out.identity.discontinuity, true);
  assert.equal(out.identity.pairedShareOfBase, Math.round((27 / 3095) * 1e6) / 1e6);
  assert.equal(out.unpaired.groupCount, 2, 'unpaired objects are grouped, not listed as thousands of unrelated lines');
  assert.ok(out.unchecked.some((s) => s.includes('27 of 3095')));
  // The near side of the threshold is NOT a discontinuity.
  const healthy = compare(side([...shared, ...mk('B', 5, 'x')]), side([...shared, ...mk('R', 5, 'y')]));
  assert.equal(healthy.identity.discontinuity, false);
});

test('discontinuity is decided on the unrounded pairing ratio, not the six-decimal display value', () => {
  // 1 of 3 paired = 0.3333333...; the displayed share rounds to 0.333333, which is BELOW a threshold of
  // 0.3333332 although the true ratio is above it. Negative control: a threshold just above the true ratio flags it.
  const base = side([beam('P'), beam('X1'), beam('X2')]);
  const revised = side([beam('P'), beam('Y1'), beam('Y2')]);
  const out = compareSnapshots({ base, revised, profile: profileWith({ discontinuityThreshold: 0.3333332 }) });
  assert.equal(out.identity.pairedShareOfBase, 0.333333);
  assert.equal(out.identity.discontinuity, false);
  const above = compareSnapshots({ base, revised, profile: profileWith({ discontinuityThreshold: 0.3333334 }) });
  assert.equal(above.identity.discontinuity, true);
});

test('blank and duplicate identities are excluded with receipts and never paired', () => {
  const base = side([beam('G1'), beam(undefined), beam('D'), beam('D'), beam('E', { extraGuids: ['E2'] })]);
  const revised = side([beam('G1'), beam(null, { name: 'blank-b' }), beam('D'), beam('E', { extraGuids: ['E2'] })]);
  const out = compare(base, revised);
  assert.equal(out.identity.paired, 1, 'only G1 can pair');
  assert.equal(out.identity.excluded.blank.base.count, 1);
  assert.deepEqual(out.identity.excluded.blank.base.entityIds, ['element:2']);
  assert.equal(out.identity.excluded.blank.revised.count, 1);
  // 'D' appears on two entities in base (BOTH disqualified), 'E' has two identity rows on one entity.
  assert.deepEqual(out.identity.excluded.duplicate.base.entityIds, ['element:3', 'element:4', 'element:5']);
  // revised: 'D' is unique there but the BASE holds it twice, so it cannot be paired and must not be called "added";
  // 'E' carries two identity rows, so it is excluded on both sides.
  assert.deepEqual(out.identity.excluded.duplicate.revised.entityIds, ['element:3', 'element:4']);
  assert.equal(out.summary.added + out.summary.removed, 0, 'an excluded object is not reported as added or removed');
  assert.equal(out.identity.base, 5);
  assert.ok(out.unchecked.some((s) => s.includes('no usable identity')));
});

test('relationships rebuilt through GUIDs: renumbered ids give no change; a host change gives a relationships change on both ends', () => {
  const objects = [{ guid: 'H1', type: 'BEAM' }, { guid: 'H2', type: 'BEAM' }, { guid: 'P', type: 'PLATE' }];
  const base = side(objects, { relationships: [['host', 'P', 'H1']] });
  const same = side(objects, { relationships: [['host', 'P', 'H1']], idOffset: 700 });
  assert.deepEqual(compare(base, same).changes, []);
  const moved = side(objects, { relationships: [['host', 'P', 'H2']], idOffset: 700 });
  const out = compare(base, moved);
  assert.equal(out.summary.changed, 3, 'the part, the old host and the new host');
  const part = rowFor(out, id('P'));
  assert.ok(part.kinds.includes('relationships'));
  assert.deepEqual(part.fields.map((f) => [f.key, f.op]), [[`relationship:host:out:${id('H1')}`, 'removed'], [`relationship:host:out:${id('H2')}`, 'added']]);
  assert.equal(out.summary.byKind.relationships, 3);
  assert.deepEqual([out.summary.relationships.added, out.summary.relationships.removed], [1, 1]);
});

test('a relationship to an added object follows from the add and is not a change to the paired end', () => {
  const base = side([{ guid: 'H1' }]);
  const revised = side([{ guid: 'H1' }, { guid: 'P' }], { relationships: [['host', 'P', 'H1']] });
  const out = compare(base, revised);
  assert.equal(out.summary.changed, 0);
  assert.equal(out.summary.added, 1);
  assert.equal(out.summary.relationships.onUnpaired, 1);
});

test('placement: bounds moved beyond the tolerance is placement; inside it is nothing; the limitation is always stated', () => {
  const at = (x) => [x, 0, 0, x + 3000, 200, 190];
  const base = side([beam('G1', { bounds: at(0) }), beam('G2', { bounds: at(0) })]);
  const revised = side([beam('G1', { bounds: at(1) }), beam('G2', { bounds: at(1.5) })]);   // 1.0 is within (<=), 1.5 is over
  const out = compare(base, revised);
  assert.equal(rowFor(out, id('G1')), undefined, 'a move equal to the tolerance is not a change');
  const moved = rowFor(out, id('G2'));
  assert.deepEqual(moved.kinds, ['placement']);
  assert.deepEqual([moved.fields[0].key, moved.fields[0].kind], ['geometry.bounds', 'placement']);
  assert.deepEqual(moved.fields[0].after.slice(0, 1), [1.5]);
  assert.equal(out.summary.byKind.placement, 1);
  for (const o of [out, compare(base, base)]) {
    assert.ok(o.unchecked.some((s) => s.includes('bounding box only') && s.includes('not detected')));
    assert.ok(o.unchecked.some((s) => s.startsWith('Holes, cuts, fittings and solid geometry were not compared')));
    assert.ok(o.unchecked.includes('Tekla component internals were not compared.'), 'profile statements are carried through');
  }
});

test('multi-tile geometry compares as the union of its tile bounds, so tile ordinals never matter', () => {
  const tiled = (a, b) => ({ ...beam('G1'), bounds: undefined, geometryTiles: [a, b] });
  const mk = (tiles) => {
    const s = side([beam('G1')]);
    s.entities.entities[0].geometry = tiles.map((bounds, tileOrdinal) => ({ tileOrdinal, bounds }));
    return s;
  };
  void tiled;
  const out = compare(mk([[0, 0, 0, 10, 10, 10], [10, 0, 0, 20, 10, 10]]), mk([[10, 0, 0, 20, 10, 10], [0, 0, 0, 10, 10, 10]]));
  assert.deepEqual(out.changes, []);
});

test('witness: agrees, witness-only (the log says modified, the diff saw nothing) and diff-only', () => {
  const base = side([beam('A'), beam('B'), beam('C'), beam('D')]);
  const revised = side([beam('A', { props: { 'native:profile': 'HEB300', 'native:length': 3000, 'native:material': 'S355' } }), beam('B'), beam('C'), beam('D'), beam('N')]);
  const out = compare(base, revised, {
    witness: { entries: [{ identity: id('A'), action: 'modified' }, { identity: id('B'), action: 'modified' }, { identity: 'N', action: 'created' }] },
    digests: { witness: 'w'.repeat(64) },
  });
  assert.equal(out.witness.agrees.count, 2);
  assert.deepEqual(out.witness.agrees.items.map((i) => i.identity), [id('A'), id('N')], 'a bare value is accepted and resolved to the namespaced identity');
  assert.deepEqual(out.witness.witnessOnly.items, [{ identity: id('B'), action: 'modified', diff: 'unchanged' }]);
  assert.equal(out.witness.diffOnly.count, 0);
  const second = compare(base, revised, { witness: { entries: [{ identity: id('A'), action: 'modified' }] } });
  assert.deepEqual(second.witness.diffOnly.items, [{ identity: id('N'), status: 'added' }]);
  assert.equal(out.witnessOnly, undefined);
  assert.equal(out.witness.digest, 'w'.repeat(64));
  // The witness NEVER changes the change set.
  assert.deepEqual(second.changes, out.changes);
  assert.equal(compare(base, revised).witness, null);
});

test('determinism: byte-identical output for identical inputs, and for the same rows in a different order', () => {
  const objects = [beam('G1'), beam('G2', { udas: [{ kind: 'text', defId: 3, name: 'Phase', value: 'a' }] }), beam('G3'), beam('G4')];
  const base = side(objects, { relationships: [['host', 'G1', 'G2']] });
  const revised = side([objects[1], objects[0], beam('G3', { name: 'renamed' }), beam('NEW')], { relationships: [['host', 'G1', 'G3']], idOffset: 9 });
  const a = canonicalJson(compare(base, revised));
  const b = canonicalJson(compare(structuredClone(base), structuredClone(revised)));
  assert.equal(a, b);
  const shuffled = structuredClone(revised);
  shuffled.properties.properties.reverse();
  shuffled.relationships.relationships.reverse();
  // Row order differs, so the numeric entity ids attached to the fixture differ not at all: entities keep their order.
  assert.equal(canonicalJson(compare(structuredClone(base), shuffled)), a);
  assert.equal(JSON.parse(a).inputs.profileDigest, JSON.parse(a).inputs.profileDigest);
});

test('profile digest is canonical: key order in the profile does not change it', () => {
  const reordered = Object.fromEntries(Object.entries(PROFILE).reverse());
  const a = compare(side([beam('G1')]), side([beam('G1')]));
  const b = compareSnapshots({ base: side([beam('G1')]), revised: side([beam('G1')]), profile: reordered });
  assert.equal(a.inputs.profileDigest, b.inputs.profileDigest);
  assert.match(a.inputs.profileDigest, /^[0-9a-f]{64}$/);
});

test('one unchanged pair: summary.unchanged = paired - changed and nothing is emitted for it', () => {
  const out = compare(side([beam('ONLY')]), side([beam('ONLY')], { idOffset: 3 }));
  assert.deepEqual(out.summary, { added: 0, removed: 0, changed: 0, unchanged: 1, byKind: { 'shape-fields': 0, placement: 0, properties: 0, relationships: 0 }, relationships: { base: 0, revised: 0, added: 0, removed: 0, onUnpaired: 0 } });
  assert.deepEqual(out.changes, []);
  assert.equal(out.identity.paired, 1);
});

/** Reconciliation law the FloLess parser enforces: summary[status] = emitted rows + omitted[status]. */
function assertReconciles(out) {
  for (const status of ['added', 'removed', 'changed']) {
    const emitted = out.changes.filter((c) => c.status === status).length;
    assert.equal(out.summary[status], emitted + out.truncated.omitted[status], `summary.${status} = emitted + omitted`);
  }
  assert.equal(out.identity.added, out.summary.added);
  assert.equal(out.identity.removed, out.summary.removed);
  assert.equal(out.summary.unchanged, out.identity.paired - out.summary.changed);
  assert.equal(out.identity.base, out.identity.paired + out.identity.removed + out.identity.excluded.blank.base.count + out.identity.excluded.duplicate.base.count);
}

test('truncation by row budget reconciles, keeps the pinned order, and says "partial list"', () => {
  const baseObjects = [...Array.from({ length: 5 }, (_, i) => beam(`R${i}`)), ...Array.from({ length: 6 }, (_, i) => beam(`C${i}`))];
  const revisedObjects = [...Array.from({ length: 6 }, (_, i) => beam(`C${i}`, { props: { 'native:profile': 'HEB300', 'native:length': 3000, 'native:material': 'S355' } })), ...Array.from({ length: 4 }, (_, i) => beam(`A${i}`))];
  const full = compare(side(baseObjects), side(revisedObjects));
  assertReconciles(full);
  assert.equal(full.changes.length, 15);
  assert.deepEqual(full.changes.map((c) => c.status), [...Array(5).fill('removed'), ...Array(4).fill('added'), ...Array(6).fill('changed')], 'pinned order: removed, added, changed');
  const cut = compareSnapshots({ base: side(baseObjects), revised: side(revisedObjects), profile: profileWith({ budgets: { ...PROFILE.budgets, changes: 7 } }) });
  assertReconciles(cut);
  assert.equal(cut.changes.length, 7);
  assert.deepEqual(cut.truncated.omitted, { added: 2, removed: 0, changed: 6 }, 'rows are dropped from the END of the pinned order (removed, then added, then changed)');
  assert.deepEqual(cut.changes, full.changes.slice(0, 7), 'which rows survive is deterministic');
  assert.deepEqual(cut.summary, full.summary, 'counts are complete: computed before the cap');
  assert.ok(cut.unchecked.some((s) => s.startsWith('Partial list: 8 change(s)')));
});

test('truncation by output bytes sheds rows to fit, still reconciles, and never exceeds the budget', () => {
  const baseObjects = Array.from({ length: 400 }, (_, i) => beam(`R${i}`, { name: 'same' }));
  const revisedObjects = Array.from({ length: 400 }, (_, i) => beam(`A${i}`, { name: 'same' }));
  const budget = 40 * 1024;
  const out = compareSnapshots({ base: side(baseObjects), revised: side(revisedObjects), profile: profileWith({ budgets: { ...PROFILE.budgets, outputBytes: budget } }) });
  assertReconciles(out);
  assert.ok(Buffer.byteLength(canonicalJson(out)) <= budget);
  assert.ok(out.changes.length > 0 && out.changes.length < 800);
  assert.equal(out.summary.removed + out.summary.added, 800);
  assert.ok(out.unchecked.some((s) => s.startsWith('Partial list')));
});

test('group ids, receipts and per-object fields are capped with their full counts kept', () => {
  const many = Array.from({ length: 10 }, (_, i) => beam(`U${i}`, { name: 'same name' }));
  const blanks = Array.from({ length: 6 }, () => beam(undefined));
  const manyProps = Object.fromEntries(Array.from({ length: 60 }, (_, i) => [`p:${String(i).padStart(2, '0')}`, i]));
  const wide = (v) => ({ guid: 'WIDE', name: 'W', props: Object.fromEntries(Object.entries(manyProps).map(([k, x]) => [k, x + v])) });
  const out = compareSnapshots({
    base: side([...many, ...blanks, wide(0)]), revised: side([wide(1)]),
    profile: profileWith({ budgets: { ...PROFILE.budgets, groupIds: 4, receiptsPerKind: 2 } }),
  });
  const group = out.unpaired.groups[0];
  assert.deepEqual([group.removed.count, group.removed.entityIds.length, group.removed.truncated], [10, 4, 6]);
  assert.deepEqual([out.identity.excluded.blank.base.count, out.identity.excluded.blank.base.entityIds.length, out.identity.excluded.blank.base.truncated], [6, 2, 4]);
  assert.equal(out.truncated.groups, 6);
  assert.equal(out.truncated.receipts, 4);
  const wideRow = rowFor(out, id('WIDE'));
  assert.equal(wideRow.fields.length, 50);
  assert.equal(wideRow.truncatedFields, 10);
  assert.equal(out.summary.byKind.properties, 1, 'kind counts come from ALL fields, before the per-object cap');
  assert.ok(out.unchecked.some((s) => s.includes('10 changed field(s) are not listed')));
});

test('ambiguous UDA collision: two definitions normalising to one name are NOT compared, and are listed as unchecked', () => {
  const o = (a, b) => beam('G1', { udas: [{ kind: 'text', defId: 1, name: 'Phase', value: a }, { kind: 'text', defId: 2, name: 'PHASE', value: b }, { kind: 'text', defId: 3, name: 'Zone', value: 'z' }] });
  const out = compare(side([o('x', 'y')]), side([o('x', 'CHANGED')], { udaOffset: 10 }));
  assert.deepEqual(out.changes, [], 'neither colliding field is compared');
  assert.equal(out.ambiguousFields.count, 1);
  assert.deepEqual(out.ambiguousFields.items, [{ identity: id('G1'), key: 'uda:text:name:phase', base: 2, revised: 2 }]);
  assert.ok(out.unchecked.some((s) => s.includes('ambiguous-field')));
  // The non-colliding UDA ('Zone') is still compared.
  const third = compare(side([o('x', 'y')]), side([beam('G1', { udas: [{ kind: 'text', defId: 1, name: 'Phase', value: 'x' }, { kind: 'text', defId: 2, name: 'PHASE', value: 'y' }, { kind: 'text', defId: 3, name: 'Zone', value: 'other' }] })]));
  assert.deepEqual(third.changes[0].fields.map((f) => f.key), ['uda:text:name:zone']);
});

test('a UDA name change is a removed key plus an added key (never a silent rename)', () => {
  const out = compare(
    side([beam('G1', { udas: [{ kind: 'text', defId: 1, name: 'Phase', value: 'x' }] })]),
    side([beam('G1', { udas: [{ kind: 'text', defId: 1, name: 'Stage', value: 'x' }] })]),
  );
  assert.deepEqual(out.changes[0].fields.map((f) => [f.key, f.op]), [['uda:text:name:phase', 'removed'], ['uda:text:name:stage', 'added']]);
});

test('unit and value type changes are reported on the field', () => {
  const a = side([beam('G1')]);
  const b = side([beam('G1')]);
  b.properties.properties.find((r) => r.sourceKey === 'native:length').unit = 'in';
  const f = compare(a, b).changes[0].fields[0];
  assert.deepEqual([f.key, f.kind, f.unitBefore, f.unitAfter], ['native:length', 'shape-fields', null, 'in']);
});

test('strict profile rejection: unknown keys, missing keys, bad kinds, bad templates and bad budgets', () => {
  const bad = (mutate, pattern) => {
    const p = structuredClone(PROFILE);
    mutate(p);
    assert.throws(() => validateProfile(p), (e) => e instanceof SnapshotCompareError && e.code === 'profile-invalid' && pattern.test(e.message), pattern.source);
  };
  assert.doesNotThrow(() => validateProfile(PROFILE));
  bad((p) => { p.extra = 1; }, /unknown key 'extra'/);
  bad((p) => { delete p.budgets; }, /missing 'budgets'/);
  bad((p) => { p.schema = 'other/v1'; }, /schema must be/);
  bad((p) => { p.identity.extra = 1; }, /profile\.identity has unknown key/);
  bad((p) => { p.identity.namespace = ''; }, /namespace/);
  bad((p) => { p.kinds['native:x'] = 'relationships'; }, /must be one of/);
  bad((p) => { p.kinds['a*b'] = 'properties'; }, /invalid/);
  bad((p) => { p.keyRules[0].extra = 1; }, /keyRules\[0\] has unknown key/);
  bad((p) => { p.keyRules[0].match = '('; }, /not a valid regular expression/);
  bad((p) => { p.keyRules[0].key = 'uda:$3'; }, /capture \$3/);
  bad((p) => { p.keyRules[0].key = 'uda:{id}'; }, /unsupported placeholder/);
  bad((p) => { p.keyRules[0].key = 'uda:{name|rot13}'; }, /unsupported placeholder/);
  bad((p) => { p.geometry.field = 'position'; }, /bounds/);
  bad((p) => { p.geometry.toleranceMm = -1; }, /toleranceMm/);
  bad((p) => { p.discontinuityThreshold = 0; }, /discontinuityThreshold/);
  bad((p) => { p.budgets.changes = 1.5; }, /budgets\.changes/);
  bad((p) => { p.budgets.outputBytes = 10; }, /outputBytes/);
  bad((p) => { p.volatile = ['']; }, /volatile/);
  assert.throws(() => compareSnapshots({ base: side([]), revised: side([]), profile: { ...PROFILE, extra: 1 } }), /unknown key/);
});

test('documents must be exactly { entities | properties | relationships: [...] }', () => {
  const ok = side([beam('G1')]);
  assert.throws(() => compare({ ...ok, entities: { entities: [], extra: [] } }, ok), (e) => e.code === 'document-invalid');
  assert.throws(() => compare({ ...ok, properties: [] }, ok), (e) => e.code === 'document-invalid');
  assert.throws(() => compare(ok, { ...ok, relationships: { rows: [] } }), (e) => e.code === 'document-invalid');
  const dup = side([beam('G1'), beam('G2')]);
  dup.entities.entities[1].id = dup.entities.entities[0].id;
  assert.throws(() => compare(ok, dup), /two entities with id/);
  const badBounds = side([beam('G1')]);
  badBounds.entities.entities[0].geometry = [{ tileOrdinal: 0, bounds: [1, 2] }];
  assert.throws(() => compare(ok, badBounds), /six finite numbers/);
  assert.throws(() => compare(ok, ok, { witness: { entries: [{ identity: 'x', action: 'moved' }] } }), (e) => e.code === 'witness-invalid');
  assert.throws(() => compare(ok, ok, { witness: { entries: [], extra: 1 } }), (e) => e.code === 'witness-invalid');
});

test('PERFORMANCE SMOKE: 25,000 vs 25,000 synthetic objects compare in under 10 seconds', () => {
  const N = 25000;
  const gen = (edit) => side(Array.from({ length: N }, (_, i) => beam(`G${i}`, {
    udas: [{ kind: 'text', defId: 1, name: 'Phase', value: `P${i % 7}` }],
    props: { 'native:profile': edit && i % 500 === 0 ? 'HEB300' : 'HEA200', 'native:length': 3000 + (i % 13), 'native:material': 'S355', 'native:class': `${i % 5}` },
    bounds: [i, 0, 0, i + 3000, 200, 190],
  })), { idOffset: edit ? 100000 : 0 });
  const base = gen(false);
  const revised = gen(true);
  const started = performance.now();
  const out = compare(base, revised);
  const elapsedMs = performance.now() - started;
  assert.equal(out.identity.paired, N);
  assert.equal(out.summary.changed, 50, 'every 500th object changed profile');
  assertReconciles(out);
  assert.ok(elapsedMs < 10000, `took ${elapsedMs.toFixed(0)} ms`);
  const serialised = canonicalJson(out);
  assert.ok(Buffer.byteLength(serialised) < PROFILE.budgets.outputBytes);
});

test('REAL SNAPSHOT (optional): the facade documents of a published snapshot compare with themselves to zero changes', (t) => {
  const dir = process.env.AWARE_CHANGE_SET_REAL_SNAPSHOT_DIR;
  if (!dir || !existsSync(join(dir, 'entities.json'))) return t.skip('set AWARE_CHANGE_SET_REAL_SNAPSHOT_DIR to a directory holding a real snapshot\'s entities/properties/relationships.json');
  const read = (name) => JSON.parse(readFileSync(join(dir, name), 'utf8'));
  const doc = { entities: read('entities.json'), properties: read('properties.json'), relationships: read('relationships.json') };
  const out = compare(doc, doc);
  assert.equal(out.summary.changed + out.summary.added + out.summary.removed, 0);
  assert.equal(out.identity.discontinuity, false);
  assertReconciles(out);
});

test('a paired object with no bounding box in either state is counted as placement-not-compared, never silently unchanged', () => {
  // A real Tekla read (the Feasibility model, 2026-11-10) left one beam with `geometry: []` in both states; it had
  // been moved 500 mm and the change set called it unchanged without a word.
  const objects = [beam('G1'), beam('G2', { bounds: undefined }), beam('G3', { bounds: undefined })];
  const out = compare(side(objects), side(objects, { idOffset: 50 }));
  assert.equal(out.summary.unchanged, 3, 'nothing comparable differs');
  assert.ok(out.unchecked.includes('2 paired object(s) have no bounding box in either saved state, so whether they moved was not checked.'), out.unchecked.join(' | '));
  // Negative control: with bounds everywhere the statement is absent.
  const all = [beam('G1'), beam('G2'), beam('G3')];
  const clean = compare(side(all), side(all, { idOffset: 50 }));
  assert.ok(!clean.unchecked.some((s) => s.includes('no bounding box')), clean.unchecked.join(' | '));
});

// ─── placement fields and per-field tolerances (floless.app#3365) ──────────────────────────────────────
// A provider may publish where an object sits as plain numeric fields (a start point, a direction). The profile
// marks them `placement` and says how far each may differ; AWARE applies that and knows nothing else about them.
const PLACED = profileWith({
  kinds: { ...PROFILE.kinds, 'native:start-point-*': 'placement', 'native:axis-direction-*': 'placement' },
  tolerances: { 'native:start-point-*': 0.5, 'native:axis-direction-*': 0.001 },
});
const placedBeam = (guid, x, axisX = 1, extra = {}) => beam(guid, {
  bounds: undefined, props: { ...beam(guid).props, 'native:start-point-x': x, 'native:axis-direction-x': axisX }, ...extra,
});

test('placement fields: a 500 mm move of an object with no bounding box is a placement change', () => {
  const base = side([placedBeam('G1', 0), placedBeam('G2', 0)]);
  const revised = side([placedBeam('G1', 500), placedBeam('G2', 0)], { idOffset: 50 });
  const out = compareSnapshots({ base, revised, profile: PLACED });
  const moved = rowFor(out, id('G1'));
  assert.deepEqual(moved.kinds, ['placement']);
  assert.deepEqual(moved.fields.map((f) => [f.key, f.kind, f.op, f.before, f.after]),
    [['native:start-point-x', 'placement', 'modified', 0, 500]]);
  assert.equal(rowFor(out, id('G2')), undefined);
  assert.equal(out.summary.byKind.placement, 1);
  // The fields count as placement evidence: nothing is listed as not checked, and the wording names the fields.
  assert.ok(!out.unchecked.some((s) => s.includes('was not checked')), out.unchecked.join(' | '));
  assert.ok(out.unchecked.some((s) => s.startsWith("Placement was compared on each object's position fields")));
  assert.ok(out.unchecked.includes('Solid geometry was not compared; only the listed fields and bounding boxes were.'));
  assert.ok(!out.unchecked.some((s) => s.includes('bounding box only')), 'the bounding-box-only claim is not made');
  // Negative control: the same move under a profile that does not mark the field placement is a property change,
  // and the objects still count as placement-not-compared.
  const plain = compareSnapshots({ base, revised, profile: PROFILE });
  assert.deepEqual(rowFor(plain, id('G1')).kinds, ['properties']);
  assert.ok(plain.unchecked.includes('2 paired object(s) have no bounding box in either saved state, so whether they moved was not checked.'));
});

test('tolerances: a difference up to the field tolerance is the same value; beyond it is a change', () => {
  const base = side([placedBeam('G1', 0), placedBeam('G2', 0), placedBeam('G3', 0, 1), placedBeam('G4', 0, 1)]);
  const revised = side([placedBeam('G1', 0.5), placedBeam('G2', 0.5001), placedBeam('G3', 0, 0.9995), placedBeam('G4', 0, 0.998)]);
  const out = compareSnapshots({ base, revised, profile: PLACED });
  assert.equal(rowFor(out, id('G1')), undefined, 'a difference equal to the tolerance is not a change');
  assert.deepEqual(rowFor(out, id('G2')).fields.map((f) => f.key), ['native:start-point-x']);
  assert.equal(rowFor(out, id('G3')), undefined);
  assert.deepEqual(rowFor(out, id('G4')).fields.map((f) => f.key), ['native:axis-direction-x']);
  // Without tolerances every difference is a change.
  const exact = compareSnapshots({ base, revised, profile: profileWith({ kinds: PLACED.kinds }) });
  assert.equal(exact.summary.changed, 4);
});

test('tolerances never hide a unit, type or presence change, and apply only to finite numbers', () => {
  const base = side([placedBeam('G1', 0), placedBeam('G2', 0), placedBeam('G3', 0)]);
  const revised = side([placedBeam('G1', '0'), placedBeam('G2', 0), beam('G3', { bounds: undefined })]);
  revised.properties.properties.find((row) => row.entityId === 'element:2' && row.sourceKey === 'native:start-point-x').unit = 'in';
  const out = compareSnapshots({ base, revised, profile: PLACED });
  assert.deepEqual(rowFor(out, id('G1')).fields.map((f) => [f.key, f.before, f.after, f.typeBefore, f.typeAfter]),
    [['native:start-point-x', 0, '0', 'number', 'string']]);
  assert.deepEqual(rowFor(out, id('G2')).fields.map((f) => [f.key, f.unitBefore, f.unitAfter]), [['native:start-point-x', null, 'in']]);
  assert.deepEqual(rowFor(out, id('G3')).fields.map((f) => [f.key, f.op]).sort(),
    [['native:axis-direction-x', 'removed'], ['native:start-point-x', 'removed']]);
});

test('profile.tolerances is optional and strict: exact or single-* prefix keys, non-negative finite numbers', () => {
  assert.doesNotThrow(() => validateProfile(PROFILE));
  assert.doesNotThrow(() => validateProfile(PLACED));
  for (const tolerances of [[], { 'a*b': 1 }, { '*': 1 }, { '': 1 }, { a: -1 }, { a: '1' }, { a: Number.POSITIVE_INFINITY }, { a: null }]) {
    assert.throws(() => validateProfile(profileWith({ tolerances })),
      (error) => error instanceof SnapshotCompareError && error.code === 'profile-invalid', JSON.stringify(tolerances));
  }
  // Exact beats prefix; the longest prefix wins.
  const compiled = validateProfile(profileWith({ tolerances: { 'native:a*': 1, 'native:ab*': 2, 'native:abc': 3 } }));
  assert.deepEqual(['native:a', 'native:ab1', 'native:abc', 'native:x'].map((key) => compiled.toleranceOf(key)), [1, 2, 3, undefined]);
  // The profile digest covers tolerances, so a review records which tolerances it used.
  const a = compareSnapshots({ base: side([beam('G1')]), revised: side([beam('G1')]), profile: PLACED });
  const b = compareSnapshots({ base: side([beam('G1')]), revised: side([beam('G1')]),
    profile: profileWith({ ...PLACED, tolerances: { 'native:start-point-*': 0.6 } }) });
  assert.notEqual(a.inputs.profileDigest, b.inputs.profileDigest);
});

test('an ambiguous placement key alone does not count as placement compared', () => {
  const make = () => {
    const doc = side([beam('G1', { bounds: undefined })]);
    for (const value of [1, 2]) {
      doc.properties.properties.push({ id: `p${value}`, entityId: 'element:1', name: 'x', value, valueType: 'number', unit: null, sourceKey: 'native:start-point-x' });
    }
    return doc;
  };
  const out = compareSnapshots({ base: make(), revised: make(), profile: PLACED });
  assert.equal(out.ambiguousFields.count, 1);
  assert.ok(out.unchecked.includes('1 paired object(s) have no bounding box and no placement field in either saved state, so whether they moved was not checked.'),
    out.unchecked.join(' | '));
});
