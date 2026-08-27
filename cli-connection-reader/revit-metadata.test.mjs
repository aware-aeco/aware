import assert from 'node:assert/strict';
import test from 'node:test';
import { makeMetadataFixture } from './model-fixtures.mjs';
import { normalizeRevitMetadata } from './revit-metadata.mjs';

const geometry = [
  { nodeName: 'part-a', primitiveOrdinal: 0, positions: [[0, 0, 0], [10, 0, 0], [0, 10, 0]], triangles: [[0, 1, 2]] },
  { nodeName: 'part-b', primitiveOrdinal: 0, positions: [[20, 0, 0], [30, 0, 0], [20, 10, 0]], triangles: [[0, 1, 2]] },
];

function makeMetadataV2() {
  const metadata = makeMetadataFixture();
  metadata.schemaVersion = '2';
  metadata.parameterGroups[0].id = '1';
  metadata.parameters = [
    { id: '1', name: 'IfcGUID', unit: null, valueEncoding: 'provider-display', valueType: 'string', value: 'display-only-guid' },
    { id: '2', name: 'Length', unit: 'mm', valueEncoding: 'provider-display', valueType: 'number', value: -0 },
  ];
  metadata.parameterGroups[0].parameters = [0, 1];
  delete metadata.elements[0].ifcGuid;
  return metadata;
}

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

test('v2 preserves provider-display provenance, normalizes negative zero, and never derives IfcGUID identity', () => {
  const result = normalizeRevitMetadata(makeMetadataV2(), geometry.slice(0, 1));
  assert.equal(JSON.parse(result.propertiesBytes).schemaVersion, '2');
  assert.deepEqual(result.properties.map((row) => ({
    valueEncoding: row.valueEncoding, valueType: row.valueType, value: row.value, unit: row.unit,
  })), [
    { valueEncoding: 'provider-display', valueType: 'string', value: 'display-only-guid', unit: null },
    { valueEncoding: 'provider-display', valueType: 'number', value: 0, unit: 'mm' },
  ]);
  assert.equal(result.entities[0].ifcGuid, null);
  assert.deepEqual({
    metadataSchemaVersion: result.coverage.metadataSchemaVersion,
    nativeParameterGroups: result.coverage.nativeParameterGroups,
    nativeParameters: result.coverage.nativeParameters,
    elementGroupReferences: result.coverage.elementGroupReferences,
    expandedProperties: result.coverage.expandedProperties,
    orphanParameterGroups: result.coverage.orphanParameterGroups,
    orphanParameters: result.coverage.orphanParameters,
    canonicalPropertyBytes: result.coverage.canonicalPropertyBytes,
    effectivePropertyLimits: result.coverage.effectivePropertyLimits,
  }, {
    metadataSchemaVersion: '2', nativeParameterGroups: 1, nativeParameters: 2,
    elementGroupReferences: 1, expandedProperties: 2, orphanParameterGroups: 0, orphanParameters: 0,
    canonicalPropertyBytes: result.propertiesBytes.length,
    effectivePropertyLimits: { maxExpandedPropertyRows: 100_000, maxCanonicalPropertyBytes: 16 * 1024 * 1024 },
  });
});

test('v2 source-storage rows retain authoritative semantics and alone may supply IfcGUID identity', () => {
  const metadata = makeMetadataV2();
  metadata.parameters[0] = {
    id: '1', name: 'IfcGUID', unit: null, valueEncoding: 'source-storage',
    readable: true, storageType: 'string', value: 'authoritative-guid',
  };
  const result = normalizeRevitMetadata(metadata, geometry.slice(0, 1));
  assert.equal(result.entities[0].ifcGuid, 'authoritative-guid');
  assert.deepEqual(result.properties[0], {
    entityId: 'element:1001', groupId: 'parameter-group:1', groupName: 'Identity Data', groupOrdinal: 0,
    parameterId: 'parameter:1', parameterOrdinal: 0, name: 'IfcGUID', unit: null,
    valueEncoding: 'source-storage', readable: true, storageType: 'string', value: 'authoritative-guid',
  });
});

test('v2 rejects duplicate references and reports unreachable table rows without expanding them', () => {
  const duplicateGroup = makeMetadataV2();
  duplicateGroup.elements[0].parameterGroups = [0, 0];
  assert.throws(() => normalizeRevitMetadata(duplicateGroup, geometry.slice(0, 1)), /duplicate references/);

  const duplicateParameter = makeMetadataV2();
  duplicateParameter.parameterGroups[0].parameters = [0, 0];
  assert.throws(() => normalizeRevitMetadata(duplicateParameter, geometry.slice(0, 1)), /duplicate references/);

  const orphaned = makeMetadataV2();
  orphaned.parameterGroups.push({ id: '2', name: 'Unused group', parameters: [] });
  orphaned.parameters.push({ id: '3', name: 'Unused', unit: null, valueEncoding: 'provider-display', valueType: 'string', value: 'unused' });
  const result = normalizeRevitMetadata(orphaned, geometry.slice(0, 1));
  assert.equal(result.coverage.orphanParameterGroups, 1);
  assert.equal(result.coverage.orphanParameters, 1);
  assert.equal(result.properties.length, 2);
});

test('v2 enforces independent pre-append row and canonical property byte ceilings', () => {
  const metadata = makeMetadataV2();
  assert.throws(() => normalizeRevitMetadata(metadata, geometry.slice(0, 1), {
    propertyExpansionLimits: { maxExpandedPropertyRows: 1 },
  }), (error) => error.code === 'reference-output-too-large');
  assert.throws(() => normalizeRevitMetadata(metadata, geometry.slice(0, 1), {
    propertyExpansionLimits: { maxCanonicalPropertyBytes: 64 },
  }), (error) => error.code === 'reference-output-too-large');
  assert.throws(() => normalizeRevitMetadata(metadata, geometry.slice(0, 1), {
    propertyExpansionLimits: { maxExpandedPropertyRows: 2_000_001 },
  }), /hard ceiling/);
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

test('directed multigraph relationship kinds preserve parallel edges with distinct ids', () => {
  const metadata = makeMetadataFixture();
  metadata.relations = [
    { id: '10', kind: 'depends-on', from: '1001', to: '1001' },
    { id: '11', kind: 'depends-on', from: '1001', to: '1001' },
  ];
  const result = normalizeRevitMetadata(metadata, geometry.slice(0, 1));
  assert.deepEqual(result.relationships.map((edge) => edge.id), ['relation:10', 'relation:11']);
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

test('explicit non-drawable elements remain indexed with empty geometry', () => {
  const metadata = makeMetadataFixture();
  const nonDrawable = { ...metadata.elements[0], id: '1002', appearances: [] };
  delete nonDrawable.ifcGuid;
  metadata.elements.push(nonDrawable);
  const result = normalizeRevitMetadata(metadata, geometry.slice(0, 1));
  assert.equal(result.coverage.discoveredEntities, 2);
  assert.equal(result.coverage.indexedEntities, 2);
  assert.equal(result.coverage.drawableEntities, 1);
  assert.deepEqual(result.entities.find((entity) => entity.id === 'element:1002').geometry, []);
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

test('aggregate property expansion is bounded before repeated references allocate rows', () => {
  const metadata = makeMetadataFixture();
  metadata.parameterGroups[0].parameters = [0, 0];
  metadata.elements[0].parameterGroups = [0, 0];
  assert.throws(
    () => normalizeRevitMetadata(metadata, geometry.slice(0, 1), { limits: { maxParameters: 3 } }),
    (error) => error.code === 'reference-output-too-large',
  );
});
