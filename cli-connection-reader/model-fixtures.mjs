import { canonicalJsonBytes } from './model-contract.mjs';

const align4 = (value) => (value + 3) & ~3;

export function makeGlbFixture(options = {}) {
  const positions = options.positions ?? [[0, 0, 0], [1, 0, 0], [0, 1, 0]];
  const indices = options.indices ?? [0, 1, 2];
  const positionBytes = Buffer.alloc(positions.length * 12);
  positions.forEach((point, index) => point.forEach((coordinate, axis) => positionBytes.writeFloatLE(coordinate, index * 12 + axis * 4)));
  const indexOffset = align4(positionBytes.length);
  const binary = Buffer.alloc(indexOffset + indices.length * 4);
  positionBytes.copy(binary);
  indices.forEach((value, index) => binary.writeUInt32LE(value, indexOffset + index * 4));
  const json = {
    asset: { version: '2.0' },
    scene: options.scene ?? 0,
    scenes: options.scenes ?? [{ nodes: [0] }],
    nodes: options.nodes ?? [{ name: options.nodeName ?? 'part-a', mesh: 0 }],
    meshes: options.meshes ?? [{ primitives: [{ attributes: { POSITION: 0 }, indices: 1, mode: options.mode ?? 4 }] }],
    buffers: [{ byteLength: binary.length }],
    bufferViews: [
      { buffer: 0, byteOffset: 0, byteLength: positionBytes.length },
      { buffer: 0, byteOffset: indexOffset, byteLength: indices.length * 4 },
    ],
    accessors: [
      { bufferView: 0, byteOffset: 0, componentType: 5126, count: positions.length, type: 'VEC3' },
      { bufferView: 1, byteOffset: 0, componentType: 5125, count: indices.length, type: 'SCALAR' },
    ],
  };
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

