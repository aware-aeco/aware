import assert from 'node:assert/strict';
import test from 'node:test';

import { canonicalJsonBytes } from './model-contract.mjs';
import { partitionMetadataRecords } from './model-metadata-shards.mjs';

const records = [
  { key: 'entity:001', record: { id: 'entity:001', name: 'Column' } },
  { key: 'entity:002', record: { id: 'entity:002', name: 'Beam' } },
  { key: 'entity:003', record: { id: 'entity:003', name: 'Bolt' } },
];

test('partitions ordered metadata at whole-record boundaries', () => {
  const single = partitionMetadataRecords('entities', records);
  const twoRecordBytes = canonicalJsonBytes({
    family: 'entities', records: records.slice(0, 2).map((entry) => entry.record),
    schemaVersion: 'aware.model-metadata-shard/v2',
  }).length;
  const split = partitionMetadataRecords('entities', records, {
    limits: { shardBytes: twoRecordBytes, shardRecords: 10 },
  });
  assert.equal(single.shards.length, 1);
  assert.equal(split.shards.length, 2);
  assert.deepEqual(split.shards.map((shard) => shard.receipt.itemCount), [2, 1]);
  assert.deepEqual(split.shards.map((shard) => shard.receipt.idRange), [
    { first: 'entity:001', last: 'entity:002' },
    { first: 'entity:003', last: 'entity:003' },
  ]);
  assert.equal(split.index.index.itemCount, 3);
  assert.equal(split.shards.every((shard) => shard.bytes.equals(canonicalJsonBytes(shard.value))), true);
  assert.equal(single.shards[0].bytes.toString('utf8'),
    '{"family":"entities","records":[{"id":"entity:001","name":"Column"},{"id":"entity:002","name":"Beam"},{"id":"entity:003","name":"Bolt"}],"schemaVersion":"aware.model-metadata-shard/v2"}');
});

test('partition bytes do not depend on caller object-key order', () => {
  const shuffledKeys = records.map((entry) => ({
    key: entry.key, record: { name: entry.record.name, id: entry.record.id },
  }));
  const built = partitionMetadataRecords('entities', shuffledKeys);
  assert.deepEqual(
    partitionMetadataRecords('entities', records).shards[0].bytes,
    built.shards[0].bytes,
  );
  shuffledKeys[0].record.id = 'mutated-after-build';
  assert.equal(built.shards[0].value.records[0].id, 'entity:001');
});

test('rejects duplicate, reversed, sparse and oversized records', () => {
  let laterRecordRead = false;
  const unreadLater = {
    get key() { laterRecordRead = true; throw new Error('must not read later records'); },
    record: { id: 'unreachable' },
  };
  assert.throws(
    () => partitionMetadataRecords('entities', [records[0], records[0], unreadLater]),
    (error) => error.code === 'reference-artifact-v2-duplicate',
  );
  assert.equal(laterRecordRead, false);
  assert.throws(
    () => partitionMetadataRecords('entities', [unreadLater]),
    (error) => error.code === 'reference-artifact-v2-invalid',
  );
  const throwingLimits = Object.defineProperty({}, 'shardBytes', {
    enumerable: true, get() { throw new Error('unreadable limit'); },
  });
  assert.throws(
    () => partitionMetadataRecords('entities', records, { limits: throwingLimits }),
    (error) => error.code === 'reference-artifact-v2-limit-invalid',
  );
  assert.throws(
    () => partitionMetadataRecords('entities', [records[1], records[0]]),
    (error) => error.code === 'reference-artifact-v2-order-invalid',
  );
  assert.throws(
    () => partitionMetadataRecords('entities', new Array(1)),
    (error) => error.code === 'reference-artifact-v2-invalid',
  );
  assert.throws(
    () => partitionMetadataRecords('entities', new Array(100_000_000)),
    (error) => error.code === 'reference-artifact-v2-invalid',
  );
  assert.throws(
    () => partitionMetadataRecords('entities', [records[0]], { limits: { shardBytes: 64 } }),
    (error) => error.code === 'reference-artifact-v2-limit',
  );
});

test('empty optional families produce empty canonical indexes', () => {
  for (const family of ['properties', 'relationships']) {
    const result = partitionMetadataRecords(family, [], { limits: { shardBytes: 64 } });
    assert.deepEqual(result.shards, []);
    assert.equal(result.index.index.itemCount, 0);
  }
  assert.throws(
    () => partitionMetadataRecords('entities', []),
    (error) => error.code === 'reference-artifact-v2-invalid',
  );
});

test('metadata nesting has one common admission limit', () => {
  let accepted = 'leaf';
  for (let depth = 0; depth < 128; depth += 1) accepted = { value: accepted };
  assert.equal(partitionMetadataRecords('entities', [{ key: 'entity:deep', record: accepted }])
    .shards[0].value.records.length, 1);
  const refused = { value: accepted };
  assert.throws(
    () => partitionMetadataRecords('entities', [{ key: 'entity:too-deep', record: refused }]),
    (error) => error.code === 'reference-artifact-v2-invalid',
  );
});
