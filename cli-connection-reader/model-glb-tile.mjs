import { ModelReaderError } from './model-contract.mjs';
import { parseGlb } from './revit-glb.mjs';

function invalid(message, details = undefined) {
  throw new ModelReaderError('reference-geometry-invalid', 'canonical-artifact', false, message, details);
}

function index(value, length, label) {
  if (!Number.isSafeInteger(value) || value < 0 || value >= length) invalid(`${label} is out of range.`);
  return value;
}

function range(offset, length, bound, label) {
  if (!Number.isSafeInteger(offset) || !Number.isSafeInteger(length) || offset < 0 || length < 0
      || offset > bound || length > bound - offset) invalid(`${label} is outside its buffer.`);
}

function identityTransform(node) {
  const same = (actual, expected) => actual === undefined
    || (Array.isArray(actual) && actual.length === expected.length
      && actual.every((value, position) => value === expected[position]));
  return same(node.matrix, [1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1])
    && same(node.translation, [0, 0, 0]) && same(node.rotation, [0, 0, 0, 1])
    && same(node.scale, [1, 1, 1]);
}

function renderedMeshes(document) {
  if (document.scene !== 0 || !Array.isArray(document.scenes) || document.scenes.length !== 1
      || !document.scenes[0] || !Array.isArray(document.scenes[0].nodes)
      || document.scenes[0].nodes.length === 0) {
    invalid('Geometry tile must contain one explicit rendered scene.');
  }
  const visitedNodes = new Set(); const meshes = new Set();
  const pending = [...document.scenes[0].nodes];
  while (pending.length) {
    const nodeIndex = pending.pop();
    index(nodeIndex, document.nodes.length, 'Scene node');
    if (visitedNodes.has(nodeIndex)) invalid('Geometry tile scene graph must be a tree.');
    visitedNodes.add(nodeIndex);
    const node = document.nodes[nodeIndex];
    if (node.extensions !== undefined) invalid('Geometry tile node extensions are unsupported.');
    if (node.mesh !== undefined) {
      index(node.mesh, document.meshes.length, 'Scene mesh');
      if (meshes.has(node.mesh)) invalid('Geometry tile meshes must be rendered exactly once.');
      meshes.add(node.mesh);
    }
    if (node.children !== undefined) {
      if (!Array.isArray(node.children)) invalid('Geometry tile node children are invalid.');
      pending.push(...node.children);
    }
  }
  if (visitedNodes.size !== document.nodes.length || meshes.size !== document.meshes.length) {
    invalid('Every geometry tile node and mesh must belong to the rendered scene.');
  }
  return meshes;
}

const COMPONENT_BYTES = Object.freeze({ 5120: 1, 5121: 1, 5122: 2, 5123: 2, 5125: 4, 5126: 4 });
const TYPE_COMPONENTS = Object.freeze({ SCALAR: 1, VEC2: 2, VEC3: 3, VEC4: 4, MAT2: 4, MAT3: 9, MAT4: 16 });

function accessorLayout(document, parsed, accessorIndex, declaredBinaryLength, label) {
  const accessor = document.accessors[index(accessorIndex, document.accessors.length, `${label} accessor`)];
  const componentBytes = COMPONENT_BYTES[accessor?.componentType];
  const components = TYPE_COMPONENTS[accessor?.type];
  if (!accessor || !componentBytes || !components || accessor.sparse !== undefined
      || !Number.isSafeInteger(accessor.count) || accessor.count < 1
      || (accessor.normalized !== undefined && typeof accessor.normalized !== 'boolean')) {
    invalid(`${label} accessor layout is unsupported.`);
  }
  const view = document.bufferViews[index(accessor.bufferView, document.bufferViews.length, `${label} bufferView`)];
  const viewOffset = view?.byteOffset ?? 0; const accessorOffset = accessor.byteOffset ?? 0;
  const elementBytes = componentBytes * components;
  const stride = view?.byteStride ?? elementBytes;
  if (!view || view.buffer !== 0 || !Number.isSafeInteger(stride) || stride < elementBytes
      || stride > 252 || stride % componentBytes !== 0
      || !Number.isSafeInteger(viewOffset) || viewOffset < 0
      || !Number.isSafeInteger(accessorOffset) || accessorOffset < 0
      || viewOffset % componentBytes !== 0 || accessorOffset % componentBytes !== 0) {
    invalid(`${label} accessor layout is unsupported.`);
  }
  range(viewOffset, view.byteLength, declaredBinaryLength, `${label} bufferView`);
  range(accessorOffset, (accessor.count - 1) * stride + elementBytes, view.byteLength, `${label} accessor`);
  return { accessor, offset: viewOffset + accessorOffset, stride, elementBytes };
}

function unsignedIndex(binary, componentType, offset) {
  if (componentType === 5121) return binary.readUInt8(offset);
  if (componentType === 5123) return binary.readUInt16LE(offset);
  return binary.readUInt32LE(offset);
}

/** Validate a flattened, self-contained GLB tile and derive bounds from its POSITION bytes. */
export function validateGlbTile(input, options = {}) {
  let parsed;
  try { parsed = parseGlb(input, { limits: options.limits }); }
  catch (error) { invalid('Geometry tile is not a valid GLB.', error); }
  const document = parsed.json;
  const declaredBinaryLength = document?.buffers?.[0]?.byteLength;
  if (!document || typeof document !== 'object' || Array.isArray(document)
      || document.asset?.version !== '2.0' || !Array.isArray(document.buffers)
      || document.buffers.length !== 1 || document.buffers[0]?.uri !== undefined
      || !Number.isSafeInteger(declaredBinaryLength) || declaredBinaryLength < 0
      || parsed.binary.length < declaredBinaryLength || parsed.binary.length - declaredBinaryLength > 3
      || !Array.isArray(document.bufferViews) || !Array.isArray(document.accessors)
      || !Array.isArray(document.meshes) || !Array.isArray(document.nodes)) {
    invalid('Geometry tile must be a self-contained GLB 2.0 document.');
  }
  if (document.nodes.some((node) => !node || typeof node !== 'object' || Array.isArray(node)
      || !identityTransform(node) || node.skin !== undefined || node.weights !== undefined)
      || (Array.isArray(document.skins) && document.skins.length > 0)
      || (Array.isArray(document.animations) && document.animations.length > 0)) {
    invalid('Geometry tile nodes must be flattened to identity transforms.');
  }
  renderedMeshes(document);
  const positions = new Map(); let primitiveCount = 0;
  for (const mesh of document.meshes) {
    if (!mesh || !Array.isArray(mesh.primitives) || mesh.weights !== undefined) {
      invalid('Geometry tile mesh primitives are invalid.');
    }
    for (const primitive of mesh.primitives) {
      if (!primitive || typeof primitive !== 'object' || Array.isArray(primitive)
          || !primitive.attributes || typeof primitive.attributes !== 'object'
          || Array.isArray(primitive.attributes) || !Number.isSafeInteger(primitive.attributes.POSITION)
          || primitive.targets !== undefined || primitive.extensions !== undefined) {
        invalid('Every geometry primitive requires a POSITION accessor.');
      }
      const position = accessorLayout(
        document, parsed, primitive.attributes.POSITION, declaredBinaryLength, 'POSITION',
      );
      if (position.accessor.type !== 'VEC3' || position.accessor.componentType !== 5126
          || position.accessor.normalized === true) {
        invalid('POSITION accessor layout is unsupported.');
      }
      positions.set(primitive.attributes.POSITION, position);
      for (const [semantic, accessorIndex] of Object.entries(primitive.attributes)) {
        if (!semantic || !Number.isSafeInteger(accessorIndex)) {
          invalid('Geometry primitive attributes are invalid.');
        }
        const layout = semantic === 'POSITION' ? position : accessorLayout(
          document, parsed, accessorIndex, declaredBinaryLength, semantic,
        );
        if (layout.accessor.count !== position.accessor.count) {
          invalid('Geometry primitive attribute counts do not match POSITION.');
        }
      }
      if (primitive.indices !== undefined) {
        const indices = accessorLayout(document, parsed, primitive.indices, declaredBinaryLength, 'Index');
        if (indices.accessor.type !== 'SCALAR' || ![5121, 5123, 5125].includes(indices.accessor.componentType)) {
          invalid('Geometry primitive index accessor is unsupported.');
        }
        for (let item = 0; item < indices.accessor.count; item += 1) {
          if (unsignedIndex(parsed.binary, indices.accessor.componentType,
            indices.offset + item * indices.stride) >= position.accessor.count) {
            invalid('Geometry primitive index exceeds the POSITION vertex count.');
          }
        }
      }
      if (primitive.mode !== undefined
          && (!Number.isSafeInteger(primitive.mode) || primitive.mode < 0 || primitive.mode > 6)) {
        invalid('Geometry primitive mode is invalid.');
      }
      primitiveCount += 1;
    }
  }
  if (positions.size === 0) invalid('Geometry tile contains no primitives.');
  const derived = [Infinity, Infinity, Infinity, -Infinity, -Infinity, -Infinity];
  for (const position of positions.values()) {
    for (let item = 0; item < position.accessor.count; item += 1) {
      const offset = position.offset + item * position.stride;
      for (let axis = 0; axis < 3; axis += 1) {
        const value = parsed.binary.readFloatLE(offset + axis * 4);
        if (!Number.isFinite(value) || !Number.isSafeInteger(value) && Math.abs(value) > Number.MAX_SAFE_INTEGER) {
          invalid('POSITION contains a non-canonical coordinate.');
        }
        const canonical = Object.is(value, -0) ? 0 : value;
        derived[axis] = Math.min(derived[axis], canonical);
        derived[axis + 3] = Math.max(derived[axis + 3], canonical);
      }
    }
  }
  return { bounds: derived, primitiveCount };
}
