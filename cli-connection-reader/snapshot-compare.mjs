/**
 * compare-snapshots — pair the objects of two saved states of one model by a native identity and say
 * what was added, removed and changed (`aware.model-change-set/v1`).
 *
 * PURE AND FORMAT-BLIND ON PURPOSE. The `model-reference-reader` contract keeps format meaning inside an
 * enrolled read-only provider; AWARE validates only the generic interoperability shape. So nothing in
 * here knows what Tekla, a UDA or `native:guid` is. The caller pins a comparison PROFILE (data) that says
 * which property row is an object's identity, which keys are volatile, how a property's comparison key is
 * formed, which keys are shape fields or placement fields, how far a bounding box may move and (optional
 * `tolerances`) how far a numeric field may differ and still be the same value. This file only applies it.
 *
 * Inputs are the three canonical query documents of each side (`{entities}`, `{properties}`,
 * `{relationships}`), exactly as a caller retains them. The exported I/O wrapper hashes each file's bytes
 * and echoes the digests; the caller (which holds the signed receipts) decides whether they match.
 *
 * GUARANTEES
 *  - Pairing is by identity ONLY. Grouping of unpaired objects (type + name) is a presentation of
 *    unpaired objects and never a pairing.
 *  - Numeric ids (`element:<n>`, property ids, UDA definition ids) never decide anything: relationships
 *    are compared as (kind, identity(from), identity(to)), and a property is keyed by the profile's key
 *    rules, not by its row id.
 *  - Counts are always complete; rows are capped afterwards, in a pinned order, and the cap is stated.
 *  - Canonical JSON (sorted keys): the same inputs give byte-identical output.
 */
import { createHash } from 'node:crypto';
import { existsSync, readFileSync, statSync } from 'node:fs';
import { join, resolve } from 'node:path';

export const CHANGE_SET_SCHEMA = 'aware.model-change-set/v1';
export const PROFILE_SCHEMA = 'aware.model-compare-profile/v1';
export const COMPARABLE_KINDS = ['shape-fields', 'placement', 'properties'];
const KIND_ORDER = ['shape-fields', 'placement', 'properties', 'relationships'];
const WITNESS_ACTIONS = ['created', 'modified', 'deleted'];
const FIELD_CAP = 50;
const BOUNDS_KEY = 'geometry.bounds';

export class SnapshotCompareError extends Error {
  constructor(code, message) {
    super(message);
    this.name = 'SnapshotCompareError';
    this.code = code;
  }
}
export const fail = (code, message) => { throw new SnapshotCompareError(code, message); };

// ─── canonical JSON + digests ───────────────────────────────────────────────────────────────────────

/** JSON with every object's keys sorted by UTF-16 code unit, so equal values are equal bytes. */
export function canonicalJson(value) {
  if (value === null || typeof value !== 'object') {
    if (typeof value === 'number' && !Number.isFinite(value)) fail('non-finite-number', 'canonical JSON cannot carry a non-finite number');
    return JSON.stringify(value);
  }
  if (Array.isArray(value)) return `[${value.map((item) => (item === undefined ? 'null' : canonicalJson(item))).join(',')}]`;
  const keys = Object.keys(value).filter((key) => value[key] !== undefined).sort();
  return `{${keys.map((key) => `${JSON.stringify(key)}:${canonicalJson(value[key])}`).join(',')}}`;
}
export const sha256 = (bytes) => createHash('sha256').update(bytes).digest('hex');
export const codeUnitCompare = (a, b) => (a < b ? -1 : a > b ? 1 : 0);

// ─── profile ────────────────────────────────────────────────────────────────────────────────────────

export const isObject = (v) => v !== null && typeof v === 'object' && !Array.isArray(v);
export const isCount = (v) => Number.isInteger(v) && v >= 0;

export function exactKeys(value, keys, label, optional = []) {
  if (!isObject(value)) fail('profile-invalid', `${label} must be an object`);
  for (const key of Object.keys(value)) if (!keys.includes(key) && !optional.includes(key)) fail('profile-invalid', `${label} has unknown key '${key}'`);
  for (const key of keys) if (!(key in value)) fail('profile-invalid', `${label} is missing '${key}'`);
}

/** A map from an exact key, or a prefix ending in one '*', to a value. Exact beats prefix; longest prefix wins. */
function keyPatterns(map, label, check) {
  if (!isObject(map)) fail('profile-invalid', `${label} must be an object`);
  const exact = new Map();
  const prefixes = [];
  for (const [key, value] of Object.entries(map)) {
    if (key === '' || key === '*' || key.slice(0, -1).includes('*')) fail('profile-invalid', `${label} key '${key}' is invalid (use an exact key, or a prefix ending in a single '*')`);
    check(key, value);
    if (key.endsWith('*')) prefixes.push([key.slice(0, -1), value]); else exact.set(key, value);
  }
  prefixes.sort((a, b) => b[0].length - a[0].length || codeUnitCompare(a[0], b[0]));
  return (key) => {
    if (exact.has(key)) return exact.get(key);
    for (const [prefix, value] of prefixes) if (key.startsWith(prefix)) return value;
    return undefined;
  };
}
function stringList(value, label) {
  if (!Array.isArray(value) || value.some((item) => typeof item !== 'string' || item === '')) {
    fail('profile-invalid', `${label} must be an array of non-empty strings`);
  }
  return value;
}

const TEMPLATE_PLACEHOLDER = /\{(name|groupName)(?:\|(nfkc-lower))?\}/g;

function compileKeyRule(rule, index) {
  const label = `profile.keyRules[${index}]`;
  exactKeys(rule, ['match', 'key'], label);
  if (typeof rule.match !== 'string' || rule.match === '' || rule.match.length > 512) fail('profile-invalid', `${label}.match must be a regex string of at most 512 characters`);
  if (typeof rule.key !== 'string' || rule.key === '' || rule.key.length > 512) fail('profile-invalid', `${label}.key must be a template string of at most 512 characters`);
  let regex;
  try { regex = new RegExp(rule.match, 'u'); } catch { fail('profile-invalid', `${label}.match is not a valid regular expression`); }
  const groups = new RegExp(`${rule.match}|`, 'u').exec('').length - 1;
  // Validate the template: only $1..$9 captures that exist and the two whitelisted {field|transform} forms.
  const stripped = rule.key.replace(TEMPLATE_PLACEHOLDER, '');
  if (/[{}]/.test(stripped)) fail('profile-invalid', `${label}.key has an unsupported placeholder (only {name}, {groupName}, with optional |nfkc-lower)`);
  for (const capture of stripped.matchAll(/\$(.)/gu)) {
    if (!/^[1-9]$/.test(capture[1]) || Number(capture[1]) > groups) fail('profile-invalid', `${label}.key references capture $${capture[1]} that the match does not define`);
  }
  if (/\$$/.test(stripped)) fail('profile-invalid', `${label}.key ends with a bare '$'`);
  return { regex, template: rule.key };
}

/** Strict validation: unknown or missing keys are rejected. Returns the profile plus its compiled helpers. */
export function validateProfile(profile) {
  exactKeys(profile, [
    'schema', 'id', 'version', 'identity', 'ignoreRowIdPrefixes', 'volatile', 'keyRules', 'kinds',
    'geometry', 'discontinuityThreshold', 'budgets', 'uncheckedStatements',
  ], 'profile', ['tolerances']);
  if (profile.schema !== PROFILE_SCHEMA) fail('profile-invalid', `profile.schema must be '${PROFILE_SCHEMA}'`);
  for (const key of ['id', 'version']) {
    if (typeof profile[key] !== 'string' || profile[key] === '') fail('profile-invalid', `profile.${key} must be a non-empty string`);
  }
  exactKeys(profile.identity, ['sourceKey', 'namespace'], 'profile.identity');
  for (const key of ['sourceKey', 'namespace']) {
    if (typeof profile.identity[key] !== 'string' || profile.identity[key] === '') fail('profile-invalid', `profile.identity.${key} must be a non-empty string`);
  }
  stringList(profile.ignoreRowIdPrefixes, 'profile.ignoreRowIdPrefixes');
  stringList(profile.volatile, 'profile.volatile');
  if (!Array.isArray(profile.keyRules)) fail('profile-invalid', 'profile.keyRules must be an array');
  const keyRules = profile.keyRules.map(compileKeyRule);
  const kindPattern = keyPatterns(profile.kinds, 'profile.kinds', (key, kind) => {
    if (!COMPARABLE_KINDS.includes(kind)) fail('profile-invalid', `profile.kinds['${key}'] must be one of ${COMPARABLE_KINDS.join(', ')}`);
  });
  // Optional: how far a numeric field may differ and still be the same value, in the field's own unit.
  const tolerancePattern = profile.tolerances === undefined ? () => undefined
    : keyPatterns(profile.tolerances, 'profile.tolerances', (key, tolerance) => {
      if (typeof tolerance !== 'number' || !Number.isFinite(tolerance) || tolerance < 0) fail('profile-invalid', `profile.tolerances['${key}'] must be a non-negative number`);
    });
  const placementFields = Object.values(profile.kinds).includes('placement');
  exactKeys(profile.geometry, ['field', 'toleranceMm'], 'profile.geometry');
  if (profile.geometry.field !== 'bounds') fail('profile-invalid', "profile.geometry.field must be 'bounds'");
  if (typeof profile.geometry.toleranceMm !== 'number' || !Number.isFinite(profile.geometry.toleranceMm) || profile.geometry.toleranceMm < 0) {
    fail('profile-invalid', 'profile.geometry.toleranceMm must be a non-negative number');
  }
  const threshold = profile.discontinuityThreshold;
  if (typeof threshold !== 'number' || !(threshold > 0 && threshold <= 1)) fail('profile-invalid', 'profile.discontinuityThreshold must be a number in (0, 1]');
  exactKeys(profile.budgets, ['changes', 'groupIds', 'receiptsPerKind', 'outputBytes'], 'profile.budgets');
  for (const key of ['changes', 'groupIds', 'receiptsPerKind']) {
    if (!isCount(profile.budgets[key])) fail('profile-invalid', `profile.budgets.${key} must be a non-negative integer`);
  }
  if (!Number.isInteger(profile.budgets.outputBytes) || profile.budgets.outputBytes < 1024) fail('profile-invalid', 'profile.budgets.outputBytes must be an integer of at least 1024');
  stringList(profile.uncheckedStatements, 'profile.uncheckedStatements');
  return {
    profile,
    keyRules,
    volatile: new Set(profile.volatile),
    ignorePrefixes: profile.ignoreRowIdPrefixes,
    placementFields,
    kindOf(key) {
      return kindPattern(key) ?? 'properties';
    },
    toleranceOf(key) {
      return tolerancePattern(key);
    },
  };
}

/** The comparison key of a property row: the first key rule that matches its sourceKey, else the sourceKey. */
function comparisonKey(row, compiled) {
  const sourceKey = row.sourceKey;
  for (const rule of compiled.keyRules) {
    const match = rule.regex.exec(sourceKey);
    if (!match || match[0] !== sourceKey) continue;   // a rule must match the WHOLE key, not a prefix
    return rule.template.replace(/\{(name|groupName)(?:\|(nfkc-lower))?\}|\$([1-9])/g, (_, field, transform, capture) => {
      if (capture !== undefined) return match[Number(capture)] ?? '';
      const text = row[field];
      if (typeof text !== 'string') return '';
      return transform === 'nfkc-lower' ? text.normalize('NFKC').toLowerCase() : text;
    });
  }
  return sourceKey;
}

// ─── document validation + indexing ─────────────────────────────────────────────────────────────────

function documentRows(doc, name, label) {
  if (!isObject(doc) || Object.keys(doc).length !== 1 || !Array.isArray(doc[name])) {
    fail('document-invalid', `${label} must be exactly { ${name}: [...] }`);
  }
  return doc[name];
}

function unionBounds(entity, label) {
  if (entity.geometry === undefined) return null;
  if (!Array.isArray(entity.geometry)) fail('document-invalid', `${label} geometry must be an array`);
  let union = null;
  for (const item of entity.geometry) {
    const bounds = item?.bounds;
    if (!Array.isArray(bounds) || bounds.length !== 6 || bounds.some((n) => typeof n !== 'number' || !Number.isFinite(n))) {
      fail('document-invalid', `${label} has geometry bounds that are not six finite numbers`);
    }
    if (!union) union = bounds.slice();
    else for (let i = 0; i < 6; i += 1) union[i] = i < 3 ? Math.min(union[i], bounds[i]) : Math.max(union[i], bounds[i]);
  }
  return union;
}

/**
 * Reduce one side to what pairing needs. Identity rows are read in the same pass that groups the other
 * rows by entity, so each document is walked once and the parsed arrays can be dropped afterwards.
 */
function indexSide(side, docs, compiled) {
  const label = `${side}`;
  const entityRows = documentRows(docs.entities, 'entities', `${label} entities.json`);
  const propertyRows = documentRows(docs.properties, 'properties', `${label} properties.json`);
  const relationshipRows = documentRows(docs.relationships, 'relationships', `${label} relationships.json`);
  const identitySourceKey = compiled.profile.identity.sourceKey;
  const counts = { entities: entityRows.length, properties: propertyRows.length, relationships: relationshipRows.length, ignoredRows: 0, orphanProperties: 0, unkeyedProperties: 0 };

  const entities = new Map();
  for (const entity of entityRows) {
    if (!isObject(entity) || typeof entity.id !== 'string' || entity.id === '') fail('document-invalid', `${label} has an entity without a string id`);
    if (entities.has(entity.id)) fail('document-invalid', `${label} has two entities with id '${entity.id}'`);
    entities.set(entity.id, entity);
  }

  const rowsByEntity = new Map();
  const identityRows = new Map();
  for (const row of propertyRows) {
    if (!isObject(row)) fail('document-invalid', `${label} has a property row that is not an object`);
    if (typeof row.id === 'string' && compiled.ignorePrefixes.some((prefix) => row.id.startsWith(prefix))) { counts.ignoredRows += 1; continue; }
    if (typeof row.entityId !== 'string' || !entities.has(row.entityId)) { counts.orphanProperties += 1; continue; }
    if (typeof row.sourceKey !== 'string' || row.sourceKey === '') { counts.unkeyedProperties += 1; continue; }
    if (row.sourceKey === identitySourceKey) {
      const list = identityRows.get(row.entityId);
      if (list) list.push(row); else identityRows.set(row.entityId, [row]);
      continue;
    }
    const list = rowsByEntity.get(row.entityId);
    if (list) list.push(row); else rowsByEntity.set(row.entityId, [row]);
  }

  const blank = [];
  const duplicate = new Set();
  const byIdentity = new Map();   // identity -> entityId[]
  for (const [entityId] of entities) {
    const rows = identityRows.get(entityId);
    if (!rows) { blank.push(entityId); continue; }
    if (rows.length > 1) { duplicate.add(entityId); continue; }
    const value = rows[0].value;
    if (typeof value !== 'string' || value.trim() === '') { blank.push(entityId); continue; }
    const identity = `${compiled.profile.identity.namespace}:${value}`;
    const owners = byIdentity.get(identity);
    if (owners) owners.push(entityId); else byIdentity.set(identity, [entityId]);
  }
  const pairable = new Map();     // identity -> entityId
  const identityOfEntity = new Map();
  // Every identity VALUE that this side cannot vouch for (shared by two entities, or carried twice by one).
  // The other side must not report such an identity as added or removed: that would claim an absence.
  const unusableIdentities = new Set();
  for (const [entityId, rows] of identityRows) {
    if (rows.length > 1) for (const row of rows) if (typeof row.value === 'string' && row.value.trim() !== '') unusableIdentities.add(`${compiled.profile.identity.namespace}:${row.value}`);
  }
  for (const [identity, owners] of byIdentity) {
    if (owners.length > 1) { unusableIdentities.add(identity); for (const entityId of owners) duplicate.add(entityId); continue; }
    pairable.set(identity, owners[0]);
    identityOfEntity.set(owners[0], identity);
  }

  const relationships = new Set();
  let unresolved = 0;
  for (const row of relationshipRows) {
    if (!isObject(row) || typeof row.kind !== 'string' || typeof row.from !== 'string' || typeof row.to !== 'string') {
      fail('document-invalid', `${label} has a relationship without string kind/from/to`);
    }
    const from = identityOfEntity.get(row.from);
    const to = identityOfEntity.get(row.to);
    if (from === undefined || to === undefined) { unresolved += 1; continue; }
    relationships.add(`${row.kind}\u0000${from}\u0000${to}`);
  }

  return {
    side, counts, entities, rowsByEntity, pairable, identityOfEntity, unusableIdentities,
    blank, duplicate, relationships, unresolvedRelationships: unresolved,
    droppedIdentities: new Set(),   // pairable on this side, but the other side holds the identity twice
  };
}

// ─── entity comparison ──────────────────────────────────────────────────────────────────────────────

const equalValue = (a, b) => canonicalJson(a) === canonicalJson(b);

/** Equal, or two finite numbers no further apart than the key's tolerance (a difference equal to it is equal). */
function sameValue(key, before, after, compiled) {
  if (equalValue(before, after)) return true;
  const tolerance = compiled.toleranceOf(key);
  return tolerance !== undefined && typeof before === 'number' && typeof after === 'number'
    && Number.isFinite(before) && Number.isFinite(after) && Math.abs(before - after) <= tolerance;
}

function keyedRows(rows, compiled) {
  const keyed = new Map();
  for (const row of rows ?? []) {
    if (compiled.volatile.has(row.sourceKey)) continue;
    const key = comparisonKey(row, compiled);
    if (compiled.volatile.has(key)) continue;
    const list = keyed.get(key);
    if (list) list.push(row); else keyed.set(key, [row]);
  }
  return keyed;
}

function field(key, kind, op, before, after, extra = {}) {
  return { key, kind, op, before, after, ...extra };
}

/** Fields that differ between the two states of one paired object, plus the keys it refused to compare. */
function diffEntity(baseEntity, baseRows, revisedEntity, revisedRows, compiled, tolerance) {
  const fields = [];
  const ambiguous = [];
  const classify = (key) => compiled.kindOf(key);

  for (const key of ['type', 'name']) {
    const before = baseEntity[key] ?? null;
    const after = revisedEntity[key] ?? null;
    if (!equalValue(before, after)) fields.push(field(`entity.${key}`, classify(`entity.${key}`), 'modified', before, after));
  }

  const baseBounds = unionBounds(baseEntity, `entity ${baseEntity.id}`);
  const revisedBounds = unionBounds(revisedEntity, `entity ${revisedEntity.id}`);
  if (baseBounds || revisedBounds) {
    const moved = !baseBounds || !revisedBounds || baseBounds.some((n, i) => Math.abs(n - revisedBounds[i]) > tolerance);
    if (moved) {
      fields.push(field(BOUNDS_KEY, 'placement', !baseBounds ? 'added' : !revisedBounds ? 'removed' : 'modified', baseBounds, revisedBounds));
    }
  }

  const baseKeyed = keyedRows(baseRows, compiled);
  const revisedKeyed = keyedRows(revisedRows, compiled);
  const keys = new Set([...baseKeyed.keys(), ...revisedKeyed.keys()]);
  // Placement is compared when either state has a bounding box OR a field the profile marks placement.
  let placementCompared = !!(baseBounds || revisedBounds);
  for (const key of keys) {
    const before = baseKeyed.get(key);
    const after = revisedKeyed.get(key);
    if ((before && before.length > 1) || (after && after.length > 1)) {
      ambiguous.push({ key, base: before?.length ?? 0, revised: after?.length ?? 0 });
      continue;   // two properties normalise to one key: comparing either would be a guess
    }
    const b = before?.[0];
    const a = after?.[0];
    const kind = classify(key);
    if (kind === 'placement') placementCompared = true;   // an ambiguous key above was not compared
    if (!b) fields.push(field(key, kind, 'added', null, a.value ?? null, { unitAfter: a.unit ?? null }));
    else if (!a) fields.push(field(key, kind, 'removed', b.value ?? null, null, { unitBefore: b.unit ?? null }));
    else if (!sameValue(key, b.value ?? null, a.value ?? null, compiled) || !equalValue(b.unit ?? null, a.unit ?? null) || !equalValue(b.valueType ?? null, a.valueType ?? null)) {
      const extra = {};
      if (!equalValue(b.unit ?? null, a.unit ?? null)) { extra.unitBefore = b.unit ?? null; extra.unitAfter = a.unit ?? null; }
      if (!equalValue(b.valueType ?? null, a.valueType ?? null)) { extra.typeBefore = b.valueType ?? null; extra.typeAfter = a.valueType ?? null; }
      fields.push(field(key, kind, 'modified', b.value ?? null, a.value ?? null, extra));
    }
  }
  return { fields, ambiguous, placementCompared };
}

// ─── the comparison ─────────────────────────────────────────────────────────────────────────────────

function capped(list, cap) {
  return list.length > cap ? { items: list.slice(0, cap), omitted: list.length - cap } : { items: list, omitted: 0 };
}

function parseWitness(witness) {
  if (!isObject(witness) || Object.keys(witness).length !== 1 || !Array.isArray(witness.entries)) fail('witness-invalid', 'witness must be exactly { entries: [...] }');
  const actions = new Map();
  for (const [index, entry] of witness.entries.entries()) {
    if (!isObject(entry) || Object.keys(entry).length !== 2 || typeof entry.identity !== 'string' || entry.identity === '' || !WITNESS_ACTIONS.includes(entry.action)) {
      fail('witness-invalid', `witness entry ${index} must be { identity: string, action: ${WITNESS_ACTIONS.join('|')} }`);
    }
    const set = actions.get(entry.identity);
    if (set) set.add(entry.action); else actions.set(entry.identity, new Set([entry.action]));
  }
  return actions;
}

const STATUS_FOR_ACTION = { created: 'added', modified: 'changed', deleted: 'removed' };

/**
 * The pairing every consumer shares (compare-snapshots and check-snapshots): index both sides, drop the
 * identities a side cannot vouch for, split identities into removed / added / paired, and attribute
 * relationship differences to paired endpoints. Exported so a second command pairs EXACTLY the same way.
 */
export function pairSnapshots({ base, revised, profile: rawProfile }) {
  const compiled = validateProfile(rawProfile);
  const baseSide = indexSide('base', base, compiled);
  const revisedSide = indexSide('revised', revised, compiled);
  return { compiled, ...pairIndexedSides(compiled, baseSide, revisedSide) };
}

function pairIndexedSides(compiled, baseSide, revisedSide) {
  // An identity the OTHER side holds twice cannot be paired, and must not be called added or removed either.
  for (const [mine, theirs] of [[baseSide, revisedSide], [revisedSide, baseSide]]) {
    for (const [identity, entityId] of [...mine.pairable]) {
      if (!theirs.unusableIdentities.has(identity)) continue;
      mine.pairable.delete(identity);
      mine.identityOfEntity.delete(entityId);
      mine.duplicate.add(entityId);
      mine.droppedIdentities.add(identity);
    }
  }
  for (const side of [baseSide, revisedSide]) side.duplicate = [...side.duplicate].sort(codeUnitCompare);

  const removedIdentities = [];
  const addedIdentities = [];
  const pairedIdentities = [];
  for (const identity of baseSide.pairable.keys()) (revisedSide.pairable.has(identity) ? pairedIdentities : removedIdentities).push(identity);
  for (const identity of revisedSide.pairable.keys()) if (!baseSide.pairable.has(identity)) addedIdentities.push(identity);
  removedIdentities.sort(codeUnitCompare);
  addedIdentities.sort(codeUnitCompare);
  pairedIdentities.sort(codeUnitCompare);
  const pairedSet = new Set(pairedIdentities);

  // Relationship differences, attributed to a paired endpoint only when BOTH endpoints are paired: a link
  // to an added or removed object follows from that add/remove and is not a second change.
  const relationshipFields = new Map();   // identity -> field[]
  const pushRelationship = (identity, entry) => {
    const list = relationshipFields.get(identity);
    if (list) list.push(entry); else relationshipFields.set(identity, [entry]);
  };
  const relationshipSummary = { base: baseSide.relationships.size, revised: revisedSide.relationships.size, added: 0, removed: 0, onUnpaired: 0 };
  const attribute = (triple, op) => {
    const [kind, from, to] = triple.split('\u0000');
    if (!pairedSet.has(from) || !pairedSet.has(to)) { relationshipSummary.onUnpaired += 1; return; }
    relationshipSummary[op] += 1;
    const before = op === 'removed';
    pushRelationship(from, field(`relationship:${kind}:out:${to}`, 'relationships', op, before ? to : null, before ? null : to));
    pushRelationship(to, field(`relationship:${kind}:in:${from}`, 'relationships', op, before ? from : null, before ? null : from));
  };
  for (const triple of baseSide.relationships) if (!revisedSide.relationships.has(triple)) attribute(triple, 'removed');
  for (const triple of revisedSide.relationships) if (!baseSide.relationships.has(triple)) attribute(triple, 'added');
  return { baseSide, revisedSide, removedIdentities, addedIdentities, pairedIdentities, pairedSet, relationshipFields, relationshipSummary };
}

/** Every paired object's field differences (relationships included), unsorted and uncapped. Unchanged pairs have no fields. */
export function diffPairs(pairing) {
  const { compiled, baseSide, revisedSide, pairedIdentities, relationshipFields } = pairing;
  const tolerance = compiled.profile.geometry.toleranceMm;
  const out = [];
  for (const identity of pairedIdentities) {
    const baseId = baseSide.pairable.get(identity);
    const revisedId = revisedSide.pairable.get(identity);
    const baseEntity = baseSide.entities.get(baseId);
    const revisedEntity = revisedSide.entities.get(revisedId);
    const { fields, ambiguous, placementCompared } = diffEntity(baseEntity, baseSide.rowsByEntity.get(baseId), revisedEntity, revisedSide.rowsByEntity.get(revisedId), compiled, tolerance);
    const extra = relationshipFields.get(identity);
    if (extra) fields.push(...extra);
    out.push({ identity, baseId, revisedId, baseEntity, revisedEntity, fields, ambiguous, placementCompared });
  }
  return out;
}

/**
 * @param {{ base: {entities, properties, relationships}, revised: {...}, profile: object, witness?: object|null,
 *           digests?: object }} input
 */
export function compareSnapshots({ base, revised, profile: rawProfile, witness = null, digests = {} }) {
  const compiled = validateProfile(rawProfile);
  const { profile } = compiled;
  const tolerance = profile.geometry.toleranceMm;
  const budgets = profile.budgets;
  const baseSide = indexSide('base', base, compiled);
  const revisedSide = indexSide('revised', revised, compiled);
  const witnessActions = witness ? parseWitness(witness) : null;
  const pairing = { compiled, ...pairIndexedSides(compiled, baseSide, revisedSide) };
  const { removedIdentities, addedIdentities, pairedIdentities, pairedSet, relationshipSummary } = pairing;

  const rowOf = (side, identity, status) => {
    const entityId = side.pairable.get(identity);
    const entity = side.entities.get(entityId);
    return {
      status, identity,
      entityIdBase: side === baseSide ? entityId : null,
      entityIdRevised: side === revisedSide ? entityId : null,
      type: entity.type ?? null, name: entity.name ?? null,
    };
  };
  const removedRows = removedIdentities.map((identity) => rowOf(baseSide, identity, 'removed'));
  const addedRows = addedIdentities.map((identity) => rowOf(revisedSide, identity, 'added'));

  const changedRows = [];
  const ambiguousReceipts = [];
  const byKind = Object.fromEntries(KIND_ORDER.map((kind) => [kind, 0]));
  let fieldsOmitted = 0;
  let placementNotCompared = 0;   // paired objects with no bounding box and no placement field on either side
  for (const { identity, baseId, revisedId, revisedEntity, fields, ambiguous, placementCompared } of diffPairs(pairing)) {
    if (!placementCompared) placementNotCompared += 1;
    for (const item of ambiguous) ambiguousReceipts.push({ identity, ...item });
    if (fields.length === 0) continue;
    fields.sort((a, b) => codeUnitCompare(a.key, b.key));
    const kinds = KIND_ORDER.filter((kind) => fields.some((f) => f.kind === kind));
    for (const kind of kinds) byKind[kind] += 1;
    const kept = fields.slice(0, FIELD_CAP);
    fieldsOmitted += fields.length - kept.length;
    changedRows.push({
      status: 'changed', identity, entityIdBase: baseId, entityIdRevised: revisedId,
      type: revisedEntity.type ?? null, name: revisedEntity.name ?? null,
      kinds, fields: kept, truncatedFields: fields.length - kept.length,
    });
  }
  const allRows = [...removedRows, ...addedRows, ...changedRows];   // pinned order; each already by identity

  // Unpaired objects, grouped by type + name. A presentation of unpaired objects, never a pairing.
  const groupMap = new Map();
  const groupOf = (row, bucket) => {
    const key = canonicalJson([row.type, row.name]);
    let group = groupMap.get(key);
    if (!group) { group = { type: row.type, name: row.name, removed: { count: 0, ids: [] }, added: { count: 0, ids: [] } }; groupMap.set(key, group); }
    group[bucket].count += 1;
    group[bucket].ids.push(bucket === 'removed' ? row.entityIdBase : row.entityIdRevised);
  };
  for (const row of removedRows) groupOf(row, 'removed');
  for (const row of addedRows) groupOf(row, 'added');
  let groupIdsOmitted = 0;
  const groups = [...groupMap.values()]
    .sort((a, b) => codeUnitCompare(String(a.type), String(b.type)) || codeUnitCompare(String(a.name), String(b.name)))
    .map((group) => {
      const out = { type: group.type, name: group.name };
      for (const bucket of ['removed', 'added']) {
        const { items, omitted } = capped(group[bucket].ids, budgets.groupIds);
        groupIdsOmitted += omitted;
        out[bucket] = { count: group[bucket].count, entityIds: items, truncated: omitted };
      }
      return out;
    });

  // Excluded receipts.
  let receiptsOmitted = 0;
  const receipt = (ids) => {
    const { items, omitted } = capped(ids, budgets.receiptsPerKind);
    receiptsOmitted += omitted;
    return { count: ids.length, entityIds: items, truncated: omitted };
  };
  const excluded = {
    blank: { base: receipt(baseSide.blank), revised: receipt(revisedSide.blank) },
    duplicate: { base: receipt(baseSide.duplicate), revised: receipt(revisedSide.duplicate) },
  };

  ambiguousReceipts.sort((a, b) => codeUnitCompare(a.identity, b.identity) || codeUnitCompare(a.key, b.key));
  const ambiguousCapped = capped(ambiguousReceipts, budgets.receiptsPerKind);
  receiptsOmitted += ambiguousCapped.omitted;

  // Witness agreement.
  let witnessSection = null;
  if (witnessActions) {
    const statusOf = new Map();
    for (const row of allRows) statusOf.set(row.identity, row.status);
    const agrees = [];
    const witnessOnly = [];
    const diffOnly = [];
    const agreed = new Set();
    for (const identity of [...witnessActions.keys()].sort(codeUnitCompare)) {
      // Accept a bare value as well as the namespaced identity; the namespace prefix is unambiguous.
      const resolved = statusOf.has(identity) || pairedSet.has(identity) || baseSide.pairable.has(identity) || revisedSide.pairable.has(identity)
        ? identity : `${profile.identity.namespace}:${identity}`;
      const status = statusOf.get(resolved);
      for (const action of [...witnessActions.get(identity)].sort()) {
        if (status !== undefined && STATUS_FOR_ACTION[action] === status) { agrees.push({ identity: resolved, action }); agreed.add(resolved); }
        else witnessOnly.push({ identity: resolved, action, diff: status ?? (pairedSet.has(resolved) ? 'unchanged' : 'absent') });
      }
    }
    for (const row of allRows) if (!agreed.has(row.identity)) diffOnly.push({ identity: row.identity, status: row.status });
    for (const list of [agrees, witnessOnly, diffOnly]) list.sort((a, b) => codeUnitCompare(a.identity, b.identity) || codeUnitCompare(a.action ?? a.status, b.action ?? b.status));
    const section = { digest: digests.witness ?? null, entries: [...witnessActions.values()].reduce((n, set) => n + set.size, 0) };
    for (const [name, list] of [['agrees', agrees], ['witnessOnly', witnessOnly], ['diffOnly', diffOnly]]) {
      const { items, omitted } = capped(list, budgets.receiptsPerKind);
      receiptsOmitted += omitted;
      section[name] = { count: list.length, items, truncated: omitted };
    }
    witnessSection = section;
  }

  // Summary: counts are complete because they come from the full lists, before any cap.
  const baseTotal = baseSide.counts.entities;
  // The threshold is compared against the exact ratio; only the reported share is rounded for display.
  const pairedRatio = baseTotal === 0 ? null : pairedIdentities.length / baseTotal;
  const pairedShare = pairedRatio === null ? null : Math.round(pairedRatio * 1e6) / 1e6;
  const identitySection = {
    base: baseTotal, revised: revisedSide.counts.entities,
    paired: pairedIdentities.length, added: addedIdentities.length, removed: removedIdentities.length,
    pairedShareOfBase: pairedShare,
    excluded,
    discontinuity: pairedRatio !== null && pairedRatio < profile.discontinuityThreshold,
    discontinuityThreshold: profile.discontinuityThreshold,
  };
  const summary = {
    added: addedRows.length, removed: removedRows.length, changed: changedRows.length,
    unchanged: pairedIdentities.length - changedRows.length,
    byKind, relationships: relationshipSummary,
  };

  // AWARE knows nothing of what a profile's placement fields mean, so when there are any the profile's own
  // statements say what they cover; AWARE states only what it did.
  const statements = compiled.placementFields ? [
    `Placement was compared on each object's position fields and, where it has one, its bounding box within ${tolerance} mm. A difference smaller than a field's tolerance is not reported.`,
    'Solid geometry was not compared; only the listed fields and bounding boxes were.',
    ...profile.uncheckedStatements,
  ] : [
    `Placement was compared on each object's bounding box only, within ${tolerance} mm. A rotation or a move that keeps the bounding box within that tolerance is not detected.`,
    'Holes, cuts, fittings and solid geometry were not compared; only the listed shape fields and bounding boxes were.',
    ...profile.uncheckedStatements,
  ];
  if (placementNotCompared > 0) {
    statements.push(compiled.placementFields
      ? `${placementNotCompared} paired object(s) have no bounding box and no placement field in either saved state, so whether they moved was not checked.`
      : `${placementNotCompared} paired object(s) have no bounding box in either saved state, so whether they moved was not checked.`);
  }
  const excludedTotal = baseSide.blank.length + revisedSide.blank.length + baseSide.duplicate.length + revisedSide.duplicate.length;
  if (excludedTotal > 0) {
    statements.push(`${excludedTotal} object(s) have no usable identity (blank or duplicated) and were never paired: base ${baseSide.blank.length} blank / ${baseSide.duplicate.length} duplicate, revised ${revisedSide.blank.length} blank / ${revisedSide.duplicate.length} duplicate.`);
  }
  if (ambiguousReceipts.length > 0) {
    statements.push(`${ambiguousReceipts.length} field(s) were not compared because two or more properties of one object share a comparison key (ambiguous-field).`);
  }
  const unresolved = baseSide.unresolvedRelationships + revisedSide.unresolvedRelationships;
  if (unresolved > 0) statements.push(`${unresolved} relationship(s) were not compared because an end has no usable identity.`);
  if (identitySection.discontinuity) {
    statements.push(`Only ${pairedIdentities.length} of ${baseTotal} base objects paired by identity (below ${profile.discontinuityThreshold}); the two states may not share one lineage, so added and removed counts are not a reliable change list.`);
  }
  if (fieldsOmitted > 0) statements.push(`${fieldsOmitted} changed field(s) are not listed (each object lists at most ${FIELD_CAP}).`);

  const buildOutput = (rowCount, groupCount) => {
    const rows = allRows.slice(0, rowCount);
    const emitted = { added: 0, removed: 0, changed: 0 };
    for (const row of rows) emitted[row.status] += 1;
    const omitted = { added: summary.added - emitted.added, removed: summary.removed - emitted.removed, changed: summary.changed - emitted.changed };
    let groupsOmitted = groupIdsOmitted;
    for (const group of groups.slice(groupCount)) groupsOmitted += group.removed.entityIds.length + group.added.entityIds.length;
    const tail = [];
    const rowsOmitted = omitted.added + omitted.removed + omitted.changed;
    if (rowsOmitted > 0) tail.push(`Partial list: ${rowsOmitted} change(s) are not listed (the list is limited to ${budgets.changes} and ${budgets.outputBytes} bytes). Counts are complete.`);
    if (groupsOmitted > 0) tail.push(`${groupsOmitted} unpaired object id(s) are not listed in groups. Group counts are complete.`);
    if (receiptsOmitted > 0) tail.push(`${receiptsOmitted} receipt id(s) are not listed (the list is limited to ${budgets.receiptsPerKind} per kind). Counts are complete.`);
    return {
      schema: CHANGE_SET_SCHEMA,
      inputs: {
        base: digests.base ?? null, revised: digests.revised ?? null,
        profileDigest: sha256(canonicalJson(profile)), witnessDigest: digests.witness ?? null,
        rows: { base: baseSide.counts, revised: revisedSide.counts },
      },
      profile: { id: profile.id, version: profile.version },
      identity: identitySection,
      changes: rows,
      unpaired: { groupCount: groups.length, groups: groups.slice(0, groupCount) },
      ambiguousFields: { count: ambiguousReceipts.length, items: ambiguousCapped.items, truncated: ambiguousCapped.omitted },
      summary,
      truncated: { omitted, groups: groupsOmitted, receipts: receiptsOmitted },
      unchecked: [...statements, ...tail],
      witness: witnessSection,
    };
  };

  // Fit the output byte budget: shed change rows first (from the end of the pinned order), then groups.
  const size = (output) => Buffer.byteLength(canonicalJson(output));
  let rowCount = Math.min(budgets.changes, allRows.length);
  let groupCount = groups.length;
  let output = buildOutput(rowCount, groupCount);
  if (size(output) > budgets.outputBytes) {
    const largest = (limit, fits) => {
      let lo = 0, hi = limit;
      while (lo < hi) { const mid = Math.ceil((lo + hi) / 2); if (fits(mid)) lo = mid; else hi = mid - 1; }
      return lo;
    };
    rowCount = largest(rowCount, (n) => size(buildOutput(n, groupCount)) <= budgets.outputBytes);
    if (size(buildOutput(rowCount, groupCount)) > budgets.outputBytes) {
      groupCount = largest(groupCount, (n) => size(buildOutput(rowCount, n)) <= budgets.outputBytes);
    }
    output = buildOutput(rowCount, groupCount);
    if (size(output) > budgets.outputBytes) fail('budget-too-small', `profile.budgets.outputBytes (${budgets.outputBytes}) cannot hold even the counts of this comparison`);
  }
  return output;
}

// ─── file I/O wrapper (the CLI path) ────────────────────────────────────────────────────────────────

export const SIDE_FILES = [['entities', 'entities.json'], ['properties', 'properties.json'], ['relationships', 'relationships.json']];

export function readJsonFile(path, label) {
  if (!existsSync(path) || !statSync(path).isFile()) fail('input-missing', `${label} not found: ${path}`);
  const bytes = readFileSync(path);
  let value;
  try { value = JSON.parse(bytes.toString('utf8')); } catch { fail('document-invalid', `${label} is not valid JSON`); }
  return { value, digest: sha256(bytes) };
}

/** Read both saved states' three documents from `base-dir` and `revised-dir`, hashing each file's exact bytes. */
export function readSnapshotSides(args) {
  const sides = {};
  const digests = {};
  for (const [name, dirKey] of [['base', 'base-dir'], ['revised', 'revised-dir']]) {
    const dir = resolve(args[dirKey]);
    if (!existsSync(dir) || !statSync(dir).isDirectory()) fail('input-missing', `\`${dirKey}\` is not a directory: ${dir}`);
    sides[name] = {};
    digests[name] = {};
    for (const [key, file] of SIDE_FILES) {
      const { value, digest } = readJsonFile(join(dir, file), `${name} ${file}`);
      sides[name][key] = value;
      digests[name][key] = digest;
    }
  }
  return { sides, digests };
}

/** Read both sides, the profile and the optional witness from disk, hashing each file's exact bytes. */
export function compareSnapshotsFromFiles(args) {
  for (const key of ['base-dir', 'revised-dir', 'profile-path']) {
    if (typeof args[key] !== 'string' || args[key] === '') fail('input-invalid', `\`${key}\` is required, as a string`);
  }
  if (args['witness-path'] !== undefined && (typeof args['witness-path'] !== 'string' || args['witness-path'] === '')) fail('input-invalid', '`witness-path` must be a non-empty string when given');
  const { sides, digests } = readSnapshotSides(args);
  const profile = readJsonFile(resolve(args['profile-path']), 'profile').value;
  let witness = null;
  if (args['witness-path'] !== undefined) {
    const read = readJsonFile(resolve(args['witness-path']), 'witness');
    witness = read.value;
    digests.witness = read.digest;
  }
  return compareSnapshots({ base: sides.base, revised: sides.revised, profile, witness, digests });
}

/** The bridge entry point: returns the canonical JSON text to write on stdout. */
export function runSnapshotCompareCommand(args) {
  return canonicalJson(compareSnapshotsFromFiles(args));
}
