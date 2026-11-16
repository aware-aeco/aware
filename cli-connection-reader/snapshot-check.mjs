/**
 * check-snapshots — run a pinned set of data-defined checks over the objects a change touched, and say which
 * findings the change introduced, which were already there and which it resolved
 * (`aware.model-change-findings/v1`).
 *
 * PURE AND FORMAT-BLIND, like `compare-snapshots`, and built on the SAME pairing: this file imports the
 * pairing, the per-pair diff, the canonical JSON and the profile validation from `snapshot-compare.mjs`
 * instead of copying them, so an object is "added" or "changed" here exactly when `compare-snapshots` says so.
 * What a check tests is the check profile's data (`aware.model-check-profile/v1`); this file only applies it.
 *
 * SCOPE. Every revised object that is added or changed (from the full, uncapped pairing - never the capped
 * change rows) plus every identity the caller names for a re-check (`focus`). Unchanged objects that are not
 * named are not checked, and the result says so in `unchecked`.
 *
 * GUARANTEES
 *  - Read-only; canonical JSON (sorted keys): the same inputs give byte-identical output.
 *  - Counts are always complete; lists are capped afterwards, in a pinned order, and the cap is stated.
 *  - A finding is identified by `correspondenceKey` (which rule, which objects, where) and described by
 *    `evidenceFingerprint` (that plus the rule version and the evidence), so "the same finding as before"
 *    and "the same finding, but different" are different, exactly recomputable statements.
 */
import { resolve } from 'node:path';
import {
  canonicalJson, codeUnitCompare, diffPairs, exactKeys, fail, indexRevisedSide, isCount, isObject, pairSnapshots, readJsonFile,
  readSnapshotSide, readSnapshotSides, sha256, validateProfile, comparisonKey,
} from './snapshot-compare.mjs';
import {
  canonicalRows, compileCoverage, compileParameters, COVERAGE_SCHEMA, evaluateSingleRule, KEY_IS_IDENTITY_TEXT, NOT_EVALUATED_REASONS,
  OUT_OF_RANGE_WORSENING, PROPERTY_PREDICATES, PROPERTY_TARGET, RELATIONSHIP_TARGET, V2_PREDICATES, V2_TARGETS,
} from './snapshot-check-rules.mjs';
import {
  CLASH_PREDICATES, CLASH_TARGET, evaluateClash, loadClashEngine, normalizeGeometrySide, readGeometryDir,
  validateClashRule, validateGeometryBlock,
} from './snapshot-clash.mjs';

export const FINDINGS_SCHEMA = 'aware.model-change-findings/v1';
export const CHECK_PROFILE_SCHEMA = 'aware.model-check-profile/v1';
// v2 (model-change-set 0.4.0): the property and relationship predicates, `parameters`, a profile `scope`. A v2 profile
// gets the v2 findings document; a v1 profile gets exactly the v1 document it always got.
export const FINDINGS_SCHEMA_V2 = 'aware.model-change-findings/v2';
export const CHECK_PROFILE_SCHEMA_V2 = 'aware.model-check-profile/v2';
export const VALIDATION_SCHEMA = 'aware.model-check-validation/v1';
export const EVALUATOR_AGENT = 'model-change-set';
export const EVALUATOR_VERSION = '0.5.0';   // keep equal to 20-agents/aeco/engineering/model-change-set/manifest.yaml (a test pins it)
export { COVERAGE_SCHEMA };
const SEVERITIES = ['warning', 'error', 'info'];
const PREDICATES = ['blank', ...CLASH_PREDICATES];
const TARGETS = ['entity.name', CLASH_TARGET];
const SCOPES = ['changed', 'all'];
const TARGET_OF = {
  'property-missing': PROPERTY_TARGET, 'property-not-in': PROPERTY_TARGET, 'property-out-of-range': PROPERTY_TARGET,
  'relationship-missing': RELATIONSHIP_TARGET,
};
const DIRECTIONS = ['higher-is-worse', 'lower-is-worse'];
const digest = (value) => sha256(Buffer.from(canonicalJson(value), 'utf8'));

// ─── check profile + focus ──────────────────────────────────────────────────────────────────────────

const nonEmptyString = (v) => typeof v === 'string' && v !== '';

/** Strict validation: unknown or missing keys are rejected, as are predicates and targets this build does not know. */
export function validateCheckProfile(profile) {
  const v2 = isObject(profile) && profile.schema === CHECK_PROFILE_SCHEMA_V2;
  exactKeys(profile, ['schema', 'id', 'version', 'budgets', 'rules'], 'check profile', v2 ? ['geometry', 'scope'] : ['geometry']);
  if (profile.schema !== CHECK_PROFILE_SCHEMA && !v2) fail('profile-invalid', `check profile.schema must be '${CHECK_PROFILE_SCHEMA}' or '${CHECK_PROFILE_SCHEMA_V2}'`);
  // v1 is `changed` only. A v2 profile says which objects its one-object rules judge; clash keeps its own scope.
  const scopeMode = profile.scope === undefined ? 'changed' : profile.scope;
  if (!SCOPES.includes(scopeMode)) fail('profile-invalid', `check profile.scope must be one of ${SCOPES.join(', ')}`);
  const predicates = v2 ? [...PREDICATES, ...V2_PREDICATES] : PREDICATES;
  const targets = v2 ? [...TARGETS, ...V2_TARGETS] : TARGETS;
  for (const key of ['id', 'version']) if (!nonEmptyString(profile[key])) fail('profile-invalid', `check profile.${key} must be a non-empty string`);
  exactKeys(profile.budgets, ['findings', 'focusResults'], 'check profile.budgets');
  for (const key of ['findings', 'focusResults']) {
    if (!isCount(profile.budgets[key])) fail('profile-invalid', `check profile.budgets.${key} must be a non-negative integer`);
  }
  if (!Array.isArray(profile.rules)) fail('profile-invalid', 'check profile.rules must be an array');
  const seen = new Set();
  const rules = profile.rules.map((rule, index) => {
    const label = `check profile.rules[${index}]`;
    exactKeys(rule, ['id', 'version', 'severity', 'predicate', 'target', 'appliesTo', 'worsening'], label, v2 ? ['parameters'] : []);
    for (const key of ['id', 'version']) if (!nonEmptyString(rule[key])) fail('profile-invalid', `${label}.${key} must be a non-empty string`);
    if (seen.has(rule.id)) fail('profile-invalid', `${label}.id '${rule.id}' is used by two rules`);
    seen.add(rule.id);
    if (!SEVERITIES.includes(rule.severity)) fail('profile-invalid', `${label}.severity must be one of ${SEVERITIES.join(', ')}`);
    if (!predicates.includes(rule.predicate)) fail('profile-invalid', `${label}.predicate '${String(rule.predicate)}' is not known (expected: ${predicates.join(', ')})`);
    if (!targets.includes(rule.target)) fail('profile-invalid', `${label}.target '${String(rule.target)}' is not known (expected: ${targets.join(', ')})`);
    exactKeys(rule.appliesTo, ['types'], `${label}.appliesTo`);
    if (!Array.isArray(rule.appliesTo.types) || rule.appliesTo.types.length === 0 || rule.appliesTo.types.some((t) => typeof t !== 'string')) {
      fail('profile-invalid', `${label}.appliesTo.types must be a non-empty array of strings`);
    }
    let worsening = null;
    if (rule.worsening !== null) {
      exactKeys(rule.worsening, ['path', 'direction'], `${label}.worsening`);
      if (!nonEmptyString(rule.worsening.path)) fail('profile-invalid', `${label}.worsening.path must be a non-empty string`);
      if (!DIRECTIONS.includes(rule.worsening.direction)) fail('profile-invalid', `${label}.worsening.direction must be one of ${DIRECTIONS.join(', ')}`);
      // `blank` has no number to move; accepting a metric it can never produce would be a silent no-op.
      if (rule.predicate === 'blank') fail('profile-invalid', `${label}.worsening must be null for the 'blank' predicate`);
      worsening = rule.worsening;
    }
    let params = null;
    if (rule.predicate === 'blank') {
      if (rule.target !== 'entity.name') fail('profile-invalid', `${label}.target must be 'entity.name' for the 'blank' predicate`);
      if (rule.parameters !== undefined) fail('profile-invalid', `${label}.parameters is not allowed for the 'blank' predicate`);
    } else if (V2_PREDICATES.includes(rule.predicate)) {
      if (rule.target !== TARGET_OF[rule.predicate]) fail('profile-invalid', `${label}.target must be '${TARGET_OF[rule.predicate]}' for the '${rule.predicate}' predicate`);
      params = compileParameters(rule, label);
      if (rule.predicate === 'property-out-of-range') {
        // The distance outside the range is the one number this predicate produces; it is always the worsening metric.
        if (worsening !== null && (worsening.path !== OUT_OF_RANGE_WORSENING.path || worsening.direction !== OUT_OF_RANGE_WORSENING.direction)) {
          fail('profile-invalid', `${label}.worsening must be null or { path: 'distanceOutside', direction: 'higher-is-worse' } for the 'property-out-of-range' predicate`);
        }
        worsening = OUT_OF_RANGE_WORSENING;
      } else if (worsening !== null) fail('profile-invalid', `${label}.worsening must be null for the '${rule.predicate}' predicate`);
    } else {
      if (rule.parameters !== undefined) fail('profile-invalid', `${label}.parameters is not allowed for the '${rule.predicate}' predicate`);
      validateClashRule({ ...rule, worsening }, label);
    }
    return { ...rule, types: new Set(rule.appliesTo.types), worsening, params };
  });
  // Clash rules (plan 2026-11-12 S3): one rule per predicate, and the geometry block that says how to read the solids.
  const clash = rules.filter((rule) => CLASH_PREDICATES.includes(rule.predicate));
  const used = new Set();
  for (const rule of clash) {
    if (used.has(rule.predicate)) fail('profile-invalid', `check profile.rules: the '${rule.predicate}' predicate is used by two rules`);
    used.add(rule.predicate);
  }
  let geo = null;
  if (profile.geometry !== undefined) geo = validateGeometryBlock(profile.geometry);
  if (clash.length > 0 && geo === null) fail('profile-invalid', 'check profile.geometry is required when a rule uses a clash predicate');
  const clashRules = clash.length === 0 ? null : {
    overlap: clash.find((r) => r.predicate === 'clash-overlap'),
    contact: clash.find((r) => r.predicate === 'clash-contact'),
    bolted: clash.find((r) => r.predicate === 'clash-bolted-plies'),
  };
  return { profile, rules, geo, clashRules, v2, scopeMode };
}

/** A property rule that reads the comparison profile's identity row cannot be judged (identity rows are never kept as properties). */
function checkRuleKeysAgainstComparison(checks, compiled) {
  const identityKey = compiled.profile.identity.sourceKey;
  // The identity row's canonical key too: a key rule may rename it, and a rule naming the renamed key reads the same row.
  const identityCanonical = comparisonKey({ sourceKey: identityKey }, compiled);
  for (const rule of checks.rules) {
    if (PROPERTY_PREDICATES.includes(rule.predicate) && (rule.params.key === identityKey || rule.params.key === identityCanonical)) {
      fail('profile-invalid', `check profile rule '${rule.id}' reads '${identityKey}', the comparison profile's identity property, which every checked object has by construction`);
    }
  }
}

/** `{ identities: [string...] }` with no duplicates, plus optional `sets: [[a, b], ...]` (pairs to re-check; no duplicates). */
export function parseFocus(focus) {
  const keys = isObject(focus) ? Object.keys(focus) : [];
  const shapeOk = isObject(focus) && Array.isArray(focus.identities) && keys.every((k) => k === 'identities' || k === 'sets');
  if (!shapeOk) fail('focus-invalid', 'focus must be exactly { identities: [...] } (and optionally sets: [[a, b], ...])');
  const seen = new Set();
  for (const [index, identity] of focus.identities.entries()) {
    if (!nonEmptyString(identity)) fail('focus-invalid', `focus.identities[${index}] must be a non-empty string`);
    if (seen.has(identity)) fail('focus-invalid', `focus.identities names '${identity}' twice`);
    seen.add(identity);
  }
  const sets = [];
  if (focus.sets !== undefined) {
    if (!Array.isArray(focus.sets)) fail('focus-invalid', 'focus.sets must be an array of identity pairs');
    const pairs = new Set();
    for (const [index, set] of focus.sets.entries()) {
      if (!Array.isArray(set) || set.length !== 2 || !set.every(nonEmptyString) || set[0] === set[1]) fail('focus-invalid', `focus.sets[${index}] must be two different non-empty identity strings`);
      const sorted = [...set].sort(codeUnitCompare);
      const key = sorted.join('\u0000');
      if (pairs.has(key)) fail('focus-invalid', `focus.sets names the pair '${sorted[0]}', '${sorted[1]}' twice`);
      pairs.add(key);
      sets.push(sorted);
    }
    sets.sort((p, q) => codeUnitCompare(p.join('\u0000'), q.join('\u0000')));
  }
  return { identities: [...focus.identities].sort(codeUnitCompare), sets };
}

/** The sorted identities of a focus document (the one-object re-checks). */
export function validateFocus(focus) {
  return parseFocus(focus).identities;
}

// ─── predicates ─────────────────────────────────────────────────────────────────────────────────────

/** The finding a rule produces for one entity, or null. `blank` on `entity.name`: missing, not a string, or empty after NFKC + trim. */
function evaluate(rule, entity) {
  const raw = entity.name;
  const blank = typeof raw !== 'string' || raw.normalize('NFKC').trim() === '';
  return blank ? { evidence: { name: raw === undefined ? null : raw }, locus: {} } : null;
}

/**
 * The ONE judge of a one-object rule on one side of one object, used by pair mode (for both of its sides) and by single mode
 * (for the only state), so the two modes cannot disagree about an object. Returns
 * `{ status: 'finding', evidence, locus } | { status: 'passed' } | { status: 'not-evaluated', reason }`.
 * A property rule on the key of a REAL identity row of the state cannot be judged: identity rows are never kept as
 * properties, so the row would read as absent. Not evaluated (never a finding, never clean); every other rule is unaffected.
 * `ctx`: { side, coverage, entityId, entity, keyed, identityKeys }.
 */
export function judgeOneObject(rule, { side, coverage, entityId, entity, keyed, identityKeys }) {
  if (PROPERTY_PREDICATES.includes(rule.predicate) && identityKeys.has(rule.params.key)) return { status: 'not-evaluated', reason: 'key-is-identity' };
  if (rule.predicate === 'blank') {
    const hit = evaluate(rule, entity);
    return hit ? { status: 'finding', ...hit } : { status: 'passed' };
  }
  return evaluateSingleRule(rule, { side, coverage, entityId, keyed });
}

/** Who judged, under which profile: the version of this agent and the sha256 of the canonical check profile it was given. */
const evaluatorEcho = (checkProfile) => ({ agent: EVALUATOR_AGENT, version: EVALUATOR_VERSION, profileSha256: digest(checkProfile) });

const correspondenceKeyOf = (ruleId, participants, locus) => digest({ ruleId, participants: [...participants].sort(codeUnitCompare), locus });
const fingerprintOf = (correspondenceKey, ruleVersion, evidence) => digest({ correspondenceKey, ruleVersion, evidence });

/** `pre-existing-worsened` / `-improved` only when the rule names a numeric metric and it moved; otherwise changed. */
export function presenceOfChanged(rule, baseEvidence, revisedEvidence) {
  if (!rule.worsening) return 'pre-existing-changed';
  const before = baseEvidence?.[rule.worsening.path];
  const after = revisedEvidence?.[rule.worsening.path];
  if (typeof before !== 'number' || typeof after !== 'number' || !Number.isFinite(before) || !Number.isFinite(after) || before === after) return 'pre-existing-changed';
  const worse = rule.worsening.direction === 'higher-is-worse' ? after > before : after < before;
  return worse ? 'pre-existing-worsened' : 'pre-existing-improved';
}

// ─── the check ──────────────────────────────────────────────────────────────────────────────────────

// A not-evaluated earlier verdict that still says the pair was fine for this rule (the parts overlapped, or the ply was
// not in the group): the finding is new, not unknown.
const PRESENCE_CLEAN_REASONS = new Set(['interpenetrates', 'ply-not-in-group', 'stack-still-split']);

function capped(list, cap) {
  return list.length > cap ? { items: list.slice(0, cap), omitted: list.length - cap } : { items: list, omitted: 0 };
}

/**
 * @param {{ base, revised, profile, checkProfile, focus?: object|null, digests?: object,
 *           geometry?: { base: { tiles }, revised: { tiles } }, engine?: object,
 *           coverage?: { base?: object|null, revised?: object|null }, mode?: 'pair'|'single' }} input
 *   `mode: 'single'` (0.5.0) checks the `revised` state alone; see `checkSingleState`.
 *   `geometry` and `engine` are needed only by a check profile with clash rules (`engine` from `loadClashEngine()`).
 *   `coverage` (`aware.model-read-coverage/v1` documents) is read only by a v2 check profile's property and relationship rules.
 */
export function checkSnapshots({ base, revised, profile: rawProfile, checkProfile: rawCheckProfile, focus = null, digests = {}, geometry = null, engine = null, coverage = null, mode = 'pair' }) {
  if (!MODES.includes(mode)) fail('input-invalid', `mode must be one of ${MODES.join(', ')}`);
  if (mode === 'single') refuseClashRules(rawCheckProfile);
  const checks = validateCheckProfile(rawCheckProfile);
  if (mode === 'single') {
    // One state, no pairing: nothing about an earlier state may be supplied (it would be silently ignored otherwise).
    if (base != null || focus != null || geometry != null) fail('input-invalid', 'single mode takes only the later state: `base`, `focus` and `geometry` are not accepted');
    if (coverage && coverage.base != null) fail('input-invalid', 'single mode takes no base coverage');
    return checkSingleState({ revised, profile: rawProfile, checks, digests, coverage });
  }
  const focusDoc = focus === null ? { identities: [], sets: [] } : parseFocus(focus);
  const focusIdentities = focusDoc.identities;
  const clashRules = checks.clashRules;
  let clashGeometry = null;
  if (clashRules) {
    if (geometry === null || !isObject(geometry) || geometry.base === undefined || geometry.revised === undefined) {
      fail('input-invalid', 'the check profile has clash rules, so a base and a revised geometry input are required');
    }
    if (engine === null) fail('engine-unavailable', 'the clash engine is not loaded');
    clashGeometry = { base: normalizeGeometrySide(geometry.base, 'base'), revised: normalizeGeometrySide(geometry.revised, 'revised') };
  }
  const pairing = pairSnapshots({ base, revised, profile: rawProfile });
  const { compiled, baseSide, revisedSide, addedIdentities } = pairing;
  const v2 = checks.v2;
  if (v2) checkRuleKeysAgainstComparison(checks, compiled);
  const coverageOf = { base: null, revised: null };
  if (v2 && coverage) for (const side of ['base', 'revised']) if (coverage[side] != null) coverageOf[side] = compileCoverage(coverage[side], `${side} coverage`);

  // Scope: every added object, every changed pair, every named identity that is present in the revised state.
  const changed = new Set();
  for (const pair of diffPairs(pairing)) {
    if (pair.fields.length > 0) changed.add(pair.identity);
  }
  const focusPresent = focusIdentities.filter((identity) => revisedSide.pairable.has(identity));
  const scope = new Set([...addedIdentities, ...changed, ...focusPresent]);
  const scopeList = [...scope].sort(codeUnitCompare);
  // A v2 profile with scope 'all' judges every applicable object of the revised state, so a standard that is new today
  // finds what was already there as pre-existing. Under 'changed' its single-object rules judge added, changed and named
  // objects, EXCEPT `relationship-missing`: whether an object has a relationship is a fact about the whole object, not about
  // a diff (deleting a host changes nothing about the beam it hosted), so that rule always judges every applicable object.
  // Each rule says which it did in `evaluatedScope`. Clash keeps `scope` (its own closure rules).
  const allObjects = v2 && checks.scopeMode === 'all';
  const judgesAll = (rule) => allObjects || rule.predicate === 'relationship-missing';
  const wholeState = v2 && checks.rules.some((rule) => (rule.predicate === 'blank' || V2_PREDICATES.includes(rule.predicate)) && judgesAll(rule));
  const singleScopeList = wholeState ? [...revisedSide.pairable.keys()].sort(codeUnitCompare) : scopeList;

  const objectFor = (identity) => {
    const revisedId = revisedSide.pairable.get(identity);
    const revisedEntity = revisedSide.entities.get(revisedId);
    const baseId = baseSide.pairable.get(identity) ?? null;
    const baseEntity = baseId === null ? null : baseSide.entities.get(baseId);
    return { identity, revisedId, revisedEntity, baseId, baseEntity };
  };
  const participantOf = (identity) => {
    const { revisedId, revisedEntity, baseId } = objectFor(identity);
    return { identity, entityIdBase: baseId, entityIdRevised: revisedId, type: revisedEntity.type ?? null, name: revisedEntity.name ?? null };
  };
  const sides = { base: { side: baseSide, coverage: coverageOf.base }, revised: { side: revisedSide, coverage: coverageOf.revised } };
  const keyedMemo = { base: new Map(), revised: new Map() };
  const keyedOf = (sideName, entityId) => {
    let keyed = keyedMemo[sideName].get(entityId);
    if (!keyed) { keyed = canonicalRows(sides[sideName].side.rowsByEntity.get(entityId), compiled); keyedMemo[sideName].set(entityId, keyed); }
    return keyed;
  };
  // One object, one rule, one side (the shared `judgeOneObject`). Each state is judged with its OWN identity keys: a
  // property rule on a key that is an identity key of that state is not evaluable there. So a state gets the same verdict
  // as the later side of a pair as it does on its own (single mode), whatever the other state's identity keys were.
  const judge = (rule, sideName, entity, entityId) => judgeOneObject(rule, {
    side: sides[sideName].side, coverage: sides[sideName].coverage, entityId, entity, keyed: keyedOf(sideName, entityId),
    identityKeys: sides[sideName].side.identityKeys,
  });
  const withKeys = (rule, identity, hit) => {
    const correspondenceKey = correspondenceKeyOf(rule.id, [identity], hit.locus);
    return { correspondenceKey, locus: hit.locus, evidence: hit.evidence, evidenceFingerprint: fingerprintOf(correspondenceKey, rule.version, hit.evidence) };
  };

  // The one-object rules run per scope object; clash rules are pair rules and run below.
  const singleRules = checks.rules.filter((rule) => rule.predicate === 'blank' || V2_PREDICATES.includes(rule.predicate));
  const byRule = Object.fromEntries(checks.rules.map((rule) => [rule.id, { evaluated: 0, findings: 0, notApplicable: 0 }]));
  const lostBase = {};   // ruleId -> objects whose earlier state could not be judged
  if (v2) for (const rule of singleRules) { byRule[rule.id].notEvaluated = {}; lostBase[rule.id] = 0; }
  const findings = [];
  const resolved = [];
  const stateOf = new Map();   // `${identity}\u0000${ruleId}` -> 'finding' | 'passed' | 'not-applicable'
  let evaluated = 0;
  let notApplicable = 0;

  for (const identity of singleScopeList) {
    const { revisedId, revisedEntity, baseId, baseEntity } = objectFor(identity);
    const participants = [participantOf(identity)];
    for (const rule of singleRules) {
      if (!judgesAll(rule) && !scope.has(identity)) continue;   // this rule judges what changed, and this object did not
      if (!rule.types.has(revisedEntity.type)) {
        notApplicable += 1;
        byRule[rule.id].notApplicable += 1;
        stateOf.set(`${identity}\u0000${rule.id}`, 'not-applicable');
        continue;
      }
      const nowVerdict = judge(rule, 'revised', revisedEntity, revisedId);
      if (nowVerdict.status === 'not-evaluated') {
        // Neither a pass nor a finding: the object stays out of `stateOf` (a re-check row says 'not-evaluated').
        const reasons = byRule[rule.id].notEvaluated;
        reasons[nowVerdict.reason] = (reasons[nowVerdict.reason] ?? 0) + 1;
        continue;
      }
      evaluated += 1;
      byRule[rule.id].evaluated += 1;
      const baseVerdict = baseEntity && rule.types.has(baseEntity.type) ? judge(rule, 'base', baseEntity, baseId) : null;
      const before = baseVerdict && baseVerdict.status === 'finding' ? withKeys(rule, identity, baseVerdict) : null;
      const baseLost = baseVerdict !== null && baseVerdict.status === 'not-evaluated';
      if (baseLost) lostBase[rule.id] += 1;
      if (nowVerdict.status === 'finding') {
        const now = withKeys(rule, identity, nowVerdict);
        let presence = 'introduced';
        if (before) presence = before.evidenceFingerprint === now.evidenceFingerprint ? 'pre-existing' : presenceOfChanged(rule, before.evidence, now.evidence);
        // The earlier state had this object but could not judge it: "introduced" would be unproven.
        else if (baseLost) presence = 'unknown';
        byRule[rule.id].findings += 1;
        stateOf.set(`${identity}\u0000${rule.id}`, 'finding');
        findings.push({
          ruleId: rule.id, ruleVersion: rule.version, severity: rule.severity,
          correspondenceKey: now.correspondenceKey, participants, locus: now.locus,
          evidence: now.evidence, evidenceFingerprint: now.evidenceFingerprint,
          presence, baseEvidenceFingerprint: before ? before.evidenceFingerprint : null,
        });
      } else {
        stateOf.set(`${identity}\u0000${rule.id}`, 'passed');
        // Resolved by the change only when the rule still applies and now passes: an object the rule no longer
        // applies to (its type changed) was not shown to be fixed, so it is not listed as resolved. A revised state
        // that was not judged never resolves anything (that `continue` above).
        if (before) {
          resolved.push({
            ruleId: rule.id, ruleVersion: rule.version, correspondenceKey: before.correspondenceKey, participants, locus: before.locus,
            evidence: before.evidence, evidenceFingerprint: before.evidenceFingerprint,
          });
        }
      }
    }
  }

  // v2: objects the revised state excluded for an unusable identity (blank or duplicate) cannot be judged; each one a
  // rule applies to by type is counted, so `revisedComplete` stays false while any exist.
  if (v2) {
    const excludedIds = new Set([...revisedSide.blank, ...revisedSide.duplicate]);
    for (const rule of singleRules) {
      let count = 0;
      for (const entityId of excludedIds) if (rule.types.has(revisedSide.entities.get(entityId)?.type)) count += 1;
      if (count > 0) byRule[rule.id].notEvaluated['identity-unusable'] = count;
    }
  }

  // Clash: pair rules over the solids of what changed and its dependency closure (plan 2026-11-12 S3).
  let clash = null;
  if (clashRules) {
    const clashScope = new Set(scope);
    // The comparison profile may call a small move "unchanged" (its bounds tolerance), yet any move can make or break a
    // clash (a 0.06 mm move crosses a 0.1 mm contact tolerance). Re-diff with no bounds tolerance so every moved part is in
    // the clash scope.
    const compareTolerance = compiled.profile.geometry.toleranceMm;
    if (compareTolerance > 0) {
      const tight = { ...pairing, compiled: { ...compiled, profile: { ...compiled.profile, geometry: { ...compiled.profile.geometry, toleranceMm: 0 } } } };
      for (const pair of diffPairs(tight)) if (pair.fields.length > 0) clashScope.add(pair.identity);
    }
    for (const set of focusDoc.sets) for (const identity of set) if (revisedSide.pairable.has(identity)) clashScope.add(identity);
    clash = evaluateClash({
      pairing, rules: clashRules, geo: checks.geo, geometry: clashGeometry, engine, scope: clashScope,
      removed: new Set(pairing.removedIdentities), focusSets: focusDoc.sets,
    });
    const rulesById = new Map(checks.rules.map((rule) => [rule.id, rule]));
    for (const unit of clash.units) {
      const rule = rulesById.get(unit.ruleId);
      const participants = unit.members.map(participantOf);
      const correspondenceKey = correspondenceKeyOf(rule.id, unit.members, {});
      const before = unit.base && unit.base.status === 'finding'
        ? { evidence: unit.base.evidence, evidenceFingerprint: fingerprintOf(correspondenceKey, rule.version, unit.base.evidence) }
        : null;
      if (unit.rev.status === 'finding') {
        const evidence = unit.rev.evidence;
        const evidenceFingerprint = fingerprintOf(correspondenceKey, rule.version, evidence);
        let presence = 'introduced';
        if (before) presence = before.evidenceFingerprint === evidenceFingerprint ? 'pre-existing' : presenceOfChanged(rule, before.evidence, evidence);
        // The earlier state had this pair but could not judge it (no solid, open mesh, over budget, approximate and clean,
        // the rule not applying): "introduced" would be unproven, so the presence is unknown.
        else if (unit.base && unit.base.status === 'not-evaluated' && !PRESENCE_CLEAN_REASONS.has(unit.base.reasonKey)) presence = 'unknown';
        findings.push({
          ruleId: rule.id, ruleVersion: rule.version, severity: rule.severity,
          correspondenceKey, participants, locus: {}, evidence, evidenceFingerprint,
          presence, baseEvidenceFingerprint: before ? before.evidenceFingerprint : null,
        });
      } else if (unit.rev.status === 'passed' && before) {
        resolved.push({
          ruleId: rule.id, ruleVersion: rule.version, correspondenceKey, participants, locus: {},
          evidence: before.evidence, evidenceFingerprint: before.evidenceFingerprint,
        });
      }
    }
    for (const rule of Object.values(clashRules)) {
      if (!rule) continue;
      byRule[rule.id] = clash.byRule[rule.id];
      evaluated += clash.byRule[rule.id].evaluated;
    }
  }
  const order = (a, b) => codeUnitCompare(a.ruleId, b.ruleId) || codeUnitCompare(a.correspondenceKey, b.correspondenceKey);
  findings.sort(order);
  resolved.sort(order);

  // Focus: one row per named identity and rule. "Absent" is only said when the revised state has no identity
  // it could not vouch for; otherwise the object might be one of the excluded ones.
  const revisedExcludedIdentities = new Set([...revisedSide.unusableIdentities, ...revisedSide.droppedIdentities]);
  const absenceProvable = revisedSide.blank.length === 0 && revisedSide.duplicate.length === 0;
  const focusRows = [];
  for (const identity of focusIdentities) {
    const present = revisedSide.pairable.has(identity);
    for (const rule of [...singleRules].sort((a, b) => codeUnitCompare(a.id, b.id))) {
      let state;
      if (present) state = stateOf.get(`${identity}\u0000${rule.id}`) ?? 'not-evaluated';
      else if (revisedExcludedIdentities.has(identity) || !absenceProvable) state = 'excluded';
      else state = 'absent';
      focusRows.push({ identity, ruleId: rule.id, state });
    }
  }

  const findingsCap = capped(findings, checks.profile.budgets.findings);
  const resolvedCap = capped(resolved, checks.profile.budgets.findings);
  const focusCap = capped(focusRows, checks.profile.budgets.focusResults);
  const omitted = { findings: findingsCap.omitted, resolvedInChange: resolvedCap.omitted, focusResults: focusCap.omitted };
  let setsCap = null;
  if (clash) {
    setsCap = capped(clash.rows, checks.geo.budgets.focusSets);
    omitted.focusSets = setsCap.omitted;
  }
  const complete = Object.values(omitted).every((n) => n === 0);

  if (v2) {
    for (const rule of checks.rules) {
      const stat = byRule[rule.id];
      if (singleRules.includes(rule)) {
        const total = Object.values(stat.notEvaluated).reduce((n, count) => n + count, 0);
        stat.coverage = { revisedComplete: total === 0, baseComplete: lostBase[rule.id] === 0 };
        stat.evaluatedScope = judgesAll(rule) ? 'all' : 'changed';
      } else {
        // A clash rule counts pairs it could not judge, not objects: same shape, one pair-level reason.
        const pairs = stat.notEvaluated;
        stat.notEvaluated = pairs > 0 ? { 'pair-not-evaluated': pairs } : {};
        stat.coverage = { revisedComplete: pairs === 0, baseComplete: null };
      }
    }
  }
  const summaryByRule = Object.fromEntries(checks.rules.map((rule) => [rule.id, byRule[rule.id].findings]));
  // Two different reasons an object cannot be checked. Pairing puts a revised object in `duplicate` both when its
  // identity is used twice in the later state AND when the earlier state uses it twice (so it cannot be matched).
  const earlierTwice = revisedSide.droppedIdentities.size;
  const laterTwice = revisedSide.duplicate.length - earlierTwice;
  const excludedCount = revisedSide.blank.length + revisedSide.duplicate.length;
  const unchecked = [
    v2 && checks.scopeMode === 'all'
      ? 'Single-object checks ran on every object of the later state that a rule applies to, changed or not; clash checks ran on added and changed objects, and on objects named for a re-check, only.'
      : 'Checks ran on added and changed objects, and on objects named for a re-check, only; unchanged objects were not checked.',
    `Only the ${checks.rules.length} rule(s) of check profile '${checks.profile.id}' were run; anything they do not test was not checked.`,
  ];
  if (excludedCount > 0) {
    unchecked.push(`${excludedCount} object(s) in the later state were not checked: their identity was blank (${revisedSide.blank.length}), used twice in the later state (${laterTwice}), or used twice in the earlier state so they cannot be matched (${earlierTwice}).`);
  }
  if (clash) unchecked.push(...clash.unchecked);
  if (v2) {
    for (const rule of singleRules) {
      for (const [reason, count] of Object.entries(byRule[rule.id].notEvaluated).sort(([a], [b]) => codeUnitCompare(a, b))) {
        unchecked.push(`Rule '${rule.id}' could not be evaluated for ${count} object(s): ${reason === 'key-is-identity' ? KEY_IS_IDENTITY_TEXT : reason}. They are neither passed nor reported.`);
      }
    }
  }
  if (!complete) {
    const parts = [`${omitted.findings} finding(s)`, `${omitted.resolvedInChange} resolved finding(s)`, `${omitted.focusResults} re-check row(s)`];
    if (clash) parts.push(`${omitted.focusSets} pair re-check row(s)`);
    unchecked.push(`Partial list: ${parts.slice(0, -1).join(', ')} and ${parts.at(-1)} are not listed (the lists are limited by the check profile's budgets). Counts are complete.`);
  }

  const inputs = {
    base: digests.base ?? null, revised: digests.revised ?? null,
    profileDigest: digest(compiled.profile),
    checkProfileDigest: digest(checks.profile),
    focusDigest: focus === null ? null : digest(focus),
  };
  if (clash) inputs.geometry = clash.inputsGeometry;
  const scopeOut = {
    added: addedIdentities.length, changed: changed.size, focus: focusPresent.length,
    evaluated, notApplicable, excluded: excludedCount, byRule,
  };
  if (v2) { scopeOut.mode = checks.scopeMode; scopeOut.objects = singleScopeList.length; }
  if (clash) { scopeOut.notEvaluated = clash.scope.notEvaluated; scopeOut.solids = clash.scope.solids; }
  const out = {
    schema: v2 ? FINDINGS_SCHEMA_V2 : FINDINGS_SCHEMA,
    inputs,
    checkProfile: { id: checks.profile.id, version: checks.profile.version },
    scope: scopeOut,
    findings: findingsCap.items,
    resolvedInChange: resolvedCap.items,
    focusResults: focusCap.items,
    summary: {
      findings: findings.length,
      introduced: findings.filter((f) => f.presence === 'introduced').length,
      preExisting: findings.filter((f) => f.presence.startsWith('pre-existing')).length,
      resolvedInChange: resolved.length,
      byRule: summaryByRule,
    },
    truncated: complete ? null : { omitted },
    complete,
    unchecked,
  };
  if (clash) {
    out.focusSets = setsCap.items;
    out.summary.unknownPresence = findings.filter((f) => f.presence === 'unknown').length;
  }
  if (v2) {
    const unknown = findings.filter((f) => f.presence === 'unknown').length;
    out.summary.unknown = unknown;   // introduced + preExisting + unknown === findings
    out.summary.unknownPresence = unknown;
    out.evaluator = evaluatorEcho(checks.profile);
    out.coverageSha256 = { base: digests.coverage?.base ?? null, revised: digests.coverage?.revised ?? null };
  }
  return out;
}

// ─── single mode (model-change-set 0.5.0) ───────────────────────────────────────────────────────────

const MODES = ['pair', 'single'];

/**
 * Single mode judges ONE state on its own, so a profile is accepted only when every rule can be answered from that state
 * alone: a v2 profile (the v1 document has no place for "present"), scope 'all' (no diff decides what is judged), and no
 * clash predicate (clash is a pair-of-solids rule that needs geometry). Refused here, never skipped silently.
 */
function requireSingleModeProfile(checks) {
  if (!checks.v2) fail('profile-invalid', `single mode needs a '${CHECK_PROFILE_SCHEMA_V2}' check profile; a '${CHECK_PROFILE_SCHEMA}' profile is not accepted`);
  if (checks.scopeMode !== 'all') fail('profile-invalid', "single mode needs a check profile with scope 'all': there is no earlier state to say what changed");
  const clash = checks.rules.find((rule) => CLASH_PREDICATES.includes(rule.predicate));
  if (clash) fail('profile-invalid', `single mode cannot run the '${clash.predicate}' predicate of rule '${clash.id}': clash rules need geometry and an earlier state`);
}

/** Said before the profile's own validation, so a clash rule is refused as clash (not as a missing geometry block). */
function refuseClashRules(rawCheckProfile) {
  const rules = isObject(rawCheckProfile) && Array.isArray(rawCheckProfile.rules) ? rawCheckProfile.rules : [];
  const clash = rules.find((rule) => isObject(rule) && CLASH_PREDICATES.includes(rule.predicate));
  if (clash) fail('profile-invalid', `single mode cannot run the '${clash.predicate}' predicate${typeof clash.id === 'string' ? ` of rule '${clash.id}'` : ''}: clash rules need geometry and an earlier state`);
}

/** The findings of ONE state: every present violation of the profile's one-object rules, none of them compared with anything. */
function checkSingleState({ revised, profile: rawProfile, checks, digests = {}, coverage = null }) {
  requireSingleModeProfile(checks);
  const { compiled, revisedSide } = indexRevisedSide({ revised, profile: rawProfile });
  checkRuleKeysAgainstComparison(checks, compiled);
  const revisedCoverage = coverage && coverage.revised != null ? compileCoverage(coverage.revised, 'revised coverage') : null;
  const identityKeys = revisedSide.identityKeys;
  const keyedMemo = new Map();
  const keyedOf = (entityId) => {
    let keyed = keyedMemo.get(entityId);
    if (!keyed) { keyed = canonicalRows(revisedSide.rowsByEntity.get(entityId), compiled); keyedMemo.set(entityId, keyed); }
    return keyed;
  };
  const singleRules = checks.rules.filter((rule) => rule.predicate === 'blank' || V2_PREDICATES.includes(rule.predicate));
  const byRule = Object.fromEntries(checks.rules.map((rule) => [rule.id, { evaluated: 0, findings: 0, notApplicable: 0, notEvaluated: {} }]));
  const scopeList = [...revisedSide.pairable.keys()].sort(codeUnitCompare);
  const findings = [];
  let evaluated = 0;
  let notApplicable = 0;
  for (const identity of scopeList) {
    const entityId = revisedSide.pairable.get(identity);
    const entity = revisedSide.entities.get(entityId);
    const participants = [{ identity, entityIdBase: null, entityIdRevised: entityId, type: entity.type ?? null, name: entity.name ?? null }];
    for (const rule of singleRules) {
      if (!rule.types.has(entity.type)) { notApplicable += 1; byRule[rule.id].notApplicable += 1; continue; }
      const verdict = judgeOneObject(rule, { side: revisedSide, coverage: revisedCoverage, entityId, entity, keyed: keyedOf(entityId), identityKeys });
      if (verdict.status === 'not-evaluated') {
        const reasons = byRule[rule.id].notEvaluated;
        reasons[verdict.reason] = (reasons[verdict.reason] ?? 0) + 1;
        continue;
      }
      evaluated += 1;
      byRule[rule.id].evaluated += 1;
      if (verdict.status !== 'finding') continue;
      const correspondenceKey = correspondenceKeyOf(rule.id, [identity], verdict.locus);
      byRule[rule.id].findings += 1;
      findings.push({
        ruleId: rule.id, ruleVersion: rule.version, severity: rule.severity,
        correspondenceKey, participants, locus: verdict.locus,
        evidence: verdict.evidence, evidenceFingerprint: fingerprintOf(correspondenceKey, rule.version, verdict.evidence),
        presence: 'present', baseEvidenceFingerprint: null,
      });
    }
  }
  // Objects excluded for an unusable identity (blank or used twice in this state) cannot be judged; each one a rule applies
  // to by type is counted, so `revisedComplete` stays false while any exist.
  const excludedIds = new Set([...revisedSide.blank, ...revisedSide.duplicate]);
  for (const rule of singleRules) {
    let count = 0;
    for (const entityId of excludedIds) if (rule.types.has(revisedSide.entities.get(entityId)?.type)) count += 1;
    if (count > 0) byRule[rule.id].notEvaluated['identity-unusable'] = count;
  }
  findings.sort((a, b) => codeUnitCompare(a.ruleId, b.ruleId) || codeUnitCompare(a.correspondenceKey, b.correspondenceKey));
  for (const rule of checks.rules) {
    const stat = byRule[rule.id];
    const total = Object.values(stat.notEvaluated).reduce((n, count) => n + count, 0);
    stat.coverage = { revisedComplete: total === 0, baseComplete: null };
    stat.evaluatedScope = 'all';
  }
  const findingsCap = capped(findings, checks.profile.budgets.findings);
  const omitted = { findings: findingsCap.omitted, resolvedInChange: 0, focusResults: 0 };
  const complete = findingsCap.omitted === 0;
  const excludedCount = excludedIds.size;
  const unchecked = [
    'Only this model version was checked: nothing was compared with an earlier version, so nothing is said about what changed or whether a finding is new.',
    'Single-object checks ran on every object of this version that a rule applies to.',
    `Only the ${checks.rules.length} rule(s) of check profile '${checks.profile.id}' were run; anything they do not test was not checked.`,
    'Clash checks were not run: they need geometry and an earlier version to compare.',
  ];
  if (excludedCount > 0) {
    unchecked.push(`${excludedCount} object(s) in this version were not checked: their identity was blank (${revisedSide.blank.length}) or used twice in this version (${revisedSide.duplicate.length}).`);
  }
  for (const rule of singleRules) {
    for (const [reason, count] of Object.entries(byRule[rule.id].notEvaluated).sort(([a], [b]) => codeUnitCompare(a, b))) {
      unchecked.push(`Rule '${rule.id}' could not be evaluated for ${count} object(s): ${reason === 'key-is-identity' ? KEY_IS_IDENTITY_TEXT : reason}. They are neither passed nor reported.`);
    }
  }
  if (!complete) unchecked.push(`Partial list: ${omitted.findings} finding(s) are not listed (the list is limited by the check profile's budgets). Counts are complete.`);
  return {
    schema: FINDINGS_SCHEMA_V2,
    mode: 'single',
    inputs: {
      base: null, revised: digests.revised ?? null,
      profileDigest: digest(compiled.profile), checkProfileDigest: digest(checks.profile), focusDigest: null,
    },
    checkProfile: { id: checks.profile.id, version: checks.profile.version },
    scope: { added: 0, changed: 0, focus: 0, evaluated, notApplicable, excluded: excludedCount, byRule, mode: 'all', objects: scopeList.length },
    findings: findingsCap.items,
    resolvedInChange: [],
    focusResults: [],
    summary: {
      findings: findings.length,
      present: findings.length,
      byRule: Object.fromEntries(checks.rules.map((rule) => [rule.id, byRule[rule.id].findings])),
    },
    truncated: complete ? null : { omitted },
    complete,
    unchecked,
    evaluator: evaluatorEcho(checks.profile),
    coverageSha256: { base: null, revised: digests.coverage?.revised ?? null },
  };
}

// ─── file I/O wrapper (the CLI path) ────────────────────────────────────────────────────────────────

export async function checkSnapshotsFromFiles(args) {
  if (args['validate-only'] !== undefined && typeof args['validate-only'] !== 'boolean') fail('input-invalid', '`validate-only` must be a boolean when given');
  if (args['validate-only'] === true) return validateOnly(args);
  if (args.mode !== undefined && !MODES.includes(args.mode)) fail('input-invalid', `\`mode\` must be one of ${MODES.join(', ')} when given`);
  if (args.mode === 'single') return checkSingleFromFiles(args);
  for (const key of ['base-dir', 'revised-dir', 'profile-path', 'check-profile-path']) {
    if (typeof args[key] !== 'string' || args[key] === '') fail('input-invalid', `\`${key}\` is required, as a string`);
  }
  if (args['focus-path'] !== undefined && (typeof args['focus-path'] !== 'string' || args['focus-path'] === '')) fail('input-invalid', '`focus-path` must be a non-empty string when given');
  for (const key of ['base-geometry-dir', 'revised-geometry-dir', 'coverage-base-path', 'coverage-revised-path']) {
    if (args[key] !== undefined && (typeof args[key] !== 'string' || args[key] === '')) fail('input-invalid', `\`${key}\` must be a non-empty string when given`);
  }
  const { sides, digests } = readSnapshotSides(args);
  const profile = readJsonFile(resolve(args['profile-path']), 'profile').value;
  const checkProfile = readJsonFile(resolve(args['check-profile-path']), 'check profile').value;
  const focus = args['focus-path'] === undefined ? null : readJsonFile(resolve(args['focus-path']), 'focus').value;
  // Coverage is read only by a v2 profile (a v1 findings document has nowhere to echo it).
  let coverage = null;
  if (isObject(checkProfile) && checkProfile.schema === CHECK_PROFILE_SCHEMA_V2) {
    coverage = { base: null, revised: null };
    digests.coverage = { base: null, revised: null };
    for (const [side, key] of [['base', 'coverage-base-path'], ['revised', 'coverage-revised-path']]) {
      if (args[key] === undefined) continue;
      const read = readJsonFile(resolve(args[key]), `${side} coverage`);
      coverage[side] = read.value;
      digests.coverage[side] = read.digest;
    }
  }
  // Only a profile that asks for clash needs geometry or the engine; every other profile runs exactly as before.
  const wantsClash = isObject(checkProfile) && Array.isArray(checkProfile.rules)
    && checkProfile.rules.some((rule) => isObject(rule) && CLASH_PREDICATES.includes(rule.predicate));
  let geometry = null;
  let engine = null;
  if (wantsClash) {
    if (args['base-geometry-dir'] === undefined || args['revised-geometry-dir'] === undefined) {
      fail('input-invalid', 'the check profile has clash rules, so `base-geometry-dir` and `revised-geometry-dir` are required');
    }
    geometry = { base: readGeometryDir(args['base-geometry-dir'], 'base-geometry-dir'), revised: readGeometryDir(args['revised-geometry-dir'], 'revised-geometry-dir') };
    engine = await loadClashEngine();
  }
  return checkSnapshots({ base: sides.base, revised: sides.revised, profile, checkProfile, focus, digests, geometry, engine, coverage });
}

/** Single mode from files: only the later state, the comparison profile, the check profile and (optionally) its coverage. */
async function checkSingleFromFiles(args) {
  for (const key of ['revised-dir', 'profile-path', 'check-profile-path']) {
    if (typeof args[key] !== 'string' || args[key] === '') fail('input-invalid', `\`${key}\` is required, as a string in single mode`);
  }
  for (const key of ['base-dir', 'coverage-base-path', 'focus-path', 'base-geometry-dir', 'revised-geometry-dir']) {
    if (args[key] !== undefined) fail('input-invalid', `\`${key}\` is not accepted in single mode: only one model version is checked`);
  }
  if (args['coverage-revised-path'] !== undefined && (typeof args['coverage-revised-path'] !== 'string' || args['coverage-revised-path'] === '')) {
    fail('input-invalid', '`coverage-revised-path` must be a non-empty string when given');
  }
  const read = readSnapshotSide('revised', 'revised-dir', args['revised-dir']);
  const profile = readJsonFile(resolve(args['profile-path']), 'profile').value;
  const checkProfile = readJsonFile(resolve(args['check-profile-path']), 'check profile').value;
  const digests = { revised: read.digests, coverage: { base: null, revised: null } };
  let coverage = null;
  if (args['coverage-revised-path'] !== undefined) {
    const covered = readJsonFile(resolve(args['coverage-revised-path']), 'revised coverage');
    coverage = { base: null, revised: covered.value };
    digests.coverage.revised = covered.digest;
  }
  return checkSnapshots({ base: null, revised: read.side, profile, checkProfile, digests, coverage, mode: 'single' });
}

/**
 * `--validate-only`: validate a check profile (and, when given, check it against the comparison profile) and print the
 * evaluator echo, without reading any snapshot. Lets a caller learn what this build accepts and would echo before a review.
 */
function validateOnly(args) {
  if (typeof args['check-profile-path'] !== 'string' || args['check-profile-path'] === '') fail('input-invalid', '`check-profile-path` is required, as a string');
  if (args['profile-path'] !== undefined && (typeof args['profile-path'] !== 'string' || args['profile-path'] === '')) fail('input-invalid', '`profile-path` must be a non-empty string when given');
  const checkProfile = readJsonFile(resolve(args['check-profile-path']), 'check profile').value;
  if (args.mode !== undefined && !MODES.includes(args.mode)) fail('input-invalid', `\`mode\` must be one of ${MODES.join(', ')} when given`);
  if (args.mode === 'single') refuseClashRules(checkProfile);
  const checks = validateCheckProfile(checkProfile);
  if (args.mode === 'single') requireSingleModeProfile(checks);
  let comparisonChecked = false;
  if (args['profile-path'] !== undefined) {
    const compiled = validateProfile(readJsonFile(resolve(args['profile-path']), 'profile').value);
    if (checks.v2) checkRuleKeysAgainstComparison(checks, compiled);
    comparisonChecked = true;
  }
  return {
    schema: VALIDATION_SCHEMA,
    valid: true,
    evaluator: evaluatorEcho(checks.profile),
    checkProfile: { schema: checks.profile.schema, id: checks.profile.id, version: checks.profile.version },
    scope: checks.scopeMode,
    rules: checks.rules.length,
    comparisonProfileChecked: comparisonChecked,
    supports: {
      checkProfileSchemas: [CHECK_PROFILE_SCHEMA, CHECK_PROFILE_SCHEMA_V2],
      findingsSchemas: [FINDINGS_SCHEMA, FINDINGS_SCHEMA_V2],
      coverageSchema: COVERAGE_SCHEMA,
      modes: MODES,
      predicates: [...PREDICATES, ...V2_PREDICATES],
      notEvaluatedReasons: NOT_EVALUATED_REASONS,
    },
  };
}

/** The bridge entry point: returns the canonical JSON text to write on stdout. */
export async function runSnapshotCheckCommand(args) {
  return canonicalJson(await checkSnapshotsFromFiles(args));
}
