#!/usr/bin/env node
// Time-to-first-render vs time-to-complete, measured (aware-aeco/aware#405).
//
// The issue's claim is a latency one — "IFC Lite showed first visible geometry in roughly two
// seconds where AWARE's consumer waits ~43 s for the reader alone" — so it is settled by measuring,
// not by reasoning about the design. This drives the REAL bridge twice over the same file:
//
//   baseline     the #402 artifact path. A consumer can draw nothing until the whole artifact
//                exists AND has been parsed, so its time-to-first-render IS its time-to-complete.
//   progressive  the #405 path (`batch-size` + a progress channel). Time-to-first-render is the
//                moment the first announced segment has been read and parsed — the same work a
//                renderer does before it can put triangles on screen.
//
// Both runs pay the identical tessellation cost; the only difference is when the consumer may act.
//
// Usage:
//   node bench-progressive.mjs                          # synthesise a model, then measure
//   node bench-progressive.mjs --ifc C:/models/PALM.ifc # measure the real file from the issue
//   node bench-progressive.mjs --objects 2000 --batch-size 100
//
// `--ifc` is how the PALM.ifc comparison the acceptance criteria ask for is run: that file is 66 MiB
// and cannot live in the repo, so the benchmark takes a path and synthesises a stand-in when none is
// given. The synthetic model is deliberately dull geometry (extruded circular columns) — it exists
// to make the walk long enough to measure, not to imitate a coordination model.

import { execFile } from 'node:child_process';
import { mkdtempSync, openSync, readFileSync, rmSync, statSync, writeSync, closeSync } from 'node:fs';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
import { tmpdir } from 'node:os';

// Measuring is the entry point; the synthetic-model writer is exported so an end-to-end check can
// build a model to read without also running a benchmark as an import side effect.
const invokedDirectly = !!process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href;
const args = invokedDirectly ? parseArgs(process.argv.slice(2)) : { objects: 0, batchSize: 0, ifc: null };

/** `--objects 400 --batch-size 50 --ifc path` with defaults. */
function parseArgs(argv) {
  const out = { objects: 400, batchSize: 50, ifc: null };
  for (let i = 0; i < argv.length; i += 2) {
    const value = argv[i + 1];
    if (argv[i] === '--objects') out.objects = Number(value);
    else if (argv[i] === '--batch-size') out.batchSize = Number(value);
    else if (argv[i] === '--ifc') out.ifc = value;
    else throw new Error(`unknown flag ${argv[i]} (see the header of this file)`);
  }
  return out;
}

// ── a synthetic model, so the benchmark runs without a 66 MiB file ────────────────────────────────

/** IFC's own base64 alphabet for GlobalIds — order matters only in that it must be stable. */
const GUID_CHARS = '0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz_$';
function guid(n) {
  let s = '';
  for (let i = 0; i < 22; i++) {
    s = GUID_CHARS[(n + i * 7) % 64] + s;
  }
  return s;
}

/**
 * Write an IFC4 file holding `count` extruded circular columns on a grid.
 *
 * Circles rather than boxes on purpose: web-ifc tessellates a circular profile into tens of
 * triangles where a rectangle gives twelve, so each object carries a mesh worth streaming and the
 * walk is dominated by tessellation — as it is on a real model.
 */
export function writeSyntheticIfc(path, count) {
  const fd = openSync(path, 'w');
  const w = (s) => writeSync(fd, s);
  try {
    w(`ISO-10303-21;
HEADER;
FILE_DESCRIPTION(('ViewDefinition [CoordinationView]'),'2;1');
FILE_NAME('','2026-08-12T00:00:00',(''),(''),'aware bench','aware bench','');
FILE_SCHEMA(('IFC4'));
ENDSEC;
DATA;
#1=IFCSIUNIT(*,.LENGTHUNIT.,$,.METRE.);
#2=IFCUNITASSIGNMENT((#1));
#3=IFCCARTESIANPOINT((0.,0.,0.));
#4=IFCDIRECTION((0.,0.,1.));
#5=IFCDIRECTION((1.,0.,0.));
#6=IFCAXIS2PLACEMENT3D(#3,#4,#5);
#7=IFCGEOMETRICREPRESENTATIONCONTEXT($,'Model',3,1.0E-05,#6,$);
#8=IFCPROJECT('${guid(1)}',$,'Synthetic benchmark model',$,$,$,$,(#7),#2);
#9=IFCCARTESIANPOINT((0.,0.,0.));
#10=IFCAXIS2PLACEMENT3D(#9,$,$);
#11=IFCLOCALPLACEMENT($,#10);
#12=IFCSITE('${guid(2)}',$,'Site',$,$,#11,$,$,.ELEMENT.,$,$,$,$,$);
#13=IFCCARTESIANPOINT((0.,0.,0.));
#14=IFCAXIS2PLACEMENT3D(#13,$,$);
#15=IFCLOCALPLACEMENT(#11,#14);
#16=IFCBUILDING('${guid(3)}',$,'Building',$,$,#15,$,$,.ELEMENT.,$,$,$);
#17=IFCRELAGGREGATES('${guid(4)}',$,$,$,#8,(#12));
#18=IFCRELAGGREGATES('${guid(5)}',$,$,$,#12,(#16));
`);
    const products = [];
    for (let i = 0; i < count; i++) {
      const b = 18 + i * 12;
      const x = ((i % 40) * 3).toFixed(1);
      const y = (Math.floor(i / 40) * 3).toFixed(1);
      products.push(`#${b + 12}`);
      w(`#${b + 1}=IFCCARTESIANPOINT((0.,0.,0.));
#${b + 2}=IFCAXIS2PLACEMENT3D(#${b + 1},$,$);
#${b + 3}=IFCLOCALPLACEMENT(#15,#${b + 2});
#${b + 4}=IFCCARTESIANPOINT((0.,0.));
#${b + 5}=IFCAXIS2PLACEMENT2D(#${b + 4},$);
#${b + 6}=IFCCIRCLEPROFILEDEF(.AREA.,$,#${b + 5},0.15);
#${b + 7}=IFCCARTESIANPOINT((${x},${y},0.));
#${b + 8}=IFCAXIS2PLACEMENT3D(#${b + 7},$,$);
#${b + 9}=IFCEXTRUDEDAREASOLID(#${b + 6},#${b + 8},#4,3.5);
#${b + 10}=IFCSHAPEREPRESENTATION(#7,'Body','SweptSolid',(#${b + 9}));
#${b + 11}=IFCPRODUCTDEFINITIONSHAPE($,$,(#${b + 10}));
#${b + 12}=IFCCOLUMN('${guid(100 + i)}',$,'COL-${i + 1}',$,$,#${b + 3},#${b + 11},$,$);
`);
    }
    w(`#${19 + count * 12}=IFCRELCONTAINEDINSPATIALSTRUCTURE('${guid(6)}',$,$,$,(${products.join(',')}),#16);
ENDSEC;
END-ISO-10303-21;
`);
  } finally {
    closeSync(fd);
  }
  return path;
}

// ── driving the bridge ────────────────────────────────────────────────────────────────────────────

/**
 * Run `read-model` once, watching the progress channel exactly as the runtime does.
 *
 * `onFirstBatch` fires the moment a segment is announced, and is where the benchmark does the work a
 * renderer would: read the segment and parse it. Anything measured before that parse would be
 * measuring the announcement, not the render.
 */
function runRead(config, env, onRecord) {
  return new Promise((resolve, reject) => {
    const started = process.hrtime.bigint();
    const child = execFile(process.execPath, ['index.mjs', 'read-model'], {
      cwd: import.meta.dirname, encoding: 'utf8', maxBuffer: 1 << 30, env: { ...process.env, ...env },
    }, (err, stdout) => {
      clearInterval(poll);
      if (err) return reject(err);
      resolve({ stdout, elapsedMs: ms(started) });
    });
    child.stdin.end(JSON.stringify(config));

    let offset = 0;
    const channel = env.AWARE_PROGRESS_FILE;
    const poll = setInterval(() => {
      if (!channel) return;
      let text;
      try {
        text = readFileSync(channel, 'utf8');
      } catch { return; }            // not created yet
      const fresh = text.slice(offset);
      const cut = fresh.lastIndexOf('\n');
      if (cut < 0) return;
      offset += cut + 1;
      for (const line of fresh.slice(0, cut).split('\n')) {
        if (!line.trim()) continue;
        onRecord(JSON.parse(line)['$aware-progress'], ms(started));
      }
      // 10 ms rather than the runtime's 50: the benchmark must not attribute its own polling
      // interval to the mechanism it is measuring.
    }, 10);
  });
}

const ms = (from) => Number(process.hrtime.bigint() - from) / 1e6;
const fmt = (n) => `${n.toFixed(0).padStart(7)} ms`;

async function main() {
  const work = mkdtempSync(join(tmpdir(), 'aware-bench-'));
  try {
    const ifcPath = args.ifc ?? writeSyntheticIfc(join(work, 'synthetic.ifc'), args.objects);
    console.log(`model:  ${ifcPath}`);
    console.log(`bytes:  ${statSync(ifcPath).size.toLocaleString()}`);

    // ── baseline: today's artifact path ──────────────────────────────────────────────────────────
    const baseDir = join(work, 'baseline');
    const base = await runRead({ 'ifc-path': ifcPath }, { AWARE_ARTIFACT_DIR: baseDir }, () => {});
    const baseId = JSON.parse(base.stdout)['$aware-artifact'].id;
    const parseStart = process.hrtime.bigint();
    const whole = JSON.parse(readFileSync(join(baseDir, baseId), 'utf8'));
    const baseParseMs = ms(parseStart);
    // The consumer's first render cannot precede the parse of the one artifact it was given.
    const baseFirstRender = base.elapsedMs + baseParseMs;

    // ── progressive: segments announced as they land ─────────────────────────────────────────────
    const progDir = join(work, 'progressive');
    const channel = join(work, 'progress.jsonl');
    let firstRenderMs = null;
    let firstRenderObjects = 0;
    const phases = [];
    const prog = await runRead(
      { 'ifc-path': ifcPath, 'batch-size': args.batchSize },
      { AWARE_ARTIFACT_DIR: progDir, AWARE_PROGRESS_FILE: channel },
      (record, at) => {
        phases.push(`${record.phase}@${at.toFixed(0)}ms`);
        if (firstRenderMs === null && record.phase === 'batch') {
          // The announcement plus the read-and-parse a renderer must pay before it can draw.
          const load = process.hrtime.bigint();
          const segment = JSON.parse(readFileSync(join(progDir, record.artifact.id), 'utf8'));
          firstRenderObjects = segment.objects.length;
          firstRenderMs = at + ms(load);
        }
      },
    );
    const progDescriptor = JSON.parse(prog.stdout)['$aware-artifact'];

    console.log(`objects: ${whole.count.toLocaleString()}   artifact: ${(progDescriptor.bytes / 1048576).toFixed(1)} MiB   segments: ${progDescriptor.segments}`);
    console.log('');
    console.log('                       first render     complete');
    console.log(`  baseline (#402)      ${fmt(baseFirstRender)}    ${fmt(base.elapsedMs + baseParseMs)}`);
    console.log(`  progressive (#405)   ${fmt(firstRenderMs ?? NaN)}    ${fmt(prog.elapsedMs)}`);
    console.log('');
    if (firstRenderMs) {
      console.log(`  first geometry ${(baseFirstRender / firstRenderMs).toFixed(1)}x sooner `
        + `(${firstRenderObjects} objects of ${whole.count} drawable after ${firstRenderMs.toFixed(0)} ms)`);
    }
    console.log(`  complete-path overhead: ${((prog.elapsedMs / base.elapsedMs - 1) * 100).toFixed(1)}%`);
    console.log(`  phases: ${phases.slice(0, 6).join(' ')}${phases.length > 6 ? ' …' : ''}`);
  } finally {
    rmSync(work, { recursive: true, force: true });
  }
}

if (invokedDirectly) {
  main().catch((e) => {
    process.stderr.write(`bench: ${e && e.message ? e.message : e}\n`);
    process.exit(1);
  });
}
