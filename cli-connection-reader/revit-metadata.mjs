import {
  assertClosedObject, canonicalJsonBytes, lowerableLimits, lowerablePropertyExpansionLimits,
  ModelReaderError, parseJsonStrict, sha256,
} from './model-contract.mjs';

const POSITIVE_INT64 = /^(?:[1-9]\d*)$/;
const SIGNED_INT64 = /^(?:0|-?[1-9]\d*)$/;
const MAX_INT64 = 9223372036854775807n;
const MIN_INT64 = -9223372036854775808n;

function invalid(message, code = 'reference-metadata-invalid') {
  throw new ModelReaderError(code, 'normalize-metadata', false, message);
}

function closed(value, required, optional, label) {
  try { return assertClosedObject(value, required, optional, label); }
  catch (error) { invalid(error instanceof Error ? error.message : `${label} is invalid`); }
}

function positiveId(value, label) {
  if (typeof value !== 'string' || !POSITIVE_INT64.test(value)) invalid(`${label} must be a positive int64 decimal string`);
  try { if (BigInt(value) > MAX_INT64) invalid(`${label} exceeds int64`); }
  catch { invalid(`${label} must be a positive int64 decimal string`); }
  return value;
}

function signedId(value, label) {
  if (typeof value !== 'string' || !SIGNED_INT64.test(value)) invalid(`${label} must be a signed int64 decimal string`);
  try {
    const parsed = BigInt(value);
    if (parsed < MIN_INT64 || parsed > MAX_INT64) invalid(`${label} exceeds int64`);
  } catch { invalid(`${label} must be a signed int64 decimal string`); }
  return value;
}

function text(value, label, nullable = false) {
  if (nullable && value === null) return null;
  if (typeof value !== 'string') invalid(`${label} must be ${nullable ? 'a string or null' : 'a string'}`);
  return value;
}

function list(value, label, limit) {
  if (!Array.isArray(value) || value.length > limit) invalid(`${label} is missing or exceeds its count limit`);
  return value;
}

function uniqueTable(records, label, limit, allowed = []) {
  const ids = new Set();
  return list(records, label, limit).map((record, index) => {
    closed(record, ['id', 'name'], allowed, `${label}[${index}]`);
    const id = positiveId(record.id, `${label}[${index}].id`);
    if (ids.has(id)) invalid(`${label} contains duplicate id ${id}`);
    ids.add(id);
    return { ...record, id, name: text(record.name, `${label}[${index}].name`) };
  });
}

function tableIndex(value, table, label, nullable = false) {
  if (nullable && value === null) return null;
  if (!Number.isSafeInteger(value) || value < 0 || value >= table.length) invalid(`${label} index is out of range`);
  return table[value];
}

function validateSourceStorageParameter(parameter, index, tagged = false) {
  const required = tagged
    ? ['id', 'name', 'unit', 'valueEncoding', 'readable', 'storageType', 'value']
    : ['id', 'name', 'unit', 'readable', 'storageType', 'value'];
  closed(parameter, required, [], `parameters[${index}]`);
  if (tagged && parameter.valueEncoding !== 'source-storage') invalid(`parameters[${index}].valueEncoding must be source-storage`);
  const id = positiveId(parameter.id, `parameters[${index}].id`);
  const name = text(parameter.name, `parameters[${index}].name`);
  const unit = text(parameter.unit, `parameters[${index}].unit`, true);
  if (typeof parameter.readable !== 'boolean') invalid(`parameters[${index}].readable must be boolean`);
  const storageType = parameter.storageType;
  let value = parameter.value;
  if (storageType === 'none') {
    if (value !== null || parameter.readable !== false) invalid('none parameter must be unreadable null');
  } else if (storageType === 'boolean') {
    if (typeof value !== 'boolean' || !parameter.readable) invalid('boolean parameter has the wrong value type');
  } else if (storageType === 'integer') {
    value = signedId(value, `parameters[${index}].value`);
    if (!parameter.readable) invalid('integer parameter must be readable');
  } else if (storageType === 'double') {
    if (typeof value !== 'number' || !Number.isFinite(value) || !parameter.readable) invalid('double parameter must be a readable finite number');
    value = Object.is(value, -0) ? 0 : value;
  } else if (storageType === 'string') {
    value = text(value, `parameters[${index}].value`);
    if (!parameter.readable) invalid('string parameter must be readable');
  } else if (storageType === 'element-id') {
    value = signedId(value, `parameters[${index}].value`);
    if (!parameter.readable) invalid('element-id parameter must be readable');
  } else invalid(`parameters[${index}] has unsupported storageType`);
  return { id, name, unit, ...(tagged ? { valueEncoding: 'source-storage' } : {}), readable: parameter.readable, storageType, value };
}

function validateV2Parameter(parameter, index) {
  if (parameter?.valueEncoding === 'source-storage') return validateSourceStorageParameter(parameter, index, true);
  closed(parameter, ['id', 'name', 'unit', 'valueEncoding', 'valueType', 'value'], [], `parameters[${index}]`);
  if (parameter.valueEncoding !== 'provider-display') invalid(`parameters[${index}].valueEncoding is unsupported`);
  const id = positiveId(parameter.id, `parameters[${index}].id`);
  const name = text(parameter.name, `parameters[${index}].name`);
  const unit = text(parameter.unit, `parameters[${index}].unit`, true);
  if (!['string', 'number'].includes(parameter.valueType)) invalid(`parameters[${index}].valueType is unsupported`);
  let value = parameter.value;
  if (parameter.valueType === 'string') value = text(value, `parameters[${index}].value`);
  else {
    if (typeof value !== 'number' || !Number.isFinite(value)) invalid(`parameters[${index}].value must be a finite number`);
    value = Object.is(value, -0) ? 0 : value;
  }
  return { id, name, unit, valueEncoding: 'provider-display', valueType: parameter.valueType, value };
}

function assertUniqueReferences(refs, label) {
  if (new Set(refs).size !== refs.length) invalid(`${label} contains duplicate references`);
}

const propertyDocumentBaseBytes = Buffer.byteLength('{"properties":[],"schemaVersion":"2"}', 'utf8');

function bounds(parts) {
  const positions = parts.flatMap((part) => part.positions ?? []);
  if (!positions.length) return null;
  const min = [...positions[0]];
  const max = [...positions[0]];
  for (let index = 1; index < positions.length; index += 1) {
    for (let axis = 0; axis < 3; axis += 1) {
      if (positions[index][axis] < min[axis]) min[axis] = positions[index][axis];
      if (positions[index][axis] > max[axis]) max[axis] = positions[index][axis];
    }
  }
  return { min, max };
}

function compareDecimal(a, b) {
  const left = BigInt(a.includes(':') ? a.slice(a.indexOf(':') + 1) : a);
  const right = BigInt(b.includes(':') ? b.slice(b.indexOf(':') + 1) : b);
  return left < right ? -1 : left > right ? 1 : 0;
}

function digestIds(ids) { return sha256(canonicalJsonBytes([...ids].sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b))))); }

function assertAcyclic(relations, kind) {
  const adjacency = new Map();
  const parents = new Map();
  for (const relation of relations.filter((entry) => entry.kind === kind)) {
    if (parents.has(relation.to)) invalid(`${kind} relationship has an ambiguous parent`, 'reference-relationship-invalid');
    parents.set(relation.to, relation.from);
    const targets = adjacency.get(relation.from) ?? [];
    targets.push(relation.to); adjacency.set(relation.from, targets);
  }
  const visiting = new Set();
  const visited = new Set();
  for (const start of adjacency.keys()) {
    if (visited.has(start)) continue;
    const stack = [{ id: start, exit: false }];
    while (stack.length) {
      const frame = stack.pop();
      if (frame.exit) {
        visiting.delete(frame.id);
        visited.add(frame.id);
        continue;
      }
      if (visiting.has(frame.id)) invalid(`${kind} relationship contains a cycle`, 'reference-relationship-invalid');
      if (visited.has(frame.id)) continue;
      visiting.add(frame.id);
      stack.push({ id: frame.id, exit: true });
      const targets = adjacency.get(frame.id) ?? [];
      for (let index = targets.length - 1; index >= 0; index -= 1) {
        stack.push({ id: targets[index], exit: false });
      }
    }
  }
}

export function normalizeRevitMetadata(input, geometryParts, options = {}) {
  const limits = lowerableLimits(options.limits);
  const propertyLimits = lowerablePropertyExpansionLimits(options.propertyExpansionLimits);
  let metadata = input;
  if (typeof input === 'string' || Buffer.isBuffer(input) || input instanceof Uint8Array) {
    try { metadata = parseJsonStrict(input, { maxBytes: limits.maxMetadataBytes, maxDepth: limits.maxJsonDepth }); }
    catch (error) { invalid(error instanceof Error ? error.message : 'metadata JSON is invalid'); }
  }
  closed(metadata,
    ['schemaVersion', 'document', 'types', 'levels', 'parameterGroups', 'parameters', 'elements', 'relations'], [], 'metadata');
  if (!['1', '2'].includes(metadata.schemaVersion)) invalid('unsupported metadata schemaVersion');
  const metadataV2 = metadata.schemaVersion === '2';
  closed(metadata.document, ['kind', 'id'], [], 'document');
  if (metadata.document.kind !== 'revit-project' || typeof metadata.document.id !== 'string' || !metadata.document.id) invalid('document must be an identified Revit project');

  const types = uniqueTable(metadata.types, 'types', limits.maxEntities);
  const levels = uniqueTable(metadata.levels, 'levels', limits.maxEntities, ['elevation']).map((level, index) => {
    if (level.elevation !== undefined && (typeof level.elevation !== 'number' || !Number.isFinite(level.elevation))) invalid(`levels[${index}].elevation must be finite`);
    return level;
  });
  const parameters = list(metadata.parameters, 'parameters', limits.maxParameters)
    .map(metadataV2 ? validateV2Parameter : (parameter, index) => validateSourceStorageParameter(parameter, index, false));
  if (new Set(parameters.map((entry) => entry.id)).size !== parameters.length) invalid('parameters contains duplicate ids');
  if (metadataV2) parameters.forEach((parameter, index) => {
    if (parameter.id !== String(index + 1)) invalid(`parameters[${index}].id must equal its one-based table index`);
  });
  const parameterGroups = uniqueTable(metadata.parameterGroups, 'parameterGroups', limits.maxParameters, ['parameters']).map((group, index) => {
    const refs = list(group.parameters, `parameterGroups[${index}].parameters`, limits.maxParameters);
    if (metadataV2) {
      if (group.id !== String(index + 1)) invalid(`parameterGroups[${index}].id must equal its one-based table index`);
      assertUniqueReferences(refs, `parameterGroups[${index}].parameters`);
    }
    return { ...group, parameters: refs.map((value, ordinal) => tableIndex(value, parameters, `parameterGroups[${index}].parameters[${ordinal}]`)) };
  });

  if (!Array.isArray(geometryParts)) invalid('geometry parts are required for explicit joins');
  const geometryByName = new Map();
  for (const [index, part] of geometryParts.entries()) {
    if (!part || typeof part.nodeName !== 'string' || !part.nodeName) invalid(`geometry part ${index} has no node name`);
    const parts = geometryByName.get(part.nodeName) ?? [];
    parts.push(part); geometryByName.set(part.nodeName, parts);
  }
  for (const parts of geometryByName.values()) parts.sort((a, b) => (a.primitiveOrdinal ?? 0) - (b.primitiveOrdinal ?? 0));

  const rawElements = list(metadata.elements, 'elements', limits.maxEntities);
  const elementIds = new Set();
  const owners = new Map();
  const propertyRows = [];
  const reachedGroups = new Set();
  const reachedParameters = new Set();
  let elementGroupReferences = 0;
  let canonicalPropertyBytes = propertyDocumentBaseBytes;
  const entities = rawElements.map((element, elementOrdinal) => {
    closed(element,
      ['id', 'revitClass', 'category', 'family', 'type', 'level', 'parameterGroups', 'appearances'], ['ifcGuid'], `elements[${elementOrdinal}]`);
    const id = positiveId(element.id, `elements[${elementOrdinal}].id`);
    if (elementIds.has(id)) invalid(`elements contains duplicate id ${id}`);
    elementIds.add(id);
    const appearances = list(element.appearances, `elements[${elementOrdinal}].appearances`, limits.maxNodes);
    if (new Set(appearances).size !== appearances.length || appearances.some((name) => typeof name !== 'string' || !name)) invalid(`element ${id} appearances must be unique non-empty strings`);
    const joined = [];
    for (const name of appearances) {
      const parts = geometryByName.get(name);
      if (!parts) invalid(`appearance '${name}' does not resolve to an active-scene geometry node`, 'reference-metadata-join-ambiguous');
      if (owners.has(name)) invalid(`appearance '${name}' belongs to more than one entity`, 'reference-metadata-join-ambiguous');
      owners.set(name, id);
      joined.push({ nodeName: name, parts: parts.map((part) => part.primitiveOrdinal ?? 0) });
    }
    const groupRefs = list(element.parameterGroups, `elements[${elementOrdinal}].parameterGroups`, limits.maxParameters);
    if (metadataV2) assertUniqueReferences(groupRefs, `elements[${elementOrdinal}].parameterGroups`);
    elementGroupReferences += groupRefs.length;
    const groups = groupRefs
      .map((value, ordinal) => tableIndex(value, parameterGroups, `elements[${elementOrdinal}].parameterGroups[${ordinal}]`));
    const guidValues = [];
    groups.forEach((group, groupOrdinal) => {
      reachedGroups.add(group.id);
      group.parameters.forEach((parameter, parameterOrdinal) => {
        reachedParameters.add(parameter.id);
        const row = {
          entityId: `element:${id}`,
          groupId: `parameter-group:${group.id}`,
          groupName: group.name,
          groupOrdinal,
          parameterId: `parameter:${parameter.id}`,
          parameterOrdinal,
          name: parameter.name,
          unit: parameter.unit,
          ...(metadataV2
            ? (parameter.valueEncoding === 'provider-display'
              ? { valueEncoding: 'provider-display', valueType: parameter.valueType }
              : { valueEncoding: 'source-storage', readable: parameter.readable, storageType: parameter.storageType })
            : { readable: parameter.readable, storageType: parameter.storageType }),
          value: parameter.value,
        };
        if (metadataV2) {
          if (propertyRows.length >= propertyLimits.maxExpandedPropertyRows) invalid('expanded property count exceeds its limit', 'reference-output-too-large');
          const separatorBytes = propertyRows.length > 0 ? 1 : 0;
          const nextBytes = canonicalPropertyBytes + separatorBytes + canonicalJsonBytes(row).length;
          if (!Number.isSafeInteger(nextBytes) || nextBytes > propertyLimits.maxCanonicalPropertyBytes) {
            invalid('canonical property artifact exceeds its byte limit', 'reference-output-too-large');
          }
          canonicalPropertyBytes = nextBytes;
        } else if (propertyRows.length >= limits.maxParameters) {
          invalid('expanded property count exceeds its limit', 'reference-output-too-large');
        }
        propertyRows.push(row);
        if (parameter.valueEncoding !== 'provider-display' && parameter.name === 'IfcGUID'
          && parameter.storageType === 'string' && parameter.readable && parameter.value) guidValues.push(parameter.value);
      });
    });
    if (new Set(guidValues).size > 1) invalid(`element ${id} has conflicting IfcGUID parameters`);
    const ifcGuid = guidValues[0] ?? null;
    if (element.ifcGuid !== undefined && element.ifcGuid !== ifcGuid) invalid(`element ${id} redundant ifcGuid does not match its authoritative parameter`);
    const type = tableIndex(element.type, types, `elements[${elementOrdinal}].type`, true);
    const level = tableIndex(element.level, levels, `elements[${elementOrdinal}].level`, true);
    return {
      id: `element:${id}`,
      sourceElementId: id,
      ifcGuid,
      revitClass: text(element.revitClass, `elements[${elementOrdinal}].revitClass`, true),
      category: text(element.category, `elements[${elementOrdinal}].category`, true),
      family: text(element.family, `elements[${elementOrdinal}].family`, true),
      typeId: type ? `type:${type.id}` : null,
      typeName: type?.name ?? null,
      levelId: level ? `level:${level.id}` : null,
      levelName: level?.name ?? null,
      geometry: joined,
      bounds: bounds(joined.flatMap((item) => geometryByName.get(item.nodeName))),
    };
  }).sort((a, b) => compareDecimal(a.id, b.id));

  const guidCounts = new Map();
  for (const entity of entities) {
    if (entity.ifcGuid) guidCounts.set(entity.ifcGuid, (guidCounts.get(entity.ifcGuid) ?? 0) + 1);
  }
  for (const entity of entities) {
    if (entity.ifcGuid && guidCounts.get(entity.ifcGuid) > 1) entity.ifcGuid = null;
  }

  propertyRows.sort((a, b) => compareDecimal(a.entityId, b.entityId) || a.groupOrdinal - b.groupOrdinal || a.parameterOrdinal - b.parameterOrdinal || compareDecimal(a.parameterId, b.parameterId));
  const relationIds = new Set();
  const relationships = list(metadata.relations, 'relations', limits.maxRelationships).map((relation, index) => {
    closed(relation, ['id', 'kind', 'from', 'to'], ['providerRelationKind'], `relations[${index}]`);
    const id = positiveId(relation.id, `relations[${index}].id`);
    if (relationIds.has(id)) invalid(`relations contains duplicate id ${id}`);
    relationIds.add(id);
    if (!['contains', 'hosts', 'depends-on', 'provider-explicit'].includes(relation.kind)) invalid(`relation ${id} has unsupported kind`, 'reference-relationship-invalid');
    const from = positiveId(relation.from, `relations[${index}].from`);
    const to = positiveId(relation.to, `relations[${index}].to`);
    if (!elementIds.has(from) || !elementIds.has(to)) invalid(`relation ${id} has a missing endpoint`, 'reference-relationship-invalid');
    let providerRelationKind;
    if (relation.kind === 'provider-explicit') {
      providerRelationKind = text(relation.providerRelationKind, `relations[${index}].providerRelationKind`);
      if (!providerRelationKind || Buffer.byteLength(providerRelationKind) > 256) invalid(`relation ${id} providerRelationKind is invalid`);
    } else if (relation.providerRelationKind !== undefined) invalid(`relation ${id} has an unexpected providerRelationKind`);
    return { id: `relation:${id}`, kind: relation.kind, from: `element:${from}`, to: `element:${to}`, ...(providerRelationKind ? { providerRelationKind } : {}) };
  });
  assertAcyclic(relationships, 'contains');
  assertAcyclic(relationships, 'hosts');
  relationships.sort((a, b) => Buffer.compare(Buffer.from(a.kind), Buffer.from(b.kind)) || compareDecimal(a.from, b.from) || compareDecimal(a.to, b.to) || compareDecimal(a.id, b.id));

  const unclaimedGeometryNodes = [...geometryByName.keys()].filter((name) => !owners.has(name)).sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b)));
  const artifactSchemaVersion = metadataV2 ? '2' : '1';
  const entitiesBytes = canonicalJsonBytes({ schemaVersion: artifactSchemaVersion, entities });
  const propertiesBytes = canonicalJsonBytes({ schemaVersion: artifactSchemaVersion, properties: propertyRows });
  const relationshipsBytes = canonicalJsonBytes({ schemaVersion: artifactSchemaVersion, relationships });
  if (metadataV2 && propertiesBytes.length !== canonicalPropertyBytes) invalid('canonical property byte accounting mismatch');
  for (const [label, bytes] of [['entities', entitiesBytes], ['properties', propertiesBytes], ['relationships', relationshipsBytes]]) {
    if (bytes.length > limits.maxComponentJsonBytes) invalid(`${label} artifact exceeds its byte limit`, 'reference-output-too-large');
  }
  const coverage = {
    discoveredEntities: rawElements.length,
    indexedEntities: entities.length,
    drawableEntities: entities.filter((entity) => entity.geometry.length > 0).length,
    geometryNodes: geometryByName.size,
    properties: propertyRows.length,
    relationships: relationships.length,
    unclaimedGeometryNodes,
    entitySetSha256: digestIds(entities.map((entity) => entity.id)),
    geometryNodeSetSha256: digestIds(geometryByName.keys()),
    ...(metadataV2 ? {
      metadataSchemaVersion: '2',
      nativeParameterGroups: parameterGroups.length,
      nativeParameters: parameters.length,
      elementGroupReferences,
      expandedProperties: propertyRows.length,
      orphanParameterGroups: parameterGroups.length - reachedGroups.size,
      orphanParameters: parameters.length - reachedParameters.size,
      canonicalPropertyBytes: propertiesBytes.length,
      effectivePropertyLimits: propertyLimits,
    } : {}),
  };
  return { entities, properties: propertyRows, relationships, entitiesBytes, propertiesBytes, relationshipsBytes, coverage };
}
