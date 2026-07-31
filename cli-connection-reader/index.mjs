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
//   extract   inputs { ifc-path, id }                -> { connection: {id,name,type,members,parts:[mesh…],recipe?} }
//             Tessellate ONE candidate (by its IfcElementAssembly GlobalId) into mesh scene parts, AND —
//             when the parts match a supported pattern (a base plate with a vertical anchor grid) — fit a
//             parametric `recipe:{kind,params}` so the consumer can import it as an EDITABLE recipe rather
//             than opaque mesh. `parts` is always returned as the fallback; `recipe` only when confident.

import { readFileSync } from 'node:fs';
import { dirname, basename, sep } from 'node:path';
import { pathToFileURL } from 'node:url';
import { createRequire } from 'node:module';
import { unzipSync } from 'fflate'; // tiny pure-JS unzip for .ifczip inputs
import * as WebIFC from 'web-ifc'; // package export resolves to the node build (auto-locates its .wasm)
import { recognizeBasePlate, recognizeShearPlate } from './recognize.mjs'; // fit a parametric recipe from the tessellated parts

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

function listConnections(api, modelID) {
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
      const m = pg.flatTransformation; // 4x4 column-major, metres
      const base = positions.length / 3;
      for (let v = 0; v < verts.length; v += 6) {
        const x = verts[v], y = verts[v + 1], z = verts[v + 2];
        positions.push(
          (m[0] * x + m[4] * y + m[8] * z + m[12]) * M_TO_MM,
          (m[1] * x + m[5] * y + m[9] * z + m[13]) * M_TO_MM,
          (m[2] * x + m[6] * y + m[10] * z + m[14]) * M_TO_MM,
        );
      }
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
 * What the file DECLARES as its length unit, verbatim: "MILLI.METRE" | "METRE" | … | null.
 *
 * This is PROVENANCE, not a conversion factor for geometry. Measured 2026-07-25: web-ifc reads
 * IfcUnitAssignment itself and normalises tessellated geometry to metres before we ever see a vertex,
 * so nothing downstream may multiply mesh coordinates by this. It exists so a user can see what they
 * were handed, and so a file that LIES about its units can be spotted and overridden.
 * (floless.app/docs/superpowers/specs/2026-07-25-reference-objects-units-evidence.md)
 *
 * null means the file did not say — an honest "unknown", never a guess.
 */
export function declaredUnit(api, modelID) {
  return lengthUnit(api, modelID).declared;
}

/** The declared LENGTHUNIT as `{ declared, id }` — the label for humans, the express id so the scale
 *  can be resolved. Both null when the file declares no length unit. */
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
 * the declared unit, so it includes local profile coordinates and ignores placement nesting. It is
 * good enough for "is this thing 1000x off, or sitting 74 m from the origin?" and is not the
 * authoritative extent — that comes from real geometry in `read-model`.
 */
export function probeModel(api, modelID) {
  const { declared, id: unitId } = lengthUnit(api, modelID);
  const toMM = unitId == null ? null : unitToMM(api, modelID, unitId);
  // An unresolvable unit means we cannot state the extent in millimetres. Say so with `bbox: null`
  // rather than emitting a plausible-looking box scaled by a guessed 1 — a wrong extent is what
  // drives a consumer's "this looks 1000x off" logic, so a guess here causes the exact misjudgement
  // the size check exists to make correctly.
  if (toMM == null) {
    return {
      schema: api.GetModelSchema ? api.GetModelSchema(modelID) : null,
      units: { declared },
      elements: placedElements(api, modelID).size,
      bbox: null,
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
    bbox: empty ? null : { min, max },
  };
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
  // Push each containment down through IfcRelAggregates. Repeat until nothing new is learned so a
  // nested assembly resolves at any depth; the pass count is bounded so a cyclic file cannot spin.
  const children = assemblyChildren(api, modelID);
  for (let pass = 0; pass < 8; pass++) {
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
 * `read-model` — the whole file as reference geometry.
 *
 * Unlike `extract`, this tessellates EVERY element rather than only connection hardware, and returns
 * what the file says about each one (name, IFC type, storey, profile, material) alongside its mesh.
 * Positions stay in the file's own world frame, in millimetres — a consumer re-anchors and places.
 *
 * `maxVertices` is an in-process CIRCUIT BREAKER, not a preflight gate. Be honest about which it is:
 * an exact vertex count cannot be known before tessellating, so a cap applied by the caller after this
 * returns would fire only once the oversized payload had already been built and serialised — reporting
 * the freeze it was meant to prevent. Decrementing as meshes stream is what actually stops it. The
 * byte-size check a caller does before invoking us is the only true preflight protection.
 */
export function readModel(api, modelID, maxVertices = Infinity) {
  const storeys = storeyByElement(api, modelID);
  const materials = materialByElement(api, modelID);
  const objects = [];
  let budget = maxVertices;
  let skipped = 0; // products web-ifc streamed but that carry no drawable triangle

  api.StreamAllMeshes(modelID, (flatMesh) => {
    const positions = [];
    const indices = [];
    const geoms = flatMesh.geometries;
    // NOTE: `geometries` carries one entry PER INSTANCE, each with its own flatTransformation — which
    // is exactly how mapped/instanced items work. Do not de-duplicate by geometryExpressID: the
    // Motebello file serves 19 objects from 14 shapes, and de-duplicating silently loses five walls.
    for (let i = 0; i < geoms.size(); i++) {
      const pg = geoms.get(i);
      const geom = api.GetGeometry(modelID, pg.geometryExpressID);
      const verts = api.GetVertexArray(geom.GetVertexData(), geom.GetVertexDataSize()); // [x,y,z,nx,ny,nz]*
      const idx = api.GetIndexArray(geom.GetIndexData(), geom.GetIndexDataSize());
      const m = pg.flatTransformation; // 4x4 column-major, metres
      const base = positions.length / 3;
      for (let v = 0; v < verts.length; v += 6) {
        const x = verts[v], y = verts[v + 1], z = verts[v + 2];
        positions.push(
          (m[0] * x + m[4] * y + m[8] * z + m[12]) * M_TO_MM,
          (m[1] * x + m[5] * y + m[9] * z + m[13]) * M_TO_MM,
          (m[2] * x + m[6] * y + m[10] * z + m[14]) * M_TO_MM,
        );
      }
      for (let k = 0; k < idx.length; k++) indices.push(base + idx[k]);
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
    objects.push({
      id: (line && strOf(line.GlobalId)) || String(id),
      name: (line && strOf(line.Name)) || null,
      ifcType: typeName(api.GetLineType(modelID, id)),
      storey: storeys.get(id) ?? null,
      profile: profileOf(api, modelID, id),
      material: materials.get(id) ?? null,
      positions,
      indices,
    });
  });

  return { objects, skipped };
}

function extractConnection(api, modelID, guid) {
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

async function main() {
  const command = process.argv[2];
  const args = JSON.parse(readStdin() || '{}');
  const ifcPath = args['ifc-path'] || args.ifcPath || args.path;
  if (!ifcPath) throw new Error('`ifc-path` is required');

  // The bridge protocol requires PURE JSON on stdout, and both model-open and tessellation print — so
  // the guard goes up BEFORE the model is opened and comes down only to emit the result.
  const realWrite = process.stdout.write.bind(process.stdout);
  process.stdout.write = process.stderr.write.bind(process.stderr);

  const handle = await openApi(ifcPath);
  const { api, modelID } = handle;
  let result;
  try {
    if (command === 'list') {
      result = listConnections(api, modelID);
    } else if (command === 'extract') {
      const id = args.id || (args.selector && args.selector.id);
      if (!id) throw new Error('`id` (an IfcElementAssembly GlobalId, from `list`) is required for extract');
      result = extractConnection(api, modelID, id);
    } else if (command === 'probe') {
      result = probeModel(api, modelID);
    } else if (command === 'read-model') {
      result = readModel(api, modelID, args['max-vertices']);
    } else {
      throw new Error(`unknown command '${command}' (expected: list | extract | probe | read-model)`);
    }
  } finally {
    closeApi(handle);
    process.stdout.write = realWrite; // restore before emitting the pure-JSON result
  }
  realWrite(JSON.stringify(result));
}

// Only run as a CLI when this file IS the entry point. Without this guard, importing the module from a
// test executes main(), which reads fd 0 and exits non-zero.
//
// The packaged case is checked FIRST and does not depend on argv[1]. Shipped as a SEA the bundle is
// CJS inside a renamed node.exe, where argv[1] is not this script and may be absent entirely — an
// argv-first guard would evaluate false and leave the bridge silently doing nothing, which is a far
// worse failure than the one the guard prevents.
const invokedDirectly = isPackagedExe()
  || (!!process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href);
if (invokedDirectly) {
  main().catch((e) => {
    process.stderr.write(`connection-reader: ${e && e.message ? e.message : e}\n`);
    process.exit(1);
  });
}
