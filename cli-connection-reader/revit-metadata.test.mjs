import assert from 'node:assert/strict';
import test from 'node:test';
import { makeMetadataFixture } from './model-fixtures.mjs';
import { normalizeRevitMetadata } from './revit-metadata.mjs';

const geometry = [
  { nodeName: 'part-a', primitiveOrdinal: 0, positions: [[0, 0, 0], [10, 0, 0], [0, 10, 0]], triangles: [[0, 1, 2]] },
  { nodeName: 'part-b', primitiveOrdinal: 0, positions: [[20, 0, 0], [30, 0, 0], [20, 10, 0]], triangles: [[0, 1, 2]] },
];

test('explicit indexed metadata resolves to stable namespaced identity and multipart geometry', () => {
  const result = normalizeRevitMetadata(makeMetadataFixture({ elementId: '9223372036854775806', nodeNames: ['part-a', 'part-b'] }), geometry);
  assert.equal(result.entities[0].id, 'element:9223372036854775806');
  assert.equal(result.entities[0].typeId, 'type:2001');
  assert.equal(result.entities[0].levelId, 'level:3001');
  assert.deepEqual(result.entities[0].geometry.map((item) => item.nodeName), ['part-a', 'part-b']);
  assert.equal(result.properties[0].storageType, 'string');
  assert.equal(result.properties[0].value, '1FixtureGuid00000000000');
  assert.equal(result.entities[0].ifcGuid, '1FixtureGuid00000000000');
});

test('parameter storage types preserve signed element ids, null, empty, boolean and finite doubles', () => {
  const metadata = makeMetadataFixture();
  metadata.parameters = [
    { id: '1', name: 'None', unit: null, readable: false, storageType: 'none', value: null },
    { id: '2', name: 'Bool', unit: null, readable: true, storageType: 'boolean', value: true },
    { id: '3', name: 'Integer', unit: null, readable: true, storageType: 'integer', value: '-12' },
    { id: '4', name: 'Double', unit: 'ft', readable: true, storageType: 'double', value: 1.25 },
    { id: '5', name: 'String', unit: null, readable: true, storageType: 'string', value: '' },
    { id: '6', name: 'Element', unit: null, readable: true, storageType: 'element-id', value: '-2000011' },
  ];
  metadata.parameterGroups[0].parameters = [0, 1, 2, 3, 4, 5];
  delete metadata.elements[0].ifcGuid;
  const result = normalizeRevitMetadata(metadata, geometry.slice(0, 1));
  assert.deepEqual(result.properties.map((row) => row.value), [null, true, '-12', 1.25, '', '-2000011']);
});

test('explicit relations validate endpoints, provider kinds, acyclic parents, and canonical order', () => {
  const metadata = makeMetadataFixture({ elementId: '2', nodeNames: ['part-b'] });
  const firstElement = { ...metadata.elements[0], id: '1', appearances: ['part-a'] };
  delete firstElement.ifcGuid;
  metadata.elements.unshift(firstElement);
  metadata.relations = [
    { id: '11', kind: 'provider-explicit', providerRelationKind: 'Joins', from: '2', to: '1' },
    { id: '10', kind: 'contains', from: '1', to: '2' },
  ];
  const result = normalizeRevitMetadata(metadata, geometry);
  assert.deepEqual(result.relationships.map((edge) => edge.id), ['relation:10', 'relation:11']);
  assert.equal(result.relationships[1].providerRelationKind, 'Joins');
  metadata.relations.push({ id: '12', kind: 'contains', from: '2', to: '1' });
  assert.throws(() => normalizeRevitMetadata(metadata, geometry), /cycle/);
});

test('ambiguous, missing, and duplicate appearance ownership is refused without name inference', () => {
  const duplicate = makeMetadataFixture({ nodeNames: ['part-a'] });
  const duplicateOwner = { ...duplicate.elements[0], id: '1002', appearances: ['part-a'] };
  delete duplicateOwner.ifcGuid;
  duplicate.elements.push(duplicateOwner);
  assert.throws(() => normalizeRevitMetadata(duplicate, geometry.slice(0, 1)), /more than one entity/);
  assert.throws(() => normalizeRevitMetadata(makeMetadataFixture({ nodeNames: ['1001_0'] }), geometry), /does not resolve/);
  const numeric = makeMetadataFixture();
  numeric.elements[0].id = 9007199254740992;
  assert.throws(() => normalizeRevitMetadata(numeric, geometry.slice(0, 1)), /decimal string/);
});

test('set-like element permutations produce identical canonical artifact bytes and exact coverage', () => {
  const metadata = makeMetadataFixture({ elementId: '2', nodeNames: ['part-b'] });
  const firstElement = { ...metadata.elements[0], id: '1', appearances: ['part-a'] };
  delete firstElement.ifcGuid;
  metadata.elements.push(firstElement);
  const first = normalizeRevitMetadata(metadata, geometry);
  const second = normalizeRevitMetadata({ ...metadata, elements: [...metadata.elements].reverse() }, [...geometry].reverse());
  assert.deepEqual(first.entitiesBytes, second.entitiesBytes);
  assert.deepEqual(first.propertiesBytes, second.propertiesBytes);
  assert.deepEqual(first.relationshipsBytes, second.relationshipsBytes);
  assert.equal(first.coverage.discoveredEntities, 2);
  assert.equal(first.coverage.drawableEntities, 2);
  assert.equal(first.coverage.unclaimedGeometryNodes.length, 0);
});

test('duplicate authoritative IfcGUID values make every matching entity uncomparable', () => {
  const metadata = makeMetadataFixture({ elementId: '2', nodeNames: ['part-b'] });
  const firstElement = { ...metadata.elements[0], id: '1', appearances: ['part-a'] };
  delete firstElement.ifcGuid;
  metadata.elements.push(firstElement);
  const result = normalizeRevitMetadata(metadata, geometry);
  assert.deepEqual(result.entities.map((entity) => entity.ifcGuid), [null, null]);
});

test('long valid relationship chains are checked without recursive stack overflow', () => {
  const count = 15_000;
  const metadata = {
    schemaVersion: '1',
    document: { kind: 'revit-project', id: 'long-chain' },
    types: [], levels: [], parameterGroups: [], parameters: [],
    elements: Array.from({ length: count }, (_, index) => ({
      id: String(index + 1), revitClass: null, category: null, family: null,
      type: null, level: null, parameterGroups: [], appearances: [`node-${index + 1}`],
    })),
    relations: Array.from({ length: count - 1 }, (_, index) => ({
      id: String(index + 1), kind: 'contains', from: String(index + 1), to: String(index + 2),
    })),
  };
  const geometry = Array.from({ length: count }, (_, index) => ({
    nodeName: `node-${index + 1}`, primitiveOrdinal: 0, positions: [],
  }));
  const result = normalizeRevitMetadata(metadata, geometry);
  assert.equal(result.relationships.length, count - 1);
});
