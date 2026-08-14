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
