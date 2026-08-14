/**
 * compare — match the objects of two versions of one IFC by GlobalId, and say what changed.
 *
 * PURE AND WEB-IFC-FREE ON PURPOSE. Everything here takes plain objects, so the identity rules and the
 * refusal — the two things that decide whether this feature tells the truth — are unit-testable without
 * a WASM parser or a sample file. `index.mjs` owns the I/O and hands this arrays.
 *
 * GLOBALID ONLY, and that is a researched decision rather than a shortcut: IfcOpenShell's `ifcdiff`
 * matches on GlobalId and nothing else, Tekla matches on GUID and falls back to MANUAL LINKING, and
 * Solibri — which does run a geometry fallback — shipped a checkbox in 9.7.15 to turn it off. See the
 * design doc §5.0. When ids cannot answer, this refuses (`diffObjects`) rather than guessing.
 */

/** Every property flattened to `SetName.PropertyName` so a changed field can be NAMED, not just counted. */
function flattenProperties(sets) {
  const out = {};
  for (const set of sets ?? []) {
    for (const p of set.properties ?? []) out[`${set.name}.${p.name}`] = p.value;
  }
  return out;
}

/**
 * Reduce a `read-model` object to the small record the comparison needs.
 *
 * DROPS `positions` AND `indices`, which is the point: `compare` streams both models through this and
 * would otherwise hold two complete tessellations at once — the memory bill the design rejected an
 * overlay diff and a floless-side diff for. What survives is a fixed handful of numbers per object.
 */
export function comparableFrom(o) {
  const p = o.positions ?? [];
  const n = p.length / 3;
  let centroid = null;
  let extents = null;
  if (n > 0) {
    let sx = 0, sy = 0, sz = 0;
    let minX = Infinity, minY = Infinity, minZ = Infinity;
    let maxX = -Infinity, maxY = -Infinity, maxZ = -Infinity;
    for (let i = 0; i < p.length; i += 3) {
      const x = p[i], y = p[i + 1], z = p[i + 2];
      sx += x; sy += y; sz += z;
      if (x < minX) minX = x; if (x > maxX) maxX = x;
      if (y < minY) minY = y; if (y > maxY) maxY = y;
      if (z < minZ) minZ = z; if (z > maxZ) maxZ = z;
    }
    centroid = [sx / n, sy / n, sz / n];
    // SORTED, so a rotation by a multiple of 90 degrees about a coordinate axis is not a shape change —
    // a member turned on its side is the same member, moved. An ARBITRARY rotation does change an
    // axis-aligned box, so a beam at 30 degrees fires `geometry` too; that over-reports and never hides,
    // and the general case needs a rotation-invariant descriptor, which is a research problem.
    extents = [maxX - minX, maxY - minY, maxZ - minZ].sort((a, b) => a - b);
  }
  return {
    // `globalId`, NEVER `o.id` — `id` falls back to the expressID, and two exports of one model share
    // many, so keying on it fabricates matches between unrelated objects. See `readModel`'s object
    // literal, where the two fields are deliberately separated.
    globalId: o.globalId ?? null,
    name: o.name ?? null,
    ifcType: o.ifcType ?? null,
    storey: o.storey ?? null,
    profile: o.profile ?? null,
    material: o.material ?? null,
    centroid,
    extents,
    triangles: Math.floor((o.indices?.length ?? 0) / 3),
    properties: flattenProperties(o.propertySets),
  };
}

/**
 * Split a file's objects into "has an id we can match on" and "does not".
 *
 * BOTH HALVES OF "USABLE" COME FROM AN EXISTING RULE — floless's `revalidateSubSelection` already
 * refuses a blank id and a duplicated one, for the reasons its header sets out: in a file where every
 * id is `''` the check waves through whatever landed at that slot, and two objects sharing an id make
 * "the id still matches" true after a reorder that swapped them. An id that cannot distinguish anything
 * does not get to vote.
 *
 * A DUPLICATED ID DISQUALIFIES EVERY COPY, not the second one onward. Keeping the first would be
 * choosing arbitrarily between two objects the file gives us no way to tell apart, and the whole point
 * of this module is not doing that.
 */
export function partitionByUsableId(objects) {
  const seen = new Map();
  for (const o of objects) {
    if (!o.globalId) continue;
    seen.set(o.globalId, (seen.get(o.globalId) ?? 0) + 1);
  }
  const usable = new Map();
  const uncomparable = { count: 0, blank: 0, duplicated: 0, byType: {}, byStorey: {} };
  for (const o of objects) {
    if (o.globalId && seen.get(o.globalId) === 1) { usable.set(o.globalId, o); continue; }
    uncomparable.count++;
    if (o.globalId) uncomparable.duplicated++; else uncomparable.blank++;
    const t = o.ifcType ?? 'UNKNOWN';
    const s = o.storey ?? 'UNKNOWN';
    uncomparable.byType[t] = (uncomparable.byType[t] ?? 0) + 1;
    uncomparable.byStorey[s] = (uncomparable.byStorey[s] ?? 0) + 1;
  }
  return { usable, uncomparable };
}

/**
 * WHAT COUNTS AS A CHANGE — borrowed from Tekla's "comparison set", which exists because a diff that
 * reports every difference reports mostly noise (an export timestamp, a 0.1 mm float wobble).
 *
 * FIXED in this slice and ECHOED in the output. Tekla makes it user-configurable and will eventually be
 * right to; until then the echo is what stops a change list stored today from meaning something
 * different when the set grows.
 */
export const CRITERIA = ['location', 'geometry', 'ifcType', 'name', 'profile', 'material', 'properties'];

const dist = (a, b) => (a && b ? Math.hypot(a[0] - b[0], a[1] - b[1], a[2] - b[2]) : 0);

/** Same keys, same values — and the key set is compared BOTH ways so a DELETED property is a change. */
function propertiesDiffer(a, b) {
  const keys = new Set([...Object.keys(a), ...Object.keys(b)]);
  for (const k of keys) if (a[k] !== b[k]) return true;
  return false;
}

/**
 * Which criteria fired, in `CRITERIA` order. Empty means unchanged.
 *
 * `tolerance` applies to BOTH location and geometry: the same float noise that moves a centroid moves
 * an extent, and applying it to one but not the other would make a file that passes the location check
 * fail the shape check for the same underlying wobble.
 */
export function changedBy(a, b, tolerance) {
  const fired = [];
  if (dist(a.centroid, b.centroid) > tolerance) fired.push('location');
  const shapeMoved = (a.extents && b.extents)
    ? a.extents.some((v, i) => Math.abs(v - b.extents[i]) > tolerance)
    : Boolean(a.extents) !== Boolean(b.extents);
  if (shapeMoved || a.triangles !== b.triangles) fired.push('geometry');
  if (a.ifcType !== b.ifcType) fired.push('ifcType');
  if (a.name !== b.name) fired.push('name');
  if (a.profile !== b.profile) fired.push('profile');
  if (a.material !== b.material) fired.push('material');
  if (propertiesDiffer(a.properties, b.properties)) fired.push('properties');
  return fired;
}
