#!/usr/bin/env node
// aware-connection-reader — AWARE cli-transport bridge.
//
// Reads a steel connection out of an IFC file as tessellated geometry, so a connection designed
// in any tool (IDEA StatiCa, Tekla, …) can be imported into a model. Geometry parsing is done by
// the web-ifc WASM engine (bundled) — this bridge only groups elements into connection units and
// maps web-ifc's tessellation into AWARE's generic `mesh` scene primitive (`positions`+`indices`,
// canonical millimetres). It never re-implements IFC geometry.
//
// Protocol (the AWARE CliInvoker contract): argv = [command, "--json-stdin"]; the JSON args object
// arrives on stdin; exactly one JSON result object is written to stdout; a non-zero exit + a stderr
// message signals failure. web-ifc's own diagnostics go to stderr, so stdout stays pure JSON.
//
// Commands:
//   list      inputs { ifc-path }                    -> { connections: [ {id,name,type,plates,bolts,welds,members} ] }
//             Fast: enumerate connection candidates (one per IfcElementAssembly that carries
//             connection hardware) WITHOUT tessellating — this backs the "which connection?" picker.
//   extract   inputs { ifc-path, id }                -> { connection: {id,name,type,frame,members,parts:[mesh…],recipe?} }
//             Tessellate ONE candidate (by its IfcElementAssembly GlobalId) into mesh scene parts, AND —
//             when the parts match a supported pattern (a base plate with a vertical anchor grid) — fit a
//             parametric `recipe:{kind,params}` so the consumer can import it as an EDITABLE recipe rather
//             than opaque mesh. `parts` is always returned as the fallback; `recipe` only when confident.

import { closeSync, mkdirSync, openSync, readFileSync, renameSync, statSync, writeSync } from 'node:fs';
import { dirname, basename, sep } from 'node:path';
import { randomUUID } from 'node:crypto';
import { pathToFileURL } from 'node:url';
import { createRequire } from 'node:module';
import { unzipSync } from 'fflate'; // tiny pure-JS unzip for .ifczip inputs
import * as WebIFC from 'web-ifc'; // package export resolves to the node build (auto-locates its .wasm)
import { recognizeBasePlate, recognizeShearPlate } from './recognize.mjs'; // fit a parametric recipe from the tessellated parts
import { emitProgress, progressEnabled, progressRecordBytes, MAX_RECORD_BYTES } from './progress.mjs'; // tell the runtime what a long read is doing, while it does it
import { comparableFrom, diffObjects } from './compare.mjs'; // match two versions of one file by GlobalId, and refuse when nothing pairs

// web-ifc returns geometry in metres (SI base unit); AWARE scenes are canonical millimetres.
const M_TO_MM = 1000;

/**
 * Are we running as the packaged single-file executable (Node SEA) rather than plain `node`?
 *
 * Asks Node directly. An earlier cut inferred it from `basename(process.execPath)` not starting with
 * "node", which is not a reliable SEA test in either direction: a packaged binary renamed
 * `node-reference-reader.exe` would be taken for plain Node — and since `import.meta` is inert in the
 * CJS bundle, the entry guard below would then be false and the exe would exit cleanly having done
 * NOTHING. The same misclassification also skipped the sibling-.wasm path. `node:sea.isSea()` is exact.
 */
function isPackagedExe() {
  const sea = loadSea();
  if (sea && typeof sea.isSea === 'function') return sea.isSea();
  // Fallback for a Node too old for node:sea (added 20.12/21.7). Only reached in plain-ESM dev runs,
  // where `import.meta.url` IS live and gives the entry guard a real second signal.
  return !basename(process.execPath).toLowerCase().startsWith('node');
}

// `node:sea` must be loaded through whichever module system we ended up in: the shipped bundle is
// CJS (where a global `require` exists and `import.meta` does not), the dev entry is ESM (where the
// reverse holds). Top-level `await import()` would be the tidy answer but esbuild cannot emit it for
// a CJS output format, so this branches instead.
function loadSea() {
  try {
    if (typeof require === 'function') return require('node:sea');
  } catch { /* no node:sea on this runtime */ }
  try {
    return createRequire(import.meta.url)('node:sea');
  } catch { /* ESM on a Node without node:sea */ }
  return null;
}

// Connection-hardware element types (the parts that MAKE a connection) and the members it sits on.
const HARDWARE = new Map([
  [WebIFC.IFCPLATE, 'plate'],
  [WebIFC.IFCMECHANICALFASTENER, 'bolt'],
  [WebIFC.IFCFASTENER, 'weld'],
]);
const MEMBER_TYPES = new Set([WebIFC.IFCBEAM, WebIFC.IFCCOLUMN, WebIFC.IFCMEMBER]);

function readStdin() {
  return readFileSync(0, 'utf8'); // fd 0
}

function strOf(v) {
  return v && typeof v === 'object' && 'value' in v ? v.value : v;
}

// Pull the first *.ifc entry out of an .ifczip (a ZIP container) as raw bytes.
function unzipInnerIfc(zipBytes, ifcPath) {
  const files = unzipSync(zipBytes);
  const name = Object.keys(files).find((n) => /\.ifc$/i.test(n));
  if (!name) throw new Error(`${ifcPath}: .ifczip archive contains no .ifc entry`);
  return files[name];
}

async function openModel(api, ifcPath) {
  let bytes = new Uint8Array(readFileSync(ifcPath));
  // An .ifczip is a ZIP holding the .ifc; web-ifc's OpenModel wants raw IFC SPF bytes, so unzip and
  // hand it the inner .ifc. Detect by the ZIP magic ("PK\x03\x04") rather than the extension, since a
  // caller's path casing/extension can't be trusted.
  if (bytes[0] === 0x50 && bytes[1] === 0x4b && bytes[2] === 0x03 && bytes[3] === 0x04) {
    bytes = unzipInnerIfc(bytes, ifcPath);
  }
  // COORDINATE_TO_ORIGIN:false keeps model coordinates (we place the connection ourselves);
  // USE_FAST_BOOLS lets web-ifc resolve the IfcBooleanClippingResult copes.
  return api.OpenModel(bytes, { COORDINATE_TO_ORIGIN: false, USE_FAST_BOOLS: true });
}

// Map IfcElementAssembly -> its aggregated child express ids (via IfcRelAggregates).
function assemblyChildren(api, modelID) {
  const byAssembly = new Map();
  const rels = api.GetLineIDsWithType(modelID, WebIFC.IFCRELAGGREGATES);
  for (let i = 0; i < rels.size(); i++) {
    const rel = api.GetLine(modelID, rels.get(i));
    const parent = rel.RelatingObject && rel.RelatingObject.value;
    if (parent == null) continue;
    const kids = (rel.RelatedObjects || []).map((o) => o.value);
    const prev = byAssembly.get(parent);
    if (prev) prev.push(...kids);
    else byAssembly.set(parent, kids);
  }
  return byAssembly;
}

// Classify one assembly's children into hardware (plate/bolt/weld) + member references. `beamMembers` is
// the members that are NOT columns — a fin plate hangs off the supported BEAM, and IfcRelAggregates ordering
// isn't reliably beam-first, so recognizeShearPlate's advisory `main` is corrected from this by IFC type.
function classify(api, modelID, childIds) {
  const hardware = []; // { expressID, role }
  const members = []; // member GlobalId strings
  const beamMembers = []; // member GlobalIds that aren't columns (the supported member a shear plate hangs off)
  for (const cid of childIds) {
    const t = api.GetLineType(modelID, cid);
    const role = HARDWARE.get(t);
    if (role) {
      hardware.push({ expressID: cid, role });
    } else if (MEMBER_TYPES.has(t)) {
      const line = api.GetLine(modelID, cid);
      const gid = strOf(line.GlobalId);
      members.push(gid);
      if (t !== WebIFC.IFCCOLUMN) beamMembers.push(gid);
    }
  }
  return { hardware, members, beamMembers };
}

// Human label for an assembly. Tekla stamps Name = the family (COLUMN/BEAM/ANGLE) and Tag = the
// mark (C102, B170, …); prefer "Name Tag" so the picker reads like the drawing ("COLUMN C102").
function assemblyLabel(asm) {
  const name = strOf(asm.Name);
  const tag = strOf(asm.Tag);
  if (name && tag) return `${name} ${tag}`;
  return name || tag || strOf(asm.ObjectType) || 'Connection';
}

// Exported so `extract.test.mjs` can drive the real pipeline against the in-repo fixtures — the
// frame these two produce is not observable from `recognize.mjs` alone (#347).
export function listConnections(api, modelID) {
  const kids = assemblyChildren(api, modelID);
  const asmIds = api.GetLineIDsWithType(modelID, WebIFC.IFCELEMENTASSEMBLY);
  const out = [];
  for (let i = 0; i < asmIds.size(); i++) {
    const aid = asmIds.get(i);
    const { hardware, members } = classify(api, modelID, kids.get(aid) || []);
    if (hardware.length === 0) continue; // not a connection (no plates/bolts/welds)
    const asm = api.GetLine(modelID, aid);
    const counts = { plate: 0, bolt: 0, weld: 0 };
    for (const h of hardware) counts[h.role]++;
    out.push({
      id: strOf(asm.GlobalId),
      name: assemblyLabel(asm),
      type: strOf(asm.ObjectType) || null,
      plates: counts.plate,
      bolts: counts.bolt,
      welds: counts.weld,
      members: members.length,
    });
  }
  return { connections: out };
}

// Tessellate the given express ids (one pass over the model's meshes, filtered to our set).
// Returns [{ id, role, positions:[mm…], indices:[…] }] with positions transformed to world mm.
function tessellate(api, modelID, wantById) {
  const parts = [];
  api.StreamAllMeshesWithTypes(modelID, [...HARDWARE.keys()], (flatMesh) => {
    const role = wantById.get(flatMesh.expressID);
    if (!role) return;
    const positions = [];
    const indices = [];
    const geoms = flatMesh.geometries;
    for (let i = 0; i < geoms.size(); i++) {
      const pg = geoms.get(i);
      const geom = api.GetGeometry(modelID, pg.geometryExpressID);
      const verts = api.GetVertexArray(geom.GetVertexData(), geom.GetVertexDataSize()); // [x,y,z,nx,ny,nz]*
      const idx = api.GetIndexArray(geom.GetIndexData(), geom.GetIndexDataSize());
      const m = pg.flatTransformation; // 4x4 column-major, metres, in web-ifc's Y-up frame
      const base = positions.length / 3;
      for (let v = 0; v < verts.length; v += 6) {
        const x = verts[v], y = verts[v + 1], z = verts[v + 2];
        // Placement transform and the Y-up -> Z-up rotation in one step, identical to
        // `readModel` below — the inverse of `toWebIfcYUp`, folded into the matrix
        // multiply so the hot loop allocates nothing.
        //
        // #347: `extract` used to hand web-ifc's frame straight through while
        // `read-model` and `probe` answered in the file's own Z-up one. One binary,
        // two frames, and the difference lived only in prose — so a consumer that
        // imported a recipe from one command and a mesh from the other got a
        // connection lying on its side. `ifc.write` has no `meta.up` knob either, so
        // `extract` -> `ifc.write` had ALWAYS written it sideways; that composition is
        // correct for free now.
        positions.push(
          (m[0] * x + m[4] * y + m[8] * z + m[12]) * M_TO_MM,
          -(m[2] * x + m[6] * y + m[10] * z + m[14]) * M_TO_MM,
          (m[1] * x + m[5] * y + m[9] * z + m[13]) * M_TO_MM,
        );
      }
      // Winding survives untouched: the rotation has determinant +1, so it cannot turn
      // a front face into a back face. (A mirror would, and would need an index flip.)
      for (let k = 0; k < idx.length; k++) indices.push(base + idx[k]);
      geom.delete();
    }
    if (positions.length >= 9 && indices.length >= 3) {
      const line = api.GetLine(modelID, flatMesh.expressID);
      parts.push({ id: strOf(line.GlobalId) || String(flatMesh.expressID), role, positions, indices });
    }
  });
  return parts;
}

// ---------------------------------------------------------------------------------------------
// probe / read-model — the whole file as a REFERENCE MODEL (borrowed geometry a consumer overlays
// but never owns), as opposed to list/extract's one-connection-to-import job.
// ---------------------------------------------------------------------------------------------

/**
 * The declared LENGTHUNIT as `{ declared, id }` — the label for humans, the express id so the scale
 * can be resolved. Both null when the file declares no length unit.
 *
 * `declared` is what the file SAYS its length unit is, verbatim: "MILLI.METRE" | "METRE" | … | null.
 * It is PROVENANCE, not a conversion factor for geometry. Measured 2026-07-25: web-ifc reads
 * IfcUnitAssignment itself and normalises tessellated geometry to metres before we ever see a vertex,
 * so nothing downstream may multiply mesh coordinates by it. It exists so a user can see what they
 * were handed, and so a file that LIES about its units can be spotted and overridden.
 * (floless.app/docs/superpowers/specs/2026-07-25-reference-objects-units-evidence.md)
 *
 * null means the file did not say — an honest "unknown", never a guess. `id` is what
 * `unitToMM` resolves for the approximate preflight extent; see `probeModel`.
 */
function lengthUnit(api, modelID) {
  const ids = api.GetLineIDsWithType(modelID, WebIFC.IFCUNITASSIGNMENT);
  for (let i = 0; i < ids.size(); i++) {
    const asg = api.GetLine(modelID, ids.get(i));
    for (const ref of asg.Units || []) {
      let u;
      try { u = api.GetLine(modelID, ref.value); } catch { continue; }
      if (!u || strOf(u.UnitType) !== 'LENGTHUNIT') continue;
      const name = strOf(u.Name);
      if (!name) continue;
      const prefix = strOf(u.Prefix);
      return { declared: prefix ? `${prefix}.${name}` : name, id: ref.value };
    }
  }
  return { declared: null, id: null };
}

// SI prefix -> multiplier, for turning a declared IfcSIUnit into millimetres.
const SI_PREFIX = {
  EXA: 1e18, PETA: 1e15, TERA: 1e12, GIGA: 1e9, MEGA: 1e6, KILO: 1e3, HECTO: 1e2, DECA: 10,
  DECI: 1e-1, CENTI: 1e-2, MILLI: 1e-3, MICRO: 1e-6, NANO: 1e-9, PICO: 1e-12, FEMTO: 1e-15, ATTO: 1e-18,
};

/**
 * How many millimetres one file-unit is, for the APPROXIMATE preflight extent only (see probeModel).
 *
 * Returns `null` when the unit cannot be resolved — deliberately, and the caller must handle it.
 * An earlier cut used a five-entry metric lookup that silently fell back to a factor of 1. That is
 * exactly the failure this whole feature was built to avoid: an imperial file (IfcConversionBasedUnit,
 * e.g. inches) would be scaled by 1 and reported as a bbox "in millimetres" that was wrong by 25.4x,
 * with nothing to indicate it. A guessed factor is worse than an honest unknown.
 *
 * Handles both shapes IFC allows for a length unit:
 *  - IfcSIUnit — Name METRE plus an optional Prefix.
 *  - IfcConversionBasedUnit — a named unit (inch, foot) whose ConversionFactor is an IfcMeasureWithUnit
 *    giving its size in terms of another unit, which is itself resolved recursively.
 */
export function unitToMMForTest(api, modelID, unitId) { return unitToMM(api, modelID, unitId); }

function unitToMM(api, modelID, unitId, depth = 0) {
  if (depth > 4) return null; // a malformed file could otherwise cycle through ConversionFactor
  let u;
  try { u = api.GetLine(modelID, unitId); } catch { return null; }
  if (!u) return null;
  const name = strOf(u.Name);
  if (name === 'METRE') {
    const prefix = strOf(u.Prefix);
    const mult = prefix ? SI_PREFIX[prefix] : 1;
    return mult == null ? null : 1000 * mult; // metres -> mm
  }
  const cf = u.ConversionFactor && u.ConversionFactor.value;
  if (cf != null) {
    let m;
    try { m = api.GetLine(modelID, cf); } catch { return null; }
    const value = strOf(m && m.ValueComponent);
    const compId = m && m.UnitComponent && m.UnitComponent.value;
    if (typeof value === 'number' && compId != null) {
      const base = unitToMM(api, modelID, compId, depth + 1);
      return base == null ? null : value * base;
    }
  }
  return null;
}

/**
 * The physical elements placed in the spatial structure, via IfcRelContainedInSpatialStructure.
 *
 * Deliberately NOT a hardcoded list of IFC element types: that list differs across IFC2X3 / IFC4 /
 * IFC4X3 and would silently undercount whichever schema it was not written against. Containment is
 * schema-stable and is also the right question — "what did the author place in this building?".
 */
function placedElements(api, modelID) {
  const ids = new Set();
  const rels = api.GetLineIDsWithType(modelID, WebIFC.IFCRELCONTAINEDINSPATIALSTRUCTURE);
  for (let i = 0; i < rels.size(); i++) {
    const rel = api.GetLine(modelID, rels.get(i));
    for (const e of rel.RelatedElements || []) if (e && e.value != null) ids.add(e.value);
  }
  return ids;
}

/**
 * `probe` — answer the cheap questions WITHOUT tessellating: what schema, what units, how many
 * elements, and roughly where and how big it sits.
 *
 * This exists so a consumer can decide whether to load a file at all before paying to tessellate it.
 * Tessellating a 300 MB model in order to discover it is too big is the exact freeze the size cap is
 * meant to prevent.
 *
 * `bbox` is APPROXIMATE and in millimetres: it comes from the file's own IfcCartesianPoints scaled by
 * the declared unit, so it includes local profile coordinates and ignores placement nesting. It is not
 * the authoritative extent — that comes from real geometry in `read-model`.
 *
 * THE BOX IS USUALLY PINNED TO THE WORLD ORIGIN, so its midpoint is not the model's position. Every
 * placement origin is a point — the representation context, the site, the building, each product — and
 * they sit at (0,0,0), so on a file authored wholly in the positive octant `min` is [0,0,0] exactly and
 * the midpoint lands about half way from the origin to the model. The DOMINANT term is therefore
 * `distance-from-origin / 2`, which is a property of the file rather than of this function — but not
 * the whole error: `baseplate-origin.ifc` is authored AT the origin, where that term predicts ~0, and
 * still measures 0.44, because the box cannot see the swept column. That residual is THAT FIXTURE'S,
 * not a floor under every file — a model given as explicit 3D points (BRep, tessellated items) hands
 * this loop the real vertices and can measure ~0, and an unseen sweep symmetric about the point-box
 * midpoint moves the centre not at all. probe.test.mjs pins the 0.44 for that one fixture. Measured:
 * 0.01 model longest-edges for `example-steel-framing.ifc` (authored from the origin), 0.44 for
 * `baseplate-origin.ifc`, 9.6-12.6 for this repo's offset connection fixtures (~23 m out, ~1 m
 * connection), 12.9 for Motebello. (The [0,0,0] min is not a law — `baseplate-origin.ifc` straddles the
 * origin and reports a negative min.)
 *
 * `bbox` ALONE CANNOT TELL YOU WHICH CASE YOU ARE IN. This comment used to claim it could, via the rule
 * "|max| much larger than the box's own span means origin-pinned". Whenever a box CONTAINS the origin,
 * |max| is at most the span on each axis, so the ratio is <= 1 — exactly 1 when min is [0,0,0] AND the
 * box has some span; a file whose usable 3D points all sit at the origin yields min == max == [0,0,0],
 * where the ratio is 0/0 rather than 1 (a legal output — a point, not the null reserved for NO usable
 * points) — and
 * "much larger" cannot happen. It therefore only ever reports its other arm, "the midpoint is fine",
 * about files whose midpoint is 9.6-12.6 longest-edges out.
 *
 * That is a fact about the FILES, not about this function, and the distinction matters: the loop below
 * bounds the file's own 3D points and never inserts the origin, so a file CAN produce a box that
 * excludes the origin, and such a box CAN score above 1. Neither is automatic — points at (1,1,1) and
 * (-1,-1,-1) are both nonzero and still bracket the origin, and an origin-excluding box scores at most
 * 1 if some other axis is wide enough. Every file measured here anchors those points at (0,0,0), which
 * is ordinary but not guaranteed.
 *
 * Nor is a high ratio a licence to trust the midpoint: a point box CAN exclude the origin, score ~84,
 * and still sit 0.44 longest-edges off, because it cannot see the swept column. There is no cheap
 * substitute — for position, as for size, the answer is `read-model`. Pinned by `probe.test.mjs`
 * against the five in-repo fixtures.
 *
 * The far corner tracks the model within one longest-edge on every file measured EXCEPT under a
 * rotated frame — `baseplate-rot.ifc` puts it 10.3 out, on the wrong side, and nothing in the response
 * marks that case, so it is an observation and not a method. The box's SPAN is not a position signal
 * either, and it is not even monotonic in the distance you would be reading off it: measured against
 * each model's longest edge it runs 0.2x on `baseplate-origin.ifc`, ~1x on `example-steel-framing.ifc`
 * (both authored at the origin) and 17-18x on the four offset fixtures.
 *
 * Why it is not simply fixed: a point-based extent cannot bound a swept solid (the size lives in
 * `IfcRectangleProfileDef`/`IfcExtrudedAreaSolid` NUMBERS, not in any point); composing placements is
 * a no-op on files that place at the origin; and excluding transform points empties the box, because
 * on such a file every point IS a placement origin. All three measured in aware-aeco/aware#348.
 */
export function probeModel(api, modelID) {
  const { declared, id: unitId } = lengthUnit(api, modelID);
  const toMM = unitId == null ? null : unitToMM(api, modelID, unitId);
  const breakdown = modelBreakdown(api, modelID);
  // An unresolvable unit means we cannot state the extent in millimetres. Say so with `bbox: null`
  // rather than emitting a plausible-looking box scaled by a guessed 1 — a wrong extent is what
  // drives a consumer's "this looks 1000x off" logic, so a guess here causes the exact misjudgement
  // the size check exists to make correctly.
  if (toMM == null) {
    return {
      schema: api.GetModelSchema ? api.GetModelSchema(modelID) : null,
      units: { declared },
      elements: placedElements(api, modelID).size,
      frame: FILE_Z_UP,
      bbox: null,
      ...breakdown,
    };
  }
  const min = [Infinity, Infinity, Infinity];
  const max = [-Infinity, -Infinity, -Infinity];
  const pts = api.GetLineIDsWithType(modelID, WebIFC.IFCCARTESIANPOINT);
  for (let i = 0; i < pts.size(); i++) {
    let c;
    try { c = api.GetLine(modelID, pts.get(i)); } catch { continue; }
    const co = c && c.Coordinates;
    if (!co || co.length < 3) continue; // 2D points (profile geometry) tell us nothing about extent
    for (let k = 0; k < 3; k++) {
      const v = (strOf(co[k]) ?? 0) * toMM;
      if (!Number.isFinite(v)) continue;
      if (v < min[k]) min[k] = v;
      if (v > max[k]) max[k] = v;
    }
  }
  // No usable 3D points → null, NOT a zero box. A zero box is a claim ("this model is a point at the
  // origin"); null is the truth ("we could not tell").
  const empty = !Number.isFinite(min[0]);
  return {
    schema: api.GetModelSchema ? api.GetModelSchema(modelID) : null,
    units: { declared },
    elements: placedElements(api, modelID).size,
    frame: FILE_Z_UP,
    bbox: empty ? null : { min, max },
    ...breakdown,
  };
}

/**
 * What is IN the file, by storey and by IFC type — so a consumer can offer "which part do you want?"
 * before committing to a read it may not be able to afford.
 *
 * THESE COUNT THE POPULATION A `read-model` FILTER SELECTS, not the one `elements` counts, and the two
 * genuinely differ: `elements` is what the author placed directly in the spatial structure (a real
 * 12-storey model: 5,878), while a read returns those plus everything transitively aggregated beneath
 * them (17,460). A breakdown that predicted the smaller number would mislead every consumer that used
 * it to size a read — which is the only reason to ask.
 *
 * They are an UPPER BOUND on what a read returns, and cannot be anything else: whether an element
 * carries a drawable triangle is only knowable by tessellating it, which is precisely what `probe`
 * exists not to do. A read reports the difference as `skipped`.
 *
 * SCOPE, precisely: elements IN the spatial structure. An element outside it is not listed — measured
 * on the same 12-storey model, that is exactly one object, the IfcSite's own surface, which is the top
 * of the structure rather than something contained by it. Such an element cannot be reached by
 * `storeys` (it has none) but CAN be reached by `ifc-types`, so the type rows are a lower bound for
 * that one filter. Stating the population beats inventing a count for geometry probe cannot see.
 *
 * No tessellation happens here — this walks the same relationship tables `read-model` already walks
 * before it streams, so `probe` stays the cheap call it is documented to be.
 */
function modelBreakdown(api, modelID) {
  const storeys = storeyByElement(api, modelID);
  const byStorey = new Map();
  const byType = new Map();
  for (const [id, label] of storeys) {
    const key = label ?? null;
    byStorey.set(key, (byStorey.get(key) ?? 0) + 1);
    let entity = null;
    try { entity = typeName(api.GetLineType(modelID, id)); } catch { entity = null; }
    byType.set(entity, (byType.get(entity) ?? 0) + 1);
  }
  // Descending by count: the first rows are the ones worth offering, and a 16-type model should not
  // make a consumer sort before it can show anything useful.
  const rows = (m, key) => [...m.entries()]
    .sort((a, b) => b[1] - a[1] || String(a[0]).localeCompare(String(b[0])))
    .map(([name, elements]) => ({ [key]: name, elements }));
  return { storeys: rows(byStorey, 'name'), types: rows(byType, 'name') };
}

// Numeric IFC type code -> its entity name ("IFCBEAM"). Built by reversing web-ifc's own exported
// constants rather than hardcoding a table, so it stays correct across schema versions and web-ifc
// upgrades. Lazily built: it is a few hundred entries and most runs never need it.
let TYPE_NAMES = null;
function typeName(code) {
  if (!TYPE_NAMES) {
    TYPE_NAMES = new Map();
    for (const [k, v] of Object.entries(WebIFC)) {
      if (typeof v === 'number' && k.startsWith('IFC')) TYPE_NAMES.set(v, k);
    }
  }
  return TYPE_NAMES.get(code) ?? null;
}

/**
 * Entity name -> its numeric IFC type code — `typeName` read the other way, for the `ifc-types` filter.
 *
 * Case-insensitive, because a filter value is typed by a human or copied out of a `probe` breakdown,
 * and "IfcBeam" naming the same entity as "IFCBEAM" is not a distinction worth enforcing. Returns null
 * for a name web-ifc does not know, which the caller reports rather than silently matching nothing —
 * a typo that quietly selects zero objects reads exactly like a model that has none.
 */
function typeCode(name) {
  typeName(0); // force TYPE_NAMES to build
  const want = String(name ?? '').trim().toUpperCase();
  if (!want) return null;
  for (const [code, entity] of TYPE_NAMES) if (entity === want) return code;
  return null;
}

// element expressID -> the name of the spatial structure (storey) containing it.
//
// Spatial containment is only ever declared on the OUTERMOST element: IFC forbids an assembly's parts
// from also being contained, so a bolt inside an IfcElementAssembly has no containment of its own.
// Reading direct containment alone therefore reports `storey: null` for every part of every assembly —
// so aggregated children inherit their parent's storey, transitively.
function storeyByElement(api, modelID) {
  const out = new Map();
  const rels = api.GetLineIDsWithType(modelID, WebIFC.IFCRELCONTAINEDINSPATIALSTRUCTURE);
  for (let i = 0; i < rels.size(); i++) {
    const rel = api.GetLine(modelID, rels.get(i));
    const structId = rel.RelatingStructure && rel.RelatingStructure.value;
    if (structId == null) continue;
    let label = null;
    try { label = strOf(api.GetLine(modelID, structId).Name) ?? null; } catch { /* unnamed storey */ }
    for (const e of rel.RelatedElements || []) if (e && e.value != null) out.set(e.value, label);
  }
  // Push each containment down through IfcRelAggregates, repeating until a pass learns nothing new,
  // so a nested assembly resolves at ANY depth.
  //
  // There is deliberately no pass cap. Termination comes from the `!out.has(kid)` guard: every
  // iteration that continues has strictly grown `out`, which is bounded by the number of elements in
  // the file, so a cyclic aggregation cannot spin. An earlier cut also capped the passes at 8 "for
  // safety" — but relationship iteration order is arbitrary, so a reverse-ordered chain advances only
  // one level per pass, and that cap silently returned `storey: null` for anything nested deeper.
  // A bound that can truncate a legitimate answer is not a safety measure.
  const children = assemblyChildren(api, modelID);
  for (;;) {
    let learned = 0;
    for (const [parent, kids] of children) {
      if (!out.has(parent)) continue;
      const label = out.get(parent);
      for (const kid of kids) {
        if (kid != null && !out.has(kid)) { out.set(kid, label); learned++; }
      }
    }
    if (!learned) break;
  }
  return out;
}

// element expressID -> a material name, following IfcRelAssociatesMaterial through the three shapes a
// material association can take (a bare material, a layer set, a profile set).
//
// The material string is load-bearing, not decoration: it is the signal that says DO NOT CONVERT THIS.
// A file can name a member "girder" and type it IfcBeam while its material is wood_spruce_beam — type
// alone would happily turn timber into steel.
function materialByElement(api, modelID) {
  const out = new Map();
  const nameOf = (id) => {
    let line;
    try { line = api.GetLine(modelID, id); } catch { return null; }
    if (!line) return null;
    const direct = strOf(line.Name);
    if (direct) return direct;
    // A layer/profile/constituent set — take the first member's material name.
    for (const key of ['MaterialLayers', 'MaterialProfiles', 'MaterialConstituents', 'Materials', 'ForLayerSet']) {
      const v = line[key];
      if (!v) continue;
      const items = Array.isArray(v) ? v : [v];
      for (const it of items) {
        if (it == null) continue;
        if (it.value != null) { const n = nameOf(it.value); if (n) return n; }
      }
    }
    if (line.Material && line.Material.value != null) return nameOf(line.Material.value);
    return null;
  };
  const rels = api.GetLineIDsWithType(modelID, WebIFC.IFCRELASSOCIATESMATERIAL);
  for (let i = 0; i < rels.size(); i++) {
    let rel;
    try { rel = api.GetLine(modelID, rels.get(i)); } catch { continue; }
    const matId = rel.RelatingMaterial && rel.RelatingMaterial.value;
    if (matId == null) continue;
    const name = nameOf(matId);
    if (!name) continue;
    // Note this keys on whatever the relationship points at — which may be an element OCCURRENCE or
    // an element TYPE. The type case is resolved below.
    for (const e of rel.RelatedObjects || []) if (e && e.value != null) out.set(e.value, name);
  }

  // IFC lets a material be attached to the element TYPE rather than each occurrence, with the
  // occurrence free to override it. Following only direct associations therefore returns null for
  // perfectly ordinary files — and since `material` is the signal that says DO NOT CONVERT THIS, a
  // null there silently re-arms the exact mistake it exists to prevent (converting timber to steel).
  // So walk IfcRelDefinesByType and let each occurrence inherit its type's material, without ever
  // overwriting a material the occurrence declared for itself.
  const byType = api.GetLineIDsWithType(modelID, WebIFC.IFCRELDEFINESBYTYPE);
  for (let i = 0; i < byType.size(); i++) {
    let rel;
    try { rel = api.GetLine(modelID, byType.get(i)); } catch { continue; }
    const typeId = rel.RelatingType && rel.RelatingType.value;
    if (typeId == null) continue;
    const inherited = out.get(typeId);
    if (!inherited) continue;
    for (const e of rel.RelatedObjects || []) {
      if (e && e.value != null && !out.has(e.value)) out.set(e.value, inherited);
    }
  }
  return out;
}

/**
 * One property's value, rendered as the file wrote it.
 *
 * IFC wraps every value in a typed carrier (`IfcLabel`, `IfcReal`, `IfcBoolean`…), and the several
 * property kinds carry it under different keys — a single value, an enumeration, a list, a bounded
 * range. They are flattened to text here because the consumer's job is to SHOW what the file says:
 * no unit conversion, no localisation, no rounding. A value we cannot read becomes null rather than
 * the string "undefined", so "the file did not say" stays distinguishable from "the file said this".
 */
function propertyValue(line) {
  const one = (v) => {
    const raw = strOf(v);
    if (raw == null) return null;
    return typeof raw === 'string' ? raw : String(raw);
  };
  const many = (vs) => {
    const parts = (vs || []).map(one).filter((v) => v != null);
    return parts.length ? parts.join(', ') : null;
  };
  if (line.NominalValue != null) return one(line.NominalValue);          // IfcPropertySingleValue
  if (line.EnumerationValues != null) return many(line.EnumerationValues); // IfcPropertyEnumeratedValue
  if (line.ListValues != null) return many(line.ListValues);              // IfcPropertyListValue
  if (line.UpperBoundValue != null || line.LowerBoundValue != null) {     // IfcPropertyBoundedValue
    const lo = one(line.LowerBoundValue), hi = one(line.UpperBoundValue);
    return lo != null || hi != null ? `${lo ?? ''}..${hi ?? ''}` : null;
  }
  return null;
}

/**
 * The express ids a property-definition select points at — one, or SINCE IFC4 a whole list.
 *
 * `IfcRelDefinesByProperties.RelatingPropertyDefinition` is an `IfcPropertySetDefinitionSelect`, and
 * IFC4 widened it to admit an `IfcPropertySetDefinitionSet`: several property sets attached by ONE
 * relationship. web-ifc hands that back as `{ value: [2, 4] }`.
 *
 * Reading `.value` as a single id therefore passed an ARRAY to GetLine, which matches no line — so
 * every set in such a relationship vanished. Not a partial read: a silent, total loss, on a valid and
 * ordinary IFC4 encoding. Elements may carry handles or bare numbers, so both are flattened.
 */
function definitionIds(select) {
  if (!select) return [];
  const v = select.value;
  if (v == null) return [];
  const list = Array.isArray(v) ? v : [v];
  return list.map((e) => (e && typeof e === 'object' && 'value' in e ? e.value : e)).filter((id) => id != null);
}

/** One IfcPropertySet as `{ name, properties: [{ name, value }] }`, or null when it holds nothing. */
function readPropertySet(api, modelID, setId) {
  let set;
  try { set = api.GetLine(modelID, setId); } catch { return null; }
  if (!set || !Array.isArray(set.HasProperties)) return null;   // IfcElementQuantity lands here and is skipped
  const properties = [];
  for (const ref of set.HasProperties) {
    if (!ref || ref.value == null) continue;
    let prop;
    try { prop = api.GetLine(modelID, ref.value); } catch { continue; }
    if (!prop) continue;
    const name = strOf(prop.Name);
    if (name == null) continue;
    properties.push({ name: String(name), value: propertyValue(prop) });
  }
  if (!properties.length) return null;
  return { name: strOf(set.Name) == null ? null : String(strOf(set.Name)), properties };
}

/**
 * Every property set each element carries, grouped and named exactly as the file wrote them.
 *
 * THIS IS WHERE THE MEANING LIVES IN A REAL FILE, and the reference-objects design says so with
 * evidence: in `11134_V_Motebello_Heistopp_Rev.ifc` every object is literally named `-`, so the name,
 * type and storey say nothing at all — while 31 property sets carry 271 values under one vendor set,
 * in Norwegian. A reader that returns only the six well-known fields hands the consumer an object
 * tree it cannot identify anything in.
 *
 * Nothing is normalised, translated or renamed. Vendor sets are the norm rather than the exception,
 * and their value is in being shown as authored; a reader that tidied them would be discarding the
 * only thing that distinguishes one proxy from another.
 *
 * Occurrence and TYPE sets are both collected, mirroring `materialByElement` and for the same reason:
 * IFC lets a property sit on the element type with the occurrence inheriting it, so following only
 * IfcRelDefinesByProperties returns nothing for perfectly ordinary exports. A set the occurrence
 * declares wins over a type set of the same name — that is what an occurrence-level override means —
 * and the type's remaining sets are appended rather than dropped.
 */
export function propertySetsByElement(api, modelID) {
  const out = new Map();          // element expressID -> [{ name, properties }]
  const byTypeId = new Map();     // type expressID -> [{ name, properties }]
  const cache = new Map();        // set expressID -> parsed set (files reuse one set across elements)

  const parse = (setId) => {
    if (cache.has(setId)) return cache.get(setId);
    const parsed = readPropertySet(api, modelID, setId);
    cache.set(setId, parsed);
    return parsed;
  };
  const add = (map, key, set) => {
    if (!set) return;
    const list = map.get(key) || [];
    list.push(set);
    map.set(key, list);
  };

  const rels = api.GetLineIDsWithType(modelID, WebIFC.IFCRELDEFINESBYPROPERTIES);
  for (let i = 0; i < rels.size(); i++) {
    let rel;
    try { rel = api.GetLine(modelID, rels.get(i)); } catch { continue; }
    for (const setId of definitionIds(rel.RelatingPropertyDefinition)) {
      const parsed = parse(setId);
      if (!parsed) continue;
      for (const e of rel.RelatedObjects || []) if (e && e.value != null) add(out, e.value, parsed);
    }
  }

  // Type-level sets hang off the type object itself (`HasPropertySets`), not off a relationship, so
  // they are collected by walking the types an occurrence is bound to.
  const byType = api.GetLineIDsWithType(modelID, WebIFC.IFCRELDEFINESBYTYPE);
  for (let i = 0; i < byType.size(); i++) {
    let rel;
    try { rel = api.GetLine(modelID, byType.get(i)); } catch { continue; }
    const typeId = rel.RelatingType && rel.RelatingType.value;
    if (typeId == null) continue;
    if (!byTypeId.has(typeId)) {
      let typeLine;
      try { typeLine = api.GetLine(modelID, typeId); } catch { typeLine = null; }
      const sets = [];
      for (const ref of (typeLine && typeLine.HasPropertySets) || []) {
        if (!ref || ref.value == null) continue;
        const parsed = parse(ref.value);
        if (parsed) sets.push(parsed);
      }
      byTypeId.set(typeId, sets);
      // A type can ALSO be the RelatedObject of an IfcRelDefinesByProperties, which the first pass
      // filed under the type's own id. Those are the type's sets too.
      for (const s of out.get(typeId) || []) sets.push(s);
    }
    const inherited = byTypeId.get(typeId);
    if (!inherited.length) continue;
    for (const e of rel.RelatedObjects || []) {
      if (!e || e.value == null) continue;
      out.set(e.value, mergeInherited(out.get(e.value) || [], inherited));
    }
  }
  return out;
}

/**
 * The occurrence's sets merged with its type's, the occurrence winning PROPERTY BY PROPERTY.
 *
 * Set-level precedence would lose data the file plainly states: a type `Pset_WallCommon` carrying
 * {FireRating, LoadBearing} beside an occurrence `Pset_WallCommon` carrying only {FireRating} would
 * drop LoadBearing entirely, because the occurrence "won" the whole set. Property-level is also what
 * the sibling `ifc-inspector.entities.get-by-guid` does, and two agents reading the same file must not
 * disagree about what it says.
 *
 * Copies rather than mutating: parsed sets are cached and shared across every element that references
 * them, so appending to one in place would leak another element's type properties into it.
 */
export function mergeInherited(own, inherited) {
  const merged = own.map((s) => ({ name: s.name, properties: [...s.properties] }));
  for (const t of inherited) {
    const mine = merged.find((s) => s.name === t.name);
    if (!mine) { merged.push({ name: t.name, properties: [...t.properties] }); continue; }
    const taken = new Set(mine.properties.map((p) => p.name));
    for (const p of t.properties) if (!taken.has(p.name)) mine.properties.push(p);
  }
  return merged;
}

// The profile NAME an element's swept-solid representation was built from ("W10X33"), or null.
//
// The name is what makes a catalogue lookup possible at conversion time, and §4.4 of the design makes
// that mandatory: sections are looked up by name, NEVER measured off the mesh. This very file writes
// W10X33 as a plain 150x250 box while the real section is 247x202, so a converter that measured would
// come out ~25% narrow on the flange with nothing on screen looking wrong.
function profileOf(api, modelID, expressID) {
  let el;
  try { el = api.GetLine(modelID, expressID); } catch { return null; }
  const repId = el && el.Representation && el.Representation.value;
  if (repId == null) return null;
  let rep;
  try { rep = api.GetLine(modelID, repId); } catch { return null; }
  for (const r of rep.Representations || []) {
    if (r == null || r.value == null) continue;
    let shape;
    try { shape = api.GetLine(modelID, r.value); } catch { continue; }
    for (const item of shape.Items || []) {
      if (item == null || item.value == null) continue;
      let solid;
      try { solid = api.GetLine(modelID, item.value); } catch { continue; }
      const areaId = solid && solid.SweptArea && solid.SweptArea.value;
      if (areaId == null) continue;
      try {
        const name = strOf(api.GetLine(modelID, areaId).ProfileName);
        if (name) return name;
      } catch { /* not a named profile */ }
    }
  }
  return null;
}

/**
 * Does this file author ANY surface colour at all?
 *
 * THIS QUESTION HAS TO BE ASKED, because web-ifc's per-geometry colour cannot answer it. An
 * unstyled geometry does not come back colourless — it comes back **opaque white**, which is
 * indistinguishable from a wall the architect painted white. Measured 2026-08-03:
 * `example-steel-framing.ifc` carries zero style entities and every one of its 13 objects reports
 * `{1,1,1,1}`, while `Building-Architecture.ifc` reports the same white for 6 objects that are
 * genuinely styled that way. A consumer handed white for both would paint an entirely unstyled
 * model glaring white and call it "the file's real colours".
 *
 * So the file is asked once, and the answer gates the whole `colors` field: no surface style
 * anywhere means no object gets a colour, and a consumer renders its own default rather than a
 * white that was never authored.
 *
 * WHY THE FILE AND NOT THE ELEMENT. The precise question — "was THIS geometry styled?" — was tried
 * first and does not survive contact with a real model. Resolving `IfcStyledItem.Item` against
 * `geometryExpressID`, plus material-associated styles via `IfcMaterialDefinitionRepresentation`,
 * agreed with web-ifc on four small files and then missed **11,257** genuinely-coloured geometries
 * on `Steel IFC.ifc` and 370 on `Hospital Arch.ifc` — web-ifc reaches colour through routes that
 * reimplementation did not, and a resolver whose misses are invisible is worse than none. The
 * file-level question is exact for the case that actually bites (a file with no palette at all) and
 * is one index lookup rather than a second style engine.
 *
 * The cost of stopping here, stated plainly: an element that is unstyled *inside* a styled file
 * still reports white, because that is what web-ifc resolved and this reader will not guess past
 * it. On `Hospital Arch.ifc` that is 696 of 31,381 placed geometries — and a FILTERED read sharpens
 * it, because a filter that selects only unstyled elements still sees the file's palette and so
 * still reports their white as authored.
 *
 * WHY `IfcIndexedColourMap` IS DELIBERATELY NOT COUNTED. IFC4 lets a tessellated face set carry
 * per-face colour through `IfcIndexedColourMap` + `IfcColourRgbList` with no `IfcSurfaceStyle`
 * anywhere, so on paper this gate has a false negative there. Measured 2026-08-03 against web-ifc
 * 0.0.77 with a hand-built IFC4 file — one `IfcTriangulatedFaceSet`, four faces coloured red, green,
 * blue and yellow, zero surface styles — **web-ifc reports `{1,1,1,1}`**: it does not implement that
 * route. So counting the colour map here would not recover those colours, it would switch the gate on
 * and publish web-ifc's default white as though the file had authored it, for a file that is in fact
 * brightly coloured. Suppressing is the honest answer until the engine can answer.
 */
export function fileAuthorsColour(api, modelID) {
  // IfcSurfaceStyle is the root of every colour route web-ifc actually IMPLEMENTS (IfcStyledItem,
  // presentation style assignments, material definition representations all terminate in one). Its
  // mere presence is the signal; which elements it reaches is deliberately not asked (see above).
  return api.GetLineIDsWithType(modelID, WebIFC.IFCSURFACESTYLE).size() > 0;
}

/**
 * Round a colour channel to what an 8-bit authored value round-trips through, and no further.
 *
 * Colours are authored as k/255 overwhelmingly often, and 4 decimal places carry that back exactly
 * (0.4627 x 255 = 117.99 -> 118) while spending 6 characters instead of the 19 a raw float costs.
 * On a 206,621-geometry model that is the difference between a few hundred KB and a few MB of
 * response for information nobody can see.
 */
const chan = (v) => Math.round(Math.max(0, Math.min(1, Number(v) || 0)) * 1e4) / 1e4;

/**
 * Accumulate one placed geometry's colour as a RUN over the index buffer.
 *
 * COLOUR IS PER PLACED GEOMETRY, NOT PER OBJECT, and a real model says so: 6,358 of the 77,118
 * objects in `Steel IFC.ifc` carry more than one colour among their geometries. Collapsing to one
 * colour per object would be a visible lie on 8% of an ordinary steel export — a bolted assembly
 * rendered entirely in its bolt's colour.
 *
 * Runs are contiguous because the index buffer is built in geometry order, so each geometry owns
 * `[start, start+count)` and a consumer maps a run straight onto a draw group. Adjacent runs of the
 * SAME colour are merged rather than emitted separately: with 16 distinct colours across 206,621
 * geometries, unmerged runs would be mostly repetition.
 */
export function pushColourRun(runs, rgba, start, count) {
  if (count <= 0) return runs;
  const last = runs[runs.length - 1];
  if (last && last.start + last.count === start
    && last.rgba[0] === rgba[0] && last.rgba[1] === rgba[1]
    && last.rgba[2] === rgba[2] && last.rgba[3] === rgba[3]) {
    last.count += count;
    return runs;
  }
  runs.push({ rgba, start, count });
  return runs;
}

/**
 * web-ifc's Y-up reading of a point given in the file's own Z-up world frame: `(x, y, z) -> (x, z, -y)`.
 *
 * This is web-ifc's `NormalizeIFC` — the fixed rotation it bakes into every flat mesh transform, so
 * that IFC's Z-up world comes out in the Y-up frame a renderer wants. `read-model` undoes it (below),
 * which is what makes its meshes comparable with `probe`'s bbox.
 *
 * Exported so the #343 regression test can reconstruct the pre-fix, Y-up reading of the SAME meshes
 * and assert that it fails the frame-agreement check the fixed output passes. Without that arm the
 * check could be trivially satisfiable and still green.
 */
export const toWebIfcYUp = ([x, y, z]) => [x, z, -y];

/**
 * The frame each command's coordinates are in, stated IN THE OUTPUT so a consumer can check it at
 * runtime instead of inferring it from a version number.
 *
 * Version numbers cannot answer this question, which is why the field exists. Measured 2026-08-01
 * against a real `aware app run`: (a) the bridge binary is installed separately from the agent
 * (`aware sidecar install connection-reader`) and a stale one only prints a warning and runs anyway
 * (cli/src/runtime/invoker.rs), so an agent manifest saying 1.0.0 can still be served by a bridge
 * that returns the old frame; and (b) even now that an app's `requires:` pin IS enforced at compile
 * and run (aware-aeco/aware#349 — it was enforced nowhere when this was written), the pin constrains
 * the AGENT version, and the bridge binary is installed separately from the agent. So the only
 * trustworthy answer is still the one the producing binary puts in its own payload.
 */
export const FILE_Z_UP = 'z-up'; // IFC's own world frame: X/Y in plan, Z up
// Retained though no command emits it any more: it is the documented value of the `frame` field a
// PRE-#347 bridge still sends, and a consumer comparing against it needs the spelling to be stated
// somewhere authoritative rather than guessed. Deleting it would not remove the value from the wire.
export const WEB_IFC_Y_UP = 'y-up'; // web-ifc's renderer frame: X/Z in plan, Y up

/**
 * `read-model` — the whole file as reference geometry.
 *
 * Unlike `extract`, this tessellates EVERY element rather than only connection hardware, and returns
 * what the file says about each one (name, IFC type, storey, profile, material) alongside its mesh.
 * Positions are in the file's own world frame — IFC's Z-up, the SAME frame `probe`'s bbox reports —
 * in millimetres. A consumer re-anchors and places.
 *
 * That frame costs one rotation, because web-ifc does not emit it. web-ifc bakes a fixed
 * `NormalizeIFC` into every flat mesh transform, mapping the file's Z-up world to its own Y-up
 * renderer frame: `(x, y, z) -> (x, z, -y)`. `probe` never sees it (it reads IfcCartesianPoints
 * straight off the file), so leaving it in made the two commands answer in different frames — a
 * reference model that renders on its side and a bbox that cannot be compared against the mesh
 * (aware-aeco/aware#343). We undo it here rather than in each consumer: the manifest promises the
 * file's own world frame, and one rotation in the reader beats the same rotation re-derived, or
 * forgotten, by every consumer.
 *
 * `maxVertices` is an in-process CIRCUIT BREAKER, not a preflight gate. Be honest about which it is:
 * an exact vertex count cannot be known before tessellating, so a cap applied by the caller after this
 * returns would fire only once the oversized payload had already been built and serialised — reporting
 * the freeze it was meant to prevent. Decrementing as meshes stream is what actually stops it. The
 * byte-size check a caller does before invoking us is the only true preflight protection.
 */
export function readModel(api, modelID, maxVertices = Infinity, opts = {}) {
  const storeys = storeyByElement(api, modelID);
  const objects = [];
  const sink = typeof opts.onObject === 'function' ? opts.onObject : null;
  let count = 0;
  let budget = maxVertices;
  let byteBudget = Number.isFinite(opts.maxBytes) ? opts.maxBytes : Infinity;
  let bytes = 0;
  let skipped = 0; // products web-ifc streamed but that carry no drawable triangle

  // RESOLVED BEFORE THE REST OF THE PREP, not after, and the ordering is the point rather than a
  // tidy-up. `onSelected` is how a progressive consumer learns whether its filter was honoured, and
  // it may only act on that BEFORE the first segment — so every millisecond between here and the
  // walk is dead air it must wait through. The maps below cost real time on the files this command
  // exists for (property sets are collected for every element in the file), and the selection needs
  // none of them: `selectExpressIds` reads `storeys` and nothing else. Behaviour is unchanged —
  // both maps are consulted only from `onMesh`, which cannot run until `StreamMeshes` is called.
  const selection = selectExpressIds(api, modelID, opts, storeys);
  // ABSENT AND EMPTY ARE DIFFERENT ANSWERS, the same distinction the `selected` output field draws:
  // `null` says no filter was asked for, so there is nothing to confirm. A receipt is passed only
  // when one was applied, which is what keeps "unfiltered" apart from "a filter that matched
  // everything" on the channel exactly as it is kept apart in the response.
  if (typeof opts.onSelected === 'function') opts.onSelected(selection.applied ? selection.report : null);

  const materials = materialByElement(api, modelID);
  const propertySets = propertySetsByElement(api, modelID);
  // Asked ONCE per file, not per element: web-ifc reports opaque white for unstyled geometry, so
  // without this a file that authors no colour at all would be served back as a white building.
  const authorsColour = fileAuthorsColour(api, modelID);

  const onMesh = (flatMesh) => {
    const positions = [];
    const indices = [];
    const colors = [];
    const geoms = flatMesh.geometries;
    // NOTE: `geometries` carries one entry PER INSTANCE, each with its own flatTransformation — which
    // is exactly how mapped/instanced items work. Do not de-duplicate by geometryExpressID: the
    // Motebello file serves 19 objects from 14 shapes, and de-duplicating silently loses five walls.
    for (let i = 0; i < geoms.size(); i++) {
      const pg = geoms.get(i);
      const geom = api.GetGeometry(modelID, pg.geometryExpressID);
      const verts = api.GetVertexArray(geom.GetVertexData(), geom.GetVertexDataSize()); // [x,y,z,nx,ny,nz]*
      const idx = api.GetIndexArray(geom.GetIndexData(), geom.GetIndexDataSize());
      const m = pg.flatTransformation; // 4x4 column-major, metres, in web-ifc's Y-up frame
      const base = positions.length / 3;
      for (let v = 0; v < verts.length; v += 6) {
        const x = verts[v], y = verts[v + 1], z = verts[v + 2];
        // Placement transform and the Y-up -> Z-up rotation in one step: the file frame's X is
        // web-ifc's X, its Y is -web-ifc-Z, and its Z (up) is web-ifc's Y — the inverse of
        // `toWebIfcYUp`, folded into the matrix multiply so the hot loop allocates nothing.
        positions.push(
          (m[0] * x + m[4] * y + m[8] * z + m[12]) * M_TO_MM,
          -(m[2] * x + m[6] * y + m[10] * z + m[14]) * M_TO_MM,
          (m[1] * x + m[5] * y + m[9] * z + m[13]) * M_TO_MM,
        );
      }
      // Winding is preserved without touching the indices: the rotation above has determinant +1, so
      // it cannot turn a front face into a back face. (A mirror would, and would need a flip here.)
      const runStart = indices.length;
      for (let k = 0; k < idx.length; k++) indices.push(base + idx[k]);
      // The run is recorded AFTER the indices it describes, against the buffer as it now stands, so
      // `start` can never drift from what it points at.
      if (authorsColour) {
        const c = pg.color;
        pushColourRun(colors, [chan(c.x), chan(c.y), chan(c.z), chan(c.w)], runStart, idx.length);
      }
      geom.delete();
    }
    // A part is renderable only with >=3 vertices and >=1 triangle. Drop the degenerate ones: an object
    // that loads but cannot be drawn is worse than an absent one, because it looks like success.
    // Dropping them SILENTLY would be the same mistake one level up, so they are counted and reported.
    if (positions.length < 9 || indices.length < 3) { skipped++; return; }

    budget -= positions.length / 3;
    if (budget < 0) {
      throw new Error(`that file is too complex to load as a reference model (over ${Math.round(maxVertices / 1000)}k vertices)`);
    }

    const id = flatMesh.expressID;
    let line;
    try { line = api.GetLine(modelID, id); } catch { line = null; }
    const object = {
      // THE FILE'S OWN GlobalId, or null — SEPARATE from `id`, which falls back to the expressID below.
      // That fallback is right for `id` (a consumer needs *something* to address an object by) and
      // catastrophic for anything comparing two files: an expressID is a file-local sequence number, so
      // two exports of one model share many, and a comparison keyed on `id` would pair unrelated objects
      // and call it a match. `compare` keys on THIS field and treats null as uncomparable.
      globalId: (line && strOf(line.GlobalId)) || null,
      id: (line && strOf(line.GlobalId)) || String(id),
      name: (line && strOf(line.Name)) || null,
      ifcType: typeName(api.GetLineType(modelID, id)),
      storey: storeys.get(id) ?? null,
      profile: profileOf(api, modelID, id),
      material: materials.get(id) ?? null,
      // ALWAYS AN ARRAY, empty when the file carries none — so a consumer renders "no properties"
      // rather than having to tell absent from empty, and never has to null-check before iterating.
      propertySets: propertySets.get(id) ?? [],
      positions,
      indices,
      // OMITTED ENTIRELY, not sent empty, when the file authors no surface colour. `[]` would have to
      // mean "this object has no colour" while the honest statement is "this file has none to give",
      // and a consumer told the first would have no way back to its own default. Same discipline as
      // `selected`: a field that is absent is a different answer from a field that is empty.
      ...(authorsColour ? { colors } : {}),
    };
    count++;

    // SERIALIZE ONCE. The sink writes JSON and the budget counts JSON, so producing the text here and
    // handing it on means one `JSON.stringify` per object rather than two — on a 289 MB response the
    // duplicate was a second 289 MB of serialisation for callers who set a budget.
    const text = (sink || byteBudget !== Infinity) ? JSON.stringify(object) : null;

    // The byte budget is OPT-IN (absent => Infinity) and it is deliberately not a default. Streaming
    // the response removed the ceiling that used to make a size limit compulsory, so imposing one here
    // would invent a refusal where none is needed. It exists because a CONSUMER may still have a
    // ceiling of its own, and would rather be told the size than discover it as an out-of-memory.
    if (byteBudget !== Infinity) {
      // BYTES, not `.length`. A JS string's length is UTF-16 code units, and the pipe carries UTF-8 —
      // so on the property values that motivated this reader (Norwegian, in one real file) a budget
      // measured by length silently lets the response run over the size it promised. Review measured
      // 69,363 units against 69,440 bytes on one small fixture; the gap grows with the file.
      bytes += Buffer.byteLength(text, 'utf8');
      if (bytes > byteBudget) {
        throw new Error(
          `that file returns more reference geometry than the ${mib(byteBudget)} budget allows `
          + `(over ${mib(bytes)} so far). Read one storey or a few IFC types at a time — `
          + 'see `storeys` / `ifc-types`.',
        );
      }
    }

    if (sink) sink(object, text); else objects.push(object);
  };

  // A FILTER IS RESOLVED TO EXPRESS IDS AND HANDED TO `StreamMeshes`, so the elements nobody asked for
  // are never tessellated at all. Filtering inside the callback would be far simpler and nearly
  // worthless: web-ifc has already built the mesh by the time the callback fires, so it would save the
  // bytes while paying the whole cost — which on the files that motivated this is the part that hurts.
  if (selection.ids) api.StreamMeshes(modelID, [...selection.ids], onMesh);
  else api.StreamAllMeshes(modelID, onMesh);

  return {
    frame: FILE_Z_UP,
    objects,
    skipped,
    count,
    // THE RECEIPT THAT SEPARATES TWO SILENCES. Without it, "this file authors no colour" and "the
    // bridge that read it predates colours" are both an absent `colors` on every object, and a
    // consumer cannot tell the permanent condition from the one an install would fix. Present from
    // 1.3.0 always — `false` is a real answer about the file, absence is a statement about the
    // bridge. Same discipline as `selected` and `budget`, which exist for exactly this reason.
    colorsAvailable: authorsColour,
    // Present ONLY when a filter was asked for, so "no filter" and "a filter that matched everything"
    // stay distinguishable, and so an older consumer sees no new field at all.
    ...(selection.applied ? { selected: selection.report } : {}),
    // The RECEIPT for `max-bytes`, and it exists for the same reason `selected` does. Review pointed
    // out that a caller passing only a budget got no acknowledgement from either bridge: a new one
    // emitted no `selected` (no subset filter was applied) and an old one silently ignores the input,
    // so a successful response could not be told from an unenforced budget. Echoing what was honoured
    // — and what it cost — makes "this bridge understood me" checkable on the one call that matters.
    ...(byteBudget !== Infinity ? { budget: { maxBytes: byteBudget, bytes } } : {}),
  };
}

/**
 * A byte count a human can read, without rounding a real limit away to "0 MB".
 *
 * `Math.round(n / 1048576)` reported a 1 KB budget as `0 MB budget allows (over 0 MB so far)`, which
 * names neither the limit nor the size — the two things the message exists to give.
 */
function mib(bytes) {
  if (bytes < 1024) return `${bytes} bytes`;
  if (bytes < 1048576) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / 1048576).toFixed(bytes < 10 * 1048576 ? 1 : 0)} MB`;
}

/**
 * The three classes web-ifc's `StreamAllMeshes` skips, which a filter must skip too.
 *
 * THIS LIST IS NOT OURS — it mirrors web-ifc 0.0.77, whose mesh walk explicitly excludes
 * `IFCOPENINGELEMENT`, `IFCOPENINGSTANDARDCASE` and `IFCSPACE` (`src/cpp/wasm/web-ifc-wasm.cpp`).
 * `StreamMeshes` honours whatever ids it is handed and applies no such rule, so anything on this list
 * that a filter selects would come back as an object an unfiltered read has never returned.
 *
 * Both kinds matter and they are different kinds. The openings are VOIDS — the prisms cut out of walls
 * for doors, 740 of them in one real model, which would render as solid boxes filling every doorway.
 * `IfcSpace` is a room VOLUME, so a `storeys:` or a broad `ifc-types:` filter would hand back the air
 * in every room as geometry. Review caught the second: excluding `IfcFeatureElementSubtraction` alone
 * covers both opening types (the standard case is a subtype) and misses spaces entirely.
 *
 * The subtraction SUPERTYPE is queried with `includeInherited` rather than the two opening types by
 * name, so a schema that adds another subtraction kind is covered without revisiting this — the same
 * reason `placedElements` refuses to hardcode an element-type list.
 */
function notInAWholeRead(api, modelID) {
  const out = new Set();
  for (const type of [WebIFC.IFCFEATUREELEMENTSUBTRACTION, WebIFC.IFCSPACE]) {
    const found = api.GetLineIDsWithType(modelID, type, true);
    for (let i = 0; i < found.size(); i++) out.add(found.get(i));
  }
  return out;
}

/**
 * Turn the `storeys` / `ifc-types` / `ids` filters into the set of expressIDs to tessellate.
 *
 * Returns `{ ids: null }` when nothing was asked for — the whole-file read, unchanged, so the cost of
 * this feature to an existing caller is one Map lookup that never happens.
 *
 * SEVERAL FILTERS INTERSECT. `storeys: [L2], ifc-types: [IFCBEAM]` means the beams on L2, not their
 * union — "narrow it down" is what every one of these is for, so widening on a second term would be
 * the surprising reading.
 *
 * A VALUE THAT MATCHES NOTHING IS REPORTED, NOT SWALLOWED. A misspelled storey or entity name selects
 * zero objects, which is indistinguishable from a model that genuinely has none — so the unmatched
 * values come back in `selected.unmatched` and the caller can say "there is no storey called that"
 * instead of "this model is empty".
 *
 * The candidate population is `storeyByElement`'s keys: everything placed in the spatial structure
 * plus everything transitively aggregated beneath it. Measured on a real 12-storey model, that covers
 * every object `StreamAllMeshes` produces (all 17,460 resolved to a named storey, none fell through),
 * which is what makes it safe to resolve `ids` against rather than walking every line in the file.
 */
function selectExpressIds(api, modelID, opts, storeys) {
  const list = (v) => (Array.isArray(v) ? v.map((s) => String(s ?? '').trim()).filter(Boolean) : []);
  const wantStoreys = list(opts.storeys);
  const wantTypes = list(opts.ifcTypes);
  const wantIds = list(opts.ids);

  // PRESENT-BUT-EMPTY IS NOT THE SAME AS ABSENT, and getting that wrong fails in the one direction
  // that must never happen. `storeys: []` or `storeys: ['']` normalises to an empty list; treating
  // that as "no filter given" returns THE WHOLE BUILDING to a caller who asked for a subset because
  // it cannot afford the building. So the decision is made on whether the key was supplied at all,
  // and an empty one narrows to nothing — the safe direction, and visible in `selected.candidates`.
  const given = (v) => Array.isArray(v);
  if (!given(opts.storeys) && !given(opts.ifcTypes) && !given(opts.ids)) {
    return { ids: null, applied: false };
  }

  const unmatched = [];
  let ids = null;                       // null = "not narrowed yet"; every term intersects into it
  const narrow = (next) => { ids = ids === null ? next : new Set([...ids].filter((id) => next.has(id))); };

  if (given(opts.storeys)) {
    // Case-insensitive for the same reason as the type names: a storey label is copied off a probe
    // breakdown or typed by hand, and "L2" vs "l2" is not a distinction a user means to draw.
    const want = new Set(wantStoreys.map((s) => s.toUpperCase()));
    const seen = new Set();
    const hit = new Set();
    for (const [id, label] of storeys) {
      if (label == null) continue;
      seen.add(label.toUpperCase());
      if (want.has(label.toUpperCase())) hit.add(id);
    }
    for (const s of wantStoreys) if (!seen.has(s.toUpperCase())) unmatched.push({ storey: s });
    narrow(hit);
  }

  if (given(opts.ifcTypes)) {
    const hit = new Set();
    for (const name of wantTypes) {
      const code = typeCode(name);
      // UNKNOWN and KNOWN-BUT-ABSENT are both unmatched. Only the first was reported at first, so
      // asking a model for IFCCHIMNEY — a real entity this file happens not to contain — returned
      // zero objects and an empty `unmatched`, which reads as "the filter worked and there are none"
      // in exactly the same way a typo does. If a value contributed nothing, say so.
      if (code == null) { unmatched.push({ ifcType: name }); continue; }
      // `includeInherited` is deliberately true: asking for IFCBUILDINGELEMENT and being handed
      // nothing, because every actual element is a subtype of it, would be a filter that punishes
      // knowing the schema.
      const found = api.GetLineIDsWithType(modelID, code, true);
      if (!found.size()) { unmatched.push({ ifcType: name }); continue; }
      for (let i = 0; i < found.size(); i++) hit.add(found.get(i));
    }
    narrow(hit);
  }

  if (given(opts.ids)) {
    // EVERY PRODUCT, not just the ones in the spatial structure — measured, after the first cut
    // resolved GlobalIds against `storeys`'s keys alone. On a real 12-storey model that covers 17,459
    // of 17,460 objects; the one it misses is the IfcSite's own surface, which carries geometry while
    // being the top of the spatial structure rather than something contained by it. A filter that
    // cannot name an object the same read returns is a filter with a hole in it, and the hole would
    // have been invisible on every file whose site carries no geometry.
    const byGuid = new Map();
    const products = api.GetLineIDsWithType(modelID, WebIFC.IFCPRODUCT, true);
    for (let i = 0; i < products.size(); i++) {
      const id = products.get(i);
      let guid = null;
      try { guid = strOf(api.GetLine(modelID, id).GlobalId) ?? null; } catch { guid = null; }
      if (guid) byGuid.set(guid, id);
    }
    const hit = new Set();
    for (const guid of wantIds) {
      const id = byGuid.get(guid);
      if (id == null) unmatched.push({ id: guid }); else hit.add(id);
    }
    narrow(hit);
  }

  // A FILTER MUST SELECT A SUBSET OF WHAT AN UNFILTERED READ RETURNS. `StreamMeshes` honours whatever
  // ids it is handed, while `StreamAllMeshes` — the unfiltered walk — leaves out subtraction features.
  // Measured on a real 12-storey model: 17,460 objects unfiltered against 18,200 reachable by id, the
  // whole difference being 740 IfcOpeningElements. Those are VOIDS — the holes doors and windows are
  // cut from, not things anybody wants overlaid — so without this a filtered read would draw 740
  // phantom boxes filling every opening, and the same file would describe two different models
  // depending on whether you narrowed it.
  for (const id of notInAWholeRead(api, modelID)) ids?.delete(id);

  return {
    ids: ids ?? new Set(),
    applied: true,
    report: {
      ...(wantStoreys.length ? { storeys: wantStoreys } : {}),
      ...(wantTypes.length ? { ifcTypes: wantTypes } : {}),
      ...(wantIds.length ? { ids: wantIds } : {}),
      candidates: (ids ?? new Set()).size,
      unmatched,
    },
  };
}

export function extractConnection(api, modelID, guid) {
  const kids = assemblyChildren(api, modelID);
  const asmIds = api.GetLineIDsWithType(modelID, WebIFC.IFCELEMENTASSEMBLY);
  for (let i = 0; i < asmIds.size(); i++) {
    const aid = asmIds.get(i);
    const asm = api.GetLine(modelID, aid);
    if (strOf(asm.GlobalId) !== guid) continue;
    const { hardware, members, beamMembers } = classify(api, modelID, kids.get(aid) || []);
    const wantById = new Map(hardware.map((h) => [h.expressID, h.role]));
    const parts = tessellate(api, modelID, wantById);
    // Try each supported type; the recognizers are mutually exclusive (base plate = vertical anchors, shear
    // plate = horizontal bolts), so order is safe — base-plate first is just a cheap deterministic default.
    const recipe = recognizeBasePlate(parts, members) || recognizeShearPlate(parts, members);
    // `main` is advisory (the consumer overrides it with the member it applies the connection to). A fin plate
    // hangs off the supported BEAM — set it by IFC type rather than trust members[0], which could be the
    // support column when IfcRelAggregates lists it first.
    if (recipe && recipe.kind === 'shear-plate' && beamMembers.length) recipe.main = beamMembers[0];
    return {
      connection: {
        id: guid,
        name: assemblyLabel(asm),
        type: strOf(asm.ObjectType) || null,
        // The file's own frame since #347 — the same one `read-model` and `probe`
        // report. The field stays (a consumer must be able to CHECK the frame rather
        // than infer it from a version number, which was #343's point); what changed
        // is that all three commands now answer the same.
        frame: FILE_Z_UP,
        members,
        parts,
        ...(recipe ? { recipe } : {}),
      },
    };
  }
  throw new Error(`no IfcElementAssembly with GlobalId ${guid}`);
}

/**
 * Init web-ifc and open a model. Exported so the commands and the tests share ONE open sequence —
 * a test that opened the model differently (different flags, different wasm path) would be testing a
 * model nobody ships.
 *
 * Returns a handle; pass it to `closeApi` when done.
 */
export async function openApi(ifcPath) {
  // NOTE: this deliberately does NOT touch process.stdout. An earlier cut installed the stdout guard
  // here so both callers inherited it — which silently corrupted `node --test`, whose reporter streams
  // structured records over stdout: four tests ran and the summary reported one. Stream hygiene is the
  // CLI protocol's concern, so it lives in main(), which owns the protocol. SetLogLevel(OFF) below
  // quiets web-ifc at the source, which is what keeps the test path clean.
  const api = new WebIFC.IfcAPI();
  // When packaged as a single-file exe (Node SEA), web-ifc can't auto-locate its .wasm relative to a
  // real module on disk — point it at the .wasm shipped alongside the exe. Under plain `node` we skip
  // it and let web-ifc auto-locate.
  if (isPackagedExe()) {
    api.SetWasmPath(dirname(process.execPath) + sep, true);
  }
  await api.Init();
  try {
    if (WebIFC.LogLevel && typeof api.SetLogLevel === 'function') {
      api.SetLogLevel(WebIFC.LogLevel.LOG_LEVEL_OFF ?? 6);
    }
  } catch { /* older web-ifc without SetLogLevel — main()'s stdout guard still covers the CLI path */ }
  const modelID = await openModel(api, ifcPath);
  return { api, modelID };
}

/** Close a handle from `openApi`. */
export function closeApi({ api, modelID }) {
  api.CloseModel(modelID);
}

export async function main(command = process.argv[2], stdinText = undefined) {
  const args = JSON.parse((stdinText ?? readStdin()) || '{}');

  // ARGUMENT VALIDATION HAPPENS BEFORE THE STDOUT GUARD GOES UP. The guard swaps `process.stdout.write`
  // for stderr's; a `throw` between installing it and the `finally` that restores it leaves the process
  // with stdout permanently redirected, so the error path emits nothing on the channel the protocol
  // reads. The original code had the same shape (the guard went up before `openApi`), so this fixes an
  // existing latent bug rather than only avoiding a new one.
  const ifcPath = args['ifc-path'] || args.ifcPath || args.path;
  if (command === 'compare') {
    if (!args['base-ifc-path']) throw new Error('`base-ifc-path` is required for compare');
    if (!args['revised-ifc-path']) throw new Error('`revised-ifc-path` is required for compare');
  } else if (typeof ifcPath !== 'string' || !ifcPath) {
    // TYPE-checked, not merely truthy: `basename()` on a truthy non-string (a number, an array from a
    // hand-written JSON payload) throws, and everything after this point runs under the stdout guard.
    throw new Error('`ifc-path` is required, as a string');
  }

  // The bridge protocol requires PURE JSON on stdout, and both model-open and tessellation print — so
  // the guard goes up BEFORE the model is opened and comes down only to emit the result.
  const realWrite = process.stdout.write.bind(process.stdout);
  process.stdout.write = process.stderr.write.bind(process.stderr);

  // `compare` opens TWO models, neither of which is `ifc-path`, so it owns its own opens and closes
  // rather than borrowing the single-handle shape below.
  if (command === 'compare') {
    let result;
    try {
      result = await compareCommand(args);
    } finally {
      process.stdout.write = realWrite;   // restored on the throw path too, which is the point
    }
    realWrite(JSON.stringify(result));
    return;
  }

  let handle;
  let result;
  let streamed = false;
  try {
    // INSIDE the try, and `ifcPath` is type-checked above. `basename()` on a truthy non-string throws,
    // and this line used to sit between the guard going up and the try — the last remaining path that
    // could leak a redirected stdout.
    //
    // Opening a 66 MiB IFC is seconds of work before the first triangle exists, so it is its own
    // phase rather than dead air a consumer cannot distinguish from a hung bridge (#405).
    emitProgress({ phase: 'parse', message: `opening ${basename(ifcPath)}` });
    // INSIDE the try: `openApi` on a corrupt or unreadable file throws, and in the original shape that
    // throw escaped before the `finally`, leaking the guard exactly as above.
    handle = await openApi(ifcPath);
    const { api, modelID } = handle;
    if (command === 'list') {
      result = listConnections(api, modelID);
    } else if (command === 'extract') {
      const id = args.id || (args.selector && args.selector.id);
      if (!id) throw new Error('`id` (an IfcElementAssembly GlobalId, from `list`) is required for extract');
      result = extractConnection(api, modelID, id);
    } else if (command === 'probe') {
      result = probeModel(api, modelID);
    } else if (command === 'read-model') {
      result = readModelStreamed(api, modelID, args, realWrite);
      streamed = true;
    } else {
      throw new Error(`unknown command '${command}' (expected: list | extract | probe | read-model | compare)`);
    }
  } finally {
    if (handle) closeApi(handle);          // `handle` may be undefined if openApi itself threw
    process.stdout.write = realWrite;      // restore before emitting the pure-JSON result
  }
  if (!streamed || result?.['$aware-artifact']) realWrite(JSON.stringify(result));
}

/**
 * `compare` — read both files, reduce each object to a comparable AS IT STREAMS, then diff.
 *
 * ONE MODEL OPEN AT A TIME, and the objects never accumulate: `readModel`'s `onObject` sink hands each
 * object over the moment it is tessellated, `comparableFrom` keeps a fixed handful of numbers, and the
 * triangles are dropped. Reading both into arrays first would hold two complete tessellations — the
 * memory bill the design rejected an overlay diff for, arriving through the back door.
 */
async function compareCommand(args) {
  /**
   * PER-SIDE FILTERS, because the two versions are not always read under the same one. A user may pick
   * a new filter when a storey name changed ("Level 1" became "L01"), and the comparison is then
   * explicitly not like-for-like — but it still has to RUN. A single shared filter makes that case
   * unrepresentable. The bare `storeys`/`ifc-types`/`ids` remain as the shorthand for "the same on both
   * sides", which is the ordinary case.
   */
  const sideFilter = (side) => ({
    // THE KEYS ARE `storeys` / `ifcTypes` / `ids` — camelCase for the types one, because that is what
    // `selectExpressIds` reads. Passing `'ifc-types'` here would be IGNORED, and if it were the only
    // filter given, `given()` fails on all three and the WHOLE BUILDING is read — the exact
    // silent-wrong-answer direction. `readModelStreamed` does this same mapping.
    storeys: args[`${side}-storeys`] ?? args.storeys,
    ifcTypes: args[`${side}-ifc-types`] ?? args['ifc-types'] ?? args.ifcTypes,
    ids: args[`${side}-ids`] ?? args.ids,
    maxBytes: args['max-bytes'] ?? args.maxBytes,
  });

  const readComparables = async (path, side) => {
    emitProgress({ phase: 'parse', message: `opening ${side}: ${basename(path)}` });
    const handle = await openApi(path);
    const out = [];
    try {
      const res = readModel(handle.api, handle.modelID, args['max-vertices'], {
        ...sideFilter(side),
        // `(object, text)` is the real signature — `text` is the pre-serialised form and is deliberately
        // ignored here, because we keep a reduced record rather than the JSON.
        onObject: (object) => { out.push(comparableFrom(object)); },
      });
      // `skipped` is carried, not discarded: a file whose every product is undrawable arrives as an
      // empty array, and diffObjects needs to tell that apart from an empty file.
      return { objects: out, selected: res.selected ?? null, skipped: res.skipped ?? 0 };
    } finally {
      closeApi(handle);
    }
  };

  const base = await readComparables(args['base-ifc-path'], 'base');
  const revised = await readComparables(args['revised-ifc-path'], 'revised');
  const result = diffObjects(base.objects, revised.objects, {
    tolerance: Number.isFinite(args['position-tolerance-mm']) ? args['position-tolerance-mm'] : 1,
    includeUnchanged: args['include-unchanged'] === true,
    skipped: { base: base.skipped, revised: revised.skipped },
  });
  // The frame is STATED rather than left to a version number, for the reason every other command in
  // this bridge states it: the binary is installed separately and a stale one still runs.
  return { frame: FILE_Z_UP, ...result, selected: { base: base.selected, revised: revised.selected } };
}

/**
 * `read-model`, written to stdout AS IT IS PRODUCED rather than assembled and stringified.
 *
 * THIS IS THE FIX FOR THE CEILING (aware-aeco/aware#352). `JSON.stringify(result)` over a whole model
 * builds one string, and a string has a maximum length — so the command died with V8's raw
 * `Invalid string length` on real coordination models. Measured 2026-08-01, three of five real files
 * were unreadable at EVERY `max-vertices` budget: the budget error says "raise it", and raising it far
 * enough to satisfy the file lands on the ceiling instead. No number the caller could pass was an
 * answer, which is what made this a defect rather than a limit.
 *
 * One object at a time is the unit that makes it go away: each is at most a few hundred kilobytes, so
 * no single `JSON.stringify` approaches the limit however large the model is.
 *
 * `objects` is emitted LAST despite being the only field the caller waits on, and that ordering is
 * load-bearing rather than cosmetic: `skipped` and `selected` are only known once the walk is over, so
 * writing them first would mean buffering the geometry to learn them — reintroducing the very peak
 * this exists to remove. JSON object keys are unordered by definition, so no consumer may notice.
 *
 * `write` is passed in rather than taken from `process.stdout` because the stdout guard is STILL UP
 * while this runs: web-ifc prints during tessellation, and those lines must keep going to stderr until
 * the last mesh has streamed. So the guard stays on `process.stdout.write` and the real handle is
 * handed here — the one path allowed to put bytes on the protocol's stdout.
 */
function readModelStreamed(api, modelID, args, write) {
  const artifactDir = process.env.AWARE_ARTIFACT_DIR;
  // One app run can invoke this bridge more than once. The descriptor id must therefore be unique
  // within the run-owned directory; otherwise a later reader result overwrites an earlier one.
  const artifactId = `read-model-${randomUUID()}.json`;
  let fd = null;
  let artifactPath = null;
  let tempPath = null;
  if (artifactDir) {
    mkdirSync(artifactDir, { recursive: true });
    artifactPath = `${artifactDir}/${artifactId}`;
    tempPath = `${artifactPath}.${process.pid}.tmp`;
    fd = openSync(tempPath, 'w');
    write = (chunk) => writeSync(fd, chunk);
  }
  // OPT-IN, and off by default (#405). Segments cost a second copy of the geometry on disk and buy
  // nothing for a consumer that waits for the whole artifact anyway, so the caller that intends to
  // render progressively is the one that asks. Absent, this command behaves exactly as it did.
  // `progressEnabled()` is part of the condition, not an optimisation: a segment nobody can be TOLD
  // about is a second copy of the model written for no reader at all. The announcement is the
  // delivery, so no channel means no segments.
  const batchSize = batchSizeOf(args);
  const segments = artifactDir && batchSize && progressEnabled()
    ? new SegmentWriter(artifactDir, artifactId.slice(0, -'.json'.length), batchSize)
    : null;
  let first = true;
  try {
  write('{"frame":' + JSON.stringify(FILE_Z_UP) + ',"objects":[');
  const tail = readModel(api, modelID, args['max-vertices'], {
    storeys: args.storeys,
    ifcTypes: args['ifc-types'] ?? args.ifcTypes,
    ids: args.ids,
    maxBytes: args['max-bytes'] ?? args.maxBytes,
    // The `tessellate` record is published from HERE rather than before the read, because this is
    // the one moment that satisfies both halves of what it has to say: the filter has resolved to
    // expressIDs (so the receipt exists) and not one segment has been written yet (so a consumer
    // can still decide whether it may draw what follows). See `tessellateRecord`.
    onSelected: (report) => emitProgress(tessellateRecord(report)),
    // `text` is the object already serialised by readModel — see the note there on serialising once.
    onObject: (object, text) => {
      const serialized = text ?? JSON.stringify(object);
      if (!first) write(',');
      first = false;
      write(serialized);
      // The SAME serialised text goes into the segment, so progressive delivery costs one extra
      // write per object and never a second `JSON.stringify` — the duplicate serialisation the
      // whole-model path exists to avoid.
      if (segments) segments.add(serialized);
    },
  });
  if (segments) segments.flush();
  write(']');
  for (const [key, value] of Object.entries(tail)) {
    // `frame` is already written and `objects` was the stream; everything else is the tail.
    if (key === 'frame' || key === 'objects') continue;
    write(',' + JSON.stringify(key) + ':' + JSON.stringify(value));
  }
  write('}');
  if (!fd) {
    emitProgress({ phase: 'complete', done: tail.count, total: tail.count });
    return tail;
  }
  closeSync(fd);
  fd = null;
  renameSync(tempPath, artifactPath);
  const descriptor = {
    id: artifactId,
    mediaType: 'application/json',
    bytes: statSync(artifactPath).size,
    items: tail.count,
    ...(segments ? { segments: segments.count } : {}),
  };
  // The completion record carries the WHOLE artifact's descriptor, so a consumer that renders from
  // segments learns in one place that the walk is over and where the durable copy is — without
  // having to correlate the trace's node output back to the progress stream it was following.
  emitProgress({ phase: 'complete', done: tail.count, total: tail.count, artifact: descriptor });
  return { '$aware-artifact': descriptor };
  } finally {
    if (fd != null) closeSync(fd);
  }
}

/** The receipt fields that echo caller-supplied values, and so have no bound of their own. */
const ECHOED_LISTS = ['ids', 'unmatched', 'ifcTypes', 'storeys'];

/**
 * The `tessellate` record, carrying the filter receipt a progressive consumer decides on (#407).
 *
 * WHAT IT IS FOR. Segments arrive with no statement of what was selected, so a consumer drawing
 * them cannot tell a bridge that honoured `storeys: [L2]` and is streaming one floor from a
 * pre-1.2.0 bridge that ignored the filter, exited zero, and is streaming the whole building. Both
 * look like geometry turning up. `selected` in the node output settles it, but only once the walk
 * is over — which is the wait progressive delivery exists to remove. So the same receipt is
 * published here, once, before the first segment.
 *
 * ABSENT KEEPS ITS MEANING. No `selected` on this record still reads as "this bridge cannot
 * answer", exactly as an absent `selected`, `budget` or `colorsAvailable` does elsewhere in this
 * contract — so an unfiltered read publishes none and nothing existing changes.
 *
 * WHY IT IS BOUNDED. The runtime drops a record over MAX_RECORD_BYTES rather than truncating it,
 * and drops it silently. A receipt echoes caller-supplied values, and `ids` is documented for
 * "re-reading a known selection" — a few hundred GlobalIds is an ordinary use of it and ~8 KiB of
 * JSON. Emitting the full echo would therefore delete the whole record precisely when the caller
 * filtered hardest, and the consumer would read that absence as "old bridge, do not draw": the
 * feature silently off in its best case. So the echoed lists are dropped, largest first, until the
 * record fits, and every one dropped is NAMED in `elided`.
 *
 * Dropped, not shortened, and named rather than quietly omitted — because a half-echoed list is a
 * DIFFERENT filter, and a consumer comparing it against what it sent would find a mismatch it
 * could only read as "this bridge honoured something else". `candidates` survives every elision:
 * it is the count that says a filter ran at all, and it is one number.
 */
export function tessellateRecord(report) {
  const base = { phase: 'tessellate', message: 'walking geometry' };
  if (!report) return base;

  const kept = { ...report };
  const elided = [];
  for (;;) {
    const selected = elided.length ? { ...kept, elided: [...elided].sort() } : kept;
    const record = { ...base, selected };
    if (progressRecordBytes(record) <= MAX_RECORD_BYTES) return record;
    // Largest first, so the fewest lists are lost to make room.
    const biggest = ECHOED_LISTS
      .filter((key) => Array.isArray(kept[key]) && kept[key].length)
      .sort((a, b) => JSON.stringify(kept[b]).length - JSON.stringify(kept[a]).length)[0];
    // Unreachable — `{ phase, message, candidates, elided }` is a few dozen bytes — but if the
    // record cannot be made to fit, publish the PHASE rather than let the runtime drop the lot. A
    // consumer then falls back to wait-for-complete, which is what absence already tells it.
    if (!biggest) return base;
    delete kept[biggest];
    elided.push(biggest);
  }
}

/**
 * `batch-size` in objects, or 0 for "do not segment".
 *
 * Anything that is not a positive whole number means off rather than throwing: this input only ever
 * adds an optional delivery path, and refusing a read outright over a malformed hint would fail the
 * geometry the caller actually asked for.
 */
function batchSizeOf(args) {
  const raw = args['batch-size'] ?? args.batchSize;
  const n = Number(raw);
  return Number.isInteger(n) && n > 0 ? n : 0;
}

/**
 * Ordered geometry segments, written into the run-owned artifact directory as the walk produces
 * them and announced on the progress channel once each is durable (#405).
 *
 * WHY SEGMENTS AND NOT A STREAM OF EVENTS. The trace is the runtime's subscription channel and it
 * must stay bounded — that is the whole point of #402 — so the geometry cannot travel through it.
 * A segment is a normal run-owned artifact: the consumer fetches it with the same
 * `aware app artifact` call it already uses, and the runtime never holds a mesh in memory.
 *
 * ANNOUNCED AFTER THE RENAME, never before. A consumer acts on the record by opening the file, so
 * advertising a path still being written would hand it a truncated document — and the failure would
 * look like corrupt geometry rather than a race.
 */
class SegmentWriter {
  constructor(dir, stem, size) {
    this.dir = dir;
    this.stem = stem;
    this.size = size;
    this.pending = [];
    this.count = 0;      // segments written
    this.delivered = 0;  // objects handed over so far
  }

  add(text) {
    this.pending.push(text);
    if (this.pending.length >= this.size) this.write();
  }

  /** The final partial segment. A model whose object count is not a multiple of the batch size must
   *  not silently lose its last few objects from the progressive path. */
  flush() {
    if (this.pending.length) this.write();
  }

  write() {
    const seq = this.count + 1;
    // Zero-padded so the ids sort lexicographically in the order they were produced — a consumer
    // listing the directory sees the model's own order without parsing numbers out of names.
    const id = `${this.stem}.seg-${String(seq).padStart(5, '0')}.json`;
    const path = `${this.dir}/${id}`;
    const temp = `${path}.${process.pid}.tmp`;
    const fd = openSync(temp, 'w');
    try {
      // ONE write per segment, not one per object. The whole-model path streams object by object
      // because its document has no bound; a segment does — `batch-size` is that bound — so joining
      // here costs a string the size of one batch and saves a syscall per object. Measured on a
      // 12,000-object / 46 MiB model (`bench-progressive.mjs`): it took the progressive path's
      // overhead on time-to-COMPLETE from ~47% to ~22-34%. The rest is the second copy of the
      // geometry itself, which is what buys the segments.
      //
      // Self-contained: `frame` rides on every segment so a consumer can place the first batch
      // without waiting for the tail keys of the complete artifact.
      writeSync(
        fd,
        `{"frame":${JSON.stringify(FILE_Z_UP)},"seq":${seq},"objects":[${this.pending.join(',')}]}`,
      );
    } finally {
      closeSync(fd);
    }
    renameSync(temp, path);
    const items = this.pending.length;
    this.pending = [];
    this.count = seq;
    this.delivered += items;
    emitProgress({
      phase: 'batch',
      seq,
      done: this.delivered,
      artifact: { id, mediaType: 'application/json', bytes: statSync(path).size, items, seq },
    });
  }
}

// Only run as a CLI when this file IS the entry point. Without this guard, importing the module from a
// test executes main(), which reads fd 0 and exits non-zero.
//
// SEA ownership belongs to `model-dispatcher.mjs`, the package/bin and build entrypoint. Keeping an
// `isPackagedExe()` arm here would make this lazily imported IFC module launch a second main inside
// the same executable and race the dispatcher for stdin. Plain source invocation remains supported.
const invokedDirectly = !!process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href;
if (invokedDirectly) {
  main().catch((e) => {
    process.stderr.write(`connection-reader: ${e && e.message ? e.message : e}\n`);
    process.exit(1);
  });
}
