// Shared fixture for the compare-snapshots tests: a Tekla-shaped pinned profile. It is plain DATA - AWARE has no
// Tekla knowledge, and nothing in snapshot-compare.mjs reads these keys except as the profile instructs.
import { PROFILE_SCHEMA } from './snapshot-compare.mjs';

export const GUID_KEY = 'native:guid';

/** A Tekla-shaped pinned profile. Plain data: AWARE has no Tekla knowledge. */
export const PROFILE = {
  schema: PROFILE_SCHEMA,
  id: 'test.tekla',
  version: '1.0.0',
  identity: { sourceKey: GUID_KEY, namespace: 'lineage-A' },
  ignoreRowIdPrefixes: ['viewer:'],
  volatile: ['native:id'],
  keyRules: [{ match: '^uda:(text|number):(0|[1-9][0-9]*)$', key: 'uda:$1:name:{name|nfkc-lower}' }],
  kinds: {
    'native:profile': 'shape-fields', 'native:shape': 'shape-fields', 'native:length': 'shape-fields',
    'uda:*': 'properties', 'native:material': 'properties',
  },
  geometry: { field: 'bounds', toleranceMm: 1 },
  discontinuityThreshold: 0.5,
  budgets: { changes: 5000, groupIds: 200, receiptsPerKind: 2000, outputBytes: 32 * 1024 * 1024 },
  uncheckedStatements: ['Tekla component internals were not compared.'],
};
