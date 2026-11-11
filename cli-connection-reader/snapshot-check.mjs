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

export const FINDINGS_SCHEMA = 'aware.model-change-findings/v1';
export const CHECK_PROFILE_SCHEMA = 'aware.model-check-profile/v1';
const SEVERITIES = ['warning', 'error', 'info'];
const PREDICATES = ['blank'];
const TARGETS = ['entity.name'];
const DIRECTIONS = ['higher-is-worse', 'lower-is-worse'];
const digest = (value) => sha256(Buffer.from(canonicalJson(value), 'utf8'));

// ─── check profile + focus ──────────────────────────────────────────────────────────────────────────

const nonEmptyString = (v) => typeof v === 'string' && v !== '';

/** Strict validation: unknown or missing keys are rejected, as are predicates and targets this build does not know. */
export function validateCheckProfile(profile) {
  exactKeys(profile, ['schema', 'id', 'version', 'budgets', 'rules'], 'check profile');
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
    return { ...rule, types: new Set(rule.appliesTo.types), worsening };
  });
  return { profile, rules };
}

/** Exactly `{ identities: [string...] }`, no duplicates. */
export function validateFocus(focus) {
  if (!isObject(focus) || Object.keys(focus).length !== 1 || !Array.isArray(focus.identities)) fail('focus-invalid', 'focus must be exactly { identities: [...] }');
  const seen = new Set();
  for (const [index, identity] of focus.identities.entries()) {
    if (!nonEmptyString(identity)) fail('focus-invalid', `focus.identities[${index}] must be a non-empty string`);
    if (seen.has(identity)) fail('focus-invalid', `focus.identities names '${identity}' twice`);
    seen.add(identity);
  }
  return [...focus.identities].sort(codeUnitCompare);
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

function capped(list, cap) {
  return list.length > cap ? { items: list.slice(0, cap), omitted: list.length - cap } : { items: list, omitted: 0 };
}

/**
 * @param {{ base, revised, profile, checkProfile, focus?: object|null, digests?: object }} input
 */
export function checkSnapshots({ base, revised, profile: rawProfile, checkProfile: rawCheckProfile, focus = null, digests = {} }) {
  const checks = validateCheckProfile(rawCheckProfile);
  const focusIdentities = focus === null ? [] : validateFocus(focus);
  const pairing = pairSnapshots({ base, revised, profile: rawProfile });
  const { compiled, baseSide, revisedSide, addedIdentities, pairedIdentities } = pairing;

  // Scope: every added object, every changed pair, every named identity that is present in the revised state.
  const changed = new Set();
  const pairInfo = new Map();
  for (const pair of diffPairs(pairing)) {
    pairInfo.set(pair.identity, pair);
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
  const findingFor = (rule, identity, entity) => {
    if (!rule.types.has(entity.type)) return null;
    const hit = evaluate(rule, entity);
    if (!hit) return null;
    const correspondenceKey = correspondenceKeyOf(rule.id, [identity], hit.locus);
    return { correspondenceKey, locus: hit.locus, evidence: hit.evidence, evidenceFingerprint: fingerprintOf(correspondenceKey, rule.version, hit.evidence) };
  };

  const byRule = Object.fromEntries(checks.rules.map((rule) => [rule.id, { evaluated: 0, findings: 0, notApplicable: 0 }]));
  const findings = [];
  const resolved = [];
  const stateOf = new Map();   // `${identity}\u0000${ruleId}` -> 'finding' | 'passed' | 'not-applicable'
  let evaluated = 0;
  let notApplicable = 0;

  for (const identity of scopeList) {
    const { revisedId, revisedEntity, baseId, baseEntity } = objectFor(identity);
    const participants = [{
      identity, entityIdBase: baseId, entityIdRevised: revisedId,
      type: revisedEntity.type ?? null, name: revisedEntity.name ?? null,
    }];
    for (const rule of checks.rules) {
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
    for (const rule of [...checks.rules].sort((a, b) => codeUnitCompare(a.id, b.id))) {
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
  const complete = omitted.findings + omitted.resolvedInChange + omitted.focusResults === 0;

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
  if (!complete) {
    unchecked.push(`Partial list: ${omitted.findings} finding(s), ${omitted.resolvedInChange} resolved finding(s) and ${omitted.focusResults} re-check row(s) are not listed (the lists are limited by the check profile's budgets). Counts are complete.`);
  }

  return {
    schema: FINDINGS_SCHEMA,
    inputs: {
      base: digests.base ?? null, revised: digests.revised ?? null,
      profileDigest: digest(compiled.profile),
      checkProfileDigest: digest(checks.profile),
      focusDigest: focus === null ? null : digest(focus),
    },
    checkProfile: { id: checks.profile.id, version: checks.profile.version },
    scope: {
      added: addedIdentities.length, changed: changed.size, focus: focusPresent.length,
      evaluated, notApplicable, excluded: excludedCount, byRule,
    },
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
}

// ─── file I/O wrapper (the CLI path) ────────────────────────────────────────────────────────────────

export function checkSnapshotsFromFiles(args) {
  for (const key of ['base-dir', 'revised-dir', 'profile-path', 'check-profile-path']) {
    if (typeof args[key] !== 'string' || args[key] === '') fail('input-invalid', `\`${key}\` is required, as a string`);
  }
  if (args['focus-path'] !== undefined && (typeof args['focus-path'] !== 'string' || args['focus-path'] === '')) fail('input-invalid', '`focus-path` must be a non-empty string when given');
  const { sides, digests } = readSnapshotSides(args);
  const profile = readJsonFile(resolve(args['profile-path']), 'profile').value;
  const checkProfile = readJsonFile(resolve(args['check-profile-path']), 'check profile').value;
  const focus = args['focus-path'] === undefined ? null : readJsonFile(resolve(args['focus-path']), 'focus').value;
  return checkSnapshots({ base: sides.base, revised: sides.revised, profile, checkProfile, focus, digests });
}

/** The bridge entry point: returns the canonical JSON text to write on stdout. */
export function runSnapshotCheckCommand(args) {
  return canonicalJson(checkSnapshotsFromFiles(args));
}
