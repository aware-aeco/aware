import assert from 'node:assert/strict';
import test from 'node:test';
import { makeGlbFixture, makeMetadataFixture } from './model-fixtures.mjs';

test('generated GLB fixtures are byte deterministic and carry no committed binary fixture', () => {
  const options = { positions: [[0, 0, 0], [1, 0, 0], [0, 1, 0]], indices: [0, 1, 2], nodeName: 'part-a' };
  const first = makeGlbFixture(options);
  const second = makeGlbFixture({ indices: [0, 1, 2], nodeName: 'part-a', positions: options.positions });
  assert.deepEqual(first, second);
  assert.equal(first.subarray(0, 4).toString('ascii'), 'glTF');
});

test('generated metadata uses decimal-string identities and explicit appearance joins', () => {
  const metadata = makeMetadataFixture({ elementId: '9223372036854775806', nodeNames: ['part-a', 'part-b'] });
  assert.equal(metadata.elements[0].id, '9223372036854775806');
  assert.deepEqual(metadata.elements[0].appearances, ['part-a', 'part-b']);
  assert.equal(metadata.parameters[0].storageType, 'string');
});

