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
  const positions = new Set(); let primitiveCount = 0;
  for (const mesh of document.meshes) {
    if (!mesh || !Array.isArray(mesh.primitives) || mesh.weights !== undefined) {
      invalid('Geometry tile mesh primitives are invalid.');
    }
    for (const primitive of mesh.primitives) {
      if (!primitive || typeof primitive !== 'object' || Array.isArray(primitive)
          || !Number.isSafeInteger(primitive.attributes?.POSITION)
          || primitive.targets !== undefined) {
        invalid('Every geometry primitive requires a POSITION accessor.');
      }
      positions.add(primitive.attributes.POSITION);
      primitiveCount += 1;
    }
  }
  if (positions.size === 0) invalid('Geometry tile contains no primitives.');
  const derived = [Infinity, Infinity, Infinity, -Infinity, -Infinity, -Infinity];
  for (const accessorIndex of positions) {
    const accessor = document.accessors[index(accessorIndex, document.accessors.length, 'POSITION accessor')];
    if (!accessor || accessor.type !== 'VEC3' || accessor.componentType !== 5126
        || accessor.sparse !== undefined || !Number.isSafeInteger(accessor.count) || accessor.count < 1) {
      invalid('POSITION accessor layout is unsupported.');
    }
    const view = document.bufferViews[index(accessor.bufferView, document.bufferViews.length, 'POSITION bufferView')];
    if (!view || view.buffer !== 0) invalid('POSITION bufferView is invalid.');
    const viewOffset = view.byteOffset ?? 0; const accessorOffset = accessor.byteOffset ?? 0;
    const stride = view.byteStride ?? 12;
    if (!Number.isSafeInteger(stride) || stride < 12 || stride % 4 !== 0) invalid('POSITION byte stride is invalid.');
    range(viewOffset, view.byteLength, declaredBinaryLength, 'POSITION bufferView');
    range(accessorOffset, (accessor.count - 1) * stride + 12, view.byteLength, 'POSITION accessor');
    for (let item = 0; item < accessor.count; item += 1) {
      const offset = viewOffset + accessorOffset + item * stride;
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
