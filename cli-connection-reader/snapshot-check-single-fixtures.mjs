// Synthetic facade documents for the single-mode tests of `check-snapshots` (model-change-set 0.5.0). Invented
// vocabulary: AWARE has no format knowledge, so every key here is just data.
import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { canonicalJson } from './snapshot-compare.mjs';
import { GUID_KEY, PROFILE } from './snapshot-compare-fixtures.mjs';
import { CHECK_PROFILE_SCHEMA_V2, COVERAGE_SCHEMA } from './snapshot-check.mjs';

export const GRADE = 'native:grade';
export const LENGTH = 'native:length';
export const id = (guid) => `lineage-A:${guid}`;
export const grade = (value) => ({ key: GRADE, value });

/** objects: { guid, type, name, guidName, props: [{ key, value, unit?, name? }] }; rels: [kind, fromGuid|'element:n', toGuid|'element:n']. */
export function side(objects, rels = []) {
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
    if (o.guid) push(n++, { key: GUID_KEY, value: o.guid, name: o.guidName });
    for (const p of o.props ?? []) push(n++, p);
  });
  const ref = (token) => (token.startsWith('element:') ? token : entityIdOf.get(token));
  const relationships = rels.map(([kind, from, to], i) => ({ id: `rel:${i}`, kind, from: ref(from), to: ref(to) }));
  return { entities: { entities }, properties: { properties }, relationships: { relationships } };
}

/** A coverage document: every entity reads `read`; `perObject[guid]` overrides it; `omit` leaves entities out. */
export function coverageOf(objects, { read = ['native:*', 'uda:*'], kinds = ['hosts'], perObject = {}, omit = [] } = {}) {
  const entities = {};
  objects.forEach((o, index) => {
    if (omit.includes(o.guid)) return;
    entities[`element:${index + 1}`] = { properties: perObject[o.guid] ?? read };
  });
  return { schema: COVERAGE_SCHEMA, entities, relationships: { complete: kinds } };
}

export const rule = (predicate, parameters, patch = {}) => ({
  id: `firm.${predicate}`, version: '1.0.0', severity: 'warning', predicate,
  target: predicate === 'relationship-missing' ? 'relationship' : predicate === 'blank' ? 'entity.name' : 'property',
  appliesTo: { types: ['Beam'] }, worsening: null, ...(parameters === undefined ? {} : { parameters }), ...patch,
});
export const V2 = (rules, patch = {}) => ({
  schema: CHECK_PROFILE_SCHEMA_V2, id: 'firm.standards', version: '1.0.0', scope: 'all',
  budgets: { findings: 2000, focusResults: 20000 }, rules, ...patch,
});

/** Every one-object predicate, each with a rule id of its own. */
export const ALL_RULES = () => [
  rule('blank', undefined),
  rule('property-missing', { key: GRADE }),
  rule('property-not-in', { key: GRADE, values: ['S355', 'S275'], match: 'exact' }),
  rule('property-out-of-range', { key: LENGTH, min: 100, max: 500, unit: 'mm' }),
  rule('relationship-missing', { kind: 'hosts', end: 'to' }),
];

/** A later state that trips every predicate at least once, and leaves some objects unjudged for a labelled reason. */
export function richObjects() {
  return [
    { guid: 'G1', props: [grade('S355'), { key: LENGTH, value: 250, unit: 'mm' }] },        // clean for grade and length
    { guid: 'G2', props: [{ key: LENGTH, value: 900, unit: 'mm' }] },                      // no grade, too long
    { guid: 'G3', props: [grade('S999'), { key: LENGTH, value: 'long', unit: 'mm' }] },    // grade not in list, length not numeric
    { guid: 'G4', name: '   ', props: [grade('S275'), { key: LENGTH, value: 10, unit: 'cm' }] },   // blank name, wrong unit
    { guid: 'G5', props: [grade('S355')] },                                                // no length row
    { guid: 'H1', type: 'Host', name: 'host', props: [] },                                // not applicable (type)
  ];
}
export const richRelationships = () => [['hosts', 'G1', 'H1']];

/** Write a state's three documents as the files the bridge reads. */
export function writeSide(dir, doc) {
  mkdirSync(dir, { recursive: true });
  writeFileSync(join(dir, 'entities.json'), canonicalJson(doc.entities));
  writeFileSync(join(dir, 'properties.json'), canonicalJson(doc.properties));
  writeFileSync(join(dir, 'relationships.json'), canonicalJson(doc.relationships));
}

export { PROFILE, GUID_KEY };
