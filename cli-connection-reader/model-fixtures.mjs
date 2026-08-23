import { canonicalJsonBytes } from './model-contract.mjs';

const align4 = (value) => (value + 3) & ~3;

export function makeGlbFixture(options = {}) {
  const positions = options.positions ?? [[0, 0, 0], [1, 0, 0], [0, 1, 0]];
  const indices = options.indices ?? [0, 1, 2];
  const positionBytes = Buffer.alloc(positions.length * 12);
  positions.forEach((point, index) => point.forEach((coordinate, axis) => positionBytes.writeFloatLE(coordinate, index * 12 + axis * 4)));
  const indexOffset = align4(positionBytes.length);
  const indexEnd = indexOffset + indices.length * 4;
  const colorOffset = align4(indexEnd);
  const colors = options.colors ?? null;
  const binary = Buffer.alloc(colorOffset + (colors ? colors.length * 16 : 0));
  positionBytes.copy(binary);
  indices.forEach((value, index) => binary.writeUInt32LE(value, indexOffset + index * 4));
  colors?.forEach((color, index) => color.forEach((component, channel) => binary.writeFloatLE(component, colorOffset + index * 16 + channel * 4)));
  const primitive = { attributes: { POSITION: 0 }, indices: 1, mode: options.mode ?? 4 };
  if (colors) primitive.attributes.COLOR_0 = 2;
  if (options.materialColor) primitive.material = 0;
  const defaultNode = { name: options.nodeName ?? 'part-a', mesh: 0 };
  if (options.translation) defaultNode.translation = options.translation;
  if (options.rotation) defaultNode.rotation = options.rotation;
  if (options.scale) defaultNode.scale = options.scale;
  if (options.matrix) defaultNode.matrix = options.matrix;
  const json = {
    asset: { version: '2.0' },
    scenes: options.scenes ?? [{ nodes: [0] }],
    nodes: options.nodes ?? [defaultNode],
    meshes: options.meshes ?? [{ primitives: [primitive] }],
    buffers: [{ byteLength: binary.length, ...(options.externalUri ? { uri: options.externalUri } : {}) }],
    bufferViews: [
      { buffer: 0, byteOffset: 0, byteLength: positionBytes.length },
      { buffer: 0, byteOffset: indexOffset, byteLength: indices.length * 4 },
      ...(colors ? [{ buffer: 0, byteOffset: colorOffset, byteLength: colors.length * 16 }] : []),
    ],
    accessors: [
      { bufferView: 0, byteOffset: 0, componentType: 5126, count: positions.length, type: 'VEC3' },
      { bufferView: 1, byteOffset: 0, componentType: 5125, count: indices.length, type: 'SCALAR' },
      ...(colors ? [{ bufferView: 2, byteOffset: 0, componentType: 5126, count: colors.length, type: 'VEC4' }] : []),
    ],
    ...(options.materialColor ? { materials: [{ pbrMetallicRoughness: { baseColorFactor: options.materialColor } }] } : {}),
    ...(options.extensionsUsed ? { extensionsUsed: options.extensionsUsed } : {}),
  };
  if (!options.omitScene) json.scene = options.scene ?? 0;
  const jsonBytes = canonicalJsonBytes(json);
  const jsonLength = align4(jsonBytes.length);
  const binaryLength = align4(binary.length);
  const out = Buffer.alloc(12 + 8 + jsonLength + 8 + binaryLength);
  out.writeUInt32LE(0x46546c67, 0);
  out.writeUInt32LE(2, 4);
  out.writeUInt32LE(out.length, 8);
  out.writeUInt32LE(jsonLength, 12);
  out.writeUInt32LE(0x4e4f534a, 16);
  out.fill(0x20, 20, 20 + jsonLength);
  jsonBytes.copy(out, 20);
  const binHeader = 20 + jsonLength;
  out.writeUInt32LE(binaryLength, binHeader);
  out.writeUInt32LE(0x004e4942, binHeader + 4);
  binary.copy(out, binHeader + 8);
  return out;
}

export function makeMetadataFixture(options = {}) {
  const elementId = options.elementId ?? '1001';
  return {
    schemaVersion: '1',
    document: { kind: 'revit-project', id: 'document:fixture' },
    types: [{ id: '2001', name: 'Fixture Type' }],
    levels: [{ id: '3001', name: 'Level 1', elevation: 0 }],
    parameterGroups: [{ id: '4001', name: 'Identity Data', parameters: [0] }],
    parameters: [{ id: '5001', name: 'IfcGUID', unit: null, readable: true, storageType: 'string', value: options.ifcGuid ?? '1FixtureGuid00000000000' }],
    elements: [{
      id: elementId,
      revitClass: 'FamilyInstance',
      category: 'Structural Framing',
      family: 'Fixture Family',
      type: 0,
      level: 0,
      parameterGroups: [0],
      appearances: options.nodeNames ?? ['part-a'],
      ifcGuid: options.ifcGuid ?? '1FixtureGuid00000000000',
    }],
    relations: [],
  };
}
