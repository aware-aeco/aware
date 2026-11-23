// Pair-mode clash scenarios whose canonical output was captured from the model-change-set 0.5.0 code (origin/main
// ff2e81683, before single-mode clash, `minOverlap`, per-rule pairs and the bounded broad phase existed) into
// test-fixtures/check-clash-pair-golden/. The current code must write exactly those bytes for a profile that uses none of
// the 0.6.0 additions (a v2 profile's `evaluator` block is the one allowed difference: its version, and the
// `evaluator.geometry` echo 0.6.0 adds). Every input below is built from numbers by snapshot-clash-fixtures.mjs, so the
// GLB tiles and their digests are the same bytes on every machine.
import { PROFILE } from './snapshot-compare-fixtures.mjs';
import { CHECK_PROFILE_SCHEMA_V2 } from './snapshot-check.mjs';
import { box, clashChecks, openCheck, world } from './snapshot-clash-fixtures.mjs';

const id = (guid) => `lineage-A:${guid}`;
const plate = (guid, z0, z1, extra = {}) => ({ guid, type: 'Contour plate', solid: box([0, 0, z0], [100, 100, z1]), ...extra });
const bolt = (guid, plies, extra = {}) => ({ guid, type: 'Bolt group', bolted: plies, ...extra });

function input(baseParts, revisedParts, { checks = clashChecks(), focus = null, hosts = [] } = {}) {
  const b = world(baseParts, { hosts });
  const r = world(revisedParts, { hosts });
  return { base: b.docs, revised: r.docs, profile: PROFILE, checkProfile: checks, focus, geometry: { base: b.tiles, revised: r.tiles } };
}

const v2 = (checks) => ({ ...checks, schema: CHECK_PROFILE_SCHEMA_V2, scope: 'changed' });

export const CASES = {
  // overlap, contact, a bolted stack, a focus with pair re-checks and an identity re-check
  busy: () => input(
    [plate('A', 0, 10), plate('B', 5, 15), plate('C', 40, 50), bolt('BG', ['A', 'C'])],
    [plate('A', 0, 10), plate('B', 2, 12), plate('C', 10, 20), plate('D', 20, 30), bolt('BG', ['A', 'C', 'D'])],
    { focus: { identities: [id('B')], sets: [[id('A'), id('B')], [id('A'), id('D')]] } },
  ),
  // a weld in its own joint, a bolt shaft through its plies and into another part, an unknown-parts weld in an assembly
  joints: () => input(
    [],
    [
      { guid: 'ASM', type: 'Assembly' }, plate('P1', 0, 10), plate('P2', 10, 20), plate('P3', 10, 20, { solid: box([200, 0, 10], [300, 100, 20]) }),
      { guid: 'W', type: 'Physical', solid: box([40, 40, 8], [60, 60, 12]), welded: ['P1'] },
      { guid: 'BG', type: 'Bolt group', solid: box([45, 45, 2], [55, 55, 8]), bolted: ['P1', 'P2'] },
      { guid: 'W2', type: 'Physical', solid: box([500, 500, 500], [510, 510, 510]) },
      plate('P4', 0, 10, { solid: box([200, 0, 0], [300, 100, 10]) }),
    ],
    { hosts: [['ASM', 'P3'], ['ASM', 'P4'], ['ASM', 'W2']] },
  ),
  // approximate and missing solids, a 3 mm bolted gap, and a 0.05 mm contact
  quality: () => input(
    [plate('A', 0, 10)],
    [
      plate('A', 0, 10), plate('B', 5, 15, { coverage: openCheck('operations-evaluated', 'cuts-not-applied') }),
      { guid: 'C', type: 'Beam', coverage: openCheck('geometry-reconstruction', 'part-geometry-missing') },
      plate('D', 10.05, 20.05, { solid: box([300, 0, 10.05], [400, 100, 20.05]) }), plate('E', 0, 10, { solid: box([300, 0, 0], [400, 100, 10]) }),
      plate('F', 23, 33, { solid: box([300, 0, 23.05], [400, 100, 33]) }), bolt('BG', ['D', 'F']),
    ],
  ),
  // a v2 profile running clash next to nothing else (scope 'changed')
  v2: () => input(
    [plate('A', 0, 10), plate('B', 50, 60)],
    [plate('A', 0, 10), plate('B', 5, 15), plate('C', 15, 25)],
    { checks: v2(clashChecks()) },
  ),
};
