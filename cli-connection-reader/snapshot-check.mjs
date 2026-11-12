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
  canonicalJson, codeUnitCompare, diffPairs, exactKeys, fail, isCount, isObject, pairSnapshots, readJsonFile,
  readSnapshotSides, sha256,
} from './snapshot-compare.mjs';
import {
  CLASH_PREDICATES, CLASH_TARGET, evaluateClash, loadClashEngine, normalizeGeometrySide, readGeometryDir,
  validateClashRule, validateGeometryBlock,
} from './snapshot-clash.mjs';

export const FINDINGS_SCHEMA = 'aware.model-change-findings/v1';
export const CHECK_PROFILE_SCHEMA = 'aware.model-check-profile/v1';
const SEVERITIES = ['warning', 'error', 'info'];
const PREDICATES = ['blank', ...CLASH_PREDICATES];
const TARGETS = ['entity.name', CLASH_TARGET];
const DIRECTIONS = ['higher-is-worse', 'lower-is-worse'];
const digest = (value) => sha256(Buffer.from(canonicalJson(value), 'utf8'));

// ─── check profile + focus ──────────────────────────────────────────────────────────────────────────

const nonEmptyString = (v) => typeof v === 'string' && v !== '';

/** Strict validation: unknown or missing keys are rejected, as are predicates and targets this build does not know. */
export function validateCheckProfile(profile) {
  exactKeys(profile, ['schema', 'id', 'version', 'budgets', 'rules'], 'check profile', ['geometry']);
  if (profile.schema !== CHECK_PROFILE_SCHEMA) fail('profile-invalid', `check profile.schema must be '${CHECK_PROFILE_SCHEMA}'`);
  for (const key of ['id', 'version']) if (!nonEmptyString(profile[key])) fail('profile-invalid', `check profile.${key} must be a non-empty string`);
  exactKeys(profile.budgets, ['findings', 'focusResults'], 'check profile.budgets');
  for (const key of ['findings', 'focusResults']) {
    if (!isCount(profile.budgets[key])) fail('profile-invalid', `check profile.budgets.${key} must be a non-negative integer`);
  }
  if (!Array.isArray(profile.rules)) fail('profile-invalid', 'check profile.rules must be an array');
  const seen = new Set();
  const rules = profile.rules.map((rule, index) => {
    const label = `check profile.rules[${index}]`;
    exactKeys(rule, ['id', 'version', 'severity', 'predicate', 'target', 'appliesTo', 'worsening'], label);
    for (const key of ['id', 'version']) if (!nonEmptyString(rule[key])) fail('profile-invalid', `${label}.${key} must be a non-empty string`);
    if (seen.has(rule.id)) fail('profile-invalid', `${label}.id '${rule.id}' is used by two rules`);
    seen.add(rule.id);
    if (!SEVERITIES.includes(rule.severity)) fail('profile-invalid', `${label}.severity must be one of ${SEVERITIES.join(', ')}`);
    if (!PREDICATES.includes(rule.predicate)) fail('profile-invalid', `${label}.predicate '${String(rule.predicate)}' is not known (expected: ${PREDICATES.join(', ')})`);
    if (!TARGETS.includes(rule.target)) fail('profile-invalid', `${label}.target '${String(rule.target)}' is not known (expected: ${TARGETS.join(', ')})`);
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
    if (rule.predicate === 'blank') {
      if (rule.target !== 'entity.name') fail('profile-invalid', `${label}.target must be 'entity.name' for the 'blank' predicate`);
    } else validateClashRule({ ...rule, worsening }, label);
    return { ...rule, types: new Set(rule.appliesTo.types), worsening };
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
  return { profile, rules, geo, clashRules };
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
 *           geometry?: { base: { tiles }, revised: { tiles } }, engine?: object }} input
 *   `geometry` and `engine` are needed only by a check profile with clash rules (`engine` from `loadClashEngine()`).
 */
export function checkSnapshots({ base, revised, profile: rawProfile, checkProfile: rawCheckProfile, focus = null, digests = {}, geometry = null, engine = null }) {
  const checks = validateCheckProfile(rawCheckProfile);
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

  // Scope: every added object, every changed pair, every named identity that is present in the revised state.
  const changed = new Set();
  for (const pair of diffPairs(pairing)) {
    if (pair.fields.length > 0) changed.add(pair.identity);
  }
  const focusPresent = focusIdentities.filter((identity) => revisedSide.pairable.has(identity));
  const scope = new Set([...addedIdentities, ...changed, ...focusPresent]);
  const scopeList = [...scope].sort(codeUnitCompare);

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
  const findingFor = (rule, identity, entity) => {
    if (!rule.types.has(entity.type)) return null;
    const hit = evaluate(rule, entity);
    if (!hit) return null;
    const correspondenceKey = correspondenceKeyOf(rule.id, [identity], hit.locus);
    return { correspondenceKey, locus: hit.locus, evidence: hit.evidence, evidenceFingerprint: fingerprintOf(correspondenceKey, rule.version, hit.evidence) };
  };

  // The one-object rules run per scope object; clash rules are pair rules and run below.
  const singleRules = checks.rules.filter((rule) => rule.predicate === 'blank');
  const byRule = Object.fromEntries(checks.rules.map((rule) => [rule.id, { evaluated: 0, findings: 0, notApplicable: 0 }]));
  const findings = [];
  const resolved = [];
  const stateOf = new Map();   // `${identity}\u0000${ruleId}` -> 'finding' | 'passed' | 'not-applicable'
  let evaluated = 0;
  let notApplicable = 0;

  for (const identity of scopeList) {
    const { revisedEntity, baseEntity } = objectFor(identity);
    const participants = [participantOf(identity)];
    for (const rule of singleRules) {
      if (!rule.types.has(revisedEntity.type)) {
        notApplicable += 1;
        byRule[rule.id].notApplicable += 1;
        stateOf.set(`${identity}\u0000${rule.id}`, 'not-applicable');
      } else {
        evaluated += 1;
        byRule[rule.id].evaluated += 1;
      }
      const now = findingFor(rule, identity, revisedEntity);
      const before = baseEntity ? findingFor(rule, identity, baseEntity) : null;
      if (now) {
        let presence = 'introduced';
        if (before) presence = before.evidenceFingerprint === now.evidenceFingerprint ? 'pre-existing' : presenceOfChanged(rule, before.evidence, now.evidence);
        byRule[rule.id].findings += 1;
        stateOf.set(`${identity}\u0000${rule.id}`, 'finding');
        findings.push({
          ruleId: rule.id, ruleVersion: rule.version, severity: rule.severity,
          correspondenceKey: now.correspondenceKey, participants, locus: now.locus,
          evidence: now.evidence, evidenceFingerprint: now.evidenceFingerprint,
          presence, baseEvidenceFingerprint: before ? before.evidenceFingerprint : null,
        });
      } else {
        if (rule.types.has(revisedEntity.type)) stateOf.set(`${identity}\u0000${rule.id}`, 'passed');
        // Resolved by the change only when the rule still applies and now passes: an object the rule no longer
        // applies to (its type changed) was not shown to be fixed, so it is not listed as resolved.
        if (before && rule.types.has(revisedEntity.type)) {
          resolved.push({
            ruleId: rule.id, ruleVersion: rule.version, correspondenceKey: before.correspondenceKey, participants, locus: before.locus,
            evidence: before.evidence, evidenceFingerprint: before.evidenceFingerprint,
          });
        }
      }
    }
  }

  // Clash: pair rules over the solids of what changed and its dependency closure (plan 2026-11-12 S3).
  let clash = null;
  if (clashRules) {
    const clashScope = new Set(scope);
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

  const summaryByRule = Object.fromEntries(checks.rules.map((rule) => [rule.id, byRule[rule.id].findings]));
  // Two different reasons an object cannot be checked. Pairing puts a revised object in `duplicate` both when its
  // identity is used twice in the later state AND when the earlier state uses it twice (so it cannot be matched).
  const earlierTwice = revisedSide.droppedIdentities.size;
  const laterTwice = revisedSide.duplicate.length - earlierTwice;
  const excludedCount = revisedSide.blank.length + revisedSide.duplicate.length;
  const unchecked = [
    'Checks ran on added and changed objects, and on objects named for a re-check, only; unchanged objects were not checked.',
    `Only the ${checks.rules.length} rule(s) of check profile '${checks.profile.id}' were run; anything they do not test was not checked.`,
  ];
  if (excludedCount > 0) {
    unchecked.push(`${excludedCount} object(s) in the later state were not checked: their identity was blank (${revisedSide.blank.length}), used twice in the later state (${laterTwice}), or used twice in the earlier state so they cannot be matched (${earlierTwice}).`);
  }
  if (clash) unchecked.push(...clash.unchecked);
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
  if (clash) { scopeOut.notEvaluated = clash.scope.notEvaluated; scopeOut.solids = clash.scope.solids; }
  const out = {
    schema: FINDINGS_SCHEMA,
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
  return out;
}

// ─── file I/O wrapper (the CLI path) ────────────────────────────────────────────────────────────────

export async function checkSnapshotsFromFiles(args) {
  for (const key of ['base-dir', 'revised-dir', 'profile-path', 'check-profile-path']) {
    if (typeof args[key] !== 'string' || args[key] === '') fail('input-invalid', `\`${key}\` is required, as a string`);
  }
  if (args['focus-path'] !== undefined && (typeof args['focus-path'] !== 'string' || args['focus-path'] === '')) fail('input-invalid', '`focus-path` must be a non-empty string when given');
  for (const key of ['base-geometry-dir', 'revised-geometry-dir']) {
    if (args[key] !== undefined && (typeof args[key] !== 'string' || args[key] === '')) fail('input-invalid', `\`${key}\` must be a non-empty string when given`);
  }
  const { sides, digests } = readSnapshotSides(args);
  const profile = readJsonFile(resolve(args['profile-path']), 'profile').value;
  const checkProfile = readJsonFile(resolve(args['check-profile-path']), 'check profile').value;
  const focus = args['focus-path'] === undefined ? null : readJsonFile(resolve(args['focus-path']), 'focus').value;
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
  return checkSnapshots({ base: sides.base, revised: sides.revised, profile, checkProfile, focus, digests, geometry, engine });
}

/** The bridge entry point: returns the canonical JSON text to write on stdout. */
export async function runSnapshotCheckCommand(args) {
  return canonicalJson(await checkSnapshotsFromFiles(args));
}
