/**
 * The single-object rules of `aware.model-check-profile/v2` and the read-coverage input they need.
 *
 * Four predicates on top of `blank`: `property-missing`, `property-not-in`, `property-out-of-range` (they read one
 * property row, found by its CANONICAL key: the key after the comparison profile's `keyRules`, formed by the very
 * function `compare-snapshots` uses) and `relationship-missing` (it reads the side's SOURCE relationship rows).
 *
 * FORMAT-BLIND. Nothing here knows what a key means. A rule names a key, a list, a bound or a relationship kind as
 * data, and this file only applies it.
 *
 * NOT EVALUATED IS NOT PASSED. The snapshot facade cannot say by itself whether "no row" means "not set" or "not
 * read". So a predicate is judged for an object only when the side's read coverage (`aware.model-read-coverage/v1`)
 * affirmatively covers what the predicate reads; a value that is absent, not a number or in another unit than the rule
 * is not judged either. Each of those is a labelled `notEvaluated` reason, never a pass and never a finding.
 */
import { comparisonKey, exactKeys, fail, isObject, providerRelationKindsOf, SnapshotCompareError } from './snapshot-compare.mjs';

const PROVIDER_EXPLICIT = 'provider-explicit';

export const COVERAGE_SCHEMA = 'aware.model-read-coverage/v1';
export const PROPERTY_TARGET = 'property';
export const RELATIONSHIP_TARGET = 'relationship';
export const PROPERTY_PREDICATES = ['property-missing', 'property-not-in', 'property-out-of-range'];
export const RELATIONSHIP_PREDICATES = ['relationship-missing'];
export const V2_PREDICATES = [...PROPERTY_PREDICATES, ...RELATIONSHIP_PREDICATES];
export const V2_TARGETS = [PROPERTY_TARGET, RELATIONSHIP_TARGET];
export const MATCH_MODES = ['exact', 'nfkc-casefold'];
export const ENDS = ['to', 'from'];
export const NOT_EVALUATED_REASONS = [
  'coverage-not-supplied', 'source-did-not-read', 'value-missing', 'value-not-numeric', 'unit-mismatch',
  'key-ambiguous', 'identity-unusable', 'relationship-endpoint-unusable', 'key-is-identity',
];
export const KEY_IS_IDENTITY_TEXT = "this rule names the property used as the object's identity, so it cannot be checked reliably";
export const LIMITS = { values: 5000, valueChars: 64, keyChars: 512, kindChars: 256, unitChars: 64 };
export const OUT_OF_RANGE_WORSENING = { path: 'distanceOutside', direction: 'higher-is-worse' };

const boundedString = (v, max) => typeof v === 'string' && v !== '' && v.length <= max;
// `nfkc-casefold` is defined, not approximated: NFKC, then lower case, then the three folds lower-casing misses, ß and ẞ
// to `ss` and the final sigma to the plain one. Letters Unicode keeps distinct (a dotless ı is not an i) stay distinct.
// Spelled out so the same rule gives the same answer in every build.
const foldText = (text) => text.normalize('NFKC').toLowerCase().replace(/[\u00DF\u1E9E]/g, 'ss').replace(/\u03C2/g, '\u03C3');
const identity = (text) => text;
const isBlankValue = (v) => v === null || v === undefined || (typeof v === 'string' && v.normalize('NFKC').trim() === '');

function propertyKey(key, where) {
  if (!boundedString(key, LIMITS.keyChars)) fail('profile-invalid', `${where}.key must be a non-empty string of at most ${LIMITS.keyChars} characters`);
  if (key.includes('*')) fail('profile-invalid', `${where}.key must be one exact canonical property key (no '*')`);
}

/** Validate and compile a v2 rule's `parameters` for its predicate. Unknown keys and missing keys are rejected. */
export function compileParameters(rule, label) {
  const where = `${label}.parameters`;
  const p = rule.parameters;
  if (p === undefined) fail('profile-invalid', `${label}.parameters is required for the '${rule.predicate}' predicate`);
  if (rule.predicate === 'property-missing') {
    exactKeys(p, ['key'], where);
    propertyKey(p.key, where);
    return { key: p.key };
  }
  if (rule.predicate === 'property-not-in') {
    exactKeys(p, ['key', 'values', 'match'], where);
    propertyKey(p.key, where);
    if (!MATCH_MODES.includes(p.match)) fail('profile-invalid', `${where}.match must be one of ${MATCH_MODES.join(', ')}`);
    if (!Array.isArray(p.values) || p.values.length === 0 || p.values.length > LIMITS.values) {
      fail('profile-invalid', `${where}.values must be an array of 1 to ${LIMITS.values} strings`);
    }
    for (const [index, value] of p.values.entries()) {
      if (!boundedString(value, LIMITS.valueChars)) fail('profile-invalid', `${where}.values[${index}] must be a non-empty string of at most ${LIMITS.valueChars} characters`);
    }
    const fold = p.match === 'nfkc-casefold' ? foldText : identity;
    return { key: p.key, match: p.match, valueSet: new Set(p.values.map(fold)), fold };
  }
  if (rule.predicate === 'property-out-of-range') {
    exactKeys(p, ['key', 'unit'], where, ['min', 'max']);
    propertyKey(p.key, where);
    if (!boundedString(p.unit, LIMITS.unitChars)) fail('profile-invalid', `${where}.unit must be a non-empty string of at most ${LIMITS.unitChars} characters`);
    for (const bound of ['min', 'max']) {
      if (p[bound] !== undefined && (typeof p[bound] !== 'number' || !Number.isFinite(p[bound]))) fail('profile-invalid', `${where}.${bound} must be a finite number`);
    }
    if (p.min === undefined && p.max === undefined) fail('profile-invalid', `${where} needs at least one of min and max`);
    if (p.min !== undefined && p.max !== undefined && p.min > p.max) fail('profile-invalid', `${where}.min must not be greater than max`);
    return { key: p.key, unit: p.unit, min: p.min ?? null, max: p.max ?? null };
  }
  // relationship-missing
  exactKeys(p, ['kind', 'end'], where, ['providerRelationKinds']);
  if (!boundedString(p.kind, LIMITS.kindChars)) fail('profile-invalid', `${where}.kind must be a non-empty string of at most ${LIMITS.kindChars} characters`);
  if (!ENDS.includes(p.end)) fail('profile-invalid', `${where}.end must be one of ${ENDS.join(', ')}`);
  if (!('providerRelationKinds' in p)) return { kind: p.kind, end: p.end, providerKinds: null };
  // #882: `provider-explicit` rows whose providerRelationKind is listed count as rows of `kind`. `kind` is then the
  // semantic kind; naming `provider-explicit` itself there would alias the family onto itself.
  if (p.kind === PROVIDER_EXPLICIT) fail('profile-invalid', `${where}.providerRelationKinds needs a semantic kind (such as hosts), not '${PROVIDER_EXPLICIT}'`);
  return { kind: p.kind, end: p.end, providerKinds: providerRelationKindsOf(p.providerRelationKinds, `${where}.providerRelationKinds`) };
}

// ─── read coverage ──────────────────────────────────────────────────────────────────────────────────

// exactKeys reports `profile-invalid`; a coverage document has its own code.
function coverageKeys(value, keys, label) {
  try { exactKeys(value, keys, label); } catch (error) {
    if (error instanceof SnapshotCompareError) fail('coverage-invalid', error.message);
    throw error;
  }
}

/**
 * `{ schema, entities: { "<entityId>": { properties: [exact key | "<prefix>:*"] } }, relationships: { complete: [kind] } }`.
 * A family ends at a colon boundary (`uda:*` covers `uda:text:name:x`); a bare `*` is rejected. Matching is against the
 * CANONICAL key. An entity absent from `entities` has nothing read.
 */
export function compileCoverage(doc, label) {
  coverageKeys(doc, ['schema', 'entities', 'relationships'], label);
  if (doc.schema !== COVERAGE_SCHEMA) fail('coverage-invalid', `${label}.schema must be '${COVERAGE_SCHEMA}'`);
  if (!isObject(doc.entities)) fail('coverage-invalid', `${label}.entities must be an object keyed by entity id`);
  const entities = new Map();
  for (const [entityId, entry] of Object.entries(doc.entities)) {
    const where = `${label}.entities['${entityId}']`;
    if (entityId === '') fail('coverage-invalid', `${label}.entities has an empty entity id`);
    coverageKeys(entry, ['properties'], where);
    if (!Array.isArray(entry.properties)) fail('coverage-invalid', `${where}.properties must be an array`);
    const exact = new Set();
    const prefixes = [];
    for (const item of entry.properties) {
      if (!boundedString(item, LIMITS.keyChars)) fail('coverage-invalid', `${where}.properties must hold non-empty strings`);
      if (item.endsWith(':*')) {
        const prefix = item.slice(0, -1);   // keeps the colon: the family boundary
        if (prefix === ':' || prefix.includes('*')) fail('coverage-invalid', `${where}.properties '${item}' is not a family '<prefix>:*'`);
        prefixes.push(prefix);
      } else if (item.includes('*')) fail('coverage-invalid', `${where}.properties '${item}' is invalid (an exact key, or a family '<prefix>:*')`);
      else exact.add(item);
    }
    entities.set(entityId, { exact, prefixes });
  }
  coverageKeys(doc.relationships, ['complete'], `${label}.relationships`);
  if (!Array.isArray(doc.relationships.complete) || doc.relationships.complete.some((k) => !boundedString(k, LIMITS.kindChars))) {
    fail('coverage-invalid', `${label}.relationships.complete must be an array of non-empty relationship kinds`);
  }
  return { entities, complete: new Set(doc.relationships.complete) };
}

/** null when the side's coverage affirmatively covers the key on this entity, else the notEvaluated reason. */
function propertyCoverageReason(coverage, entityId, key) {
  if (coverage === null) return 'coverage-not-supplied';
  const read = coverage.entities.get(entityId);
  if (!read) return 'source-did-not-read';
  if (read.exact.has(key) || read.prefixes.some((prefix) => key.startsWith(prefix))) return null;
  return 'source-did-not-read';
}
const relationshipCoverageReason = (coverage, kind) => (coverage === null ? 'coverage-not-supplied' : coverage.complete.has(kind) ? null : 'source-did-not-read');

// ─── evaluation ─────────────────────────────────────────────────────────────────────────────────────

/** The rows of an entity by canonical key (the caller memoises it per entity). */
export function canonicalRows(rows, compiledProfile) {
  const keyed = new Map();
  for (const row of rows ?? []) {
    const key = comparisonKey(row, compiledProfile);
    const list = keyed.get(key);
    if (list) list.push(row); else keyed.set(key, [row]);
  }
  return keyed;
}

const notEvaluated = (reason) => ({ status: 'not-evaluated', reason });
const passed = { status: 'passed' };
const found = (evidence) => ({ status: 'finding', evidence, locus: {} });

/**
 * Judge one object on one side. `ctx`: { side (indexed side), coverage (compiled or null), entityId, keyed (Map by canonical key) }.
 * Returns { status: 'finding', evidence, locus } | { status: 'passed' } | { status: 'not-evaluated', reason }.
 */
export function evaluateSingleRule(rule, ctx) {
  const { params } = rule;
  if (rule.predicate === 'relationship-missing') {
    if (!params.providerKinds) {
      // Today's order, unchanged: coverage first, then presence.
      const reason = relationshipCoverageReason(ctx.coverage, params.kind);
      if (reason) return notEvaluated(reason);
      const ends = ctx.side.relationshipEnds.get(params.kind);
      if (ends && ends[params.end].has(ctx.entityId)) return passed;
      // A row of this kind may have ended here but was discarded because its other end has no usable identity: whether the
      // object has the relationship is then unknown, not "missing".
      if (ends && ends[params.end === 'to' ? 'toLost' : 'fromLost'].has(ctx.entityId)) return notEvaluated('relationship-endpoint-unusable');
      return found({ kind: params.kind, end: params.end });
    }
    // #882, with providerRelationKinds: a usable row of either family proves the relationship, whatever was read. Saying it
    // is MISSING needs both families read completely: `kind`, and every `provider-explicit` row.
    const families = [ctx.side.relationshipEnds.get(params.kind)];
    const index = ctx.side.providerRelations(params.providerKinds);
    for (const providerKind of params.providerKinds) families.push(index.get(providerKind).ends);
    const present = families.filter(Boolean);
    if (present.some((ends) => ends[params.end].has(ctx.entityId))) return passed;
    const reason = relationshipCoverageReason(ctx.coverage, params.kind) ?? relationshipCoverageReason(ctx.coverage, PROVIDER_EXPLICIT);
    if (reason) return notEvaluated(reason);
    const lost = params.end === 'to' ? 'toLost' : 'fromLost';
    if (present.some((ends) => ends[lost].has(ctx.entityId))) return notEvaluated('relationship-endpoint-unusable');
    return found({ kind: params.kind, end: params.end, providerRelationKinds: params.providerKinds });
  }
  const reason = propertyCoverageReason(ctx.coverage, ctx.entityId, params.key);
  if (reason) return notEvaluated(reason);
  const rows = ctx.keyed.get(params.key);
  if (rows && rows.length > 1) return notEvaluated('key-ambiguous');
  const row = rows ? rows[0] : null;
  const value = row ? (row.value ?? null) : null;
  if (rule.predicate === 'property-missing') {
    if (row === null || isBlankValue(value)) return found({ key: params.key, present: row !== null, value });
    return passed;
  }
  if (row === null || isBlankValue(value)) return notEvaluated('value-missing');
  if (rule.predicate === 'property-not-in') {
    const text = typeof value === 'string' ? value : typeof value === 'object' ? JSON.stringify(value) : String(value);
    return params.valueSet.has(params.fold(text)) ? passed : found({ key: params.key, value, match: params.match });
  }
  // property-out-of-range
  if (typeof value !== 'number' || !Number.isFinite(value)) return notEvaluated('value-not-numeric');
  if ((row.unit ?? null) !== params.unit) return notEvaluated('unit-mismatch');
  const distanceOutside = params.min !== null && value < params.min ? params.min - value
    : params.max !== null && value > params.max ? value - params.max : 0;
  return distanceOutside > 0
    ? found({ key: params.key, value, unit: params.unit, min: params.min, max: params.max, distanceOutside })
    : passed;
}
