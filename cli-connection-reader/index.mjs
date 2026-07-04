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
import { unzipSync } from 'fflate'; // tiny pure-JS unzip for .ifczip inputs
import * as WebIFC from 'web-ifc'; // package export resolves to the node build (auto-locates its .wasm)
import { recognizeBasePlate, recognizeShearPlate } from './recognize.mjs'; // fit a parametric recipe from the tessellated parts

// web-ifc returns geometry in metres (SI base unit); AWARE scenes are canonical millimetres.
const M_TO_MM = 1000;

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

async function main() {
  const command = process.argv[2];
  const args = JSON.parse(readStdin() || '{}');
  const ifcPath = args['ifc-path'] || args.ifcPath || args.path;
  if (!ifcPath) throw new Error('`ifc-path` is required');

  // web-ifc's WASM prints diagnostics to stdout (C++ printf → Module.print); the bridge protocol
  // requires PURE JSON on stdout. Route any stray stdout to stderr while we work, and restore it
  // only to emit the result. SetLogLevel(OFF) quiets the diagnostics at the source too.
  const realWrite = process.stdout.write.bind(process.stdout);
  process.stdout.write = process.stderr.write.bind(process.stderr);

  const api = new WebIFC.IfcAPI();
  // When packaged as a single-file exe (Node SEA), web-ifc can't auto-locate its .wasm relative to a
  // real module on disk — point it at the .wasm shipped alongside the exe. Plain `node index.mjs`
  // (dev) runs from node.exe, so the basename starts with "node" → skip and let web-ifc auto-locate.
  if (!basename(process.execPath).toLowerCase().startsWith('node')) {
    api.SetWasmPath(dirname(process.execPath) + sep, true);
  }
  await api.Init();
  try {
    if (WebIFC.LogLevel && typeof api.SetLogLevel === 'function') {
      api.SetLogLevel(WebIFC.LogLevel.LOG_LEVEL_OFF ?? 6);
    }
  } catch { /* older web-ifc without SetLogLevel — the stdout guard still covers us */ }

  let result;
  const modelID = await openModel(api, ifcPath);
  try {
    if (command === 'list') {
      result = listConnections(api, modelID);
    } else if (command === 'extract') {
      const id = args.id || (args.selector && args.selector.id);
      if (!id) throw new Error('`id` (an IfcElementAssembly GlobalId, from `list`) is required for extract');
      result = extractConnection(api, modelID, id);
    } else {
      throw new Error(`unknown command '${command}' (expected: list | extract)`);
    }
  } finally {
    api.CloseModel(modelID);
    process.stdout.write = realWrite; // restore before emitting the pure-JSON result
  }
  realWrite(JSON.stringify(result));
}

main().catch((e) => {
  process.stderr.write(`connection-reader: ${e && e.message ? e.message : e}\n`);
  process.exit(1);
});
