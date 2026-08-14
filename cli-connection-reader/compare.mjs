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

const row = (status, o, extra = {}) => ({
  status, id: o.globalId, name: o.name, ifcType: o.ifcType, storey: o.storey,
  profile: o.profile, material: o.material, centroid: o.centroid, ...extra,
});

/**
 * The change list, or a refusal.
 *
 * THE REFUSAL IS THE LOAD-BEARING PART. With no geometry fallback, "everything added and everything
 * removed" is the precise symptom of an exporter that regenerated every GlobalId — and reporting it as
 * a change list is not a weak answer but a confident lie about somebody else's building. So when the
 * ids paired NOTHING while both files hold objects, this returns the refusal and NO `changes` at all.
 * Returning the list alongside a flag was rejected: a caller would render it.
 *
 * ONE PAIR IS ENOUGH TO PROCEED. The ids either carry signal or they do not, and a single shared id
 * says they do — a project genuinely can replace all but one object between revisions.
 *
 * AN EMPTY SIDE IS NOT A REFUSAL — UNLESS IT IS EMPTY FOR THE WRONG REASON. A first version against an
 * empty file pairs nothing for a reason already visible in the counts. But `readModel` DROPS products
 * carrying no drawable triangle and reports them as `skipped`, so a side can arrive as `[]` while the
 * file is full of objects — and reporting that as a wholesale deletion is the same lie by another road.
 * `skipped` is therefore an input, not decoration.
 */
export function diffObjects(baseObjects, revisedObjects, opts = {}) {
  const tolerance = Number.isFinite(opts.tolerance) ? opts.tolerance : 1;
  const skipped = { base: opts.skipped?.base ?? 0, revised: opts.skipped?.revised ?? 0 };
  const base = partitionByUsableId(baseObjects);
  const revised = partitionByUsableId(revisedObjects);

  const identity = {
    base: { objects: baseObjects.length, usableIds: base.usable.size, skipped: skipped.base, uncomparable: base.uncomparable },
    revised: { objects: revisedObjects.length, usableIds: revised.usable.size, skipped: skipped.revised, uncomparable: revised.uncomparable },
    paired: 0, added: 0, removed: 0,
  };

  let paired = 0;
  for (const id of base.usable.keys()) if (revised.usable.has(id)) paired++;

  // An "empty" side that is empty because everything was skipped is a READER outcome, not a model fact,
  // and the two get different sentences because only one of them is repairable.
  const emptyBySkip = (objs, n) => objs.length === 0 && n > 0;
  if (emptyBySkip(baseObjects, skipped.base) || emptyBySkip(revisedObjects, skipped.revised)) {
    identity.refused = {
      reason: 'every object in one of these files could not be drawn, so nothing could be compared',
      baseObjects: baseObjects.length, revisedObjects: revisedObjects.length,
      skipped, uncomparable: base.uncomparable.count + revised.uncomparable.count,
    };
    return { identity, summary: null, criteria: CRITERIA };
  }

  if (paired === 0 && baseObjects.length > 0 && revisedObjects.length > 0) {
    identity.refused = {
      reason: 'no object in either file shares a usable IFC id with the other',
      baseObjects: baseObjects.length,
      revisedObjects: revisedObjects.length,
      skipped,
      uncomparable: base.uncomparable.count + revised.uncomparable.count,
    };
    return { identity, summary: null, criteria: CRITERIA };
  }

  const changes = [];
  let unchanged = 0;
  for (const [id, b] of base.usable) {
    const r = revised.usable.get(id);
    if (!r) { changes.push(row('removed', b)); identity.removed++; continue; }
    const fired = changedBy(b, r, tolerance);
    if (fired.length === 0) {
      unchanged++;
      if (opts.includeUnchanged) changes.push(row('unchanged', r));
      continue;
    }
    const extra = { changedBy: fired };
    if (fired.includes('location') && b.centroid && r.centroid) {
      extra.delta = [r.centroid[0] - b.centroid[0], r.centroid[1] - b.centroid[1], r.centroid[2] - b.centroid[2]];
      extra.distance = Math.hypot(...extra.delta);
    }
    if (fired.includes('properties')) {
      const keys = new Set([...Object.keys(b.properties), ...Object.keys(r.properties)]);
      extra.fields = [...keys].filter((k) => b.properties[k] !== r.properties[k])
        .map((k) => ({ name: k, from: b.properties[k] ?? null, to: r.properties[k] ?? null }));
    }
    changes.push(row('changed', r, extra));
  }
  for (const [id, r] of revised.usable) if (!base.usable.has(id)) { changes.push(row('added', r)); identity.added++; }

  identity.paired = paired;
  return {
    identity,
    changes,
    criteria: CRITERIA,
    summary: {
      added: identity.added,
      removed: identity.removed,
      changed: changes.filter((c) => c.status === 'changed').length,
      unchanged,
      uncomparable: base.uncomparable.count + revised.uncomparable.count,
    },
  };
}
