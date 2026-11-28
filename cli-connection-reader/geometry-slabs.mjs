/**
 * Independent difference oracle for convex decimal profiles extruded along Z.
 * All arrangement predicates and clipping endpoints use reduced rationals in
 * decimal-grid units. Numbers are used only for metric and artifact checks.
 */
const zero = [0n, 1n];
const one = [1n, 1n];
const next = (list, i) => list[(i + 1) % list.length];
const compare = (a, b) => {
  const d = a[0] * b[1] - b[0] * a[1];
  return d < 0n ? -1 : Number(d > 0n);
};

function gcd(a, b) {
  while (b) [a, b] = [b, a % b];
  return a < 0n ? -a : a;
}

function pointOn(a, b, [n, d]) {
  const x = a[0] * d + (b[0] - a[0]) * n;
  const y = a[1] * d + (b[1] - a[1]) * n;
  const factor = gcd(gcd(x, y), d);
  return [x / factor, y / factor, d / factor];
}

const key = p => p.join(',');
const edges = polygon => polygon.map((a, i) => [a, next(polygon, i)]);
const cross = ([x, y], [u, v]) => x * v - y * u;
const subtract = (a, b) => a.map((v, i) => v - b[i]);

function intersects([a, b], [c, d]) {
  const u = subtract(b, a);
  const v = subtract(d, c);
  const offset = subtract(c, a);
  const denominator = cross(u, v);
  if (denominator !== 0n) {
    const sign = denominator > 0n ? 1n : -1n;
    const t = [sign * cross(offset, v), sign * denominator];
    const s = [sign * cross(offset, u), sign * denominator];
    return [t, s].every(r => compare(r, zero) >= 0 && compare(r, one) <= 0) ? [t] : [];
  }
  if (cross(offset, u) !== 0n) return [];
  const axis = u[0] === 0n ? 1 : 0;
  const sign = u[axis] > 0n ? 1n : -1n;
  return [c, d].map(p => [sign * (p[axis] - a[axis]), sign * u[axis]])
    .filter(r => compare(r, zero) > 0 && compare(r, one) < 0);
}

function occupancySides(p, direction, polygon, charge) {
  let left = true;
  let right = true;
  for (const [a, b] of polygon) {
    charge();
    const edge = subtract(b, a);
    const sign = cross(edge, [p[0] - a[0] * p[2], p[1] - a[1] * p[2]]);
    if (sign < 0n) return [false, false];
    if (sign === 0n) {
      const derivative = edge[0] * direction[0] + edge[1] * direction[1];
      left &&= derivative > 0n;
      right &&= derivative < 0n;
    }
  }
  return [left, right];
}

function arrangement(a, b, charge) {
  const occupied = new Map();
  const removed = new Map();
  for (const [i, polygon] of [a, b].entries()) for (const edge of polygon) {
    const [start, end] = edge;
    const cuts = [zero, one];
    for (const other of i === 0 ? b : a) {
      charge();
      cuts.push(...intersects(edge, other));
    }
    cuts.sort(compare);
    for (let j = 1; j < cuts.length; j += 1) {
      const [u, v] = [cuts[j - 1], cuts[j]];
      if (compare(u, v) === 0) continue;
      const p = pointOn(start, end, [u[0] * v[1] + v[0] * u[1], 2n * u[1] * v[1]]);
      const direction = subtract(end, start);
      const [al, ar] = i === 0 ? [true, false] : occupancySides(p, direction, a, charge);
      const [bl, br] = i === 1 ? [true, false] : occupancySides(p, direction, b, charge);
      for (const [left, right, section] of [[al && !bl, ar && !br, occupied], [al && bl, ar && br, removed]]) {
        if (left === right) continue;
        const segment = [pointOn(start, end, u), pointOn(start, end, v)];
        if (right) segment.reverse();
        section.set(segment.map(key).join(';'), segment);
      }
    }
  }
  return [[...occupied.values()], [...removed.values()]];
}

function difference(a, b) {
  return [a[0] * b[2] - b[0] * a[2], a[1] * b[2] - b[1] * a[2]];
}

function section(edges, reject) {
  const heads = new Map();
  const tails = new Set();
  for (const [a, b] of edges) {
    if (heads.has(key(a)) || tails.has(key(b))) reject('mesh_invalid');
    heads.set(key(a), b);
    tails.add(key(b));
  }
  const result = [];
  if ([...tails].some(tail => !heads.has(tail))) reject('mesh_invalid');
  for (const [a] of edges) {
    if (!heads.has(key(a))) continue;
    const loop = [a];
    let cursor = a;
    do {
      const following = heads.get(key(cursor));
      if (!following) reject('mesh_invalid');
      heads.delete(key(cursor));
      loop.push(following);
      cursor = following;
    } while (key(cursor) !== key(a));
    loop.pop();
    const corners = loop.filter((p, i) => cross(difference(p, loop.at(i - 1)), difference(next(loop, i), p)) !== 0n);
    if (corners.length < 3) reject('mesh_invalid');
    result.push(corners.map((p, i) => [p, next(corners, i)]));
  }
  return result;
}

// Squared point-to-segment distance as an exact rational in grid units.
// Segment interiors in these arrangements cannot cross after splitting.
const dot = ([x, y], [u, v]) => x * u + y * v;
function pointDistance(p, a, b) {
  const u = difference(p, a);
  const v = difference(b, a);
  const projection = dot(u, v);
  const length2 = dot(v, v);
  if (projection <= 0n) return [dot(u, u), (p[2] * a[2]) ** 2n];
  if (projection * b[2] >= length2 * p[2]) {
    const w = difference(p, b);
    return [dot(w, w), (p[2] * b[2]) ** 2n];
  }
  return [cross(u, v) ** 2n, (p[2] * a[2]) ** 2n * length2];
}

function ratio(numerator, denominator) {
  const approximate = value => {
    const magnitude = value < 0n ? -value : value;
    const shift = Math.max(0, magnitude.toString(2).length - 54);
    return [Number(magnitude >> BigInt(shift)) * (value < 0n ? -1 : 1), shift];
  };
  const [n, ns] = approximate(numerator);
  const [d, ds] = approximate(denominator);
  return n / d * 2 ** (ns - ds);
}

function metricLower(squared, numeric) {
  // Bound rounding in the truncated mantissas, division, square root and
  // conversion from decimal grid units.
  return numeric(Math.sqrt(ratio(...squared))) * (1 - 8 * Number.EPSILON);
}

function gapBetween(a, b, numeric, charge) {
  let closest;
  for (const left of a) for (const right of b) {
    charge();
    if (intersects(left, right).length || intersects(right, left).length) return 0;
    const [u, v] = left.map(p => [...p, 1n]);
    const [w, x] = right.map(p => [...p, 1n]);
    for (const pair of [[u, w, x], [v, w, x], [w, u, v], [x, u, v]]) {
      const squared = pointDistance(...pair);
      if (!closest || compare(squared, closest) < 0) closest = squared;
    }
  }
  return metricLower(closest, numeric);
}

function minimumClearance(edges, numeric, charge) {
  let smallest;
  const measure = value => {
    if (!smallest || compare(value, smallest) < 0) smallest = value;
  };
  for (const [i, [a, b]] of edges.entries()) {
    const delta = difference(a, b);
    measure([dot(delta, delta), (a[2] * b[2]) ** 2n]);
    for (let j = i + 1; j < edges.length; j += 1) {
      charge();
      const [c, d] = edges[j];
      // Incident sides define a corner. Still measure their opposite endpoints,
      // which can approach the neighboring side at an acute corner.
      for (const [p, u, v] of [[a, c, d], [b, c, d], [c, a, b], [d, a, b]]) {
        if (key(p) !== key(u) && key(p) !== key(v)) {
          const value = pointDistance(p, u, v);
          if (value[0] !== 0n) measure(value);
        }
      }
    }
  }
  if (!smallest) return Infinity;
  return metricLower(smallest, numeric);
}

function sum(values) {
  let result = 0;
  let correction = 0;
  for (const term of values) {
    const corrected = term - correction;
    const next = result + corrected;
    correction = (next - result) - corrected;
    result = next;
  }
  return result;
}

const length = ([a, b]) => Math.hypot(b[0] - a[0], b[1] - a[1]);
function areaTerms(boundary, base, numeric) {
  return boundary.map(([a, b]) => {
    const relative = p => [p[0] - base[0] * p[2], p[1] - base[1] * p[2]];
    return numeric(numeric(ratio(cross(relative(a), relative(b)), a[2] * b[2]))) / 2;
  });
}

export function buildSlabs(a, b, base, shift, numeric, budget, disjoint, charge, reject) {
  const levels = [...new Set([...a.z, ...b.z])].filter(z => z >= a.z[0] && z <= a.z[1])
    .sort((x, y) => x < y ? -1 : Number(x > y));
  const whole = edges(a.xy).map(e => e.map(p => [...p, 1n]));
  const hasOverlapZ = a.z[0] < b.z[1] && b.z[0] < a.z[1];
  const [occupiedLoops, removedLoops] = !disjoint && hasOverlapZ
    ? arrangement(edges(a.xy), edges(b.xy), charge).map(e => section(e, reject))
    : [[whole], []];
  const occupied = occupiedLoops.flat();
  const removed = removedLoops.flat();
  const clearance = [Infinity, Infinity, Infinity];
  for (const boundary of new Set([whole, occupied, removed])) {
    const width = minimumClearance(boundary, numeric, charge);
    clearance[0] = Math.min(clearance[0], width);
    clearance[1] = clearance[0];
  }
  if (!disjoint && hasOverlapZ && !removed.length) {
    const gap = gapBetween(edges(a.xy), edges(b.xy), numeric, charge);
    if (gap > 0) {
      clearance[0] = Math.min(clearance[0], gap);
      clearance[1] = clearance[0];
    }
  }
  if (!disjoint && !hasOverlapZ) {
    const gap = a.z[1] <= b.z[0] ? b.z[0] - a.z[1] : a.z[0] - b.z[1];
    if (gap > 0n) {
      const planarGap = gapBetween(edges(a.xy), edges(b.xy), numeric, charge);
      // Crossing boundaries have zero gap. Containment also has zero
      // occupied-set gap even when the boundary edges are separate.
      const contains = (polygon, point) => edges(polygon).every(([u, v]) => {
        charge();
        return cross(subtract(v, u), subtract(point, u)) >= 0n;
      });
      if (planarGap === 0 || contains(a.xy, b.xy[0]) || contains(b.xy, a.xy[0])) {
        clearance[2] = numeric(gap) * (1 - 8 * Number.EPSILON);
      }
    }
  }
  const xy = p => p.slice(0, 2).map((v, i) => numeric(ratio(v - base[i] * p[2], p[2])) + shift[i]);
  const z = v => numeric(v - base[2]) + shift[2];
  const slabs = [];
  for (let i = 1; i < levels.length; i += 1) {
    const [low, high] = levels.slice(i - 1, i + 1);
    const cutting = removed.length > 0 && low >= b.z[0] && high <= b.z[1];
    const previous = slabs.at(-1);
    if (previous?.cutting === cutting) previous.high = high;
    else slabs.push({ low, high, cutting });
  }
  const loopSigns = occupiedLoops.map(loop => {
    const terms = areaTerms(loop, base, numeric);
    const signed = sum(terms);
    if (Math.abs(signed) <= 32 * Number.EPSILON * sum(terms.map(Math.abs))) reject('budget_exceeded');
    return Math.sign(signed);
  });
  const shells = { positive: 0, negative: 0 };
  for (const [i, slab] of slabs.entries()) {
    if (!slab.cutting) {
      if (i === 0 || !occupied.length) shells.positive += 1;
    } else if (slabs.length === 1) {
      shells.positive = loopSigns.filter(sign => sign > 0).length;
    } else if (i === 0 && occupied.length) {
      shells.positive += 1;
    } else if (i > 0 && i + 1 < slabs.length) {
      shells.negative += loopSigns.filter(sign => sign < 0).length;
    }
  }
  let min = [Infinity, Infinity, Infinity];
  let max = [-Infinity, -Infinity, -Infinity];
  let allowance = 0;
  let volumeError = 0;
  const epsilon = Math.hypot(...budget);
  const dimension = a.box.high.map((v, i) => v - a.box.low[i]);
  for (const slab of slabs) {
    const height = numeric(slab.high - slab.low);
    clearance[2] = Math.min(clearance[2], height * (1 - 8 * Number.EPSILON));
    slab.low = z(slab.low);
    slab.high = z(slab.high);
    const retained = slab.cutting ? occupied : whole;
    slab.boundary = retained.map(edge => edge.map(xy));
    slab.removed = slab.cutting ? removed.map(edge => edge.map(xy)) : [];
    const terms = areaTerms(retained, base, numeric);
    const removedTerms = areaTerms(slab.cutting ? removed : [], base, numeric);
    slab.volume = sum(terms) * height;
    slab.removedVolume = sum(removedTerms) * height;
    volumeError += 32 * Number.EPSILON * sum(terms.map(Math.abs)) * height;
    if (removedTerms.length && !(slab.removedVolume > 32 * Number.EPSILON * sum(removedTerms.map(Math.abs)) * height)) {
      reject('budget_exceeded');
    }
    if (!slab.boundary.length) continue;
    if (!(slab.volume > 32 * Number.EPSILON * sum(terms.map(Math.abs)) * height)) reject('budget_exceeded');
    for (const p of slab.boundary.flat()) {
      min = min.map((v, i) => Math.min(v, i === 2 ? slab.low : p[i]));
      max = max.map((v, i) => Math.max(v, i === 2 ? slab.high : p[i]));
    }
    // Cover the entire epsilon-neighborhood of each side and both caps by
    // expanded boxes. Overlaps only increase this volume bound.
    allowance += sum(slab.boundary.map(edge => 2 * epsilon * (length(edge) + 2 * epsilon) * (height + 2 * epsilon)));
    allowance += 4 * epsilon * (dimension[0] + 2 * epsilon) * (dimension[1] + 2 * epsilon);
  }
  const volume = sum(slabs.map(slab => slab.volume));
  return { slabs, shells, clearance, min, max, volume, volumeError,
    allowance: allowance + volumeError + Math.abs(volume) * 1e-12 };
}

function intervals(faces, z, charge, reject) {
  const result = [];
  for (const { loops, low, high, tangent } of faces) {
    charge();
    if (!(low < z && z < high)) continue;
    const hits = [];
    for (const loop of loops) for (const [i, a] of loop.entries()) {
      charge();
      const b = next(loop, i);
      if ((a[2] > z) === (b[2] > z)) continue;
      const up = a[2] < b[2];
      // Adjacent faces traverse shared edges in opposite directions. Start
      // from the lower endpoint in both faces so their intersections agree.
      const [bottom, top] = up ? [a, b] : [b, a];
      const p = bottom.slice(0, 2).map((v, axis) =>
        v + (top[axis] - v) * (z - bottom[2]) / (top[2] - bottom[2]));
      hits.push({ p, up, t: p[0] * tangent[0] + p[1] * tangent[1] });
    }
    hits.sort((a, b) => a.t - b.t);
    if (hits.length % 2 !== 0) reject('mesh_invalid');
    for (let i = 0; i < hits.length; i += 2) {
      if (hits[i].up || !hits[i + 1].up) reject('mesh_invalid');
      result.push([hits[i].p, hits[i + 1].p]);
    }
  }
  return result;
}

function line(edge) {
  const size = length(edge);
  const origin = edge[0];
  const unit = edge[1].map((v, i) => (v - origin[i]) / size);
  return {
    size, project: p => (p[0] - origin[0]) * unit[0] + (p[1] - origin[1]) * unit[1],
    distance: p => Math.abs((p[0] - origin[0]) * unit[1] - (p[1] - origin[1]) * unit[0]),
  };
}

function winding(edges, p, arithmetic, charge, reject) {
  let sum = 0;
  for (const [a, b] of edges) {
    charge();
    if ((a[1] > p[1]) === (b[1] > p[1])) continue;
    const [dx, dy] = subtract(b, a);
    const side = dx * (p[1] - a[1]) - dy * (p[0] - a[0]);
    if (Math.abs(side) <= arithmetic * (Math.abs(dx) + Math.abs(dy))) reject('budget_exceeded');
    if (dy > 0 && side > 0) sum += 1;
    if (dy < 0 && side < 0) sum -= 1;
  }
  return sum;
}

function compareSection(expected, actual, tolerance, arithmetic, clearance, charge, reject) {
  const refs = expected.map(edge => ({ ...line(edge), intervals: [] }));
  for (const [a, b] of actual) {
    if (length([a, b]) <= arithmetic) continue;
    const matches = [];
    for (const ref of refs) {
      charge();
      const lo = ref.project(a);
      const hi = ref.project(b);
      if (hi > lo && lo >= -tolerance && hi <= ref.size + tolerance &&
          ref.distance(a) <= tolerance && ref.distance(b) <= tolerance) matches.push([ref, lo, hi]);
    }
    if (matches.length !== 1) reject('mesh_invalid');
    const [[ref, lo, hi]] = matches;
    ref.intervals.push([lo, hi]);
  }
  // Overlay all directed boundary intervals. There must be one traversal of
  // every reference side, without a gap, a reversed side or duplicate coverage
  // beyond the endpoint allowance. Closed boundaries with this orientation
  // determine winding in every cell away from the boundary strips.
  for (const ref of refs) {
    ref.intervals.sort((a, b) => a[0] - b[0]);
    let cursor = 0;
    for (const [lo, hi] of ref.intervals) {
      if (Math.abs(lo - cursor) > tolerance || hi <= cursor - arithmetic) reject('mesh_invalid');
      cursor = hi;
    }
    if (Math.abs(cursor - ref.size) > tolerance) reject('mesh_invalid');
  }
  // Each open cell meets a directed side, unless the section is empty.
  // Check both incident cells using midpoint samples outside the possible
  // boundary displacement. The coverage test above rules out extra loops.
  for (const [a, b] of expected) {
    const [dx, dy] = subtract(b, a);
    const size = Math.hypot(dx, dy);
    const distance = Math.min(size, clearance) / 4;
    const middle = a.map((v, i) => (v + b[i]) / 2);
    for (const sign of [-1, 1]) {
      const sample = [middle[0] - sign * distance * dy / size, middle[1] + sign * distance * dx / size];
      const expectedWinding = winding(expected, sample, arithmetic, charge, reject);
      if (expectedWinding !== Number(sign > 0)) reject('budget_exceeded');
      if (winding(actual, sample, arithmetic, charge, reject) !== expectedWinding) reject('mesh_invalid');
    }
  }
}

function compareCaps(transition, tolerance, arithmetic, charge, reject) {
  const actual = new Map();
  for (const loop of transition.faces) for (const [i, point] of loop.entries()) {
    charge();
    const a = point.slice(0, 2);
    const b = next(loop, i).slice(0, 2);
    const id = [a, b].map(key).join(';');
    const reversed = [b, a].map(key).join(';');
    if (actual.has(reversed)) actual.delete(reversed);
    else {
      if (actual.has(id)) reject('mesh_invalid');
      actual.set(id, [a, b]);
    }
  }
  const expected = [...transition.below, ...transition.above.map(([a, b]) => [b, a])];
  const supports = [];
  for (const edge of expected) {
    const [a, b] = edge;
    const existing = supports.find(ref => {
      charge();
      return ref.distance(a) <= arithmetic && ref.distance(b) <= arithmetic;
    });
    if (existing) {
      const start = existing.project(a);
      const end = existing.project(b);
      existing.spans.push([Math.min(start, end), Math.max(start, end)]);
    } else {
      const ref = line(edge);
      supports.push({ ...ref, spans: [[0, ref.size]], events: [] });
    }
  }
  // A cap edge can cover adjacent collinear segments. Keep gaps between
  // separate spans so a nearby edge cannot match through an empty interval.
  for (const ref of supports) {
    const spans = [];
    for (const [lo, hi] of ref.spans.sort((a, b) => a[0] - b[0])) {
      const last = spans.at(-1);
      if (last && lo <= last[1] + arithmetic) last[1] = Math.max(last[1], hi);
      else spans.push([lo, hi]);
    }
    ref.spans = spans;
  }
  for (const [edges, sign] of [[expected, 1], [[...actual.values()], -1]]) {
    for (const [a, b] of edges) {
      if (length([a, b]) <= arithmetic) continue;
      const matches = supports.filter(ref => {
        charge();
        const bound = sign > 0 ? arithmetic : tolerance;
        if (ref.distance(a) > bound || ref.distance(b) > bound) return false;
        const start = ref.project(a);
        const end = ref.project(b);
        const lo = Math.min(start, end);
        const hi = Math.max(start, end);
        return ref.spans.some(([from, to]) => {
          charge();
          return lo >= from - bound && hi <= to + bound &&
            Math.min(hi, to) - Math.max(lo, from) > arithmetic;
        });
      });
      if (matches.length !== 1) reject('mesh_invalid');
      const [ref] = matches;
      ref.events.push([ref.project(a), sign], [ref.project(b), -sign]);
    }
  }
  for (const { events } of supports) {
    events.sort((a, b) => a[0] - b[0]);
    let winding = 0;
    for (let i = 0; i < events.length;) {
      const anchor = events[i][0];
      do {
        winding += events[i][1];
        i += 1;
      } while (i < events.length && events[i][0] - anchor <= tolerance);
      if (winding !== 0) reject('mesh_invalid');
    }
  }
}

function normalOf(loop) {
  const origin = loop[0];
  const terms = loop.map((p, i) => {
    const [x, y, z] = subtract(p, origin);
    const [u, v, w] = subtract(next(loop, i), origin);
    return [y * w - z * v, z * u - x * w, x * v - y * u];
  });
  return [0, 1, 2].map(axis => sum(terms.map(term => term[axis])));
}

function boundaryShells(boundary, charge, reject) {
  const owners = new Map();
  const parent = boundary.map((_, i) => i);
  const root = i => {
    while (parent[i] !== i) {
      parent[i] = parent[parent[i]];
      i = parent[i];
    }
    return i;
  };
  const volumes = boundary.map((face, i) => {
    const terms = [];
    for (const loop of face) {
      for (const p of loop) {
        charge();
        const id = key(p);
        if (owners.has(id)) parent[root(i)] = root(owners.get(id));
        else owners.set(id, i);
      }
      const [a] = loop;
      for (let j = 1; j + 1 < loop.length; j += 1) {
        const [b, c] = [loop[j], loop[j + 1]];
        terms.push((a[0] * (b[1] * c[2] - b[2] * c[1]) +
                    a[1] * (b[2] * c[0] - b[0] * c[2]) +
                    a[2] * (b[0] * c[1] - b[1] * c[0])) / 6);
      }
    }
    return [sum(terms), sum(terms.map(Math.abs))];
  });
  const shells = new Map();
  for (const [i, pair] of volumes.entries()) {
    const id = root(i);
    if (!shells.has(id)) shells.set(id, []);
    shells.get(id).push(pair);
  }
  let positive = 0;
  let negative = 0;
  for (const pieces of shells.values()) {
    const signed = sum(pieces.map(([net]) => net));
    const bound = 32 * Number.EPSILON * sum(pieces.map(([, absolute]) => absolute));
    if (Math.abs(signed) <= bound) reject('budget_exceeded');
    if (signed > 0) positive += 1;
    else negative += 1;
  }
  return { positive, negative };
}

export function checkSlabs(oracle, boundary, volume, budget, arithmetic, charge, reject) {
  if (oracle.volume === 0) {
    if (boundary.length || volume !== 0) reject('mesh_invalid');
    return;
  }
  if (!(volume > 0) || Math.abs(volume - oracle.volume) > oracle.allowance) reject('mesh_invalid');
  const shells = boundaryShells(boundary, charge, reject);
  if (shells.positive !== oracle.shells.positive || shells.negative !== oracle.shells.negative) reject('mesh_invalid');
  const xyTolerance = Math.hypot(budget[0], budget[1]);
  const zTolerance = budget[2];
  const sides = [];
  const transitions = [];
  const candidateZ = new Set();
  const min = [Infinity, Infinity, Infinity];
  const max = [-Infinity, -Infinity, -Infinity];
  const supports = oracle.slabs.flatMap(slab => slab.boundary.map(line));
  for (const [i, slab] of oracle.slabs.entries()) {
    const previous = oracle.slabs[i - 1];
    if (previous?.boundary.length || slab.boundary.length) {
      transitions.push({ z: slab.low, below: previous?.boundary ?? [], above: slab.boundary, faces: [] });
    }
  }
  const last = oracle.slabs.at(-1);
  if (last.boundary.length) transitions.push({ z: last.high, below: last.boundary, above: [], faces: [] });
  for (const loops of boundary) {
    let low = Infinity;
    let high = -Infinity;
    for (const loop of loops) {
      charge(loop.length);
      for (const [x, y, z] of loop) {
        min[0] = Math.min(min[0], x);
        min[1] = Math.min(min[1], y);
        min[2] = Math.min(min[2], z);
        max[0] = Math.max(max[0], x);
        max[1] = Math.max(max[1], y);
        max[2] = Math.max(max[2], z);
        low = Math.min(low, z);
        high = Math.max(high, z);
        candidateZ.add(z);
      }
    }
    const [nx, ny, nz] = normalOf(loops[0]);
    if (high - low <= 2 * zTolerance && Math.abs(nz) > Math.hypot(nx, ny)) {
      const transition = transitions.find(({ z }) => Math.max(Math.abs(z - low), Math.abs(z - high)) <= zTolerance);
      if (!transition) reject('mesh_invalid');
      transition.faces.push(...loops);
      continue;
    }
    const tangent = [-ny, nx];
    if (Math.hypot(...tangent) === 0 || !supports.some(ref => {
      charge();
      return loops.every(loop => loop.every(p => {
        charge();
        return ref.distance(p) <= xyTolerance;
      }));
    })) reject('mesh_invalid');
    sides.push({ loops, low, high, tangent });
  }
  if (min.some((v, i) => Math.abs(v - oracle.min[i]) > budget[i]) ||
      max.some((v, i) => Math.abs(v - oracle.max[i]) > budget[i])) reject('mesh_invalid');
  for (const transition of transitions) compareCaps(transition, xyTolerance, arithmetic, charge, reject);
  // Section edges are affine between vertex levels. Test between all vertex
  // events outside the transition bands, including subdivisions introduced
  // by tessellation. The side-plane check bounds displacement between samples.
  const levels = [...candidateZ].sort((a, b) => a - b);
  for (const slab of oracle.slabs) {
    const low = slab.low + zTolerance;
    const high = slab.high - zTolerance;
    const breaks = [low, ...levels.filter(z => z > low && z < high), high];
    for (let i = 1; i < breaks.length; i += 1) {
      if (breaks[i] - breaks[i - 1] <= arithmetic) continue;
      const z = (breaks[i] + breaks[i - 1]) / 2;
      compareSection(slab.boundary, intervals(sides, z, charge, reject), xyTolerance, arithmetic,
        oracle.clearance[0], charge, reject);
    }
  }
}
