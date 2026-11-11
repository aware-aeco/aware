// Inputs for the compare-snapshots golden fixtures (test-fixtures/compare-golden/*.json). The goldens were
// produced by the ORIGINAL `snapshot-compare.mjs` from origin/main (commit 1a8bcf000) before it was refactored
// to share its pairing with check-snapshots; `snapshot-compare.golden.test.mjs` asserts the current code gives
// exactly the same bytes. Plain data and one document builder; no AWARE code is imported except the profile
// fixture, so the same module drives both the generator and the test.
import { GUID_KEY, PROFILE } from './snapshot-compare-fixtures.mjs';

/** objects: { guid, extraGuids, type, name, bounds, props: {sourceKey: value}, udas: [{kind, defId, name, value}] } */
export function side(objects, { relationships = [] } = {}) {
  const entities = [];
  const properties = [];
  const entityIdOf = new Map();
  objects.forEach((o, index) => {
    const id = `element:${index + 1}`;
    if (o.guid) entityIdOf.set(o.guid, id);
    entities.push({ id, type: o.type ?? 'BEAM', name: o.name ?? `part ${index}`, geometry: o.bounds ? [{ tileOrdinal: 0, bounds: o.bounds }] : [] });
    let n = 0;
    const push = (sourceKey, name, value) => {
      properties.push({ id: `prop:${id}:${n}`, entityId: id, groupName: 'g', groupOrdinal: 0, name, value, valueType: typeof value === 'number' ? 'number' : 'string', unit: null, propertyOrdinal: n, semantic: 'x', dimension: 'none', isUda: sourceKey.startsWith('uda:'), sourceKey });
      n += 1;
    };
    if (o.guid) push(GUID_KEY, 'GUID', o.guid);
    for (const guid of o.extraGuids ?? []) push(GUID_KEY, 'GUID', guid);
    push('native:id', 'ID', 100000 + index);
    for (const [key, value] of Object.entries(o.props ?? {})) push(key, key.split(':')[1], value);
    for (const uda of o.udas ?? []) push(`uda:${uda.kind}:${uda.defId}`, uda.name, uda.value);
  });
  const rels = relationships.map(([kind, from, to], i) => ({ id: `rel:${i}`, kind, from: entityIdOf.get(from), to: entityIdOf.get(to) }));
  return { entities: { entities }, properties: { properties }, relationships: { relationships: rels } };
}

const beam = (guid, extra = {}) => ({ guid, name: `B-${guid}`, props: { 'native:profile': 'HEA200', 'native:length': 3000, 'native:material': 'S355' }, bounds: [0, 0, 0, 3000, 200, 190], ...extra });
const budgets = (patch) => ({ ...PROFILE.budgets, ...patch });

const many = (prefix, count, extra = () => ({})) => Array.from({ length: count }, (_, i) => beam(`${prefix}${i}`, extra(i)));

export const CASES = {
  basic: {
    base: side([beam('G1'), beam('G2'), beam('G3', { udas: [{ kind: 'text', defId: 5, name: 'Phase', value: 'P1' }] }), beam('G4'), beam('G5')], { relationships: [['welded', 'G1', 'G2'], ['welded', 'G4', 'G5']] }),
    revised: side([beam('G1'), beam('G2', { props: { 'native:profile': 'HEB300', 'native:length': 3000, 'native:material': 'S355' } }), beam('G3', { udas: [{ kind: 'text', defId: 9, name: 'Phase', value: 'P2' }] }), beam('G4', { bounds: [500, 0, 0, 3500, 200, 190] }), beam('G6'), beam('G7')], { relationships: [['welded', 'G1', 'G2'], ['welded', 'G1', 'G4']] }),
    profile: PROFILE,
    witness: { entries: [{ identity: 'lineage-A:G2', action: 'modified' }, { identity: 'lineage-A:G6', action: 'created' }, { identity: 'G5', action: 'deleted' }, { identity: 'lineage-A:G1', action: 'modified' }] },
    digests: { base: { entities: 'b1', properties: 'b2', relationships: 'b3' }, revised: { entities: 'r1', properties: 'r2', relationships: 'r3' }, witness: 'w1' },
  },
  capped: {
    base: side([...many('K', 6), ...many('X', 5)]),
    revised: side([...many('K', 6, (i) => ({ props: { 'native:profile': `HEB${i}`, 'native:length': 3000 } })), ...many('N', 7, (i) => ({ name: `new ${i % 2}` }))]),
    profile: { ...PROFILE, budgets: budgets({ changes: 3, groupIds: 2, receiptsPerKind: 1 }) },
    witness: { entries: [{ identity: 'lineage-A:K0', action: 'modified' }, { identity: 'lineage-A:ZZ', action: 'created' }, { identity: 'lineage-A:N1', action: 'created' }] },
    digests: { witness: 'w2' },
  },
  bytebudget: {
    base: side(many('K', 30)),
    revised: side(many('K', 30, (i) => ({ props: { 'native:profile': `HEB${i}`, 'native:length': 3000 + i } }))),
    profile: { ...PROFILE, budgets: budgets({ outputBytes: 6000 }) },
  },
  ambiguous: {
    base: side([beam('A1', { udas: [{ kind: 'text', defId: 5, name: 'Phase', value: 'P1' }, { kind: 'text', defId: 6, name: 'phase', value: 'P1' }] }), beam('A2', { udas: [{ kind: 'text', defId: 5, name: 'Zone', value: 'Z1' }] })]),
    revised: side([beam('A1', { udas: [{ kind: 'text', defId: 7, name: 'PHASE', value: 'P2' }, { kind: 'text', defId: 8, name: 'phase', value: 'P3' }] }), beam('A2', { udas: [{ kind: 'text', defId: 7, name: 'Zone', value: 'Z2' }] })]),
    profile: PROFILE,
  },
  identities: {
    base: side([beam('D1'), beam('D2'), beam('S1'), { ...beam('B1'), guid: null }, beam('T1'), beam('T1', { name: 'second T1' }), beam('U1', { extraGuids: ['U2'] })]),
    revised: side([beam('D1'), beam('D2', { extraGuids: ['D2'] }), beam('S1'), beam('S1', { name: 'dup S1' }), { ...beam('B2'), guid: null }, beam('T1'), beam('U2'), beam('V1')]),
    profile: PROFILE,
  },
  discontinuity: {
    base: side(many('O', 6)),
    revised: side([...many('P', 6), beam('O0')]),
    profile: PROFILE,
  },
  placement: {
    base: side([beam('M1', { bounds: undefined, props: { 'native:x': 100, 'native:len': 5 } }), beam('M2', { bounds: undefined, props: { 'native:x': 100 } }), beam('M3', { bounds: undefined })]),
    revised: side([beam('M1', { bounds: undefined, props: { 'native:x': 100.4, 'native:len': 9 } }), beam('M2', { bounds: undefined, props: { 'native:x': 500 } }), beam('M3', { bounds: undefined })]),
    profile: { ...PROFILE, kinds: { ...PROFILE.kinds, 'native:x': 'placement' }, tolerances: { 'native:x': 0.5, 'native:l*': 1 } },
  },
};
