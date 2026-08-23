import { canonicalJsonBytes, lowerableLimits, ModelReaderError, parseJsonStrict } from './model-contract.mjs';

const GLB_MAGIC = 0x46546c67;
const JSON_CHUNK = 0x4e4f534a;
const BIN_CHUNK = 0x004e4942;
const COMPONENT_BYTES = new Map([[5121, 1], [5123, 2], [5125, 4], [5126, 4]]);
const IDENTITY = [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1];

function invalid(message, code = 'reference-geometry-invalid') {
  throw new ModelReaderError(code, 'normalize-geometry', false, message);
}

function checkedRange(offset, length, bound, label) {
  if (!Number.isSafeInteger(offset) || !Number.isSafeInteger(length) || offset < 0 || length < 0 || offset > bound || length > bound - offset) {
    invalid(`${label} range is outside its buffer`);
  }
}

export function parseGlb(input, options = {}) {
  const limits = lowerableLimits(options.limits);
  const bytes = Buffer.from(input);
  if (bytes.length > limits.maxInputGlbBytes) invalid('GLB exceeds its byte limit', 'reference-output-too-large');
  if (bytes.length < 20) invalid('GLB length is too short');
  if (bytes.readUInt32LE(0) !== GLB_MAGIC || bytes.readUInt32LE(4) !== 2) invalid('GLB header or version is invalid');
  if (bytes.readUInt32LE(8) !== bytes.length) invalid('GLB declared length does not match its bytes');
  let cursor = 12;
  let jsonText = null;
  let binary = null;
  while (cursor < bytes.length) {
    checkedRange(cursor, 8, bytes.length, 'GLB chunk header');
    const length = bytes.readUInt32LE(cursor);
    const type = bytes.readUInt32LE(cursor + 4);
    if (length % 4 !== 0) invalid('GLB chunk length is not 4-byte aligned');
    cursor += 8;
    checkedRange(cursor, length, bytes.length, 'GLB chunk');
    const payload = bytes.subarray(cursor, cursor + length);
    cursor += length;
    if (type === JSON_CHUNK) {
      if (jsonText !== null || binary !== null) invalid('GLB JSON chunk must be first and unique');
      if (length > limits.maxGlbJsonBytes) invalid('GLB JSON exceeds its byte limit', 'reference-output-too-large');
      jsonText = payload.toString('utf8').replace(/[\u0000\u0020]+$/u, '');
    } else if (type === BIN_CHUNK) {
      if (jsonText === null || binary !== null) invalid('GLB BIN chunk is duplicated or precedes JSON');
      binary = Buffer.from(payload);
    } else {
      invalid('GLB contains an unsupported chunk type', 'reference-geometry-unsupported');
    }
  }
  if (jsonText === null) invalid('GLB has no JSON chunk');
  let json;
  try { json = parseJsonStrict(jsonText, { maxBytes: limits.maxGlbJsonBytes, maxDepth: limits.maxJsonDepth }); }
  catch (error) { invalid(error instanceof Error ? error.message : 'GLB JSON is invalid'); }
  return { json, binary: binary ?? Buffer.alloc(0), jsonText };
}

function array(value, label, max) {
  if (!Array.isArray(value) || value.length > max) invalid(`${label} is missing or exceeds its count limit`);
  return value;
}

function safeIndex(value, length, label) {
  if (!Number.isSafeInteger(value) || value < 0 || value >= length) invalid(`${label} is out of range`);
  return value;
}

function finiteArray(value, length, label) {
  if (!Array.isArray(value) || value.length !== length || value.some((entry) => typeof entry !== 'number' || !Number.isFinite(entry))) invalid(`${label} must contain ${length} finite numbers`);
  return value;
}

function multiply(a, b) {
  const out = new Array(16).fill(0);
  for (let column = 0; column < 4; column += 1) {
    for (let row = 0; row < 4; row += 1) {
      for (let k = 0; k < 4; k += 1) out[column * 4 + row] += a[k * 4 + row] * b[column * 4 + k];
    }
  }
  return out;
}

function nodeMatrix(node) {
  if (node.matrix !== undefined && (node.translation !== undefined || node.rotation !== undefined || node.scale !== undefined)) invalid('node cannot mix matrix and TRS');
  if (node.matrix !== undefined) return [...finiteArray(node.matrix, 16, 'node matrix')];
  const translation = node.translation === undefined ? [0, 0, 0] : finiteArray(node.translation, 3, 'node translation');
  const scale = node.scale === undefined ? [1, 1, 1] : finiteArray(node.scale, 3, 'node scale');
  let quaternion = node.rotation === undefined ? [0, 0, 0, 1] : finiteArray(node.rotation, 4, 'node rotation');
  const norm = Math.hypot(...quaternion);
  if (!Number.isFinite(norm) || norm === 0 || Math.abs(norm - 1) > 1e-3) invalid('node quaternion is outside the unit-length tolerance');
  quaternion = quaternion.map((entry) => entry / norm);
  const [x, y, z, w] = quaternion;
  const [sx, sy, sz] = scale;
  return [
    (1 - 2 * y * y - 2 * z * z) * sx, (2 * x * y + 2 * z * w) * sx, (2 * x * z - 2 * y * w) * sx, 0,
    (2 * x * y - 2 * z * w) * sy, (1 - 2 * x * x - 2 * z * z) * sy, (2 * y * z + 2 * x * w) * sy, 0,
    (2 * x * z + 2 * y * w) * sz, (2 * y * z - 2 * x * w) * sz, (1 - 2 * x * x - 2 * y * y) * sz, 0,
    translation[0], translation[1], translation[2], 1,
  ];
}

function determinant3(matrix) {
  const a = matrix[0], b = matrix[4], c = matrix[8];
  const d = matrix[1], e = matrix[5], f = matrix[9];
  const g = matrix[2], h = matrix[6], i = matrix[10];
  return a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g);
}

function transform(matrix, point) {
  const [x, y, z] = point;
  const tx = matrix[0] * x + matrix[4] * y + matrix[8] * z + matrix[12];
  const ty = matrix[1] * x + matrix[5] * y + matrix[9] * z + matrix[13];
  const tz = matrix[2] * x + matrix[6] * y + matrix[10] * z + matrix[14];
  return canonicalVector([tx * 1000, -tz * 1000, ty * 1000]);
}

function canonicalFloat(value) {
  const rounded = Math.fround(value);
  if (!Number.isFinite(rounded)) invalid('transformed coordinate overflows float32');
  return Object.is(rounded, -0) ? 0 : rounded;
}
const canonicalVector = (values) => values.map(canonicalFloat);

function positionBounds(positions) {
  if (!positions.length) return { min: [0, 0, 0], max: [0, 0, 0] };
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

function accessorReader(document, binary, accessorIndex, expected, label) {
  const accessors = document.accessors;
  const views = document.bufferViews;
  const accessor = accessors[safeIndex(accessorIndex, accessors.length, `${label} accessor`)];
  if (!accessor || typeof accessor !== 'object' || accessor.sparse !== undefined) invalid(`${label} accessor is sparse or invalid`, 'reference-geometry-unsupported');
  if (expected.types && !expected.types.includes(accessor.type)) invalid(`${label} accessor has unsupported type`, 'reference-geometry-unsupported');
  if (expected.components && !expected.components.includes(accessor.componentType)) invalid(`${label} accessor has unsupported component type`, 'reference-geometry-unsupported');
  if (!Number.isSafeInteger(accessor.count) || accessor.count < 0) invalid(`${label} accessor count is invalid`);
  const width = accessor.type === 'VEC4' ? 4 : accessor.type === 'VEC3' ? 3 : accessor.type === 'SCALAR' ? 1 : 0;
  const componentBytes = COMPONENT_BYTES.get(accessor.componentType);
  if (!width || !componentBytes) invalid(`${label} accessor layout is unsupported`, 'reference-geometry-unsupported');
  const view = views[safeIndex(accessor.bufferView, views.length, `${label} bufferView`)];
  if (view.buffer !== 0) invalid(`${label} accessor does not bind buffers[0]`);
  const stride = view.byteStride ?? width * componentBytes;
  if (!Number.isSafeInteger(stride) || stride < width * componentBytes || stride % componentBytes !== 0) invalid(`${label} byteStride is invalid`);
  const viewOffset = view.byteOffset ?? 0;
  const accessorOffset = accessor.byteOffset ?? 0;
  const viewLength = view.byteLength;
  checkedRange(viewOffset, viewLength, document.buffers[0].byteLength, `${label} bufferView`);
  if (accessorOffset % componentBytes !== 0) invalid(`${label} accessor is misaligned`);
  const required = accessor.count === 0 ? 0 : (accessor.count - 1) * stride + width * componentBytes;
  checkedRange(accessorOffset, required, viewLength, `${label} accessor`);
  const base = viewOffset + accessorOffset;
  const readComponent = (offset) => {
    if (accessor.componentType === 5126) return binary.readFloatLE(offset);
    if (accessor.componentType === 5125) return binary.readUInt32LE(offset);
    if (accessor.componentType === 5123) return binary.readUInt16LE(offset);
    return binary.readUInt8(offset);
  };
  return {
    count: accessor.count,
    normalized: accessor.normalized === true,
    componentType: accessor.componentType,
    read(index) {
      safeIndex(index, accessor.count, `${label} item`);
      const offset = base + index * stride;
      const result = [];
      for (let component = 0; component < width; component += 1) result.push(readComponent(offset + component * componentBytes));
      return result;
    },
  };
}

function indicesFor(primitive, document, binary, vertexCount, remainingIndices) {
  let indices;
  if (primitive.indices === undefined) {
    if (vertexCount > remainingIndices) invalid('model index count exceeds its limit', 'reference-output-too-large');
    indices = Array.from({ length: vertexCount }, (_, index) => index);
  }
  else {
    const reader = accessorReader(document, binary, primitive.indices, { types: ['SCALAR'], components: [5121, 5123, 5125] }, 'indices');
    if (reader.count > remainingIndices) invalid('model index count exceeds its limit', 'reference-output-too-large');
    indices = Array.from({ length: reader.count }, (_, index) => reader.read(index)[0]);
  }
  for (const index of indices) if (index >= vertexCount) invalid('index value exceeds POSITION count');
  return indices;
}

function expandTriangles(indices, mode) {
  const triangles = [];
  if (mode === 4) {
    if (indices.length % 3 !== 0) invalid('TRIANGLES index count is not divisible by three');
    for (let index = 0; index < indices.length; index += 3) triangles.push(indices.slice(index, index + 3));
  } else if (mode === 5) {
    for (let index = 2; index < indices.length; index += 1) triangles.push(index % 2 === 0 ? [indices[index - 2], indices[index - 1], indices[index]] : [indices[index - 1], indices[index - 2], indices[index]]);
  } else if (mode === 6) {
    for (let index = 2; index < indices.length; index += 1) triangles.push([indices[0], indices[index - 1], indices[index]]);
  } else invalid('primitive mode is unsupported', 'reference-geometry-unsupported');
  return triangles;
}

function baseColor(document, primitive) {
  if (primitive.material === undefined) return [1, 1, 1, 1];
  const material = document.materials[safeIndex(primitive.material, document.materials.length, 'material')];
  const allowed = new Set(['pbrMetallicRoughness', 'alphaMode', 'doubleSided', 'emissiveFactor']);
  for (const key of Object.keys(material)) if (!allowed.has(key)) invalid(`material property '${key}' is unsupported`, 'reference-geometry-unsupported');
  if (material.alphaMode !== undefined && material.alphaMode !== 'OPAQUE') invalid('non-opaque material is unsupported', 'reference-geometry-unsupported');
  if (material.doubleSided === true) invalid('double-sided material is unsupported', 'reference-geometry-unsupported');
  if (material.emissiveFactor && finiteArray(material.emissiveFactor, 3, 'emissiveFactor').some((entry) => entry !== 0)) invalid('emissive material is unsupported', 'reference-geometry-unsupported');
  const pbr = material.pbrMetallicRoughness ?? {};
  for (const key of Object.keys(pbr)) if (!['baseColorFactor', 'metallicFactor', 'roughnessFactor'].includes(key)) invalid(`material PBR property '${key}' is unsupported`, 'reference-geometry-unsupported');
  const factor = pbr.baseColorFactor === undefined ? [1, 1, 1, 1] : finiteArray(pbr.baseColorFactor, 4, 'baseColorFactor');
  if (factor.some((entry) => entry < 0 || entry > 1)) invalid('baseColorFactor is outside [0,1]');
  return factor;
}

function colorsFor(primitive, document, binary, count) {
  const factor = baseColor(document, primitive);
  if (primitive.attributes.COLOR_0 === undefined) return Array.from({ length: count }, () => factor.map(canonicalFloat));
  const reader = accessorReader(document, binary, primitive.attributes.COLOR_0,
    { types: ['VEC3', 'VEC4'], components: [5121, 5123, 5126] }, 'COLOR_0');
  if (reader.count !== count) invalid('COLOR_0 count does not match POSITION');
  if (reader.componentType === 5126 && reader.normalized) invalid('FLOAT COLOR_0 cannot be normalized');
  if (reader.componentType !== 5126 && !reader.normalized) invalid('integer COLOR_0 must be normalized');
  const divisor = reader.componentType === 5121 ? 255 : reader.componentType === 5123 ? 65535 : 1;
  return Array.from({ length: count }, (_, index) => {
    const raw = reader.read(index).map((entry) => entry / divisor);
    if (raw.length === 3) raw.push(1);
    return raw.map((entry, channel) => canonicalFloat(entry * factor[channel]));
  });
}

function tupleCompare(a, b) {
  for (let index = 0; index < a.length; index += 1) {
    if (a[index] < b[index]) return -1;
    if (a[index] > b[index]) return 1;
  }
  return 0;
}

function canonicalPart({ nodeName, primitiveOrdinal, positions, colors, triangles, reverseWinding }) {
  const sourceRecords = positions.map((position, index) => ({ tuple: [...position, ...colors[index]], position, color: colors[index] }));
  const unique = new Map();
  for (const record of sourceRecords) unique.set(record.tuple.join(','), record);
  const records = [...unique.values()].sort((a, b) => tupleCompare(a.tuple, b.tuple));
  const canonicalIndex = new Map(records.map((record, index) => [record.tuple.join(','), index]));
  const remapped = [];
  for (const triangle of triangles) {
    let indices = triangle.map((sourceIndex) => canonicalIndex.get(sourceRecords[sourceIndex].tuple.join(',')));
    if (new Set(indices).size !== 3) continue;
    if (reverseWinding) indices = [indices[0], indices[2], indices[1]];
    const rotations = [indices, [indices[1], indices[2], indices[0]], [indices[2], indices[0], indices[1]]];
    rotations.sort(tupleCompare);
    remapped.push(rotations[0]);
  }
  remapped.sort(tupleCompare);
  return {
    nodeName,
    primitiveOrdinal,
    positions: records.map((record) => record.position),
    colors: records.map((record) => record.color),
    triangles: remapped,
  };
}

function profile(document, binary, limits) {
  if (!document || typeof document !== 'object' || document.asset?.version !== '2.0') invalid('GLB asset version must be 2.0');
  for (const field of ['extensions', 'extensionsUsed', 'extensionsRequired']) if (document[field] !== undefined) invalid('GLB extensions are unsupported', 'reference-geometry-unsupported');
  const buffers = array(document.buffers, 'buffers', 1);
  if (buffers.length !== 1 || buffers[0].uri !== undefined) invalid('external resource URI is refused', 'reference-external-resource-refused');
  if (!Number.isSafeInteger(buffers[0].byteLength) || buffers[0].byteLength < 0 || buffers[0].byteLength > binary.length || binary.length - buffers[0].byteLength > 3) invalid('BIN chunk length does not match buffers[0]');
  const scenes = array(document.scenes, 'scenes', limits.maxScenes);
  const nodes = array(document.nodes, 'nodes', limits.maxNodes);
  const meshes = array(document.meshes, 'meshes', limits.maxMeshes);
  document.accessors = array(document.accessors, 'accessors', limits.maxAccessors);
  document.bufferViews = array(document.bufferViews, 'bufferViews', limits.maxBufferViews);
  document.materials = document.materials === undefined ? [] : array(document.materials, 'materials', limits.maxPrimitives);
  if (!Number.isSafeInteger(document.scene)) invalid('GLB requires an explicit active scene');
  const active = scenes[safeIndex(document.scene, scenes.length, 'active scene')];
  const roots = array(active.nodes, 'active scene nodes', limits.maxNodes);
  const visiting = new Set();
  const visited = new Set();
  const names = new Set();
  const walked = [];
  const walk = (nodeIndex, parent, depth) => {
    safeIndex(nodeIndex, nodes.length, 'node');
    if (depth > limits.maxNodeDepth) invalid('active scene exceeds its depth limit');
    if (visiting.has(nodeIndex)) invalid('active scene contains a cycle');
    if (visited.has(nodeIndex)) invalid('active scene node has more than one parent');
    visiting.add(nodeIndex); visited.add(nodeIndex);
    const node = nodes[nodeIndex];
    const world = multiply(parent, nodeMatrix(node));
    if (node.mesh !== undefined) {
      safeIndex(node.mesh, meshes.length, 'node mesh');
      if (typeof node.name !== 'string' || !node.name) invalid('drawable node requires a non-empty name');
      if (names.has(node.name)) invalid('drawable node names must be unique');
      names.add(node.name);
      walked.push({ node, world, mesh: meshes[node.mesh] });
    }
    if (node.children !== undefined) for (const child of array(node.children, 'node children', limits.maxNodes)) walk(child, world, depth + 1);
    visiting.delete(nodeIndex);
  };
  for (const root of roots) walk(root, IDENTITY, 0);
  return walked;
}

function degenerate(a, b, c) {
  const ab = b.map((value, index) => value - a[index]);
  const ac = c.map((value, index) => value - a[index]);
  const cross = [ab[1] * ac[2] - ab[2] * ac[1], ab[2] * ac[0] - ab[0] * ac[2], ab[0] * ac[1] - ab[1] * ac[0]];
  return cross.every((entry) => entry === 0);
}

export function normalizeRevitGlb(input, options = {}) {
  const limits = lowerableLimits(options.limits);
  const { json: document, binary } = parseGlb(input, { limits });
  if (document.skins !== undefined || document.animations !== undefined) invalid('skins and animations are unsupported', 'reference-geometry-unsupported');
  if (Array.isArray(document.nodes) && document.nodes.some((node) => node?.skin !== undefined)) invalid('skinned nodes are unsupported', 'reference-geometry-unsupported');
  const nodes = profile(document, binary, limits);
  const parts = [];
  let inputTriangles = 0;
  let droppedDegenerateTriangles = 0;
  let totalVertices = 0;
  let totalIndices = 0;
  let totalPrimitives = 0;
  for (const { node, world, mesh } of nodes) {
    const determinant = determinant3(world);
    if (!Number.isFinite(determinant) || Math.abs(determinant) < 1e-12) invalid('node transform is singular');
    const primitives = array(mesh.primitives, 'mesh primitives', limits.maxPrimitives);
    primitives.forEach((primitive, primitiveOrdinal) => {
      totalPrimitives += 1;
      if (totalPrimitives > limits.maxPrimitives) invalid('model primitive count exceeds its limit', 'reference-output-too-large');
      if (primitive.targets !== undefined || primitive.extensions !== undefined) invalid('primitive extensions or morph targets are unsupported', 'reference-geometry-unsupported');
      if (!primitive.attributes || typeof primitive.attributes !== 'object' || Object.keys(primitive.attributes).some((key) => !['POSITION', 'COLOR_0'].includes(key))) invalid('primitive attributes are unsupported', 'reference-geometry-unsupported');
      const positionsReader = accessorReader(document, binary, primitive.attributes.POSITION, { types: ['VEC3'], components: [5126] }, 'POSITION');
      if (positionsReader.normalized || positionsReader.count > limits.maxVertices - totalVertices) invalid('model vertex count exceeds its limit', 'reference-output-too-large');
      totalVertices += positionsReader.count;
      const positions = Array.from({ length: positionsReader.count }, (_, index) => transform(world, positionsReader.read(index)));
      const colors = colorsFor(primitive, document, binary, positions.length);
      const indices = indicesFor(primitive, document, binary, positions.length, limits.maxIndices - totalIndices);
      totalIndices += indices.length;
      const expanded = expandTriangles(indices, primitive.mode ?? 4);
      inputTriangles += expanded.length;
      const drawable = expanded.filter(([a, b, c]) => {
        const drop = a === b || b === c || a === c || degenerate(positions[a], positions[b], positions[c]);
        if (drop) droppedDegenerateTriangles += 1;
        return !drop;
      });
      parts.push(canonicalPart({ nodeName: node.name, primitiveOrdinal, positions, colors, triangles: drawable, reverseWinding: determinant < 0 }));
    });
  }
  parts.sort((a, b) => Buffer.compare(Buffer.from(a.nodeName), Buffer.from(b.nodeName)) || a.primitiveOrdinal - b.primitiveOrdinal);
  const glb = buildCanonicalGlb(parts);
  if (glb.length > limits.maxCanonicalGlbBytes) invalid('canonical GLB exceeds its limit', 'reference-output-too-large');
  return {
    glb,
    parts,
    coverage: { inputTriangles, outputTriangles: parts.reduce((sum, part) => sum + part.triangles.length, 0), droppedDegenerateTriangles },
  };
}

function align4(value) { return (value + 3) & ~3; }

function buildCanonicalGlb(parts) {
  const chunks = [];
  const bufferViews = [];
  const accessors = [];
  const meshes = [];
  const nodes = [];
  let offset = 0;
  const append = (bytes, target) => {
    const aligned = align4(offset);
    if (aligned > offset) chunks.push(Buffer.alloc(aligned - offset));
    offset = aligned;
    const view = bufferViews.length;
    bufferViews.push({ buffer: 0, byteOffset: offset, byteLength: bytes.length, target });
    chunks.push(bytes); offset += bytes.length;
    return view;
  };
  for (const part of parts) {
    const positions = Buffer.alloc(part.positions.length * 12);
    part.positions.forEach((point, index) => point.forEach((value, axis) => positions.writeFloatLE(value, index * 12 + axis * 4)));
    const colors = Buffer.alloc(part.colors.length * 16);
    part.colors.forEach((color, index) => color.forEach((value, channel) => colors.writeFloatLE(value, index * 16 + channel * 4)));
    const flatIndices = part.triangles.flat();
    const indices = Buffer.alloc(flatIndices.length * 4);
    flatIndices.forEach((value, index) => indices.writeUInt32LE(value, index * 4));
    const positionView = append(positions, 34962);
    const colorView = append(colors, 34962);
    const indexView = append(indices, 34963);
    const positionAccessor = accessors.length;
    const { min: mins, max: maxs } = positionBounds(part.positions);
    accessors.push({ bufferView: positionView, byteOffset: 0, componentType: 5126, count: part.positions.length, type: 'VEC3', min: mins, max: maxs });
    const colorAccessor = accessors.length;
    accessors.push({ bufferView: colorView, byteOffset: 0, componentType: 5126, count: part.colors.length, type: 'VEC4' });
    const indexAccessor = accessors.length;
    accessors.push({ bufferView: indexView, byteOffset: 0, componentType: 5125, count: flatIndices.length, type: 'SCALAR' });
    const mesh = meshes.length;
    meshes.push({ primitives: [{ attributes: { POSITION: positionAccessor, COLOR_0: colorAccessor }, indices: indexAccessor, mode: 4 }] });
    nodes.push({ name: part.nodeName, mesh });
  }
  const binary = Buffer.concat(chunks);
  const document = {
    accessors,
    asset: { version: '2.0' },
    bufferViews,
    buffers: [{ byteLength: binary.length }],
    meshes,
    nodes,
    scene: 0,
    scenes: [{ nodes: nodes.map((_, index) => index) }],
  };
  const jsonBytes = canonicalJsonBytes(document);
  const jsonLength = align4(jsonBytes.length);
  const binaryLength = align4(binary.length);
  const output = Buffer.alloc(12 + 8 + jsonLength + 8 + binaryLength);
  output.writeUInt32LE(GLB_MAGIC, 0); output.writeUInt32LE(2, 4); output.writeUInt32LE(output.length, 8);
  output.writeUInt32LE(jsonLength, 12); output.writeUInt32LE(JSON_CHUNK, 16); output.fill(0x20, 20, 20 + jsonLength); jsonBytes.copy(output, 20);
  const binOffset = 20 + jsonLength;
  output.writeUInt32LE(binaryLength, binOffset); output.writeUInt32LE(BIN_CHUNK, binOffset + 4); binary.copy(output, binOffset + 8);
  return output;
}
