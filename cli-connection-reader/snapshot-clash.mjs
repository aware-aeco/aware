/**
 * Clash for `check-snapshots` (model-change-set 0.3.0): three data-defined pair rules over the solids of the parts a
 * change touched and their dependency closure.
 *
 *   clash-overlap       two solids share a region thicker than `contactTolerance`        (clash.interpenetration)
 *   clash-contact       two solids touch and no bolt or weld joins them                   (clash.unjoined-contact)
 *   clash-bolted-plies  a ply a bolt group names touches no ply of the rest of its stack  (clash.bolted-plies-apart)
 *
 * FORMAT-BLIND, like the rest of check-snapshots: which node holds an entity's mesh, which coverage property says
 * whether a solid is trustworthy, and which properties name the parts a bolt or weld joins are all the check
 * profile's `geometry` block (plan 2026-11-12-change-review-s3-clash-PLAN.md §3). This file knows no host.
 *
 * Narrow phase: manifold-3d (Apache-2.0, WASM, single-threaded) for A ∩ B, split into regions and measured region by
 * region. Contact and gap use AWARE's own exact triangle-to-triangle distance (`snapshot-clash-geometry.mjs`).
 * Nothing here is random, parallel or clock-dependent: the same inputs give the same bytes.
 */
import { createHash } from 'node:crypto';
import { existsSync, readFileSync, statSync } from 'node:fs';
import { join, resolve } from 'node:path';
import { codeUnitCompare, exactKeys, fail, isCount, isObject, providerRelationKindsOf, readJsonFile } from './snapshot-compare.mjs';
import { manifoldWasmBytes } from './manifold-wasm.mjs';
import {
  boxOf, checkMesh, faceDirections, meshDistance, mm, mm3, point, readTileMeshes, triangleBoxes, weld,
} from './snapshot-clash-geometry.mjs';

export const CLASH_PREDICATES = ['clash-overlap', 'clash-contact', 'clash-bolted-plies'];
export const CLASH_TARGET = 'solid';
export const GEOMETRY_KEYS = ['node', 'units', 'contactTolerance', 'weldTolerance', 'qualification', 'joins', 'groups', 'budgets'];
const GROUP_KINDS = ['hosts', 'contains'];
const JOIN_FORMATS = ['identity-key-list'];
const PROPERTY_FIELDS = ['id', 'sourceKey'];   // which property-row field the coverage pattern is matched against
const WORSENING_PATHS = { 'clash-overlap': ['overlapThicknessMm', 'overlapVolumeMm3'], 'clash-contact': [], 'clash-bolted-plies': ['gapMm'] };

const nonEmpty = (v) => typeof v === 'string' && v !== '';
const positive = (v) => typeof v === 'number' && Number.isFinite(v) && v > 0;
const stringList = (value, label) => {
  if (!Array.isArray(value) || value.length === 0 || value.some((item) => !nonEmpty(item))) fail('profile-invalid', `${label} must be a non-empty array of non-empty strings`);
  return value;
};
const regexOf = (source, label) => {
  if (!nonEmpty(source) || source.length > 512) fail('profile-invalid', `${label} must be a regular expression string of at most 512 characters`);
  try { return new RegExp(source, 'u'); } catch { return fail('profile-invalid', `${label} is not a valid regular expression`); }
};
export const camelCase = (text) => text.split(/[^A-Za-z0-9]+/).filter(Boolean).map((w, i) => (i === 0 ? w.toLowerCase() : w[0].toUpperCase() + w.slice(1).toLowerCase())).join('');

/** Strict validation of the profile's `geometry` block (additive to `aware.model-check-profile/v1`). */
export function validateGeometryBlock(geometry) {
  // `minOverlap` (0.6.0) is optional: absent, an overlap is reported as soon as it is thicker than `contactTolerance`.
  exactKeys(geometry, GEOMETRY_KEYS, 'check profile.geometry', ['minOverlap']);
  exactKeys(geometry.node, ['entityIdPattern', 'nodeName'], 'check profile.geometry.node');
  const nodeRegex = regexOf(geometry.node.entityIdPattern, 'check profile.geometry.node.entityIdPattern');
  if (!nonEmpty(geometry.node.nodeName) || geometry.node.nodeName.length > 512) fail('profile-invalid', 'check profile.geometry.node.nodeName must be a template string of at most 512 characters');
  const groups = new RegExp(`${geometry.node.entityIdPattern}|`, 'u').exec('').length - 1;
  for (const capture of geometry.node.nodeName.matchAll(/\$(.?)/gu)) {
    if (!/^[1-9]$/.test(capture[1]) || Number(capture[1]) > groups) fail('profile-invalid', `check profile.geometry.node.nodeName references capture $${capture[1]} that the pattern does not define`);
  }
  if (geometry.units !== 'mm') fail('profile-invalid', "check profile.geometry.units must be 'mm'");
  if (!positive(geometry.contactTolerance)) fail('profile-invalid', 'check profile.geometry.contactTolerance must be a positive number');
  if (!positive(geometry.weldTolerance) || geometry.weldTolerance > geometry.contactTolerance) fail('profile-invalid', 'check profile.geometry.weldTolerance must be a positive number no larger than contactTolerance');
  exactKeys(geometry.qualification, ['propertyKeyPattern', 'propertyField', 'json', 'checks', 'whenPresent', 'passStates'], 'check profile.geometry.qualification');
  if (!PROPERTY_FIELDS.includes(geometry.qualification.propertyField)) fail('profile-invalid', `check profile.geometry.qualification.propertyField must be one of ${PROPERTY_FIELDS.join(', ')}`);
  const coverageRegex = regexOf(geometry.qualification.propertyKeyPattern, 'check profile.geometry.qualification.propertyKeyPattern');
  if (!nonEmpty(geometry.qualification.json)) fail('profile-invalid', 'check profile.geometry.qualification.json must name the checks field of the coverage JSON');
  stringList(geometry.qualification.checks, 'check profile.geometry.qualification.checks');
  const whenPresent = geometry.qualification.whenPresent;
  if (!Array.isArray(whenPresent) || whenPresent.some((item) => !nonEmpty(item))) fail('profile-invalid', 'check profile.geometry.qualification.whenPresent must be an array (possibly empty) of non-empty strings');
  if (new Set(whenPresent).size !== whenPresent.length) fail('profile-invalid', 'check profile.geometry.qualification.whenPresent must not name a check twice');
  const overlap = whenPresent.find((name) => geometry.qualification.checks.includes(name));
  if (overlap !== undefined) fail('profile-invalid', `check profile.geometry.qualification.whenPresent names '${overlap}', which checks already requires`);
  stringList(geometry.qualification.passStates, 'check profile.geometry.qualification.passStates');
  exactKeys(geometry.joins, ['bolt', 'weld'], 'check profile.geometry.joins');
  exactKeys(geometry.joins.bolt, ['types', 'plies'], 'check profile.geometry.joins.bolt');
  stringList(geometry.joins.bolt.types, 'check profile.geometry.joins.bolt.types');
  exactKeys(geometry.joins.bolt.plies, ['property', 'format'], 'check profile.geometry.joins.bolt.plies');
  exactKeys(geometry.joins.weld, ['types', 'parts', 'unknownWhen'], 'check profile.geometry.joins.weld');
  stringList(geometry.joins.weld.types, 'check profile.geometry.joins.weld.types');
  exactKeys(geometry.joins.weld.parts, ['property', 'format'], 'check profile.geometry.joins.weld.parts');
  exactKeys(geometry.joins.weld.unknownWhen, ['property', 'absent'], 'check profile.geometry.joins.weld.unknownWhen');
  for (const [label, spec] of [['bolt.plies', geometry.joins.bolt.plies], ['weld.parts', geometry.joins.weld.parts]]) {
    if (!nonEmpty(spec.property)) fail('profile-invalid', `check profile.geometry.joins.${label}.property must be a non-empty string`);
    if (!JOIN_FORMATS.includes(spec.format)) fail('profile-invalid', `check profile.geometry.joins.${label}.format must be one of ${JOIN_FORMATS.join(', ')}`);
  }
  if (!nonEmpty(geometry.joins.weld.unknownWhen.property)) fail('profile-invalid', 'check profile.geometry.joins.weld.unknownWhen.property must be a non-empty string');
  if (geometry.joins.weld.unknownWhen.absent !== true) fail('profile-invalid', 'check profile.geometry.joins.weld.unknownWhen.absent must be true (the only condition this build knows)');
  if (!Array.isArray(geometry.groups)) fail('profile-invalid', 'check profile.geometry.groups must be an array');
  const names = new Set();
  const groupList = geometry.groups.map((group, index) => {
    const label = `check profile.geometry.groups[${index}]`;
    exactKeys(group, ['kind', 'name'], label, ['providerRelationKinds']);
    if (!GROUP_KINDS.includes(group.kind)) fail('profile-invalid', `${label}.kind '${String(group.kind)}' is not in the relationship vocabulary (${GROUP_KINDS.join(', ')})`);
    const providerKinds = 'providerRelationKinds' in group ? providerRelationKindsOf(group.providerRelationKinds, `${label}.providerRelationKinds`) : [];
    if (!nonEmpty(group.name) || camelCase(group.name) === '') fail('profile-invalid', `${label}.name must be a non-empty string with letters or digits`);
    const evidenceKey = camelCase(group.name);
    if (names.has(evidenceKey)) fail('profile-invalid', `${label}.name '${group.name}' is used twice`);
    names.add(evidenceKey);
    return { kind: group.kind, name: group.name, evidenceKey, providerKinds: new Set(providerKinds) };
  });
  // The first `hosts` group is also where a weld with unknown parts is looked for (`weld-parts-unknown`, a verdict). A
  // profile that uses providerRelationKinds (0.7.0) must leave no doubt which group that is; older profiles keep the
  // first-group rule unchanged.
  if (groupList.some((g) => g.providerKinds.size > 0) && groupList.filter((g) => g.kind === 'hosts').length > 1) {
    fail('profile-invalid', 'check profile.geometry.groups may hold only one hosts group when a group lists providerRelationKinds (the hosts group is where a weld with unknown parts is looked for)');
  }
  exactKeys(geometry.budgets, ['trianglesPerObject', 'trianglePairsPerPair', 'gapSearchMm', 'pairs', 'focusSets'], 'check profile.geometry.budgets');
  for (const key of ['trianglesPerObject', 'trianglePairsPerPair', 'pairs']) {
    if (!isCount(geometry.budgets[key]) || geometry.budgets[key] < 1) fail('profile-invalid', `check profile.geometry.budgets.${key} must be a positive integer`);
  }
  if (!isCount(geometry.budgets.focusSets)) fail('profile-invalid', 'check profile.geometry.budgets.focusSets must be a non-negative integer');
  if (!positive(geometry.budgets.gapSearchMm) || geometry.budgets.gapSearchMm < geometry.contactTolerance) fail('profile-invalid', 'check profile.geometry.budgets.gapSearchMm must be a positive number no smaller than contactTolerance');
  const minOverlapGiven = geometry.minOverlap !== undefined;
  if (minOverlapGiven && (!positive(geometry.minOverlap) || geometry.minOverlap < geometry.contactTolerance)) {
    fail('profile-invalid', 'check profile.geometry.minOverlap must be a positive number no smaller than contactTolerance');
  }
  return {
    raw: geometry, nodeRegex, coverageRegex, groupList,
    tolerance: geometry.contactTolerance, weldTolerance: geometry.weldTolerance, budgets: geometry.budgets,
    // A REPORTING threshold only: whether two solids pass into each other is still decided at `contactTolerance`.
    minOverlap: minOverlapGiven ? geometry.minOverlap : geometry.contactTolerance, minOverlapGiven,
    boltTypes: new Set(geometry.joins.bolt.types), weldTypes: new Set(geometry.joins.weld.types),
  };
}

/** The pair rules: the only predicates `appliesTo.pairs` may narrow (bolted plies follow the bolt group, not a type pair). */
export const PAIR_PREDICATES = ['clash-overlap', 'clash-contact'];
const typePairKey = (a, b) => (codeUnitCompare(a, b) <= 0 ? `${a}\u0000${b}` : `${b}\u0000${a}`);

/**
 * `appliesTo.pairs` (0.6.0): which unordered pairs of types a pair rule checks. Each type must also be in `appliesTo.types`;
 * the list is normalised (each pair sorted, duplicates removed, sorted). `[]` is invalid: a rule that checks no pair is left
 * out of the profile instead. Absent (`null`): every pair of `types`, as before.
 */
export function compileRulePairs(rule, label) {
  const raw = rule.appliesTo.pairs;
  if (raw === undefined) return null;
  if (!PAIR_PREDICATES.includes(rule.predicate)) fail('profile-invalid', `${label}.appliesTo.pairs is allowed only for the ${PAIR_PREDICATES.join(' and ')} predicates`);
  if (!Array.isArray(raw) || raw.length === 0) fail('profile-invalid', `${label}.appliesTo.pairs must be a non-empty array of [typeA, typeB] pairs (leave the rule out to check no pair)`);
  const keys = new Set();
  for (const [index, pair] of raw.entries()) {
    if (!Array.isArray(pair) || pair.length !== 2 || !pair.every(nonEmpty)) fail('profile-invalid', `${label}.appliesTo.pairs[${index}] must be two non-empty type strings`);
    for (const type of pair) if (!rule.appliesTo.types.includes(type)) fail('profile-invalid', `${label}.appliesTo.pairs[${index}] names '${type}', which is not in appliesTo.types`);
    keys.add(typePairKey(pair[0], pair[1]));
  }
  const list = [...keys].sort(codeUnitCompare).map((key) => key.split('\u0000'));
  return { set: keys, list };
}

/** Predicate/target pairing and worsening metric for a clash rule, checked at profile validation. */
export function validateClashRule(rule, label) {
  if (rule.target !== CLASH_TARGET) fail('profile-invalid', `${label}.target must be '${CLASH_TARGET}' for the '${rule.predicate}' predicate`);
  if (rule.worsening !== null && !WORSENING_PATHS[rule.predicate].includes(rule.worsening.path)) {
    fail('profile-invalid', `${label}.worsening.path '${rule.worsening.path}' is not a number the '${rule.predicate}' predicate produces (${WORSENING_PATHS[rule.predicate].join(', ') || 'none'})`);
  }
}

// ─── the engine ─────────────────────────────────────────────────────────────────────────────────────

let enginePromise = null;
/** Load manifold-3d once per process. The wasm bytes come from `manifold-wasm.mjs` (inlined in the shipped bridge). */
export function loadClashEngine() {
  enginePromise ??= (async () => {
    const { default: Module } = await import('manifold-3d');
    const wasm = await Module({ wasmBinary: manifoldWasmBytes() });
    wasm.setup();
    return { Manifold: wasm.Manifold, Mesh: wasm.Mesh };
  })();
  return enginePromise;
}

// ─── geometry input ─────────────────────────────────────────────────────────────────────────────────

const sha256Of = (bytes) => createHash('sha256').update(bytes).digest('hex');

/** `{ tiles: [{ordinal, file, sha256, bytes}] }` from a geometry directory, every digest verified. */
export function readGeometryDir(dir, label) {
  const root = resolve(dir);
  if (!existsSync(root) || !statSync(root).isDirectory()) fail('input-missing', `\`${label}\` is not a directory: ${root}`);
  const doc = readJsonFile(join(root, 'geometry.json'), `${label} geometry.json`).value;
  exactKeys(doc, ['tiles'], `${label} geometry.json`, ['qualification']);
  if (!Array.isArray(doc.tiles)) fail('document-invalid', `${label} geometry.json tiles must be an array`);
  const tiles = doc.tiles.map((tile, index) => {
    exactKeys(tile, ['ordinal', 'file', 'sha256'], `${label} geometry.json tiles[${index}]`);
    if (!isCount(tile.ordinal)) fail('document-invalid', `${label} geometry.json tiles[${index}].ordinal must be a non-negative integer`);
    if (!nonEmpty(tile.file) || !/^[A-Za-z0-9_][A-Za-z0-9._-]*$/.test(tile.file)) fail('document-invalid', `${label} geometry.json tiles[${index}].file must be a plain file name`);
    if (typeof tile.sha256 !== 'string' || !/^[0-9a-f]{64}$/.test(tile.sha256)) fail('document-invalid', `${label} geometry.json tiles[${index}].sha256 must be 64 lower-case hex characters`);
    const path = join(root, tile.file);
    if (!existsSync(path) || !statSync(path).isFile()) fail('input-missing', `${label} tile not found: ${tile.file}`);
    return { ordinal: tile.ordinal, file: tile.file, sha256: tile.sha256, bytes: readFileSync(path) };
  });
  let qualification = null;
  if (doc.qualification !== undefined) {
    const q = doc.qualification;
    exactKeys(q, ['file', 'sha256'], `${label} geometry.json qualification`);
    if (!nonEmpty(q.file) || !/^[A-Za-z0-9_][A-Za-z0-9._-]*$/.test(q.file)) fail('document-invalid', `${label} geometry.json qualification.file must be a plain file name`);
    if (typeof q.sha256 !== 'string' || !/^[0-9a-f]{64}$/.test(q.sha256)) fail('document-invalid', `${label} geometry.json qualification.sha256 must be 64 lower-case hex characters`);
    const path = join(root, q.file);
    if (!existsSync(path) || !statSync(path).isFile()) fail('input-missing', `${label} qualification file not found: ${q.file}`);
    qualification = { file: q.file, sha256: q.sha256, bytes: readFileSync(path) };
  }
  return { tiles, qualification };
}

/** Verify digests, order by ordinal, refuse duplicates; returns the tiles (with bytes) and what is echoed in `inputs.geometry`. */
export function normalizeGeometrySide(input, label) {
  if (!isObject(input) || !Array.isArray(input.tiles)) fail('input-invalid', `${label} geometry must be { tiles: [...] }`);
  const ordinals = new Set(); const files = new Set();
  for (const tile of input.tiles) {
    if (ordinals.has(tile.ordinal) || files.has(tile.file)) fail('document-invalid', `${label} geometry names tile ${tile.ordinal}/${tile.file} twice`);
    ordinals.add(tile.ordinal); files.add(tile.file);
    if (sha256Of(tile.bytes) !== tile.sha256) fail('geometry-digest-mismatch', `${label} tile ${tile.file} does not match its sha256`);
  }
  const tiles = [...input.tiles].sort((a, b) => a.ordinal - b.ordinal);
  // Coverage rows handed over beside the tiles (the query properties shard does not hold them). They are used to
  // qualify solids only: never compared, never part of the pairing.
  let qualification = null;
  let qualificationRows = null;
  if (input.qualification !== undefined && input.qualification !== null) {
    const q = input.qualification;
    if (sha256Of(q.bytes) !== q.sha256) fail('geometry-digest-mismatch', `${label} qualification file ${q.file} does not match its sha256`);
    let doc;
    try { doc = JSON.parse(Buffer.from(q.bytes).toString('utf8')); } catch { fail('geometry-invalid', `${label} qualification file ${q.file} is not JSON`); }
    if (!isObject(doc) || Object.keys(doc).length !== 1 || !Array.isArray(doc.properties)) fail('geometry-invalid', `${label} qualification file ${q.file} must be exactly { properties: [...] }`);
    qualificationRows = new Map();
    for (const [index, row] of doc.properties.entries()) {
      if (!isObject(row) || !nonEmpty(row.entityId)) fail('geometry-invalid', `${label} qualification file properties[${index}] must be an object with an entityId`);
      const list = qualificationRows.get(row.entityId);
      if (list) list.push(row); else qualificationRows.set(row.entityId, [row]);
    }
    qualification = { file: q.file, sha256: q.sha256 };
  }
  return { tiles, qualificationRows, echo: { qualification, tiles: tiles.map((t) => ({ file: t.file, ordinal: t.ordinal, sha256: t.sha256 })) } };
}

function meshesOf(side, label) {
  const merged = new Map();
  for (const tile of side.tiles) {
    for (const [name, mesh] of readTileMeshes(tile.bytes, `${label} tile ${tile.file}`)) {
      if (merged.has(name)) fail('geometry-invalid', `${label} holds two nodes named '${name}'`);
      merged.set(name, mesh);
    }
  }
  return merged;
}

// ─── one side of the comparison ─────────────────────────────────────────────────────────────────────

const lowerKey = (text) => text.toLowerCase();

function makeSide({ name, indexed, namespace, geo, meshes, engine, handles, qualificationRows }) {
  const { tolerance } = geo;
  const identities = [...indexed.pairable.keys()].sort(codeUnitCompare);
  const entityOf = (identity) => indexed.entities.get(indexed.pairable.get(identity));
  const typeOf = (identity) => entityOf(identity)?.type;
  const prop = (identity, key) => (indexed.rowsByEntity.get(indexed.pairable.get(identity)) ?? []).find((r) => r.sourceKey === key)?.value;
  const coverageField = geo.raw.qualification.propertyField;
  const rowsMatching = (identity, regex) => {
    const entityId = indexed.pairable.get(identity);
    // Rows handed over beside the tiles replace the snapshot's own: they are the only place the coverage lives.
    const all = qualificationRows ? (qualificationRows.get(entityId) ?? [])
      : [...(indexed.rowsByEntity.get(entityId) ?? []), ...(indexed.unkeyedByEntity.get(entityId) ?? [])];
    return all.filter((r) => typeof r[coverageField] === 'string' && regex.test(r[coverageField]))
      .sort((a, b) => codeUnitCompare(a[coverageField], b[coverageField]));
  };

  const lower = new Map();
  for (const identity of identities) {
    const key = lowerKey(identity.slice(namespace.length + 1));
    lower.set(key, lower.has(key) ? null : identity);
  }
  const resolveToken = (token) => {
    const exact = `${namespace}:${token}`;
    if (indexed.pairable.has(exact)) return exact;
    return lower.get(lowerKey(token)) ?? null;
  };

  // ── joins: which parts a bolt group or weld names ──
  const anchors = new Map();
  const readList = (identity, spec) => {
    const rows = (indexed.rowsByEntity.get(indexed.pairable.get(identity)) ?? []).filter((r) => r.sourceKey === spec.property);
    // Two rows with this key that disagree cannot be told apart by their order: the join is then not known.
    const conflict = new Set(rows.map((r) => r.value)).size > 1;
    const value = conflict ? rows.map((r) => r.value).filter((v) => typeof v === 'string').join(',') : rows[0]?.value;
    if (typeof value !== 'string') return null;
    const tokens = value.split(',').map((t) => t.trim()).filter(Boolean);
    const parts = new Set(); const unmatched = [];
    for (const token of tokens) { const id = resolveToken(token); if (id === null) unmatched.push(token); else parts.add(id); }
    // On a conflict every candidate part from every row is kept, so each of them reads as join-uncertain, never as unjoined.
    return { parts: [...parts].sort(codeUnitCompare), unmatched: [...new Set(unmatched)].sort(codeUnitCompare), conflict };
  };
  for (const identity of identities) {
    const type = typeOf(identity);
    if (geo.boltTypes.has(type)) {
      const read = readList(identity, geo.raw.joins.bolt.plies);
      anchors.set(identity, { kind: 'bolt', known: read !== null && !read.conflict, conflict: read?.conflict === true, parts: read?.parts ?? [], unmatched: read?.unmatched ?? [] });
    } else if (geo.weldTypes.has(type)) {
      const read = readList(identity, geo.raw.joins.weld.parts);
      const unknown = prop(identity, geo.raw.joins.weld.unknownWhen.property) === undefined;
      anchors.set(identity, { kind: 'weld', known: read !== null && !unknown && !read.conflict, conflict: read?.conflict === true, parts: read?.parts ?? [], unmatched: read?.unmatched ?? [] });
    }
  }
  const joinedPairs = new Set(); const named = new Map(); const uncertainParts = new Set();
  const pairKey = (a, b) => (codeUnitCompare(a, b) <= 0 ? `${a}\u0000${b}` : `${b}\u0000${a}`);
  for (const [identity, anchor] of anchors) {
    if (anchor.known) {
      for (let i = 0; i < anchor.parts.length; i += 1) for (let j = i + 1; j < anchor.parts.length; j += 1) joinedPairs.add(pairKey(anchor.parts[i], anchor.parts[j]));
      named.set(identity, new Set(anchor.parts));
    }
    if (anchor.unmatched.length > 0 || anchor.conflict) for (const part of anchor.parts) uncertainParts.add(part);
  }

  // ── groups: same assembly / same component ──
  const parentsByGroup = new Map();
  for (const group of geo.groupList) {
    const parents = new Map();
    const add = (from, to) => { const set = parents.get(to); if (set) set.add(from); else parents.set(to, new Set([from])); };
    for (const triple of indexed.relationships) {
      const [kind, from, to] = triple.split('\u0000');
      if (kind === group.kind) add(from, to);
    }
    // A `provider-explicit` row whose providerRelationKind the profile lists for this group means the same (#882).
    for (const providerKind of group.providerKinds) {
      for (const [from, to] of indexed.providerRelationships.get(providerKind) ?? []) add(from, to);
    }
    parentsByGroup.set(group.evidenceKey, parents);
  }
  const shares = (evidenceKey, a, b) => {
    const parents = parentsByGroup.get(evidenceKey);
    const pa = parents?.get(a); const pb = parents?.get(b);
    if (!pa || !pb) return false;
    for (const p of pa) if (pb.has(p)) return true;
    return false;
  };
  const hostsGroup = geo.groupList.find((g) => g.kind === 'hosts');
  const unknownAssemblies = new Set();
  if (hostsGroup) {
    for (const [identity, anchor] of anchors) {
      if (anchor.kind !== 'weld' || anchor.known) continue;
      for (const p of parentsByGroup.get(hostsGroup.evidenceKey).get(identity) ?? []) unknownAssemblies.add(p);
    }
  }
  const inUnknownAssembly = (a, b) => {
    if (!hostsGroup || unknownAssemblies.size === 0) return false;
    const parents = parentsByGroup.get(hostsGroup.evidenceKey);
    const pa = parents.get(a); const pb = parents.get(b);
    if (!pa || !pb) return false;
    for (const p of pa) if (pb.has(p) && unknownAssemblies.has(p)) return true;
    return false;
  };

  // ── coverage + solids ──
  const coverageOf = (identity) => {
    const rows = rowsMatching(identity, geo.coverageRegex);
    if (rows.length === 0) return null;
    let doc = rows[0].value;
    if (typeof doc === 'string') { try { doc = JSON.parse(doc); } catch { return { unreadable: true, checks: [] }; } }
    const list = isObject(doc) ? doc[geo.raw.qualification.json] : undefined;
    if (!Array.isArray(list)) return { unreadable: true, checks: [] };
    return { unreadable: false, checks: list.filter(isObject) };
  };
  const failingReason = (coverage) => {
    if (coverage === null) return { key: 'coverage:missing', code: null, text: 'no import coverage for this part' };
    if (coverage.unreadable) return { key: 'coverage:unreadable', code: null, text: 'its import coverage could not be read' };
    const { checks: required, whenPresent } = geo.raw.qualification;
    for (const checkName of [...required, ...whenPresent]) {
      const found = coverage.checks.find((c) => c.name === checkName);
      if (!found) {
        if (whenPresent.includes(checkName)) continue;   // a row carries only the checks that apply to its kind of object
        return { key: `coverage:${checkName}:missing`, code: null, text: `the coverage check '${checkName}' is missing` };
      }
      if (!geo.raw.qualification.passStates.includes(found.state)) {
        const code = typeof found.code === 'string' && found.code !== '' ? found.code : String(found.state);
        return { key: `coverage:${checkName}:${code}`, code: typeof found.code === 'string' && found.code !== '' ? found.code : null, text: `'${checkName}' is ${found.state}${found.code ? ` (${found.code})` : ''}` };
      }
    }
    return null;
  };
  const openCode = (coverage) => {
    if (!coverage || coverage.unreadable) return null;
    const pinned = failingReason(coverage);
    if (pinned?.code) return pinned.code;
    const other = coverage.checks.find((c) => !geo.raw.qualification.passStates.includes(c.state) && typeof c.code === 'string' && c.code !== '');
    return other ? other.code : null;
  };

  const nodeNameOf = (identity) => {
    const entityId = indexed.pairable.get(identity);
    const match = geo.nodeRegex.exec(entityId);
    if (!match || match[0] !== entityId) return null;
    return geo.raw.node.nodeName.replace(/\$([1-9])/g, (_, n) => match[Number(n)] ?? '');
  };
  const rawMesh = (identity) => {
    const nodeName = nodeNameOf(identity);
    const mesh = nodeName === null ? undefined : meshes.get(nodeName);
    return mesh && mesh.tris.length > 0 ? mesh : null;
  };
  const rawBoxes = new Map();
  const rawBox = (identity) => {
    if (!rawBoxes.has(identity)) { const mesh = rawMesh(identity); rawBoxes.set(identity, mesh ? boxOf(mesh.positions) : null); }
    return rawBoxes.get(identity);
  };

  const solids = new Map();
  const solid = (identity) => {
    let s = solids.get(identity);
    if (s) return s;
    const mesh = rawMesh(identity);
    const coverage = coverageOf(identity);
    s = { identity, state: 'evaluated', reasonKey: null, reason: null, approximate: false, closed: false, manifold: undefined };
    if (!mesh) {
      const code = openCode(coverage) ?? 'no-solid';
      Object.assign(s, { state: 'no-geometry', reasonKey: code, reason: `its solid could not be built (${code})` });
    } else if (mesh.tris.length / 3 > geo.budgets.trianglesPerObject) {
      Object.assign(s, { state: 'over-budget', reasonKey: 'triangle-budget', reason: `its solid has more than ${geo.budgets.trianglesPerObject} triangles` });
    } else {
      const welded = weld(mesh.positions, mesh.tris, geo.weldTolerance);
      const verdict = checkMesh(welded.verts, welded.tris);
      s.verts = welded.verts; s.tris = welded.tris; s.volume = verdict.volume; s.area = verdict.area;
      s.box = boxOf(welded.verts);
      s.triBoxes = triangleBoxes(welded.verts, welded.tris);
      s.origin = [Math.floor(s.box[0]), Math.floor(s.box[1]), Math.floor(s.box[2])];
      if (verdict.reason) {
        Object.assign(s, { state: 'approximate', reasonKey: verdict.reason, reason: `its mesh is not a closed solid (${verdict.reason})`, approximate: true, closed: false });
      } else {
        s.closed = true;
        const failing = failingReason(coverage);
        if (failing) Object.assign(s, { state: 'approximate', reasonKey: failing.key, reason: `its solid is approximate: ${failing.text}`, approximate: true });
      }
    }
    solids.set(identity, s);
    return s;
  };
  const manifoldOf = (s) => {
    if (s.manifold !== undefined) return s.manifold;
    const vertProperties = new Float32Array(s.verts.length);
    for (let i = 0; i < s.verts.length; i += 3) for (let axis = 0; axis < 3; axis += 1) vertProperties[i + axis] = s.verts[i + axis] - s.origin[axis];
    try {
      const m = new engine.Manifold(new engine.Mesh({ numProp: 3, vertProperties, triVerts: Uint32Array.from(s.tris) }));
      const status = m.status();
      if (status !== 'NoError' || !(m.volume() > 0)) {
        m.delete();
        s.manifold = null;
        Object.assign(s, { state: 'approximate', reasonKey: `manifold-${status}`, reason: `its mesh is not a valid solid (${status})`, approximate: true, closed: false });
      } else { s.manifold = m; handles.push(m); }
    } catch (error) {
      s.manifold = null;
      const status = String(error?.message ?? error).replace(/\s+/g, '-').toLowerCase();
      Object.assign(s, { state: 'approximate', reasonKey: `manifold-${status}`, reason: `its mesh is not a valid solid (${String(error?.message ?? error)})`, approximate: true, closed: false });
    }
    return s.manifold;
  };
  const directions = (s) => { s.directions ??= faceDirections(s.verts, s.tris); return s.directions; };

  return {
    name, indexed, identities, typeOf, entityOf, tolerance, anchors, joinedPairs, named, uncertainParts, rawBox, solid, manifoldOf, directions,
    has: (identity) => indexed.pairable.has(identity),
    nodeNameOf, shares, inUnknownAssembly, pairKey,
    joined: (a, b) => joinedPairs.has(pairKey(a, b)),
    ownJoint: (a, b) => {
      for (const [x, y] of [[a, b], [b, a]]) { if (named.get(x)?.has(y)) return anchors.get(x).kind; }
      return null;
    },
    pairs: new Map(), stacks: new Map(),
  };
}

// ─── the pair evaluation ────────────────────────────────────────────────────────────────────────────

const OWN_JOINT_TEXT = {
  bolt: 'a bolt passes through the parts it joins; the hole is not checked',
  weld: 'a weld sits in the joint it makes; the joint is not checked',
};
const verdict = (status, extra = {}) => ({ status, ...extra });
const belowThresholdText = (thickness, minOverlap) => `the parts overlap by ${mm(thickness)} mm, no thicker than the report threshold (minOverlap ${mm(minOverlap)} mm)`;
const notEvaluated = (reasonKey, reason) => verdict('not-evaluated', { reasonKey, reason });
const boxesNear = (a, b, t) => a[0] <= b[3] + t && a[3] >= b[0] - t && a[1] <= b[4] + t && a[4] >= b[1] - t && a[2] <= b[5] + t && a[5] >= b[2] - t;

/** The thickest region A ∩ B shares, if any exceeds t. Each region is measured alone along the face directions of both parts. */
function overlapOf(sc, A, B) {
  const mA = sc.manifoldOf(A); const mB = sc.manifoldOf(B);
  if (!mA || !mB) return { failed: true };
  const temps = [];
  try {
    return overlapRegions(sc, A, B, mA, mB, temps);
  } finally {
    for (const h of temps) { try { h.delete(); } catch { /* already freed */ } }
  }
}

function overlapRegions(sc, A, B, mA, mB, temps) {
  const t = sc.tolerance;
  const shifted = mB.translate([B.origin[0] - A.origin[0], B.origin[1] - A.origin[1], B.origin[2] - A.origin[2]]);
  temps.push(shifted);
  const inter = mA.intersect(shifted);
  temps.push(inter);
  if (inter.numTri() === 0) return { failed: false, evidence: null };
  const regions = inter.decompose();
  temps.push(...regions);
  const dirMap = new Map();
  for (const d of [...sc.directions(A), ...sc.directions(B)]) {
    const key = d.map((c) => { const r = Math.round(c * 1e9); return r === 0 ? 0 : r; }).join(',');
    if (!dirMap.has(key)) dirMap.set(key, d);
  }
  let dirs = [...dirMap.values()];
  const measured = [];
  let maxVerts = 0;
  for (const region of regions) {
    const mesh = region.getMesh();
    const verts = mesh.vertProperties;
    maxVerts = Math.max(maxVerts, verts.length / mesh.numProp);
    measured.push({ region, verts, numProp: mesh.numProp });
  }
  if (dirs.length * maxVerts > 5e7) {   // pathological curved meshes: coarser de-duplication keeps the work bounded
    const coarse = new Map();
    for (const d of dirs) coarse.set(d.map((c) => { const r = Math.round(c * 1e6); return r === 0 ? 0 : r; }).join(','), d);
    dirs = [...coarse.values()];
  }
  const big = [];
  for (const { region, verts, numProp } of measured) {
    const n = verts.length / numProp;
    let thickness = Infinity;
    for (const d of dirs) {
      let lo = Infinity; let hi = -Infinity;
      for (let i = 0; i < n; i += 1) {
        const p = verts[i * numProp] * d[0] + verts[i * numProp + 1] * d[1] + verts[i * numProp + 2] * d[2];
        if (p < lo) lo = p;
        if (p > hi) hi = p;
      }
      if (hi - lo < thickness) thickness = hi - lo;
    }
    if (!(thickness > t)) continue;
    const box = [Infinity, Infinity, Infinity, -Infinity, -Infinity, -Infinity];
    for (let i = 0; i < n; i += 1) for (let axis = 0; axis < 3; axis += 1) {
      const v = verts[i * numProp + axis] + A.origin[axis];
      if (v < box[axis]) box[axis] = v;
      if (v > box[axis + 3]) box[axis + 3] = v;
    }
    big.push({ thickness, volume: region.volume(), box });
  }
  if (big.length === 0) return { failed: false, evidence: null };
  let thickest = big[0];
  for (const r of big) if (r.thickness > thickest.thickness || (r.thickness === thickest.thickness && r.volume > thickest.volume)) thickest = r;
  const total = inter.volume();
  // "Inside" means what is left of a part outside the other is a sliver no thicker than the tolerance: its volume is at
  // most (t / 2) x its own surface area. Comparing volumes against t x the part's area alone would call a plate that
  // overlaps another by 80% coincident.
  const sliver = (m) => { const n = m.numTri() === 0 || m.volume() <= (t / 2) * m.surfaceArea(); return n; };
  const aOut = mA.subtract(shifted); temps.push(aOut);
  const bOut = shifted.subtract(mA); temps.push(bOut);
  const insideA = sliver(aOut);
  const insideB = sliver(bOut);
  const kind = insideA && insideB ? 'coincident' : insideA || insideB ? 'contained' : 'overlap';
  return {
    failed: false,
    thickness: thickest.thickness,   // unrounded: the report threshold is compared with this, never with the rounded evidence
    evidence: {
      kind,
      overlapThicknessMm: mm(thickest.thickness),
      overlapExtentMm: [mm(thickest.box[3] - thickest.box[0]), mm(thickest.box[4] - thickest.box[1]), mm(thickest.box[5] - thickest.box[2])],
      locationMm: point([(thickest.box[0] + thickest.box[3]) / 2, (thickest.box[1] + thickest.box[4]) / 2, (thickest.box[2] + thickest.box[5]) / 2]),
      regions: big.length,
      overlapVolumeMm3: mm3(big.reduce((sum, r) => sum + r.volume, 0)),
      thicknessMethod: 'across-part-faces',
    },
  };
}

function evalPair(sc, a, b, ctx) {
  const key = sc.pairKey(a, b);
  const cached = sc.pairs.get(key);
  if (cached) return cached;
  const result = computePair(sc, a, b, ctx);
  sc.pairs.set(key, result);
  return result;
}

function computePair(sc, a, b, ctx) {
  const both = (v) => ({ interp: v, contact: v });
  const A = sc.solid(a); const B = sc.solid(b);
  for (const [id, s] of [[a, A], [b, B]]) {
    if (s.state === 'no-geometry' || s.state === 'over-budget') return both(notEvaluated(s.state === 'over-budget' ? 'triangle-budget' : 'no-geometry', `${id}: ${s.reason}`));
  }
  const own = sc.ownJoint(a, b);
  const joined = own !== null || sc.joined(a, b);
  if (own === null) {
    for (const [id, s] of [[a, A], [b, B]]) {
      if (!s.closed) return both(notEvaluated('mesh-not-closed', `${id}: ${s.reason}`));
      sc.manifoldOf(s);
      if (!s.closed) return both(notEvaluated('mesh-not-closed', `${id}: ${s.reason}`));
    }
    if ((A.tris.length / 3) * (B.tris.length / 3) > ctx.geo.budgets.trianglePairsPerPair) {
      return both(notEvaluated('triangle-budget', 'the two solids have more triangle pairs than the check limit allows'));
    }
  }
  const approximate = A.state === 'approximate' || B.state === 'approximate';
  const approxText = () => `${[A, B].filter((s) => s.state === 'approximate').map((s) => `${s === A ? a : b}: ${s.reason}`).join('; ')}`;
  const t = sc.tolerance;
  const near = A.box !== undefined && B.box !== undefined && boxesNear(A.box, B.box, t);

  let interp;
  let overlap = null;
  if (own !== null) interp = notEvaluated('own-joint', OWN_JOINT_TEXT[own]);
  else {
    let thickness = 0;
    if (near) {
      const r = overlapOf(sc, A, B);
      if (r.failed) return both(notEvaluated('mesh-not-closed', `${[A, B].find((s) => !s.closed)?.identity}: ${[A, B].find((s) => !s.closed)?.reason}`));
      overlap = r.evidence;
      thickness = r.thickness;
    }
    // The parts pass into each other (classified at the contact tolerance, so contact stays answered by this verdict either
    // way); an overlap no thicker than the report threshold is measured and counted, never listed.
    if (overlap && thickness > ctx.geo.minOverlap) interp = verdict('finding', { evidence: { ...overlap, approximate }, approximate });
    // Below the threshold passes the rule, which an approximate solid can never establish: then it is not evaluated.
    else if (overlap) interp = approximate ? notEvaluated('approximate', approxText()) : verdict('below-threshold', { thickness });
    else interp = approximate ? notEvaluated('approximate', approxText()) : verdict('passed');
  }

  let contact;
  if (own !== null) contact = approximate ? notEvaluated('approximate', approxText()) : verdict('passed');
  else if (overlap) contact = notEvaluated('interpenetrates', 'the two parts pass into each other (see the interpenetration finding)');
  else {
    const hit = near ? meshDistance(A, B, t) : null;
    if (!hit) contact = approximate ? notEvaluated('approximate', approxText()) : verdict('passed');
    else if (joined) contact = approximate ? notEvaluated('approximate', approxText()) : verdict('passed');
    else if (sc.uncertainParts.has(a) || sc.uncertainParts.has(b)) {
      contact = notEvaluated('join-uncertain', 'a bolt or weld here names a part that is not in this saved state, so FloLess cannot tell whether these two parts are joined');
    } else if (sc.inUnknownAssembly(a, b)) {
      contact = notEvaluated('weld-parts-unknown', 'a weld here names no parts, so FloLess cannot tell whether these are welded');
    } else {
      // The measured values are written last, so a group whose name reads the same as one of them can never replace it.
      const evidence = {};
      for (const group of ctx.geo.groupList) evidence[group.evidenceKey] = sc.shares(group.evidenceKey, a, b);
      Object.assign(evidence, { gapMm: mm(hit.distance), locationMm: point(hit.location), approximate });
      contact = verdict('finding', { evidence, approximate });
    }
  }
  return { interp, contact };
}

// ─── bolted stacks ──────────────────────────────────────────────────────────────────────────────────

function evalStack(sc, anchor, ctx) {
  const cached = sc.stacks.get(anchor);
  if (cached) return cached;
  const result = computeStack(sc, anchor, ctx);
  sc.stacks.set(anchor, result);
  return result;
}

function computeStack(sc, anchor, ctx) {
  const info = sc.anchors.get(anchor);
  const stop = (reasonKey, reason) => ({ status: 'not-evaluated', reasonKey, reason, plies: info?.parts ?? [], detached: [], main: [] });
  if (!info || info.kind !== 'bolt' || !info.known) return stop('plies-unknown', 'this bolt group does not say which parts it joins');
  if (info.unmatched.length > 0) return stop('join-uncertain', `this bolt group names ${info.unmatched.length} part(s) that are not in this saved state, so the stack cannot be checked`);
  const plies = info.parts;
  if (plies.length < 2) return stop('fewer-than-two-plies', 'this bolt group joins fewer than two parts, so there is no stack to check');
  for (const ply of plies) {
    const s = sc.solid(ply);
    if (s.state === 'evaluated') sc.manifoldOf(s);
    if (s.state !== 'evaluated' || !s.closed) {
      const key = s.state === 'no-geometry' ? 'no-geometry' : s.state === 'over-budget' ? 'triangle-budget' : 'ply-not-evaluated';
      return stop(key, `${ply}: ${s.reason ?? 'its solid is not fully evaluated'}`);
    }
  }
  const t = sc.tolerance;
  const cap = ctx.geo.budgets.gapSearchMm;
  const dist = new Map();
  for (let i = 0; i < plies.length; i += 1) for (let j = i + 1; j < plies.length; j += 1) {
    const A = sc.solid(plies[i]); const B = sc.solid(plies[j]);
    if ((A.tris.length / 3) * (B.tris.length / 3) > ctx.geo.budgets.trianglePairsPerPair) return stop('triangle-budget', `${plies[i]} and ${plies[j]} have more triangle pairs than the check limit allows`);
    dist.set(`${i},${j}`, meshDistance(A, B, cap));
  }
  const parent = plies.map((_, i) => i);
  const find = (x) => { while (parent[x] !== x) { parent[x] = parent[parent[x]]; x = parent[x]; } return x; };
  for (const [key, hit] of dist) {
    if (hit && hit.distance <= t) { const [i, j] = key.split(',').map(Number); parent[find(i)] = find(j); }
  }
  const groups = new Map();
  plies.forEach((ply, i) => { const root = find(i); const g = groups.get(root); if (g) g.push(i); else groups.set(root, [i]); });
  if (groups.size === 1) return { status: 'connected', reasonKey: null, reason: null, plies, detached: [], main: plies };
  let main = null;
  for (const members of groups.values()) {   // plies are sorted, so members[0] is each group's smallest identity
    if (main === null || members.length > main.length || (members.length === main.length && members[0] < main[0])) main = members;
  }
  const mainSet = new Set(main);
  const detached = [];
  plies.forEach((ply, i) => {
    if (mainSet.has(i)) return;
    let best = null;
    for (const m of main) {
      const hit = dist.get(i < m ? `${i},${m}` : `${m},${i}`);
      if (hit && (best === null || hit.distance < best.distance)) best = hit;
    }
    detached.push({ ply, gap: best });
  });
  return { status: 'split', reasonKey: null, reason: null, plies, detached, main: main.map((i) => plies[i]) };
}

const stackEvidence = (entry, cap) => (entry.gap
  ? { gapMm: mm(entry.gap.distance), locationMm: point(entry.gap.location), method: 'whole-ply-nearest' }
  : { gapMm: null, gapAtLeastMm: cap, locationMm: null, method: 'whole-ply-nearest' });

// ─── the whole evaluation ───────────────────────────────────────────────────────────────────────────

/** The earlier side of a check of ONE state (single mode): it holds nothing, so nothing is compared with it. */
const EMPTY_SIDE = Object.freeze({
  identities: [], anchors: new Map(), has: () => false, typeOf: () => undefined, rawBox: () => null, tolerance: 0,
});

/** Whether a pair rule checks this pair of parts on this side: `na` (a type is outside it), `excluded` (its pair policy leaves the type pair out) or `applies`. */
function ruleAppliesTo(rule, sc, x, y) {
  const tx = sc.typeOf(x); const ty = sc.typeOf(y);
  if (!rule.types.has(tx) || !rule.types.has(ty)) return 'na';
  if (rule.pairs && !rule.pairs.set.has(typePairKey(tx, ty))) return 'excluded';
  return 'applies';
}

/** A max-heap holding at most `limit` entries: the `limit` smallest by `cmp` are kept, every other one is handed to `drop`. */
function boundedSmallest(limit, cmp, drop) {
  const heap = [];
  const up = (i) => { while (i > 0) { const p = (i - 1) >> 1; if (cmp(heap[i], heap[p]) <= 0) break; [heap[i], heap[p]] = [heap[p], heap[i]]; i = p; } };
  const down = (i) => {
    for (;;) {
      const l = 2 * i + 1; const r = l + 1; let m = i;
      if (l < heap.length && cmp(heap[l], heap[m]) > 0) m = l;
      if (r < heap.length && cmp(heap[r], heap[m]) > 0) m = r;
      if (m === i) return;
      [heap[i], heap[m]] = [heap[m], heap[i]]; i = m;
    }
  };
  return {
    offer(entry) {
      if (heap.length < limit) { heap.push(entry); up(heap.length - 1); return; }
      if (cmp(entry, heap[0]) < 0) { const out = heap[0]; heap[0] = entry; down(0); drop(out); } else drop(entry);
    },
    sorted: () => [...heap].sort(cmp),
  };
}

/**
 * @param {object} input
 *   pairing      the shared pairing (`pairSnapshots` + `diffPairs` scope already applied by the caller); in single mode
 *                `{ compiled, revisedSide, baseSide: null }`
 *   rules        { overlap, contact, bolted } compiled rules (each optional)
 *   geo          compiled geometry block
 *   geometry     { base, revised } normalised geometry sides (single mode: `revised` only)
 *   engine       the loaded manifold engine
 *   scope        Set of in-scope identities (added, changed, focus present; single mode: the focus, or every identity)
 *   removed      Set of identities removed in the revised state (single mode: empty)
 *   focusSets    [[a, b], ...] sorted pairs the caller wants re-checked
 *   single       true for a check of one state (0.6.0): no earlier side, and the wording says so
 *   requested    single mode with a focus: the requested identities, sorted (the focus before its closure)
 */
export function evaluateClash(input) {
  const { pairing, rules, geo, geometry, engine, scope, focusSets, single = false, requested = null } = input;
  const handles = [];
  const ctx = { geo, engine, handles };
  try {
    const namespace = pairing.compiled.profile.identity.namespace;
    const pairRules = [rules.overlap, rules.contact].filter(Boolean);
    const pairTypes = new Set(pairRules.flatMap((r) => [...r.types]));
    const common = { namespace, geo, engine, handles };
    const rev = makeSide({ ...common, name: 'revised', indexed: pairing.revisedSide, meshes: meshesOf(geometry.revised, 'revised'), qualificationRows: geometry.revised.qualificationRows });
    const base = pairing.baseSide
      ? makeSide({ ...common, name: 'base', indexed: pairing.baseSide, meshes: meshesOf(geometry.base, 'base'), qualificationRows: geometry.base.qualificationRows })
      : EMPTY_SIDE;

    // ── the dependency closure (plan §3.5) ──
    const changedAnchors = new Set();
    const closure = new Set(scope);
    const partsOf = (identity) => [...(rev.anchors.get(identity)?.parts ?? []), ...(base.anchors.get(identity)?.parts ?? [])];
    for (const identity of new Set([...rev.anchors.keys(), ...base.anchors.keys()])) {
      if (scope.has(identity) || input.removed.has(identity)) { changedAnchors.add(identity); for (const p of partsOf(identity)) closure.add(p); }
    }
    const anchorsToCheck = new Set(changedAnchors);
    for (const identity of new Set([...rev.anchors.keys(), ...base.anchors.keys()])) {
      if (partsOf(identity).some((p) => scope.has(p))) { anchorsToCheck.add(identity); for (const p of partsOf(identity)) closure.add(p); }
    }

    const units = [];
    const byRule = {};
    // Per rule, what the v2 and single-mode documents say beyond the v1 counts: why each pair was not judged, the pairs its
    // pair policy left out, and the overlaps measured but too thin to report.
    const detail = {};
    for (const rule of Object.values(rules)) {
      if (!rule) continue;
      byRule[rule.id] = { evaluated: 0, findings: 0, notApplicable: 0, notEvaluated: 0 };
      detail[rule.id] = { reasons: {}, pairExcluded: 0, belowThreshold: { count: 0, max: null } };
    }
    const notEvaluatedStats = { pairs: 0, stacks: 0, byReason: {} };
    const bump = (reasonKey, kind) => { notEvaluatedStats[kind] += 1; notEvaluatedStats.byReason[reasonKey] = (notEvaluatedStats.byReason[reasonKey] ?? 0) + 1; };
    const notJudged = (rule, reasonKey) => {
      byRule[rule.id].notEvaluated += 1;
      const reasons = detail[rule.id].reasons;
      reasons[reasonKey] = (reasons[reasonKey] ?? 0) + 1;
    };
    const ownJointCounts = {};
    const stackPlies = new Set();   // the plies a bolt-stack evaluation inspected: their solids are in scope for the tally too

    // ── candidate pairs (the broad phase), from both sides, bounded ──
    // A side proposes a pair when both parts are of a type some pair rule covers, at least one is in the closure, and their
    // raw boxes are within the tolerance (exactly what the sweep below finds, so it is also how a re-check asks the question).
    const proposes = (sc, x, y) => {
      if (!sc.has(x) || !sc.has(y) || !pairTypes.has(sc.typeOf(x)) || !pairTypes.has(sc.typeOf(y))) return false;
      if (!closure.has(x) && !closure.has(y)) return false;
      const a = sc.rawBox(x); const b = sc.rawBox(y);
      return Boolean(a && b && boxesNear(a, b, sc.tolerance));
    };
    const sweep = (sc, visit) => {
      const items = [];
      for (const identity of sc.identities) {
        if (!pairTypes.has(sc.typeOf(identity))) continue;
        const box = sc.rawBox(identity);
        if (box) items.push({ identity, box });
      }
      items.sort((x, y) => x.box[0] - y.box[0] || codeUnitCompare(x.identity, y.identity));
      const t = sc.tolerance;
      for (let i = 0; i < items.length; i += 1) {
        for (let j = i + 1; j < items.length && items[j].box[0] <= items[i].box[3] + t; j += 1) {
          if (!closure.has(items[i].identity) && !closure.has(items[j].identity)) continue;
          if (boxesNear(items[i].box, items[j].box, t)) visit(...[items[i].identity, items[j].identity].sort(codeUnitCompare));
        }
      }
    };
    // Only pairs a rule checks count against the pair budget (a pair its rule's pair policy leaves out is counted as such for
    // that rule, before any budget is used). The first `budgets.pairs` of them, in a fixed order, are kept; every later one
    // is counted and not stored, so a whole-model check holds at most that many pairs.
    const order = (p, q) => codeUnitCompare(p.first, q.first) || codeUnitCompare(p.second, q.second);
    const kept = boundedSmallest(geo.budgets.pairs, order, (entry) => {
      bump('pair-budget', 'pairs');
      for (const rule of entry.applicable) notJudged(rule, 'pair-budget');
    });
    const consider = (x, y) => {
      const applicable = [];
      for (const rule of pairRules) {
        const status = ruleAppliesTo(rule, rev, x, y);
        if (status === 'applies') applicable.push(rule);
        else if (status === 'excluded') detail[rule.id].pairExcluded += 1;
      }
      if (applicable.length === 0) return;
      const first = closure.has(x) ? x : y;
      kept.offer({ pair: [x, y], first, second: first === x ? y : x, applicable });
    };
    if (pairRules.length > 0) {
      sweep(rev, consider);
      // The earlier state proposes a pair only when both parts are in the later one and the later state did not propose it.
      sweep(base, (x, y) => { if (rev.has(x) && rev.has(y) && !proposes(rev, x, y)) consider(x, y); });
    }
    const ordered = kept.sorted();
    const keptPairs = new Set(ordered.map(({ pair: [x, y] }) => rev.pairKey(x, y)));
    const beyondBudget = (x, y) => !keptPairs.has(rev.pairKey(x, y)) && (proposes(rev, x, y) || proposes(base, x, y))
      && pairRules.some((rule) => ruleAppliesTo(rule, rev, x, y) === 'applies');
    const PAIR_BUDGET_TEXT = 'this pair is beyond the number of pairs the check limit allows';

    for (const { pair, applicable } of ordered) {
      const [x, y] = pair;
      const revPair = evalPair(rev, x, y, ctx);
      const basePair = base.has(x) && base.has(y) ? evalPair(base, x, y, ctx) : null;
      let counted = false;
      for (const rule of applicable) {
        const key = rule.predicate === 'clash-overlap' ? 'interp' : 'contact';
        const rv = revPair[key];
        const stat = byRule[rule.id];
        // The interpenetration finding answers a contact question only when that rule ran for this pair; without it the
        // pair is a pair the contact rule could not judge, and is counted as such.
        if (rv.reasonKey === 'interpenetrates' && applicable.includes(rules.overlap)) { /* answered by the interpenetration verdict */ }
        else if (rv.status === 'not-evaluated') {
          notJudged(rule, rv.reasonKey);
          if (!counted) { bump(rv.reasonKey, 'pairs'); counted = true; if (rv.reasonKey === 'own-joint') ownJointCounts[rv.reason] = (ownJointCounts[rv.reason] ?? 0) + 1; }
        } else {
          stat.evaluated += 1;
          if (rv.status === 'finding') stat.findings += 1;
          else if (rv.status === 'below-threshold') {
            const below = detail[rule.id].belowThreshold;
            below.count += 1;
            if (below.max === null || rv.thickness > below.max) below.max = rv.thickness;
          }
        }
        // A pair that exists in the earlier state but to which the rule did not apply there was not checked there.
        const appliedBefore = basePair && ruleAppliesTo(rule, base, x, y) === 'applies';
        const before = !basePair ? null : appliedBefore ? basePair[key] : notEvaluated('rule-not-applicable-in-base', 'the rule did not apply to both parts in the earlier state');
        units.push({ ruleId: rule.id, members: [x, y], rev: rv, base: before });
      }
    }

    if (rules.bolted) {
      const rule = rules.bolted;
      const stat = byRule[rule.id];
      const cap = geo.budgets.gapSearchMm;
      for (const anchor of [...anchorsToCheck].sort(codeUnitCompare)) {
        const type = rev.has(anchor) ? rev.typeOf(anchor) : undefined;
        if (!rev.has(anchor) || !geo.boltTypes.has(type) || !rule.types.has(type)) continue;
        const rs = evalStack(rev, anchor, ctx);
        // A stack the rule did not apply to in the earlier state (its type was outside the rule then) was not checked there.
        const baseIsBolt = base.has(anchor) && base.anchors.get(anchor)?.kind === 'bolt';
        const bs = !baseIsBolt ? null
          : rule.types.has(base.typeOf(anchor)) ? evalStack(base, anchor, ctx)
            : notEvaluated('rule-not-applicable-in-base', 'the rule did not apply to this bolt group in the earlier state');
        for (const ply of rs.plies ?? []) stackPlies.add(ply);
        if (rs.status === 'not-evaluated') { notJudged(rule, rs.reasonKey); bump(rs.reasonKey, 'stacks'); } else stat.evaluated += 1;
        const plyStateOf = (stack, ply) => {
          if (stack === null) return null;
          if (stack.status === 'not-evaluated') return notEvaluated(stack.reasonKey, stack.reason);
          const found = stack.detached.find((d) => d.ply === ply);
          if (found) return verdict('finding', { evidence: stackEvidence(found, cap) });
          if (!stack.plies.includes(ply)) return notEvaluated('ply-not-in-group', 'this ply is no longer part of the bolt group');
          return stack.status === 'connected' ? verdict('passed') : notEvaluated('stack-still-split', 'the stack is still split, so this ply cannot be shown to be fixed');
        };
        const plySet = new Set([...(rs.detached ?? []).map((d) => d.ply), ...(bs?.detached ?? []).map((d) => d.ply)]);
        for (const ply of [...plySet].sort(codeUnitCompare)) {
          // A ply that is gone from the revised state cannot be a finding there (and is not a participant to name).
          if (!rev.has(ply)) continue;
          const rv = plyStateOf(rs, ply);
          const bv = plyStateOf(bs, ply);
          if (rv.status === 'finding') stat.findings += 1;
          units.push({ ruleId: rule.id, members: [anchor, ply], rev: rv, base: bv });
        }
      }
    }

    // ── the solids in scope, by state (revised side) ──
    const tally = { total: 0, evaluated: 0, approximate: 0, noGeometry: 0, overBudget: 0, byReason: {} };
    const tallyReasons = { 'no-geometry': {}, approximate: {}, 'over-budget': {} };
    const count = (state, reasonKey) => {
      const field = { approximate: 'approximate', 'no-geometry': 'noGeometry', 'over-budget': 'overBudget' }[state];
      tally[field] += 1;
      tally.byReason[reasonKey] = (tally.byReason[reasonKey] ?? 0) + 1;
      tallyReasons[state][reasonKey] = (tallyReasons[state][reasonKey] ?? 0) + 1;
    };
    const unknown = [];
    const requestedSet = new Set(requested ?? []);
    for (const identity of [...closure].sort(codeUnitCompare)) {
      if (!rev.has(identity) || !(pairTypes.has(rev.typeOf(identity)) || stackPlies.has(identity) || requestedSet.has(identity))) continue;
      const s = rev.solid(identity);
      if (s.state === 'evaluated') rev.manifoldOf(s);
      tally.total += 1;
      if (s.state === 'evaluated') tally.evaluated += 1;
      else count(s.state, s.reasonKey);
      if (requestedSet.has(identity) && s.state === 'no-geometry') unknown.push(identity);
    }
    // A requested identity this state does not hold (or holds twice, or blank) has no solid here: it is counted with the
    // solids that could not be built, never dropped.
    for (const identity of requested ?? []) {
      if (rev.has(identity)) continue;
      tally.total += 1;
      count('no-geometry', 'not-in-snapshot');
      unknown.push(identity);
    }
    unknown.sort(codeUnitCompare);

    // ── re-checks for pairs ──
    const rows = [];
    const ruleList = Object.values(rules).filter(Boolean).sort((p, q) => codeUnitCompare(p.id, q.id));
    const revisedExcluded = new Set([...pairing.revisedSide.unusableIdentities, ...pairing.revisedSide.droppedIdentities]);
    const absenceProvable = pairing.revisedSide.blank.length === 0 && pairing.revisedSide.duplicate.length === 0;
    for (const [x, y] of focusSets) {
      for (const rule of ruleList) {
        const row = { identities: [x, y], ruleId: rule.id, state: null };
        const missing = [x, y].filter((id) => !rev.has(id));
        if (missing.length > 0) {
          const absent = missing.some((id) => !revisedExcluded.has(id) && absenceProvable);
          row.state = absent ? 'absent' : 'excluded';
        } else if (rule.predicate === 'clash-bolted-plies') {
          const anchorsHere = [x, y].filter((id) => geo.boltTypes.has(rev.typeOf(id)) && rule.types.has(rev.typeOf(id)));
          if (anchorsHere.length !== 1) row.state = 'not-applicable';
          else {
            const anchor = anchorsHere[0]; const ply = anchor === x ? y : x;
            const stack = evalStack(rev, anchor, ctx);
            if (stack.status === 'not-evaluated') Object.assign(row, { state: 'not-evaluated', reason: stack.reason });
            else if (!stack.plies.includes(ply)) Object.assign(row, { state: 'not-evaluated', reason: 'this ply is no longer part of the bolt group' });
            else if (stack.detached.some((d) => d.ply === ply)) row.state = 'finding';
            else if (stack.status === 'connected') row.state = 'passed';
            else Object.assign(row, { state: 'not-evaluated', reason: 'the stack is still split, so this ply cannot be shown to be fixed' });
          }
        } else if (ruleAppliesTo(rule, rev, x, y) !== 'applies') row.state = 'not-applicable';
        else {
          const pairVerdict = beyondBudget(x, y)
            ? notEvaluated('pair-budget', PAIR_BUDGET_TEXT)
            : evalPair(rev, x, y, ctx)[rule.predicate === 'clash-overlap' ? 'interp' : 'contact'];
          // An overlap no thicker than the report threshold passes the rule as the profile sets it: the re-check says passed,
          // and its reason says the parts still overlap by that much, so a pass never reads as "no overlap".
          if (pairVerdict.status === 'below-threshold') Object.assign(row, { state: 'passed', reason: belowThresholdText(pairVerdict.thickness, geo.minOverlap) });
          else {
            row.state = pairVerdict.status;
            if (pairVerdict.status === 'not-evaluated') row.reason = pairVerdict.reason;
          }
        }
        rows.push(row);
      }
    }
    rows.sort((p, q) => codeUnitCompare(p.identities.join('\u0000'), q.identities.join('\u0000')) || codeUnitCompare(p.ruleId, q.ruleId));

    // ── what was not checked, in words ──
    const listReasons = (counts) => Object.entries(counts).sort(([p], [q]) => codeUnitCompare(p, q)).map(([k, n]) => `${k}: ${n}`).join(', ');
    const parts = single ? 'part(s)' : 'changed part(s)';
    const unchecked = [];
    if (!single) {
      unchecked.push('Clash was checked between added and changed parts (and the parts a bolt or weld joins to them) and their neighbours; pairs of two unchanged parts were not checked.');
    } else if (requested !== null) {
      unchecked.push(`Only pairs that involve the ${requested.length} selected object(s), their connection parts, or anything touching them were checked (${[...closure].filter((identity) => rev.has(identity)).length} object(s) in all).`);
    }
    if (tally.noGeometry > 0) unchecked.push(`${tally.noGeometry} ${parts} could not be checked for clash because their solid could not be built (${listReasons(tallyReasons['no-geometry'])}).`);
    if (tally.approximate > 0) unchecked.push(`${tally.approximate} ${parts} have an approximate solid; a clean result for them is not claimed (${listReasons(tallyReasons.approximate)}).`);
    if (tally.overBudget > 0) unchecked.push(`${tally.overBudget} ${parts} have more triangles than the check limit allows and were not checked.`);
    const ownTotal = Object.values(ownJointCounts).reduce((s, n) => s + n, 0);
    if (ownTotal > 0) unchecked.push(`${ownTotal} pair(s) of a bolt or weld and a part it names were not checked for interpenetration (${Object.keys(ownJointCounts).sort(codeUnitCompare).join('; ')}).`);
    const tooLarge = notEvaluatedStats.byReason['triangle-budget'] ?? 0;
    if (tooLarge > 0) unchecked.push(`${tooLarge} pair(s) were too large to check within the limits.`);
    const beyond = notEvaluatedStats.byReason['pair-budget'] ?? 0;
    if (beyond > 0) unchecked.push(`${beyond} candidate pair(s) were not checked: the check stops at ${geo.budgets.pairs} pair(s), taken in a fixed order.`);
    const LISTED = new Set(['own-joint', 'triangle-budget', 'pair-budget']);
    const otherNot = notEvaluatedStats.pairs + notEvaluatedStats.stacks - ownTotal - tooLarge - beyond;
    if (otherNot > 0) unchecked.push(`${otherNot} pair(s) or bolt stack(s) could not be checked: ${listReasons(Object.fromEntries(Object.entries(notEvaluatedStats.byReason).filter(([k]) => !LISTED.has(k))))}.`);
    for (const rule of pairRules) {
      if (!rule.pairs) continue;
      const types = [...rule.types].sort(codeUnitCompare);
      const left = [];
      for (let i = 0; i < types.length; i += 1) for (let j = i; j < types.length; j += 1) if (!rule.pairs.set.has(typePairKey(types[i], types[j]))) left.push(`${types[i]} with ${types[j]}`);
      if (left.length > 0) unchecked.push(`Rule '${rule.id}' does not check ${left.join(', ')}: the check profile leaves these type pairs out (${detail[rule.id].pairExcluded} nearby pair(s) not checked).`);
    }
    if (rules.overlap && detail[rules.overlap.id].belowThreshold.count > 0) {
      const below = detail[rules.overlap.id].belowThreshold;
      unchecked.push(`${below.count} overlap(s) no thicker than ${geo.minOverlap} mm (the thickest ${mm(below.max)} mm) were measured and are not listed: the check profile reports only overlaps thicker than that.`);
    }
    const reserved = new Set(['gapMm', 'locationMm', 'approximate']);
    for (const group of geo.groupList) {
      if (reserved.has(group.evidenceKey)) unchecked.push(`The group '${group.name}' has the same name as a measured value (${group.evidenceKey}), so its yes/no is not shown on a contact finding; rename the group to see it.`);
    }
    if (rules.bolted) unchecked.push("Bearing at each bolt was not checked: this snapshot does not hold each bolt's position, so bolted parts were checked as whole plies (a stack counts as connected when every ply touches the rest).");

    const out = {
      units, rows, unchecked, byRule, detail,
      scope: {
        notEvaluated: notEvaluatedStats,
        solids: { total: tally.total, evaluated: tally.evaluated, approximate: tally.approximate, noGeometry: tally.noGeometry, overBudget: tally.overBudget, byReason: tally.byReason },
      },
      inputsGeometry: { base: single ? null : geometry.base.echo, revised: geometry.revised.echo },
    };
    if (requested !== null) out.focus = { requested: requested.length, effective: [...closure].filter((identity) => rev.has(identity)).length, unknown };
    return out;
  } finally {
    for (const h of handles) { try { h.delete(); } catch { /* already freed */ } }
  }
}

/**
 * What a check profile's geometry block and pair rules were read as (0.6.0), in AWARE's own words: echoed in
 * `evaluator.geometry` so a caller can assert the answer was measured with the settings it sent.
 */
export function geometryEcho(geo, clashRules) {
  const pairs = {};
  for (const rule of [clashRules.overlap, clashRules.contact]) if (rule) pairs[rule.id] = rule.pairs ? rule.pairs.list : null;
  return {
    contactTolerance: geo.tolerance, minOverlap: geo.minOverlap, weldTolerance: geo.weldTolerance, budgets: { ...geo.budgets }, pairs,
  };
}
