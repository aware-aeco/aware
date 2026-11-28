import { checkMesh } from './snapshot-clash-geometry.mjs';

const triangleLimit = 100_000;

function weld({ positions_mm, triangles }) {
  if (triangles.length > triangleLimit) throw new Error('mesh triangle limit exceeded');
  const seen = new Map();
  const positions = [];
  const remap = positions_mm.map(point => {
    if (point.length !== 3 || !point.every(Number.isFinite)) throw new Error('expected finite position');
    const key = point.join(',');
    if (!seen.has(key)) {
      seen.set(key, positions.length);
      positions.push(point.map(value => value === 0 ? 0 : value));
    }
    return seen.get(key);
  });
  const faces = triangles.map(face => {
    if (face.length !== 3 || !face.every(index => Number.isSafeInteger(index) && index >= 0 && index < remap.length)) {
      throw new Error('invalid triangle index');
    }
    const welded = face.map(index => remap[index]);
    if (new Set(welded).size !== 3) throw new Error('collapsed triangle');
    return welded;
  });
  return { positions_mm: positions, triangles: faces };
}

function checkVertexFans({ positions_mm, triangles }) {
  const links = positions_mm.map(() => new Map());
  for (const [a, b, c] of triangles) {
    for (const [vertex, from, to] of [[a, b, c], [b, c, a], [c, a, b]]) {
      links[vertex].set(from, to);
    }
  }
  for (const link of links) {
    const start = link.keys().next().value;
    if (start === undefined) throw new Error('unused position');
    let cursor = start;
    let visited = 0;
    do {
      cursor = link.get(cursor);
      visited += 1;
    } while (cursor !== start && cursor !== undefined && visited <= link.size);
    if (cursor !== start || visited !== link.size) throw new Error('disconnected vertex fan');
  }
}

export function validateMesh(input, allowEmpty = false) {
  const mesh = weld(input);
  if (allowEmpty && !mesh.positions_mm.length && !mesh.triangles.length) return { mesh, volume: 0 };
  const { reason, volume, area } = checkMesh(mesh.positions_mm.flat(), mesh.triangles.flat(), 0);
  if (reason === 'mesh-open' || reason === 'mesh-orientation') throw new Error('inconsistent oriented mesh edges or volume');
  if (reason || !Number.isFinite(area) || !Number.isFinite(volume) || volume <= 0) {
    throw new Error('invalid mesh area or volume');
  }
  checkVertexFans(mesh);
  return { mesh, volume };
}

export function toFloat32Mesh(input) {
  const { mesh } = validateMesh(input);
  const positions = mesh.positions_mm.map(point => point.map(Math.fround));
  if (new Set(positions.map(point => point.join(','))).size !== positions.length) {
    throw new Error('float32 conversion collapses vertices');
  }
  return validateMesh({ positions_mm: positions, triangles: mesh.triangles }).mesh;
}

function construct({ Manifold, Mesh }, input) {
  const mesh = toFloat32Mesh(input);
  const source = new Mesh({
    numProp: 3,
    vertProperties: Float32Array.from(mesh.positions_mm.flat()),
    triVerts: Uint32Array.from(mesh.triangles.flat()),
  });
  const solid = new Manifold(source);
  try {
    const status = solid.status();
    if (status !== 'NoError' || !(solid.volume() > 0)) throw new Error(`invalid Manifold operand (${status})`);
    return solid;
  } catch (error) {
    solid.delete();
    throw error;
  }
}

export function readManifoldMesh(solid) {
  const status = solid.status();
  if (status !== 'NoError') throw new Error(`Manifold subtraction failed (${status})`);
  const output = solid.getMesh();
  const positions = [];
  for (let i = 0; i < output.vertProperties.length; i += output.numProp) {
    positions.push(Array.from(output.vertProperties.subarray(i, i + 3)));
  }
  for (const [i, from] of output.mergeFromVert.entries()) {
    const to = output.mergeToVert[i];
    if (!positions[from] || !positions[to] || positions[from].some((value, axis) => value !== positions[to][axis])) {
      throw new Error('Manifold merge changes vertex positions');
    }
  }
  const triangles = [];
  for (let i = 0; i < output.triVerts.length; i += 3) {
    triangles.push(Array.from(output.triVerts.subarray(i, i + 3)));
  }
  return validateMesh({ positions_mm: positions, triangles }, true).mesh;
}

export function subtractMesh(engine, a, b) {
  const left = construct(engine, a);
  try {
    const right = construct(engine, b);
    try {
      const cut = left.subtract(right);
      try {
        return readManifoldMesh(cut);
      } finally {
        cut.delete();
      }
    } finally {
      right.delete();
    }
  } finally {
    left.delete();
  }
}
